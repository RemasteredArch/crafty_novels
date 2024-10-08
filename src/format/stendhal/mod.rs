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

//! Parsing for [Stendhal] format.
//! See [`Stendhal`] for more details.
//!
//! [Stendhal]: https://modrinth.com/mod/stendhal
//!
//! # Examples
//!
//! ```rust
//! use crafty_novels::{
//!     import::Stendhal,
//!     syntax::{minecraft::Format, Metadata, Token, TokenList},
//!     Tokenize,
//! };
//! # use std::error::Error;
//!
//! # fn main() -> Result<(), Box<dyn Error>> {
//! let input = "title: crafty_novels
//! author: RemasteredArch
//! pages:
//! ##- Italic:§o text §rreset";
//!
//! let expected_metadata = Box::new([
//!     Metadata::Title("crafty_novels".into()),
//!     Metadata::Author("RemasteredArch".into()),
//! ]);
//! let expected_tokens = Box::new([
//!     Token::ThematicBreak,
//!     Token::Text("Italic:".into()),
//!     Token::Format(Format::Italic),
//!     Token::Space,
//!     Token::Text("text".into()),
//!     Token::Space,
//!     Token::Format(Format::Reset),
//!     Token::Text("reset".into()),
//!     Token::LineBreak,
//! ]);
//!
//! assert_eq!(
//!     Stendhal::tokenize_string(input)?,
//!     TokenList::new_from_boxed(expected_metadata, expected_tokens)
//! );
//! #
//! #     Ok(())
//! # }
//! ```

use crate::{
    syntax::{Token, TokenList},
    Tokenize,
};
pub use error::TokenizeError;
use std::io::{BufRead, BufReader, Read};

mod error;
mod parse;
#[cfg(test)]
mod test;

/// Parses the [Stendhal] format.
///
/// # Expected format
///
/// *Convention: `"a string"` `'a single character'` (the `"` or `'` are not necessarily present).*
///
/// The first three lines make up the frontmatter:
/// 1. Starts with `"title: "`, the rest is considered the title of the book
/// 2. Starts with `"author: "`, the rest is considered the author's name, which is probably
///    whoever exported the book
/// 3. Starts and ends with `"pages:"`
///
/// For the rest of the book:
/// - Any line that starts with `"#- "` is considered the start of a new page, and the text
///   following the `"#- "` makes up the first line of the new page
/// - `'§'`, followed a one of a set of characters makes up a format code, represented in syntax by
///   [`Format`][`crate::syntax::minecraft::Format`]
///     - The resulting format continues until the next line ending or
///       [reset][`crate::syntax::minecraft::Format::Reset`] format code
///
/// [Stendhal]: https://modrinth.com/mod/stendhal
pub struct Stendhal;

impl Tokenize for Stendhal {
    type Error = TokenizeError;

    /// Parse a string in the Stendhal format into an abstract syntax vector.
    ///
    /// # Errors
    ///
    /// - [`crate::syntax::ConversionError::MissingFormatCode`] if it encounters a `'§'` that isn't
    ///   followed by another character
    /// - [`crate::syntax::ConversionError::NoSuchFormatCode`] if it encounters a `'§'` isn't
    ///   followed by a valid [`Format`][`crate::syntax::minecraft::Format`] character
    /// - [`TokenizeError::IncompleteOrMissingFrontmatter`] if `input` ends before the frontmatter
    ///   parsing is finished
    fn tokenize_string(input: &str) -> Result<TokenList, Self::Error> {
        let mut input = input.lines();
        let mut tokens: Vec<Token> = vec![];

        // Could be recovered by capturing the state of `input` before calling, then reverting on
        // certain errors.
        let metadata = parse::frontmatter(&mut input)?;

        for line in input {
            parse::line(&mut tokens, line)?;
        }

        Ok(TokenList::new_from_boxed(metadata, tokens.into()))
    }

    /// Parse a file in the Stendhal format into an abstract syntax vector.
    ///
    /// # Errors
    ///
    /// - [`crate::syntax::ConversionError::MissingFormatCode`] if it encounters a `'§'` that isn't
    ///   followed by another character
    /// - [`crate::syntax::ConversionError::NoSuchFormatCode`] if it encounters a `'§'` isn't
    ///   followed by a valid [`Format`][`crate::syntax::minecraft::Format`] character
    /// - [`TokenizeError::IncompleteOrMissingFrontmatter`] if `input` ends before the frontmatter
    ///   parsing is finished
    /// - [`TokenizeError::Io`] if the a line from `input` is an I/O error of some kind
    fn tokenize_reader(input: impl Read) -> Result<TokenList, Self::Error> {
        /// Get a refrence to the next element in `$iter` or return [`Error::UnexpectedEndOfIter`]
        /// or the encapsulated [`Error::Io`].
        macro_rules! next {
            ($iter:expr) => {
                &$iter
                    .next()
                    .ok_or(Self::Error::IncompleteOrMissingFrontmatter)??
            };
        }

        let mut iter = BufReader::new(input).lines();
        let mut tokens: Vec<Token> = vec![];

        let chunk: [&str; 3] = [next!(iter), next!(iter), next!(iter)];
        let metadata = parse::frontmatter(&mut chunk.into_iter())?;

        for line in iter {
            parse::line(&mut tokens, &line?)?;
        }

        Ok(TokenList::new_from_boxed(metadata, tokens.into()))
    }
}
