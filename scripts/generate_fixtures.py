#!/usr/bin/env python3
"""Generate tokenization reference fixtures for testing."""

import json
import sys

try:
    import tiktoken
except ImportError:
    print(
        "Error: tiktoken not installed. Run: pip install tiktoken==0.5.2",
        file=sys.stderr,
    )
    sys.exit(1)

test_cases = [
    # Basic ASCII
    "Hello world",
    "The quick brown fox jumps over the lazy dog",
    "",
    " ",
    "\n",
    "a",
    # Unicode
    "Hello 世界 🌍",
    "Emoji test: 🎉🎊🎈",
    "Japanese: こんにちは世界",
    "Arabic: مرحبا بالعالم",
    "Mixed: Hello 世界 test 123",
    # Edge cases
    "a" * 100,  # Repeated chars
    "Word " * 50,  # Repeated words
    "Mixed 123 !@# symbols $%^",
    "\n\n\n",  # Multiple newlines
    "  spaces  everywhere  ",
    # Code-like text
    "def hello():\n    print('world')",
    "const x = 42;",
    "import React from 'react';",
    # Longer samples
    "This is a longer piece of text that should tokenize into multiple tokens. " * 5,
]

encodings = {
    "gpt2": "gpt2",
    "p50k_base": "text-davinci-003",
    "cl100k_base": "gpt-4",
    "o200k_base": "gpt-4o",
}

fixtures = {}
for enc_name, model in encodings.items():
    try:
        enc = tiktoken.encoding_for_model(model)
    except KeyError:
        # Fall back to get_encoding if model not found
        enc = tiktoken.get_encoding(enc_name)

    fixtures[enc_name] = []
    for text in test_cases:
        token_count = len(enc.encode(text))
        fixtures[enc_name].append({"input": text, "expected_tokens": token_count})

# Write fixtures
output_file = "tests/fixtures/tokenization_reference.json"
with open(output_file, "w", encoding="utf-8") as f:
    json.dump(fixtures, f, indent=2, ensure_ascii=False)

print(f"✅ Generated {sum(len(v) for v in fixtures.values())} test fixtures")
print(f"   Output: {output_file}")
print(f"\nFixtures by encoding:")
for enc_name, cases in fixtures.items():
    print(f"   {enc_name}: {len(cases)} test cases")
