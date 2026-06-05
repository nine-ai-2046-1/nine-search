mod config;
mod providers;

use std::collections::HashMap;
use std::process;

fn print_help() {
    println!("nine-search - A modular search CLI");
    println!();
    println!("USAGE:");
    println!("  nine-search --query \"search term\" [OPTIONS]");
    println!();
    println!("OPTIONS:");
    println!("  --query <string>              Search query (required)");
    println!("  --help, -h                    Print this help message");
    println!();
    println!("DEFAULT PROVIDER: markdown-search (free, no API key needed)");
    println!();
    println!("CONFIG:");
    println!("  Config file: ~/.config/nine-search/config.toml");
    println!("  markdown-search: Set defaults in [markdown-search] section.");
    println!("  tavily: Add API key and set defaults in [tavily] section.");
}

fn parse_args() -> HashMap<String, String> {
    let args: Vec<String> = std::env::args().skip(1).collect();
    let mut params = HashMap::new();
    let mut i = 0;

    while i < args.len() {
        let arg = &args[i];

        if arg == "--help" || arg == "-h" {
            print_help();
            process::exit(0);
        }

        if arg.starts_with("--") {
            let key = arg[2..].to_string();
            i += 1;
            if i >= args.len() {
                eprintln!("Error: flag '{}' is missing a value", arg);
                process::exit(1);
            }
            params.insert(key, args[i].clone());
        } else {
            eprintln!("Error: unexpected argument '{}'", arg);
            process::exit(1);
        }

        i += 1;
    }

    params
}

fn validate_params(
    params: &HashMap<String, String>,
    valid: &HashMap<String, providers::ParamDef>,
) -> Vec<String> {
    let mut errors = Vec::new();

    for (key, value) in params {
        match valid.get(key) {
            Some(def) => {
                match def.param_type {
                    providers::ParamType::Int => {
                        if value.parse::<i64>().is_err() {
                            errors.push(format!("'{}' must be an integer, got '{}'", key, value));
                        }
                    }
                    providers::ParamType::Bool => {
                        if value != "true" && value != "false" {
                            errors.push(format!("'{}' must be true or false, got '{}'", key, value));
                        }
                    }
                    _ => {}
                }
            }
            None => {
                errors.push(format!("unknown parameter '{}'", key));
            }
        }
    }

    errors
}

fn main() {
    // Load or create config
    let config = config::load_or_create();

    // Check if curl is available
    let curl_check = process::Command::new("curl")
        .arg("--version")
        .output();
    if curl_check.is_err() {
        eprintln!("Error: curl is not installed or not in PATH");
        eprintln!("Please install curl to use nine-search.");
        process::exit(1);
    }

    // Parse arguments
    let argv_params = parse_args();

    // Get provider from registry
    let registry = providers::build_registry();
    let provider = match registry.get(&config.default_provider) {
        Some(p) => p.as_ref(),
        None => {
            eprintln!("Error: unknown provider '{}'", config.default_provider);
            eprintln!("Available providers: {}", registry.keys().cloned().collect::<Vec<_>>().join(", "));
            process::exit(1);
        }
    };

    // Validate API key only if provider needs it
    let provider_config = if config::provider_needs_api_key(&config.default_provider) {
        match config::validate_api_key(&config) {
            Some(p) => Some(p),
            None => {
                eprintln!("Error: API key not configured for provider '{}'", config.default_provider);
                eprintln!("Please edit ~/.config/nine-search/config.toml and add your API key.");
                process::exit(1);
            }
        }
    } else {
        None
    };

    // Load config defaults based on provider
    let config_defaults = match config.default_provider.as_str() {
        "markdown-search" => config::markdown_search_defaults_to_hashmap(&config.markdown_search),
        "tavily" => config::tavily_defaults_to_hashmap(&config.tavily),
        _ => HashMap::new(),
    };

    // Merge config defaults with argv (argv wins)
    let params = config::merge_params(&config_defaults, &argv_params);

    // Validate params against provider schema
    let valid_params = provider.valid_params();
    let errors = validate_params(&params, &valid_params);
    if !errors.is_empty() {
        eprintln!("Error: invalid parameters:");
        for err in &errors {
            eprintln!("  - {}", err);
        }
        process::exit(1);
    }

    // Check required params
    if !params.contains_key("query") {
        eprintln!("Error: --query is required");
        process::exit(1);
    }

    // Build curl command
    let api_key = provider_config.map(|p| p.key.as_str()).unwrap_or("");
    let curl_args = provider.build_curl_args(&params, api_key);

    // Execute curl
    let output = process::Command::new("curl")
        .args(&curl_args)
        .output()
        .expect("Failed to execute curl");

    // Handle HTTP errors
    if !output.status.success() {
        let stderr = String::from_utf8_lossy(&output.stderr);
        eprintln!("Error: curl failed with status: {}", output.status);
        eprintln!("{}", stderr);
        process::exit(1);
    }

    // Check for HTTP error codes in response
    let stdout = String::from_utf8_lossy(&output.stdout);

    // Try to detect HTTP errors from curl output
    // curl -s doesn't output HTTP headers, so we check the response body
    if let Ok(value) = serde_json::from_str::<serde_json::Value>(&stdout) {
        // Check if it's an error response
        if let Some(success) = value.get("success") {
            if success == false {
                let error_msg = value.get("error")
                    .and_then(|e| e.as_str())
                    .unwrap_or("Unknown error");
                eprintln!("Error: {}", error_msg);
                process::exit(1);
            }
        }
    }

    // Parse and pretty-print response
    match provider.parse_response(&stdout) {
        Ok(value) => {
            println!("{}", serde_json::to_string_pretty(&value).unwrap());
        }
        Err(e) => {
            eprintln!("Error: {}", e);
            eprintln!("Raw response: {}", stdout);
            process::exit(1);
        }
    }
}
