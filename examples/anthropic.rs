use aws_sdk_bedrockruntime::primitives::Blob;
use aws_sdk_bedrockruntime::Client;

use stone_mason::{
    anthropic::{AnthropicModel::Claude, AnthropicParamsBuilder, AnthropicResponse},
    BaseModel, FromModelOutput,
    ModelVersion::*,
};

#[tokio::main]
async fn main() {
    let shared_config = aws_config::from_env().region("us-west-2").load().await;

    let client = Client::new(&shared_config);

    let model = BaseModel::Anthropic(Claude(V2));

    let prompt = "Outline a README.md file for an open source library called Hematite, which \
    is a tool for working with Amazon Bedrock in Rust.";

    let formatted_prompt = format!("\n\nHuman: {prompt}\n\nAssistant:");

    println!("{}", formatted_prompt);

    let params = AnthropicParamsBuilder::default()
        .prompt(formatted_prompt)
        .temperature(0.3)
        .max_tokens_to_sample(1000)
        .build()
        .unwrap();

    let body_str = serde_json::to_string(&params).unwrap();

    let body = Blob::new(body_str);

    let res = client
        .invoke_model()
        .model_id(model.to_string())
        .content_type("application/json")
        .body(body)
        .send()
        .await
        .unwrap();

    let parsed = AnthropicResponse::from_model_output(&res).unwrap();

    println!("\n\n{}", parsed.completion)
}
