## Why

目前 config.toml 只有 API key 同 provider 設定，每次用 CLI 都要打一堆 `--param` flag。將 Tavily 所有支援嘅參數預設值放喺 config.toml，用戶可以直接改 config 就得，唔使每次都打 flag。CLI argv 可以 override config 嘅值，更加靈活。

## What Changes

- config.toml 新增 `[tavily]` section，包含所有 Tavily API 參數嘅預設值
- 每個 config option 上面加 Cantonese + emoji comment，方便用戶理解
- Config struct 新增 `tavily` section 嘅 Deserialize
- CLI flow 改為：load config defaults → merge with CLI argv → validate → call API

## Capabilities

### New Capabilities

_(none — extending existing capabilities)_

### Modified Capabilities

- `config-management`: Config struct 新增 `[tavily]` section，DEFAULT_CONFIG 加入所有 Tavily 參數預設值
- `tavily-provider`: build_curl_args 改為接受 config defaults + argv override 嘅 merged params
- `cli-argument-parsing`: Arg parsing flow 改為先 load config defaults，再用 argv override

## Impact

- `src/config.rs`: Config struct + DEFAULT_CONFIG 大改
- `src/main.rs`: Main flow 改為 merge config defaults with argv
- `src/providers/tavily.rs`: build_curl_args 唔變，但 params 嘅來源改變
