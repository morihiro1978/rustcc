#[derive(Debug)]
struct Tokenizer<'a> {
    code: &'a str,
    chars: std::iter::Peekable<std::str::Chars<'a>>,
}

impl<'a> Tokenizer<'a> {
    pub fn new(code: &str) -> Tokenizer {
        Tokenizer {
            code: &code,
            chars: code.chars().peekable(),
        }
    }

    pub fn next(&mut self) -> Option<String> {
        let mut tok = String::new();

        loop {
            if let Some(c) = self.chars.next() {
                match c {
                    // >, >=, <, <=, =, ==, !, !=
                    c @ '>' | c @ '<' | c @ '=' | c @ '!' => {
                        tok.push(c);
                        if self.chars.peek() == Some(&'=') {
                            tok.push(self.chars.next().unwrap());
                        }
                        break;
                    }
                    // +, ++
                    c @ '+' => {
                        tok.push(c);
                        if self.chars.peek() == Some(&'+') {
                            tok.push(self.chars.next().unwrap());
                        }
                        break;
                    }
                    // -, --
                    c @ '-' => {
                        tok.push(c);
                        if self.chars.peek() == Some(&'-') {
                            tok.push(self.chars.next().unwrap());
                        }
                        break;
                    }
                    // 1 char sign
                    c @ '*'
                    | c @ '/'
                    | c @ '('
                    | c @ ')'
                    | c @ ';'
                    | c @ '{'
                    | c @ '}'
                    | c @ ',' => {
                        tok.push(c);
                        break;
                    }
                    // char
                    '\'' => {
                        tok.push('\'');
                        if self.chars.peek() != Some(&'\'') {
                            tok.push(self.chars.next().unwrap());
                            if self.chars.peek() != Some(&'\'') {
                                panic!("After ' is wrong.");
                            }
                        }
                        tok.push(self.chars.next().unwrap());
                        break;
                    }
                    // String
                    '"' => {
                        tok.push('"');
                        loop {
                            let c = self.chars.next().unwrap();
                            tok.push(c);
                            if c == '"' {
                                break;
                            }
                        }
                        break;
                    }
                    // num
                    c @ '0'...'9' => {
                        tok.push(c);
                        loop {
                            match self.chars.peek() {
                                Some(_c @ '0'...'9') => tok.push(self.chars.next().unwrap()),
                                _ => break,
                            }
                        }
                        break;
                    }
                    // ident
                    c @ 'a'...'z' | c @ 'A'...'Z' => {
                        tok.push(c);
                        loop {
                            match self.chars.peek() {
                                Some(_c @ '0'...'9') | Some(_c @ 'a'...'z')
                                | Some(_c @ 'A'...'Z') => tok.push(self.chars.next().unwrap()),
                                _ => break,
                            }
                        }
                        break;
                    }
                    c if c.is_whitespace() => {
                        if !tok.is_empty() {
                            break;
                        }
                    }
                    _ => {
                        break;
                    }
                };
            } else {
                break;
            }
        }

        if tok.is_empty() {
            None
        } else {
            Some(tok)
        }
    }
}

fn main() {
    let code: String = std::env::args().nth(1).unwrap();
    let mut tok = Tokenizer::new(&code);

    println!("code: {}", code);
    println!("tok: {:?}", tok);
    while let Some(c) = tok.next() {
        println!("tok.next: {}", c);
    }
}
