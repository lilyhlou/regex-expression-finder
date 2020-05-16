use super::tokenizer::{Token, Tokenizer};
use std::iter::Peekable;

/**
 * thegrep - Tar Heel Extended Regular Expressions - Parser
 *
 * Author: <Taylor Montgomery, Lily Lou>
 * ONYEN: <tayjomo, loulh>
 *
 * UNC Honor Pledge: I pledge I have received no unauthorized aid
 * on this assignment. I further pledge not to distribute my solution
 * to this code to anyone other than the course staff.
 */

/* == Begin Syntax Tree Elements == */
#[derive(Debug, PartialEq)]
pub enum AST {
    Alternation(Box<AST>, Box<AST>),
    Catenation(Box<AST>, Box<AST>),
    Closure(Box<AST>),
    OneOrMore(Box<AST>),
    Char(char),
    AnyChar,
}

/* Helper factory functions for building Exprs */
pub fn create_alternation(lhs: AST, rhs: AST) -> AST {
    AST::Alternation(Box::new(lhs), Box::new(rhs))
}

pub fn create_catenation(lhs: AST, rhs: AST) -> AST {
    AST::Catenation(Box::new(lhs), Box::new(rhs))
}

pub fn create_closure(expression: AST) -> AST {
    AST::Closure(Box::new(expression))
}

pub fn create_one_or_more(expression: AST) -> AST {
    AST::OneOrMore(Box::new(expression))
}

pub fn create_char(value: char) -> AST {
    AST::Char(value)
}

pub fn create_any_char() -> AST {
    AST::AnyChar
}
/* == End Syntax Tree Elements == */

pub struct Parser<'tokens> {
    tokens: Peekable<Tokenizer<'tokens>>,
}

impl<'tokens> Parser<'tokens> {
    pub fn parse(tokenizer: Tokenizer<'tokens>) -> Result<AST, String> {
        let mut parser = Parser {
            tokens: tokenizer.peekable(),
        };
        let p = parser.reg_expr();
        if parser.tokens.peek() != None {
            Err(format!(
                "Expected end of input, found {:?}",
                parser.tokens.next().unwrap()
            ))
        } else {
            Ok(p?)
        }
    }
}

#[cfg(test)]
mod public_api {
    use super::*;

    mod createAST {
        use super::*;
        #[test]
        fn parse_char() {
            let parsed = Parser::parse(Tokenizer::new("a")).unwrap();
            assert_eq!(parsed, create_char('a'));
        }

        #[test]
        fn parse_catenation() {
            let parsed = Parser::parse(Tokenizer::new("ab")).unwrap();
            assert_eq!(
                parsed,
                create_catenation(create_char('a'), create_char('b'))
            );
        }

        #[test]
        fn parse_closure() {
            let parsed = Parser::parse(Tokenizer::new("a.*")).unwrap();
            assert_eq!(
                parsed,
                create_catenation(create_char('a'), create_closure(create_any_char()))
            );
        }

        #[test]
        fn parse_alternation() {
            let parsed = Parser::parse(Tokenizer::new("a|b")).unwrap();
            assert_eq!(
                parsed,
                create_alternation(create_char('a'), create_char('b'))
            );
        }

        #[test]
        fn parse_any_char() {
            let parsed = Parser::parse(Tokenizer::new(".")).unwrap();
            assert_eq!(parsed, create_any_char());
        }

        #[test]
        fn parse_oneOrMore() {
            let parsed = Parser::parse(Tokenizer::new("a+")).unwrap();
            assert_eq!(parsed, create_one_or_more(create_char('a')));
        }

        #[test]
        fn parse_all() {
            let parsed = Parser::parse(Tokenizer::new("ab|c+d*(e.)")).unwrap();
            assert_eq!(
                parsed,
                create_alternation(
                    create_catenation(create_char('a'), create_char('b')),
                    create_catenation(
                        create_one_or_more(create_char('c')),
                        create_catenation(
                            create_closure(create_char('d')),
                            create_catenation(create_char('e'), create_any_char())
                        )
                    )
                )
            );
        }
    }
}

/**
 * Internal-only parser methods to process the grammar via recursive descent.
 */
impl<'tokens> Parser<'tokens> {
    // RegExpr     -> Catenation (UnionBar RegExpr)?
    fn reg_expr(&mut self) -> Result<AST, String> {
        let l = self.catenation()?;
        //l stands for left hand side
        if let Some(next_token) = self.tokens.peek() {
            // checks for next token to see if there is a union bar or if it
            // should be returned to atom if it is the closing paren
            match next_token {
                Token::UnionBar => {
                    self.tokens.next();
                    let rhs = self.reg_expr()?;
                    return Ok(create_alternation(l, rhs));
                }
                Token::RParen => return Ok(l),
                _ => Err(format!("Unexpected input")),
            }
        } else {
            return Ok(l);
        }
    }

