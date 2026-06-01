## 1. Config Structure Update

- [x] 1.1 Add `TavilyConfig` struct to `config.rs` with all 19 optional Tavily params as `Option<T>` fields
- [x] 1.2 Update `Config` struct to include `tavily: Option<TavilyConfig>`
- [x] 1.3 Update `DEFAULT_CONFIG` const with `[tavily]` section, all defaults, and Cantonese emoji comments

## 2. Config Merge Logic

- [x] 2.1 Add `tavily_defaults_to_hashmap()` function to convert `TavilyConfig` to `HashMap<String, String>`
- [x] 2.2 Add `merge_params()` function: config defaults + CLI argv override (argv wins)

## 3. Main Flow Update

- [x] 3.1 Update `main.rs` to load Tavily config defaults after loading config
- [x] 3.2 Update `main.rs` to merge config defaults with CLI argv before validation
- [x] 3.3 Update `print_help()` to mention config file usage

## 4. Verification

- [x] 4.1 Test: `cargo build` succeeds
- [x] 4.2 Test: default config file has all Cantonese comments and correct defaults
- [x] 4.3 Test: CLI argv overrides config values
