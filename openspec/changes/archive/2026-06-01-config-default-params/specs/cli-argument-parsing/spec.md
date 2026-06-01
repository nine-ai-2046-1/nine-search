## MODIFIED Requirements

### Requirement: Provider param validation
The system SHALL validate merged params (config defaults + CLI argv override) against the active provider's `valid_params()` schema before making any API call.

#### Scenario: All params valid
- **WHEN** all merged args match the provider's schema (correct types, known keys)
- **THEN** the request proceeds

#### Scenario: Invalid param detected
- **WHEN** a merged arg is not in the provider's valid_params or has wrong type
- **THEN** the CLI prints which params are invalid and exits with code 1

## ADDED Requirements

### Requirement: Config-argv merge
The system SHALL merge config.toml `[tavily]` defaults with CLI argv values, where CLI argv values take precedence over config values for any parameter.

#### Scenario: CLI overrides config
- **WHEN** config has `max_results = 5` and CLI has `--max_results 10`
- **THEN** the merged params use `max_results = 10`

#### Scenario: Config used when CLI omits param
- **WHEN** config has `topic = "news"` and CLI does not provide `--topic`
- **THEN** the merged params use `topic = "news"`

#### Scenario: Empty config section
- **WHEN** config has no `[tavily]` section
- **THEN** CLI argv values are used directly without any config defaults
