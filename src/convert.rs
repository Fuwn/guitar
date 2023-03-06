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

#![allow(
  clippy::cast_possible_truncation,
  clippy::cast_precision_loss,
  clippy::cast_sign_loss
)]

use crate::{
  pitch::Pitch,
  unit::{Cent, Frequency, MidiNote, Octave, Semitone},
  utility::capitalize_first_letter,
  NOTES,
};

/// Finds the [`Frequency`] of pitch given the number of [`Semitone`]s above a
/// base [`Frequency`].
///
/// # Examples
///
/// ```rust
/// assert_eq!(
///   guitar::convert::semitones_to_frequency(1, 440.),
///   466.1637615180899,
/// );
/// ```
#[must_use]
pub fn semitones_to_frequency(
  semitones: Semitone,
  base_frequency: Frequency,
) -> Frequency {
  // https://pages.mtu.edu/~suits/NoteFreqCalcs.html
  base_frequency * (semitones as Frequency / 12.).exp2()
}

/// Find the number of [`Semitone`]s between the [`Frequency`] and a base
/// [`Frequency`].
///
/// The number of [`Semitone`]s is rounded to the nearest integer. If you want
/// to get the non-equal number of [`Semitone`]s, use
/// [`frequency_to_non_equal_semitones`].
///
/// # Examples
///
/// ```rust
/// assert_eq!(guitar::convert::frequency_to_semitones(466.16, 440.), 1);
/// ```
#[must_use]
pub fn frequency_to_semitones(
  frequency: Frequency,
  base_frequency: Frequency,
) -> Semitone {
  (12. * (frequency / base_frequency).log2()).round() as Semitone
}

/// Finds the number of [`Semitone`]s between the [`Frequency`] and a base
/// [`Frequency`].
///
/// The number of [`Semitone`]s is rounded to the nearest integer. This is
/// useful for calculating the [`Frequency`] of a pitch with a non-equal
/// temperament. If you want to get the equal number of [`Semitone`]s, use
/// [`frequency_to_semitones`].
///
/// # Examples
///
/// ```rust
/// assert_eq!(
///   guitar::convert::frequency_to_non_equal_semitones(466.1637615180899, 440.),
///   1.0000000000000009,
/// );
/// ```
#[must_use]
pub fn frequency_to_non_equal_semitones(
  frequency: Frequency,
  base_frequency: Frequency,
) -> crate::unit::NonEqualSemitone {
  12. * (frequency / base_frequency).log2()
}

/// Finds the [`Octave`] of a [`Frequency`], with respect to a base
/// [`Frequency`].
///
/// # Examples
///
/// ```rust
/// assert_eq!(
///   guitar::convert::frequency_to_octave(466.16, 440.),
///   4,
/// );
#[must_use]
pub fn frequency_to_octave(
  frequency: Frequency,
  base_frequency: Frequency,
) -> Octave {
  let midi_note = frequency_to_midi_note(frequency, base_frequency);
  let octave = ({ midi_note as f64 } / 12.).floor() - 1.;

  octave as Octave
}

/// Finds the MIDI note of a [`Frequency`], with respect to a base
/// [`Frequency`].
///
/// # Examples
///
/// ```rust
/// assert_eq!(
///   guitar::convert::frequency_to_midi_note(466.16, 440.),
///   70,
/// );
#[must_use]
pub fn frequency_to_midi_note(
  frequency: Frequency,
  base_frequency: Frequency,
) -> MidiNote {
  let a4_midi_note = 69;
  let semitones = frequency_to_semitones(frequency, base_frequency);
  let midi_note = a4_midi_note + semitones;

  midi_note as MidiNote
}

/// Find the number of [`Cent`]s of a pitch given the number of [`Semitone`]s
/// above any base [`Frequency`].
///
/// # Examples
///
/// ```rust
/// assert_eq!(guitar::convert::semitones_to_cents(1), 100.);
/// ```
#[must_use]
pub fn semitones_to_cents(semitones: Semitone) -> Cent {
  semitones as f64 * 100.
}

/// Find the number of [`Semitone`]s of a pitch given the number of [`Cent`]s.
///
/// # Examples
///
/// ```rust
/// assert_eq!(guitar::convert::cents_to_semitones(100.), 1);
/// ```
#[must_use]
pub fn cents_to_semitones(cents: Cent) -> Semitone {
  (cents / 100.) as Semitone
}

