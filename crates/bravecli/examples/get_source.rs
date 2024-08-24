use clap::Parser;
use color_eyre::eyre::{Report, Result};
use fantoccini::elements::Element;
use fantoccini::{ClientBuilder, Locator};
use sysinfo::System;
use tokio::process::Command;
use tokio::time::{sleep, Duration};

#[derive(Debug, Parser)]
#[command(author, version, about, long_about = "Get the HTML contents of a page")]
pub struct Cli {
    /// Web URL
    pub url: String,

    /// Wait for this element to be present in the screen.
    #[clap(long)]
    pub element: Option<String>,

    /// Wait for an amount of seconds, waiting for  -the page to load.
    #[clap(long)]
    pub seconds: Option<u16>,
}

#[tokio::main]
async fn main() -> Result<()> {
    env_logger::builder().filter_level(log::LevelFilter::Error).init();
    color_eyre::install().map_err(Report::msg)?;

    let cli = Cli::parse();

    log::info!("Prepare the geckodriver command.");
    let mut command = Command::new("geckodriver");
    command.stdout(std::process::Stdio::null());
    command.stderr(std::process::Stdio::null());
    command.env("MOZ_HEADLESS", "1");
    command.env("MOZ_REMOTE_SETTINGS_DEVTOOLS", "1");
    command.arg("-b").arg("/Applications/Firefox.app/Contents/MacOS/firefox");

    log::info!("Launch gecokriver");
    let mut child = command.kill_on_drop(true).spawn().expect("Failed to start geckodriver");

    // Check if Firefox process is running
    let mut sys = System::new_all();
    sys.refresh_all();

    // Define timeout duration and sleep interval
    let timeout_duration = Duration::from_secs(10); // Total timeout period
    let sleep_interval = Duration::from_millis(500); // Interval between checks
    let start_time = tokio::time::Instant::now();

    log::info!("Checking for Firefox process...");
    loop {
        let mut sys = System::new_all();
        sys.refresh_all();

        if sys.processes_by_name("firefox".as_ref()).count() > 0 {
            log::info!("Headless Firefox instance detected.");
            break;
        }

        if start_time.elapsed() >= timeout_duration {
            log::error!("Timeout: Failed to detect a running Firefox instance.");
            return Err(Report::msg("Firefox process not found within the timeout period"));
        }

        sleep(sleep_interval).await;
    }

    log::info!("Create webdriver client");
    let c = ClientBuilder::native().connect("http://localhost:4444").await.map_err(Report::msg)?;

    log::info!("Getting webpage");
    c.goto(&cli.url).await?;
    let url = c.current_url().await?;
    assert_eq!(url.as_ref(), &cli.url);

    if let Some(element) = cli.element {
        log::info!("Wait for locator: {element}");
        let _: Element = c.wait().for_element(Locator::Css(&element)).await?;
    } else if let Some(seconds) = cli.seconds {
        sleep(Duration::from_secs(seconds as u64)).await;
    }

    let source = c.source().await?;
    println!("{}", source);

    c.close().await.map_err(Report::msg)?;
    // Send SIGTERM and wait for the process to exit gracefully
    log::info!("Attempting to terminate geckodriver gracefully.");
    match child.id() {
        Some(pid) => {
            use nix::sys::signal::{kill, Signal};
            use nix::unistd::Pid;
            kill(Pid::from_raw(pid as i32), Signal::SIGTERM).expect("Failed to send SIGTERM");

            // Wait for a maximum of 5 seconds for the child to exit
            let timeout = Duration::from_secs(5);
            let result = tokio::select! {
                _ = child.wait() => {
                    log::info!("geckodriver terminated gracefully.");
                    Ok(())
                }
                _ = sleep(timeout) => {
                    log::warn!("geckodriver did not terminate gracefully within timeout.");
                    Err("Timeout")
                }
            };

            // If timeout occurred, send SIGKILL
            if result.is_err() {
                log::info!("Forcefully killing geckodriver.");
                child.kill().await.expect("Failed to kill geckodriver forcefully");
            }
        }
        None => log::warn!("geckodriver process already exited."),
    }

    Ok(())
}
