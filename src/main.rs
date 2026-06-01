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
    println!("  --search_depth <string>       Search depth: basic, advanced, fast, ultra-fast");
    println!("  --max_results <int>           Maximum results (0-20)");
    println!("  --topic <string>              Topic: general, news, finance");
    println!("  --include_images <bool>       Include images in response");
    println!("  --include_answer <bool>       Include LLM-generated answer");
    println!("  --help, -h                    Print this help message");
    println!();
    println!("CONFIG:");
    println!("  Config file: ~/.config/nine-search/config.toml");
    println!("  Set default values in [tavily] section. CLI flags override config.");
    println!("  Add your Tavily API key to the config file before use.");
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

    // Validate API key
    let provider_config = match config::validate_api_key(&config) {
        Some(p) => p,
        None => {
            eprintln!("Error: API key not configured for provider '{}'", config.default_provider);
            eprintln!("Please edit ~/.config/nine-search/config.toml and add your API key.");
            process::exit(1);
        }
    };

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

    // Load Tavily config defaults and merge with argv
    let config_defaults = config::tavily_defaults_to_hashmap(&config.tavily);
    let params = config::merge_params(&config_defaults, &argv_params);

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
    let curl_args = provider.build_curl_args(&params, &provider_config.key);

    // Execute curl
    let output = process::Command::new("curl")
        .args(&curl_args)
        .output()
        .expect("Failed to execute curl");

    if !output.status.success() {
        let stderr = String::from_utf8_lossy(&output.stderr);
        eprintln!("Error: curl failed with status: {}", output.status);
        eprintln!("{}", stderr);
        process::exit(1);
    }

    // Parse and pretty-print response
    let stdout = String::from_utf8_lossy(&output.stdout);
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
