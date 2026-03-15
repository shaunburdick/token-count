use token_count::api::consent::ConsentPrompt;
use token_count::cli::{list_models, read_stdin, Cli};
use token_count::error::TokenError;
use token_count::tokenizers::registry::ModelRegistry;
use token_count::{count_tokens, select_formatter};

fn main() {
    let cli = Cli::parse_args();

    // Handle --list-models flag
    if cli.list_models {
        list_models();
        std::process::exit(0);
    }

    // Run the main tokenization flow
    let result = run(cli);

    if let Err(e) = result {
        eprintln!("Error: {}", e);
        std::process::exit(e.exit_code());
    }
}

fn run(cli: Cli) -> Result<(), TokenError> {
    // Check if consent is needed for API call
    if cli.accurate && !cli.yes {
        let registry = ModelRegistry::global();
        let model_config = registry.get_model(&cli.model)?;

        // Check if this is a Claude model (requires API)
        if model_config.encoding == "anthropic-claude" {
            let consent = ConsentPrompt {
                provider: "Anthropic",
                api_endpoint: "https://api.anthropic.com/v1/messages/count_tokens",
            };

            match consent.ask() {
                Ok(true) => {
                    // User consented, proceed with API call
                }
                Ok(false) => {
                    // User declined, fall back to estimation
                    eprintln!(
                        "API call cancelled by user. Falling back to estimation mode (±10% accuracy)."
                    );
                    eprintln!("To use estimation without this prompt, omit the --accurate flag.");
                    // Continue with estimation by clearing the accurate flag
                    return run_tokenization(&cli, false);
                }
                Err(e) => {
                    // Non-interactive mode or other error
                    return Err(e);
                }
            }
        }
    }

    // Proceed with tokenization (user consented or no consent needed)
    run_tokenization(&cli, cli.accurate)
}

/// Run tokenization with the given accurate flag
fn run_tokenization(cli: &Cli, accurate: bool) -> Result<(), TokenError> {
    // Read input from stdin
    let input = read_stdin()?;

    // Count tokens using the specified model, including verbosity for debug mode
    let result = count_tokens(&input, &cli.model, accurate, cli.verbose)?;

    // Select the appropriate formatter based on verbosity level
    let formatter = select_formatter(cli.verbose);

    // Format and print the output
    println!("{}", formatter.format(&result));

    Ok(())
}
