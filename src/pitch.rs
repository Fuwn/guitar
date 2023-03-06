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

use crate::{
  convert,
  convert::{frequency_to_pitch, semitones_to_frequency},
  unit::{Cent, Frequency, MidiNote, Octave, Semitone},
  utility::capitalize_first_letter,
};

/// A structure which represents the [`Pitch`] of a [`Note`].
///
/// # Examples
///
/// ```rust
/// let _ = guitar::Pitch::new("C", 4); 
/// ```
#[derive(Debug, Clone, PartialEq, PartialOrd)]
pub struct Pitch {
  pitch: String,
  frequency: Frequency,
  base_frequency: Frequency,
  base_midi_note: MidiNote,
  octave: Octave,
}

impl Pitch {
  /// Creates a new [`Pitch`] from the name of a [`Pitch`].
  ///
  /// # Examples
  ///
  /// ```rust
  /// let _ = guitar::Pitch::new("C", 4); 
  /// ```
  #[must_use]
  pub fn new(pitch: &str, octave: Octave) -> Self {
    let mut pitch = capitalize_first_letter(pitch);

    if pitch.ends_with('b') {
      if let Some(enharmonic) = convert::enharmonic(&pitch) {
        pitch = enharmonic;
      }
    }

    Self {
      pitch: pitch.to_string(),
      frequency: convert::pitch_and_octave_to_frequency(
        &pitch, octave, 440., 69,
      ),
      base_frequency: 440.,
      base_midi_note: 69,
      octave,
    }
  }

  /// Creates a new [`Pitch`] given all parameters.
  ///
  /// Be careful when using this function, as it doesn't check if the given
  /// [`Pitch`] is valid.
  #[must_use]
  pub fn new_from_complete(
    pitch: &str,
    frequency: Frequency,
    base_frequency: Frequency,
    base_midi_note: MidiNote,
    octave: Octave,
  ) -> Self {
    Self {
      pitch: pitch.to_string(),
      frequency,
      base_frequency,
      base_midi_note,
      octave,
    }
  }

  /// Creates a new [`Pitch`] with
  #[must_use]
  pub fn new_from_builder(
    pitch: Option<&str>,
    frequency: Option<Frequency>,
    base_frequency: Frequency,
    base_midi_note: MidiNote,
    octave: Option<Octave>,
  ) -> Self {
    if let (Some(pitch), Some(_), Some(octave)) = (pitch, frequency, octave) {
      Self::new(pitch, octave)
    } else if let (Some(pitch), Some(octave)) = (pitch, octave) {
      Self {
        pitch: pitch.to_string(),
        frequency: convert::pitch_and_octave_to_frequency(
          pitch,
          octave,
          base_frequency,
          base_midi_note,
        ),
        base_frequency,
        base_midi_note,
        octave,
      }
    } else if let Some(frequency) = frequency {
      frequency_to_pitch(frequency, base_frequency, base_midi_note)
    } else {
      Self::new("A", 4)
    }
  }

  /// Creates a new, specific [`Note`] given the note's frequency and
  /// duration.
  /// # Examples
  ///
  /// ```rust
  /// let _ = guitar::Pitch::new_from_frequency(493.88); 
  /// ```
  #[must_use]
  pub fn new_from_frequency(frequency: Frequency) -> Self {
    frequency_to_pitch(frequency, 440., 69)
  }

  /// Creates a new [`Pitch`] from the number of [`Semitone`]s above the
  /// default base frequency of 440 Hz.
  ///
  /// # Examples
  ///
  /// ```rust
  /// let _ = guitar::Pitch::new_from_semitones(0); 
  /// ```
  #[must_use]
  pub fn new_from_semitones(semitones: Semitone) -> Self {
    let frequency = semitones_to_frequency(semitones, 440.);

    frequency_to_pitch(frequency, 440., 69)
  }

  /// Returns the pitch of the [`Pitch`].
  #[must_use]
  pub fn pitch(&self) -> &str { &self.pitch }

