
#[derive(Debug)]
pub enum TokenType {
    Instruction,
    Label,
    HexValue,
    DecimalValue,
    Literal
}

pub struct Token {
    pub token_type : TokenType,
    pub literal : String,
}

impl Token {
    pub fn new(token_type : TokenType,literal : &str) -> Token {
        Token {
            token_type : token_type,
            literal : String::from(literal)
        }
    } 
}

#[allow(non_camel_case_types)]
#[derive(PartialEq, Eq)]
#[derive(Debug)]
pub enum LexerState {
    NO_STATE , 
    INSTRUCTION_MNEMONIC ,
    HEX_VALUE ,
    DECIMAL_VALUE ,
    LABEL , 
    LITERAL,
}

pub struct Lexer {
   internal_state : LexerState,
   current_character : char,
   current_position : usize,
   string_buffer : String
}

impl Lexer {

    pub fn new() -> Lexer {
        Lexer {
            internal_state : LexerState::NO_STATE,
            current_character : char::REPLACEMENT_CHARACTER,
            current_position : 0,
            string_buffer : String::new()
        }
    }

    pub fn tokenize(&mut self,program : &str) -> Vec<Token> {
        let _program = program.to_string();
        let mut tokens : Vec<Token> = Vec::new();
        for line in _program.lines() {
            while self.current_position < line.len() {
                self.current_character = line.as_bytes()[self.current_position] as char;
                match self.current_character  {
                    ' ' => {
                        //tokenize
                        match self.internal_state {
                            LexerState::INSTRUCTION_MNEMONIC => {
                                tokens.push(
                                    Token::new(
                                        TokenType::Instruction , self.string_buffer.as_str() 
                                    )
                                )
                            }
                            LexerState::LABEL => {
                                tokens.push(
                                    Token::new(
                                        TokenType::Label , self.string_buffer.as_str()
                                    )
                                )
                            } 
                            LexerState::DECIMAL_VALUE => {
                                tokens.push(
                                    Token::new(
                                        TokenType::DecimalValue , self.string_buffer.as_str()
                                    )
                                );
                            } 
                            LexerState::HEX_VALUE => {
                                tokens.push(
                                    Token::new(
                                        TokenType::HexValue , self.string_buffer.as_str()
                                    )
                                );
                            }
                            _ => {
                                      
                            }
                        }
                        self.string_buffer = "".to_string();
                        self.internal_state = LexerState::NO_STATE;
                    }
                    'a'..='z' | 'A'..='Z'|'_' => {
                        match self.internal_state {
                            LexerState::NO_STATE => {
                                self.string_buffer.push(self.current_character);
                                self.internal_state = LexerState::LITERAL;
                            }
                            LexerState::LITERAL => {
                                self.string_buffer.push(self.current_character);
                                match self.string_buffer.as_str() {
                                    "push" | "pop" | "add" | "sub" | "halt" => {
                                        if self.current_position == line.len() - 1 {
                                            tokens.push(
                                                Token::new(
                                                    TokenType::Instruction,self.string_buffer.as_str()
                                                )
                                            );
                                        } else {
                                            self.internal_state = LexerState::INSTRUCTION_MNEMONIC;
                                        }
                                    } 
                                    _ => {
                                        if self.current_position == line.len() - 1 {
                                            tokens.push(
                                                Token::new(
                                                    TokenType::Literal, self.string_buffer.as_str()
                                                )
                                            );
                                        }     
                                    }
                                }
                            }
                            LexerState::HEX_VALUE => {
                                match self.current_character {
                                    'A'..='F' | 'a'..='f' => {
                                        self.string_buffer.push(self.current_character);
                                        if self.current_position == line.len() - 1 {
                                            tokens.push(
                                                Token::new(
                                                    TokenType::HexValue,self.string_buffer.as_str()
                                                )
                                            );
                                        }
                                    }
                                    _ => {}
                                }
                            }
                            LexerState::INSTRUCTION_MNEMONIC => {
                               self.string_buffer.push(self.current_character);
                               match self.string_buffer.as_str() {
                                    "push" | "pop" | "add" | "sub" | "halt" => {
                                        // do nothing !    
                                    } 
                                    _ => {
                                        self.internal_state = LexerState::LITERAL;
                                        if self.current_position == line.len() - 1 {
                                            tokens.push(
                                                Token::new(
                                                    TokenType::Label , self.string_buffer.as_str()
                                                )
                                            );
                                        }
                                    }
                               }
                            }
                            _ => {
                                
                            }
                        }
                    }
                    '0'..='9' => {
                        match self.internal_state {
                            LexerState::NO_STATE => {
                                self.string_buffer.push(self.current_character);
                                self.internal_state = LexerState::DECIMAL_VALUE;
                                if self.current_position == line.len() - 1 {
                                    tokens.push(
                                        Token::new(
                                            TokenType::DecimalValue,self.string_buffer.as_str()
                                        )
                                    );
                                } 
                            }
                            LexerState::HEX_VALUE => {
                                self.string_buffer.push(self.current_character);
                                if self.current_position == line.len() - 1 {
                                    tokens.push(
                                        Token::new(
                                            TokenType::HexValue , self.string_buffer.as_str()
                                        )
                                    );
                                }
                            }
                            _ => {}
                        }
                    }
                    ':' => {
                        match self.internal_state {
                            LexerState::LITERAL => {
                                self.internal_state = LexerState::LABEL;
                                self.string_buffer.push(self.current_character);
                                if self.current_position == line.len() - 1 {
                                    tokens.push(
                                        Token::new(
                                            TokenType::Label , self.string_buffer.as_str()
                                        )
                                    );
                                }
                            }
                            _ => {}
                        }
                    }
                    '#' => {
                        self.string_buffer.push(self.current_character);
                        match self.internal_state { 
                            LexerState::NO_STATE => {
                                self.internal_state = LexerState::HEX_VALUE;
                            } 
                            _ => {
                            }
                        } 
                    }
                    _=> {
                        /* unrecognized character or commend or something  ? */
                    }
                }
                self.current_position += 1;
            }
            self.string_buffer = "".to_string();
            self.current_position = 0;
            self.internal_state = LexerState::NO_STATE;
        }
        
        tokens
    }
}