/// Finds the [`Cent`]s between two [`Frequency`]s.
///
/// # Examples
///
/// ```rust
/// assert_eq!(
///   guitar::convert::cents_between_frequencies(466.1637615180899, 440.),
///   100.00000000000007,
/// );
#[must_use]
pub fn cents_between_frequencies(
  second_frequency: Frequency,
  first_frequency: Frequency,
) -> Cent {
  1200. * (second_frequency / first_frequency).log2()
}

/// Finds the enharmonic of a pitch (as a string), if it exists.
#[must_use]
pub fn enharmonic(pitch: &str) -> Option<String> {
  let pitch = capitalize_first_letter(pitch);
  let enharmonic_note;
  let natural_note = if pitch.ends_with('b') || pitch.ends_with('#') {
    &pitch[..1]
  } else {
    return None;
  };
  // This `unwrap` *should* never fail...
  let note_index = NOTES.iter().position(|&n| n == natural_note).unwrap_or(0);
  let enharmonic_index;

  if pitch.ends_with('#') {
    enharmonic_index = (note_index + 1) % 12;

    enharmonic_note = NOTES[enharmonic_index + 1].to_string() + "b";
  } else if pitch.ends_with('b') {
    enharmonic_index = (note_index - 1) % 12;

    enharmonic_note = NOTES[enharmonic_index - 1].to_string() + "#";
  } else {
    return None;
  };

  Some(enharmonic_note)
}

/// Find the number of [`Semitone`]s above any base [`Frequency`] given a
/// pitch.
#[must_use]
pub fn pitch_and_octave_to_semitones(pitch: &str, octave: Octave) -> Semitone {
  let pitch = capitalize_first_letter(pitch);
  let pitch_offset =
    i64::from(*crate::notes::NOTES_OFFSET.get(&pitch).unwrap_or(&0));
  let semitones = pitch_offset + (octave - 4) * 12;

  semitones as Semitone
}

/// Encapsulate a pitch as a [`Pitch`] given the [`Frequency`] and a base
/// [`Frequency`].
///
/// # Examples
///
/// ```rust
/// assert_eq!(
///   guitar::convert::frequency_to_note(466.16, 440., 69).semitones(),
///   guitar::Pitch::new("A#", 4).semitones(),
/// );
#[must_use]
pub fn frequency_to_pitch(
  frequency: Frequency,
  base_frequency: Frequency,
  base_midi_note: MidiNote,
) -> Pitch {
  let semitones_above_a4 = 12. * (frequency / base_frequency).log2();
  let nearest_midi_pitch = semitones_above_a4.round();
  let midi_pitch = nearest_midi_pitch + base_midi_note as f64;
  let octave = (midi_pitch / 12.).floor() - 1.;
  let pitch = match (midi_pitch % 12.) as u64 {
    0 => "C",
    1 => "C#",
    2 => "D",
    3 => "D#",
    4 => "E",
    5 => "F",
    6 => "F#",
    7 => "G",
    8 => "G#",
    9 => "A",
    10 => "A#",
    11 => "B",
    _ => unreachable!(),
  };

  Pitch::new(pitch, octave as Octave)
}

/// Finds the [`Frequency`] of a [`MidiNote`] with respect to a base
/// [`Frequency`] and base [`MidiNote`].
#[must_use]
pub fn midi_note_to_frequency(
  midi_note: MidiNote,
  base_frequency: Frequency,
  base_midi_note: MidiNote,
) -> Frequency {
  base_frequency * ((midi_note - base_midi_note) as f64 / 12.).exp2()
}

/// Finds the [`Frequency`] of a pitch and its [`Octave`], with respect to a
/// base [`Frequency`] and base [`MidiNote`].
#[must_use]
pub fn pitch_and_octave_to_frequency(
  pitch: &str,
  octave: Octave,
  base_frequency: Frequency,
  base_midi_note: MidiNote,
) -> Frequency {
  let pitch = capitalize_first_letter(pitch);
  // This used to work, but sometime when I moved the [`Pitch`] related things
  // out of [`Note`], it stopped working. I messed around and after a few
  // seconds I realized that the now working solution, under it, works...
  // I'm not sure why, but I'm not going to question it.

  // let semitones_above_a4 =
  //   pitch_and_octave_to_semitones(pitch, octave) + (octave - 4) * 12;
  let semitones_above_a4 = pitch_and_octave_to_semitones(&pitch, octave);
  let midi_pitch = semitones_above_a4 + base_midi_note;

  midi_note_to_frequency(midi_pitch as MidiNote, base_frequency, base_midi_note)
}
