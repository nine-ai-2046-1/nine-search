use std::collections::HashMap;
use crate::providers::{Provider, ParamDef, ParamType};

pub struct TavilyProvider;

impl Provider for TavilyProvider {
    fn name(&self) -> &str {
        "Tavily Search"
    }

    fn base_url(&self) -> &str {
        "https://api.tavily.com/search"
    }

    fn auth_header(&self, key: &str) -> String {
        format!("Authorization: Bearer {}", key)
    }

    fn valid_params(&self) -> HashMap<String, ParamDef> {
        let mut params = HashMap::new();

        params.insert("query".to_string(), ParamDef { param_type: ParamType::String, required: true });
        params.insert("search_depth".to_string(), ParamDef { param_type: ParamType::Enum, required: false });
        params.insert("chunks_per_source".to_string(), ParamDef { param_type: ParamType::Int, required: false });
        params.insert("max_results".to_string(), ParamDef { param_type: ParamType::Int, required: false });
        params.insert("topic".to_string(), ParamDef { param_type: ParamType::Enum, required: false });
        params.insert("time_range".to_string(), ParamDef { param_type: ParamType::Enum, required: false });
        params.insert("start_date".to_string(), ParamDef { param_type: ParamType::String, required: false });
        params.insert("end_date".to_string(), ParamDef { param_type: ParamType::String, required: false });
        params.insert("include_answer".to_string(), ParamDef { param_type: ParamType::Bool, required: false });
        params.insert("include_raw_content".to_string(), ParamDef { param_type: ParamType::Bool, required: false });
        params.insert("include_images".to_string(), ParamDef { param_type: ParamType::Bool, required: false });
        params.insert("include_image_descriptions".to_string(), ParamDef { param_type: ParamType::Bool, required: false });
        params.insert("include_favicon".to_string(), ParamDef { param_type: ParamType::Bool, required: false });
        params.insert("include_domains".to_string(), ParamDef { param_type: ParamType::Array, required: false });
        params.insert("exclude_domains".to_string(), ParamDef { param_type: ParamType::Array, required: false });
        params.insert("country".to_string(), ParamDef { param_type: ParamType::String, required: false });
        params.insert("auto_parameters".to_string(), ParamDef { param_type: ParamType::Bool, required: false });
        params.insert("exact_match".to_string(), ParamDef { param_type: ParamType::Bool, required: false });
        params.insert("include_usage".to_string(), ParamDef { param_type: ParamType::Bool, required: false });
        params.insert("safe_search".to_string(), ParamDef { param_type: ParamType::Bool, required: false });

        params
    }

    fn build_curl_args(&self, params: &HashMap<String, String>, key: &str) -> Vec<String> {
        let mut args = vec![
            "-s".to_string(),
            "-X".to_string(),
            "POST".to_string(),
            self.base_url().to_string(),
            "-H".to_string(),
            self.auth_header(key),
            "-H".to_string(),
            "Content-Type: application/json".to_string(),
        ];

        let mut body = serde_json::Map::new();
        for (k, v) in params {
            if let Ok(val) = serde_json::from_str::<serde_json::Value>(v) {
                body.insert(k.clone(), val);
            } else {
                body.insert(k.clone(), serde_json::Value::String(v.clone()));
            }
        }

        args.push("-d".to_string());
        args.push(serde_json::to_string(&body).unwrap_or_default());

        args
    }

    fn parse_response(&self, json: &str) -> Result<serde_json::Value, String> {
        serde_json::from_str(json).map_err(|e| format!("Failed to parse JSON: {}", e))
    }
}
