use crate::tokenizer::token::{lookup_ident, Token, TokenKind};

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

    fn next_token(&mut self) -> Token {
        self.skip_whitespace();
        let token = match self.ch {
            '=' => Lexer::new_token(TokenKind::ASSIGN, self.ch),
            ';' => Lexer::new_token(TokenKind::SEMICOLON, self.ch),
            '(' => Lexer::new_token(TokenKind::LPAREM, self.ch),
            ')' => Lexer::new_token(TokenKind::RPAREM, self.ch),
            ',' => Lexer::new_token(TokenKind::COMMA, self.ch),
            '+' => Lexer::new_token(TokenKind::PLUS, self.ch),
            '{' => Lexer::new_token(TokenKind::LBRACE, self.ch),
            '}' => Lexer::new_token(TokenKind::RBRACE, self.ch),
            '\0' => Token {
                kind: TokenKind::EOF,
                literal: "".to_string(),
            },
            '-' => Lexer::new_token(TokenKind::MINUS, self.ch),
            '!' => Lexer::new_token(TokenKind::BANG, self.ch),
            '*' => Lexer::new_token(TokenKind::ASTERISK, self.ch),
            '/' => Lexer::new_token(TokenKind::SLASH, self.ch),
            '>' => Lexer::new_token(TokenKind::GT, self.ch),
            '<' => Lexer::new_token(TokenKind::LT, self.ch),
            _ => {
                return if Lexer::is_letter(self.ch) {
                    let literal = self.read_identifier();
                    let kind = lookup_ident(&literal);
                    Token { kind, literal }
                } else if Lexer::is_digit(self.ch) {
                    let kind = TokenKind::INT;
                    let literal = self.read_number();
                    Token { kind, literal }
                } else {
                    Lexer::new_token(TokenKind::ILLEGAL, self.ch)
                }
            }
        };
        self.read_char();
        return token;
    }

    fn skip_whitespace(&mut self) {
        while self.ch.is_ascii_whitespace() {
            self.read_char();
        }
    }

    fn is_letter(ch: char) -> bool {
        return ch.is_alphabetic() || ch == '_';
    }

    fn read_identifier(&mut self) -> String {
        let mut identifier = String::new();

        while Lexer::is_letter(self.ch) {
            identifier.push(self.ch);
            self.read_char();
        }

        return identifier;
    }

    fn is_digit(ch: char) -> bool {
        return ch.is_numeric();
    }

    fn read_number(&mut self) -> String {
        let mut num = String::from("");

        while Lexer::is_digit(self.ch) {
            num.push(self.ch);
            self.read_char();
        }
        return num;
    }

    fn new_token(kind: TokenKind, ch: char) -> Token {
        return Token {
            kind,
            literal: ch.to_string(),
        };
    }
}

#[cfg(test)]
mod test {
    use crate::lexer::lexer::Lexer;
    use crate::tokenizer::token::{Token, TokenKind};

