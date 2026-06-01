## ADDED Requirements

### Requirement: Tavily search via curl
The system SHALL execute Tavily searches by invoking `curl -s -X POST https://api.tavily.com/search` with `Authorization: Bearer <key>` header and a JSON body containing the validated parameters.

#### Scenario: Successful search
- **WHEN** a valid query is provided with a valid API key
- **THEN** the CLI calls curl, receives a JSON response, and pretty-prints it to stdout

#### Scenario: curl not installed
- **WHEN** the `curl` binary is not found on the system
- **THEN** the CLI prints an error message saying curl is required and exits with code 1

### Requirement: Tavily param definitions
The Tavily provider SHALL accept these parameters: `query` (String, required), `search_depth` (Enum: advanced/basic/fast/ultra-fast), `chunks_per_source` (Int, 1-3), `max_results` (Int, 0-20), `topic` (Enum: general/news/finance), `time_range` (Enum), `start_date` (String), `end_date` (String), `include_answer` (Bool|Enum), `include_raw_content` (Bool|Enum), `include_images` (Bool), `include_image_descriptions` (Bool), `include_favicon` (Bool), `include_domains` (Array), `exclude_domains` (Array), `country` (String), `auto_parameters` (Bool), `exact_match` (Bool), `include_usage` (Bool), `safe_search` (Bool).

#### Scenario: All optional params passed
- **WHEN** user provides all supported optional params with valid values
- **THEN** all params are included in the JSON body sent to Tavily

#### Scenario: Required query missing
- **WHEN** `--query` is not provided
- **THEN** the CLI prints an error that query is required and exits with code 1

### Requirement: Tavily response pretty-print
The system SHALL deserialize the curl stdout as JSON and pretty-print it using `serde_json::to_string_pretty`.

#### Scenario: Valid JSON response
- **WHEN** Tavily returns valid JSON
- **THEN** the output is formatted, readable JSON printed to stdout

#### Scenario: Invalid JSON response
- **WHEN** Tavily returns non-JSON content (e.g., HTML error page)
- **THEN** the CLI prints the raw response body and exits with code 1
