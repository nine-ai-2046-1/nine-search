## ADDED Requirements

### Requirement: Config file auto-creation
The system SHALL create `~/.config/nine-search/config.toml` if it does not exist, with default values: `default_provider = "tavily"` and a `[[providers]]` entry for tavily with an empty key.

#### Scenario: First run creates config
- **WHEN** the CLI is invoked and `~/.config/nine-search/config.toml` does not exist
- **THEN** the file is created with default content and the CLI proceeds

### Requirement: Config file parsing
The system SHALL parse `~/.config/nine-search/config.toml` as TOML with fields `default_provider` (string) and `providers` (array of tables with `id`, `name`, `key`).

#### Scenario: Valid config loads successfully
- **WHEN** the config file exists and contains valid TOML with a default_provider and matching provider entry
- **THEN** the config is loaded and the CLI proceeds

#### Scenario: Malformed config errors clearly
- **WHEN** the config file exists but contains invalid TOML or missing required fields
- **THEN** the CLI prints an error message explaining the issue and exits with code 1

### Requirement: API key validation
The system SHALL check that the API key for the default provider is non-empty before proceeding.

#### Scenario: Empty API key warns and exits
- **WHEN** the default provider's `key` field is empty or missing
- **THEN** the CLI prints a warning telling the user to fill in the API key at the config path, and exits with code 1

#### Scenario: Valid API key proceeds
- **WHEN** the default provider's `key` field is a non-empty string
- **THEN** the CLI proceeds to argument parsing
