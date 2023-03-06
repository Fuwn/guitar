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

use std::collections::HashMap;

use once_cell::sync::Lazy;

pub const NOTES: [&str; 12] = [
  "C", "C#", "D", "D#", "E", "F", "F#", "G", "G#", "A", "A#", "B",
];

pub static NOTES_OFFSET: Lazy<HashMap<String, i8>> = Lazy::new(|| {
  let mut notes = HashMap::new();
  let mut i = -9;

  for note in NOTES {
    notes.insert(note.to_string(), i);

    i += 1;
  }

  notes
});
