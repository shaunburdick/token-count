//! Claude model definitions and metadata
//!
//! This module defines all supported Claude models with their context windows,
//! descriptions, and aliases.

use crate::tokenizers::registry::ModelConfig;

/// Get all Claude model configurations
///
/// Returns a vector of ModelConfig for all supported Claude models.
/// Models are ordered by generation (newest first) and tier (Opus > Sonnet > Haiku).
pub fn claude_models() -> Vec<ModelConfig> {
    vec![
        // Claude 4.6 Generation (Latest - February 2026)
        ModelConfig {
            name: "claude-opus-4-6".to_string(),
            encoding: "anthropic-claude".to_string(),
            context_window: 1_000_000,
            description: "Claude Opus 4.6 - Most capable, 1M context (Feb 2026)".to_string(),
            aliases: vec![
                "opus-4-6".to_string(),
                "opus-4.6".to_string(),
                "opus".to_string(),
                "claude-opus-4.6".to_string(),
                "anthropic/claude-opus-4-6".to_string(),
            ],
        },
        ModelConfig {
            name: "claude-sonnet-4-6".to_string(),
            encoding: "anthropic-claude".to_string(),
            context_window: 1_000_000,
            description: "Claude Sonnet 4.6 - Balanced, 1M context (Feb 2026, recommended)"
                .to_string(),
            aliases: vec![
                "sonnet-4-6".to_string(),
                "sonnet-4.6".to_string(),
                "sonnet".to_string(),
                "claude".to_string(), // Default alias
                "claude-sonnet-4.6".to_string(),
                "anthropic/claude-sonnet-4-6".to_string(),
            ],
        },
        ModelConfig {
            name: "claude-haiku-4-5".to_string(),
            encoding: "anthropic-claude".to_string(),
            context_window: 200_000,
            description: "Claude Haiku 4.5 - Fast and efficient, 200K context".to_string(),
            aliases: vec![
                "haiku-4-5".to_string(),
                "haiku-4.5".to_string(),
                "haiku".to_string(),
                "claude-haiku-4.5".to_string(),
                "anthropic/claude-haiku-4-5".to_string(),
            ],
        },
    ]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_claude_models_count() {
        let models = claude_models();
        assert_eq!(models.len(), 3, "Should have 3 Claude models");
    }

    #[test]
    fn test_claude_default_alias() {
        let models = claude_models();
        let sonnet = models.iter().find(|m| m.name == "claude-sonnet-4-6");
        assert!(sonnet.is_some());
        assert!(
            sonnet.unwrap().aliases.contains(&"claude".to_string()),
            "claude alias should point to sonnet-4-6"
        );
    }

    #[test]
    fn test_all_models_have_anthropic_encoding() {
        let models = claude_models();
        for model in models {
            assert_eq!(
                model.encoding, "anthropic-claude",
                "All Claude models should use anthropic-claude encoding"
            );
        }
    }

    #[test]
    fn test_context_windows() {
        let models = claude_models();
        let opus = models.iter().find(|m| m.name == "claude-opus-4-6").unwrap();
        let sonnet = models.iter().find(|m| m.name == "claude-sonnet-4-6").unwrap();
        let haiku = models.iter().find(|m| m.name == "claude-haiku-4-5").unwrap();

        assert_eq!(opus.context_window, 1_000_000);
        assert_eq!(sonnet.context_window, 1_000_000);
        assert_eq!(haiku.context_window, 200_000);
    }

    #[test]
    fn test_all_models_have_aliases() {
        let models = claude_models();
        for model in models {
            assert!(!model.aliases.is_empty(), "Model {} should have aliases", model.name);
        }
    }
}
