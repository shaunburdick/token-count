// Contract: Model Registry Interface
//
// This file defines the interface for model resolution and listing.
// It is a specification document, not compilable code.

/// Model registry for resolving model names to configurations
///
/// Singleton registry (via Lazy static) that maps model names and aliases
/// to ModelConfig structs. Supports case-insensitive, normalized matching.
pub struct ModelRegistry {
    /// Map of canonical model names to configurations
    models: HashMap<&'static str, &'static ModelConfig>,

    /// Map of aliases to canonical model names
    /// Example: "gpt4" -> "gpt-4", "openai/gpt-4" -> "gpt-4"
    aliases: HashMap<&'static str, &'static str>,
}

impl ModelRegistry {
    /// Register a model and its aliases
    ///
    /// # Arguments
    /// * `config` - Static model configuration (must have 'static lifetime)
    ///
    /// # Panics
    /// * If model name conflicts with existing model
    /// * If alias conflicts with existing model name or alias
    ///
    /// # Example
    /// ```rust
    /// let mut registry = ModelRegistry::new();
    /// registry.register(&GPT_4);
    /// ```
    fn register(&mut self, config: &'static ModelConfig);

    /// Resolve a model name or alias to a configuration
    ///
    /// Resolution order:
    /// 1. Normalize input (lowercase, trim whitespace)
    /// 2. Check for exact match in models
    /// 3. Check for match in aliases
    /// 4. Return error with suggestions if not found
    ///
    /// # Arguments
    /// * `input` - Model name or alias (e.g., "gpt-4", "GPT4", "openai/gpt-4")
    ///
    /// # Returns
    /// * `Ok(&'static ModelConfig)` - Model configuration
    /// * `Err(TokenError::UnknownModel)` - Model not found (includes suggestions)
    ///
    /// # Example
    /// ```rust
    /// let config = REGISTRY.resolve("gpt4")?;
    /// assert_eq!(config.name, "gpt-4");
    /// ```
    pub fn resolve(&self, input: &str) -> Result<&'static ModelConfig, TokenError>;

    /// List all registered models
    ///
    /// # Returns
    /// * `Vec<&'static ModelConfig>` - All models, sorted by name
    ///
    /// # Example
    /// ```rust
    /// let models = REGISTRY.list_models();
    /// for model in models {
    ///     println!("{}: {} ({})", model.provider, model.name, model.encoding);
    /// }
    /// ```
    pub fn list_models(&self) -> Vec<&'static ModelConfig>;
}

/// Global model registry (lazy static)
///
/// Initialized on first access, contains all supported models.
pub static REGISTRY: Lazy<ModelRegistry> = Lazy::new(|| {
    let mut registry = ModelRegistry::new();

    // Register OpenAI models (MVP)
    registry.register(&GPT_35_TURBO);
    registry.register(&GPT_4);
    registry.register(&GPT_4_TURBO);
    registry.register(&GPT_4O);

    // Future: register Claude, Gemini, Llama models

    registry
});

// Implementation Notes:
//
// 1. Normalization:
//    - Convert to lowercase (case-insensitive matching)
//    - Trim leading/trailing whitespace
//    - Do NOT remove hyphens or underscores (part of model names)
//
// 2. Conflict Detection:
//    - Model name must be unique across all models
//    - Alias must not conflict with any model name
//    - Alias must not conflict with any other alias
//    - Panic on conflict (programming error, should be caught in tests)
//
// 3. Suggestions:
//    - Use Levenshtein distance (strsim crate) to find similar models
//    - Return up to 3 suggestions, sorted by distance
//    - Always include hint to use --list-models
//
// 4. Performance:
//    - HashMap lookups are O(1) (fast resolution)
//    - Lazy initialization (only pay cost on first use)
//    - Static lifetime avoids runtime allocations
//
// 5. Thread Safety:
//    - Lazy<ModelRegistry> is thread-safe (single initialization)
//    - No mutable state after initialization
//    - Safe to call resolve() from multiple threads

// Fuzzy Matching Helper:

/// Find similar model names using Levenshtein distance
///
/// # Arguments
/// * `input` - User input (normalized)
/// * `max_distance` - Maximum edit distance (default: 2)
///
/// # Returns
/// * `String` - Formatted suggestions (e.g., "  - gpt-4\n  - gpt-4o")
///
/// # Example
/// ```rust
/// let suggestions = find_similar_models("gpt5", 2);
/// // Returns: "  - gpt-4\n  - gpt-4o\n  - gpt-4-turbo"
/// ```
pub fn find_similar_models(input: &str, max_distance: usize) -> String {
    // 1. Get all model names and aliases
    // 2. Calculate Levenshtein distance for each
    // 3. Filter by max_distance
    // 4. Sort by distance (ascending)
    // 5. Take top 3
    // 6. Format as "  - model\n  - model\n  - model"
}

// Model Configuration Structure:

/// Configuration for a supported LLM model
///
/// Static data structure (all fields have 'static lifetime).
/// Registered in MODEL_REGISTRY on first access.
#[derive(Debug, Clone)]
pub struct ModelConfig {
    /// Canonical model name (e.g., "gpt-4")
    pub name: &'static str,

    /// Tokenizer encoding name (e.g., "cl100k_base")
    pub encoding: &'static str,

    /// Maximum context window size in tokens
    pub context_window: usize,

    /// List of recognized aliases (e.g., ["gpt4", "openai/gpt-4"])
    pub aliases: &'static [&'static str],

    /// Provider name (e.g., "OpenAI")
    pub provider: &'static str,
}

// Example Model Definitions (OpenAI):

pub const GPT_35_TURBO: ModelConfig = ModelConfig {
    name: "gpt-3.5-turbo",
    encoding: "cl100k_base",
    context_window: 16_385,
    aliases: &["gpt35", "gpt3.5", "openai/gpt-3.5-turbo"],
    provider: "OpenAI",
};

pub const GPT_4: ModelConfig = ModelConfig {
    name: "gpt-4",
    encoding: "cl100k_base",
    context_window: 8_192,
    aliases: &["gpt4", "openai/gpt-4"],
    provider: "OpenAI",
};

pub const GPT_4_TURBO: ModelConfig = ModelConfig {
    name: "gpt-4-turbo",
    encoding: "cl100k_base",
    context_window: 128_000,
    aliases: &["gpt4-turbo", "openai/gpt-4-turbo"],
    provider: "OpenAI",
};

pub const GPT_4O: ModelConfig = ModelConfig {
    name: "gpt-4o",
    encoding: "o200k_base",
    context_window: 128_000,
    aliases: &["gpt4o", "openai/gpt-4o"],
    provider: "OpenAI",
};

// Future Extensions (Post-MVP):
//
// 1. Model Families:
//    pub enum ModelFamily { GPT3, GPT4, Claude3, Gemini, Llama }
//    - Group models by family for easier listing
//
// 2. Model Versioning:
//    pub version: &'static str; // e.g., "2024-04-09"
//    - Track specific model versions
//
// 3. Cost Information (if scope changes):
//    pub cost_per_1k_input_tokens: f64;
//    pub cost_per_1k_output_tokens: f64;
//    - Enable cost estimation (currently out of scope)
//
// 4. Capabilities:
//    pub supports_function_calling: bool;
//    pub supports_vision: bool;
//    - Model-specific features (for future use)
