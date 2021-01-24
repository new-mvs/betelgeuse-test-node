// This file is part of Betelgeuse.
//
// Copyright (C) 2018-2020 Betelgeuse Network
// SPDX-License-Identifier: GPL-3.0
//
// Betelgeuse is free software: you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.
//
// Betelgeuse is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
// GNU General Public License for more details.
//
// You should have received a copy of the GNU General Public License
// along with Betelgeuse. If not, see <https://www.gnu.org/licenses/>.

#![cfg_attr(not(feature = "std"), no_std)]

pub mod relay_authorities;
pub mod relayer_game;

// --- betelgeuse ---
pub use relay_authorities::*;
pub use relayer_game::*;
