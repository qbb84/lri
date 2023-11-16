use crate::tokenizer::token::{Token, TokenKind};


struct Lexer {
    input: Vec<char>,
    position: usize,
    read_position: usize,
    ch: char,
}

impl Lexer {
    pub fn new(input: &str) -> Lexer {
        let mut lexer = Lexer {
            input: input.chars().collect(),
            position: 0,
            read_position: 0,
            ch: Default::default(),
        };
        lexer.read_char();
        return lexer;
    }

    fn read_char(&mut self) {
        if self.read_position >= self.input.len() {
            self.ch = '\0';
        } else {
            self.ch = self.input[self.read_position];
        }
        self.position = self.read_position;
        self.read_position += 1;


    }

    fn nex_token(&mut self) -> Token {
        let token = match self.ch {
            '=' =>
                Lexer::new_token(TokenKind::ASSIGN, self.ch),
            ';' =>
                Lexer::new_token(TokenKind::SEMICOLON, self.ch),
            '(' =>
                Lexer::new_token(TokenKind::LPAREM, self.ch),
            ')' =>
                Lexer::new_token(TokenKind::RPAREM, self.ch),
            ',' =>
                Lexer::new_token(TokenKind::COMMA, self.ch),
            '+' =>
                Lexer::new_token(TokenKind::PLUS, self.ch),
            '{' =>
                Lexer::new_token(TokenKind::LBRACE, self.ch),
            '}' =>
                Lexer::new_token(TokenKind::RBRACE, self.ch),
            '\0' =>
                Token { kind: TokenKind::EOF, literal: "".to_string() },
           _ => Lexer::new_token(TokenKind::ILLEGAL, self.ch),

        };
        self.read_char();
        return token;
    }

    fn new_token(kind: TokenKind, ch: char) -> Token {
        return Token { kind, literal: ch.to_string() };
    }
}

#[cfg(test)]
mod test {
    use crate::lexer::lexer::Lexer;
    use crate::tokenizer::token::{Token, TokenKind};

    #[test]
    fn test_next_token() {
       let input = "=+(){},;";

       let expected_tests: Vec<Token> = vec![
           Token {
               kind: TokenKind::ASSIGN,
               literal: "=".to_string(),
           },
           Token {
               kind: TokenKind::PLUS,
               literal: "+".to_string(),
           },
           Token {
               kind: TokenKind::LPAREM,
               literal: "(".to_string(),
           },
           Token {
               kind: TokenKind::RPAREM,
               literal: ")".to_string(),
           },
           Token {
               kind: TokenKind::LBRACE,
               literal: "{".to_string(),
           },
           Token {
               kind: TokenKind::RBRACE,
               literal: "}".to_string(),
           },
           Token {
               kind: TokenKind::COMMA,
               literal: ",".to_string(),
           },
            Token {
               kind: TokenKind::SEMICOLON,
               literal: ";".to_string(),
           },
           Token {
               kind: TokenKind::EOF,
               literal: "".to_string(),
           },

       ];

        let mut lexer = Lexer::new(input);

        for (id, exp_token) in expected_tests.into_iter().enumerate() {
            let recv_token = lexer.nex_token();

            assert_eq!(exp_token.kind, recv_token.kind, "tests[{id}] - Token type wrong. Expected={}, Got={}",
                       exp_token.kind, recv_token.kind);

            assert_eq!(exp_token.literal, recv_token.literal, "tests[{id}] - Literal wrong. Expected={}, Got={}",
                       exp_token.literal, recv_token.literal);
        }


   }

}
