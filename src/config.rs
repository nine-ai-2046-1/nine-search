use serde::Deserialize;
use std::collections::HashMap;
use std::fs;
use std::path::PathBuf;

const DEFAULT_CONFIG: &str = r#"# nine-search configuration

default_provider = "markdown-search"

[[providers]]
id = "tavily"
name = "Tavily Search"
key = ""

[[providers]]
id = "markdown-search"
name = "Markdown Search"
key = ""

[tavily]
# 🔍 搜尋深度：basic(快) / advanced(準) / fast / ultra-fast
search_depth = "basic"
# 📄 每個來源返回嘅內容片段數量（只限 advanced，範圍 1-3）
chunks_per_source = 3
# 🔢 最多返回幾多個搜尋結果（範圍 0-20）
max_results = 5
# 📂 搜尋類別：general(一般) / news(新聞) / finance(財經)
topic = "general"
# ⏰ 時間範圍：day / week / month / year（留空=全部）
time_range = ""
# 📅 開始日期（YYYY-MM-DD 格式，留空=唔限）
start_date = ""
# 📅 結束日期（YYYY-MM-DD 格式，留空=唔限）
end_date = ""
# 💡 要唔要返回 AI 生成嘅答案摘要
include_answer = false
# 📝 要唔要返回網頁原始內容
include_raw_content = false
# 🖼️ 要唔要返回相關圖片
include_images = false
# 🏷️ 要唔要返回圖片描述（需要 include_images=true）
include_image_descriptions = false
# 🔎 要唔要返回網站 favicon URL
include_favicon = false
# ✅ 只搜尋呢啲網站（例如：["github.com"]）
include_domains = []
# 🚫 唔搜尋呢啲網站
exclude_domains = []
# 🌍 指定國家提升搜尋結果排名（只限 general）
country = ""
# 🤖 自動調整搜尋參數（用 2 API credits）
auto_parameters = false
# 🎯 精確匹配搜尋（用引號包住嘅詞語）
exact_match = false
# 💰 要唔要返回 API 用量資訊
include_usage = false
# 🔒 過濾成人內容（Enterprise only）
safe_search = false

# 🆕 免費搜尋引擎，返回完整 Markdown 內容（適合 RAG）
# ⚠️ Rate limit: 30次/分鐘, 500次/日 (per IP)
[markdown-search]
# 🔢 搜尋結果數量 (1-5)
n = 5
# 🌍 搜尋地區 (us, uk, de, jp, hk...)
gl = "hk"
# 🗣️ 搜尋語言 (en, zh, ja...)
hl = "zh"
# 📄 回傳格式 (固定 json，唔好改)
format = "json"
# 🖼️ 要唔要保留圖片 Markdown
retain_images = true
"#;

#[derive(Debug, Deserialize, Default)]
pub struct TavilyConfig {
    pub search_depth: Option<String>,
    pub chunks_per_source: Option<i64>,
    pub max_results: Option<i64>,
    pub topic: Option<String>,
    pub time_range: Option<String>,
    pub start_date: Option<String>,
    pub end_date: Option<String>,
    pub include_answer: Option<bool>,
    pub include_raw_content: Option<bool>,
    pub include_images: Option<bool>,
    pub include_image_descriptions: Option<bool>,
    pub include_favicon: Option<bool>,
    pub include_domains: Option<Vec<String>>,
    pub exclude_domains: Option<Vec<String>>,
    pub country: Option<String>,
    pub auto_parameters: Option<bool>,
    pub exact_match: Option<bool>,
    pub include_usage: Option<bool>,
    pub safe_search: Option<bool>,
}

#[derive(Debug, Deserialize, Default)]
pub struct MarkdownSearchConfig {
    pub n: Option<i64>,
    pub gl: Option<String>,
    pub hl: Option<String>,
    pub format: Option<String>,
    pub retain_images: Option<bool>,
}

#[derive(Debug, Deserialize)]
pub struct Config {
    pub default_provider: String,
    pub providers: Vec<ProviderConfig>,
    pub tavily: Option<TavilyConfig>,
    pub markdown_search: Option<MarkdownSearchConfig>,
}

#[derive(Debug, Deserialize)]
pub struct ProviderConfig {
    pub id: String,
    pub name: String,
    pub key: String,
}

