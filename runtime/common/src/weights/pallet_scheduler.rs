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
//! DATE: 2022-08-13, STEPS: `50`, REPEAT: 20, LOW RANGE: `[]`, HIGH RANGE: `[]`
//! EXECUTION: Some(Wasm), WASM-EXECUTION: Compiled, CHAIN: Some("dev"), DB CACHE: 1024

// Executed Command:
// ./target/release/zeitgeist
// benchmark
// pallet
// --chain=dev
// --steps=50
// --repeat=20
// --pallet=pallet_scheduler
// --extrinsic=*
// --execution=wasm
// --wasm-execution=compiled
// --heap-pages=4096
// --template=./misc/frame_weight_template.hbs
// --output=./runtime/common/src/weights/

#![allow(unused_parens)]
#![allow(unused_imports)]

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
    fn on_initialize_periodic_named_resolved(s: u32) -> Weight {
        (0 as Weight)
            // Standard Error: 288_000
            .saturating_add((57_475_000 as Weight).saturating_mul(s as Weight))
            .saturating_add(T::DbWeight::get().reads(1 as Weight))
            .saturating_add(T::DbWeight::get().reads((3 as Weight).saturating_mul(s as Weight)))
            .saturating_add(T::DbWeight::get().writes(1 as Weight))
            .saturating_add(T::DbWeight::get().writes((4 as Weight).saturating_mul(s as Weight)))
    }
    // Storage: Scheduler Agenda (r:1 w:1)
    // Storage: Preimage PreimageFor (r:1 w:1)
    // Storage: Preimage StatusFor (r:1 w:1)
    // Storage: Scheduler Lookup (r:0 w:1)
    fn on_initialize_named_resolved(s: u32) -> Weight {
        (4_463_000 as Weight)
            // Standard Error: 730_000
            .saturating_add((45_461_000 as Weight).saturating_mul(s as Weight))
            .saturating_add(T::DbWeight::get().reads(1 as Weight))
            .saturating_add(T::DbWeight::get().reads((2 as Weight).saturating_mul(s as Weight)))
            .saturating_add(T::DbWeight::get().writes(1 as Weight))
            .saturating_add(T::DbWeight::get().writes((3 as Weight).saturating_mul(s as Weight)))
    }
    // Storage: Scheduler Agenda (r:2 w:2)
    // Storage: Preimage PreimageFor (r:1 w:1)
    // Storage: Preimage StatusFor (r:1 w:1)
    fn on_initialize_periodic_resolved(s: u32) -> Weight {
        (0 as Weight)
            // Standard Error: 619_000
            .saturating_add((50_932_000 as Weight).saturating_mul(s as Weight))
            .saturating_add(T::DbWeight::get().reads(1 as Weight))
            .saturating_add(T::DbWeight::get().reads((3 as Weight).saturating_mul(s as Weight)))
            .saturating_add(T::DbWeight::get().writes(1 as Weight))
            .saturating_add(T::DbWeight::get().writes((3 as Weight).saturating_mul(s as Weight)))
    }
    // Storage: Scheduler Agenda (r:1 w:1)
    // Storage: Preimage PreimageFor (r:1 w:1)
    // Storage: Preimage StatusFor (r:1 w:1)
    fn on_initialize_resolved(s: u32) -> Weight {
        (15_110_000 as Weight)
            // Standard Error: 275_000
            .saturating_add((37_824_000 as Weight).saturating_mul(s as Weight))
            .saturating_add(T::DbWeight::get().reads(1 as Weight))
            .saturating_add(T::DbWeight::get().reads((2 as Weight).saturating_mul(s as Weight)))
            .saturating_add(T::DbWeight::get().writes(1 as Weight))
            .saturating_add(T::DbWeight::get().writes((2 as Weight).saturating_mul(s as Weight)))
    }
    // Storage: Scheduler Agenda (r:2 w:2)
    // Storage: Preimage PreimageFor (r:1 w:0)
    // Storage: Scheduler Lookup (r:0 w:1)
    fn on_initialize_named_aborted(s: u32) -> Weight {
        (16_694_000 as Weight)
            // Standard Error: 196_000
            .saturating_add((17_440_000 as Weight).saturating_mul(s as Weight))
            .saturating_add(T::DbWeight::get().reads(2 as Weight))
            .saturating_add(T::DbWeight::get().reads((1 as Weight).saturating_mul(s as Weight)))
            .saturating_add(T::DbWeight::get().writes(2 as Weight))
            .saturating_add(T::DbWeight::get().writes((1 as Weight).saturating_mul(s as Weight)))
    }
    // Storage: Scheduler Agenda (r:2 w:2)
    // Storage: Preimage PreimageFor (r:1 w:0)
    fn on_initialize_aborted(s: u32) -> Weight {
        (31_857_000 as Weight)
            // Standard Error: 61_000
            .saturating_add((9_207_000 as Weight).saturating_mul(s as Weight))
            .saturating_add(T::DbWeight::get().reads(2 as Weight))
            .saturating_add(T::DbWeight::get().reads((1 as Weight).saturating_mul(s as Weight)))
            .saturating_add(T::DbWeight::get().writes(2 as Weight))
    }
    // Storage: Scheduler Agenda (r:2 w:2)
    // Storage: Scheduler Lookup (r:0 w:1)
    fn on_initialize_periodic_named(s: u32) -> Weight {
        (39_481_000 as Weight)
            // Standard Error: 207_000
            .saturating_add((27_770_000 as Weight).saturating_mul(s as Weight))
            .saturating_add(T::DbWeight::get().reads(1 as Weight))
            .saturating_add(T::DbWeight::get().reads((1 as Weight).saturating_mul(s as Weight)))
            .saturating_add(T::DbWeight::get().writes(1 as Weight))
            .saturating_add(T::DbWeight::get().writes((2 as Weight).saturating_mul(s as Weight)))
    }
    // Storage: Scheduler Agenda (r:2 w:2)
    fn on_initialize_periodic(s: u32) -> Weight {
        (40_590_000 as Weight)
            // Standard Error: 211_000
            .saturating_add((19_415_000 as Weight).saturating_mul(s as Weight))
            .saturating_add(T::DbWeight::get().reads(1 as Weight))
            .saturating_add(T::DbWeight::get().reads((1 as Weight).saturating_mul(s as Weight)))
            .saturating_add(T::DbWeight::get().writes(1 as Weight))
            .saturating_add(T::DbWeight::get().writes((1 as Weight).saturating_mul(s as Weight)))
    }
    // Storage: Scheduler Agenda (r:1 w:1)
    // Storage: Scheduler Lookup (r:0 w:1)
    fn on_initialize_named(s: u32) -> Weight {
        (19_540_000 as Weight)
            // Standard Error: 113_000
            .saturating_add((15_785_000 as Weight).saturating_mul(s as Weight))
            .saturating_add(T::DbWeight::get().reads(1 as Weight))
            .saturating_add(T::DbWeight::get().writes(1 as Weight))
            .saturating_add(T::DbWeight::get().writes((1 as Weight).saturating_mul(s as Weight)))
    }
    // Storage: Scheduler Agenda (r:1 w:1)
    fn on_initialize(s: u32) -> Weight {
        (45_213_000 as Weight)
            // Standard Error: 171_000
            .saturating_add((11_063_000 as Weight).saturating_mul(s as Weight))
            .saturating_add(T::DbWeight::get().reads(1 as Weight))
            .saturating_add(T::DbWeight::get().writes(1 as Weight))
    }
    // Storage: Scheduler Agenda (r:1 w:1)
    fn schedule(s: u32) -> Weight {
        (39_202_000 as Weight)
            // Standard Error: 12_000
            .saturating_add((157_000 as Weight).saturating_mul(s as Weight))
            .saturating_add(T::DbWeight::get().reads(1 as Weight))
            .saturating_add(T::DbWeight::get().writes(1 as Weight))
    }
    // Storage: Scheduler Agenda (r:1 w:1)
    // Storage: Scheduler Lookup (r:0 w:1)
    fn cancel(s: u32) -> Weight {
        (40_525_000 as Weight)
            // Standard Error: 23_000
            .saturating_add((1_563_000 as Weight).saturating_mul(s as Weight))
            .saturating_add(T::DbWeight::get().reads(1 as Weight))
            .saturating_add(T::DbWeight::get().writes(2 as Weight))
    }
    // Storage: Scheduler Lookup (r:1 w:1)
    // Storage: Scheduler Agenda (r:1 w:1)
    fn schedule_named(s: u32) -> Weight {
        (47_772_000 as Weight)
            // Standard Error: 13_000
            .saturating_add((248_000 as Weight).saturating_mul(s as Weight))
            .saturating_add(T::DbWeight::get().reads(2 as Weight))
            .saturating_add(T::DbWeight::get().writes(2 as Weight))
    }
    // Storage: Scheduler Lookup (r:1 w:1)
    // Storage: Scheduler Agenda (r:1 w:1)
    fn cancel_named(s: u32) -> Weight {
        (50_379_000 as Weight)
            // Standard Error: 21_000
            .saturating_add((1_535_000 as Weight).saturating_mul(s as Weight))
            .saturating_add(T::DbWeight::get().reads(2 as Weight))
            .saturating_add(T::DbWeight::get().writes(2 as Weight))
    }
}