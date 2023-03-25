pub use super::parse_token::*;
pub use super::*;
pub use blex::token::*;


fn whitespace_rule(mut tokens: Vec<Token>) -> Option<Vec<Token>> {
    let ch = tokens[0].single_char().unwrap_or_default();
    if ch.is_whitespace() || ch == '\u{0}' {
        tokens[0].tags.push("ws");
    }
    Some(tokens)
}

fn paren_rule(mut tokens: Vec<Token>) -> Option<Vec<Token>> {
    let ch = tokens[0].single_char().unwrap_or_default();
    if ch == '(' || ch == ')' {
        tokens[0].tags.push("paren");
    }
    Some(tokens)
}

fn word_rule(tokens: Vec<Token>) -> Option<Vec<Token>> {
    let empty = &empty_token();
    let last_token = tokens.last().unwrap_or(empty);
    if !last_token.has_tag("ws") && !last_token.has_tag("paren") {
        None
    } else if tokens.len() == 1 {
        Some(tokens)
    } else {
        Some(vec![
            wrap(tokens[0..tokens.len() - 1].to_vec(), vec!["word"]),
            tokens.last().unwrap().clone(),
        ])
    }
}

fn remove_whitespace_rule(tokens: Vec<Token>) -> Option<Vec<Token>> {
    if tokens[0].has_tag("ws") {
        Some(vec![])
    } else {
        Some(tokens)
    }
}

fn s_expr_rules() -> Vec<impl Fn(Vec<Token>) -> Option<Vec<Token>>> {
    [
        whitespace_rule,
        paren_rule,
        word_rule,
        remove_whitespace_rule,
    ]
    .to_vec()
}

fn remove_last(mut pts: Vec<ParseToken>) -> Vec<ParseToken> {
    if pts.len() > 0 {
        pts.remove(pts.len() - 1);
    }
    pts
}

fn eval(mut pts: Vec<ParseToken>) -> Vec<ParseToken> {
    let mut l_index: Option<usize> = None;
    
    for i in 0..pts.len() {
        if pts[i].has_tag("(") {
            l_index = Some(i);
        } else if let Some(l) = l_index {
            if pts[i].has_tag(")") {
                let pts_slice = pts[(l + 1)..i].to_vec();
                let new_expr = ParseToken::new_branch_from_first(
                    eval(pts_slice), 
                    vec!["expr"]);
                pts.splice(l..=i, vec![new_expr]);
                return eval(pts);
            }
        }
    }
    pts
}

#[test]
pub fn parse_s_exprs() {
    let text = "
(define (rgb-series mk)
  (vc-append
   (series (lambda (sz) (colorize (mk sz) \"red\")))
   (series (lambda (sz) (colorize (mk sz) \"green\")))
   (series (lambda (sz) (colorize (mk sz) \"blue\")))))";

    let mut body = str_to_tokens(text);
    process_rules(s_expr_rules(), &mut body, false);

    println!("===   TOKENS   ===");
    print_tokens(body.clone());

    println!("===PARSE TOKENS===");

    let pts = tokens_to_parse_tokens(body);
    print_parse_tokens(remove_last(eval(pts)));
}

#[test]
pub fn parse_s_exprs2() {
    let text = "A (space)";

    let mut body = str_to_tokens(text);
    process_rules(s_expr_rules(), &mut body, false);

    println!("===   TOKENS   ===");
    print_tokens(body.clone());

    println!("===PARSE TOKENS===");

    let pts = tokens_to_parse_tokens(body);
    print_parse_tokens(remove_last(eval(pts)));
}