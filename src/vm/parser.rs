use super::lexer::TokenType;
use super::lexer::Token;

pub struct Node {
    value       : Option<String>,
    child_nodes : Vec<Box<Node>>
}

pub struct Scanner {
    scanner_token_list              : Vec<Token>,
    scanner_current_token_position  : usize,
}

impl Node {
    
    pub fn new() -> Box<Node> {
        Box::new(
            Node {
                value       : None,
                child_nodes : Vec::new()
            }
        )
    }

}

impl Scanner {
  
    pub fn new(token_list : Vec<Token>) -> Scanner {
        Scanner {
            scanner_token_list              : token_list,
            scanner_current_token_position  :  0
        }     
    }

  
}
