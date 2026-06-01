## ADDED Requirements

### Requirement: Loose argv parsing
The system SHALL parse command-line arguments as `--key value` pairs, collecting all flags and their values into a HashMap.

#### Scenario: Standard key-value pairs
- **WHEN** the CLI is invoked with `--query "test" --max_results 10`
- **THEN** args are parsed as `{ "query": "test", "max_results": "10" }`

#### Scenario: Boolean flags
- **WHEN** the CLI is invoked with `--include_images true`
- **THEN** args are parsed as `{ "include_images": "true" }`

#### Scenario: Missing value for flag
- **WHEN** a flag is the last argument with no value (e.g., `--query`)
- **THEN** the CLI prints an error that the flag is missing a value and exits with code 1

### Requirement: Help flag
The system SHALL support `--help` or `-h` to print usage information.

#### Scenario: Help flag invoked
- **WHEN** the CLI is invoked with `--help` or `-h`
- **THEN** usage information is printed and the CLI exits with code 0

### Requirement: Provider param validation
The system SHALL validate parsed args against the active provider's `valid_params()` schema before making any API call.

#### Scenario: All params valid
- **WHEN** all parsed args match the provider's schema (correct types, known keys)
- **THEN** the request proceeds

#### Scenario: Invalid param detected
- **WHEN** a parsed arg is not in the provider's valid_params or has wrong type
- **THEN** the CLI prints which params are invalid and exits with code 1
