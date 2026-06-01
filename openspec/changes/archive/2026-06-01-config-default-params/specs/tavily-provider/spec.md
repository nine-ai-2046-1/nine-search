## MODIFIED Requirements

### Requirement: Tavily param definitions
The Tavily provider SHALL accept these parameters: `query` (String, required), `search_depth` (Enum: advanced/basic/fast/ultra-fast), `chunks_per_source` (Int, 1-3), `max_results` (Int, 0-20), `topic` (Enum: general/news/finance), `time_range` (Enum), `start_date` (String), `end_date` (String), `include_answer` (Bool|Enum), `include_raw_content` (Bool|Enum), `include_images` (Bool), `include_image_descriptions` (Bool), `include_favicon` (Bool), `include_domains` (Array), `exclude_domains` (Array), `country` (String), `auto_parameters` (Bool), `exact_match` (Bool), `include_usage` (Bool), `safe_search` (Bool). Parameters without CLI argv values SHALL use config.toml `[tavily]` section defaults.

#### Scenario: All optional params passed via CLI
- **WHEN** user provides all supported optional params with valid values via CLI
- **THEN** all CLI params override config defaults and are included in the JSON body sent to Tavily

#### Scenario: Params from config defaults
- **WHEN** user does not provide optional params via CLI
- **THEN** config.toml `[tavily]` section values are used as defaults

#### Scenario: Mix of CLI and config params
- **WHEN** user provides some params via CLI and omits others
- **THEN** CLI params override config defaults, omitted params use config values

#### Scenario: Required query missing
- **WHEN** `--query` is not provided
- **THEN** the CLI prints an error that query is required and exits with code 1
