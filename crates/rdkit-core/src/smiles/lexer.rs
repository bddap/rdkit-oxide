//! Tokeniser for SMILES strings – first step toward full parser.

#[derive(Debug, Clone, PartialEq)]
pub enum Tok<'a> {
    Element(&'a str),
    BracketAtom(&'a str),
    Bond(char),
    BranchOpen,
    BranchClose,
    Ring(u32, Option<char>), // number, optional bond symbol preceding
}

#[derive(Debug, Clone)]
pub struct Lexer<'a> { src: &'a str, pos: usize }

impl<'a> Lexer<'a> {
    pub fn new(src: &'a str) -> Self { Self { src, pos: 0 } }

    fn peek(&self) -> Option<char> { self.src[self.pos..].chars().next() }

    fn bump(&mut self) -> Option<char> {
        if let Some(c) = self.peek() {
            self.pos += c.len_utf8();
            Some(c)
        } else { None }
    }

    fn read_number(&mut self, first: char) -> u32 {
        let mut val = first.to_digit(10).unwrap();
        while let Some(c) = self.peek() {
            if let Some(d) = c.to_digit(10) {
                self.bump();
                val = val * 10 + d;
            } else { break }
        }
        val
    }
}

impl<'a> Iterator for Lexer<'a> {
    type Item = Tok<'a>;
    fn next(&mut self) -> Option<Self::Item> {
        let c = self.bump()?;
        Some(match c {
            // Branches ----------------------------------------------------
            '(' => Tok::BranchOpen,
            ')' => Tok::BranchClose,

            // Bonds -------------------------------------------------------
            '-' | '=' | '#' | ':' | '/' | '\\' => Tok::Bond(c),

            // Ring closures ----------------------------------------------
            '0'..='9' => Tok::Ring(self.read_number(c), None),
            '%' => {
                // Multi-digit ring index %10 …
                let d1 = self.bump()?.to_digit(10)?;
                let d2 = self.bump()?.to_digit(10)?;
                Tok::Ring(10 * d1 + d2, None)
            }

            // Bracket atom ----------------------------------------------
            '[' => {
                let start = self.pos;
                while self.bump()? != ']' {}
                Tok::BracketAtom(&self.src[start..self.pos - 1])
            }

            // Elements / aromatic symbols --------------------------------
            'B' | 'C' | 'N' | 'O' | 'P' | 'S' | 'F' | 'I' => {
                let start = self.pos - c.len_utf8();
                // optional second char (l,r) for Cl/Br
                if (c == 'C' && matches!(self.peek(), Some('l'))) || (c == 'B' && matches!(self.peek(), Some('r'))) {
                    self.bump();
                }
                Tok::Element(&self.src[start..self.pos])
            }

            'c' | 'n' | 'o' | 's' | '*' => Tok::Element(&self.src[self.pos - 1..self.pos]),

            _ => return None, // invalid char – for brevity
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn tokenise_simple() {
        let toks: Vec<_> = Lexer::new("CC(=O)O").collect();
        assert_eq!(toks.len(), 7); // C C ( = O ) O
    }
}