    // Catenation -> Closure Catenation?
    fn catenation(&mut self) -> Result<AST, String> {
        let c = self.closure()?;
        if let Some(next_token) = self.tokens.peek() {
            match next_token {
                // checks for unionbar to return to reg_expr method and
                // rparen for atom method
                Token::UnionBar => return Ok(c),
                Token::RParen => return Ok(c),
                _ => {
                    let rhs = self.catenation()?;
                    return Ok(create_catenation(c, rhs));
                }
            }
        } else {
            return Ok(c);
        }
    }

    // Closure  -> Atom KleeneStar/KleenePlus?
    fn closure(&mut self) -> Result<AST, String> {
        let expr = self.atom()?;
        if let Some(kleene) = self.peek_token() {
            match kleene {
                '*' => {
                    // if next token is a kleene star, create a closure AST
                    let closure = create_closure(expr);
                    self.take_token()?;
                    return Ok(closure);
                }
                '+' => {
                    // if next token is a kleene star, create a closure AST
                    let one_more = create_one_or_more(expr);
                    self.take_token()?;
                    return Ok(one_more);
                }
                _ => return Ok(expr),
            }
        } else {
            return Ok(expr);
        }
    }

    // Atom     -> LParen RegExpr Rparen | AnyChar | Char

    fn atom(&mut self) -> Result<AST, String> {
        let t: Token = self.take_next_token()?;
        match t {
            Token::AnyChar => return Ok(create_any_char()),
            Token::Char(value) => return Ok(create_char(value)),
            Token::LParen => {
                let expr = self.reg_expr();
                match expr {
                    Err(message) => return Err(message),
                    Ok(e) => {
                        let rparen = self.consume_token(Token::RParen)?;
                        match rparen {
                            Token::RParen => return Ok(e),
                            _ => return Err(format!("Missing right parenthesis")),
                        }
                    }
                }
            }
            _ => Err(format!("Unexpected atom encountered")),
        }
    }
}

#[cfg(test)]
mod private_api {
    use super::*;

    mod parserLexemes {
        use super::*;

        #[test]
        fn reg_expr() {
            assert_eq!(Parser::from("a").reg_expr().unwrap(), create_char('a'));
        }

        #[test]
        fn union_bar_fail() {
            assert_eq!(
                Parser::from("a|").reg_expr(),
                Err(format!("Unexpected end of input"))
            );
        }
        #[test]
        fn closure() {
            assert_eq!(
                Parser::from("a*").closure().unwrap(),
                create_closure(create_char('a'))
            );
        }

        #[test]
        fn one_more() {
            assert_eq!(
                Parser::from("a+").closure().unwrap(),
                create_one_or_more(create_char('a'))
            );
        }
        #[test]
        fn no_closure() {
            assert_eq!(Parser::from("a").closure().unwrap(), create_char('a'));
        }

        #[test]
        fn atom() {
            assert_eq!(
                Parser::from("(ab)").atom().unwrap(),
                create_catenation(create_char('a'), create_char('b'))
            );
            assert_eq!(Parser::from(".").atom().unwrap(), create_any_char());
            assert_eq!(Parser::from("a").atom().unwrap(), create_char('a'));
        }

        #[test]
        fn catenation() {
            assert_eq!(
                Parser::from("a*cb").catenation().unwrap(),
                create_catenation(
                    create_closure(create_char('a')),
                    create_catenation(create_char('c'), create_char('b'))
                )
            )
        }
        #[test]
        fn rparen_fail() {
            assert_eq!(
                Parser::from("(a").atom(),
                Err(format!("Unexpected end of input"))
            );
        }
    }
}

/* Parser's Helper Methods to improve ergonomics of parsing */
impl<'tokens> Parser<'tokens> {
    fn from(input: &'tokens str) -> Parser<'tokens> {
        Parser {
            tokens: Tokenizer::new(input).peekable(),
        }
    }
    fn take_next_token(&mut self) -> Result<Token, String> {
        if let Some(token) = self.tokens.next() {
            Ok(token)
        } else {
            Err(String::from("Unexpected end of input"))
        }
    }

    //peek for kleenstar to determine if closure AST
    fn peek_token(&mut self) -> Option<char> {
        if let Some(Token::KleeneStar) = self.tokens.peek() {
            Some('*')
        } else if let Some(Token::KleenePlus) = self.tokens.peek() {
            Some('+')
        } else {
            None
        }
    }

    //consume the kleenstar token
    fn take_token(&mut self) -> Result<char, String> {
        if let Some(Token::KleeneStar) = self.tokens.peek() {
            self.tokens.next();
            Ok('*')
        } else if let Some(Token::KleenePlus) = self.tokens.peek() {
            self.tokens.next();
            Ok('+')
        } else {
            Err(format!("KleeneStar not found."))
        }
    }

    fn consume_token(&mut self, expected: Token) -> Result<Token, String> {
        if let Some(next) = self.tokens.next() {
            if next != expected {
                Err(format!("Expected: {:?} - Found {:?}", expected, next))
            } else {
                Ok(next)
            }
        } else {
            Err(String::from("Unexpected end of input"))
        }
    }
}