    #[test]
    fn test_next_token() {
        let input = r#"
        let five = 5;
        let ten = 10;

        let add = fn(x, y){
            x + y;
        };

        let result = add(five, ten);
        !-/*5;
        5 < 10 > 5;

       "#;

        let expected_tests: Vec<Token> = vec![
            Token {
                kind: TokenKind::LET,
                literal: "let".to_string(),
            },
            Token {
                kind: TokenKind::IDENT,
                literal: "five".to_string(),
            },
            Token {
                kind: TokenKind::ASSIGN,
                literal: "=".to_string(),
            },
            Token {
                kind: TokenKind::INT,
                literal: "5".to_string(),
            },
            Token {
                kind: TokenKind::SEMICOLON,
                literal: ";".to_string(),
            },
            Token {
                kind: TokenKind::LET,
                literal: "let".to_string(),
            },
            Token {
                kind: TokenKind::IDENT,
                literal: "ten".to_string(),
            },
            Token {
                kind: TokenKind::ASSIGN,
                literal: "=".to_string(),
            },
            Token {
                kind: TokenKind::INT,
                literal: "10".to_string(),
            },
            Token {
                kind: TokenKind::SEMICOLON,
                literal: ";".to_string(),
            },
            Token {
                kind: TokenKind::LET,
                literal: "let".to_string(),
            },
            Token {
                kind: TokenKind::IDENT,
                literal: "add".to_string(),
            },
            Token {
                kind: TokenKind::ASSIGN,
                literal: "=".to_string(),
            },
            Token {
                kind: TokenKind::FUNCTION,
                literal: "fn".to_string(),
            },
            Token {
                kind: TokenKind::LPAREM,
                literal: "(".to_string(),
            },
            Token {
                kind: TokenKind::IDENT,
                literal: "x".to_string(),
            },
            Token {
                kind: TokenKind::COMMA,
                literal: ",".to_string(),
            },
            Token {
                kind: TokenKind::IDENT,
                literal: "y".to_string(),
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
                kind: TokenKind::IDENT,
                literal: "x".to_string(),
            },
            Token {
                kind: TokenKind::PLUS,
                literal: "+".to_string(),
            },
            Token {
                kind: TokenKind::IDENT,
                literal: "y".to_string(),
            },
            Token {
                kind: TokenKind::SEMICOLON,
                literal: ";".to_string(),
            },
            Token {
                kind: TokenKind::RBRACE,
                literal: "}".to_string(),
            },
            Token {
                kind: TokenKind::SEMICOLON,
                literal: ";".to_string(),
            },
            Token {
                kind: TokenKind::LET,
                literal: "let".to_string(),
            },
            Token {
                kind: TokenKind::IDENT,
                literal: "result".to_string(),
            },
            Token {
                kind: TokenKind::ASSIGN,
                literal: "=".to_string(),
            },
            Token {
                kind: TokenKind::IDENT,
                literal: "add".to_string(),
            },
            Token {
                kind: TokenKind::LPAREM,
                literal: "(".to_string(),
            },
            Token {
                kind: TokenKind::IDENT,
                literal: "five".to_string(),
            },
            Token {
                kind: TokenKind::COMMA,
                literal: ",".to_string(),
            },
            Token {
                kind: TokenKind::IDENT,
                literal: "ten".to_string(),
            },
            Token {
                kind: TokenKind::RPAREM,
                literal: ")".to_string(),
            },
            Token {
                kind: TokenKind::SEMICOLON,
                literal: ";".to_string(),
            },
            Token {
                kind: TokenKind::BANG,
                literal: "!".to_string(),
            },
            Token {
                kind: TokenKind::MINUS,
                literal: "-".to_string(),
            },
            Token {
                kind: TokenKind::SLASH,
                literal: "/".to_string(),
            },
            Token {
                kind: TokenKind::ASTERISK,
                literal: "*".to_string(),
            },
            Token {
                kind: TokenKind::INT,
                literal: "5".to_string(),
            },
            Token {
                kind: TokenKind::SEMICOLON,
                literal: ";".to_string(),
            },
            Token {
                kind: TokenKind::INT,
                literal: "5".to_string(),
            },
            Token {
                kind: TokenKind::LT,
                literal: "<".to_string(),
            },
            Token {
                kind: TokenKind::INT,
                literal: "10".to_string(),
            },
            Token {
                kind: TokenKind::GT,
                literal: ">".to_string(),
            },
            Token {
                kind: TokenKind::INT,
                literal: "5".to_string(),
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
            let recv_token = lexer.next_token();

            assert_eq!(
                exp_token.kind, recv_token.kind,
                "tests[{id}] - Token type wrong. Expected={}, Got={}",
                exp_token.kind, recv_token.kind
            );

            assert_eq!(
                exp_token.literal, recv_token.literal,
                "tests[{id}] - Literal wrong. Expected={}, Got={}",
                exp_token.literal, recv_token.literal
            );
        }
    }
}
