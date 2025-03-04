// This file is part of Gear.

// Copyright (C) 2021 Gear Technologies Inc.
// SPDX-License-Identifier: GPL-3.0-or-later WITH Classpath-exception-2.0

// This program is free software: you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.

// This program is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE. See the
// GNU General Public License for more details.

// You should have received a copy of the GNU General Public License
// along with this program. If not, see <https://www.gnu.org/licenses/>.

//! Crate provides support for wasm runtime.

#![no_std]
#![warn(missing_docs)]

#[macro_use]
extern crate alloc;

cfg_if::cfg_if! {
    if #[cfg(feature = "wasmtime_backend")] {
        pub mod wasmtime;
        pub use crate::wasmtime::env::Environment;
    } else if #[cfg(feature = "wasmi_backend")] {
        pub mod wasmi;
        pub use crate::wasmi::env::Environment;
    }
}

mod funcs;
