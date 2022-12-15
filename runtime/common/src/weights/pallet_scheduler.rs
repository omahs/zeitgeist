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

//! Autogenerated weights for pallet_scheduler
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
// --pallet=pallet_scheduler
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

/// Weight functions for pallet_scheduler (automatically generated)
pub struct WeightInfo<T>(PhantomData<T>);
impl<T: frame_system::Config> pallet_scheduler::weights::WeightInfo for WeightInfo<T> {
    // Storage: Scheduler Agenda (r:2 w:2)
    // Storage: Preimage PreimageFor (r:1 w:1)
    // Storage: Preimage StatusFor (r:1 w:1)
    // Storage: Scheduler Lookup (r:0 w:1)
    fn on_initialize_periodic_named_resolved(_s: u32) -> Weight {
        Weight::from_ref_time(671_147_000 as u64)
            .saturating_add(T::DbWeight::get().reads(151 as u64))
            .saturating_add(T::DbWeight::get().writes(201 as u64))
    }
    // Storage: Scheduler Agenda (r:1 w:1)
    // Storage: Preimage PreimageFor (r:1 w:1)
    // Storage: Preimage StatusFor (r:1 w:1)
    // Storage: Scheduler Lookup (r:0 w:1)
    fn on_initialize_named_resolved(_s: u32) -> Weight {
        Weight::from_ref_time(593_999_000 as u64)
            .saturating_add(T::DbWeight::get().reads(101 as u64))
            .saturating_add(T::DbWeight::get().writes(151 as u64))
    }
    // Storage: Scheduler Agenda (r:2 w:2)
    // Storage: Preimage PreimageFor (r:1 w:1)
    // Storage: Preimage StatusFor (r:1 w:1)
    fn on_initialize_periodic_resolved(_s: u32) -> Weight {
        Weight::from_ref_time(598_978_000 as u64)
            .saturating_add(T::DbWeight::get().reads(151 as u64))
            .saturating_add(T::DbWeight::get().writes(151 as u64))
    }
    // Storage: Scheduler Agenda (r:1 w:1)
    // Storage: Preimage PreimageFor (r:1 w:1)
    // Storage: Preimage StatusFor (r:1 w:1)
    fn on_initialize_resolved(_s: u32) -> Weight {
        Weight::from_ref_time(525_627_000 as u64)
            .saturating_add(T::DbWeight::get().reads(101 as u64))
            .saturating_add(T::DbWeight::get().writes(101 as u64))
    }
    // Storage: Scheduler Agenda (r:2 w:2)
    // Storage: Preimage PreimageFor (r:1 w:0)
    // Storage: Scheduler Lookup (r:0 w:1)
    fn on_initialize_named_aborted(_s: u32) -> Weight {
        Weight::from_ref_time(156_703_000 as u64)
            .saturating_add(T::DbWeight::get().reads(52 as u64))
            .saturating_add(T::DbWeight::get().writes(52 as u64))
    }
    // Storage: Scheduler Agenda (r:2 w:2)
    // Storage: Preimage PreimageFor (r:1 w:0)
    fn on_initialize_aborted(_s: u32) -> Weight {
        Weight::from_ref_time(80_886_000 as u64)
            .saturating_add(T::DbWeight::get().reads(52 as u64))
            .saturating_add(T::DbWeight::get().writes(2 as u64))
    }
    // Storage: Scheduler Agenda (r:2 w:2)
    // Storage: Scheduler Lookup (r:0 w:1)
    fn on_initialize_periodic_named(_s: u32) -> Weight {
        Weight::from_ref_time(473_074_000 as u64)
            .saturating_add(T::DbWeight::get().reads(51 as u64))
            .saturating_add(T::DbWeight::get().writes(101 as u64))
    }
    // Storage: Scheduler Agenda (r:2 w:2)
    fn on_initialize_periodic(_s: u32) -> Weight {
        Weight::from_ref_time(358_523_000 as u64)
            .saturating_add(T::DbWeight::get().reads(51 as u64))
            .saturating_add(T::DbWeight::get().writes(51 as u64))
    }
    // Storage: Scheduler Agenda (r:1 w:1)
    // Storage: Scheduler Lookup (r:0 w:1)
    fn on_initialize_named(_s: u32) -> Weight {
        Weight::from_ref_time(185_990_000 as u64)
            .saturating_add(T::DbWeight::get().reads(1 as u64))
            .saturating_add(T::DbWeight::get().writes(51 as u64))
    }
    // Storage: Scheduler Agenda (r:1 w:1)
    fn on_initialize(_s: u32) -> Weight {
        Weight::from_ref_time(143_788_000 as u64)
            .saturating_add(T::DbWeight::get().reads(1 as u64))
            .saturating_add(T::DbWeight::get().writes(1 as u64))
    }
    // Storage: Scheduler Agenda (r:1 w:1)
    fn schedule(_s: u32) -> Weight {
        Weight::from_ref_time(49_766_000 as u64)
            .saturating_add(T::DbWeight::get().reads(1 as u64))
            .saturating_add(T::DbWeight::get().writes(1 as u64))
    }
    // Storage: Scheduler Agenda (r:1 w:1)
    // Storage: Scheduler Lookup (r:0 w:1)
    fn cancel(_s: u32) -> Weight {
        Weight::from_ref_time(43_504_000 as u64)
            .saturating_add(T::DbWeight::get().reads(1 as u64))
            .saturating_add(T::DbWeight::get().writes(2 as u64))
    }
    // Storage: Scheduler Lookup (r:1 w:1)
    // Storage: Scheduler Agenda (r:1 w:1)
    fn schedule_named(_s: u32) -> Weight {
        Weight::from_ref_time(31_280_000 as u64)
            .saturating_add(T::DbWeight::get().reads(2 as u64))
            .saturating_add(T::DbWeight::get().writes(2 as u64))
    }
    // Storage: Scheduler Lookup (r:1 w:1)
    // Storage: Scheduler Agenda (r:1 w:1)
    fn cancel_named(_s: u32) -> Weight {
        Weight::from_ref_time(46_530_000 as u64)
            .saturating_add(T::DbWeight::get().reads(2 as u64))
            .saturating_add(T::DbWeight::get().writes(2 as u64))
    }
}
