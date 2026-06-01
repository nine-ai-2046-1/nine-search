## 1. Project Setup

- [x] 1.1 Initialize Rust project with `cargo init` and set up `Cargo.toml` with dependencies: `toml`, `serde`, `serde_json`
- [x] 1.2 Create module structure: `src/config.rs`, `src/providers/mod.rs`, `src/providers/tavily.rs`

## 2. Config Management

- [x] 2.1 Implement `config.rs`: `DEFAULT_CONFIG` const, `load_or_create()` function that creates `~/.config/nine-search/config.toml` if missing
- [x] 2.2 Implement config parsing with `toml` crate: `Config` struct with `default_provider: String` and `providers: Vec<ProviderConfig>`
- [x] 2.3 Implement API key validation: check default provider's key is non-empty, warn and exit if missing

## 3. Provider System

- [x] 3.1 Define `Provider` trait in `providers/mod.rs` with methods: `name()`, `base_url()`, `auth_header()`, `valid_params()`, `build_curl_args()`, `parse_response()`
- [x] 3.2 Define `ParamDef` struct (param_type enum + required bool) and `ParamType` enum (String, Int, Bool, Enum, Array)
- [x] 3.3 Implement provider registry: `HashMap<&str, Box<dyn Provider>>`, lookup by ID from config

## 4. Tavily Provider

- [x] 4.1 Implement `tavily.rs`: struct `TavilyProvider` implementing `Provider` trait with base_url `https://api.tavily.com/search` and Bearer auth header
- [x] 4.2 Define Tavily's `valid_params()` with all 18 parameters (query required, rest optional with correct types)
- [x] 4.3 Implement `build_curl_args()`: construct `-X POST`, `-H Authorization`, `-H Content-Type`, `-d json_body` args
- [x] 4.4 Implement `parse_response()`: deserialize JSON string to `serde_json::Value`

## 5. CLI Argument Parsing

- [x] 5.1 Implement loose arg parsing in `main.rs`: iterate `std::env::args()`, collect `--key value` pairs into `HashMap<String, String>`
- [x] 5.2 Support `--help`/`-h` flag to print usage and exit
- [x] 5.3 Implement param validation: check each parsed arg against provider's `valid_params()`, print invalid params and exit if mismatch

## 6. Integration & Output

- [x] 6.1 Wire up `main.rs`: load config → validate key → parse args → select provider → validate params → build curl command → execute → parse → pretty-print
- [x] 6.2 Implement curl execution: use `std::process::Command` to invoke `curl` with args, capture stdout/stderr
- [x] 6.3 Handle curl errors: check exit code, print stderr if failed
- [x] 6.4 Pretty-print JSON output with `serde_json::to_string_pretty`
- [x] 6.5 Handle invalid JSON response: print raw body and exit with code 1

## 7. Error Handling & Polish

- [x] 7.1 Add clear error messages for all failure modes (missing curl, bad config, invalid params, API errors)
- [x] 7.2 Ensure all error paths exit with code 1, success paths exit with code 0
- [x] 7.3 Test end-to-end: `cargo build` succeeds, `nine-search --query "test"` works with valid key
