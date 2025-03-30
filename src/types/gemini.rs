use base64::{engine::general_purpose, Engine as _};
use serde::{Deserialize, Serialize};

// Request
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GeminiImageGenRequest {
    pub instances: Vec<GeminiImageGenRequestInstance>,
    pub parameters: GeminiImageGenRequestParameters,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GeminiImageGenRequestInstance {
    pub prompt: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GeminiImageGenRequestParameters {
    pub sample_count: i64,
    pub aspect_ratio: String
}


// Response
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GeminiImageGenResponse {
    pub predictions: Vec<GeminiImageGenResponsePrediction>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GeminiImageGenResponsePrediction {
    pub mime_type: String,
    #[serde(rename = "bytesBase64Encoded")]
    pub bytes_base64encoded: String,
}


// Response
#[derive(Debug)]
pub struct GeneratedImage {
    pub image_data: Vec<u8>,
    pub mime_type: String,
}

impl GeneratedImage {
    /// Creates a data URL suitable for use in HTML img tags or CSS
    pub fn to_data_url(&self) -> String {
        let base64_data = base64::engine::general_purpose::STANDARD.encode(&self.image_data);
        format!("data:{};base64,{}", self.mime_type, base64_data)
    }
}
