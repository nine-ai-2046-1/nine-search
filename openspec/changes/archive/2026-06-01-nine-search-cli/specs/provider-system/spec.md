## ADDED Requirements

### Requirement: Provider trait
The system SHALL define a `Provider` trait with methods: `name()`, `base_url()`, `auth_header(key)`, `valid_params()`, `build_request(params, key)`, and `parse_response(json)`.

#### Scenario: Provider trait is object-safe
- **WHEN** a struct implements the Provider trait
- **THEN** it can be used as `Box<dyn Provider>`

### Requirement: Per-provider param schema
Each provider SHALL define its own `valid_params()` returning a map of parameter names to `ParamDef { param_type, required }` where `param_type` is one of: String, Int, Bool, Enum, Array.

#### Scenario: Param validation passes
- **WHEN** user-supplied args all exist in the provider's valid_params and match types
- **THEN** validation passes and the request proceeds

#### Scenario: Param validation fails
- **WHEN** a user-supplied arg does not exist in the provider's valid_params or has wrong type
- **THEN** the CLI prints which params are invalid and exits with code 1

### Requirement: Provider registry
The system SHALL maintain a registry that maps provider IDs to provider implementations. The default provider ID from config is used to select the active provider.

#### Scenario: Known provider ID resolves
- **WHEN** the default_provider in config matches a registered provider
- **THEN** that provider is used for the search

#### Scenario: Unknown provider ID errors
- **WHEN** the default_provider in config does not match any registered provider
- **THEN** the CLI prints an error listing available providers and exits with code 1
