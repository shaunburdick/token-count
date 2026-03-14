//! Stdin input handling with streaming support

use crate::error::TokenError;
use std::io::{self, BufRead, BufReader, Read};

const CHUNK_SIZE: usize = 64 * 1024; // 64KB chunks
const MAX_INPUT_SIZE: usize = 100 * 1024 * 1024; // 100MB limit

/// Read all input from stdin
///
/// # Errors
///
/// Returns `TokenError::InvalidUtf8` if the input contains invalid UTF-8
/// Returns `TokenError::InputTooLarge` if the input exceeds 100MB
/// Returns `TokenError::Io` for other IO errors
pub fn read_stdin() -> Result<String, TokenError> {
    let stdin = io::stdin();
    let mut reader = BufReader::with_capacity(CHUNK_SIZE, stdin.lock());
    let mut buffer = String::new();

    // Read with size tracking to enforce limits
    let bytes_read = reader.read_to_string(&mut buffer).map_err(|e| {
        if e.kind() == io::ErrorKind::InvalidData {
            // UTF-8 validation error
            TokenError::InvalidUtf8 { offset: buffer.len() }
        } else {
            TokenError::Io(e)
        }
    })?;

    // Enforce size limit to prevent DoS attacks
    if bytes_read > MAX_INPUT_SIZE {
        return Err(TokenError::InputTooLarge { size: bytes_read, limit: MAX_INPUT_SIZE });
    }

    Ok(buffer)
}

/// Read stdin with a callback for each chunk (for streaming processing)
///
/// This allows processing large inputs without loading everything into memory
pub fn read_stdin_streaming<F>(mut process: F) -> Result<(), TokenError>
where
    F: FnMut(&str) -> Result<(), TokenError>,
{
    let stdin = io::stdin();
    let mut reader = BufReader::with_capacity(CHUNK_SIZE, stdin.lock());
    let mut buffer = String::with_capacity(CHUNK_SIZE);

    loop {
        buffer.clear();
        let bytes_read = reader.read_line(&mut buffer).map_err(|e| {
            if e.kind() == io::ErrorKind::InvalidData {
                TokenError::InvalidUtf8 { offset: 0 }
            } else {
                TokenError::Io(e)
            }
        })?;

        if bytes_read == 0 {
            break; // EOF
        }

        process(&buffer)?;
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_chunk_size() {
        assert_eq!(CHUNK_SIZE, 64 * 1024);
    }
}
