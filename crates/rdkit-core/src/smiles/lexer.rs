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

            // Bonds (may herald a ring closure) --------------------------
            '-' | '=' | '#' | ':' | '/' | '\\' => {
                // If bond char is immediately followed by ring index, fold
                // into a single Ring token capturing the bond type.
                match self.peek() {
                    Some(d @ '0'..='9') => {
                        // single-digit ring number
                        let digit = self.bump().unwrap();
                        let idx = self.read_number(digit);
                        Tok::Ring(idx, Some(c))
                    }
                    Some('%') => {
                        // multi-digit ring index: e.g. "C-%10"
                        self.bump(); // consume '%'
                        let d1 = self.bump()?.to_digit(10)?;
                        let d2 = self.bump()?.to_digit(10)?;
                        Tok::Ring(10 * d1 + d2, Some(c))
                    }
                    _ => Tok::Bond(c),
                }
            }

            // Ring closures without explicit bond ------------------------
            '0'..='9' => Tok::Ring(self.read_number(c), None),
            '%' => {
                // Multi-digit ring index %ab where a,b∈[0-9]
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

            // Elements (standard): uppercase + optional lowercase letter
            'A'..='Z' => {
                let start = self.pos - c.len_utf8();
                if matches!(self.peek(), Some('a'..='z')) {
                    self.bump();
                }
                Tok::Element(&self.src[start..self.pos])
            }

            // Aromatic single-letter lower-case atoms -------------------
            'c' | 'n' | 'o' | 's' | 'p' | 'b' => Tok::Element(&self.src[self.pos - 1..self.pos]),

            // Dot separator between disconnected components -------------
            '.' => Tok::Bond('.'),

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

    #[test]
    fn ring_with_bonds() {
        // Simple cyclohexane ring closure
        let toks: Vec<_> = Lexer::new("C1CCCCC1").collect();
        assert!(matches!(toks[1], Tok::Ring(1, None)) || matches!(toks[6], Tok::Ring(1, None)));

        // Ring with explicit '=' bond preceding the first index
        let toks: Vec<_> = Lexer::new("C=1CCCCC1").collect();
        assert!(matches!(toks[1], Tok::Ring(1, Some('='))));

        // Multi-digit ring index with bond
        let toks: Vec<_> = Lexer::new("C-%12CCCC%12").collect();
        // Expect first Ring token carries '-' bond and idx 12
        assert!(matches!(toks[1], Tok::Ring(12, Some('-'))));
    }

    #[test]
    fn multi_letter_elements_and_dot() {
        let smiles = "Na.ClSiBrCl";
        let toks: Vec<_> = Lexer::new(smiles).collect();
        // Expect sequence: Na . Cl Si Br Cl
        let expected = [
            "Na", ".", "Cl", "Si", "Br", "Cl"
        ];
        let elems: Vec<&str> = toks.into_iter().filter_map(|t| match t {
            Tok::Element(sym) => Some(sym),
            Tok::Bond('.') => Some("."),
            _ => None,
        }).collect();
        assert_eq!(elems, expected);
    }

    #[test]
    fn tokenise_realistic_cases_small_set() {
        let cases = [
            "c1ccc(CCCN2CCC(c3ccccc3)CC2)cc1",
            "F[C@H]1CN(CCCc2c[nH]c3ccc(-n4cnnc4)cc23)CC[C@@H]1NCc1ccccc1C(F)(F)F",
            "N=C(c1cc2ccc(O)cc2[nH]1)N1CCC(Cc2ccccc2)CC1",
            "C[C@@H](C(=O)N1CCC[C@H]1C(=O)N[C@H](C=O)CCCN=C(N)N)c1ccccc1",
            "O=C(Cc1ccc(OCc2ccccc2)cc1)N(O)Cc1ccccc1",
            "O=C(Nc1cccc(Oc2cccc3[nH]c(=O)[nH]c23)c1)c1ccc(Cl)c(C(F)(F)F)c1",
            "Cn1cc(C(=O)c2ccccc2)cc1/C=C/C=C/C(=O)NO",
            "c1ccc2c(NCCCCCCNc3c4c(nc5ccccc35)CCCCC4)c3c(nc2c1)CCCCC3",
            "Nc1ccccc1NC(=O)c1ccc(C(=O)Nc2cccc(Nc3ncc(-c4cccnc4)s3)c2)s1",
            "O=C(O)Cc1ccc(-c2ccccc2NC(=O)c2ccccc2-c2cc(O)c(O)c(O)c2)s1",
        ];
        for s in &cases {
            let toks: Vec<_> = Lexer::new(s).collect();
            assert!(!toks.is_empty(), "{}", s);
        }
    }
}
