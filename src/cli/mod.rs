//! CLI module for command-line interface

pub mod args;
pub mod input;

pub use args::Cli;
pub use input::{read_stdin, read_stdin_streaming};

use crate::tokenizers::registry::ModelRegistry;

/// List all supported models to stdout
pub fn list_models() {
    let registry = ModelRegistry::global();
    let models = registry.list_models();

    println!("Supported models:\n");

    for model in models {
        println!("  {}", model.name);
        println!("    Encoding: {}", model.encoding);
        println!("    Context window: {} tokens", model.context_window);

        if !model.aliases.is_empty() {
            println!("    Aliases: {}", model.aliases.join(", "));
        }

        println!();
    }

    println!("Use --model to specify a model (case-insensitive)");
    println!("Example: token-count --model gpt4");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_list_models_runs() {
        // Just ensure it doesn't panic
        list_models();
    }
}
