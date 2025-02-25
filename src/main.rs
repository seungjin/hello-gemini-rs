// main.rs
use anyhow::Result;
use base64;
use reqwest::{
    Client, Error,
    header::{CONTENT_TYPE, HeaderMap, HeaderValue},
};
use serde_json::{Value, json};
use std::fs;

// What I am trying to do;
//
// curl -X POST \
//  -H "Content-Type: application/json" \
//  -H "x-goog-api-key: YOUR_API_KEY" \
//  -H "x-goog-generativeai-api-version: v1beta" \
//  -d @request.json \
//  "https://generativelanguage.googleapis.com/v1beta/models/gemini-pro-vision:generateContent"

// struct RequestBody {}

#[tokio::main]
async fn main() -> Result<()> {
    let api_key = "AIzaSyDFw0KyRt7R3PvPmwJbuwMpWpK30RR-iyQ"; // Replace with your API key
    let image_path = "/var/home/seungjin/Downloads/e5860900baa40d24.jpg"; // Replace with your image path
    let prompt = "Two sentences image description for ALT text";

    let base64_image = encode_image_to_base64(image_path)?;

    let client = Client::new();

    let mut headers = HeaderMap::new();
    headers.insert(CONTENT_TYPE, HeaderValue::from_static("application/json"));
    headers.insert("x-goog-api-key", HeaderValue::from_str(api_key).unwrap());

    let body = json!({
         "contents": [
            {
                "parts": [
                    {
                        "inlineData": {
                            "mimeType": "image/jpeg",
                            "data": base64_image,
                        }
                    },
                    {
                        "text": prompt,
                    }
                ]
            }
        ]
    });

    let response = client.post("https://generativelanguage.googleapis.com/v1beta/models/gemini-2.0-flash:generateContent").headers(headers).json(&body).send().await?;

    if response.status().is_success() {
        let json_response: Value = response.json().await?;
        println!("{}", serde_json::to_string_pretty(&json_response).unwrap());
    } else {
        println!("Error: {:?}", response.status());
        let error_body = response.text().await?;
        println!("Response body: {}", error_body);
    }

    Ok(())
}

fn encode_image_to_base64(image_path: &str) -> Result<String> {
    let image_bytes = fs::read(image_path)?;
    let base64_image = base64::encode(&image_bytes);
    Ok(base64_image)
}
