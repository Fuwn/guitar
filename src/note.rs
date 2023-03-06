// This file is part of Guitar <https://github.com/Fuwn/guitar>.
//
// This program is free software: you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation, version 3.
//
// This program is distributed in the hope that it will be useful, but
// WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE. See the GNU
// General Public License for more details.
//
// You should have received a copy of the GNU General Public License
// along with this program. If not, see <http://www.gnu.org/licenses/>.
//
// Copyright (C) 2022-2022 Fuwn <contact@fuwn.me>
// SPDX-License-Identifier: GPL-3.0-only

use crate::{pitch::Pitch, unit::Octave};

/// A note which only keeps track of its frequency, octave, and name; enables
/// for a more flexible note system; including conversions, enharmonics, and
/// more.
///
/// # Examples
///
/// ```rust
/// let _ = guitar::Note::new("A", 4); 
/// ```
#[derive(Debug, Clone, PartialEq, PartialOrd)]
pub struct Note {
  pitch: Pitch,
}

impl Note {
  /// Creates a new, specific [`Note`] given a note's name, octave, and
  /// duration.
  ///
  /// # Examples
  ///
  /// ```rust
  /// let _ = guitar::Note::new("A", 4); 
  /// ```
  #[must_use]
  pub fn new(pitch: &str, octave: Octave) -> Self {
    let pitch = Pitch::new(pitch, octave);

    Self::new_from_pitch(pitch)
  }

  /// Creates a new, specific [`Note`] given a frequency and a [`Pitch`].
  ///
  /// This is an internal command which is made obsolete by the other,
  /// public-acing `new_from_*` methods.
  #[must_use]
  pub const fn new_from_pitch(pitch: Pitch) -> Self { Self { pitch } }

  /// Returns the [`Pitch`] of the [`Note`] for further manipulation.
  pub fn pitch(&mut self) -> &Pitch { &self.pitch }

  /// Returns the [`Pitch`] of the [`Note`] for further manipulation.
  pub fn pitch_mut(&mut self) -> &mut Pitch { &mut self.pitch }
}
