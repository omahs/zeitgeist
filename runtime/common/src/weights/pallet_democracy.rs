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

//! Autogenerated weights for pallet_democracy
//!
//! THIS FILE WAS AUTO-GENERATED USING THE SUBSTRATE BENCHMARK CLI VERSION 4.0.0-dev
//! DATE: 2022-11-25, STEPS: `50`, REPEAT: 20, LOW RANGE: `[]`, HIGH RANGE: `[]`
//! EXECUTION: Some(Wasm), WASM-EXECUTION: Compiled, CHAIN: Some("dev"), DB CACHE: 1024

// Executed Command:
// ./target/production/zeitgeist
// benchmark
// pallet
// --chain=dev
// --steps=50
// --repeat=20
// --pallet=pallet_democracy
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

/// Weight functions for pallet_democracy (automatically generated)
pub struct WeightInfo<T>(PhantomData<T>);
impl<T: frame_system::Config> pallet_democracy::weights::WeightInfo for WeightInfo<T> {
    // Storage: Democracy PublicPropCount (r:1 w:1)
    // Storage: Democracy PublicProps (r:1 w:1)
    // Storage: Democracy Blacklist (r:1 w:0)
    // Storage: Democracy DepositOf (r:0 w:1)
    fn propose() -> Weight {
        (99_190_000 as Weight)
            .saturating_add(T::DbWeight::get().reads(3 as Weight))
            .saturating_add(T::DbWeight::get().writes(3 as Weight))
    }
    // Storage: Democracy DepositOf (r:1 w:1)
    fn second(s: u32) -> Weight {
        (50_566_000 as Weight)
            // Standard Error: 7_000
            .saturating_add((445_000 as Weight).saturating_mul(s as Weight))
            .saturating_add(T::DbWeight::get().reads(1 as Weight))
            .saturating_add(T::DbWeight::get().writes(1 as Weight))
    }
    // Storage: Democracy ReferendumInfoOf (r:1 w:1)
    // Storage: Democracy VotingOf (r:1 w:1)
    // Storage: Balances Locks (r:1 w:1)
    fn vote_new(r: u32) -> Weight {
        (72_516_000 as Weight)
            // Standard Error: 12_000
            .saturating_add((405_000 as Weight).saturating_mul(r as Weight))
            .saturating_add(T::DbWeight::get().reads(3 as Weight))
            .saturating_add(T::DbWeight::get().writes(3 as Weight))
    }
    // Storage: Democracy ReferendumInfoOf (r:1 w:1)
    // Storage: Democracy VotingOf (r:1 w:1)
    // Storage: Balances Locks (r:1 w:1)
    fn vote_existing(r: u32) -> Weight {
        (69_938_000 as Weight)
            // Standard Error: 18_000
            .saturating_add((449_000 as Weight).saturating_mul(r as Weight))
            .saturating_add(T::DbWeight::get().reads(3 as Weight))
            .saturating_add(T::DbWeight::get().writes(3 as Weight))
    }
    // Storage: Democracy ReferendumInfoOf (r:1 w:1)
    // Storage: Democracy Cancellations (r:1 w:1)
    fn emergency_cancel() -> Weight {
        (34_740_000 as Weight)
            .saturating_add(T::DbWeight::get().reads(2 as Weight))
            .saturating_add(T::DbWeight::get().writes(2 as Weight))
    }
    // Storage: Democracy PublicProps (r:1 w:1)
    // Storage: Democracy NextExternal (r:1 w:1)
    // Storage: Democracy ReferendumInfoOf (r:1 w:1)
    // Storage: Democracy Blacklist (r:0 w:1)
    // Storage: Democracy DepositOf (r:1 w:1)
    // Storage: System Account (r:2 w:2)
    fn blacklist(p: u32) -> Weight {
        (102_527_000 as Weight)
            // Standard Error: 17_000
            .saturating_add((650_000 as Weight).saturating_mul(p as Weight))
            .saturating_add(T::DbWeight::get().reads(6 as Weight))
            .saturating_add(T::DbWeight::get().writes(7 as Weight))
    }
    // Storage: Democracy NextExternal (r:1 w:1)
    // Storage: Democracy Blacklist (r:1 w:0)
    fn external_propose(v: u32) -> Weight {
        (22_773_000 as Weight)
            // Standard Error: 1_000
            .saturating_add((47_000 as Weight).saturating_mul(v as Weight))
            .saturating_add(T::DbWeight::get().reads(2 as Weight))
            .saturating_add(T::DbWeight::get().writes(1 as Weight))
    }
    // Storage: Democracy NextExternal (r:0 w:1)
    fn external_propose_majority() -> Weight {
        (7_420_000 as Weight).saturating_add(T::DbWeight::get().writes(1 as Weight))
    }
    // Storage: Democracy NextExternal (r:0 w:1)
    fn external_propose_default() -> Weight {
        (7_880_000 as Weight).saturating_add(T::DbWeight::get().writes(1 as Weight))
    }
    // Storage: Democracy NextExternal (r:1 w:1)
    // Storage: Democracy ReferendumCount (r:1 w:1)
    // Storage: Democracy ReferendumInfoOf (r:0 w:1)
    fn fast_track() -> Weight {
        (35_750_000 as Weight)
            .saturating_add(T::DbWeight::get().reads(2 as Weight))
            .saturating_add(T::DbWeight::get().writes(3 as Weight))
    }
    // Storage: Democracy NextExternal (r:1 w:1)
    // Storage: Democracy Blacklist (r:1 w:1)
    fn veto_external(v: u32) -> Weight {
        (42_116_000 as Weight)
            // Standard Error: 4_000
            .saturating_add((46_000 as Weight).saturating_mul(v as Weight))
            .saturating_add(T::DbWeight::get().reads(2 as Weight))
            .saturating_add(T::DbWeight::get().writes(2 as Weight))
    }
    // Storage: Democracy PublicProps (r:1 w:1)
    // Storage: Democracy DepositOf (r:1 w:1)
    // Storage: System Account (r:2 w:2)
    fn cancel_proposal(p: u32) -> Weight {
        (79_688_000 as Weight)
            // Standard Error: 12_000
            .saturating_add((679_000 as Weight).saturating_mul(p as Weight))
            .saturating_add(T::DbWeight::get().reads(4 as Weight))
            .saturating_add(T::DbWeight::get().writes(4 as Weight))
    }
    // Storage: Democracy ReferendumInfoOf (r:0 w:1)
    fn cancel_referendum() -> Weight {
        (24_280_000 as Weight).saturating_add(T::DbWeight::get().writes(1 as Weight))
    }
    // Storage: Scheduler Lookup (r:1 w:1)
    // Storage: Scheduler Agenda (r:1 w:1)
    fn cancel_queued(r: u32) -> Weight {
        (39_489_000 as Weight)
            // Standard Error: 18_000
            .saturating_add((1_227_000 as Weight).saturating_mul(r as Weight))
            .saturating_add(T::DbWeight::get().reads(2 as Weight))
            .saturating_add(T::DbWeight::get().writes(2 as Weight))
    }
    // Storage: Democracy LowestUnbaked (r:1 w:1)
    // Storage: Democracy ReferendumCount (r:1 w:0)
    // Storage: Democracy ReferendumInfoOf (r:1 w:0)
    fn on_initialize_base(r: u32) -> Weight {
        (9_262_000 as Weight)
            // Standard Error: 23_000
            .saturating_add((6_409_000 as Weight).saturating_mul(r as Weight))
            .saturating_add(T::DbWeight::get().reads(2 as Weight))
            .saturating_add(T::DbWeight::get().reads((1 as Weight).saturating_mul(r as Weight)))
            .saturating_add(T::DbWeight::get().writes(1 as Weight))
    }
    // Storage: Democracy LowestUnbaked (r:1 w:1)
    // Storage: Democracy ReferendumCount (r:1 w:0)
    // Storage: Democracy LastTabledWasExternal (r:1 w:0)
    // Storage: Democracy NextExternal (r:1 w:0)
    // Storage: Democracy PublicProps (r:1 w:0)
    // Storage: Democracy ReferendumInfoOf (r:1 w:0)
    fn on_initialize_base_with_launch_period(r: u32) -> Weight {
        (12_094_000 as Weight)
            // Standard Error: 17_000
            .saturating_add((6_467_000 as Weight).saturating_mul(r as Weight))
            .saturating_add(T::DbWeight::get().reads(5 as Weight))
            .saturating_add(T::DbWeight::get().reads((1 as Weight).saturating_mul(r as Weight)))
            .saturating_add(T::DbWeight::get().writes(1 as Weight))
    }
    // Storage: Democracy VotingOf (r:3 w:3)
    // Storage: Democracy ReferendumInfoOf (r:1 w:1)
    // Storage: Balances Locks (r:1 w:1)
    fn delegate(r: u32) -> Weight {
        (63_558_000 as Weight)
            // Standard Error: 53_000
            .saturating_add((9_035_000 as Weight).saturating_mul(r as Weight))
            .saturating_add(T::DbWeight::get().reads(4 as Weight))
            .saturating_add(T::DbWeight::get().reads((1 as Weight).saturating_mul(r as Weight)))
            .saturating_add(T::DbWeight::get().writes(4 as Weight))
            .saturating_add(T::DbWeight::get().writes((1 as Weight).saturating_mul(r as Weight)))
    }
    // Storage: Democracy VotingOf (r:2 w:2)
    // Storage: Democracy ReferendumInfoOf (r:1 w:1)
    fn undelegate(r: u32) -> Weight {
        (33_478_000 as Weight)
            // Standard Error: 42_000
            .saturating_add((8_797_000 as Weight).saturating_mul(r as Weight))
            .saturating_add(T::DbWeight::get().reads(2 as Weight))
            .saturating_add(T::DbWeight::get().reads((1 as Weight).saturating_mul(r as Weight)))
            .saturating_add(T::DbWeight::get().writes(2 as Weight))
            .saturating_add(T::DbWeight::get().writes((1 as Weight).saturating_mul(r as Weight)))
    }
    // Storage: Democracy PublicProps (r:0 w:1)
    fn clear_public_proposals() -> Weight {
        (9_400_000 as Weight).saturating_add(T::DbWeight::get().writes(1 as Weight))
    }
    // Storage: Democracy Preimages (r:1 w:1)
    fn note_preimage(b: u32) -> Weight {
        (53_547_000 as Weight)
            // Standard Error: 0
            .saturating_add((4_000 as Weight).saturating_mul(b as Weight))
            .saturating_add(T::DbWeight::get().reads(1 as Weight))
            .saturating_add(T::DbWeight::get().writes(1 as Weight))
    }
    // Storage: Democracy Preimages (r:1 w:1)
    fn note_imminent_preimage(b: u32) -> Weight {
        (35_292_000 as Weight)
            // Standard Error: 0
            .saturating_add((4_000 as Weight).saturating_mul(b as Weight))
            .saturating_add(T::DbWeight::get().reads(1 as Weight))
            .saturating_add(T::DbWeight::get().writes(1 as Weight))
    }
    // Storage: Democracy Preimages (r:1 w:1)
    // Storage: System Account (r:1 w:0)
    fn reap_preimage(b: u32) -> Weight {
        (53_182_000 as Weight)
            // Standard Error: 0
            .saturating_add((2_000 as Weight).saturating_mul(b as Weight))
            .saturating_add(T::DbWeight::get().reads(2 as Weight))
            .saturating_add(T::DbWeight::get().writes(1 as Weight))
    }
    // Storage: Democracy VotingOf (r:1 w:1)
    // Storage: Balances Locks (r:1 w:1)
    // Storage: System Account (r:1 w:1)
    fn unlock_remove(r: u32) -> Weight {
        (45_241_000 as Weight)
            // Standard Error: 7_000
            .saturating_add((213_000 as Weight).saturating_mul(r as Weight))
            .saturating_add(T::DbWeight::get().reads(3 as Weight))
            .saturating_add(T::DbWeight::get().writes(3 as Weight))
    }
    // Storage: Democracy VotingOf (r:1 w:1)
    // Storage: Balances Locks (r:1 w:1)
    // Storage: System Account (r:1 w:1)
    fn unlock_set(r: u32) -> Weight {
        (44_693_000 as Weight)
            // Standard Error: 7_000
            .saturating_add((361_000 as Weight).saturating_mul(r as Weight))
            .saturating_add(T::DbWeight::get().reads(3 as Weight))
            .saturating_add(T::DbWeight::get().writes(3 as Weight))
    }
    // Storage: Democracy ReferendumInfoOf (r:1 w:1)
    // Storage: Democracy VotingOf (r:1 w:1)
    fn remove_vote(r: u32) -> Weight {
        (27_874_000 as Weight)
            // Standard Error: 6_000
            .saturating_add((371_000 as Weight).saturating_mul(r as Weight))
            .saturating_add(T::DbWeight::get().reads(2 as Weight))
            .saturating_add(T::DbWeight::get().writes(2 as Weight))
    }
    // Storage: Democracy ReferendumInfoOf (r:1 w:1)
    // Storage: Democracy VotingOf (r:1 w:1)
    fn remove_other_vote(r: u32) -> Weight {
        (29_844_000 as Weight)
            // Standard Error: 6_000
            .saturating_add((340_000 as Weight).saturating_mul(r as Weight))
            .saturating_add(T::DbWeight::get().reads(2 as Weight))
            .saturating_add(T::DbWeight::get().writes(2 as Weight))
    }
}
