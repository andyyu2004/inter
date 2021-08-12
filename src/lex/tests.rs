use super::*;

#[test]
fn test_lex() {
    let mut tokens = lex("(2 + x)");
    assert_eq!(tokens.next().unwrap().kind, TokenKind::OpenParen);
    assert_eq!(tokens.next().unwrap().kind, TokenKind::Integer);
    assert_eq!(tokens.next().unwrap().kind, TokenKind::Plus);
    assert_eq!(tokens.next().unwrap().kind, TokenKind::Ident);
    assert_eq!(tokens.next().unwrap().kind, TokenKind::CloseParen);
}
