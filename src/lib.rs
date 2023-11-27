pub mod ai21;
pub mod amazon;
pub mod anthropic;
pub mod cohere;
pub mod meta;
pub mod stability;

use crate::ai21::AI21LabsModel;
use crate::amazon::AmazonModel;
use crate::anthropic::AnthropicModel;
use anyhow::Result;
use aws_sdk_bedrockruntime::operation::invoke_model::InvokeModelOutput;
use cohere::CohereModel;
use meta::MetaModel;
use serde::Deserialize;
use stability::StabilityAIModel;
use std::fmt::{Display, Formatter};

/// | Provider     | Model name                 | Version | Model Id                         |
/// |--------------|----------------------------|---------|----------------------------------|
/// | AI21 Labs    | Jurassic-2 Mid             | 1.x     | ai21.j2-mid-v1                   |
/// | AI21 Labs    | Jurassic-2 Ultra           | 1.x     | ai21.j2-ultra-v1                 |
/// | Amazon       | Titan Text G1 - Lite       | 1.x     | amazon.titan-text-lite-v1        |
/// | Amazon       | Titan Embeddings G1 - Text | 1.x     | amazon.titan-embed-text-v1       |
/// | Amazon       | Titan Text G1 - Express    | 1.x     | amazon.titan-text-express-v1     |
/// | Amazon       | Titan Text G1 - Agile      | 1.x     | amazon.titan-text-agile-v1       |
/// | Anthropic    | Claude                     | 1.x     | anthropic.claude-v1              |
/// | Anthropic    | Claude                     | 2.x     | anthropic.claude-v2              |
/// | Anthropic    | Claude Instant             | 1.x     | anthropic.claude-instant-v1      |
/// | Cohere       | Command                    | 14.x    | cohere.command-text-v14          |
/// | Cohere       | Command Light              | 15.x    | cohere.command-light-text-v14    |
/// | Cohere       | Embed English              | 3.x     | cohere.embed-english-v3          |
/// | Cohere       | Embed Multilingual         | 3.x     | cohere.embed-multilingual-v3     |
/// | Meta         | Llama 2 Chat 13B           | 1.x     | meta.llama2-13b-chat-v1          |
/// | Stability AI | Stable Diffusion XL        | 0.x     | stability.stable-diffusion-xl-v0 |

pub enum BaseModel {
    AI21Labs(AI21LabsModel),
    Amazon(AmazonModel),
    Anthropic(AnthropicModel),
    Cohere(CohereModel),
    Meta(MetaModel),
    StabilityAI(StabilityAIModel),
}

#[derive(PartialEq)]
pub enum ModelVersion {
    V0,
    V1,
    V2,
    V3,
    V14,
    V15,
}

impl Display for BaseModel {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let id: String = match self {
            BaseModel::AI21Labs(model) => format!("ai21.{model}"),
            BaseModel::Amazon(model) => format!("amazon.{model}"),
            BaseModel::Anthropic(model) => format!("anthropic.{model}"),
            BaseModel::Cohere(model) => format!("cohere.{model}"),
            BaseModel::Meta(model) => format!("meta.{model}"),
            BaseModel::StabilityAI(model) => format!("stability.{model}"),
        };
        write!(f, "{id}")
    }
}

pub trait FromModelOutput<'de, T>
where
    T: Deserialize<'de>,
{
    fn from_model_output(output: &'de InvokeModelOutput) -> Result<T> {
        let res = std::str::from_utf8(output.body.as_ref())?;
        Ok(serde_json::from_str(res)?)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::ai21::AI21LabsModel;
    use crate::amazon::AmazonModel;
    use crate::anthropic::AnthropicModel;
    use crate::ModelVersion::{V0, V1, V14, V15, V2, V3};

    // Unit tests for ever single model version, in order
    #[test]
    fn test_base_model_to_string() {
        let model = BaseModel::AI21Labs(AI21LabsModel::Jurassic2Mid(V1));
        assert_eq!(model.to_string(), "ai21.j2-mid-v1");

        let model = BaseModel::AI21Labs(AI21LabsModel::Jurassic2Ultra(V1));
        assert_eq!(model.to_string(), "ai21.j2-ultra-v1");

        let model = BaseModel::Amazon(AmazonModel::TitanTextLite(V1));
        assert_eq!(model.to_string(), "amazon.titan-text-lite-v1");

        let model = BaseModel::Amazon(AmazonModel::TitanEmbeddingsText(V1));
        assert_eq!(model.to_string(), "amazon.titan-embed-text-v1");

        let model = BaseModel::Amazon(AmazonModel::TitanTextExpress(V1));
        assert_eq!(model.to_string(), "amazon.titan-text-express-v1");

        let model = BaseModel::Amazon(AmazonModel::TitanTextAgile(V1));
        assert_eq!(model.to_string(), "amazon.titan-text-agile-v1");

        let model = BaseModel::Anthropic(AnthropicModel::Claude(V1));
        assert_eq!(model.to_string(), "anthropic.claude-v1");

        let model = BaseModel::Anthropic(AnthropicModel::Claude(V2));
        assert_eq!(model.to_string(), "anthropic.claude-v2");

        let model = BaseModel::Anthropic(AnthropicModel::ClaudeInstant(V1));
        assert_eq!(model.to_string(), "anthropic.claude-instant-v1");

        let model = BaseModel::Cohere(CohereModel::Command(V14));
        assert_eq!(model.to_string(), "cohere.command-text-v14");

        let model = BaseModel::Cohere(CohereModel::CommandLight(V15));
        assert_eq!(model.to_string(), "cohere.command-light-text-v14");

        let model = BaseModel::Cohere(CohereModel::EmbedEnglish(V3));
        assert_eq!(model.to_string(), "cohere.embed-english-v3");

        let model = BaseModel::Cohere(CohereModel::EmbedMultilingual(V3));
        assert_eq!(model.to_string(), "cohere.embed-multilingual-v3");

        let model = BaseModel::Meta(MetaModel::Llama2Chat13B(V1));
        assert_eq!(model.to_string(), "meta.llama2-13b-chat-v1");

        let model = BaseModel::StabilityAI(StabilityAIModel::StableDiffusionXL(V0));
        assert_eq!(model.to_string(), "stability.stable-diffusion-xl-v0");
    }
}
