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

use crate::{unit::Frequency, Pitch};

#[derive(Debug, Clone, PartialEq, PartialOrd)]
pub struct String {
  pitch: Pitch,
  frets: Vec<Pitch>,
  fret_count: usize,
  base_frequency: Frequency,
}

impl String {
  #[must_use]
  pub fn new(pitch: Pitch, fret_count: usize) -> Self {
    let mut frets = vec![];
    let mut next_pitch = pitch.clone();

    for _ in 0..fret_count {
      let semitones = next_pitch.semitones();

      frets.push(next_pitch.clone());
      next_pitch.set_semitones(semitones + 1);
    }

    Self {
      pitch,
      frets,
      fret_count,
      base_frequency: 440.,
    }
  }

  #[must_use]
  pub const fn pitch(&self) -> &Pitch { &self.pitch }

  #[must_use]
  pub const fn frets(&self) -> &Vec<Pitch> { &self.frets }

  #[must_use]
  pub const fn fret_count(&self) -> &usize { &self.fret_count }

  #[must_use]
  pub const fn base_frequency(&self) -> &Frequency { &self.base_frequency }

  pub fn set_pitch(&mut self, pitch: Pitch) {
    let mut next_pitch = pitch.clone();

    for fret in &mut self.frets {
      let semitones = next_pitch.semitones();

      *fret = next_pitch.clone();
      next_pitch.set_semitones(semitones + 1);
    }

    self.pitch = pitch;
  }

  pub fn set_fret_count(&mut self, fret_count: usize) {
    let mut frets = vec![];
    let mut next_pitch = self.pitch.clone();

    for _ in 0..fret_count {
      let semitones = next_pitch.semitones();

      frets.push(next_pitch.clone());
      next_pitch.set_semitones(semitones + 1);
    }

    self.frets = frets;
    self.fret_count = fret_count;
  }

  pub fn set_base_frequency(&mut self, base_frequency: Frequency) {
    self.base_frequency = base_frequency;

    for fret in &mut self.frets {
      fret.set_base_frequency(base_frequency);
    }
  }
}
