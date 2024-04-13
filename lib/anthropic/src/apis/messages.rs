// See: https://docs.anthropic.com/claude/reference/messages_post

//! Messages API

use eventsource_client as es;
use futures::stream::{Stream, StreamExt};
use std::collections::HashMap;

use crate::requests::Requests;
use crate::{error, Anthropic, ApiResult, Content, Message, Usage};
use serde::{Deserialize, Serialize};

use super::MESSAGES_CREATE;

#[derive(Debug, Serialize, Deserialize, Default)]
pub struct MessageBody {
    /// The model that will complete your prompt.
    /// See this link for additional details and options: https://docs.anthropic.com/claude/docs/models-overview
    pub model: String,
    /// Input messages.
    pub messages: Vec<Message>,
    /// The maximum number of tokens to generate before stopping.
    pub max_tokens: i32,
    /// An object describing metadata about the request.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub metadata: Option<HashMap<String, String>>,
    /// Custom text sequences that will cause the model to stop generating.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stop_sequences: Option<Vec<String>>,
    /// Whether to incrementally stream the response using server-sent events.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stream: Option<bool>,
    /// System prompt
    #[serde(skip_serializing_if = "Option::is_none")]
    pub system: Option<String>,
    /// Amount of randomness injected into the response.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub temerature: Option<f32>,
    /// Only sample from the top K options for each subsequent token.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub top_k: Option<f32>,
    /// Use nucleus sampling.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub top_p: Option<f32>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct MessageResponse {
    /// Unique object identifier.
    pub id: String,
    /// Object type.
    pub r#type: String,
    /// Conversational role of the generated message.
    pub role: String,
    /// Content generated by the model.
    pub content: Content,
    /// The model that handled the request.
    pub model: String,
    /// The reason that the model stopped.
    pub stop_reason: Option<String>,
    /// Which custom stop sequence was generated, if any.
    pub stop_sequence: Option<String>,
    /// Billing and rate-limit usage.
    pub usage: Usage,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct MessageEventResponse {
    /// Unique object identifier.
    pub id: String,
    /// Object type.
    pub r#type: String,
    /// Conversational role of the generated message.
    pub role: String,
    /// Content messages.
    pub content: Vec<Content>,
    /// The model that handled the request.
    pub model: String,
    /// The reason that the model stopped.
    pub stop_reason: Option<String>,
    /// Which custom stop sequence was generated, if any.
    pub stop_sequence: Option<String>,
    /// Billing and rate-limit usage.
    pub usage: Usage,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Delta {
    /// Determines the content shape.
    pub r#type: Option<String>,
    /// Response content
    pub text: Option<String>,
    pub stop_reason: Option<String>,
    pub end_turn: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, Default)]
#[serde(rename_all = "snake_case")]
pub enum MessageEventType {
    #[default]
    Error,
    MessageStart,
    MessageDelta,
    MessageStop,
    Ping,
    ContentBlockStart,
    ContentBlockDelta,
    ContentBlockStop,
    Comment,
}

#[derive(Debug, Serialize, Deserialize, Default)]
pub struct MessageEvent {
    /// Event type
    pub r#type: MessageEventType,
    /// Init message
    pub message: Option<MessageEventResponse>,
    /// Event index
    pub index: Option<i32>,
    /// Content block
    pub content_block: Option<Content>,
    /// Delta block
    pub delta: Option<Delta>,
    /// Usage
    pub usage: Option<Usage>,
    /// Comment
    pub comment: Option<String>,
}

impl MessageEvent {
    #[must_use] pub fn new(r#type: MessageEventType) -> Self {
        Self { r#type, ..Default::default() }
    }

    #[must_use] pub fn with_comment(comment: String) -> Self {
        Self { r#type: MessageEventType::Comment, comment: Some(comment), ..Default::default() }
    }
}

pub trait MessageApi {
    /// Create a Message
    fn message_create(&self, message_body: &MessageBody) -> ApiResult<MessageResponse>;
    fn message_stream(
        &self,
        message_body: &MessageBody,
    ) -> Result<impl Stream<Item = Result<MessageEvent, error::Error>>, error::Error>;
    fn message_delta_stream(
        &self,
        message_body: &MessageBody,
    ) -> Result<impl Stream<Item = Result<String, error::Error>>, error::Error>;
}

impl MessageApi for Anthropic {
    fn message_create(&self, message_body: &MessageBody) -> ApiResult<MessageResponse> {
        let request_body = serde_json::to_value(message_body).unwrap();
        let res = self.post(MESSAGES_CREATE, request_body)?;
        let response: MessageResponse = serde_json::from_value(res).unwrap();
        Ok(response)
    }

    fn message_stream(
        &self,
        message_body: &MessageBody,
    ) -> Result<impl Stream<Item = Result<MessageEvent, error::Error>>, error::Error> {
        log::debug!("message_body: {:#?}", message_body);

        let request_body = serde_json::to_value(message_body).unwrap();
        log::debug!("request_body: {:#?}", request_body);

        let original_stream = self.stream(MESSAGES_CREATE, request_body).map_err(|e| {
            log::error!("Error creating stream: {:#?}", e);
            error::Error::SseStreamCreation(Box::new(e))
        })?;

        let mapped_stream = original_stream.map(|item| {
            item.map(|event| match event {
                es::SSE::Event(ev) => match serde_json::from_str::<MessageEvent>(&ev.data) {
                    Ok(ev) => ev,
                    Err(e) => {
                        log::error!("Error parsing event: {:#?}", ev);
                        log::error!("Error: {:#?}", e);
                        let mut me = MessageEvent::new(MessageEventType::Error);
                        me.comment = Some(format!("fail to deserialize event: {}", ev.data));
                        me
                    }
                },
                es::SSE::Comment(comment) => MessageEvent::with_comment(comment),
            })
            .map_err(error::Error::from)
        });

        Ok(mapped_stream)
    }

    fn message_delta_stream(
        &self,
        message_body: &MessageBody,
    ) -> Result<impl Stream<Item = Result<String, error::Error>>, error::Error> {
        log::debug!("message_body: {:#?}", message_body);

        let request_body = serde_json::to_value(message_body).unwrap();
        log::debug!("request_body: {:#?}", request_body);

        let original_stream = self.stream(MESSAGES_CREATE, request_body).map_err(|e| {
            log::error!("Error creating stream: {:#?}", e);
            error::Error::SseStreamCreation(Box::new(e))
        })?;

        let mapped_stream = original_stream.map(|item| {
            item.map(|event| match event {
                es::SSE::Event(ev) => match serde_json::from_str::<MessageEvent>(&ev.data) {
                    Ok(ev) => {
                        if matches!(ev.r#type, MessageEventType::ContentBlockDelta) {
                            if let Some(delta) = ev.delta {
                                if let Some(text) = delta.text {
                                    text
                                } else {
                                    Default::default()
                                }
                            } else {
                                Default::default()
                            }
                        } else {
                            Default::default()
                        }
                    }
                    Err(e) => {
                        log::error!("Error parsing event: {:#?}", ev);
                        log::error!("Error: {:#?}", e);
                        Default::default()
                    }
                },
                es::SSE::Comment(comment) => {
                    log::debug!("Comment: {:#?}", comment);
                    Default::default()
                }
            })
            .map_err(error::Error::from)
        });

        Ok(mapped_stream)
    }
}
