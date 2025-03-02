// Copyright (C) 2019-2023 Aleo Systems Inc.
// This file is part of the Leo library.

// The Leo library is free software: you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.

// The Leo library is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE. See the
// GNU General Public License for more details.

// You should have received a copy of the GNU General Public License
// along with the Leo library. If not, see <https://www.gnu.org/licenses/>.

#![forbid(unsafe_code)]
#![deny(clippy::all, clippy::missing_docs_in_private_items)]
#![doc = include_str!("../README.md")]

#[macro_use]
extern crate thiserror;

/// Contains the common functionalities for defining errors..
#[macro_use]
pub mod common;
pub use self::common::*;

/// Contains traits and types for channels through which errors go.
pub mod emitter;
/// Contains the errors for the Leo lang.
pub mod errors;
pub use self::errors::*;

/// Contains the warnings for the Leo lang.
pub mod warnings;
pub use self::warnings::*;
