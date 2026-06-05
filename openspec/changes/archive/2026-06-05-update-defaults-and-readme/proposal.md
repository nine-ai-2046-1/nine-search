## Why

markdown-search 嘅默認值唔適合香港中文用戶：地區係 us、語言係 en、冇圖片、只有 3 個結果。需要改為更適合嘅預設值，同時更新 README 文檔。

## What Changes

- `config.rs` DEFAULT_CONFIG 中 `[markdown-search]` section 默認值改為：n=5, gl="hk", hl="zh", retain_images=true
- README.md 重寫，包含完整嘅使用說明、安裝方法、config 說明

## Capabilities

### New Capabilities

_(none)_

### Modified Capabilities

- `config-management`: markdown-search 默認值改為香港中文設定

## Impact

- `src/config.rs`: DEFAULT_CONFIG 常量更新
- `README.md`: 重寫
