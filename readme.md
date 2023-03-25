Blarse is an even-more-lightweight parsing framework built in Rust. In fact, it could hardly even be called a parsing framework in the traditional sense: it is just an implementation of a token tree with a few basic functions. 

The structure of a parse tree in Blarse looks something like this:

```
int   y        =  applyFunction (     (     double )     x    )     ;
^^^   ^        ^  ^^^^^^^^^^^^^ ^     ^     ^^^^^^ ^     ^    ^     ^
Name  Name     Eq Name          Open  Open  Name   Close Name Close Semicolon
                                paren paren        Paren      Paren
^^^   ^        ^  ^^^^^^^^^^^^^ ^     ^^^^^^^^^^^^^^     ^    ^     ^
Name  Name     Eq Name          Open  Expr               Name Close Semicolon
                                paren                         Paren
^^^   ^        ^  ^^^^^^^^^^^^^ ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^     ^
Name  Name     Eq Name          Expr                                Semicolon
^^^   ^        ^  ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^     ^
Name  Name     Eq Expr                                              Semicolon
^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
Statement
```

This looks difficult to understand on the surface, and the end result is certainly opaque-looking. However, when writing the parser, we can look at this process in reverse. Instead of managing to combine two names, an equals sign, an expression, and a semicolon into a statement, we can simply understand that a statement requires these components, then see if those rules apply here. 

Like Blex, Blarse processes data by slowly transforming it while keeping its type the same. Blex transforms tokens, which start out as thin wrappers around individual characters, into more tokens, this time acting as complex and data-rich representations of lexemes. Similarly, Blarse transforms parse tokens, which begin as a thin layer over simple tokens, into more parse tokens, which build up into a top-down tree structure for crystal-clear understanding. 

Once again like Blex, processing in Blarse is done with rules of one trait bound. The bound in question is `Fn(Vec<ParseToken>) -> Vec<ParseToken>`. Just like with Blex, it is fine to mutate these parse tokens, and, in fact, this is an idiomatic way to process them. 

## Let's get Lispy!

Let's create a rule `eval` which parses S-expressions, as seen in Lisps. We'll try to parse this example code from the Quick Racket introduction:

```
(define (rgb-series mk)
  (vc-append
   (series (lambda (sz) (colorize (mk sz) "red")))
   (series (lambda (sz) (colorize (mk sz) "green")))
   (series (lambda (sz) (colorize (mk sz) "blue")))))
```

First of all, we'll need some lexer code to identify the parentheses and words. To do this this, we'll have four simple rules.

1. Tag any whitespace as "ws".
2. Tag "(" and ")" as "paren".
3. Tag any strings of characters, delimited by parens or whitespace, as "word"s.
4. Remove any whitespace.

These turn something like `A (space)` into 

```
A: word;
(: (; paren;
space: word;
): ); paren;
```

From there, we need to create a rule for parsing. The parser should turn all S-expressions (groups of words and S-expressions inside parentheses) into their own parse tokens, with the expression's elements as children. For example, `A (space)` should turn into a parse token containing one leaf token and a branch token, itself containing the leaf token `space`. So:

```
A ("word")

("expr"):
    space ("word")
```

The simplest method for implementing this involves a mixture of iteration and recursion. The function should scan the parse tokens for a left parenthesis, then a right parenthesis. When both of these are found, the function should recursively call itself on the parse tokens in between the parenthesis and wrap the result of that up into a parent parse token. 

We start with the typical function signature, giving our function the simple name of `eval`:

```
fn eval(mut pts: Vec<ParseToken>) -> Vec<ParseToken> {
	...
}
```

We will be mutating our `pts` input parameter, not copying it. Thus, we can go ahead and return `pts` at the end.

```
fn eval(mut pts: Vec<ParseToken>) -> Vec<ParseToken> {
	pts
}
```

Before that, though, we need to iterate through our parse tokens as described before. We can use a state machine to track whether or not our parentheses have been located. This would look something like:

```
A    (deeply    (nested) expression)
     ^          ^      ^
     Left paren found! |
               Left paren found!
                       Right paren found! (Execution complete)
```

This signals that we need two states: no parens found or a left paren found. We could use a boolean value to track this, but a more elegant option would be to store the index of the last left paren found as an `Option<usize>`. We can then iterate through the indices in `pts` and update this value if a left paren is found.

```
fn eval(mut pts: Vec<ParseToken>) -> Vec<ParseToken> {
    let mut l_index: Option<usize> = None;
	
    for i in 0..pts.len() {
        if pts[i].has_tag("(") {
            l_index = Some(i);
        }
    }
    pts
}
```

Now we just need to add the branch that is triggered when a right paren is found and `l_index` is `Some`. 

```
fn eval(mut pts: Vec<ParseToken>) -> Vec<ParseToken> {
    let mut l_index: Option<usize> = None;
	
    for i in 0..pts.len() {
        if pts[i].has_tag("(") {
            l_index = Some(i);
        } else if let Some(l) = l_index {
            if pts[i].has_tag(")") {
	            ...
            }
        }
    }
    pts
}
```

This branch should call `eval` recursively on the subset of `pts` from `l_index` to `i`, exclusively. It should then create a new parse token with the results of this call as its children, then splice this parse token in to replace the subset of `pts` from `l_index` to `i`, *in*clusively. Luckily, Rust has a built-in function to do this last part for us, `Vec::splice`.

```
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
            }
        }
    }
    pts
}
```

The first line (starting with `let pts_slice`) defines the area to evaluate. We then pass that into `eval` in the initializer for `new_expr`, making sure to add `expr` as a tag to the new parse token. We then call that useful `splice` function. 

However, this behavior doesn't take into account the possibility of multiple S-expressions at the same level of nesting. Remember that our original state machine terminated after a right paren was found. We can do the same thing by recursively calling `eval` on `pts` and early returning the result. 

```
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
```

Our new expression will be ignored since the parens are no longer included in it, so the next call to `eval` will advance to the next S-expression. Let's see the results on our original corpus ([[#Let's get Lispy!]]):

```
("expr"):
        define ("word")
        ("expr"):
                rgb-series ("word")
                mk ("word")
        ("expr"):
                vc-append ("word")
                ("expr"):
                        series ("word")
                        ("expr"):
                                lambda ("word")
                                ("expr"):
                                        sz ("word")
                                ("expr"):
                                        colorize ("word")
                                        ("expr"):
                                                mk ("word")
                                                sz ("word")
                                        "red" ("word")
                ("expr"):
                        series ("word")
                        ("expr"):
                                lambda ("word")
                                ("expr"):
                                        sz ("word")
                                ("expr"):
                                        colorize ("word")
                                        ("expr"):
                                                mk ("word")
                                                sz ("word")
                                        "green" ("word")
                ("expr"):
                        series ("word")
                        ("expr"):
                                lambda ("word")
                                ("expr"):
                                        sz ("word")
                                ("expr"):
                                        colorize ("word")
                                        ("expr"):
                                                mk ("word")
                                                sz ("word")
                                        "blue" ("word")

()
```

It's verbose, but it's right! We have successfully parsed some Lisp. ...Except for that irksome empty token at the end. We can deal with that using another function, `remove_last`. 

```
fn remove_last(mut pts: Vec<ParseToken>) -> Vec<ParseToken> {
    if pts.len() > 0 {
        pts.remove(pts.len() - 1);
    }
    pts
}
```

I won't repeat the output, but trust that that nagging token is gone.