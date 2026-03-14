// Contract: Output Formatter Interface
//
// This file defines the interface for formatting tokenization results.
// It is a specification document, not compilable code.

/// Output formatter trait for displaying tokenization results
///
/// Strategy pattern - select appropriate formatter based on verbosity level.
pub trait OutputFormatter {
    /// Format tokenization result as a string
    ///
    /// # Arguments
    /// * `result` - Tokenization result with count, model info, optional token IDs
    ///
    /// # Returns
    /// * `String` - Formatted output for display on stdout
    ///
    /// # Example
    /// ```rust
    /// let formatter = SimpleFormatter;
    /// let output = formatter.format(&result);
    /// println!("{}", output);
    /// ```
    fn format(&self, result: &TokenizationResult) -> String;
}

// Formatter Implementations:

/// Simple formatter (verbosity 0)
///
/// Outputs only the token count as a single number.
/// No trailing newline (POSIX-style, like wc -c).
pub struct SimpleFormatter;

impl OutputFormatter for SimpleFormatter {
    fn format(&self, result: &TokenizationResult) -> String {
        // Example output: "142"
        result.token_count.to_string()
    }
}

/// Verbose formatter (verbosity 1-2)
///
/// Outputs model info, token count, and optionally context window usage.
pub struct VerboseFormatter {
    /// Whether to include context window info (true for verbosity 2)
    pub include_context: bool,
}

impl OutputFormatter for VerboseFormatter {
    fn format(&self, result: &TokenizationResult) -> String {
        // Example output (verbosity 1):
        // Model: gpt-4 (cl100k_base encoding)
        // Tokens: 142
        //
        // Example output (verbosity 2):
        // Model: gpt-4 (cl100k_base encoding)
        // Tokens: 142
        // Context Window: 8,192 tokens
        // Usage: 1.73%
    }
}

/// Debug formatter (verbosity 3)
///
/// Outputs all available information: model info, token count, token IDs,
/// decoded tokens (sample), and context window usage.
pub struct DebugFormatter;

impl OutputFormatter for DebugFormatter {
    fn format(&self, result: &TokenizationResult) -> String {
        // Example output:
        // Model: gpt-4 (cl100k_base encoding)
        // Tokens: 2
        // Token IDs: [15339, 1917]
        // Decoded Tokens: ["Hello", " world"]
        // Context Window: 8,192 tokens
        // Usage: 0.02%
        //
        // Note: Limit token IDs and decoded tokens to first 10
        // If > 10 tokens, append "..." to indicate truncation
    }
}

// Formatter Factory:

/// Select appropriate formatter based on verbosity level
///
/// # Arguments
/// * `verbosity` - Verbosity level (0-3)
///
/// # Returns
/// * `Box<dyn OutputFormatter>` - Formatter implementation
///
/// # Example
/// ```rust
/// let formatter = select_formatter(args.verbosity);
/// let output = formatter.format(&result);
/// println!("{}", output);
/// ```
pub fn select_formatter(verbosity: u8) -> Box<dyn OutputFormatter> {
    match verbosity {
        0 => Box::new(SimpleFormatter),
        1 => Box::new(VerboseFormatter {
            include_context: false,
        }),
        2 => Box::new(VerboseFormatter {
            include_context: true,
        }),
        _ => Box::new(DebugFormatter), // verbosity >= 3
    }
}

// Implementation Notes:
//
// 1. Output Consistency:
//    - Always output to stdout (never stderr)
//    - Use consistent formatting (spacing, capitalization)
//    - Number formatting: Use commas for large numbers (8,192 not 8192)
//    - Percentage formatting: Two decimal places (1.73% not 1.7%)
//
// 2. Token Limiting:
//    - Debug formatter shows max 10 token IDs and decoded tokens
//    - If > 10 tokens, append "..." to indicate truncation
//    - Example: [15339, 1917, 11, 1268, 527, ...]
//
// 3. String Escaping:
//    - Decoded tokens should be JSON-escaped (quotes, newlines, etc.)
//    - Example: ["Hello", " world", "\n"]
//    - Use Rust's escape_default() for Unicode characters
//
// 4. Performance:
//    - Formatters should not clone data (borrow from result)
//    - String allocation is acceptable (output is small)
//    - Avoid unnecessary computation (e.g., don't decode if verbosity < 3)
//
// 5. Future Formats:
//    - JSON output: {"model": "gpt-4", "tokens": 142, "usage": 0.0173}
//    - CSV output: gpt-4,142,8192,1.73
//    - TSV output: gpt-4\t142\t8192\t1.73

