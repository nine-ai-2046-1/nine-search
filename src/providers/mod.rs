pub mod tavily;

use std::collections::HashMap;

#[derive(Debug, Clone, PartialEq)]
pub enum ParamType {
    String,
    Int,
    Bool,
    Enum,
    Array,
}

#[derive(Debug, Clone)]
pub struct ParamDef {
    pub param_type: ParamType,
    pub required: bool,
}

pub trait Provider {
    fn name(&self) -> &str;
    fn base_url(&self) -> &str;
    fn auth_header(&self, key: &str) -> String;
    fn valid_params(&self) -> HashMap<String, ParamDef>;
    fn build_curl_args(&self, params: &HashMap<String, String>, key: &str) -> Vec<String>;
    fn parse_response(&self, json: &str) -> Result<serde_json::Value, String>;
}

pub fn build_registry() -> HashMap<String, Box<dyn Provider>> {
    let mut registry: HashMap<String, Box<dyn Provider>> = HashMap::new();
    registry.insert("tavily".to_string(), Box::new(crate::providers::tavily::TavilyProvider));
    registry
}
