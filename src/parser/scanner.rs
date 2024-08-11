use std::str::Chars;

use crate::utils::peekable_iterator::NthPeekable;
pub struct Scanner<'a>
{
    iter:   NthPeekable<Chars<'a>, char>,
}

impl <'a> Scanner<'a>
{
    pub fn from_str(str: &'a str, peek_dept: usize) -> Self
    {
        Scanner
        {
            iter: NthPeekable::new(str.chars(), peek_dept),
        }
    }

    pub fn next(&mut self) -> Option<char>
    {
        self.iter.next()
    }

    pub fn peek(&mut self) -> Option<char>
    {
        self.iter.peek().cloned()
    }

    pub fn peek_next(&mut self) -> Option<char>
    {
        self.iter.peek_nth(1).cloned()
    }

    pub fn peek_nth(&mut self, index: usize) -> Option<char>
    {
        self.iter.peek_nth(index).cloned()
    }

    pub fn is_peek(&mut self, ch: char) -> bool
    {
        self.peek().map_or(false, |next_ch| next_ch==ch)
    }

    pub fn is_peek_next(&mut self, ch: char) -> bool
    {
        self.peek_next().map_or(false, |next_ch| next_ch==ch)
    }

    pub fn is_peek_ascii_digit(&mut self) -> bool
    {
        matches!(self.peek(), Some(chr) if chr.is_ascii_digit())
    }

    pub fn is_peek_next_ascii_digit(&mut self) -> bool
    {
        matches!(self.peek_next(), Some(chr) if chr.is_ascii_digit())
    }

    pub fn is_peek_identifier_char(&mut self) -> bool
    {
        matches!(self.peek(), Some(ch) if ch.is_ascii_alphabetic() || ch == '_' || ch.is_ascii_digit())
    }

    pub fn consume_if_peek_is(&mut self, ch: char)
    {
        if self.peek().map_or(false, |next_ch| next_ch==ch) {
            self.next();
        }
    }

    pub fn consume(&mut self)
    {
        self.next();
    }

    pub fn unwrap_next(&mut self) -> char
    {
        self.next().unwrap()
    }
}

#[cfg(test)]
mod tests
{
    use super::Scanner;

    #[test]
    fn test_scanner() {
        let mut scanner = Scanner::from_str("test123", 3);
        assert_eq!(scanner.is_peek('t'), true);
        assert_eq!(scanner.is_peek_next('e'), true);
        assert_eq!(scanner.peek(), Some('t'));
        assert_eq!(scanner.peek_next(), Some('e'));
        assert_eq!(scanner.peek_nth(0), Some('t'));
        assert_eq!(scanner.peek_nth(1), Some('e'));
        assert_eq!(scanner.peek_nth(2), Some('s'));

        assert_eq!(scanner.next(), Some('t'));
        assert_eq!(scanner.is_peek('e'), true);
        assert_eq!(scanner.is_peek_next('s'), true);
        assert_eq!(scanner.is_peek_ascii_digit(), false);
        assert_eq!(scanner.is_peek_next_ascii_digit(), false);
        assert_eq!(scanner.is_peek_identifier_char(), true);

        scanner.consume();
        scanner.consume();
        scanner.consume();
        assert_eq!(scanner.peek(), Some('1'));
        assert_eq!(scanner.is_peek_ascii_digit(), true);
        assert_eq!(scanner.is_peek_next_ascii_digit(), true);
        assert_eq!(scanner.is_peek_identifier_char(), true);
    }

    #[test]
    #[should_panic]
    fn test_scanner_panic() {
        let mut scanner = Scanner::from_str("test", 2);
        assert_eq!(scanner.peek_nth(2), Some('e'));
    }
}