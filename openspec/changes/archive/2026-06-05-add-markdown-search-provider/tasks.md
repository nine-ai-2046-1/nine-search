## 1. Config Update

- [x] 1.1 Add `MarkdownSearchConfig` struct to `config.rs` with fields: n (Option<i64>), gl (Option<String>), hl (Option<String>), format (Option<String>), retain_images (Option<bool>)
- [x] 1.2 Update `Config` struct to include `markdown_search: Option<MarkdownSearchConfig>`
- [x] 1.3 Update `DEFAULT_CONFIG` with `[markdown-search]` section, Cantonese emoji comments, and change `default_provider` to "markdown-search"
- [x] 1.4 Add markdown-search providers entry to DEFAULT_CONFIG (id: "markdown-search", name: "Markdown Search", key: "")

## 2. Markdown Search Provider

- [x] 2.1 Create `src/providers/markdown_search.rs` with `MarkdownSearchProvider` struct implementing Provider trait
- [x] 2.2 Implement `base_url()` returning "https://markdown.new/search"
- [x] 2.3 Implement `auth_header()` returning empty string (no auth needed)
- [x] 2.4 Implement `valid_params()` with: query (String, required), n (Int), gl (String), hl (String), format (String), retain_images (Bool)
- [x] 2.5 Implement `build_curl_args()` with POST method, JSON body containing q, n, gl, hl, format, retain_images
- [x] 2.6 Implement `parse_response()` to deserialize JSON and extract results + extracted markdown
- [x] 2.7 Implement HTTP error handling: 429 rate limit, 5xx server error, curl failure

## 3. Provider Registry

- [x] 3.1 Add `pub mod markdown_search;` to `providers/mod.rs`
- [x] 3.2 Register markdown-search provider in `build_registry()`

## 4. Main Flow Update

- [x] 4.1 Update main.rs to only pass query via CLI for markdown-search provider (other params from config)
- [x] 4.2 Update print_help() to mention markdown-search as default provider

## 5. Verification

- [x] 5.1 Test: `cargo build` succeeds
- [x] 5.2 Test: default config file has markdown-search section with Cantonese comments
- [x] 5.3 Test: `nine-search --query "test"` works with markdown-search provider
