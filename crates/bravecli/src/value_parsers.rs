pub fn q_value_parser(q: &str) -> std::result::Result<String, String> {
    if q.len() > 400 {
        return Err("Query term is too long. Maximum 400 characters allowed".to_string());
    }
    let words = q.split_whitespace().collect::<Vec<&str>>();
    if words.len() > 50 {
        return Err("Query term is too long. Maximum 50 words allowed".to_string());
    }
    Ok(q.to_owned())
}

pub fn result_filter_value_parser(result_filter: &str) -> std::result::Result<String, String> {
    let valid_values =
        ["discussions", "faq", "infobox", "news", "query", "summarizer", "videos", "web"];
    let values = result_filter.split(',').collect::<Vec<&str>>();
    for value in values.iter() {
        let has_value = valid_values.contains(value);
        if !has_value {
            return Err(
                "Invalid result filter value. It should be a comma-separated list of these values: discussions, faq, infobox, news, query, summarizer, videos, web".to_string()
            );
        }
    }
    Ok(result_filter.to_owned())
}

pub fn freshness_value_parser(freshness: &str) -> std::result::Result<String, String> {
    if ["pd", "pw", "pm"].contains(&freshness) {
        Ok(freshness.to_owned())
    } else {
        let re = regex::Regex::new(r"^\d{4}-\d{2}-\d{2}to\d{4}-\d{2}-\d{2}$")
            .map_err(|_| "Invalid regular expression".to_string())?;
        if re.is_match(freshness) {
            Ok(freshness.to_owned())
        } else {
            Err("Invalid freshness value. Must be 'pd', 'pw', 'pm' or 'YYYY-MM-DDtoYYYY-MM-DD'"
                .to_string())
        }
    }
}
