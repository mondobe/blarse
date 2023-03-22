pub mod parse_token;
#[cfg(test)]
pub mod lispy_tests;
pub use blex::*;

#[cfg(test)]
mod tests {
    use super::*;
    use parse_token::*;

    #[test]
    fn print_parse_tokens() {
        let body = "34 + 35";

        let tox = vec![
            Token{body, indices: 0..2, tags: vec!["int"]},
            Token{body, indices: 3..4, tags: vec!["oper", "plus"]},
            Token{body, indices: 5..7, tags: vec!["int"]}
        ];

        let pts = vec![
            ParseToken::new_leaf(tox[0].clone()),
            ParseToken::new_leaf(tox[1].clone()),
            ParseToken::new_leaf(tox[2].clone())
        ];

        let pt : ParseToken = ParseToken::new_branch_from_first(pts, vec!["expr", "addExpr"]);
        
        println!("{}", pt);
        println!("{}", pt.content());
    }
}
