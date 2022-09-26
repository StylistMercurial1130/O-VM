use std::fs;
#[derive(Debug)]
pub enum InsturctionType {
    Push ,
    Pop ,
    Halt ,
    Add , 
    Subtract ,
    Unknown
}

pub struct InstructionToken {

    pub literal_type : InsturctionType,
    pub operand : i64

}

impl InstructionToken {

    pub fn new(token_type : InsturctionType , operand : i64) -> InstructionToken {
        InstructionToken { literal_type: token_type, operand: operand}
    }

}

impl InsturctionType {

    pub fn generate_type_from_string(literal : &str) -> InsturctionType {

        match literal {
            "push" => return InsturctionType::Push,
            "pop" => return InsturctionType::Pop,
            "halt" => return InsturctionType::Halt,
            "add" => return InsturctionType::Add,
            "sub" => return InsturctionType::Subtract,
            _=> return InsturctionType::Unknown, 
        }

    }

}

fn debug_assembler(token : &InstructionToken) {

    match token.literal_type {
        InsturctionType::Add => println!("add {}",token.operand),
        InsturctionType::Pop => println!("pop {}",token.operand),
        InsturctionType::Halt => println!("halt {}",token.operand),
        InsturctionType::Push => println!("push {}",token.operand),
        InsturctionType::Subtract => println!("sub {}",token.operand),
        InsturctionType::Unknown => println!("{:?}",token.literal_type),
    };

}

pub fn assemble(file_path : &str) -> Vec<InstructionToken>{

    let file_content = fs::read_to_string(file_path)
                    .expect("file not found !");

    let mut instructions : Vec<InstructionToken> = Vec::new();

    for lines in file_content.lines() {
        let instruction : Vec<&str> = lines.split(' ').collect();
        let literal = instruction[0];
        let mut operand = 0;
        if instruction.len() == 2 {
            operand = instruction[1].parse().unwrap();
        }
        instructions.push({
            InstructionToken::new(
                InsturctionType::generate_type_from_string(literal),
                operand
            )
        });
    }
    instructions

}