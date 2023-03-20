pub use blex::Token;
pub use super::parse_token;
use std::fmt;
use std::ops::Range;

#[derive(Clone)]
/// Represents a tree of tokens. A tree, viewed in total, will likely have the
/// same tokens in the same order as a vector of tokens on which it is based.
/// A tree can either have a single token or a list of child parse tokens. This
/// allows trees to be built from single parse tokens.
pub enum ParseNode<'a> {
    Leaf(Range<usize>),
    Branch(Vec<&'a ParseToken<'a>>)
}

#[derive(Clone)]
pub struct ParseToken<'a> {
    pub node: ParseNode<'a>,
    pub body: &'a str,
    pub tags: Vec<&'a str>
}

impl fmt::Display for ParseToken<'_> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        self.write_indented(0, f)?;
        Ok(())
    }
}

impl <'a> ParseToken<'a> {
    fn write_indented(&self, tabs: usize, f: &mut fmt::Formatter) -> fmt::Result {
        for _ in 0..tabs {
            write!(f, "\t")?;
        }
        match &self.node {
            ParseNode::Leaf(r) => {
                write!(f, "{}\n", &self.body[r.clone()])?;
            },
            ParseNode::Branch(children) => {
                for t in &self.tags {
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

    pub fn new_leaf(tok: &'a Token) -> ParseToken<'a> {
        ParseToken { 
            node: ParseNode::Leaf(tok.indices.clone()), 
            body: tok.body, 
            tags: tok.tags.clone() 
        }
    }

    pub fn new_branch(children: Vec<&'a ParseToken>, body: &'a str, tags:Vec<&'a str>) -> ParseToken<'a> {
        ParseToken {
            node: ParseNode::Branch(children),
            body,
            tags
        }
    }

    pub fn new_branch_from_first(children: Vec<&'a ParseToken>, tags:Vec<&'a str>) -> ParseToken<'a> {
        let body = children[0].body;
        ParseToken {
            node: ParseNode::Branch(children),
            body,
            tags
        }
    }

    pub fn content(&'a self) -> &'a str {
        if let Some(cr) = self.content_range() {
            &self.body[cr]
        } else {
            ""
        }
    }

    pub fn content_range(&'a self) -> Option<Range<usize>> {
        match &self.node {
            ParseNode::Leaf(inds) => {
                Some(inds.clone())
            },
            ParseNode::Branch(children) => {
                let known_ranges: Vec<Range<usize>> = children.iter()
                    .flat_map(|&item| item.content_range()).collect();
                if let Some(last) = known_ranges.last() {
                    Some(known_ranges[0].start..last.end)
                } else {
                    None
                }
            }
        }
    }
}