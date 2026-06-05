## MODIFIED Requirements

### Requirement: Provider registry
The system SHALL maintain a registry that maps provider IDs to provider implementations. The default provider ID from config is used to select the active provider. The registry SHALL include "tavily" and "markdown-search" providers.

#### Scenario: Known provider ID resolves
- **WHEN** the default_provider in config matches a registered provider
- **THEN** that provider is used for the search

#### Scenario: Unknown provider ID errors
- **WHEN** the default_provider in config does not match any registered provider
- **THEN** the CLI prints an error listing available providers and exits with code 1

#### Scenario: markdown-search provider available
- **WHEN** default_provider is "markdown-search"
- **THEN** the markdown-search provider is used without requiring an API key
