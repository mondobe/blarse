pub use blex::Token;
pub use super::parse_token;
use std::fmt;

/// Represents a tree of tokens. A tree, viewed in total, will likely have the
/// same tokens in the same order as a vector of tokens on which it is based.
/// A tree can either have a single token or a list of child parse tokens. This
/// allows trees to be built from single parse tokens.
pub enum ParseToken<'a> {
    Leaf(Token<'a>),
    Branch(Vec<&'a ParseToken<'a>>, Vec<&'a str>)
}

impl fmt::Display for ParseToken<'_> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        self.write_indented(0, f)?;
        Ok(())
    }
}

impl ParseToken<'_> {
    pub fn write_indented(&self, tabs: usize, f: &mut fmt::Formatter) -> fmt::Result {
        for _ in 0..tabs {
            write!(f, "\t")?;
        }
        match self {
            ParseToken::Leaf(tok) => {
                write!(f, "{}\n", tok)?;
            },
            ParseToken::Branch(children, tags) => {
                for t in tags {
                    write!(f, "{}; ", t)?;
                }
                writeln!(f, ":")?;
                for pt in children {
                    pt.write_indented(tabs + 1, f)?;
                }
            }
        }
        Ok(())
    }
}