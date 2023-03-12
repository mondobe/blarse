pub mod parse_token;
pub use blex::*;

#[cfg(test)]
mod tests {
    use super::*;
    use parse_token::*;

    #[test]
    fn print_parse_tokens() {
        let t0 : ParseToken = ParseToken::Leaf(token_from_string("34", vec!["int"]));
        let t1 : ParseToken = ParseToken::Leaf(token_from_string("+", vec!["oper", "plus"]));
        let t2 : ParseToken = ParseToken::Leaf(token_from_string("35", vec!["int"]));

        let pt : ParseToken = ParseToken::Branch(vec![
            &t0,
            &t1,
            &t2
        ], vec!["expr", "addExpr"]);
        println!("{}", pt);
    }
}
