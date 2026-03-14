//! Google Gemini model definitions and metadata
//!
//! Note: Model names must match those supported by the gemini-tokenizer crate (v0.2.0).
//! All Gemini models use the same Gemma 3 SentencePiece tokenizer.

use crate::tokenizers::registry::ModelConfig;

/// Get all Google Gemini model configurations
///
/// Returns a vector of ModelConfig for all supported Gemini models.
/// Models are ordered by generation (GA first for stability) and tier (Pro > Flash > Lite).
///
/// Note: Only includes models supported by gemini-tokenizer v0.2.0. Additional models
/// may be added as the upstream library expands support.
pub fn google_models() -> Vec<ModelConfig> {
    vec![
        // Gemini 2.5 Series (GA, deprecated June 2026)
        ModelConfig {
            name: "gemini-2.5-pro".to_string(),
            encoding: "gemini-gemma3".to_string(),
            context_window: 1_000_000,
            description: "Pro model, 1M context (GA, deprecated June 2026)".to_string(),
            aliases: vec![
                "gemini-pro".to_string(),
                "gemini-2-pro".to_string(),
                "gemini-2.5".to_string(),
                "google/gemini-pro".to_string(),
            ],
        },
        ModelConfig {
            name: "gemini-2.5-flash".to_string(),
            encoding: "gemini-gemma3".to_string(),
            context_window: 1_000_000,
            description: "Flash model, 1M context (GA, deprecated June 2026, default)".to_string(),
            aliases: vec![
                "gemini".to_string(),
                "gemini-flash".to_string(),
                "gemini-2-flash".to_string(),
                "google/gemini".to_string(),
                "google/gemini-flash".to_string(),
            ],
        },
        ModelConfig {
            name: "gemini-2.5-flash-lite".to_string(),
            encoding: "gemini-gemma3".to_string(),
            context_window: 1_000_000,
            description: "Flash Lite model, 1M context (GA, deprecated June 2026, fastest)"
                .to_string(),
            aliases: vec![
                "gemini-lite".to_string(),
                "gemini-2-lite".to_string(),
                "gemini-2.5-lite".to_string(),
                "google/gemini-lite".to_string(),
            ],
        },
        // Gemini 3.x Series (Preview)
        ModelConfig {
            name: "gemini-3-pro-preview".to_string(),
            encoding: "gemini-gemma3".to_string(),
            context_window: 1_000_000,
            description: "Pro model, 1M context (Preview)".to_string(),
            aliases: vec!["gemini-3-pro".to_string(), "gemini-3".to_string()],
        },
    ]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_google_models_count() {
        let models = google_models();
        assert_eq!(models.len(), 4, "Should have 4 Gemini models");
    }

    #[test]
    fn test_default_alias() {
        let models = google_models();
        let flash = models.iter().find(|m| m.name == "gemini-2.5-flash");
        assert!(flash.is_some());
        assert!(
            flash.unwrap().aliases.contains(&"gemini".to_string()),
            "gemini alias should point to gemini-2.5-flash"
        );
    }

    #[test]
    fn test_all_have_1m_context() {
        let models = google_models();
        for model in models {
            assert_eq!(model.context_window, 1_000_000, "All models should have 1M context");
        }
    }

    #[test]
    fn test_all_use_same_encoding() {
        let models = google_models();
        for model in models {
            assert_eq!(
                model.encoding, "gemini-gemma3",
                "All models should use gemini-gemma3 encoding"
            );
        }
    }
}
