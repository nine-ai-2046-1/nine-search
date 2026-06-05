## MODIFIED Requirements

### Requirement: Markdown search config defaults
The system SHALL include a `[markdown-search]` section in config.toml with these parameters: `n` (default: 5), `gl` (default: "hk"), `hl` (default: "zh"), `format` (default: "json"), `retain_images` (default: true). Each parameter SHALL have a Cantonese comment with emoji describing its purpose.

#### Scenario: Config with markdown-search section loads
- **WHEN** the config file contains a `[markdown-search]` section with valid values
- **THEN** those values are loaded as defaults for the markdown-search provider

#### Scenario: Default config values for Hong Kong Chinese users
- **WHEN** a new default config is created
- **THEN** markdown-search defaults are n=5, gl="hk", hl="zh", retain_images=true