pub fn config_path() -> PathBuf {
    let home = std::env::var("HOME").expect("HOME environment variable not set");
    PathBuf::from(home).join(".config").join("nine-search").join("config.toml")
}

pub fn load_or_create() -> Config {
    let path = config_path();

    if !path.exists() {
        let parent = path.parent().expect("Failed to get config parent directory");
        fs::create_dir_all(parent).expect("Failed to create config directory");
        fs::write(&path, DEFAULT_CONFIG).expect("Failed to write default config");
    }

    let content = fs::read_to_string(&path).expect("Failed to read config file");
    let config: Config = toml::from_str(&content).expect("Failed to parse config file");
    config
}

pub fn validate_api_key(config: &Config) -> Option<&ProviderConfig> {
    let provider = config
        .providers
        .iter()
        .find(|p| p.id == config.default_provider);

    match provider {
        Some(p) if !p.key.is_empty() => Some(p),
        _ => None,
    }
}

pub fn tavily_defaults_to_hashmap(tavily: &Option<TavilyConfig>) -> HashMap<String, String> {
    let mut params = HashMap::new();

    if let Some(t) = tavily {
        if let Some(v) = &t.search_depth { params.insert("search_depth".to_string(), v.clone()); }
        if let Some(v) = t.chunks_per_source { params.insert("chunks_per_source".to_string(), v.to_string()); }
        if let Some(v) = t.max_results { params.insert("max_results".to_string(), v.to_string()); }
        if let Some(v) = &t.topic { params.insert("topic".to_string(), v.clone()); }
        if let Some(v) = &t.time_range { if !v.is_empty() { params.insert("time_range".to_string(), v.clone()); } }
        if let Some(v) = &t.start_date { if !v.is_empty() { params.insert("start_date".to_string(), v.clone()); } }
        if let Some(v) = &t.end_date { if !v.is_empty() { params.insert("end_date".to_string(), v.clone()); } }
        if let Some(v) = t.include_answer { params.insert("include_answer".to_string(), v.to_string()); }
        if let Some(v) = t.include_raw_content { params.insert("include_raw_content".to_string(), v.to_string()); }
        if let Some(v) = t.include_images { params.insert("include_images".to_string(), v.to_string()); }
        if let Some(v) = t.include_image_descriptions { params.insert("include_image_descriptions".to_string(), v.to_string()); }
        if let Some(v) = t.include_favicon { params.insert("include_favicon".to_string(), v.to_string()); }
        if let Some(v) = &t.include_domains { if !v.is_empty() { params.insert("include_domains".to_string(), serde_json::to_string(v).unwrap_or_default()); } }
        if let Some(v) = &t.exclude_domains { if !v.is_empty() { params.insert("exclude_domains".to_string(), serde_json::to_string(v).unwrap_or_default()); } }
        if let Some(v) = &t.country { if !v.is_empty() { params.insert("country".to_string(), v.clone()); } }
        if let Some(v) = t.auto_parameters { params.insert("auto_parameters".to_string(), v.to_string()); }
        if let Some(v) = t.exact_match { params.insert("exact_match".to_string(), v.to_string()); }
        if let Some(v) = t.include_usage { params.insert("include_usage".to_string(), v.to_string()); }
        if let Some(v) = t.safe_search { params.insert("safe_search".to_string(), v.to_string()); }
    }

    params
}

pub fn merge_params(defaults: &HashMap<String, String>, argv: &HashMap<String, String>) -> HashMap<String, String> {
    let mut merged = defaults.clone();
    for (k, v) in argv {
        merged.insert(k.clone(), v.clone());
    }
    merged
}

pub fn markdown_search_defaults_to_hashmap(markdown_search: &Option<MarkdownSearchConfig>) -> HashMap<String, String> {
    let mut params = HashMap::new();

    if let Some(m) = markdown_search {
        if let Some(v) = m.n { params.insert("n".to_string(), v.to_string()); }
        if let Some(v) = &m.gl { params.insert("gl".to_string(), v.clone()); }
        if let Some(v) = &m.hl { params.insert("hl".to_string(), v.clone()); }
        if let Some(v) = &m.format { params.insert("format".to_string(), v.clone()); }
        if let Some(v) = m.retain_images { params.insert("retain_images".to_string(), v.to_string()); }
    }

    params
}

pub fn provider_needs_api_key(provider_id: &str) -> bool {
    provider_id != "markdown-search"
}
