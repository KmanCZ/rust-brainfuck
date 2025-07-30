use std::{fs::File, io::{BufReader, Read, Result as IoResult, ErrorKind}};

#[derive(Debug)]
pub struct Position {
    line: usize,
    character: usize,
}

#[derive(Debug)]
pub enum Token {
    MoveLeft(Position),
    MoveRight(Position),
    Increment(Position),
    Decrement(Position),
    Output(Position),
    Input(Position),
    StartLoop(Position),
    EndLoop(Position),
}

pub struct Lexer {
    reader: BufReader<File>,
    tokens: Vec<Token>,
    line: usize,
    character: usize,
    last_char: [u8; 1],
}

impl Lexer {
    pub fn new(reader: BufReader<File>) -> Lexer {
        Lexer {
            reader,
            tokens: Vec::new(),
            line: 0,
            character: 0,
            last_char: [0],
        }
    }
    
    pub fn lex(&mut self) -> IoResult<()> {
        loop {
            match self.get_next_token() {
                Err(err) => return Err(err),  
                Ok(None) => break,
                Ok(_) => continue,
            }
        }
        
        Ok(())
    }
    
    pub fn tokens(&self) -> &Vec<Token> {
        &self.tokens
    }

    fn handle_char(&mut self) -> Option<Token> {
        self.character += 1;

        match self.last_char[0] as char {
            '\n' => {
                self.line += 1;
                self.character = 0;
                None
            },
            '>' => Some(Token::MoveRight(Position{ line: self.line, character: self.character })),
            '<' => Some(Token::MoveLeft(Position{ line: self.line, character: self.character })),
            '+' => Some(Token::Increment(Position{ line: self.line, character: self.character })),
            '-' => Some(Token::Decrement(Position{ line: self.line, character: self.character })),
            '.' => Some(Token::Output(Position{ line: self.line, character: self.character })),
            ',' => Some(Token::Input(Position{ line: self.line, character: self.character })),
            '[' => Some(Token::StartLoop(Position{ line: self.line, character: self.character })),
            ']' => Some(Token::EndLoop(Position{ line: self.line, character: self.character })),
            _ => None,
        }
    }

    fn get_next_token(&mut self) -> IoResult<Option<()>> {
        loop {
            match self.reader.read(&mut self.last_char) {
                Ok(0) => {
                    return Ok(None);
                }
                Ok(_) => {
                    match self.handle_char() {
                        None => continue,
                        Some(token) => {
                            self.tokens.push(token);
                            return Ok(Some(()))
                        },
                    }
                }
                Err(ref e) if e.kind() == ErrorKind::Interrupted => {
                    continue;
                }
                Err(e) => return Err(e),
            }
        }
    }
}