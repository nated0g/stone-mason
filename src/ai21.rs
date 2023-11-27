use std::fmt::{Display, Error, Formatter};
use derive_builder::Builder;
use serde::Serialize;
use crate::ModelVersion;

pub enum AI21LabsModel {
    Jurassic2Mid(ModelVersion),
    Jurassic2Ultra(ModelVersion),
}

impl Display for AI21LabsModel {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            AI21LabsModel::Jurassic2Mid(v) if *v == ModelVersion::V1 => write!(f, "j2-mid-v1"),
            AI21LabsModel::Jurassic2Ultra(v) if *v == ModelVersion::V1 => write!(f, "j2-ultra-v1"),
            _ => Err(Error),
        }
    }
}

#[derive(Serialize, Builder, Clone, Debug)]
#[builder(setter(strip_option))]
pub struct AI21InferenceParameters {
    prompt: String,

    #[builder(default = "None")]
    #[serde(skip_serializing_if = "Option::is_none")]
    temperature: Option<f32>,

    #[builder(default = "None")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename(serialize = "numResults"))]
    num_results: Option<u32>,

    #[builder(default = "None")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename(serialize = "topP"))]
    top_p: Option<f32>,

    #[builder(default = "None")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename(serialize = "topKReturn"))]
    top_k_return: Option<u32>,

    #[builder(default = "None")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename(serialize = "maxTokens"))]
    max_tokens: Option<u32>,

    #[builder(default = "None")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename(serialize = "minTokens"))]
    min_tokens: Option<u32>,

    #[builder(default = "None")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename(serialize = "stopSequences"))]
    stop_sequences: Option<Vec<String>>,

    #[builder(default = "None")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename(serialize = "countPenalty"))]
    count_penalty: Option<CountPenalty>,

    #[builder(default = "None")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename(serialize = "presencePenalty"))]
    presence_penalty: Option<PresencePenalty>,

    #[builder(default = "None")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename(serialize = "frequencyPenalty"))]
    frequency_penalty: Option<FrequencyPenalty>,
}


#[derive(Serialize, Builder, Clone, Debug, Copy)]
#[builder(setter(strip_option))]
pub struct CountPenalty {
    scale: f32,

    #[builder(default = "None")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename(serialize = "applyToWhitespaces"))]
    apply_to_whitespaces: Option<bool>,

    #[builder(default = "None")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename(serialize = "applyToPunctuations"))]
    apply_to_punctuations: Option<bool>,

    #[builder(default = "None")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename(serialize = "applyToNumbers"))]
    apply_to_numbers: Option<bool>,

    #[builder(default = "None")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename(serialize = "applyToStopwords"))]
    apply_to_stopwords: Option<bool>,

    #[builder(default = "None")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename(serialize = "applyToEmojis"))]
    apply_to_emojis: Option<bool>,
}

#[derive(Serialize, Builder, Clone, Debug, Copy)]
#[builder(setter(strip_option))]
pub struct PresencePenalty {
    scale: f32,

    #[builder(default = "None")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename(serialize = "applyToWhitespaces"))]
    apply_to_whitespaces: Option<bool>,

    #[builder(default = "None")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename(serialize = "applyToPunctuations"))]
    apply_to_punctuations: Option<bool>,

    #[builder(default = "None")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename(serialize = "applyToNumbers"))]
    apply_to_numbers: Option<bool>,

    #[builder(default = "None")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename(serialize = "applyToStopwords"))]
    apply_to_stopwords: Option<bool>,

    #[builder(default = "None")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename(serialize = "applyToEmojis"))]
    apply_to_emojis: Option<bool>,
}

#[derive(Serialize, Builder, Clone, Debug, Copy)]
#[builder(setter(strip_option))]
pub struct FrequencyPenalty {
    scale: f32,

    #[builder(default = "None")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename(serialize = "applyToWhitespaces"))]
    apply_to_whitespaces: Option<bool>,

    #[builder(default = "None")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename(serialize = "applyToPunctuations"))]
    apply_to_punctuations: Option<bool>,

    #[builder(default = "None")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename(serialize = "applyToNumbers"))]
    apply_to_numbers: Option<bool>,

    #[builder(default = "None")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename(serialize = "applyToStopwords"))]
    apply_to_stopwords: Option<bool>,

    #[builder(default = "None")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename(serialize = "applyToEmojis"))]
    apply_to_emojis: Option<bool>,
}
