#[derive(Debug, Clone, PartialEq)]
pub enum TokenType {
    PalabraReservada(String),
    Identificador(String),
    Integer(i64), // Enteros puros
    Float(f64),   // Numeros de coma flotante
    Boolean(bool),
    String(String),
    Operador(String),
    Puntuacion(char),
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
    indent_stack: Vec<usize>, // Pila para manejar indentacion dinamica
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
            indent_stack: vec![0], // Nivel base (0 espacios)
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

    // --- Metodos ---

    fn handle_newline_and_indentation(&mut self) {
        self.line += 1;
        self.column = 0;
        self.advance(); // Consumir el \n

        let mut spaces = 0;
        while let Some(c) = self.current_char {
            if c == ' ' {
                spaces += 1;
                self.advance();
            } else if c == '\t' {
                spaces += 4; // Puedes ajustarlo, pero sumamos cantidad
                self.advance();
            } else if c == '\n' {
                // reiniciar conteo
                self.line += 1;
                self.column = 0;
                spaces = 0;
                self.advance();
            } else {
                break;
            }
        }

        // Logica de Pila de Identacion
        let current_top = *self.indent_stack.last().unwrap_or(&0);
        if spaces > current_top {
            // Aumento de nivel (Indent)
            self.indent_stack.push(spaces);
        } else if spaces < current_top {
            // Retroceso de nivel (Dedent)
            while let Some(&top) = self.indent_stack.last() {
                if spaces < top {
                    self.indent_stack.pop();
                } else {
                    break;
                }
            }
        }
        self.current_indent = self.indent_stack.len() - 1;
    }

    fn read_string(&mut self, quote_type: char) -> Token {
        let start_column = self.column;
        let mut value = String::new();
        self.advance(); // Consumir comilla de apertura

        while let Some(c) = self.current_char {
            if c == '\\' {
                // Escapar el siguiente caracter
                self.advance();
                if let Some(escaped_char) = self.current_char {
                    value.push(escaped_char);
                    self.advance();
                }
            } else if c == quote_type {
                self.advance(); // Consumir comilla de cierre
                break;
            } else {
                value.push(c);
                self.advance();
            }
        }

        Token {
            token_type: TokenType::String(value.clone()), // Usamos .clone() porque lo necesitamos dos veces
            value,
            line: self.line,
            column: start_column,
            indent_level: self.current_indent,
        }
    }

    fn read_number(&mut self) -> Token {
        let start_column = self.column;
        let mut value = String::new();
        let mut has_dot = false;

        while let Some(c) = self.current_char {
            if c.is_numeric() || c == '.' {
                value.push(c);
                self.advance();
            } else if c == '.' && !has_dot {
                // Si encontramos un punto y no hemos visto uno antes, es un Float
                has_dot = true;
                value.push(c);
                self.advance();
            } else {
                // Si es otro caracter
                break;
            }
        }

        // Aprovecha las capacidades de parseo de Rust
        let token_type = if has_dot {
            // Intentamos parsear a f64. Usamos unwrap_or por seguridad,
            // aunque validamos que solo sean numeros y un punto.
            let parsed_float = value.parse::<f64>().unwrap_or(0.0);
            TokenType::Float(parsed_float)
        } else {
            // Intentamos parsear a i64
            let parsed_int = value.parse::<i64>().unwrap_or(0);
            TokenType::Integer(parsed_int)
        };

        Token {
            token_type,
            value,
            line: self.line,
            column: start_column,
            indent_level: self.current_indent,
        }
    }

    fn read_identifier_or_keyword(&mut self) -> Token {
        let start_column = self.column;
        let mut value = String::new();

        while let Some(c) = self.current_char {
            if c.is_alphanumeric() || c == '_' {
                value.push(c);
                self.advance();
            } else {
                break;
            }
        }

        // Usamos .clone() para los tipos que requieren almacenar un String
        let token_type = match value.as_str() {
            "True" => TokenType::Boolean(true),
            "False" => TokenType::Boolean(false),
            "float" | "if" | "else" | "while" | "def" | "return" => {
                TokenType::PalabraReservada(value.clone())
            }
            _ => TokenType::Identificador(value.clone()),
        };

        Token {
            token_type,
            value,
            line: self.line,
            column: start_column,
            indent_level: self.current_indent,
        }
    }

    fn read_operator(&mut self) -> Token {
        let start_column = self.column;
        let mut op = String::new();

        while let Some(c) = self.current_char {
            if "+-*/=<>!&|".contains(c) {
                op.push(c);
                self.advance();
            } else {
                break;
            }
        }

        Token {
            token_type: TokenType::Operador(op.clone()),
            value: op,
            line: self.line,
            column: start_column,
            indent_level: self.current_indent,
        }
    }

    // --- Bucle Principal del Analizador ---

    pub fn tokenize(&mut self) -> Vec<Token> {
        let mut tokens = Vec::new();

        while let Some(c) = self.current_char {
            match c {
                '\n' => self.handle_newline_and_indentation(),
                ' ' | '\t' => self.advance(), // Ignorar espacios intermedios
                '"' | '\'' => tokens.push(self.read_string(c)),
                c if c.is_alphabetic() || c == '_' => {
                    tokens.push(self.read_identifier_or_keyword())
                }
                c if c.is_numeric() => tokens.push(self.read_number()),
                // Puntuacion
                '(' | ')' | '{' | '}' | '[' | ']' | ':' | ',' | '.' => {
                    tokens.push(Token {
                        token_type: TokenType::Puntuacion(c),
                        value: c.to_string(), // Convertimos el char a String
                        line: self.line,
                        column: self.column,
                        indent_level: self.current_indent,
                    });
                    self.advance();
                }
                // Operadores agrupados
                '+' | '-' | '*' | '/' | '=' | '<' | '>' | '!' => tokens.push(self.read_operator()),
                _ => self.advance(),
            }
        }

        tokens.push(Token {
            token_type: TokenType::EOF,
            value: String::from(""), // Un string vacío para el fin de archivo
            line: self.line,
            column: self.column,
            indent_level: self.current_indent,
        });

        tokens
    }
}
