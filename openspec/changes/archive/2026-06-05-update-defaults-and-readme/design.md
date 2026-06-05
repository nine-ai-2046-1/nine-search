## Context

目前 markdown-search 嘅默認值係美國英文設定（gl="us", hl="en", n=3, retain_images=false）。用戶在香港，需要改為更適合嘅預設值。

## Goals / Non-Goals

**Goals:**
- markdown-search 默認值改為：n=5, gl="hk", hl="zh", retain_images=true
- 重寫 README.md，包含完整使用說明

**Non-Goals:**
- 改動 Tavily 設定
- 改動代碼邏輯

## Decisions

### 1. 默認值更新

```toml
[markdown-search]
n = 5           # 5 個結果
gl = "hk"       # 香港地區
hl = "zh"       # 中文語言
retain_images = true  # 保留圖片
```

**Why:** 適合香港中文用戶嘅日常使用場景。

### 2. README 內容

包含：項目簡介、安裝方法、快速開始、config 說明、provider 說明、使用範例。

## Risks / Trade-offs

- 改默認值會影響現有用戶（但目前冇其他用戶）
