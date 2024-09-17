use crate::*;

#[derive(Debug)]
enum Token {
    LParen, RParen,
    Equals,
    Comma,
    Var(Symbol),
    Fun(Symbol),
}

fn tokenize(s: &str) -> Option<Vec<Token>> {
    let mut tokens = Vec::new();
    let mut current: Option<String> = None;
    for c in s.chars() {
        if c.is_alphanumeric() {
            if let Some(s) = &mut current {
                s.push(c);
            } else {
                current = Some(c.to_string());
            }
            continue;
        }

        if let Some(s) = current.take() {
            let ch = s.chars().next()?;
            let is_var = ('A'..='Z').contains(&ch);

            let s = gsymb_add(s);
            let tok = if is_var { Token::Var(s) } else { Token::Fun(s) };
            tokens.push(tok);
        }

        if c == '=' {
            tokens.push(Token::Equals);
        } else if c == '(' {
            tokens.push(Token::LParen);
        } else if c == ')' {
            tokens.push(Token::RParen);
        } else if c == ',' {
            tokens.push(Token::Comma);
        } else if !c.is_whitespace() {
            return None;
        }
    }

    if let Some(s) = current.take() {
        let ch = s.chars().next()?;
        let is_var = ('A'..='Z').contains(&ch);

        let s = gsymb_add(s);
        let tok = if is_var { Token::Var(s) } else { Token::Fun(s) };
        tokens.push(tok);
    }

    Some(tokens)
}

pub trait Parse: Sized {
    fn assemble(tokens: &[Token]) -> Option<(&[Token], Self)>;

    fn parse(s: &str) -> Option<Self> {
        let tokens = tokenize(s)?;
        let (tokens, out) = Self::assemble(&tokens[..])?;
        if tokens.len() > 0 { return None; }
        Some(out)
    }
}

impl Parse for Equation {
    fn assemble(tokens: &[Token]) -> Option<(&[Token], Self)> {
        let (tokens, lhs) = Term::assemble(tokens)?;
        let [Token::Equals, tokens@..] = tokens else { return None; };
        let (tokens, rhs) = Term::assemble(tokens)?;
        let eq = Equation { lhs, rhs };
        Some((tokens, eq))
    }
}

impl Parse for Term {
    fn assemble(tokens: &[Token]) -> Option<(&[Token], Self)> {
        let [tok, tokens@..] = tokens else { return None; };
        if let Token::Var(s) = *tok {
            return Some((tokens, Term::Var(s)));
        }

        let Token::Fun(f) = *tok else { return None; };
        let [Token::LParen, tokens@..] = tokens else {
            let term = Term::Fun(f, Box::new([]));
            return Some((tokens, term));
        };

        let mut tokens = tokens;
        let mut children = Vec::new();
        loop {
            let (tokens2, t) = Term::assemble(tokens)?;
            children.push(t);
            tokens = tokens2;
            let [Token::Comma, tokens2@..] = tokens else { break; };
            tokens = tokens2;
        }
        let [Token::RParen, tokens@..] = tokens else { return None; };
        let term = Term::Fun(f, children.into_boxed_slice());
        Some((tokens, term))
    }
}
