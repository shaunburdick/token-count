# Anthropic API Contract

**Feature**: 003-claude-support  
**API**: Anthropic Messages API (Token Counting)  
**Version**: 2023-06-01  
**Documentation**: https://docs.anthropic.com/en/docs/build-with-claude/token-counting

---

## Endpoint

```
POST https://api.anthropic.com/v1/messages/count_tokens
```

---

## Authentication

**Method**: API Key in header

**Header**:
```
x-api-key: sk-ant-api03-...
```

**API Key Format**:
- Prefix: `sk-ant-api03-` (as of 2026)
- Obtained from: https://console.anthropic.com/

---

## Request Headers

**Required**:
```
x-api-key: <api_key>
anthropic-version: 2023-06-01
Content-Type: application/json
```

**Optional**:
```
User-Agent: token-count/<version> (Rust)
```

---

## Request Body

### Schema

```json
{
  "model": "string (required)",
  "messages": [
    {
      "role": "string (required, must be 'user' or 'assistant')",
      "content": "string (required)"
    }
  ]
}
```

### Example

```json
{
  "model": "claude-sonnet-4-6",
  "messages": [
    {
      "role": "user",
      "content": "Hello, Claude! How are you today?"
    }
  ]
}
```

### Field Constraints

- **model**: Must be valid Claude model ID (e.g., `claude-sonnet-4-6`, `claude-opus-4-6`)
- **messages**: Array must have at least 1 message
- **role**: Must be `"user"` or `"assistant"` (our use case always uses `"user"`)
- **content**: UTF-8 string, no max length specified (limited by context window)

---

## Response Body

### Success Response (200 OK)

```json
{
  "input_tokens": 8
}
```

**Field**:
- `input_tokens` (integer): Number of tokens in the input messages

### Error Response (4xx / 5xx)

```json
{
  "error": {
    "type": "string",
    "message": "string"
  }
}
```

**Error Types**:
- `invalid_request_error`: Malformed request (missing fields, invalid JSON)
- `authentication_error`: Invalid or missing API key
- `permission_error`: API key lacks required permissions
- `rate_limit_error`: Too many requests
- `api_error`: Server-side error

---

## HTTP Status Codes

| Code | Meaning | Action |
|------|---------|--------|
| 200 | Success | Parse `input_tokens` from response |
| 400 | Bad Request | Log error, return `TokenError::ApiError` |
| 401 | Unauthorized | Return `TokenError::InvalidApiKey` |
| 403 | Forbidden | Return `TokenError::ApiError` (insufficient permissions) |
| 429 | Too Many Requests | Retry with backoff or return `TokenError::RateLimited` |
| 500 | Server Error | Retry with backoff or return `TokenError::ApiServerError` |
| 503 | Service Unavailable | Retry with backoff or return `TokenError::ApiServerError` |

---

## Rate Limits

**As of 2026**:
- Free tier: 100 requests per minute (RPM)
- Paid tier: Varies by plan (typically 1000+ RPM)

**Headers** (included in response):
```
x-ratelimit-requests-limit: 100
x-ratelimit-requests-remaining: 99
x-ratelimit-requests-reset: 2026-03-14T12:00:00Z
```

**Rate Limit Response** (429):
```json
{
  "error": {
    "type": "rate_limit_error",
    "message": "Rate limit exceeded. Please try again later."
  }
}
```

**Retry-After Header** (optional):
```
Retry-After: 60
```

---

## Example Requests

### Minimal Request

```bash
curl -X POST https://api.anthropic.com/v1/messages/count_tokens \
  -H "x-api-key: sk-ant-api03-..." \
  -H "anthropic-version: 2023-06-01" \
  -H "Content-Type: application/json" \
  -d '{
    "model": "claude-sonnet-4-6",
    "messages": [{"role": "user", "content": "Hello"}]
  }'
```

**Response**:
```json
{
  "input_tokens": 2
}
```

### Code Input Request

```bash
curl -X POST https://api.anthropic.com/v1/messages/count_tokens \
  -H "x-api-key: sk-ant-api03-..." \
  -H "anthropic-version: 2023-06-01" \
  -H "Content-Type: application/json" \
  -d '{
    "model": "claude-opus-4-6",
    "messages": [{
      "role": "user",
      "content": "fn main() {\n    println!(\"Hello, world!\");\n}"
    }]
  }'
```

**Response**:
```json
{
  "input_tokens": 18
}
```

### Large Input Request

```bash
# Large text (10KB+)
curl -X POST https://api.anthropic.com/v1/messages/count_tokens \
  -H "x-api-key: sk-ant-api03-..." \
  -H "anthropic-version: 2023-06-01" \
  -H "Content-Type: application/json" \
  -d @large-input.json
```

Where `large-input.json`:
```json
{
  "model": "claude-sonnet-4-6",
  "messages": [
    {
      "role": "user",
      "content": "... 10KB of text ..."
    }
  ]
}
```

---

## Error Examples

### 401 Unauthorized (Invalid API Key)

**Request**:
```bash
curl -X POST https://api.anthropic.com/v1/messages/count_tokens \
  -H "x-api-key: invalid-key" \
  -H "anthropic-version: 2023-06-01" \
  -d '{"model": "claude-sonnet-4-6", "messages": [{"role": "user", "content": "test"}]}'
```

