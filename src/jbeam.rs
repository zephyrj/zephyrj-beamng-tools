/*
 * Copyright (c):
 * 2025 zephyrj
 * zephyrj@protonmail.com
 *
 * This file is part of zephyrj-beamng-tools.
 *
 * zephyrj-beamng-tools is free software: you can redistribute it and/or modify
 * it under the terms of the GNU General Public License as published by
 * the Free Software Foundation, either version 3 of the License, or
 * (at your option) any later version.
 *
 * zephyrj-beamng-tools is distributed in the hope that it will be useful,
 * but WITHOUT ANY WARRANTY; without even the implied warranty of
 * MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
 * GNU General Public License for more details.
 *
 * You should have received a copy of the GNU General Public License
 * along with zephyrj-beamng-tools. If not, see <https://www.gnu.org/licenses/>.
 */

use serde_hjson::{Map,Value};

pub fn from_slice(slice: &[u8]) -> serde_hjson::Result<Map<String, Value>> {
    serde_hjson::from_slice(slice)
}