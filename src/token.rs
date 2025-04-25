// Copyright (c) The Hummanta Authors. All rights reserved.
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//     http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

use logos::Logos;

#[derive(Logos, Debug, PartialEq)]
pub enum Token {
    // Keywords
    #[token("let")]
    Let,

    #[token("fn")]
    Fn,

    #[token("return")]
    Return,

    #[token("if")]
    If,

    #[token("else")]
    Else,

    #[token("while")]
    While,

    #[token("true")]
    True,

    #[token("false")]
    False,

    #[token("null")]
    Null,

    // Identifier
    #[regex(r"[a-zA-Z_][a-zA-Z0-9_]*")]
    Identifier,

    // Literals
    #[regex(r"\d+")]
    IntegerLiteral,

    #[regex(r#""([^"\\]|\\.)*""#)]
    StringLiteral,

    // Operators
    #[token("+")]
    Plus,

    #[token("-")]
    Minus,

    #[token("*")]
    Multiply,

    #[token("/")]
    Divide,

    #[token("<")]
    LessThan,

    #[token(">")]
    GreaterThan,

    #[token("=")]
    Equal,

    #[token("==")]
    EqualEqual,

    #[token("!=")]
    NotEqual,

    #[token("(")]
    LeftParen,

    #[token(")")]
    RightParen,

    #[token("{")]
    LeftBrace,

    #[token("}")]
    RightBrace,

    #[token(";")]
    Semicolon,

    #[regex(r"[ \t\n\r]+", logos::skip)]
    Whitespace,
}

#[cfg(test)]
mod test {
    use logos::Logos;

    use super::Token;

    #[test]
    fn test_let_integer() {
        let mut lexer = Token::lexer("let x = 42;");

        assert_eq!(lexer.next(), Some(Ok(Token::Let)));

        assert_eq!(lexer.next(), Some(Ok(Token::Identifier)));
        assert_eq!(lexer.slice(), "x");

        assert_eq!(lexer.next(), Some(Ok(Token::Equal)));

        assert_eq!(lexer.next(), Some(Ok(Token::IntegerLiteral)));
        assert_eq!(lexer.slice(), "42");

        assert_eq!(lexer.next(), Some(Ok(Token::Semicolon)));
    }

    #[test]
    fn test_let_string() {
        let mut lexer = Token::lexer(r#"let y = "Hello";"#);

        assert_eq!(lexer.next(), Some(Ok(Token::Let)));

        assert_eq!(lexer.next(), Some(Ok(Token::Identifier)));
        assert_eq!(lexer.slice(), "y");

        assert_eq!(lexer.next(), Some(Ok(Token::Equal)));

        assert_eq!(lexer.next(), Some(Ok(Token::StringLiteral)));
        assert_eq!(lexer.slice(), "\"Hello\"");

        assert_eq!(lexer.next(), Some(Ok(Token::Semicolon)));
    }
}
