## MODIFIED Requirements

### Requirement: Config file auto-creation
The system SHALL create `~/.config/nine-search/config.toml` if it does not exist, with default values: `default_provider = "markdown-search"`, a `[[providers]]` entry for tavily with an empty key, a `[[providers]]` entry for markdown-search with an empty key (no API key needed), a `[tavily]` section with Tavily defaults, and a `[markdown-search]` section with markdown-search defaults and Cantonese emoji comments.

#### Scenario: First run creates config
- **WHEN** the CLI is invoked and `~/.config/nine-search/config.toml` does not exist
- **THEN** the file is created with default content including `[markdown-search]` section and Cantonese emoji comments, and the CLI proceeds

### Requirement: Config file parsing
The system SHALL parse `~/.config/nine-search/config.toml` as TOML with fields `default_provider` (string), `providers` (array of tables with `id`, `name`, `key`), `tavily` (table with optional Tavily API parameters), and `markdown-search` (table with optional markdown-search parameters).

#### Scenario: Valid config loads successfully
- **WHEN** the config file exists and contains valid TOML with all required sections
- **THEN** the config is loaded and the CLI proceeds

## ADDED Requirements

### Requirement: Markdown search config defaults
The system SHALL include a `[markdown-search]` section in config.toml with these parameters: `n` (default: 3), `gl` (default: "us"), `hl` (default: "en"), `format` (default: "json"), `retain_images` (default: false). Each parameter SHALL have a Cantonese comment with emoji describing its purpose.

#### Scenario: Config with markdown-search section loads
- **WHEN** the config file contains a `[markdown-search]` section with valid values
- **THEN** those values are loaded as defaults for the markdown-search provider

#### Scenario: Config without markdown-search section
- **WHEN** the config file does not contain a `[markdown-search]` section
- **THEN** the system uses built-in defaults for all markdown-search parameters

#### Scenario: Cantonese comments present
- **WHEN** a new default config is created
- **THEN** each markdown-search parameter has a Cantonese comment with emoji above it explaining the parameter's purpose and valid values
