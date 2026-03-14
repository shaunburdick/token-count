use token_count::cli::{list_models, read_stdin, Cli};
use token_count::error::TokenError;
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
    // Read input from stdin
    let input = read_stdin()?;

    // Count tokens using the specified model
    let result = count_tokens(&input, &cli.model)?;

    // Select the appropriate formatter based on verbosity level
    let formatter = select_formatter(cli.verbose);

    // Format and print the output
    println!("{}", formatter.format(&result));

    Ok(())
}
