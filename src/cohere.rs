use crate::ModelVersion;
use crate::ModelVersion::{V14, V15, V3};
use derive_builder::Builder;
use serde::Serialize;
use std::collections::HashMap;
use std::fmt::{Display, Error, Formatter};

pub enum CohereModel {
    Command(ModelVersion),
    CommandLight(ModelVersion),
    EmbedEnglish(ModelVersion),
    EmbedMultilingual(ModelVersion),
}

impl Display for CohereModel {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            CohereModel::Command(v) if *v == V14 => write!(f, "command-text-v14"),
            CohereModel::CommandLight(v) if *v == V15 => write!(f, "command-light-text-v14"),
            CohereModel::EmbedEnglish(v) if *v == V3 => write!(f, "embed-english-v3"),
            CohereModel::EmbedMultilingual(v) if *v == V3 => write!(f, "embed-multilingual-v3"),
            _ => Err(Error),
        }
    }
}

#[derive(Serialize, Builder)]
#[builder(setter(strip_option))]
pub struct CohereParams {
    prompt: String,

    #[builder(default = "None")]
    #[serde(skip_serializing_if = "Option::is_none")]
    temperature: Option<f32>,
    #[builder(default = "None")]
    #[serde(skip_serializing_if = "Option::is_none")]
    p: Option<f32>,
    #[builder(default = "None")]
    #[serde(skip_serializing_if = "Option::is_none")]
    k: Option<f32>,

    #[builder(default = "None")]
    #[serde(skip_serializing_if = "Option::is_none")]
    max_tokens: Option<i32>,

    #[builder(default = "None")]
    #[serde(skip_serializing_if = "Option::is_none")]
    stop_sequences: Option<Vec<String>>,

    #[builder(default = "None")]
    #[serde(skip_serializing_if = "Option::is_none")]
    return_likelihoods: Option<ReturnLikelihoods>,

    #[builder(default = "None")]
    #[serde(skip_serializing_if = "Option::is_none")]
    stream: Option<bool>,

    #[builder(default = "None")]
    #[serde(skip_serializing_if = "Option::is_none")]
    num_generations: Option<i32>,

    #[builder(default = "None")]
    #[serde(skip_serializing_if = "Option::is_none")]
    logit_bias: Option<HashMap<i32, f32>>,

    #[builder(default = "None")]
    #[serde(skip_serializing_if = "Option::is_none")]
    truncate: Option<Truncate>,
}

#[derive(Serialize, Clone)]
pub enum ReturnLikelihoods {
    Generation,
    All,
    None,
}

#[derive(Serialize, Clone)]
pub enum Truncate {
    None,
    Start,
    End,
}
