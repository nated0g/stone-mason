use crate::ModelVersion;
use crate::ModelVersion::V1;
use derive_builder::Builder;
use serde::Serialize;
use std::fmt::{Display, Error, Formatter};

pub enum MetaModel {
    Llama2Chat13B(ModelVersion),
}

impl Display for MetaModel {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            MetaModel::Llama2Chat13B(v) if *v == V1 => write!(f, "llama2-13b-chat-v1"),
            _ => Err(Error),
        }
    }
}

#[derive(Serialize, Builder)]
#[builder(setter(strip_option))]
pub struct MetaParams {
    pub prompt: String,

    #[builder(default = "None")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub temperature: Option<f32>,

    #[builder(default = "None")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub top_p: Option<f32>,

    #[builder(default = "None")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_gen_len: Option<i32>,
}
