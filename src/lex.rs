use string_cache::{Atom, EmptyStaticAtomSet};

#[derive(Clone, PartialEq, Eq, Debug)]
pub struct Token {
    pub kind: TokenKind,
    pub lexeme: String,
}

impl Token {
    pub const EOF: Self = Token { kind: TokenKind::Eof, lexeme: String::new() };
}

#[derive(Clone, PartialEq, Eq, Debug)]
pub enum TokenKind {
    Integer,
    Ident,
    OpenParen,
    CloseParen,
    Plus,
    Minus,
    Star,
    Slash,
    Bang,
    Eof,
}

pub type Symbol = Atom<EmptyStaticAtomSet>;

pub fn lex(src: &str) -> impl Iterator<Item = Token> + '_ {
    let mut i = 0;
    let convert_token = move |token: rustc_lexer::Token| -> Option<Token> {
        i += token.len;
        let lexeme = &src[i - token.len..i];
        let kind = match token.kind {
            rustc_lexer::TokenKind::LineComment | rustc_lexer::TokenKind::Whitespace =>
                return None,
            rustc_lexer::TokenKind::BlockComment { terminated } => {
                assert!(terminated, "unterminated block comment");
                return None;
            }
            rustc_lexer::TokenKind::Ident => TokenKind::Ident,
            rustc_lexer::TokenKind::RawIdent => todo!(),
            rustc_lexer::TokenKind::Literal { kind, suffix_start } => match kind {
                rustc_lexer::LiteralKind::Int { base, empty_int } => TokenKind::Integer,
                rustc_lexer::LiteralKind::Float { base, empty_exponent } => todo!(),
                rustc_lexer::LiteralKind::Char { terminated } => todo!(),
                rustc_lexer::LiteralKind::Byte { terminated } => todo!(),
                rustc_lexer::LiteralKind::Str { terminated } => todo!(),
                rustc_lexer::LiteralKind::ByteStr { terminated } => todo!(),
                rustc_lexer::LiteralKind::RawStr { n_hashes, started, terminated } => todo!(),
                rustc_lexer::LiteralKind::RawByteStr { n_hashes, started, terminated } => todo!(),
            },
            rustc_lexer::TokenKind::Lifetime { starts_with_number } => todo!(),
            rustc_lexer::TokenKind::Semi => todo!(),
            rustc_lexer::TokenKind::Comma => todo!(),
            rustc_lexer::TokenKind::Dot => todo!(),
            rustc_lexer::TokenKind::OpenParen => TokenKind::OpenParen,
            rustc_lexer::TokenKind::CloseParen => TokenKind::CloseParen,
            rustc_lexer::TokenKind::OpenBrace => todo!(),
            rustc_lexer::TokenKind::CloseBrace => todo!(),
            rustc_lexer::TokenKind::OpenBracket => todo!(),
            rustc_lexer::TokenKind::CloseBracket => todo!(),
            rustc_lexer::TokenKind::At => todo!(),
            rustc_lexer::TokenKind::Pound => todo!(),
            rustc_lexer::TokenKind::Tilde => todo!(),
            rustc_lexer::TokenKind::Question => todo!(),
            rustc_lexer::TokenKind::Colon => todo!(),
            rustc_lexer::TokenKind::Dollar => todo!(),
            rustc_lexer::TokenKind::Eq => todo!(),
            rustc_lexer::TokenKind::Not => TokenKind::Bang,
            rustc_lexer::TokenKind::Lt => todo!(),
            rustc_lexer::TokenKind::Gt => todo!(),
            rustc_lexer::TokenKind::Minus => TokenKind::Minus,
            rustc_lexer::TokenKind::And => todo!(),
            rustc_lexer::TokenKind::Or => todo!(),
            rustc_lexer::TokenKind::Plus => TokenKind::Plus,
            rustc_lexer::TokenKind::Star => TokenKind::Star,
            rustc_lexer::TokenKind::Slash => TokenKind::Slash,
            rustc_lexer::TokenKind::Caret => todo!(),
            rustc_lexer::TokenKind::Percent => todo!(),
            rustc_lexer::TokenKind::Unknown => todo!(),
        };
        Some(Token { kind, lexeme: lexeme.to_owned() })
    };
    rustc_lexer::tokenize(src).filter_map(convert_token).chain(Some(Token::EOF))
}

#[cfg(test)]
mod tests;
