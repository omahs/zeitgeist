// Copyright 2021-2022 Zeitgeist PM LLC.
//
// This file is part of Zeitgeist.
//
// Zeitgeist is free software: you can redistribute it and/or modify it
// under the terms of the GNU General Public License as published by the
// Free Software Foundation, either version 3 of the License, or (at
// your option) any later version.
//
// Zeitgeist is distributed in the hope that it will be useful, but
// WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE. See the GNU
// General Public License for more details.
//
// You should have received a copy of the GNU General Public License
// along with Zeitgeist. If not, see <https://www.gnu.org/licenses/>.

//! Autogenerated weights for frame_system
//!
//! THIS FILE WAS AUTO-GENERATED USING THE SUBSTRATE BENCHMARK CLI VERSION 4.0.0-dev
//! DATE: 2022-12-15, STEPS: `2`, REPEAT: 0, LOW RANGE: `[]`, HIGH RANGE: `[]`
//! EXECUTION: Some(Native), WASM-EXECUTION: Compiled, CHAIN: Some("dev"), DB CACHE: 1024

// Executed Command:
// ./target/release/zeitgeist
// benchmark
// pallet
// --chain=dev
// --steps=2
// --repeat=0
// --pallet=frame_system
// --extrinsic=*
// --execution=native
// --detailed-log-output
// --wasm-execution=compiled
// --heap-pages=4096
// --template=./misc/frame_weight_template.hbs
// --output=./runtime/common/src/weights/

#![allow(unused_parens)]
#![allow(unused_imports)]
#![allow(clippy::unnecessary_cast)]

use core::marker::PhantomData;
use frame_support::{
    traits::Get,
    weights::{constants::RocksDbWeight, Weight},
};

/// Weight functions for frame_system (automatically generated)
pub struct WeightInfo<T>(PhantomData<T>);
impl<T: frame_system::Config> frame_system::weights::WeightInfo for WeightInfo<T> {
    fn remark(_b: u32) -> Weight {
        Weight::from_ref_time(624_889_000 as u64)
    }
    fn remark_with_event(_b: u32) -> Weight {
        Weight::from_ref_time(4_866_219_000 as u64)
    }
    // Storage: System Digest (r:1 w:1)
    // Storage: unknown [0x3a686561707061676573] (r:0 w:1)
    fn set_heap_pages() -> Weight {
        Weight::from_ref_time(15_149_000 as u64)
            .saturating_add(T::DbWeight::get().reads(1 as u64))
            .saturating_add(T::DbWeight::get().writes(2 as u64))
    }
    // Storage: Skipped Metadata (r:0 w:0)
    fn set_storage(_i: u32) -> Weight {
        Weight::from_ref_time(765_199_000 as u64)
            .saturating_add(T::DbWeight::get().writes(1000 as u64))
    }
    // Storage: Skipped Metadata (r:0 w:0)
    fn kill_storage(_i: u32) -> Weight {
        Weight::from_ref_time(763_436_000 as u64)
            .saturating_add(T::DbWeight::get().writes(1000 as u64))
    }
    // Storage: Skipped Metadata (r:0 w:0)
    fn kill_prefix(_p: u32) -> Weight {
        Weight::from_ref_time(1_787_837_000 as u64)
            .saturating_add(T::DbWeight::get().writes(1000 as u64))
    }
}
