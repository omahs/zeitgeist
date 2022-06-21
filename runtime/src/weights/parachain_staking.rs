//! Autogenerated weights for parachain_staking
//!
//! THIS FILE WAS AUTO-GENERATED USING THE SUBSTRATE BENCHMARK CLI VERSION 4.0.0-dev
//! DATE: 2022-06-16, STEPS: `50`, REPEAT: 20, LOW RANGE: `[]`, HIGH RANGE: `[]`
//! EXECUTION: Some(Wasm), WASM-EXECUTION: Compiled, CHAIN: Some("dev"), DB CACHE: 1024

// Executed Command:
// ./target/release/zeitgeist
// benchmark
// pallet
// --chain=dev
// --steps=50
// --repeat=20
// --pallet=parachain_staking
// --extrinsic=*
// --execution=wasm
// --wasm-execution=compiled
// --heap-pages=4096
// --template=./misc/frame_weight_template.hbs
// --output=./runtime/src/weights/

#![allow(unused_parens)]
#![allow(unused_imports)]

use core::marker::PhantomData;
use frame_support::{
    traits::Get,
    weights::{constants::RocksDbWeight, Weight},
};

/// Weight functions for parachain_staking (automatically generated)
pub struct WeightInfo<T>(PhantomData<T>);
impl<T: frame_system::Config> parachain_staking::weights::WeightInfo for WeightInfo<T> {
    // Storage: ParachainStaking DelegatorState (r:1 w:1)
    fn hotfix_remove_delegation_requests(x: u32) -> Weight {
        (0 as Weight)
            // Standard Error: 62_000
            .saturating_add((13_647_000 as Weight).saturating_mul(x as Weight))
            .saturating_add(T::DbWeight::get().reads((1 as Weight).saturating_mul(x as Weight)))
            .saturating_add(T::DbWeight::get().writes((1 as Weight).saturating_mul(x as Weight)))
    }
    // Storage: ParachainStaking CandidateInfo (r:4 w:0)
    // Storage: ParachainStaking CandidatePool (r:1 w:1)
    fn hotfix_update_candidate_pool_value(x: u32) -> Weight {
        (0 as Weight)
            // Standard Error: 169_000
            .saturating_add((27_624_000 as Weight).saturating_mul(x as Weight))
            .saturating_add(T::DbWeight::get().reads((1 as Weight).saturating_mul(x as Weight)))
            .saturating_add(T::DbWeight::get().writes(1 as Weight))
    }
    // Storage: ParachainStaking InflationConfig (r:1 w:1)
    fn set_staking_expectations() -> Weight {
        (38_860_000 as Weight)
            .saturating_add(T::DbWeight::get().reads(1 as Weight))
            .saturating_add(T::DbWeight::get().writes(1 as Weight))
    }
    // Storage: ParachainStaking InflationConfig (r:1 w:1)
    // Storage: ParachainStaking Round (r:1 w:0)
    fn set_inflation() -> Weight {
        (89_560_000 as Weight)
            .saturating_add(T::DbWeight::get().reads(2 as Weight))
            .saturating_add(T::DbWeight::get().writes(1 as Weight))
    }
    // Storage: ParachainStaking ParachainBondInfo (r:1 w:1)
    fn set_parachain_bond_account() -> Weight {
        (31_010_000 as Weight)
            .saturating_add(T::DbWeight::get().reads(1 as Weight))
            .saturating_add(T::DbWeight::get().writes(1 as Weight))
    }
    // Storage: ParachainStaking ParachainBondInfo (r:1 w:1)
    fn set_parachain_bond_reserve_percent() -> Weight {
        (31_360_000 as Weight)
            .saturating_add(T::DbWeight::get().reads(1 as Weight))
            .saturating_add(T::DbWeight::get().writes(1 as Weight))
    }
    // Storage: ParachainStaking TotalSelected (r:1 w:1)
    // Storage: ParachainStaking Round (r:1 w:0)
    fn set_total_selected() -> Weight {
        (38_220_000 as Weight)
            .saturating_add(T::DbWeight::get().reads(2 as Weight))
            .saturating_add(T::DbWeight::get().writes(1 as Weight))
    }
    // Storage: ParachainStaking CollatorCommission (r:1 w:1)
    fn set_collator_commission() -> Weight {
        (29_550_000 as Weight)
            .saturating_add(T::DbWeight::get().reads(1 as Weight))
            .saturating_add(T::DbWeight::get().writes(1 as Weight))
    }
    // Storage: ParachainStaking Round (r:1 w:1)
    // Storage: ParachainStaking TotalSelected (r:1 w:0)
    // Storage: ParachainStaking InflationConfig (r:1 w:1)
    fn set_blocks_per_round() -> Weight {
        (97_210_000 as Weight)
            .saturating_add(T::DbWeight::get().reads(3 as Weight))
            .saturating_add(T::DbWeight::get().writes(2 as Weight))
    }
    // Storage: ParachainStaking CandidateInfo (r:1 w:1)
    // Storage: ParachainStaking DelegatorState (r:1 w:0)
    // Storage: ParachainStaking CandidatePool (r:1 w:1)
    // Storage: System Account (r:1 w:1)
    // Storage: ParachainStaking Total (r:1 w:1)
    // Storage: ParachainStaking TopDelegations (r:0 w:1)
    // Storage: ParachainStaking BottomDelegations (r:0 w:1)
    fn join_candidates(x: u32) -> Weight {
        (137_741_000 as Weight)
            // Standard Error: 3_000
            .saturating_add((219_000 as Weight).saturating_mul(x as Weight))
            .saturating_add(T::DbWeight::get().reads(5 as Weight))
            .saturating_add(T::DbWeight::get().writes(6 as Weight))
    }
    // Storage: ParachainStaking CandidateInfo (r:1 w:1)
    // Storage: ParachainStaking Round (r:1 w:0)
    // Storage: ParachainStaking CandidatePool (r:1 w:1)
    fn schedule_leave_candidates(x: u32) -> Weight {
        (126_459_000 as Weight)
            // Standard Error: 4_000
            .saturating_add((143_000 as Weight).saturating_mul(x as Weight))
            .saturating_add(T::DbWeight::get().reads(3 as Weight))
            .saturating_add(T::DbWeight::get().writes(2 as Weight))
    }
    // Storage: ParachainStaking CandidateInfo (r:1 w:1)
    // Storage: ParachainStaking Round (r:1 w:0)
    // Storage: ParachainStaking TopDelegations (r:1 w:1)
    // Storage: System Account (r:2 w:2)
    // Storage: ParachainStaking DelegatorState (r:1 w:1)
    // Storage: ParachainStaking BottomDelegations (r:1 w:1)
    // Storage: ParachainStaking Total (r:1 w:1)
    fn execute_leave_candidates(x: u32) -> Weight {
        (41_028_000 as Weight)
            // Standard Error: 179_000
            .saturating_add((41_359_000 as Weight).saturating_mul(x as Weight))
            .saturating_add(T::DbWeight::get().reads(4 as Weight))
            .saturating_add(T::DbWeight::get().reads((2 as Weight).saturating_mul(x as Weight)))
            .saturating_add(T::DbWeight::get().writes(3 as Weight))
            .saturating_add(T::DbWeight::get().writes((2 as Weight).saturating_mul(x as Weight)))
    }
    // Storage: ParachainStaking CandidateInfo (r:1 w:1)
    // Storage: ParachainStaking CandidatePool (r:1 w:1)
    fn cancel_leave_candidates(x: u32) -> Weight {
        (108_141_000 as Weight)
            // Standard Error: 3_000
            .saturating_add((163_000 as Weight).saturating_mul(x as Weight))
            .saturating_add(T::DbWeight::get().reads(2 as Weight))
            .saturating_add(T::DbWeight::get().writes(2 as Weight))
    }
    // Storage: ParachainStaking CandidateInfo (r:1 w:1)
    // Storage: ParachainStaking CandidatePool (r:1 w:1)
    fn go_offline() -> Weight {
        (47_010_000 as Weight)
            .saturating_add(T::DbWeight::get().reads(2 as Weight))
            .saturating_add(T::DbWeight::get().writes(2 as Weight))
    }
    // Storage: ParachainStaking CandidateInfo (r:1 w:1)
    // Storage: ParachainStaking CandidatePool (r:1 w:1)
    fn go_online() -> Weight {
        (56_840_000 as Weight)
            .saturating_add(T::DbWeight::get().reads(2 as Weight))
            .saturating_add(T::DbWeight::get().writes(2 as Weight))
    }
    // Storage: ParachainStaking CandidateInfo (r:1 w:1)
    // Storage: System Account (r:1 w:1)
    // Storage: ParachainStaking Total (r:1 w:1)
    // Storage: ParachainStaking CandidatePool (r:1 w:1)
    fn candidate_bond_more() -> Weight {
        (97_210_000 as Weight)
            .saturating_add(T::DbWeight::get().reads(4 as Weight))
            .saturating_add(T::DbWeight::get().writes(4 as Weight))
    }
    // Storage: ParachainStaking CandidateInfo (r:1 w:1)
    // Storage: ParachainStaking Round (r:1 w:0)
    fn schedule_candidate_bond_less() -> Weight {
        (53_270_000 as Weight)
            .saturating_add(T::DbWeight::get().reads(2 as Weight))
            .saturating_add(T::DbWeight::get().writes(1 as Weight))
    }
    // Storage: ParachainStaking CandidateInfo (r:1 w:1)
    // Storage: ParachainStaking Round (r:1 w:0)
    // Storage: System Account (r:1 w:1)
    // Storage: ParachainStaking Total (r:1 w:1)
    // Storage: ParachainStaking CandidatePool (r:1 w:1)
    fn execute_candidate_bond_less() -> Weight {
        (99_890_000 as Weight)
            .saturating_add(T::DbWeight::get().reads(5 as Weight))
            .saturating_add(T::DbWeight::get().writes(4 as Weight))
    }
    // Storage: ParachainStaking CandidateInfo (r:1 w:1)
    fn cancel_candidate_bond_less() -> Weight {
        (42_860_000 as Weight)
            .saturating_add(T::DbWeight::get().reads(1 as Weight))
            .saturating_add(T::DbWeight::get().writes(1 as Weight))
    }
    // Storage: System Account (r:1 w:1)
    // Storage: ParachainStaking DelegatorState (r:1 w:1)
    // Storage: ParachainStaking CandidateInfo (r:1 w:1)
    // Storage: ParachainStaking TopDelegations (r:1 w:1)
    // Storage: ParachainStaking CandidatePool (r:1 w:1)
    // Storage: ParachainStaking Total (r:1 w:1)
    fn delegate(x: u32, y: u32) -> Weight {
        (201_147_000 as Weight)
            // Standard Error: 23_000
            .saturating_add((97_000 as Weight).saturating_mul(x as Weight))
            // Standard Error: 8_000
            .saturating_add((357_000 as Weight).saturating_mul(y as Weight))
            .saturating_add(T::DbWeight::get().reads(6 as Weight))
            .saturating_add(T::DbWeight::get().writes(6 as Weight))
    }
    // Storage: ParachainStaking DelegatorState (r:1 w:1)
    // Storage: ParachainStaking Round (r:1 w:0)
    fn schedule_leave_delegators() -> Weight {
        (47_660_000 as Weight)
            .saturating_add(T::DbWeight::get().reads(2 as Weight))
            .saturating_add(T::DbWeight::get().writes(1 as Weight))
    }
    // Storage: ParachainStaking DelegatorState (r:1 w:1)
    // Storage: ParachainStaking Round (r:1 w:0)
    // Storage: ParachainStaking CandidateInfo (r:1 w:1)
    // Storage: ParachainStaking TopDelegations (r:1 w:1)
    // Storage: ParachainStaking CandidatePool (r:1 w:1)
    // Storage: System Account (r:1 w:1)
    // Storage: ParachainStaking Total (r:1 w:1)
    fn execute_leave_delegators(x: u32) -> Weight {
        (0 as Weight)
            // Standard Error: 173_000
            .saturating_add((64_007_000 as Weight).saturating_mul(x as Weight))
            .saturating_add(T::DbWeight::get().reads(3 as Weight))
            .saturating_add(T::DbWeight::get().reads((2 as Weight).saturating_mul(x as Weight)))
            .saturating_add(T::DbWeight::get().writes(2 as Weight))
            .saturating_add(T::DbWeight::get().writes((2 as Weight).saturating_mul(x as Weight)))
    }
    // Storage: ParachainStaking DelegatorState (r:1 w:1)
    fn cancel_leave_delegators() -> Weight {
        (40_910_000 as Weight)
            .saturating_add(T::DbWeight::get().reads(1 as Weight))
            .saturating_add(T::DbWeight::get().writes(1 as Weight))
    }
    // Storage: ParachainStaking DelegatorState (r:1 w:1)
    // Storage: ParachainStaking Round (r:1 w:0)
    fn schedule_revoke_delegation() -> Weight {
        (47_370_000 as Weight)
            .saturating_add(T::DbWeight::get().reads(2 as Weight))
            .saturating_add(T::DbWeight::get().writes(1 as Weight))
    }
    // Storage: ParachainStaking DelegatorState (r:1 w:1)
    // Storage: ParachainStaking CandidateInfo (r:1 w:1)
    // Storage: System Account (r:1 w:1)
    // Storage: ParachainStaking TopDelegations (r:1 w:1)
    // Storage: ParachainStaking CandidatePool (r:1 w:1)
    // Storage: ParachainStaking Total (r:1 w:1)
    fn delegator_bond_more() -> Weight {
        (107_080_000 as Weight)
            .saturating_add(T::DbWeight::get().reads(6 as Weight))
            .saturating_add(T::DbWeight::get().writes(6 as Weight))
    }
    // Storage: ParachainStaking DelegatorState (r:1 w:1)
    // Storage: ParachainStaking Round (r:1 w:0)
    fn schedule_delegator_bond_less() -> Weight {
        (48_160_000 as Weight)
            .saturating_add(T::DbWeight::get().reads(2 as Weight))
            .saturating_add(T::DbWeight::get().writes(1 as Weight))
    }
    // Storage: ParachainStaking DelegatorState (r:1 w:1)
    // Storage: ParachainStaking Round (r:1 w:0)
    // Storage: ParachainStaking CandidateInfo (r:1 w:1)
    // Storage: ParachainStaking TopDelegations (r:1 w:1)
    // Storage: ParachainStaking CandidatePool (r:1 w:1)
    // Storage: System Account (r:1 w:1)
    // Storage: ParachainStaking Total (r:1 w:1)
    fn execute_revoke_delegation() -> Weight {
        (136_580_000 as Weight)
            .saturating_add(T::DbWeight::get().reads(7 as Weight))
            .saturating_add(T::DbWeight::get().writes(6 as Weight))
    }
    // Storage: ParachainStaking DelegatorState (r:1 w:1)
    // Storage: ParachainStaking Round (r:1 w:0)
    // Storage: ParachainStaking CandidateInfo (r:1 w:1)
    // Storage: System Account (r:1 w:1)
    // Storage: ParachainStaking TopDelegations (r:1 w:1)
    // Storage: ParachainStaking CandidatePool (r:1 w:1)
    // Storage: ParachainStaking Total (r:1 w:1)
    fn execute_delegator_bond_less() -> Weight {
        (121_700_000 as Weight)
            .saturating_add(T::DbWeight::get().reads(7 as Weight))
            .saturating_add(T::DbWeight::get().writes(6 as Weight))
    }
    // Storage: ParachainStaking DelegatorState (r:1 w:1)
    fn cancel_revoke_delegation() -> Weight {
        (42_140_000 as Weight)
            .saturating_add(T::DbWeight::get().reads(1 as Weight))
            .saturating_add(T::DbWeight::get().writes(1 as Weight))
    }
    // Storage: ParachainStaking DelegatorState (r:1 w:1)
    fn cancel_delegator_bond_less() -> Weight {
        (49_570_000 as Weight)
            .saturating_add(T::DbWeight::get().reads(1 as Weight))
            .saturating_add(T::DbWeight::get().writes(1 as Weight))
    }
    // Storage: ParachainStaking Round (r:1 w:1)
    // Storage: ParachainStaking Points (r:1 w:0)
    // Storage: ParachainStaking Staked (r:1 w:2)
    // Storage: ParachainStaking InflationConfig (r:1 w:0)
    // Storage: ParachainStaking ParachainBondInfo (r:1 w:0)
    // Storage: System Account (r:302 w:301)
    // Storage: ParachainStaking CollatorCommission (r:1 w:0)
    // Storage: ParachainStaking CandidatePool (r:1 w:0)
    // Storage: ParachainStaking TotalSelected (r:1 w:0)
    // Storage: ParachainStaking CandidateInfo (r:9 w:0)
    // Storage: ParachainStaking TopDelegations (r:9 w:0)
    // Storage: ParachainStaking Total (r:1 w:0)
    // Storage: ParachainStaking AwardedPts (r:2 w:1)
    // Storage: ParachainStaking AtStake (r:1 w:10)
    // Storage: ParachainStaking SelectedCandidates (r:0 w:1)
    // Storage: ParachainStaking DelayedPayouts (r:0 w:1)
    fn round_transition_on_initialize(x: u32, y: u32) -> Weight {
        (0 as Weight)
            // Standard Error: 2_055_000
            .saturating_add((73_724_000 as Weight).saturating_mul(x as Weight))
            // Standard Error: 7_000
            .saturating_add((611_000 as Weight).saturating_mul(y as Weight))
            .saturating_add(T::DbWeight::get().reads((1 as Weight).saturating_mul(x as Weight)))
    }
    // Storage: ParachainStaking DelayedPayouts (r:1 w:0)
    // Storage: ParachainStaking Points (r:1 w:0)
    // Storage: ParachainStaking AwardedPts (r:2 w:1)
    // Storage: ParachainStaking AtStake (r:1 w:1)
    // Storage: System Account (r:1 w:1)
    fn pay_one_collator_reward(y: u32) -> Weight {
        (61_020_000 as Weight)
            // Standard Error: 122_000
            .saturating_add((32_600_000 as Weight).saturating_mul(y as Weight))
            .saturating_add(T::DbWeight::get().reads(6 as Weight))
            .saturating_add(T::DbWeight::get().reads((1 as Weight).saturating_mul(y as Weight)))
            .saturating_add(T::DbWeight::get().writes(3 as Weight))
            .saturating_add(T::DbWeight::get().writes((1 as Weight).saturating_mul(y as Weight)))
    }
    // Storage: ParachainStaking Round (r:1 w:0)
    fn base_on_initialize() -> Weight {
        (6_840_000 as Weight).saturating_add(T::DbWeight::get().reads(1 as Weight))
    }
}
