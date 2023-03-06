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

// https://pages.mtu.edu/~suits/notefreqs.html
pub const SPEED_OF_SOUND_METRES_PER_SECOND: f64 = 345.0;
pub const SPEED_OF_SOUND_FEET_PER_SECOND: f64 = 1130.33;
pub const SPEED_OF_SOUND_MILES_PER_HOUR: f64 = 770.0;

/// Capitalize the first letter of a string.
///
/// This is used internally to make sure that pitch names are capitalized.
pub fn capitalize_first_letter(string: &str) -> String {
  let mut characters = string.chars();

  characters.next().map_or_else(String::new, |f| {
    f.to_uppercase().collect::<String>() + characters.as_str()
  })
}

#[must_use]
pub fn centimeters_to_inches(centimeters: f64) -> f64 { centimeters / 2.54 }

#[must_use]
pub fn frequency_to_wavelength_cm(frequency: f64) -> f64 {
  SPEED_OF_SOUND_METRES_PER_SECOND / frequency * 100.0
}
