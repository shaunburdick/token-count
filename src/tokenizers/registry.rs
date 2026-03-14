//! Model registry for managing supported models

use crate::error::TokenError;
use crate::tokenizers::{
    claude::{claude_models, ClaudeTokenizer},
    google::{google_models, GoogleTokenizer},
    openai::OpenAITokenizer,
    ModelInfo, Tokenizer,
};
use anyhow::Result;
use std::collections::HashMap;
use std::sync::OnceLock;

/// Configuration for a specific model
#[derive(Debug, Clone)]
pub struct ModelConfig {
    pub name: String,
    pub encoding: String,
    pub context_window: usize,
    pub description: String,
    pub aliases: Vec<String>,
}

/// Registry of all supported models
pub struct ModelRegistry {
    models: HashMap<String, ModelConfig>,
    aliases: HashMap<String, String>, // alias → canonical name
}

impl ModelRegistry {
    /// Create a new model registry with all supported models
    pub fn new() -> Self {
        let mut registry = Self { models: HashMap::new(), aliases: HashMap::new() };

        // GPT-3.5-turbo (default)
        registry.add_model(ModelConfig {
            name: "gpt-3.5-turbo".to_string(),
            encoding: "cl100k_base".to_string(),
            context_window: 16385,
            description: "GPT-3.5 Turbo (16K context)".to_string(),
            aliases: vec![
                "gpt-3.5".to_string(),
                "gpt35".to_string(),
                "gpt-35-turbo".to_string(),
                "openai/gpt-3.5-turbo".to_string(),
            ],
        });

        // GPT-4
        registry.add_model(ModelConfig {
            name: "gpt-4".to_string(),
            encoding: "cl100k_base".to_string(),
            context_window: 128000,
            description: "GPT-4 (128K context)".to_string(),
            aliases: vec!["gpt4".to_string(), "openai/gpt-4".to_string()],
        });

        // GPT-4-turbo
        registry.add_model(ModelConfig {
            name: "gpt-4-turbo".to_string(),
            encoding: "cl100k_base".to_string(),
            context_window: 128000,
            description: "GPT-4 Turbo (128K context)".to_string(),
            aliases: vec![
                "gpt4-turbo".to_string(),
                "gpt-4turbo".to_string(),
                "openai/gpt-4-turbo".to_string(),
            ],
        });

        // GPT-4o
        registry.add_model(ModelConfig {
            name: "gpt-4o".to_string(),
            encoding: "o200k_base".to_string(),
            context_window: 128000,
            description: "GPT-4o (128K context)".to_string(),
            aliases: vec!["gpt4o".to_string(), "openai/gpt-4o".to_string()],
        });

        // Claude models
        for model in claude_models() {
            registry.add_model(model);
        }

        // Google Gemini models
        for model in google_models() {
            registry.add_model(model);
        }

        registry
    }

    /// Get the global registry instance
    pub fn global() -> &'static Self {
        static REGISTRY: OnceLock<ModelRegistry> = OnceLock::new();
        REGISTRY.get_or_init(Self::new)
    }

    /// Add a model to the registry
    fn add_model(&mut self, config: ModelConfig) {
        let canonical_name = config.name.clone();

        // Add aliases
        for alias in &config.aliases {
            self.aliases.insert(alias.to_lowercase(), canonical_name.clone());
        }

        // Add model itself as an alias (case-insensitive)
        self.aliases.insert(canonical_name.to_lowercase(), canonical_name.clone());

        self.models.insert(canonical_name, config);
    }

    /// Resolve a model name (canonical or alias) to its canonical name
    pub fn resolve_model_name(&self, name: &str) -> Result<String, TokenError> {
        let normalized = name.trim().to_lowercase();

        if let Some(canonical) = self.aliases.get(&normalized) {
            return Ok(canonical.clone());
        }

        // Model not found - generate suggestions
        let suggestion = self.generate_suggestions(&normalized);
        Err(TokenError::UnknownModel { model: name.to_string(), suggestion })
    }

    /// Get a model configuration by name (canonical or alias)
    pub fn get_model(&self, name: &str) -> Result<&ModelConfig, TokenError> {
        let canonical = self.resolve_model_name(name)?;
        Ok(self.models.get(&canonical).expect("Canonical name must exist"))
    }

    /// Create a tokenizer for the given model
    ///
    /// # Arguments
    /// * `name` - Model name (canonical or alias)
    /// * `use_accurate` - Whether to use accurate mode for models that support it (Claude API)
    ///
    /// # Returns
    /// * `Ok(Box<dyn Tokenizer>)` - Tokenizer instance for the model
    /// * `Err(TokenError)` - Model not found or tokenizer creation failed
    pub fn get_tokenizer(
        &self,
        name: &str,
        use_accurate: bool,
    ) -> Result<Box<dyn Tokenizer>, TokenError> {
        let config = self.get_model(name)?;

        // Detect tokenizer type based on encoding
        match config.encoding.as_str() {
            "anthropic-claude" => {
                // Claude tokenizer (estimation or API)
                let tokenizer = ClaudeTokenizer::new(config.clone(), use_accurate)?;
                Ok(Box::new(tokenizer))
            }
            "gemini-gemma3" => {
                // Google Gemini tokenizer
                let tokenizer = GoogleTokenizer::new(config.clone())?;
                Ok(Box::new(tokenizer))
            }
            _ => {
                // OpenAI tokenizer (tiktoken)
                let model_info = ModelInfo {
                    name: config.name.clone(),
                    encoding: config.encoding.clone(),
                    context_window: config.context_window,
                    description: config.description.clone(),
                };

                let tokenizer = OpenAITokenizer::new(&config.encoding, model_info)
                    .map_err(|e| TokenError::Tokenization(e.to_string()))?;

                Ok(Box::new(tokenizer))
            }
        }
    }

    /// List all supported models
    pub fn list_models(&self) -> Vec<&ModelConfig> {
        let mut models: Vec<&ModelConfig> = self.models.values().collect();
        models.sort_by(|a, b| a.name.cmp(&b.name));
        models
    }

    /// Generate fuzzy suggestions for unknown model names
    fn generate_suggestions(&self, name: &str) -> String {
        let mut suggestions: Vec<(&str, usize)> = self
            .models
            .keys()
            .map(|model_name| {
                let distance = strsim::levenshtein(name, &model_name.to_lowercase());
                (model_name.as_str(), distance)
            })
            .collect();

        suggestions.sort_by_key(|&(_, dist)| dist);

        let close_matches: Vec<&str> = suggestions
            .iter()
            .take(3)
            .filter(|&&(_, dist)| dist <= 3)
            .map(|&(name, _)| name)
            .collect();

        if close_matches.is_empty() {
            "Use --list-models to see all supported models".to_string()
        } else {
            format!("Did you mean: {}?", close_matches.join(", "))
        }
    }
}

