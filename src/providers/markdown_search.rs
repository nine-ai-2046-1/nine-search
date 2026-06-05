use std::collections::HashMap;
use crate::providers::{Provider, ParamDef, ParamType};

pub struct MarkdownSearchProvider;

impl Provider for MarkdownSearchProvider {
    fn name(&self) -> &str {
        "Markdown Search"
    }

    fn base_url(&self) -> &str {
        "https://markdown.new/search"
    }

    fn auth_header(&self, _key: &str) -> String {
        String::new()
    }

    fn valid_params(&self) -> HashMap<String, ParamDef> {
        let mut params = HashMap::new();

        params.insert("query".to_string(), ParamDef { param_type: ParamType::String, required: true });
        params.insert("n".to_string(), ParamDef { param_type: ParamType::Int, required: false });
        params.insert("gl".to_string(), ParamDef { param_type: ParamType::String, required: false });
        params.insert("hl".to_string(), ParamDef { param_type: ParamType::String, required: false });
        params.insert("format".to_string(), ParamDef { param_type: ParamType::String, required: false });
        params.insert("retain_images".to_string(), ParamDef { param_type: ParamType::Bool, required: false });

        params
    }

    fn build_curl_args(&self, params: &HashMap<String, String>, _key: &str) -> Vec<String> {
        let mut args = vec![
            "-s".to_string(),
            "-X".to_string(),
            "POST".to_string(),
            self.base_url().to_string(),
            "-H".to_string(),
            "Content-Type: application/json".to_string(),
        ];

        let mut body = serde_json::Map::new();

        if let Some(query) = params.get("query") {
            body.insert("q".to_string(), serde_json::Value::String(query.clone()));
        }

        if let Some(n) = params.get("n") {
            if let Ok(val) = n.parse::<i64>() {
                body.insert("n".to_string(), serde_json::Value::Number(val.into()));
            }
        }

        if let Some(gl) = params.get("gl") {
            body.insert("gl".to_string(), serde_json::Value::String(gl.clone()));
        }

        if let Some(hl) = params.get("hl") {
            body.insert("hl".to_string(), serde_json::Value::String(hl.clone()));
        }

        if let Some(format) = params.get("format") {
            body.insert("format".to_string(), serde_json::Value::String(format.clone()));
        }

        if let Some(retain_images) = params.get("retain_images") {
            if let Ok(val) = retain_images.parse::<bool>() {
                body.insert("retain_images".to_string(), serde_json::Value::Bool(val));
            }
        }

        args.push("-d".to_string());
        args.push(serde_json::to_string(&body).unwrap_or_default());

        args
    }

    fn parse_response(&self, json: &str) -> Result<serde_json::Value, String> {
        serde_json::from_str(json).map_err(|e| format!("Invalid response from markdown.new: {}", e))
    }
}

pub fn handle_http_error(status_code: u16) {
    match status_code {
        429 => {
            eprintln!("Error: Rate limit exceeded.");
            eprintln!("markdown.new allows 30 searches/min, 500/day per IP.");
            eprintln!("Wait a moment or try again later.");
        }
        500..=599 => {
            eprintln!("Error: markdown.new server error (HTTP {})", status_code);
        }
        _ => {
            eprintln!("Error: HTTP {}", status_code);
        }
    }
    std::process::exit(1);
}
