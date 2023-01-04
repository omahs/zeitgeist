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

//! Autogenerated weights for zrml_global_disputes
//!
//! THIS FILE WAS AUTO-GENERATED USING THE SUBSTRATE BENCHMARK CLI VERSION 4.0.0-dev
//! DATE: 2023-01-04, STEPS: `10`, REPEAT: 1000, LOW RANGE: `[]`, HIGH RANGE: `[]`
//! EXECUTION: Some(Wasm), WASM-EXECUTION: Compiled, CHAIN: Some("dev"), DB CACHE: 1024

// Executed Command:
// ./target/release/zeitgeist
// benchmark
// pallet
// --chain=dev
// --steps=10
// --repeat=1000
// --pallet=zrml_global_disputes
// --extrinsic=*
// --execution=wasm
// --wasm-execution=compiled
// --heap-pages=4096
// --output=./zrml/global-disputes/src/weights.rs
// --template=./misc/weight_template.hbs

#![allow(unused_parens)]
#![allow(unused_imports)]

use core::marker::PhantomData;
use frame_support::{traits::Get, weights::Weight};

///  Trait containing the required functions for weight retrival within
/// zrml_global_disputes (automatically generated)
pub trait WeightInfoZeitgeist {
    fn vote_on_outcome(o: u32, v: u32) -> Weight;
    fn unlock_vote_balance_set(l: u32, o: u32) -> Weight;
    fn unlock_vote_balance_remove(l: u32, o: u32) -> Weight;
    fn add_vote_outcome(w: u32) -> Weight;
    fn reward_outcome_owner_with_funds(o: u32) -> Weight;
    fn purge_outcomes(k: u32, o: u32) -> Weight;
    fn refund_vote_fees(k: u32, o: u32) -> Weight;
}

/// Weight functions for zrml_global_disputes (automatically generated)
pub struct WeightInfo<T>(PhantomData<T>);
impl<T: frame_system::Config> WeightInfoZeitgeist for WeightInfo<T> {
    // Storage: GlobalDisputes GlobalDisputesInfo (r:1 w:1)
    // Storage: GlobalDisputes Outcomes (r:1 w:1)
    // Storage: GlobalDisputes Locks (r:1 w:1)
    // Storage: Balances Locks (r:1 w:1)
    fn vote_on_outcome(o: u32, v: u32) -> Weight {
        (48_823_000 as Weight)
            // Standard Error: 4_000
            .saturating_add((309_000 as Weight).saturating_mul(o as Weight))
            // Standard Error: 0
            .saturating_add((55_000 as Weight).saturating_mul(v as Weight))
            .saturating_add(T::DbWeight::get().reads(4 as Weight))
            .saturating_add(T::DbWeight::get().writes(4 as Weight))
    }
    // Storage: GlobalDisputes Locks (r:1 w:1)
    // Storage: Balances Locks (r:1 w:1)
    // Storage: System Account (r:1 w:0)
    // Storage: GlobalDisputes GlobalDisputesInfo (r:5 w:0)
    fn unlock_vote_balance_set(l: u32, o: u32) -> Weight {
        (25_163_000 as Weight)
            // Standard Error: 2_000
            .saturating_add((4_239_000 as Weight).saturating_mul(l as Weight))
            // Standard Error: 13_000
            .saturating_add((613_000 as Weight).saturating_mul(o as Weight))
            .saturating_add(T::DbWeight::get().reads(3 as Weight))
            .saturating_add(T::DbWeight::get().reads((1 as Weight).saturating_mul(l as Weight)))
            .saturating_add(T::DbWeight::get().writes(2 as Weight))
    }
    // Storage: GlobalDisputes Locks (r:1 w:1)
    // Storage: Balances Locks (r:1 w:1)
    // Storage: System Account (r:1 w:0)
    // Storage: GlobalDisputes GlobalDisputesInfo (r:5 w:0)
    fn unlock_vote_balance_remove(l: u32, o: u32) -> Weight {
        (19_407_000 as Weight)
            // Standard Error: 1_000
            .saturating_add((4_127_000 as Weight).saturating_mul(l as Weight))
            // Standard Error: 8_000
            .saturating_add((681_000 as Weight).saturating_mul(o as Weight))
            .saturating_add(T::DbWeight::get().reads(3 as Weight))
            .saturating_add(T::DbWeight::get().reads((1 as Weight).saturating_mul(l as Weight)))
            .saturating_add(T::DbWeight::get().writes(2 as Weight))
    }
    // Storage: MarketCommons Markets (r:1 w:0)
    // Storage: GlobalDisputes GlobalDisputesInfo (r:1 w:1)
    // Storage: GlobalDisputes Outcomes (r:1 w:1)
    // Storage: System Account (r:1 w:1)
    fn add_vote_outcome(w: u32) -> Weight {
        (61_676_000 as Weight)
            // Standard Error: 5_000
            .saturating_add((68_000 as Weight).saturating_mul(w as Weight))
            .saturating_add(T::DbWeight::get().reads(4 as Weight))
            .saturating_add(T::DbWeight::get().writes(3 as Weight))
    }
    // Storage: GlobalDisputes Outcomes (r:1 w:0)
    // Storage: GlobalDisputes GlobalDisputesInfo (r:1 w:0)
    // Storage: System Account (r:2 w:2)
    fn reward_outcome_owner_with_funds(o: u32) -> Weight {
        (36_443_000 as Weight)
            // Standard Error: 24_000
            .saturating_add((24_275_000 as Weight).saturating_mul(o as Weight))
            .saturating_add(T::DbWeight::get().reads(3 as Weight))
            .saturating_add(T::DbWeight::get().reads((1 as Weight).saturating_mul(o as Weight)))
            .saturating_add(T::DbWeight::get().writes(1 as Weight))
            .saturating_add(T::DbWeight::get().writes((1 as Weight).saturating_mul(o as Weight)))
    }
    // Storage: GlobalDisputes GlobalDisputesInfo (r:1 w:1)
    // Storage: GlobalDisputes Outcomes (r:3 w:2)
    fn purge_outcomes(k: u32, _o: u32) -> Weight {
        (83_016_000 as Weight)
            // Standard Error: 4_000
            .saturating_add((13_338_000 as Weight).saturating_mul(k as Weight))
            .saturating_add(T::DbWeight::get().reads(3 as Weight))
            .saturating_add(T::DbWeight::get().reads((1 as Weight).saturating_mul(k as Weight)))
            .saturating_add(T::DbWeight::get().writes(2 as Weight))
            .saturating_add(T::DbWeight::get().writes((1 as Weight).saturating_mul(k as Weight)))
    }
    // Storage: GlobalDisputes GlobalDisputesInfo (r:1 w:0)
    // Storage: GlobalDisputes Outcomes (r:3 w:2)
    fn refund_vote_fees(k: u32, _o: u32) -> Weight {
        (31_076_000 as Weight)
            // Standard Error: 4_000
            .saturating_add((13_543_000 as Weight).saturating_mul(k as Weight))
            .saturating_add(T::DbWeight::get().reads(3 as Weight))
            .saturating_add(T::DbWeight::get().reads((1 as Weight).saturating_mul(k as Weight)))
            .saturating_add(T::DbWeight::get().writes(1 as Weight))
            .saturating_add(T::DbWeight::get().writes((1 as Weight).saturating_mul(k as Weight)))
    }
}
