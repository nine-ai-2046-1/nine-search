## Why

目前只有 Tavily 一個搜尋引擎，需要 API key 且係收費服務。加入 markdown-search 作為新 provider，佢係免費、免 API key、返回完整 Markdown 內容，特別適合 RAG 同 LLM 使用場景。同時將默認 provider 改為 markdown-search。

## What Changes

- 新增 `markdown-search` provider：POST 到 `https://markdown.new/search`，免認證
- 新增 `MarkdownSearchConfig` struct，config.toml 加入 `[markdown-search]` section
- 默認 provider 改為 `"markdown-search"`
- 所有搜尋參數（除 query 外）只從 config 讀取，唔暴露 CLI argv
- 完善錯誤處理：429 rate limit、5xx server error、curl 失敗、JSON 解析錯誤

## Capabilities

### New Capabilities

- `markdown-search-provider`: markdown.new 搜尋引擎整合，免 API key，返回 JSON 格式搜尋結果 + 完整 Markdown 內容

### Modified Capabilities

- `config-management`: 新增 MarkdownSearchConfig，默認 provider 改為 markdown-search
- `provider-system`: 註冊 markdown-search provider

## Impact

- `src/config.rs`: 新增 MarkdownSearchConfig struct + DEFAULT_CONFIG 大改
- `src/providers/markdown_search.rs`: 新檔案
- `src/providers/mod.rs`: 註冊新 provider
- `src/main.rs`: 默認 provider 改為 markdown-search
