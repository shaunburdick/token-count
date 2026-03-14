// Contract: Tokenizer Trait
//
// This file defines the interface that all tokenizer implementations must follow.
// It is a specification document, not compilable code.

/// Tokenizer trait for encoding/decoding text to/from token IDs
///
/// All tokenizer implementations must be thread-safe (Send + Sync) to enable
/// future parallelization and use in multi-threaded contexts.
pub trait Tokenizer: Send + Sync {
    /// Encode text into a sequence of token IDs
    ///
    /// # Arguments
    /// * `text` - UTF-8 text to tokenize (zero-copy, borrowed)
    ///
    /// # Returns
    /// * `Ok(Vec<u32>)` - Token IDs representing the input text
    /// * `Err(TokenError::Encoding)` - If encoding fails (invalid input, encoding error)
    ///
    /// # Example
    /// ```rust
    /// let tokenizer = OpenAITokenizer::new(&GPT_4)?;
    /// let tokens = tokenizer.encode("Hello world")?;
    /// assert_eq!(tokens, vec![15339, 1917]);
    /// ```
    fn encode(&self, text: &str) -> Result<Vec<u32>, TokenError>;

    /// Count tokens without returning IDs (potentially faster)
    ///
    /// Default implementation calls `encode()` and returns length.
    /// Implementations may override with optimized counting logic.
    ///
    /// # Arguments
    /// * `text` - UTF-8 text to tokenize
    ///
    /// # Returns
    /// * `Ok(usize)` - Number of tokens in the input text
    /// * `Err(TokenError::Encoding)` - If encoding fails
    ///
    /// # Example
    /// ```rust
    /// let tokenizer = OpenAITokenizer::new(&GPT_4)?;
    /// let count = tokenizer.count_tokens("Hello world")?;
    /// assert_eq!(count, 2);
    /// ```
    fn count_tokens(&self, text: &str) -> Result<usize, TokenError> {
        Ok(self.encode(text)?.len())
    }

    /// Decode token IDs back to text
    ///
    /// # Arguments
    /// * `tokens` - Slice of token IDs to decode
    ///
    /// # Returns
    /// * `Ok(String)` - Decoded text
    /// * `Err(TokenError::Decoding)` - If decoding fails (invalid token IDs)
    ///
    /// # Example
    /// ```rust
    /// let tokenizer = OpenAITokenizer::new(&GPT_4)?;
    /// let text = tokenizer.decode(&[15339, 1917])?;
    /// assert_eq!(text, "Hello world");
    /// ```
    fn decode(&self, tokens: &[u32]) -> Result<String, TokenError>;

    /// Get model configuration for this tokenizer
    ///
    /// # Returns
    /// * `&ModelConfig` - Model metadata (name, encoding, context window, etc.)
    ///
    /// # Example
    /// ```rust
    /// let tokenizer = OpenAITokenizer::new(&GPT_4)?;
    /// let config = tokenizer.model_config();
    /// assert_eq!(config.name, "gpt-4");
    /// assert_eq!(config.encoding, "cl100k_base");
    /// ```
    fn model_config(&self) -> &ModelConfig;
}

// Implementation Notes:
//
// 1. Thread Safety:
//    - All implementations must be Send + Sync
//    - Tokenizers may cache internal state (e.g., BPE encoder)
//    - Must not use mutable state without synchronization
//
// 2. Performance:
//    - encode() should minimize allocations (use &str, not String)
//    - count_tokens() may be optimized to avoid allocating Vec<u32>
//    - Implementations should lazy-load tokenizer data (not in constructor)
//
// 3. Error Handling:
//    - encode() fails on invalid UTF-8 (should be caught earlier)
//    - decode() fails on invalid token IDs (e.g., out of vocab range)
//    - Implementations should not panic (return Result)
//
// 4. Special Tokens:
//    - OpenAI models: encode_with_special_tokens() handles <|endoftext|>, etc.
//    - Other models: TBD based on provider specifications
//
// 5. Zero-Copy:
//    - Use &str (not String) to avoid cloning large inputs
//    - Return Vec<u32> (caller owns, can reuse)
//    - Decode returns String (must allocate, UTF-8 construction)

// Future Extensions (Post-MVP):
//
// 1. Streaming Interface:
//    fn encode_streaming(&self, text: &str, callback: impl FnMut(u32)) -> Result<(), TokenError>;
//    - Useful for very large inputs (avoid allocating full Vec)
//
// 2. Batch Encoding:
//    fn encode_batch(&self, texts: &[&str]) -> Result<Vec<Vec<u32>>, TokenError>;
//    - Parallelize encoding across multiple inputs
//
// 3. Metadata:
//    fn vocab_size(&self) -> usize;
//    fn special_tokens(&self) -> &[String];
//    - Expose tokenizer internals for advanced use cases
