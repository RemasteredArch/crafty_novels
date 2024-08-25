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

use std::sync::Arc;

pub mod minecraft;

#[derive(Debug, Clone)]
pub struct TokenList {
    metadata: Arc<[Metadata]>,
    tokens: Arc<[Token]>,
}

impl TokenList {
    pub const fn new(metadata: Arc<[Metadata]>, tokens: Arc<[Token]>) -> Self {
        Self { metadata, tokens }
    }

    pub fn new_from_boxed(metadata: Box<[Metadata]>, tokens: Box<[Token]>) -> Self {
        Self {
            metadata: metadata.into(),
            tokens: tokens.into(),
        }
    }

    pub fn metadata_as_slice(&self) -> &[Metadata] {
        &self.metadata
    }

    pub fn tokens_as_slice(&self) -> &[Token] {
        &self.tokens
    }

    pub fn metadata(&self) -> Arc<[Metadata]> {
        self.metadata.clone()
    }

    pub fn tokens(&self) -> Arc<[Token]> {
        self.tokens.clone()
    }
}

/// A lexical token.
///
/// Represents an abstract representation of text or formatting.
#[derive(PartialEq, Eq, Debug)]
pub enum Token {
    Text(Box<str>),
    /// A hidden node to control text formatting.
    Format(minecraft::Format),
    Space,
    LineBreak,
    ParagraphBreak,
    /// A page break.
    ThematicBreak,
}

impl Token {
    /// Whether or not a [`Token`] corresponds to some kind of line break.
    pub const fn is_break(&self) -> bool {
        matches!(
            *self,
            Self::LineBreak | Self::ParagraphBreak | Self::ThematicBreak | Self::Space
        )
    }

    /// Whether or not a [`Token`] corresponds to some kind of white space character.
    pub const fn is_white_space(&self) -> bool {
        matches!(*self, Self::Space) || self.is_break()
    }

    /// Whether or not a [`Token`] is [`Token::Text`].
    pub const fn is_text(&self) -> bool {
        matches!(*self, Self::Text(_))
    }
}

impl From<&mut Vec<char>> for Token {
    /// Drain a [`Vec<char>`] to build a text node.
    fn from(value: &mut Vec<char>) -> Self {
        Self::Text(value.drain(..).collect::<Box<str>>())
    }
}

/// Metadata about a literary work.
#[derive(PartialEq, Eq, Debug)]
pub enum Metadata {
    Title(Box<str>),
    Author(Box<str>),
}
