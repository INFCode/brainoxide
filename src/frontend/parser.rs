use crate::frontend::tokenizer::Token;
use crate::ir::{CodeBlock, Instruction, Program};

pub struct BrainfuckParser;

pub enum ParseError {
    UnmatchedStartOfLoop,
    UnmatchedEndOfLoop,
}

impl BrainfuckParser {
    pub fn parse(tokens: Vec<Token>) -> Result<Program, ParseError> {
        let mut loop_stack = Vec::new();
        let mut current_pos = 0usize;
        let mut current_block = CodeBlock::new();
        while let Some(token) = tokens.get(current_pos) {
            current_pos += 1;
            match token {
                Token::MoveRight => current_block.add_instruction(Instruction::Move(1)),
                Token::MoveLeft => current_block.add_instruction(Instruction::Move(-1)),
                Token::Increase => current_block.add_instruction(Instruction::Add(1)),
                Token::Decrease => current_block.add_instruction(Instruction::Add(-1)),
                Token::Output => current_block.add_instruction(Instruction::Output),
                Token::Input => current_block.add_instruction(Instruction::Input),
                Token::LoopLeft => {
                    loop_stack.push(current_block);
                    current_block = CodeBlock::new();
                }
                Token::LoopRight => {
                    if let Some(mut upper_block) = loop_stack.pop() {
                        upper_block.add_instruction(Instruction::Loop(current_block));
                        current_block = upper_block;
                    } else {
                        return Err(ParseError::UnmatchedEndOfLoop);
                    }
                }
            }
        }
        if !loop_stack.is_empty() {
            return Err(ParseError::UnmatchedStartOfLoop);
        }

        Ok(Program::new(current_block))
    }
}
