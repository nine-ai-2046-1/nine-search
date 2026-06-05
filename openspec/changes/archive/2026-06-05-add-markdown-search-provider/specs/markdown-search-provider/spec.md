## ADDED Requirements

### Requirement: Markdown search via curl
The system SHALL execute searches by invoking `curl -s -X POST https://markdown.new/search` with a JSON body containing `q`, `n`, `gl`, `hl`, `format`, and `retain_images` parameters. No authentication header is required.

#### Scenario: Successful search
- **WHEN** a valid query is provided
- **THEN** the CLI calls curl, receives a JSON response, and pretty-prints it to stdout

#### Scenario: Rate limit exceeded
- **WHEN** markdown.new returns HTTP 429
- **THEN** the CLI prints "Rate limit exceeded. markdown.new allows 30/min, 500/day per IP. Wait or try again later." and exits with code 1

#### Scenario: Server error
- **WHEN** markdown.new returns HTTP 5xx
- **THEN** the CLI prints "markdown.new server error (HTTP {status})" and exits with code 1

#### Scenario: curl not installed
- **WHEN** the `curl` binary is not found on the system
- **THEN** the CLI prints an error message saying curl is required and exits with code 1

#### Scenario: curl connection failure
- **WHEN** curl fails to connect to markdown.new
- **THEN** the CLI prints the curl stderr output and exits with code 1

### Requirement: Markdown search param definitions
The markdown-search provider SHALL accept these config-only parameters: `n` (Int, 1-5, default: 3), `gl` (String, default: "us"), `hl` (String, default: "en"), `format` (String, default: "json"), `retain_images` (Bool, default: false). The `query` parameter SHALL be provided via CLI argv.

#### Scenario: All config params used
- **WHEN** config has all markdown-search params set
- **THEN** those values are used in the API request

#### Scenario: Default config values
- **WHEN** config has no [markdown-search] section
- **THEN** built-in defaults are used (n=3, gl=us, hl=en, format=json, retain_images=false)

### Requirement: Markdown search response parsing
The system SHALL parse the JSON response and extract `extracted[].markdown` content for each source.

#### Scenario: Valid JSON response with extracted content
- **WHEN** markdown.new returns valid JSON with `extracted` array
- **THEN** the output is formatted, readable JSON printed to stdout showing results with markdown content

#### Scenario: Invalid JSON response
- **WHEN** markdown.new returns non-JSON content
- **THEN** the CLI prints "Invalid response from markdown.new" followed by the raw response body and exits with code 1