**Response** (401):
```json
{
  "error": {
    "type": "authentication_error",
    "message": "invalid x-api-key"
  }
}
```

### 400 Bad Request (Missing Field)

**Request**:
```bash
curl -X POST https://api.anthropic.com/v1/messages/count_tokens \
  -H "x-api-key: sk-ant-..." \
  -H "anthropic-version: 2023-06-01" \
  -d '{"model": "claude-sonnet-4-6"}'
```

**Response** (400):
```json
{
  "error": {
    "type": "invalid_request_error",
    "message": "messages: field required"
  }
}
```

### 429 Rate Limit Exceeded

**Request** (101st request in 1 minute):
```bash
curl -X POST https://api.anthropic.com/v1/messages/count_tokens \
  -H "x-api-key: sk-ant-..." \
  -H "anthropic-version: 2023-06-01" \
  -d '{"model": "claude-sonnet-4-6", "messages": [{"role": "user", "content": "test"}]}'
```

**Response** (429):
```json
{
  "error": {
    "type": "rate_limit_error",
    "message": "Number of request tokens has exceeded your per-minute rate limit"
  }
}
```

**Headers**:
```
Retry-After: 60
x-ratelimit-requests-remaining: 0
x-ratelimit-requests-reset: 2026-03-14T12:01:00Z
```

### 500 Server Error

**Response** (500):
```json
{
  "error": {
    "type": "api_error",
    "message": "Internal server error"
  }
}
```

---

## Implementation Notes

### Timeout

**Recommended**: 30 seconds
- API typically responds in 100-500ms
- Large inputs (100KB+) may take 1-2 seconds
- 30s provides buffer for network latency

### Retry Strategy

**Recommended**:
- Max attempts: 3
- Backoff: Exponential (2s, 4s, 8s)
- Retry on: 429, 500, 503, network errors
- Don't retry on: 400, 401, 403

**Pseudocode**:
```rust
for attempt in 0..3 {
    match try_count_tokens() {
        Ok(count) => return Ok(count),
        Err(e) if is_retryable(e) && attempt < 2 => {
            sleep(2^attempt * 1000ms);
            continue;
        }
        Err(e) => return Err(e),
    }
}
```

### Error Handling

**Map HTTP status to TokenError**:
```rust
match status_code {
    200 => parse_response(body),
    401 => TokenError::InvalidApiKey,
    429 => TokenError::RateLimited,
    500..=599 => TokenError::ApiServerError(status_code),
    _ => TokenError::ApiError(format!("HTTP {}", status_code)),
}
```

### Request Construction

**Important**: Always use `"user"` role for our use case
```rust
let request = CountTokensRequest {
    model: model.to_string(),
    messages: vec![Message {
        role: "user".to_string(),  // Always "user" for token counting
        content: text.to_string(),
    }],
};
```

---

## Testing

### Unit Tests (Mock API)

```rust
#[test]
fn test_parse_success_response() {
    let json = r#"{"input_tokens": 42}"#;
    let response: CountTokensResponse = serde_json::from_str(json).unwrap();
    assert_eq!(response.input_tokens, 42);
}

#[test]
fn test_parse_error_response() {
    let json = r#"{"error": {"type": "authentication_error", "message": "invalid key"}}"#;
    let response: ApiErrorResponse = serde_json::from_str(json).unwrap();
    assert_eq!(response.error.error_type, "authentication_error");
}
```

### Integration Tests (Real API, gated)

```rust
#[tokio::test]
#[ignore] // Run only when ANTHROPIC_API_KEY is set
async fn test_real_api() {
    let api_key = env::var("ANTHROPIC_API_KEY").expect("ANTHROPIC_API_KEY not set");
    let client = ClaudeApiClient::new(api_key).unwrap();
    
    let count = client.count_tokens("claude-sonnet-4-6", "Hello, world!").await.unwrap();
    assert!(count >= 2 && count <= 5); // Reasonable range
}
```

---

## Security Considerations

1. **API Key Storage**: Never log or print API key (sanitize logs)
2. **HTTPS Only**: Always use HTTPS, never fallback to HTTP
3. **Certificate Validation**: Don't disable certificate checks
4. **Timeout**: Prevent indefinite hangs with 30s timeout
5. **Input Sanitization**: Validate UTF-8 before sending

---

## Future Considerations

### Streaming API (Not Yet Available)

Anthropic may add streaming token counting in future:
```
POST /v1/messages/count_tokens/stream
```

Would return:
```
data: {"input_tokens": 42, "output_tokens": 0}
```

**Not implemented in this feature** (API doesn't support it yet)

### Batch API (Future Enhancement)

Potential future optimization:
```json
{
  "requests": [
    {"model": "...", "messages": [...]},
    {"model": "...", "messages": [...]}
  ]
}
```

**Not implemented in this feature** (API doesn't support it yet)

---

## Related Documents

- [Data Model](../data-model.md) - Rust types for request/response
- [Anthropic API Docs](https://docs.anthropic.com/en/docs/build-with-claude/token-counting)
- [Rate Limits](https://docs.anthropic.com/en/api/rate-limits)
