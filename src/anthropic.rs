use crate::ModelVersion::{V1, V2};
use crate::{FromModelOutput, ModelVersion};
use derive_builder::Builder;
use serde::{Deserialize, Serialize};
use std::fmt::{Display, Error, Formatter};

pub enum AnthropicModel {
    Claude(ModelVersion),
    ClaudeInstant(ModelVersion),
}

impl Display for AnthropicModel {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            AnthropicModel::Claude(v) if *v == V1 => write!(f, "claude-v1"),
            AnthropicModel::Claude(v) if *v == V2 => write!(f, "claude-v2"),
            AnthropicModel::ClaudeInstant(v) if *v == V1 => write!(f, "claude-instant-v1"),
            _ => Err(Error),
        }
    }
}

#[derive(Serialize, Builder)]
#[builder(setter(strip_option))]
pub struct AnthropicParams {
    prompt: String,
    max_tokens_to_sample: u32,

    #[builder(default = "None")]
    #[serde(skip_serializing_if = "Option::is_none")]
    temperature: Option<f32>,

    #[builder(default = "None")]
    #[serde(skip_serializing_if = "Option::is_none")]
    stop_sequences: Option<Vec<String>>,

    #[builder(default = "None")]
    #[serde(skip_serializing_if = "Option::is_none")]
    top_p: Option<f32>,

    #[builder(default = "None")]
    #[serde(skip_serializing_if = "Option::is_none")]
    top_k: Option<u32>,
}

#[derive(Deserialize, Debug)]
pub struct AnthropicResponse {
    pub completion: String,
    pub stop_reason: String,
    pub stop: String,
}

impl<'de> FromModelOutput<'de, AnthropicResponse> for AnthropicResponse {}
