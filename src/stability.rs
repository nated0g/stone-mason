use crate::ModelVersion;
use crate::ModelVersion::V0;
use derive_builder::Builder;
use serde::Serialize;
use std::fmt::{Display, Error, Formatter};

pub enum StabilityAIModel {
    StableDiffusionXL(ModelVersion),
}

impl Display for StabilityAIModel {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            StabilityAIModel::StableDiffusionXL(v) if *v == V0 => {
                write!(f, "stable-diffusion-xl-v0")
            }
            _ => Err(Error),
        }
    }
}

#[derive(Builder, Debug, Clone, Serialize)]
#[builder(setter(strip_option))]
pub struct StabilityParams {
    text_prompts: Vec<TextPrompt>,

    #[builder(default = "None")]
    #[serde(skip_serializing_if = "Option::is_none")]
    cfg_scale: Option<f32>,

    #[builder(default = "None")]
    #[serde(skip_serializing_if = "Option::is_none")]
    steps: Option<i32>,

    #[builder(default = "None")]
    #[serde(skip_serializing_if = "Option::is_none")]
    seed: Option<i32>,
}

#[derive(Builder, Debug, Clone, Serialize)]
pub struct TextPrompt {
    text: String,
    weight: Option<f32>,
}
