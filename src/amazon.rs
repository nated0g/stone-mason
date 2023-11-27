use crate::ModelVersion;
use crate::ModelVersion::V1;
use derive_builder::Builder;
use serde::Serialize;
use std::fmt::{Display, Error, Formatter};

pub enum AmazonModel {
    TitanTextLite(ModelVersion),
    TitanEmbeddingsText(ModelVersion),
    TitanTextExpress(ModelVersion),
    TitanTextAgile(ModelVersion),
}

impl Display for AmazonModel {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            AmazonModel::TitanTextLite(v) if *v == V1 => write!(f, "titan-text-lite-v1"),
            AmazonModel::TitanEmbeddingsText(v) if *v == V1 => write!(f, "titan-embed-text-v1"),
            AmazonModel::TitanTextExpress(v) if *v == V1 => write!(f, "titan-text-express-v1"),
            AmazonModel::TitanTextAgile(v) if *v == V1 => write!(f, "titan-text-agile-v1"),
            _ => Err(Error),
        }
    }
}

#[derive(Serialize, Builder, Clone, Debug)]
pub struct AmazonParams {
    #[serde(rename(serialize = "inputText"))]
    input_text: String,

    #[serde(rename(serialize = "textGenerationConfig"))]
    text_generation_config: TextGenerationConfig,
}

#[derive(Serialize, Builder, Clone, Debug)]
#[builder(setter(strip_option))]
pub struct TextGenerationConfig {
    #[builder(default = "None")]
    #[serde(skip_serializing_if = "Option::is_none")]
    temperature: Option<f32>,

    #[builder(default = "None")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename(serialize = "topP"))]
    top_p: Option<f32>,

    #[builder(default = "None")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename(serialize = "maxTokenCount"))]
    max_token_count: Option<u32>,

    #[serde(rename(serialize = "stopSequences"))]
    stop_sequences: Vec<String>,
}
