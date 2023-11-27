# `stone-mason`

![](https://github.com/nated0g/hematite/actions/workflows/rust.yml/badge.svg)

`stone-mason` is a Rust library to simplify using the [Amazon Bedrock](https://aws.amazon.com/bedrock/) Rust SDK 
[aws-sdk-bedrockruntime](https://docs.rs/aws-sdk-bedrockruntime/latest/aws_sdk_bedrockruntime/).

This library is still very early in its development, much of it has not been properly tested.

## Features

- Builder structs for generating request bodies for each of the base models offered in Bedrock 
- Structs for deserialization of inference responses (WIP)
- Enums encoding model ids for all the models

## Installation

TODO

## Usage

You can run the following example with the following command:

```bash
cargo run --example anthropic
```

> [!NOTE]
> You'll need valid AWS credentials with at least `bedrock:InvokeModel` permissions in your environment, as well as to
> have configured [model access](https://docs.aws.amazon.com/bedrock/latest/userguide/model-access.html) for the particular 
> model you're trying to use.


```rust
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

    let prompt = "Outline a README.md file for an open source library called stone-mason, which \
    is for working with Amazon Bedrock in Rust.";

    let formatted_prompt = format!("\n\nHuman: {}\n\nAssistant:", prompt);

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
```

## Documentation

TODO

## Contributing

TODO

## License

This project is licensed under the MIT license. See [LICENSE](LICENSE) for more details.
