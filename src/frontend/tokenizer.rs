#[derive(Debug, PartialEq)]
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

pub struct BrainfuckTokenizer;

impl BrainfuckTokenizer {
    pub fn tokenize(input: &str) -> Vec<Token> {
        let mut tokens = Vec::new();
        let mut chars = input.chars();

        while let Some(c) = chars.next() {
            match c {
                '>' => tokens.push(Token::MoveRight),
                '<' => tokens.push(Token::MoveLeft),
                '+' => tokens.push(Token::Increase),
                '-' => tokens.push(Token::Decrease),
                '.' => tokens.push(Token::Output),
                ',' => tokens.push(Token::Input),
                '[' => tokens.push(Token::LoopLeft),
                ']' => tokens.push(Token::LoopRight),
                _ => {} // Ignore invalid characters
            }
        }

        tokens
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_tokenize() {
        let input = "++>[-<+>].";
        let tokens = BrainfuckTokenizer::tokenize(input);

        assert_eq!(
            tokens,
            vec![
                Token::Increase,
                Token::Increase,
                Token::MoveRight,
                Token::LoopLeft,
                Token::MoveLeft,
                Token::Increase,
                Token::MoveRight,
                Token::LoopRight,
                Token::Output,
            ]
        );
    }

    #[test]
    fn test_tokenize_with_invalid_chars() {
        let input = "++>[-<+>].invalid";
        let tokens = BrainfuckTokenizer::tokenize(input);

        assert_eq!(
            tokens,
            vec![
                Token::Increase,
                Token::Increase,
                Token::MoveRight,
                Token::LoopLeft,
                Token::MoveLeft,
                Token::Increase,
                Token::MoveRight,
                Token::LoopRight,
                Token::Output,
            ]
        );
    }
}
