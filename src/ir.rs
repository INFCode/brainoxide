#[derive(Debug, PartialEq)]
pub enum Instruction {
    Move(isize),
    Add(isize),
    Output,
    Input,
    Loop(CodeBlock),
}

#[derive(Debug, PartialEq)]
pub struct CodeBlock {
    instrudtions: Vec<Instruction>,
}

impl CodeBlock {
    pub fn new() -> CodeBlock {
        CodeBlock {
            instrudtions: Vec::new(),
        }
    }

    pub fn add_instruction(&mut self, inst: Instruction) {
        self.instrudtions.push(inst);
    }
}

#[derive(Debug, PartialEq)]
pub struct Program {
    code: CodeBlock,
}

impl Program {
    pub fn new(code: CodeBlock) -> Self {
        Self { code }
    }
}
