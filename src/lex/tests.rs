use super::*;

#[test]
fn test_lex() {
    let mut tokens = lex("(2 + x)");
    assert_eq!(tokens.next().unwrap(), Token { kind: TokenKind::OpenParen });
    assert_eq!(tokens.next().unwrap(), Token { kind: TokenKind::Integer(2) });
    assert_eq!(tokens.next().unwrap(), Token { kind: TokenKind::Plus });
    assert_eq!(tokens.next().unwrap(), Token { kind: TokenKind::Ident(Symbol::from("x")) });
    assert_eq!(tokens.next().unwrap(), Token { kind: TokenKind::CloseParen });
}
