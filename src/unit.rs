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

/// The internal data type of an octave.
pub type Octave = i64;
/// The internal data type of a frequency.
pub type Frequency = f64;
/// The internal data type of a semitone.
pub type Semitone = i64;
/// The internal data type of a non-equal semitone (e.g., non-equal
/// temperament).
pub type NonEqualSemitone = f64;
/// The internal data type of a MIDI note.
pub type MidiNote = i64;
/// The internal data type of cents.
pub type Cent = f64;
/// The internal data type of a count of frets.
pub type Frets = usize;