impl Default for ModelRegistry {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_resolve_canonical_name() {
        let registry = ModelRegistry::new();
        assert_eq!(registry.resolve_model_name("gpt-4").unwrap(), "gpt-4");
        assert_eq!(registry.resolve_model_name("GPT-4").unwrap(), "gpt-4");
    }

    #[test]
    fn test_resolve_alias() {
        let registry = ModelRegistry::new();
        assert_eq!(registry.resolve_model_name("gpt4").unwrap(), "gpt-4");
        assert_eq!(registry.resolve_model_name("gpt35").unwrap(), "gpt-3.5-turbo");
    }

    #[test]
    fn test_unknown_model() {
        let registry = ModelRegistry::new();
        let result = registry.resolve_model_name("gpt-5");
        assert!(result.is_err());
        assert!(result.unwrap_err().to_string().contains("gpt"));
    }

    #[test]
    fn test_list_models() {
        let registry = ModelRegistry::new();
        let models = registry.list_models();
        assert_eq!(models.len(), 11); // 4 OpenAI + 3 Claude + 4 Gemini
        assert!(models.iter().any(|m| m.name == "gpt-3.5-turbo"));
        assert!(models.iter().any(|m| m.name == "gpt-4"));
        assert!(models.iter().any(|m| m.name == "gpt-4-turbo"));
        assert!(models.iter().any(|m| m.name == "gpt-4o"));
        assert!(models.iter().any(|m| m.name == "claude-opus-4-6"));
        assert!(models.iter().any(|m| m.name == "claude-sonnet-4-6"));
        assert!(models.iter().any(|m| m.name == "claude-haiku-4-5"));
        assert!(models.iter().any(|m| m.name == "gemini-2.5-pro"));
        assert!(models.iter().any(|m| m.name == "gemini-2.5-flash"));
        assert!(models.iter().any(|m| m.name == "gemini-2.5-flash-lite"));
        assert!(models.iter().any(|m| m.name == "gemini-3-pro-preview"));
    }

    #[test]
    fn test_get_tokenizer() {
        let registry = ModelRegistry::new();
        let tokenizer = registry.get_tokenizer("gpt-4", false).unwrap();
        let count = tokenizer.count_tokens("Hello world").unwrap();
        assert_eq!(count, 2);
    }

    #[test]
    fn test_get_claude_tokenizer() {
        let registry = ModelRegistry::new();
        let tokenizer = registry.get_tokenizer("claude-sonnet-4-6", false).unwrap();
        let count = tokenizer.count_tokens("Hello world").unwrap();
        // Estimation mode: "Hello world" = 11 chars
        // Prose detection: ratio < 5% → 4.5 chars/token → 11/4.5 = 2.44 → ceil = 3 tokens
        assert_eq!(count, 3);
    }

    #[test]
    fn test_claude_alias_resolution() {
        let registry = ModelRegistry::new();
        assert_eq!(registry.resolve_model_name("claude").unwrap(), "claude-sonnet-4-6");
        assert_eq!(registry.resolve_model_name("sonnet").unwrap(), "claude-sonnet-4-6");
        assert_eq!(registry.resolve_model_name("opus").unwrap(), "claude-opus-4-6");
        assert_eq!(registry.resolve_model_name("haiku").unwrap(), "claude-haiku-4-5");
    }

    #[test]
    fn test_fuzzy_suggestions() {
        let registry = ModelRegistry::new();
        let result = registry.resolve_model_name("gpt4-tubro");
        assert!(result.is_err());
        let err = result.unwrap_err();
        assert!(err.to_string().contains("Did you mean"));
        assert!(err.to_string().contains("gpt-4-turbo"));
    }
}
