use std::iter::Peekable;
use std::str::CharIndices;

#[derive(Debug)]
enum Token<'a> {
    Integer(u64),
    Literal(char),
    Name(&'a str),
}

struct Lexer<'a> {
    source: &'a str,
    iter: Peekable<CharIndices<'a>>,
}

impl<'a> Lexer<'a> {
    fn new(source: &'a str) -> Self {
        Lexer {
            source,
            iter: source.char_indices().peekable(),
        }
    }
}

impl<'a> Iterator for Lexer<'a> {
    type Item = Token<'a>;

    fn next(&mut self) -> Option<Self::Item> {
        let (start, c) = loop {
            match self.iter.next() {
                Some((start, c)) => {
                    if !c.is_whitespace() {
                        break (start, c);
                    }
                }
                None => return None,
            };
        };

        match c {
            '0'..='9' => {
                let mut val: u64 = c.to_digit(10).unwrap() as u64;
                loop {
                    if let Some((_pos, c)) = self.iter.peek() {
                        if c.is_digit(10) {
                            val *= 10;
                            val += c.to_digit(10).unwrap() as u64;
                            self.iter.next();
                        } else {
                            break;
                        }
                    } else {
                        break;
                    }
                }
                Some(Token::Integer(val))
            }

            'a'..='z' | 'A'..='Z' | '_' => {
                let mut end = start + 1;
                loop {
                    if let Some((_pos, c)) = self.iter.peek() {
                        if c.is_ascii_alphanumeric() || *c == '_' {
                            end += 1;
                            self.iter.next();
                        } else {
                            break;
                        }
                    } else {
                        break;
                    }
                }
                let name = &self.source[start..end];
                Some(Token::Name(name))
            }

            _ => Some(Token::Literal(c)),
        }
    }
}

fn main() {
    let lexer = Lexer::new("+(   )_HELLO1_X_,234+FOO!994");

    for token in lexer {
        println!("{:?}", token);
    }
}
