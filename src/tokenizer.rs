use std::iter::Peekable;
use std::str::Chars;

/**
 * thegrep - Tar Heel Extended Regular Expressions
 *
 * Author: Lily Lou, Taylor Montgomery
 * ONYEN: loulh, tayjomo
 *
 * UNC Honor Pledge: I pledge I have received no unauthorized aid
 * on this assignment. I further pledge not to distribute my solution
 * to this code to anyone other than the course staff.
 */

/**
 * The tokens types of `thegrep` are defined below.
 */
#[derive(Debug, PartialEq)]
pub enum Token {
    LParen,
    RParen,
    UnionBar,
    KleeneStar,
    AnyChar,
    Char(char),
    KleenePlus,
}

/**
 * The internal state of a Tokenizer is maintained by a peekable character
 * iterator over a &str's Chars.
 */
pub struct Tokenizer<'str> {
    chars: Peekable<Chars<'str>>,
}

impl<'str> Tokenizer<'str> {
    pub fn new(input: &'str str) -> Tokenizer {
        Tokenizer {
            chars: input.chars().peekable(),
        }
    }
}

/**
 * The Iterator trait is implemented for Tokenizer. It will produce items of
 * type Token and has a `next` method that returns Option<Token>.
 */
impl<'str> Iterator for Tokenizer<'str> {
    type Item = Token;

    /**
     * The `next` method ignores leading whitespace and returns the next
     * complete Some(Token) in the Tokenizer's input string or None at all.
     */
    fn next(&mut self) -> Option<Token> {
        self.lex_endline();
        if let Some(c) = self.chars.peek() {
            Some(match c {
                '(' => self.lex_lparen(),
                ')' => self.lex_rparen(),
                '|' => self.lex_unionbar(),
                '*' => self.lex_kleenestar(),
                '.' => self.lex_anychar(),
                '+' => self.lex_kleeneplus(),
                _ => self.lex_char(),
            })
        } else {
            None
        }
    }
}
/**
 * Unit Tests for the 'next' method.
 */
#[cfg(test)]
mod iterator {
    use super::*;

    #[test]
    fn empty() {
        let mut tokens = Tokenizer::new("");
        assert_eq!(tokens.next(), None);
        assert_eq!(tokens.next(), None);
    }

    #[test]
    fn lparen() {
        let mut tokens = Tokenizer::new("(");
        assert_eq!(tokens.next(), Some(Token::LParen));
        assert_eq!(tokens.next(), None);
    }

    #[test]
    fn rparen() {
        let mut tokens = Tokenizer::new(")");
        assert_eq!(tokens.next(), Some(Token::RParen));
        assert_eq!(tokens.next(), None);
    }

    #[test]
    fn unionbar() {
        let mut tokens = Tokenizer::new("|");
        assert_eq!(tokens.next(), Some(Token::UnionBar));
        assert_eq!(tokens.next(), None);
    }

    #[test]
    fn kleenestar() {
        let mut tokens = Tokenizer::new("*");
        assert_eq!(tokens.next(), Some(Token::KleeneStar));
        assert_eq!(tokens.next(), None);
    }

    #[test]
    fn anychar() {
        let mut tokens = Tokenizer::new(".");
        assert_eq!(tokens.next(), Some(Token::AnyChar));
        assert_eq!(tokens.next(), None);
    }

    #[test]
    fn char() {
        let mut tokens = Tokenizer::new("m");
        assert_eq!(tokens.next(), Some(Token::Char('m')));
        assert_eq!(tokens.next(), None);
    }

    #[test]
    fn kleeneplus() {
        let mut tokens = Tokenizer::new("+");
        assert_eq!(tokens.next(), Some(Token::KleenePlus));
        assert_eq!(tokens.next(), None);
    }
    #[test]
    fn lex_phrase() {
        let mut tokens = Tokenizer::new("(2.\n*a)\n|b+");
        assert_eq!(tokens.next(), Some(Token::LParen));
        assert_eq!(tokens.next(), Some(Token::Char('2')));
        assert_eq!(tokens.next(), Some(Token::AnyChar));
        assert_eq!(tokens.next(), Some(Token::KleeneStar));
        assert_eq!(tokens.next(), Some(Token::Char('a')));
        assert_eq!(tokens.next(), Some(Token::RParen));
        assert_eq!(tokens.next(), Some(Token::UnionBar));
        assert_eq!(tokens.next(), Some(Token::Char('b')));
        assert_eq!(tokens.next(), Some(Token::KleenePlus));
        assert_eq!(tokens.next(), None);
    }
}

/*
 * Helper methods of Tokenizer are follow. None are defined as pub
 * so these are internal methods only.
 */
impl<'str> Tokenizer<'str> {
    fn lex_endline(&mut self) {
        while let Some(c) = self.chars.peek() {
            match c {
                '\t' | '\n' => self.chars.next(),
                _ => break,
            };
        }
    }
    fn lex_lparen(&mut self) -> Token {
        let c = self.chars.next().unwrap();
        match c {
            '(' => Token::LParen,
            _ => panic!("Unexpected assignment helper"),
        }
    }

    fn lex_rparen(&mut self) -> Token {
        let c = self.chars.next().unwrap();
        match c {
            ')' => Token::RParen,
            _ => panic!("Unexpected assignment helper"),
        }
    }

    fn lex_unionbar(&mut self) -> Token {
        let c = self.chars.next().unwrap();
        match c {
            '|' => Token::UnionBar,
            _ => panic!("Unexpected assignment helper"),
        }
    }

    fn lex_kleenestar(&mut self) -> Token {
        let c = self.chars.next().unwrap();
        match c {
            '*' => Token::KleeneStar,
            _ => panic!("Unexpected assignment helper"),
        }
    }

    fn lex_anychar(&mut self) -> Token {
        let c = self.chars.next().unwrap();
        match c {
            '.' => Token::AnyChar,
            _ => panic!("Unexpected assignment helper"),
        }
    }

    fn lex_char(&mut self) -> Token {
        let c = self.chars.next().unwrap();
        Token::Char(c)
    }

    fn lex_kleeneplus(&mut self) -> Token {
        let c = self.chars.next().unwrap();
        match c {
            '+' => Token::KleenePlus,
            _ => panic!("Unexpected assignment helper"),
        }
    }
}

#[cfg(test)]
mod private_api {
    use super::*;

    mod tokenizer {
        use super::*;

        #[test]
        fn lparen() {
            let mut token = Tokenizer::new("(");
            assert_eq!(token.lex_lparen(), Token::LParen);
        }

        #[test]
        fn rparen() {
            let mut token = Tokenizer::new(")");
            assert_eq!(token.lex_rparen(), Token::RParen);
        }

        #[test]
        fn anychar() {
            let mut token = Tokenizer::new(".");
            assert_eq!(token.lex_anychar(), Token::AnyChar);
        }

        #[test]
        fn unionbar() {
            let mut token = Tokenizer::new("|");
            assert_eq!(token.lex_unionbar(), Token::UnionBar);
        }

        #[test]
        fn kleenestar() {
            let mut token = Tokenizer::new("*");
            assert_eq!(token.lex_kleenestar(), Token::KleeneStar);
        }

        #[test]
        fn char() {
            let mut token = Tokenizer::new("a");
            assert_eq!(token.lex_char(), Token::Char('a'));
        }

        #[test]
        fn kleeneplus() {
            let mut token = Tokenizer::new("+");
            assert_eq!(token.lex_kleeneplus(), Token::KleenePlus);
        }
    }

}
