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

use crate::{string::String, unit::Frets, Pitch};

pub struct Fretboard {
  strings: Vec<String>,
  frets: Frets,
}

impl Fretboard {
  /// Create a new [`Fretboard`] from a [`Vec`] of [`String`]s and a number of
  /// [`Frets`].
  #[must_use]
  pub fn new_from_strings(strings: Vec<String>, frets: Frets) -> Self {
    Self { strings, frets }
  }

  /// Create a new [`Fretboard`] from a number of [`Frets`], using standard
  /// tuning for a six-string guitar (E2, A2, D3, G3, B3, E4).
  #[must_use]
  pub fn new(frets: Frets) -> Self {
    Self {
      strings: vec![
        String::new(Pitch::new("E", 2), frets),
        String::new(Pitch::new("A", 2), frets),
        String::new(Pitch::new("D", 3), frets),
        String::new(Pitch::new("G", 3), frets),
        String::new(Pitch::new("B", 3), frets),
        String::new(Pitch::new("E", 4), frets),
      ],
      frets,
    }
  }

  /// Return the [`Vec`] of [`String`]s.
  #[must_use]
  pub const fn strings(&self) -> &Vec<String> { &self.strings }

  /// Return the number of [`Frets`].
  #[must_use]
  pub const fn frets(&self) -> &Frets { &self.frets }
}
