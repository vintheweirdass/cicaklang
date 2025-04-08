use crate::{
    error::{LexError, NumberError, SpannedError, StringError, UnicodeEscapeError},
    util::PeekableWithPoint,
};
use alloc::{string::String, vec::Vec};
pub fn tokenize<'a>(
    chars: &mut PeekableWithPoint<'a>,
) -> Result<Option<Vec<LexToken<'a>>>, SpannedError<LexError>> {
    let mut tokens: Vec<LexToken> = Vec::new();

    while let Some(c) = chars.next() {
        match c {
            '/' => tokens.push(LexToken::Slash),
            ';' => tokens.push(LexToken::Semicolon),
            ':' => tokens.push(LexToken::Colon),
            ',' => tokens.push(LexToken::Comma),
            '.' => tokens.push(LexToken::Dot),
            '@' => tokens.push(LexToken::At),
            '=' => tokens.push(LexToken::Equal),
            '+' => tokens.push(LexToken::Plus),
            '-' => tokens.push(LexToken::Minus),
            '<' => tokens.push(LexToken::Comparison(ParenSide::Left)),
            '>' => tokens.push(LexToken::Comparison(ParenSide::Right)),
            '{' => tokens.push(LexToken::Brace(ParenStatus::Open)),
            '}' => tokens.push(LexToken::Brace(ParenStatus::Closed)),
            '[' => tokens.push(LexToken::Bracket(ParenStatus::Open)),
            ']' => tokens.push(LexToken::Bracket(ParenStatus::Closed)),
            '"' => tokens.push(LexToken::String(tokenize_string(chars)?)),
            ' ' | '\t' => {
                while let Some(v) = chars.peek() {
                    if v != &' ' && v != &'\t' {
                        break;
                    }
                    chars.next();
                }
                tokens.push(LexToken::Spaces);
            }
            '\r' | '\n' => {
                while let Some(v) = chars.peek() {
                    if  v != &'\r' || v != &'\n' {
                        break;
                    }
                    chars.next();
                }
                tokens.push(LexToken::Newlines);
            }
            '0'..='9' => {
                tokens.push(LexToken::Number(tokenize_number(chars)?))
            }
            '_' | 'a'..='z' | 'A'..='Z' => {
                tokens.push(LexToken::Ident(tokenize_ident(chars)?));
            }
            _ => return Err(chars.as_spanned_error(LexError::UnexpectedChar)),
        }
    }

    if tokens.len()<1 {
        return Ok(None)
    }
    return Ok(Some(tokens));
}
pub fn tokenize_ident<'a>(
    chars: &mut PeekableWithPoint<'a>,
) -> Result<&'a str, SpannedError<LexError>> {
    let start: usize = chars.num_into_usize(chars.point.index -1);
    while let Some(ch) = chars.peek() {
        if !ch.is_alphanumeric() && ch != &'_' {
            break;
        }
        chars.next();
    }
    let end: usize = chars.num_into_usize(chars.point.index);
    return Ok(&chars.point.content[start..end]);
}
pub fn tokenize_string<'a>(
    chars: &mut PeekableWithPoint<'a>,
) -> Result<&'a str, SpannedError<LexError>> {
    let mut result = String::new();
    let start: usize = chars.num_into_usize(chars.point.index);
    while let Some(c) = chars.next() {
        match c {
            '"' => {
                let end: usize = chars.num_into_usize(chars.point.index-1);
                return Ok(&chars.point.content[start..end]);
            } // end of string
            '\\' => {
                if let Some(escaped) = chars.next() {
                    result.push(match escaped {
                        'n' => '\n',
                        't' => '\t',
                        'r' => '\r',
                        '0' => '\0',
                        '"' => '"',
                        '\'' => '\'',
                        '\\' => '\\',
                        'u' => {
                            if chars.next() == Some('{') {
                                let mut hex = String::new();
                                while let Some(ch) = chars.next() {
                                    if ch == '}' {
                                        chars.next();
                                        break;
                                    }
                                    hex.push(ch);
                                    chars.next();
                                }
                                if let Ok(codepoint) = u32::from_str_radix(&hex, 16) {
                                    if let Some(ch) = char::from_u32(codepoint) {
                                        result.push(ch);
                                    }
                                    return Err(chars.as_spanned_error(LexError::String(
                                        StringError::UnicodeEscape(UnicodeEscapeError::InvalidHex),
                                    )));
                                }
                                return Err(chars.as_spanned_error(LexError::String(
                                    StringError::UnicodeEscape(UnicodeEscapeError::InvalidHex),
                                )));
                            }
                            return Err(chars.as_spanned_error(LexError::String(
                                StringError::UnicodeEscape(UnicodeEscapeError::MissingOpeningBrace),
                            )));
                        }
                        other => {
                            return Err(chars.as_spanned_error(LexError::String(
                                StringError::UnknownEscape(other),
                            )));
                        }
                    });
                } else {
                    return Err(chars.as_spanned_error(LexError::UnexpectedEof));
                }
            }
            _ => result.push(c),
        }
    }
    return Err(chars.as_spanned_error(LexError::String(StringError::UnterminatedStringLiteral)));
}
pub fn tokenize_number<'a>(
    chars: &mut PeekableWithPoint<'a>,
) -> Result<NumberToken<'a>, SpannedError<LexError>> {
    let start: usize = chars.num_into_usize(chars.point.index - 1);
    let mut dot_cap = false;
    while let Some(ch) = chars.peek() {
        if !(ch.is_numeric() || ch == &'.') {
            break;
        }
        if ch == &'.' {
            if dot_cap {
                return Err(chars.as_spanned_error(LexError::Number(NumberError::TooManyDots)));
            }
            dot_cap = true;
        }
        chars.next();
    }
    let end: usize = chars.num_into_usize(chars.point.index);
    let char = &chars.point.content[start..end];
 
    return Ok(if dot_cap {
        NumberToken::Decimal(char)
    } else {
        NumberToken::Integer(char)
    });
}
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum ParenStatus {
    Open,
    Closed,
}
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum ParenSide {
    Left,
    Right,
}
#[derive(Debug, Clone, Copy, PartialEq, PartialOrd)]
pub enum LexToken<'a> {
    Slash,
    Spaces,
    Newlines,
    Minus,
    Plus,
    Equal,
    Semicolon,
    Colon,
    Comma,
    Dot,
    At,
    Number(NumberToken<'a>),
    String(&'a str),
    Ident(&'a str),
    Bracket(ParenStatus),
    Brace(ParenStatus),
    Comparison(ParenSide),
}
#[derive(Debug, Clone, Copy, PartialEq, PartialOrd)]
pub enum NumberToken<'a> {
    Decimal(&'a str),
    Integer(&'a str)
}

// #[derive(Debug, Clone, Copy, PartialEq, PartialOrd)]
// pub enum NumberToken {
//     Usize(usize),
//     U128(u128),
//     U64(u64),
//     U32(u32),
//     U16(u16),
//     U8(u8),
//     // https://github.com/rust-lang/rust/issues/116909
//     // unstable F128(f128),
//     F64(f64),
//     F32(f32),
//     // https://github.com/rust-lang/rust/issues/116909
//     // unstable F16(f16),
//     // not exist F8(f8),
//     Isize(isize),
//     I128(i128),
//     I64(i64),
//     I32(i32),
//     I16(i16),
//     I8(i8),
// }
