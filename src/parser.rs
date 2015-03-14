use token::Token;
use lexer::Lexer;

use ast::Binop;
use ast::Expr;

#[derive(Debug, PartialEq)]
pub enum ParseError {
    UnexpectedEOF,
    UnexpectedToken(Token)
}

pub struct Parser<I> {
    lexer: Lexer<I>
}

impl<I> Parser<I> where I: Iterator<Item=char> {
    pub fn new(lexer: Lexer<I>) -> Parser<I> {
        Parser { lexer: lexer }
    }
}

impl<I> Parser<I> where I: Iterator<Item=char> {
    pub fn expr(&mut self) -> Result<Expr, ParseError> {
        let left = match self.lexer.next() {
            Some(Token::DecimalInt(_)) => Expr::Number(1.0),
            Some(t) => return Err(ParseError::UnexpectedToken(t)),
            None => return Err(ParseError::UnexpectedEOF)
        };

        let op = match self.lexer.next() {
            Some(Token::Plus) => Binop::Plus,
            Some(t) => return Err(ParseError::UnexpectedToken(t)),
            None => return Err(ParseError::UnexpectedEOF)
        };

        let right = match self.lexer.next() {
            Some(Token::DecimalInt(_)) => Expr::Number(1.0),
            Some(t) => return Err(ParseError::UnexpectedToken(t)),
            None => return Err(ParseError::UnexpectedEOF)
        };

        Ok(Expr::Binop(op, Box::new(left), Box::new(right)))
    }
}