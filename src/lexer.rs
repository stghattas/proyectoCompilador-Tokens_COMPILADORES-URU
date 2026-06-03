#[derive(Debug, Clone, PartialEq)]
pub enum TokenType {
    PalabraReservada,
    Identificador,
    Numero,
    Operador,
    Punctuation,
    EOF,
}

#[derive(Debug, Clone)]
pub struct Token {
    pub token_type: TokenType,
    pub value: String,
    pub line: usize,
    pub column: usize,
    pub indent_level: usize,
}

pub struct Lexer {
    input: Vec<char>,
    position: usize,
    current_char: Option<char>,
    line: usize,
    column: usize,
    current_indent: usize,
}

impl Lexer {
    pub fn new(input: &str) -> Self {
        let chars: Vec<char> = input.chars().collect();
        let first_char = chars.first().copied();
        
        Lexer {
            input: chars,
            position: 0,
            current_char: first_char,
            line: 1,
            column: 1,
            current_indent: 0,
        }
    }

    fn advance(&mut self) {
        self.position += 1;
        if self.position >= self.input.len() {
            self.current_char = None;
        } else {
            self.current_char = Some(self.input[self.position]);
        }
        self.column += 1;
    }

    fn handle_indentation(&mut self) {
        let mut spaces = 0;
        while let Some(c) = self.current_char {
            if c == ' ' {
                spaces += 1;
                self.advance();
            } else if c == '\t' {
                spaces += 4;
                self.advance();
            } else {
                break;
            }
        }
        self.current_indent = spaces / 4;
    }

    pub fn tokenize(&mut self) -> Vec<Token> {
        let mut tokens = Vec::new();

        // Detectar indentación de la primera línea si la hay
        if self.column == 1 {
            self.handle_indentation();
        }

        while let Some(c) = self.current_char {
            if c == '\n' {
                self.line += 1;
                self.column = 0;
                self.advance();
                self.handle_indentation();
                continue;
            }

            if c.is_whitespace() {
                self.advance();
                continue;
            }

            let start_column = self.column;

            // Identificadores y Palabras Clave
            if c.is_alphabetic() || c == '_' {
                let mut value = String::new();
                while let Some(ch) = self.current_char {
                    if ch.is_alphanumeric() || ch == '_' {
                        value.push(ch);
                        self.advance();
                    } else {
                        break;
                    }
                }

                let token_type = match value.as_str() {
                    "if" | "else" | "while" | "def" | "return" => TokenType::PalabraReservada,
                    _ => TokenType::Identificador,
                };

                tokens.push(self.create_token(token_type, value, start_column));
                continue;
            }

            // Puntuacion
            if c == '(' || c == ')' || c == ':' {
                let value = c.to_string();
                tokens.push(self.create_token(TokenType::Punctuation, value, start_column));
                self.advance();
                continue;
            }

            // Operadores simples (ej. ==)
            if c == '=' {
                self.advance();
                if let Some('=') = self.current_char {
                    tokens.push(self.create_token(TokenType::Operador, "==".to_string(), start_column));
                    self.advance();
                } else {
                    tokens.push(self.create_token(TokenType::Operador, "=".to_string(), start_column));
                }
                continue;
            }

            // Numeros simples
            if c.is_numeric() {
                let mut value = String::new();
                while let Some(ch) = self.current_char {
                    if ch.is_numeric() {
                        value.push(ch);
                        self.advance();
                    } else {
                        break;
                    }
                }
                tokens.push(self.create_token(TokenType::Numero, value, start_column));
                continue;
            }

            // Si no reconoce el carácter, avanza para no hacer un bucle infinito
            self.advance();
        }

        tokens.push(self.create_token(TokenType::EOF, String::from(""), self.column));
        tokens
    }

    fn create_token(&self, token_type: TokenType, value: String, start_column: usize) -> Token {
        Token {
            token_type,
            value,
            line: self.line,
            column: start_column,
            indent_level: self.current_indent,
        }
    }
}