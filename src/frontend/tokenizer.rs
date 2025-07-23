use encoding::{Encoding, all::UTF_8};
use std::{borrow::Cow, io::BufRead};
use thiserror::Error;

#[derive(Debug, Default)]
pub struct TokenizerConfig;

impl TokenizerConfig {
    pub fn new() -> Self {
        Self
    }
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum Token {
    MoveRight,
    MoveLeft,
    Increase,
    Decrease,
    Output,
    Input,
    LoopLeft,
    LoopRight,
}

#[derive(Error, Debug)]
pub enum TokenizeError {
    #[error("failed to read the provided input")]
    Io(#[from] std::io::Error),
    #[error("cannot decode the input into {0}: {1}")]
    Decode(String, Cow<'static, str>), // The Cow string cannot be annotated as #[source] because
                                       // it doesn't implement or dereference to std::error::Error
}

pub struct BrainfuckTokenizer {
    #[allow(dead_code)] // config is not implemented yet
    config: TokenizerConfig,
}

impl BrainfuckTokenizer {
    pub fn new(config: TokenizerConfig) -> Self {
        Self { config }
    }

    pub fn tokenize<R: BufRead>(&self, mut input: R) -> Result<Vec<Token>, TokenizeError> {
        let mut tokens = Vec::new();
        let mut program_buffer = Vec::new();
        _ = input.read_to_end(&mut program_buffer)?;
        let program = UTF_8
            .decode(program_buffer.as_slice(), encoding::DecoderTrap::Strict)
            .map_err(|msg| TokenizeError::Decode("UTF-8".to_string(), msg))?;

        for c in program.chars() {
            match c as char {
                '>' => tokens.push(Token::MoveRight),
                '<' => tokens.push(Token::MoveLeft),
                '+' => tokens.push(Token::Increase),
                '-' => tokens.push(Token::Decrease),
                '.' => tokens.push(Token::Output),
                ',' => tokens.push(Token::Input),
                '[' => tokens.push(Token::LoopLeft),
                ']' => tokens.push(Token::LoopRight),
                _ => {} // Ignore other characters
            }
        }
        Ok(tokens)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_tokenize() {
        let input = "++>[-<+>].";
        let tokenizer = BrainfuckTokenizer::new(TokenizerConfig::new());
        let tokens = tokenizer.tokenize(input.as_bytes());

        assert!(tokens.is_ok());
        assert_eq!(
            tokens.unwrap(),
            vec![
                Token::Increase,
                Token::Increase,
                Token::MoveRight,
                Token::LoopLeft,
                Token::Decrease,
                Token::MoveLeft,
                Token::Increase,
                Token::MoveRight,
                Token::LoopRight,
                Token::Output,
            ]
        );
    }

    #[test]
    fn test_tokenize_with_meaningless_chars() {
        let input = "++>[-<+>].meaningless";
        let tokenizer = BrainfuckTokenizer::new(TokenizerConfig::new());
        let tokens = tokenizer.tokenize(input.as_bytes());

        assert!(tokens.is_ok());
        assert_eq!(
            tokens.unwrap(),
            vec![
                Token::Increase,
                Token::Increase,
                Token::MoveRight,
                Token::LoopLeft,
                Token::Decrease,
                Token::MoveLeft,
                Token::Increase,
                Token::MoveRight,
                Token::LoopRight,
                Token::Output,
            ]
        );
    }

    #[test]
    fn test_tokenize_with_wrong_encoding() {
        use encoding::all::GBK;
        let input = GBK
            .encode("++>[-<+>].你好", encoding::EncoderTrap::Strict)
            .unwrap();
        let tokenizer = BrainfuckTokenizer::new(TokenizerConfig::new());
        let tokens = tokenizer.tokenize(input.as_slice());

        assert!(tokens.is_err());
        assert_eq!(
            tokens.unwrap_err().to_string(),
            "Cannot decode the input into UTF-8: invalid sequence".to_string()
        );
    }
}
