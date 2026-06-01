## MODIFIED Requirements

### Requirement: Config file auto-creation
The system SHALL create `~/.config/nine-search/config.toml` if it does not exist, with default values: `default_provider = "tavily"`, a `[[providers]]` entry for tavily with an empty key, and a `[tavily]` section with all Tavily API parameter defaults and Cantonese emoji comments.

#### Scenario: First run creates config
- **WHEN** the CLI is invoked and `~/.config/nine-search/config.toml` does not exist
- **THEN** the file is created with default content including `[tavily]` section and Cantonese emoji comments, and the CLI proceeds

### Requirement: Config file parsing
The system SHALL parse `~/.config/nine-search/config.toml` as TOML with fields `default_provider` (string), `providers` (array of tables with `id`, `name`, `key`), and `tavily` (table with optional Tavily API parameters).

#### Scenario: Valid config loads successfully
- **WHEN** the config file exists and contains valid TOML with a default_provider, matching provider entry, and optional [tavily] section
- **THEN** the config is loaded and the CLI proceeds

#### Scenario: Malformed config errors clearly
- **WHEN** the config file exists but contains invalid TOML or missing required fields
- **THEN** the CLI prints an error message explaining the issue and exits with code 1

## ADDED Requirements

### Requirement: Tavily config defaults
The system SHALL include a `[tavily]` section in config.toml with these parameters: `search_depth` (default: "basic"), `chunks_per_source` (default: 3), `max_results` (default: 5), `topic` (default: "general"), `time_range` (default: ""), `start_date` (default: ""), `end_date` (default: ""), `include_answer` (default: false), `include_raw_content` (default: false), `include_images` (default: false), `include_image_descriptions` (default: false), `include_favicon` (default: false), `include_domains` (default: []), `exclude_domains` (default: []), `country` (default: ""), `auto_parameters` (default: false), `exact_match` (default: false), `include_usage` (default: false), `safe_search` (default: false). Each parameter SHALL have a Cantonese comment with emoji describing its purpose.

#### Scenario: Config with Tavily section loads
- **WHEN** the config file contains a `[tavily]` section with valid values
- **THEN** those values are loaded as defaults for the Tavily provider

#### Scenario: Config without Tavily section
- **WHEN** the config file does not contain a `[tavily]` section
- **THEN** the system uses built-in defaults for all Tavily parameters

#### Scenario: Cantonese comments present
- **WHEN** a new default config is created
- **THEN** each Tavily parameter has a Cantonese comment with emoji above it explaining the parameter's purpose and valid values
