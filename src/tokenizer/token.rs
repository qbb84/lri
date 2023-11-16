use std::fmt::{Display, Formatter};

#[derive(PartialEq, Debug)]
pub struct Token {
    pub kind: TokenKind,
    pub literal: String,
}

#[derive(PartialEq, Debug)]
pub enum TokenKind {
    ILLEGAL,
    EOF,
    IDENT,
    INT,
    ASSIGN,
    PLUS,
    COMMA,
    SEMICOLON,
    LPAREM,
    RPAREM,
    LBRACE,
    RBRACE,
    FUNCTION,
    LET,

}

impl Display for TokenKind {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            TokenKind::ILLEGAL => write!(f, "ILLEGAL"),
            TokenKind::EOF => write!(f, "EOF"),
            TokenKind::IDENT => write!(f, "IDENT"),
            TokenKind::INT => write!(f, "INT"),
            TokenKind::ASSIGN => write!(f, "="),
            TokenKind::PLUS => write!(f, "+"),
            TokenKind::COMMA => write!(f, ","),
            TokenKind::SEMICOLON => write!(f, ";"),
            TokenKind::LPAREM => write!(f, "("),
            TokenKind::RPAREM => write!(f, ")"),
            TokenKind::LBRACE => write!(f, "{{"),
            TokenKind::RBRACE => write!(f, "}}"),
            TokenKind::FUNCTION => write!(f, "FUNCTION"),
            TokenKind::LET => write!(f, "Let"),
        }
    }
}