// TokenizationResult Structure:

/// Result of tokenization with metadata
///
/// Contains all information needed for formatting output.
/// Token IDs and decoded tokens are optional (only for verbosity >= 3).
#[derive(Debug, Clone)]
pub struct TokenizationResult {
    /// Total number of tokens
    pub token_count: usize,

    /// Model configuration used
    pub model_config: &'static ModelConfig,

    /// Token IDs (only populated if verbosity >= 3)
    pub token_ids: Option<Vec<u32>>,

    /// Decoded tokens (only populated if verbosity >= 3)
    /// One string per token (preserves token boundaries)
    pub decoded_tokens: Option<Vec<String>>,
}

impl TokenizationResult {
    /// Calculate context window usage percentage
    ///
    /// # Returns
    /// * `f64` - Percentage (0.0 to 100.0)
    ///
    /// # Example
    /// ```rust
    /// let result = TokenizationResult {
    ///     token_count: 142,
    ///     model_config: &GPT_4,
    ///     token_ids: None,
    ///     decoded_tokens: None,
    /// };
    /// assert_eq!(result.context_usage_percent(), 1.73...);
    /// ```
    pub fn context_usage_percent(&self) -> f64 {
        (self.token_count as f64 / self.model_config.context_window as f64) * 100.0
    }

    /// Get first N decoded tokens (for debug output)
    ///
    /// # Arguments
    /// * `n` - Maximum number of tokens to return
    ///
    /// # Returns
    /// * `Option<Vec<String>>` - First N tokens, or None if not available
    ///
    /// # Example
    /// ```rust
    /// let sample = result.sample_tokens(10);
    /// if let Some(tokens) = sample {
    ///     println!("Sample: {:?}", tokens);
    /// }
    /// ```
    pub fn sample_tokens(&self, n: usize) -> Option<Vec<String>> {
        self.decoded_tokens
            .as_ref()
            .map(|tokens| tokens.iter().take(n).cloned().collect())
    }
}

// Output Examples (from Feature Spec):

// Verbosity 0 (SimpleFormatter):
// 142

// Verbosity 1 (VerboseFormatter, include_context=false):
// Model: gpt-4 (cl100k_base encoding)
// Tokens: 142

// Verbosity 2 (VerboseFormatter, include_context=true):
// Model: gpt-4 (cl100k_base encoding)
// Tokens: 142
// Context Window: 8,192 tokens
// Usage: 1.73%

// Verbosity 3 (DebugFormatter):
// Model: gpt-4 (cl100k_base encoding)
// Tokens: 2
// Token IDs: [15339, 1917]
// Decoded Tokens: ["Hello", " world"]
// Context Window: 8,192 tokens
// Usage: 0.02%

// Future Extensions (Post-MVP):
//
// 1. JSON Output:
//    pub struct JsonFormatter;
//    - Output: {"model": "gpt-4", "tokens": 142, "context_window": 8192, "usage_percent": 1.73}
//    - Useful for scripting and integration
//
// 2. Machine-Readable Formats:
//    pub struct CsvFormatter;  // gpt-4,142,8192,1.73
//    pub struct TsvFormatter;  // gpt-4\t142\t8192\t1.73
//    - Useful for spreadsheet import
//
// 3. Colorized Output:
//    pub struct ColorFormatter { verbosity: u8 }
//    - Use ANSI color codes (if terminal supports it)
//    - Example: Model name in blue, token count in green
//
// 4. Table Format:
//    pub struct TableFormatter;
//    - Multi-line table with borders (for multiple inputs)
//    - Example:
//      ┌──────────┬────────┬─────────┐
//      │ Model    │ Tokens │ Usage   │
//      ├──────────┼────────┼─────────┤
//      │ gpt-4    │ 142    │ 1.73%   │
//      └──────────┴────────┴─────────┘
