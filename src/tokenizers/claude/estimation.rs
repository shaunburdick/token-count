//! Adaptive token estimation for Claude models
//!
//! This module provides content-aware estimation that adapts based on whether
//! the input is code, prose, or mixed content.

/// Classification of text content for adaptive token estimation
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ContentType {
    /// Code-heavy content (>15% code indicators)
    /// Examples: source files, JSON, config files
    /// Token ratio: ~3.0 chars/token
    Code,

    /// Prose-heavy content (<5% code indicators)
    /// Examples: documentation, natural language, articles
    /// Token ratio: ~4.5 chars/token
    Prose,

    /// Mixed content (5-15% code indicators)
    /// Examples: markdown with code blocks, technical docs
    /// Token ratio: ~3.75 chars/token
    Mixed,
}

impl ContentType {
    /// Get the estimated characters per token for this content type
    pub fn chars_per_token(&self) -> f64 {
        match self {
            Self::Code => 3.0,
            Self::Prose => 4.5,
            Self::Mixed => 3.75,
        }
    }
}

/// Detect the content type of the given text
pub fn detect_content_type(text: &str) -> ContentType {
    let total_chars = text.chars().count();
    if total_chars == 0 {
        return ContentType::Prose;
    }

    let code_indicators = count_code_indicators(text);
    let ratio = code_indicators as f64 / total_chars as f64;

    if ratio > 0.15 {
        ContentType::Code
    } else if ratio > 0.05 {
        ContentType::Mixed
    } else {
        ContentType::Prose
    }
}

/// Count code indicators in the text
fn count_code_indicators(text: &str) -> usize {
    let mut count = 0;

    // Count structural code characters
    for ch in text.chars() {
        if matches!(ch, '{' | '}' | '[' | ']' | '(' | ')' | ';' | ':' | ',' | '<' | '>') {
            count += 1;
        }
    }

    // Count code keywords (simple substring matching)
    let keywords = [
        "fn ",
        "def ",
        "function ",
        "const ",
        "let ",
        "var ",
        "import ",
        "export ",
        "class ",
        "struct ",
        "enum ",
        "impl ",
        "trait ",
        "interface ",
        "if (",
        "for (",
        "while (",
        "return ",
        "async ",
        "await ",
        "//",
        "/*",
        "*/",
        "#include",
        "#define",
    ];

    for keyword in &keywords {
        count += text.matches(keyword).count() * keyword.len();
    }

    count
}

/// Estimate token count using adaptive algorithm
pub fn estimate_tokens(text: &str) -> usize {
    if text.is_empty() {
        return 0;
    }

    let content_type = detect_content_type(text);
    let char_count = text.chars().count();
    let ratio = content_type.chars_per_token();

    (char_count as f64 / ratio).ceil() as usize
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_estimate_empty() {
        assert_eq!(estimate_tokens(""), 0);
    }

    #[test]
    fn test_estimate_single_char() {
        assert_eq!(estimate_tokens("a"), 1);
    }

    #[test]
    fn test_detect_code() {
        let code = "fn main() { println!(\"test\"); }";
        assert_eq!(detect_content_type(code), ContentType::Code);
    }

    #[test]
    fn test_detect_prose() {
        let prose = "The quick brown fox jumps over the lazy dog.";
        assert_eq!(detect_content_type(prose), ContentType::Prose);
    }

    #[test]
    fn test_detect_mixed() {
        let mixed = "## Title\n\nSome text\n\n```rust\nfn test() {}\n```\n\nMore text.";
        assert_eq!(detect_content_type(mixed), ContentType::Mixed);
    }

    #[test]
    fn test_estimate_code() {
        let code = "fn main() {}"; // 12 chars
        let tokens = estimate_tokens(code);
        // 12 / 3.0 = 4 tokens
        assert_eq!(tokens, 4);
    }

    #[test]
    fn test_estimate_prose() {
        let prose = "Hello world!"; // 12 chars
        let tokens = estimate_tokens(prose);
        // 12 / 4.5 = 2.67 -> ceil = 3 tokens
        assert_eq!(tokens, 3);
    }

    #[test]
    fn test_detect_json() {
        let json = r#"{"key": "value", "count": 42}"#;
        assert_eq!(detect_content_type(json), ContentType::Code);
    }

    #[test]
    fn test_detect_python() {
        let python =
            "def fibonacci(n):\n    return n if n < 2 else fibonacci(n-1) + fibonacci(n-2)";
        assert_eq!(detect_content_type(python), ContentType::Code);
    }

    #[test]
    fn test_chars_per_token() {
        assert_eq!(ContentType::Code.chars_per_token(), 3.0);
        assert_eq!(ContentType::Prose.chars_per_token(), 4.5);
        assert_eq!(ContentType::Mixed.chars_per_token(), 3.75);
    }
}
