// Copyright 2019 Commonwealth Labs.
// This file is part of Straightedge.

// Straightedge is free software: you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.

// Straightedge is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
// GNU General Public License for more details.

// You should have received a copy of the GNU General Public License
// along with Straightedge.  If not, see <http://www.gnu.org/licenses/>.

//! A collection of node-specific RPC methods.
//!
//! Since `straightedge` core functionality makes no assumptions
//! about the modules used inside the runtime, so do
//! RPC methods defined in `straightedge-rpc` crate.
//! It means that `core/rpc` can't have any methods that
//! need some strong assumptions about the particular runtime.
//!
//! The RPCs available in this crate however can make some assumptions
//! about how the runtime is constructed and what `SRML` modules
//! are part of it. Therefore all node-runtime-specific RPCs can
//! be placed here.

#![warn(missing_docs)]

pub mod accounts;