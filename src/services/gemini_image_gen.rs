use base64::{engine::general_purpose, Engine as _};
use reqwest::Client;
use std::env;

use thiserror::Error;

use crate::types::gemini::{GeminiImageGenRequest, GeminiImageGenRequestInstance, GeminiImageGenRequestParameters, GeminiImageGenResponse, GeneratedImage};

#[derive(Error, Debug)]
pub enum ImageGeneratorError {
    #[error("API request failed: {0}")]
    RequestError(#[from] reqwest::Error),

    #[error("Failed to decode base64 image: {0}")]
    Base64DecodeError(#[from] base64::DecodeError),

    #[error("Environment variable not found: {0}")]
    EnvVarError(#[from] std::env::VarError),

    #[error("No image generated in response")]
    NoImageGenerated,

    #[error("Invalid API response: {0}")]
    InvalidResponse(String),
}


pub struct ImageGenerator {
    client: Client,
    prompt: String,
    url: String,

}

impl ImageGenerator {
    pub fn new(model: &str, prompt: &str) -> Result<Self, ImageGeneratorError> {
        let api_key = env::var("GOOGLE_API_KEY")?;
        
        let client = Client::builder()
            .default_headers({
                let mut headers = reqwest::header::HeaderMap::new();
                headers.insert(
                    "Content-Type",
                    "application/json".parse().unwrap(),
                );
                headers
            })
            .build()?;

        Ok(Self {
            client,
            prompt: prompt.to_string(),
            // url: format!("https://generativelanguage.googleapis.com/v1beta/models/{}:generateContent?key={}", model, api_key),
            url: format!("https://generativelanguage.googleapis.com/v1beta/models/{}:predict?key={}", model, api_key),
        })
    }

    pub async fn generate(&self) -> Result<GeneratedImage, ImageGeneratorError> {
        let api_url = &self.url;

        let request = GeminiImageGenRequest {
            instances: vec![GeminiImageGenRequestInstance {
                prompt: self.prompt.clone(),
            }],
            parameters: GeminiImageGenRequestParameters {
                sample_count: 1,
                aspect_ratio: "16:9".to_string(),
            },
        };

        println!("{:?}", serde_json::to_string(&request).unwrap());

        let response = self.client.post(api_url).json(&request).send().await?;
        let gemini_response: GeminiImageGenResponse = response.json().await?;

        println!("{:?}", gemini_response);

        let predictions = gemini_response
            .predictions
            .first()
            .ok_or_else(|| ImageGeneratorError::InvalidResponse("No predictions found".to_string()))?;

        let mime_type = predictions.mime_type.clone();
        let image_data = general_purpose::STANDARD.decode(&predictions.bytes_base64encoded)?;

        Ok(GeneratedImage {
            image_data,
            mime_type,
        })
    }
}
