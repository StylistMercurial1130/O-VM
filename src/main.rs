use regex::Regex;
mod vm;
use vm::lexer::Lexer;
use std::fs;
fn main() {

    // let mut virtual_machine_context = 
    //     vm::ovm::VirtualMachineContext::create_context("./test.oasm");
    
    let file_content = fs::read_to_string("./test.oasm")
                    .expect("file not found !");

    println!("{}",file_content);

    let mut lex = Lexer::new();
    lex.tokenize(&file_content);
//    virtual_machine_context.run(); 
}