  /// Sets the name of the [`Pitch`].
  pub fn set_pitch(&mut self, pitch: &str) {
    self.pitch = capitalize_first_letter(pitch);
    self.frequency = convert::pitch_and_octave_to_frequency(
      &self.pitch,
      self.octave,
      self.base_frequency,
      self.base_midi_note,
    );
  }

  /// Returns the [`Frequency`] of the [`Note`].
  #[must_use]
  pub const fn frequency(&self) -> Frequency { self.frequency }

  /// Sets the [`Note`]'s [`Frequency`].
  pub fn set_frequency(&mut self, frequency: Frequency) {
    self.frequency = frequency;
    self.octave = convert::frequency_to_octave(frequency, self.base_frequency);
    self.pitch =
      frequency_to_pitch(frequency, self.base_frequency, self.base_midi_note)
        .pitch()
        .to_string();
  }

  /// Returns the [`Note`]'s base [`Frequency`].
  #[must_use]
  pub const fn base_frequency(&self) -> Frequency { self.base_frequency }

  /// Sets the [`Note`]'s base [`Frequency`].
  pub fn set_base_frequency(&mut self, frequency: Frequency) {
    self.base_frequency = frequency;
    self.base_midi_note =
      convert::frequency_to_midi_note(frequency, self.base_frequency);
    self.frequency = semitones_to_frequency(self.semitones(), frequency);
  }

  /// Returns the [`Note`]'s base MIDI note.
  #[must_use]
  pub const fn base_midi_note(&self) -> MidiNote { self.base_midi_note }

  /// Sets the [`Note`]'s base MIDI note.
  pub fn set_base_midi_note(&mut self, midi_note: MidiNote) {
    self.base_midi_note = midi_note;
    self.base_frequency = convert::midi_note_to_frequency(
      midi_note,
      self.base_frequency,
      self.base_midi_note,
    );
    self.frequency =
      semitones_to_frequency(self.semitones(), self.base_frequency);
  }

  /// Returns the [`Octave`] of the [`Note`].
  #[must_use]
  pub const fn octave(&self) -> Octave { self.octave }

  /// Sets the [`Note`]'s [`Octave`].
  pub fn set_octave(&mut self, octave: Octave) {
    self.octave = octave;
    self.frequency = convert::pitch_and_octave_to_frequency(
      &self.pitch,
      self.octave,
      self.base_frequency,
      self.base_midi_note,
    );
  }

  /// Returns the [`Note`]'s enharmonic pair, if it exists.
  #[must_use]
  pub fn enharmonic(&self) -> Option<Self> {
    let enharmonic = convert::enharmonic(&self.pitch);

    enharmonic.map(|enharmonic| {
      Self::new_from_complete(
        &enharmonic,
        self.frequency,
        self.base_frequency,
        self.base_midi_note,
        self.octave,
      )
    })
  }

  /// Returns the number of [`Semitone`]s that the [`Note`] is away from the
  /// base frequency.
  #[must_use]
  pub fn semitones(&self) -> Semitone {
    convert::frequency_to_semitones(self.frequency, self.base_frequency)
  }

  /// Sets the number of [`Semitone`]s that the [`Note`]'s is away from the base
  /// frequency.
  pub fn set_semitones(&mut self, semitones: Semitone) {
    self.frequency = semitones_to_frequency(semitones, self.base_frequency);
    self.octave =
      convert::frequency_to_octave(self.frequency, self.base_frequency);
    self.pitch = frequency_to_pitch(
      self.frequency,
      self.base_frequency,
      self.base_midi_note,
    )
    .pitch()
    .to_string();
  }

  /// Returns the [`Note`]'s representation as a MIDI note.
  #[must_use]
  pub fn midi_note(&self) -> MidiNote {
    convert::frequency_to_midi_note(self.frequency, self.base_frequency)
  }

  /// Returns the [`Note`]'s representation in [`Cent`]s.
  #[must_use]
  pub fn cents(&self) -> Cent {
    convert::semitones_to_cents(self.semitones()) as Cent
  }
}
