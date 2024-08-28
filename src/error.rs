// SPDX-License-Identifier: AGPL-3.0-or-later
//
// Copyright © 2024 RemasteredArch
//
// This file is part of crafty_novels.
//
// crafty_novels is free software: you can redistribute it and/or modify it under the terms of the
// GNU Affero General Public License as published by the Free Software Foundation, either version
// 3 of the License, or (at your option) any later version.
//
// crafty_novels is distributed in the hope that it will be useful, but WITHOUT ANY WARRANTY;
// without even the implied warranty of MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE. See
// the GNU Affero General Public License for more details.
//
// You should have received a copy of the GNU Affero General Public License along with
// crafty_novels. If not, see <https://www.gnu.org/licenses/>.

//! Error definitions for the crate.
//!
//! See [`Error`].

use crate::syntax::Token;

/// Represents the various possible errors for the crate.
#[derive(thiserror::Error, Debug)]
pub enum Error {
    /// Encountered when attempting to parse a malformed format string, ex. `"§ 0"` instead of
    /// `"§0"`.
    #[error("expected a two character string starting with §, received '{0}'")]
    InvalidFormatCodeString(String),
    /// Encountered when attempting to parse a format string with an invalid format code.
    #[error("no such format code '{0}'")]
    NoSuchFormatCode(char),
    /// Encountered when `'§'` is encountered but not followed by a format code.
    #[error("expected a format code after '§'")]
    MissingFormatCode,
    /// Encountered when an no HTML entity is associated with the given [`char`].
    #[error("no HTML entity associated with character '{0}'")]
    NoSuchCharLiteral(char),
    /// Encountered when an iterator ends before its consumer is finished.
    #[error("expected iterator to be longer")]
    UnexpectedEndOfIter,
    /// Encountered when trying to parse an frontmatter that is incomplete or entirely missing.
    #[error("frontmatter is not present or incomplete")]
    IncompleteOrMissingFrontmatter,
    /// Encoutered a given [`Token`] in an unexpected place.
    #[error("did not expect token")]
    UnexpectedToken(Token),
    /// Encoutered when an I/O action fails in some way.
    #[error("could not perform I/O action")]
    Io(#[from] std::io::Error),
    /// Encoutered when an [`std::fmt`] function fails in some way.
    #[error("could not format item")]
    Fmt(#[from] std::fmt::Error),
    /// Encoutered when attempting to convert invallid UTF-8 into a string.
    #[error("could not convert to UTF-8")]
    Utf8(#[from] std::string::FromUtf8Error),
}

/// Represents the possible errors encountered when parsing a document in a flexible way.
#[allow(clippy::module_name_repetitions)] // This will be exported outside of `error`
#[derive(thiserror::Error, Debug)]
pub enum TokenizeError {
    #[error("no such syntax item")]
    NoSuchSyntaxItem,
    #[error("malformed syntax item")]
    MalformedSyntaxItem,
    #[error("did not expect syntax item here")]
    UnexpectedSyntaxItem,
    #[error("{0}")]
    Other(#[from] Box<dyn std::error::Error>),
    #[error("could not perform I/O action")]
    Io(#[from] std::io::Error),
    #[error("could not format item")]
    Fmt(#[from] std::fmt::Error),
    #[error("invalid UTF-8")]
    Utf8(#[from] std::string::FromUtf8Error),
}

impl From<Error> for TokenizeError {
    fn from(err: Error) -> Self {
        use TokenizeError::{
            Fmt, Io, MalformedSyntaxItem, NoSuchSyntaxItem, UnexpectedSyntaxItem, Utf8,
        };
        match err {
            Error::InvalidFormatCodeString(_)
            | Error::NoSuchFormatCode(_)
            | Error::MissingFormatCode
            | Error::UnexpectedEndOfIter
            | Error::IncompleteOrMissingFrontmatter => MalformedSyntaxItem,
            Error::NoSuchCharLiteral(_) => NoSuchSyntaxItem,
            Error::UnexpectedToken(_) => UnexpectedSyntaxItem,
            Error::Io(e) => Io(e),
            Error::Fmt(e) => Fmt(e),
            Error::Utf8(e) => Utf8(e),
        }
    }
}
