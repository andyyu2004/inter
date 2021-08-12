macro_rules! parse_expr {
    ($src:expr) => {{
        let tokens = $crate::lex::lex($src);
        $crate::parse::Parser::new(tokens).parse_expr().unwrap()
    }};
}

#[test]
fn test_simple_parse() {
    let expr = parse_expr!("(2 + 2)");
    dbg!(&expr);
}

#[test]
fn test_precedence_parse() {
    let expr = parse_expr!("2 + 3 * 4");
    dbg!(&expr);
}

#[test]
fn test_unary_parse() {
    let expr = parse_expr!("--!!---3");
    dbg!(&expr);
}
