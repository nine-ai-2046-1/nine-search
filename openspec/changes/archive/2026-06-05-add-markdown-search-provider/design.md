## Context

nine-search 目前只有 Tavily 一個搜尋 provider。Tavily 需要 API key 且係收費服務。markdown.new/search 係一個免費、免認證嘅搜尋服務，返回完整 Markdown 內容，特別適合 RAG 同 LLM 場景。

## Goals / Non-Goals

**Goals:**
- 新增 markdown-search provider，免 API key
- 所有搜尋參數（除 query）只從 config 讀取，唔暴露 CLI argv
- 完善錯誤處理：rate limit、server error、curl 失敗
- 默認 provider 改為 markdown-search

**Non-Goals:**
- 改動 Tavily provider 嘅行為
- 支援 markdown.new 嘅 crawl 功能
- 多 provider 同時搜尋

## Decisions

### 1. 參數策略：config-only for all except query

markdown-search 嘅所有參數（n, gl, hl, format, retain_images）只從 config 讀取，CLI 只接受 `--query`。

**Why:** 呢啲參數係用戶偏好，唔會每次搜尋都改。避免 CLI 參數膨脹。

**Alternative considered:** 全部參數都支持 CLI override。Rejected — 冇必要，增加複雜度。

### 2. format 固定為 json

format 參數硬編碼為 "json"，config 中保留但唔建議修改。

**Why:** JSON 格式提供結構化數據（results + extracted markdown），方便解析。Markdown 格式只有 combined 文檔，唔夠靈活。

### 3. 錯誤處理：HTTP status code 分類

```
429 → Rate limit 錯誤，顯示限流資訊
5xx → Server 錯誤
0   → curl 連接失敗
```

**Why:** markdown.new 有明確嘅 rate limit（30/min, 500/day），需要清楚提示用戶。

### 4. Response 解析：extracted[].markdown

使用 `extracted` 陣列中嘅 `markdown` 欄位作為主要內容，唔使用 `combined`。

**Why:** `extracted` 陣列提供每個來源嘅獨立 markdown，結構更清晰。`combined` 只係拼接文本。

## Risks / Trade-offs

- **Rate limit 500/day** → 單 IP 限制較嚴，適合個人使用，唔適合高頻調用
- **免費服務穩定性** → 無 SLA 保證，可能隨時變更
- **無 search_depth 選項** → 唔似 Tavily 可以控制搜尋深度
