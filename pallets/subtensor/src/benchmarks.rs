//! Subtensor pallet benchmarking.
#![allow(clippy::arithmetic_side_effects, clippy::unwrap_used)]
#![cfg(feature = "runtime-benchmarks")]

use crate::Pallet as Subtensor;
use crate::*;
use codec::Compact;
use frame_benchmarking::v2::*;
use frame_support::assert_ok;
use frame_system::{RawOrigin, pallet_prelude::BlockNumberFor};
pub use pallet::*;
use sp_core::H256;
use sp_runtime::{
    BoundedVec,
    traits::{BlakeTwo256, Hash},
};
use sp_std::vec;

#[frame_benchmarking::v2::benchmarks]
mod pallet_benchmarks {
    use super::*;

    #[benchmark]
    fn register() {
        let netuid: u16 = 1;
        let tempo: u16 = 1;
        let hotkey: T::AccountId = account("Alice", 0, 1);
        let coldkey: T::AccountId = account("Test", 0, 2);

        Subtensor::<T>::init_new_network(netuid, tempo);
        Subtensor::<T>::set_network_registration_allowed(netuid, true);
        Subtensor::<T>::set_network_pow_registration_allowed(netuid, true);

        let block_number: u64 = Subtensor::<T>::get_current_block_as_u64();
        let (nonce, work): (u64, Vec<u8>) =
            Subtensor::<T>::create_work_for_block_number(netuid, block_number, 3, &hotkey);

        #[extrinsic_call]
        _(
            RawOrigin::Signed(hotkey.clone()),
            netuid,
            block_number,
            nonce,
            work,
            hotkey.clone(),
            coldkey.clone(),
        );
    }

    #[benchmark]
    fn set_weights() {
        let netuid: u16 = 1;
        let version_key: u64 = 1;
        let tempo: u16 = 1;

        Subtensor::<T>::init_new_network(netuid, tempo);
        Subtensor::<T>::set_max_allowed_uids(netuid, 4096);
        SubtokenEnabled::<T>::insert(netuid, true);
        Subtensor::<T>::set_network_registration_allowed(netuid, true);
        Subtensor::<T>::set_max_registrations_per_block(netuid, 4096);
        Subtensor::<T>::set_target_registrations_per_interval(netuid, 4096);

        let mut seed: u32 = 1;
        let mut dests = Vec::new();
        let mut weights = Vec::new();
        let signer: T::AccountId = account("Alice", 0, seed);

        for _ in 0..4096 {
            let hotkey: T::AccountId = account("Alice", 0, seed);
            let coldkey: T::AccountId = account("Test", 0, seed);
            seed += 1;

            Subtensor::<T>::set_burn(netuid, 1);
            let amount_to_be_staked: u64 = 1_000_000;
            Subtensor::<T>::add_balance_to_coldkey_account(&coldkey, amount_to_be_staked);

            assert_ok!(Subtensor::<T>::do_burned_registration(
                RawOrigin::Signed(coldkey.clone()).into(),
                netuid,
                hotkey.clone()
            ));
            let uid = Subtensor::<T>::get_uid_for_net_and_hotkey(netuid, &hotkey).unwrap();
            Subtensor::<T>::set_validator_permit_for_uid(netuid, uid, true);

            dests.push(uid);
            weights.push(uid);
        }

        #[extrinsic_call]
        _(
            RawOrigin::Signed(signer.clone()),
            netuid,
            dests,
            weights,
            version_key,
        );
    }

    #[benchmark]
    fn become_delegate() {
        let netuid: u16 = 1;
        let tempo: u16 = 1;

        Subtensor::<T>::init_new_network(netuid, tempo);
        SubtokenEnabled::<T>::insert(netuid, true);
        Subtensor::<T>::set_burn(netuid, 1);
        Subtensor::<T>::set_max_allowed_uids(netuid, 4096);
        Subtensor::<T>::set_network_registration_allowed(netuid, true);

        let seed: u32 = 1;
        let coldkey: T::AccountId = account("Test", 0, seed);
        let hotkey: T::AccountId = account("Alice", 0, seed);
        let amount_to_be_staked: u64 = 1_000_000_000;

        Subtensor::<T>::add_balance_to_coldkey_account(&coldkey, amount_to_be_staked);
        assert_ok!(Subtensor::<T>::do_burned_registration(
            RawOrigin::Signed(coldkey.clone()).into(),
            netuid,
            hotkey.clone()
        ));

        #[extrinsic_call]
        _(RawOrigin::Signed(coldkey.clone()), hotkey.clone());
    }

    #[benchmark]
    fn add_stake() {
        let netuid: u16 = 1;
        let tempo: u16 = 1;

        Subtensor::<T>::init_new_network(netuid, tempo);
        SubtokenEnabled::<T>::insert(netuid, true);
        Subtensor::<T>::set_burn(netuid, 1);
        Subtensor::<T>::set_network_registration_allowed(netuid, true);
        Subtensor::<T>::set_max_allowed_uids(netuid, 4096);

        let seed: u32 = 1;
        let coldkey: T::AccountId = account("Test", 0, seed);
        let hotkey: T::AccountId = account("Alice", 0, seed);
        let total_stake: u64 = 1_000_000_000;
        let amount: u64 = 60_000_000;

        Subtensor::<T>::add_balance_to_coldkey_account(&coldkey, total_stake);
        assert_ok!(Subtensor::<T>::do_burned_registration(
            RawOrigin::Signed(coldkey.clone()).into(),
            netuid,
            hotkey.clone()
        ));

        #[extrinsic_call]
        _(
            RawOrigin::Signed(coldkey.clone()),
            hotkey.clone(),
            netuid,
            amount,
        );
    }

    // #[benchmark]
    // fn add_stake_aggregate() {
    //     let netuid: u16 = 1;
    //     let tempo: u16 = 1;
    //
    //     Subtensor::<T>::init_new_network(netuid, tempo);
    //     SubtokenEnabled::<T>::insert(netuid, true);
    //     Subtensor::<T>::set_burn(netuid, 1);
    //     Subtensor::<T>::set_network_registration_allowed(netuid, true);
    //     Subtensor::<T>::set_max_allowed_uids(netuid, 4096);
    //
    //     let seed: u32 = 1;
    //     let coldkey: T::AccountId = account("Test", 0, seed);
    //     let hotkey: T::AccountId = account("Alice", 0, seed);
    //     let total_stake: u64 = 1_000_000_000;
    //     let amount: u64 = 600_000;
    //
    //     Subtensor::<T>::add_balance_to_coldkey_account(&coldkey, total_stake);
    //     assert_ok!(Subtensor::<T>::do_burned_registration(
    //         RawOrigin::Signed(coldkey.clone()).into(),
    //         netuid,
    //         hotkey.clone()
    //     ));
    //
    //     #[extrinsic_call]
    //     _(
    //         RawOrigin::Signed(coldkey.clone()),
    //         hotkey.clone(),
    //         netuid,
    //         amount,
    //     );
    // }
    //
    // #[benchmark]
    // fn remove_stake_limit_aggregate() {
    //     let netuid: u16 = 1;
    //
    //     Subtensor::<T>::increase_total_stake(1_000_000_000_000);
    //     Subtensor::<T>::init_new_network(netuid, 1);
    //     Subtensor::<T>::set_network_registration_allowed(netuid, true);
    //     SubtokenEnabled::<T>::insert(netuid, true);
    //     Subtensor::<T>::set_max_allowed_uids(netuid, 4096);
    //
    //     let seed: u32 = 1;
    //     let coldkey: T::AccountId = account("Test", 0, seed);
    //     let hotkey: T::AccountId = account("Alice", 0, seed);
    //     Subtensor::<T>::set_burn(netuid, 1);
    //
    //     let limit: u64 = 1_000_000_000;
    //     let tao_reserve: u64 = 150_000_000_000;
    //     let alpha_in: u64 = 100_000_000_000;
    //     SubnetTAO::<T>::insert(netuid, tao_reserve);
    //     SubnetAlphaIn::<T>::insert(netuid, alpha_in);
    //
    //     let wallet_bal: u64 = 1_000_000;
    //     Subtensor::<T>::add_balance_to_coldkey_account(&coldkey, wallet_bal);
    //
    //     assert_ok!(Subtensor::<T>::do_burned_registration(
    //         RawOrigin::Signed(coldkey.clone()).into(),
    //         netuid,
    //         hotkey.clone()
    //     ));
    //
    //     Subtensor::<T>::add_balance_to_coldkey_account(&coldkey, 100_000_000_000u64);
    //     assert_ok!(Subtensor::<T>::add_stake(
    //         RawOrigin::Signed(coldkey.clone()).into(),
    //         hotkey.clone(),
    //         netuid,
    //         100_000_000_000u64
    //     ));
    //
    //     let amount_unstaked: u64 = 30_000_000_000;
    //
    //     #[extrinsic_call]
    //     _(
    //         RawOrigin::Signed(coldkey.clone()),
    //         hotkey.clone(),
    //         netuid,
    //         amount_unstaked,
    //         limit,
    //         false,
    //     );
    // }
    //
    // #[benchmark]
    // fn remove_stake_aggregate() {
    //     let netuid: u16 = 1;
    //
    //     Subtensor::<T>::increase_total_stake(1_000_000_000_000);
    //     Subtensor::<T>::init_new_network(netuid, 1);
    //     Subtensor::<T>::set_network_registration_allowed(netuid, true);
    //     SubtokenEnabled::<T>::insert(netuid, true);
    //     Subtensor::<T>::set_max_allowed_uids(netuid, 4096);
    //
    //     let seed: u32 = 1;
    //     let coldkey: T::AccountId = account("Test", 0, seed);
    //     let hotkey: T::AccountId = account("Alice", 0, seed);
    //     Subtensor::<T>::set_burn(netuid, 1);
    //
    //     let wallet_bal: u64 = 1_000_000;
    //     Subtensor::<T>::add_balance_to_coldkey_account(&coldkey, wallet_bal);
    //
    //     assert_ok!(Subtensor::<T>::do_burned_registration(
    //         RawOrigin::Signed(coldkey.clone()).into(),
    //         netuid,
    //         hotkey.clone()
    //     ));
    //
    //     Subtensor::<T>::add_balance_to_coldkey_account(&coldkey, 100_000_000_000u64);
    //     assert_ok!(Subtensor::<T>::add_stake(
    //         RawOrigin::Signed(coldkey.clone()).into(),
    //         hotkey.clone(),
    //         netuid,
    //         100_000_000_000u64
    //     ));
    //
    //     let amount_unstaked: u64 = 600_000;
    //
    //     #[extrinsic_call]
    //     _(
    //         RawOrigin::Signed(coldkey.clone()),
    //         hotkey.clone(),
    //         netuid,
    //         amount_unstaked,
    //     );
    // }
    //
    // #[benchmark]
    // fn add_stake_limit_aggregate() {
    //     let netuid: u16 = 1;
    //
    //     Subtensor::<T>::init_new_network(netuid, 1);
    //     SubtokenEnabled::<T>::insert(netuid, true);
    //     Subtensor::<T>::set_burn(netuid, 1);
    //     Subtensor::<T>::set_network_registration_allowed(netuid, true);
    //     Subtensor::<T>::set_max_allowed_uids(netuid, 4096);
    //
    //     let seed: u32 = 1;
    //     let coldkey: T::AccountId = account("Test", 0, seed);
    //     let hotkey: T::AccountId = account("Alice", 0, seed);
    //
    //     let amount: u64 = 900_000_000_000;
    //     let limit: u64 = 6_000_000_000;
    //     let stake_amt: u64 = 440_000_000_000;
    //     Subtensor::<T>::add_balance_to_coldkey_account(&coldkey, amount);
    //
    //     let tao_reserve: u64 = 150_000_000_000;
    //     let alpha_in: u64 = 100_000_000_000;
    //     SubnetTAO::<T>::insert(netuid, tao_reserve);
    //     SubnetAlphaIn::<T>::insert(netuid, alpha_in);
    //
    //     assert_ok!(Subtensor::<T>::do_burned_registration(
    //         RawOrigin::Signed(coldkey.clone()).into(),
    //         netuid,
    //         hotkey.clone()
    //     ));
    //
    //     #[extrinsic_call]
    //     _(
    //         RawOrigin::Signed(coldkey.clone()),
    //         hotkey.clone(),
    //         netuid,
    //         stake_amt,
    //         limit,
    //         false,
    //     );
    // }

    #[benchmark]
    fn serve_axon() {
        let netuid: u16 = 1;
        let caller: T::AccountId = whitelisted_caller();
        let version: u32 = 2;
        let ip: u128 = 1676056785;
        let port: u16 = 128;
        let ip_type: u8 = 4;
        let protocol: u8 = 0;
        let placeholder1: u8 = 0;
        let placeholder2: u8 = 0;

        Subtensor::<T>::init_new_network(netuid, 1);
        SubtokenEnabled::<T>::insert(netuid, true);
        Subtensor::<T>::set_max_allowed_uids(netuid, 4096);

        let reg_fee: u64 = Subtensor::<T>::get_burn_as_u64(netuid);
        let deposit = reg_fee.saturating_mul(2);
        Subtensor::<T>::add_balance_to_coldkey_account(&caller, deposit);

        assert_ok!(Subtensor::<T>::do_burned_registration(
            RawOrigin::Signed(caller.clone()).into(),
            netuid,
            caller.clone()
        ));
        Subtensor::<T>::set_serving_rate_limit(netuid, 0);

        #[extrinsic_call]
        _(
            RawOrigin::Signed(caller.clone()),
            netuid,
            version,
            ip,
            port,
            ip_type,
            protocol,
            placeholder1,
            placeholder2,
        );
    }

    #[benchmark]
    fn serve_prometheus() {
        let netuid: u16 = 1;
        let caller: T::AccountId = whitelisted_caller();
        let version: u32 = 2;
        let ip: u128 = 1676056785;
        let port: u16 = 128;
        let ip_type: u8 = 4;

        Subtensor::<T>::init_new_network(netuid, 1);
        SubtokenEnabled::<T>::insert(netuid, true);
        Subtensor::<T>::set_max_allowed_uids(netuid, 4096);

        let reg_fee: u64 = Subtensor::<T>::get_burn_as_u64(netuid);
        let deposit = reg_fee.saturating_mul(2);
        Subtensor::<T>::add_balance_to_coldkey_account(&caller, deposit);

        assert_ok!(Subtensor::<T>::do_burned_registration(
            RawOrigin::Signed(caller.clone()).into(),
            netuid,
            caller.clone()
        ));
        Subtensor::<T>::set_serving_rate_limit(netuid, 0);

        #[extrinsic_call]
        _(
            RawOrigin::Signed(caller.clone()),
            netuid,
            version,
            ip,
            port,
            ip_type,
        );
    }

    #[benchmark]
    fn burned_register() {
        let netuid: u16 = 1;
        let seed: u32 = 1;
        let hotkey: T::AccountId = account("Alice", 0, seed);
        let coldkey: T::AccountId = account("Test", 0, seed);

        Subtensor::<T>::init_new_network(netuid, 1);
        SubtokenEnabled::<T>::insert(netuid, true);
        Subtensor::<T>::set_burn(netuid, 1);

        let amount: u64 = 1_000_000;
        Subtensor::<T>::add_balance_to_coldkey_account(&coldkey, amount);

        #[extrinsic_call]
        _(RawOrigin::Signed(coldkey.clone()), netuid, hotkey.clone());
    }

    #[benchmark]
    fn root_register() {
        let netuid: u16 = 1;
        let seed: u32 = 1;
        let coldkey: T::AccountId = account("Test", 0, seed);
        let hotkey: T::AccountId = account("Alice", 0, seed);

        Subtensor::<T>::init_new_network(netuid, 1);
        SubtokenEnabled::<T>::insert(netuid, true);
        Subtensor::<T>::set_burn(netuid, 1);
        Subtensor::<T>::set_network_registration_allowed(netuid, true);
        Subtensor::<T>::set_max_allowed_uids(netuid, 4096);
        assert_eq!(Subtensor::<T>::get_max_allowed_uids(netuid), 4096);

        let amount: u64 = 100_000_000_000_000;
        Subtensor::<T>::add_balance_to_coldkey_account(&coldkey, amount);

        assert_ok!(Subtensor::<T>::do_burned_registration(
            RawOrigin::Signed(coldkey.clone()).into(),
            netuid,
            hotkey.clone()
        ));

        #[extrinsic_call]
        _(RawOrigin::Signed(coldkey.clone()), hotkey.clone());
    }

    #[benchmark]
    fn register_network() {
        let seed: u32 = 1;
        let coldkey: T::AccountId = account("Test", 0, seed);
        let hotkey: T::AccountId = account("TestHotkey", 0, seed);

        Subtensor::<T>::set_network_rate_limit(1);
        let amount: u64 = 100_000_000_000_000u64.saturating_mul(2);
        Subtensor::<T>::add_balance_to_coldkey_account(&coldkey, amount);

        #[extrinsic_call]
        _(RawOrigin::Signed(coldkey.clone()), hotkey.clone());
    }

    #[benchmark]
    fn commit_weights() {
        let tempo: u16 = 1;
        let netuid: u16 = 1;
        let version_key: u64 = 0;
        let uids: Vec<u16> = vec![0];
        let weight_values: Vec<u16> = vec![10];
        let hotkey: T::AccountId = account("hot", 0, 1);
        let coldkey: T::AccountId = account("cold", 0, 2);
        let start_nonce: u64 = 300_000;

        let commit_hash: H256 = BlakeTwo256::hash_of(&(
            hotkey.clone(),
            netuid,
            uids.clone(),
            weight_values.clone(),
            version_key,
        ));

        Subtensor::<T>::init_new_network(netuid, tempo);
        Subtensor::<T>::set_network_pow_registration_allowed(netuid, true);

        let block_number: u64 = Subtensor::<T>::get_current_block_as_u64();
        let (nonce, work) = Subtensor::<T>::create_work_for_block_number(
            netuid,
            block_number,
            start_nonce,
            &hotkey,
        );
        assert_ok!(Subtensor::<T>::register(
            RawOrigin::Signed(hotkey.clone()).into(),
            netuid,
            block_number,
            nonce,
            work,
            hotkey.clone(),
            coldkey.clone()
        ));
        Subtensor::<T>::set_validator_permit_for_uid(netuid, 0, true);
        Subtensor::<T>::set_commit_reveal_weights_enabled(netuid, true);

        #[extrinsic_call]
        _(RawOrigin::Signed(hotkey.clone()), netuid, commit_hash);
    }

    #[benchmark]
    fn reveal_weights() {
        let tempo: u16 = 0;
        let netuid: u16 = 1;
        let version_key: u64 = 0;
        let uids: Vec<u16> = vec![0];
        let weight_values: Vec<u16> = vec![10];
        let salt: Vec<u16> = vec![8];
        let hotkey: T::AccountId = account("hot", 0, 1);
        let coldkey: T::AccountId = account("cold", 1, 2);

        Subtensor::<T>::init_new_network(netuid, tempo);
        Subtensor::<T>::set_network_registration_allowed(netuid, true);
        Subtensor::<T>::set_network_pow_registration_allowed(netuid, true);

        let block_number: u64 = Subtensor::<T>::get_current_block_as_u64();
        let (nonce, work) =
            Subtensor::<T>::create_work_for_block_number(netuid, block_number, 3, &hotkey);

        let _ = Subtensor::<T>::register(
            RawOrigin::Signed(hotkey.clone()).into(),
            netuid,
            block_number,
            nonce,
            work.clone(),
            hotkey.clone(),
            coldkey.clone(),
        );

        Subtensor::<T>::set_validator_permit_for_uid(netuid, 0, true);
        Subtensor::<T>::set_commit_reveal_weights_enabled(netuid, true);

        let commit_hash: H256 = BlakeTwo256::hash_of(&(
            hotkey.clone(),
            netuid,
            uids.clone(),
            weight_values.clone(),
            salt.clone(),
            version_key,
        ));
        let _ = Subtensor::<T>::commit_weights(
            RawOrigin::Signed(hotkey.clone()).into(),
            netuid,
            commit_hash,
        );

        #[extrinsic_call]
        _(
            RawOrigin::Signed(hotkey.clone()),
            netuid,
            uids.clone(),
            weight_values.clone(),
            salt.clone(),
            version_key,
        );
    }

    #[benchmark]
    fn schedule_swap_coldkey() {
        let old_coldkey: T::AccountId = account("old_cold", 0, 1);
        let new_coldkey: T::AccountId = account("new_cold", 1, 2);
        let amount: u64 = 100_000_000_000_000;
        Subtensor::<T>::add_balance_to_coldkey_account(&old_coldkey, amount);

        #[extrinsic_call]
        _(RawOrigin::Signed(old_coldkey.clone()), new_coldkey.clone());
    }

    #[benchmark]
    fn sudo_set_tx_childkey_take_rate_limit() {
        let new_rate_limit: u64 = 100;

        #[extrinsic_call]
        _(RawOrigin::Root, new_rate_limit);
    }

    #[benchmark]
    fn set_childkey_take() {
        let netuid: u16 = 1;
        let coldkey: T::AccountId = account("Cold", 0, 1);
        let hotkey: T::AccountId = account("Hot", 0, 1);
        let take: u16 = 1000;

        Subtensor::<T>::init_new_network(netuid, 1);
        Subtensor::<T>::set_network_registration_allowed(netuid, true);
        SubtokenEnabled::<T>::insert(netuid, true);

        let reg_fee: u64 = Subtensor::<T>::get_burn_as_u64(netuid);
        let deposit = reg_fee.saturating_mul(2);
        Subtensor::<T>::add_balance_to_coldkey_account(&coldkey, deposit);

        assert_ok!(Subtensor::<T>::do_burned_registration(
            RawOrigin::Signed(coldkey.clone()).into(),
            netuid,
            hotkey.clone()
        ));

        #[extrinsic_call]
        _(
            RawOrigin::Signed(coldkey.clone()),
            hotkey.clone(),
            netuid,
            take,
        );
    }

    #[benchmark]
    fn swap_coldkey() {
        let old_coldkey: T::AccountId = account("old_coldkey", 0, 0);
        let new_coldkey: T::AccountId = account("new_coldkey", 0, 0);
        let hotkey1: T::AccountId = account("hotkey1", 0, 0);
        let netuid: u16 = 1;
        let swap_cost: u64 = Subtensor::<T>::get_key_swap_cost();
        let free_balance_old: u64 = 12345 + swap_cost;

        Subtensor::<T>::init_new_network(netuid, 1);
        Subtensor::<T>::set_network_registration_allowed(netuid, true);
        Subtensor::<T>::set_network_pow_registration_allowed(netuid, true);

        let block_number: u64 = Subtensor::<T>::get_current_block_as_u64();
        let (nonce, work) =
            Subtensor::<T>::create_work_for_block_number(netuid, block_number, 3, &hotkey1);
        let _ = Subtensor::<T>::register(
            RawOrigin::Signed(old_coldkey.clone()).into(),
            netuid,
            block_number,
            nonce,
            work.clone(),
            hotkey1.clone(),
            old_coldkey.clone(),
        );

        Subtensor::<T>::add_balance_to_coldkey_account(&old_coldkey, free_balance_old);
        let name: Vec<u8> = b"The fourth Coolest Identity".to_vec();
        let identity = ChainIdentity {
            name,
            url: vec![],
            image: vec![],
            discord: vec![],
            description: vec![],
            additional: vec![],
        };
        Identities::<T>::insert(&old_coldkey, identity);

        #[extrinsic_call]
        _(
            RawOrigin::Root,
            old_coldkey.clone(),
            new_coldkey.clone(),
            swap_cost,
        );
    }

    #[benchmark]
    fn batch_reveal_weights() {
        let tempo: u16 = 0;
        let netuid: u16 = 1;
        let num_commits: usize = 10;

        let hotkey: T::AccountId = account("hot", 0, 1);
        let coldkey: T::AccountId = account("cold", 0, 2);

        Subtensor::<T>::init_new_network(netuid, tempo);
        Subtensor::<T>::set_network_registration_allowed(netuid, true);
        Subtensor::<T>::set_network_pow_registration_allowed(netuid, true);
        Subtensor::<T>::set_commit_reveal_weights_enabled(netuid, true);
        Subtensor::<T>::set_weights_set_rate_limit(netuid, 0);

        let block_number: u64 = Subtensor::<T>::get_current_block_as_u64();
        let (nonce, work) =
            Subtensor::<T>::create_work_for_block_number(netuid, block_number, 3, &hotkey);
        let origin = T::RuntimeOrigin::from(RawOrigin::Signed(hotkey.clone()));
        assert_ok!(Subtensor::<T>::register(
            origin.clone(),
            netuid,
            block_number,
            nonce,
            work.clone(),
            hotkey.clone(),
            coldkey.clone()
        ));
        Subtensor::<T>::set_validator_permit_for_uid(netuid, 0, true);

        let mut uids_list = Vec::new();
        let mut values_list = Vec::new();
        let mut salts_list = Vec::new();
        let mut version_keys = Vec::new();

        for i in 0..num_commits {
            let uids = vec![0u16];
            let values = vec![i as u16];
            let salts = vec![i as u16];
            let version_key_i: u64 = i as u64;

            let commit_hash: H256 = BlakeTwo256::hash_of(&(
                hotkey.clone(),
                netuid,
                uids.clone(),
                values.clone(),
                salts.clone(),
                version_key_i,
            ));

            assert_ok!(Subtensor::<T>::commit_weights(
                RawOrigin::Signed(hotkey.clone()).into(),
                netuid,
                commit_hash
            ));

            uids_list.push(uids);
            values_list.push(values);
            salts_list.push(salts);
            version_keys.push(version_key_i);
        }

        #[extrinsic_call]
        _(
            RawOrigin::Signed(hotkey.clone()),
            netuid,
            uids_list,
            values_list,
            salts_list,
            version_keys,
        );
    }

    #[benchmark]
    fn recycle_alpha() {
        let netuid: u16 = 1;

        let coldkey: T::AccountId = account("Test", 0, 1);
        let hotkey: T::AccountId = account("Alice", 0, 1);

        Subtensor::<T>::init_new_network(netuid, 1);
        SubtokenEnabled::<T>::insert(netuid, true);
        Subtensor::<T>::set_network_registration_allowed(netuid, true);
        Subtensor::<T>::set_burn(netuid, 1);

        let amount_to_be_staked: u64 = 1_000_000_000;
        Subtensor::<T>::add_balance_to_coldkey_account(&coldkey, amount_to_be_staked);
        assert_ok!(Subtensor::<T>::do_burned_registration(
            RawOrigin::Signed(coldkey.clone()).into(),
            netuid,
            hotkey.clone()
        ));

        let alpha_amount: u64 = 1_000_000;
        SubnetAlphaOut::<T>::insert(netuid, alpha_amount * 2);

        Subtensor::<T>::increase_stake_for_hotkey_and_coldkey_on_subnet(
            &hotkey,
            &coldkey,
            netuid,
            alpha_amount,
        );

        assert_eq!(TotalHotkeyAlpha::<T>::get(&hotkey, netuid), alpha_amount);

        #[extrinsic_call]
        _(
            RawOrigin::Signed(coldkey.clone()),
            hotkey.clone(),
            alpha_amount,
            netuid,
        );
    }

    #[benchmark]
    fn burn_alpha() {
        let netuid: u16 = 1;
        let coldkey: T::AccountId = account("Test", 0, 1);
        let hotkey: T::AccountId = account("Alice", 0, 1);

        Subtensor::<T>::init_new_network(netuid, 1);
        SubtokenEnabled::<T>::insert(netuid, true);
        Subtensor::<T>::set_network_registration_allowed(netuid, true);
        Subtensor::<T>::set_burn(netuid, 1);

        let amount_to_be_staked: u64 = 1_000_000_000;
        Subtensor::<T>::add_balance_to_coldkey_account(&coldkey, amount_to_be_staked);
        assert_ok!(Subtensor::<T>::do_burned_registration(
            RawOrigin::Signed(coldkey.clone()).into(),
            netuid,
            hotkey.clone()
        ));

        let alpha_amount: u64 = 1_000_000;
        SubnetAlphaOut::<T>::insert(netuid, alpha_amount * 2);
        Subtensor::<T>::increase_stake_for_hotkey_and_coldkey_on_subnet(
            &hotkey,
            &coldkey,
            netuid,
            alpha_amount,
        );
        assert_eq!(TotalHotkeyAlpha::<T>::get(&hotkey, netuid), alpha_amount);

        #[extrinsic_call]
        _(
            RawOrigin::Signed(coldkey.clone()),
            hotkey.clone(),
            alpha_amount,
            netuid,
        );
    }

    #[benchmark]
    fn start_call() {
        let netuid: u16 = 1;
        let coldkey: T::AccountId = account("Test", 0, 1);
        let hotkey: T::AccountId = account("Alice", 0, 1);

        Subtensor::<T>::init_new_network(netuid, 1);
        SubtokenEnabled::<T>::insert(netuid, true);
        Subtensor::<T>::set_network_registration_allowed(netuid, true);

        Subtensor::<T>::set_burn(netuid, 1);
        let amount_to_be_staked: u64 = 1_000_000;
        Subtensor::<T>::add_balance_to_coldkey_account(&coldkey, amount_to_be_staked);
        SubnetOwner::<T>::set(netuid, coldkey.clone());

        assert_ok!(Subtensor::<T>::do_burned_registration(
            RawOrigin::Signed(coldkey.clone()).into(),
            netuid,
            hotkey.clone()
        ));
        assert_eq!(SubnetOwner::<T>::get(netuid), coldkey.clone());
        assert_eq!(FirstEmissionBlockNumber::<T>::get(netuid), None);

        let current_block: u64 = Subtensor::<T>::get_current_block_as_u64();
        let duration = <T as Config>::DurationOfStartCall::get();
        let block: BlockNumberFor<T> = (current_block + duration)
            .try_into()
            .ok()
            .expect("can't convert to block number");
        frame_system::Pallet::<T>::set_block_number(block);

        #[extrinsic_call]
        _(RawOrigin::Signed(coldkey.clone()), netuid);
    }

    #[benchmark]
    fn adjust_senate() {
        let coldkey: T::AccountId = whitelisted_caller();
        let hotkey: T::AccountId = account("Alice", 0, 1);
        let root: u16 = Subtensor::<T>::get_root_netuid();

        Subtensor::<T>::init_new_network(root, 1);
        Uids::<T>::insert(root, &hotkey, 0u16);

        #[extrinsic_call]
        _(RawOrigin::Signed(coldkey.clone()), hotkey.clone());
    }

    #[benchmark]
    fn add_stake_limit() {
        let netuid: u16 = 1;
        let tempo: u16 = 1;
        let seed: u32 = 1;

        Subtensor::<T>::init_new_network(netuid, tempo);
        SubtokenEnabled::<T>::insert(netuid, true);
        Subtensor::<T>::set_burn(netuid, 1);
        Subtensor::<T>::set_network_registration_allowed(netuid, true);
        Subtensor::<T>::set_max_allowed_uids(netuid, 4096);

        let coldkey: T::AccountId = account("Test", 0, seed);
        let hotkey: T::AccountId = account("Alice", 0, seed);

        let amount = 900_000_000_000;
        let limit: u64 = 6_000_000_000;
        let amount_to_be_staked = 440_000_000_000;
        Subtensor::<T>::add_balance_to_coldkey_account(&coldkey.clone(), amount);

        let tao_reserve = 150_000_000_000_u64;
        let alpha_in = 100_000_000_000_u64;
        SubnetTAO::<T>::insert(netuid, tao_reserve);
        SubnetAlphaIn::<T>::insert(netuid, alpha_in);

        assert_ok!(Subtensor::<T>::do_burned_registration(
            RawOrigin::Signed(coldkey.clone()).into(),
            netuid,
            hotkey.clone()
        ));

        #[extrinsic_call]
        _(
            RawOrigin::Signed(coldkey.clone()),
            hotkey,
            netuid,
            amount_to_be_staked,
            limit,
            false,
        );
    }

    #[benchmark]
    fn move_stake() {
        let coldkey: T::AccountId = whitelisted_caller();
        let origin: T::AccountId = account("A", 0, 1);
        let destination: T::AccountId = account("B", 0, 2);
        let netuid: u16 = 1;

        SubtokenEnabled::<T>::insert(netuid, true);
        Subtensor::<T>::init_new_network(netuid, 1);

        let burn_fee = Subtensor::<T>::get_burn_as_u64(netuid);
        let stake_tao: u64 = 1_000_000;
        let deposit = burn_fee.saturating_mul(2).saturating_add(stake_tao);
        Subtensor::<T>::add_balance_to_coldkey_account(&coldkey, deposit);

        assert_ok!(Subtensor::<T>::burned_register(
            RawOrigin::Signed(coldkey.clone()).into(),
            netuid,
            origin.clone()
        ));

        SubnetTAO::<T>::insert(netuid, deposit);
        SubnetAlphaIn::<T>::insert(netuid, deposit);
        TotalStake::<T>::set(deposit);

        assert_ok!(Subtensor::<T>::add_stake_limit(
            RawOrigin::Signed(coldkey.clone()).into(),
            origin.clone(),
            netuid,
            stake_tao,
            u64::MAX,
            false
        ));

        let alpha_to_move: u64 =
            Subtensor::<T>::get_stake_for_hotkey_and_coldkey_on_subnet(&origin, &coldkey, netuid);

        Subtensor::<T>::create_account_if_non_existent(&coldkey, &destination);

        #[extrinsic_call]
        _(
            RawOrigin::Signed(coldkey.clone()),
            origin.clone(),
            destination.clone(),
            netuid,
            netuid,
            alpha_to_move,
        );
    }

    #[benchmark]
    fn remove_stake_limit() {
        let netuid: u16 = 1;
        let tempo: u16 = 1;
        let seed: u32 = 1;

        // Set our total stake to 1000 TAO
        Subtensor::<T>::increase_total_stake(1_000_000_000_000);

        Subtensor::<T>::init_new_network(netuid, tempo);
        Subtensor::<T>::set_network_registration_allowed(netuid, true);
        SubtokenEnabled::<T>::insert(netuid, true);

        Subtensor::<T>::set_max_allowed_uids(netuid, 4096);
        assert_eq!(Subtensor::<T>::get_max_allowed_uids(netuid), 4096);

        let coldkey: T::AccountId = account("Test", 0, seed);
        let hotkey: T::AccountId = account("Alice", 0, seed);
        Subtensor::<T>::set_burn(netuid, 1);

        let limit: u64 = 1_000_000_000;
        let tao_reserve = 150_000_000_000_u64;
        let alpha_in = 100_000_000_000_u64;
        SubnetTAO::<T>::insert(netuid, tao_reserve);
        SubnetAlphaIn::<T>::insert(netuid, alpha_in);

        let wallet_bal = 1000000u32.into();
        Subtensor::<T>::add_balance_to_coldkey_account(&coldkey.clone(), wallet_bal);

        assert_ok!(Subtensor::<T>::do_burned_registration(
            RawOrigin::Signed(coldkey.clone()).into(),
            netuid,
            hotkey.clone()
        ));

        let u64_staked_amt = 100_000_000_000;
        Subtensor::<T>::add_balance_to_coldkey_account(&coldkey.clone(), u64_staked_amt);

        assert_ok!(Subtensor::<T>::add_stake(
            RawOrigin::Signed(coldkey.clone()).into(),
            hotkey.clone(),
            netuid,
            u64_staked_amt
        ));

        let amount_unstaked: u64 = 30_000_000_000;

        #[extrinsic_call]
        _(
            RawOrigin::Signed(coldkey.clone()),
            hotkey.clone(),
            netuid,
            amount_unstaked,
            limit,
            false,
        );
    }

    #[benchmark]
    fn swap_stake_limit() {
        let coldkey: T::AccountId = whitelisted_caller::<AccountIdOf<T>>();
        let hot: T::AccountId = account("A", 0, 1);
        let netuid1: u16 = 1;
        let netuid2: u16 = 2;
        let allow: bool = true;

        SubtokenEnabled::<T>::insert(netuid1, true);
        Subtensor::<T>::init_new_network(netuid1, 1);
        SubtokenEnabled::<T>::insert(netuid2, true);
        Subtensor::<T>::init_new_network(netuid2, 1);

        let tao_reserve = 150_000_000_000_u64;
        let alpha_in = 100_000_000_000_u64;
        SubnetTAO::<T>::insert(netuid1, tao_reserve);
        SubnetAlphaIn::<T>::insert(netuid1, alpha_in);
        SubnetTAO::<T>::insert(netuid2, tao_reserve);

        Subtensor::<T>::increase_total_stake(1_000_000_000_000);

        let amount = 900_000_000_000;
        let limit_stake: u64 = 6_000_000_000;
        let limit_swap: u64 = 1_000_000_000;
        let amount_to_be_staked = 440_000_000_000;
        let amount_swapped: u64 = 30_000_000_000;
        Subtensor::<T>::add_balance_to_coldkey_account(&coldkey.clone(), amount);

        assert_ok!(Subtensor::<T>::burned_register(
            RawOrigin::Signed(coldkey.clone()).into(),
            netuid1,
            hot.clone()
        ));

        assert_ok!(Subtensor::<T>::burned_register(
            RawOrigin::Signed(coldkey.clone()).into(),
            netuid2,
            hot.clone()
        ));

        assert_ok!(Subtensor::<T>::add_stake_limit(
            RawOrigin::Signed(coldkey.clone()).into(),
            hot.clone(),
            netuid1,
            amount_to_be_staked,
            limit_stake,
            allow
        ));

        #[extrinsic_call]
        _(
            RawOrigin::Signed(coldkey.clone()),
            hot.clone(),
            netuid1,
            netuid2,
            amount_swapped,
            limit_swap,
            allow,
        );
    }

    #[benchmark]
    fn transfer_stake() {
        let coldkey: T::AccountId = whitelisted_caller();
        let dest: T::AccountId = account("B", 0, 2);
        let hot: T::AccountId = account("A", 0, 1);
        let netuid: u16 = 1;

        SubtokenEnabled::<T>::insert(netuid, true);
        Subtensor::<T>::init_new_network(netuid, 1);

        let reg_fee = Subtensor::<T>::get_burn_as_u64(netuid);
        let stake_tao: u64 = 1_000_000;
        let deposit = reg_fee.saturating_mul(2).saturating_add(stake_tao);
        Subtensor::<T>::add_balance_to_coldkey_account(&coldkey, deposit);

        assert_ok!(Subtensor::<T>::burned_register(
            RawOrigin::Signed(coldkey.clone()).into(),
            netuid,
            hot.clone()
        ));

        SubnetTAO::<T>::insert(netuid, deposit);
        SubnetAlphaIn::<T>::insert(netuid, deposit);
        TotalStake::<T>::set(deposit);

        assert_ok!(Subtensor::<T>::add_stake_limit(
            RawOrigin::Signed(coldkey.clone()).into(),
            hot.clone(),
            netuid,
            stake_tao,
            u64::MAX,
            false
        ));

        let alpha_to_transfer: u64 =
            Subtensor::<T>::get_stake_for_hotkey_and_coldkey_on_subnet(&hot, &coldkey, netuid);

        Subtensor::<T>::create_account_if_non_existent(&dest, &hot);

        #[extrinsic_call]
        _(
            RawOrigin::Signed(coldkey.clone()),
            dest.clone(),
            hot.clone(),
            netuid,
            netuid,
            alpha_to_transfer,
        );
    }

    #[benchmark]
    fn swap_stake() {
        let coldkey: T::AccountId = whitelisted_caller();
        let hot: T::AccountId = account("A", 0, 9);
        let netuid1: u16 = 1;
        let netuid2: u16 = 2;

        SubtokenEnabled::<T>::insert(netuid1, true);
        Subtensor::<T>::init_new_network(netuid1, 1);
        SubtokenEnabled::<T>::insert(netuid2, true);
        Subtensor::<T>::init_new_network(netuid2, 1);

        let reg_fee = Subtensor::<T>::get_burn_as_u64(netuid1);
        let stake_tao: u64 = 1_000_000;
        let deposit = reg_fee.saturating_mul(2).saturating_add(stake_tao);
        Subtensor::<T>::add_balance_to_coldkey_account(&coldkey, deposit);

        assert_ok!(Subtensor::<T>::burned_register(
            RawOrigin::Signed(coldkey.clone()).into(),
            netuid1,
            hot.clone()
        ));

        SubnetTAO::<T>::insert(netuid1, deposit);
        SubnetAlphaIn::<T>::insert(netuid1, deposit);
        SubnetTAO::<T>::insert(netuid2, deposit);
        SubnetAlphaIn::<T>::insert(netuid2, deposit);
        TotalStake::<T>::set(deposit);

        assert_ok!(Subtensor::<T>::add_stake_limit(
            RawOrigin::Signed(coldkey.clone()).into(),
            hot.clone(),
            netuid1,
            stake_tao,
            u64::MAX,
            false
        ));

        let alpha_to_swap: u64 =
            Subtensor::<T>::get_stake_for_hotkey_and_coldkey_on_subnet(&hot, &coldkey, netuid1);

        #[extrinsic_call]
        _(
            RawOrigin::Signed(coldkey.clone()),
            hot.clone(),
            netuid1,
            netuid2,
            alpha_to_swap,
        );
    }

    #[benchmark]
    fn batch_commit_weights() {
        let hotkey: T::AccountId = whitelisted_caller();
        let netuid: u16 = 1;
        let count: usize = 3;
        let mut netuids: Vec<Compact<u16>> = Vec::new();
        let mut hashes: Vec<H256> = Vec::new();

        Subtensor::<T>::init_new_network(netuid, 1);
        Subtensor::<T>::set_network_pow_registration_allowed(netuid, true);
        SubtokenEnabled::<T>::insert(netuid, true);

        let reg_fee = Subtensor::<T>::get_burn_as_u64(netuid);
        Subtensor::<T>::add_balance_to_coldkey_account(&hotkey, reg_fee.saturating_mul(2));

        assert_ok!(Subtensor::<T>::burned_register(
            RawOrigin::Signed(hotkey.clone()).into(),
            netuid,
            hotkey.clone()
        ));

        Subtensor::<T>::set_validator_permit_for_uid(netuid, 0, true);
        Subtensor::<T>::set_commit_reveal_weights_enabled(netuid, true);

        for i in 0..count {
            netuids.push(Compact(netuid));
            hashes.push(H256::repeat_byte(i as u8));
        }

        #[extrinsic_call]
        _(
            RawOrigin::Signed(hotkey.clone()),
            netuids.clone(),
            hashes.clone(),
        );
    }

    #[benchmark]
    fn batch_set_weights() {
        let hotkey: T::AccountId = whitelisted_caller();
        let netuid: u16 = 1;
        let version: u64 = 1;
        let entries: Vec<(Compact<u16>, Compact<u16>)> = vec![(Compact(0u16), Compact(0u16))];
        let netuids: Vec<Compact<u16>> = vec![Compact(netuid)];
        let weights: Vec<Vec<(Compact<u16>, Compact<u16>)>> = vec![entries.clone()];
        let keys: Vec<Compact<u64>> = vec![Compact(version)];

        Subtensor::<T>::init_new_network(netuid, 1);
        Subtensor::<T>::set_network_registration_allowed(netuid, true);
        SubtokenEnabled::<T>::insert(netuid, true);

        let reg_fee = Subtensor::<T>::get_burn_as_u64(netuid);
        Subtensor::<T>::add_balance_to_coldkey_account(&hotkey, reg_fee.saturating_mul(2));

        assert_ok!(Subtensor::<T>::burned_register(
            RawOrigin::Signed(hotkey.clone()).into(),
            netuid,
            hotkey.clone()
        ));

        #[extrinsic_call]
        _(
            RawOrigin::Signed(hotkey.clone()),
            netuids.clone(),
            weights.clone(),
            keys.clone(),
        );
    }

    #[benchmark]
    fn commit_crv3_weights() {
        let hotkey: T::AccountId = whitelisted_caller();
        let netuid: u16 = 1;
        let vec_commit: Vec<u8> = vec![0; MAX_CRV3_COMMIT_SIZE_BYTES as usize];
        let commit: BoundedVec<_, _> = vec_commit.try_into().unwrap();
        let round: u64 = 0;

        Subtensor::<T>::init_new_network(netuid, 1);
        Subtensor::<T>::set_network_pow_registration_allowed(netuid, true);
        SubtokenEnabled::<T>::insert(netuid, true);

        let reg_fee = Subtensor::<T>::get_burn_as_u64(netuid);
        Subtensor::<T>::add_balance_to_coldkey_account(&hotkey, reg_fee.saturating_mul(2));

        assert_ok!(Subtensor::<T>::burned_register(
            RawOrigin::Signed(hotkey.clone()).into(),
            netuid,
            hotkey.clone()
        ));

        Subtensor::<T>::set_commit_reveal_weights_enabled(netuid, true);

        #[extrinsic_call]
        _(
            RawOrigin::Signed(hotkey.clone()),
            netuid,
            commit.clone(),
            round,
        );
    }

    #[benchmark]
    fn decrease_take() {
        let coldkey: T::AccountId = whitelisted_caller();
        let hotkey: T::AccountId = account("Alice", 0, 1);
        let take: u16 = 100;

        Delegates::<T>::insert(&hotkey, 200u16);
        Owner::<T>::insert(&hotkey, &coldkey);

        #[extrinsic_call]
        _(RawOrigin::Signed(coldkey.clone()), hotkey.clone(), take);
    }

    #[benchmark]
    fn increase_take() {
        let coldkey: T::AccountId = whitelisted_caller();
        let hotkey: T::AccountId = account("Alice", 0, 2);
        let take: u16 = 150;

        Delegates::<T>::insert(&hotkey, 100u16);
        Owner::<T>::insert(&hotkey, &coldkey);

        #[extrinsic_call]
        _(RawOrigin::Signed(coldkey.clone()), hotkey.clone(), take);
    }

    #[benchmark]
    fn register_network_with_identity() {
        let coldkey: T::AccountId = whitelisted_caller();
        let hotkey: T::AccountId = account("Alice", 0, 1);
        let identity: Option<SubnetIdentityOfV2> = None;

        Subtensor::<T>::set_network_registration_allowed(1, true);
        Subtensor::<T>::set_network_rate_limit(1);
        let amount: u64 = 9_999_999_999_999;
        Subtensor::<T>::add_balance_to_coldkey_account(&coldkey, amount);

        #[extrinsic_call]
        _(
            RawOrigin::Signed(coldkey.clone()),
            hotkey.clone(),
            identity.clone(),
        );
    }

    #[benchmark]
    fn serve_axon_tls() {
        let caller: T::AccountId = whitelisted_caller();
        let netuid: u16 = 1;
        let version: u32 = 1;
        let ip: u128 = 0xC0A8_0001;
        let port: u16 = 30333;
        let ip_type: u8 = 4;
        let proto: u8 = 0;
        let p1: u8 = 0;
        let p2: u8 = 0;
        let cert: Vec<u8> = vec![];

        Subtensor::<T>::init_new_network(netuid, 1);
        Subtensor::<T>::set_network_registration_allowed(netuid, true);
        SubtokenEnabled::<T>::insert(netuid, true);

        let reg_fee = Subtensor::<T>::get_burn_as_u64(netuid);
        let deposit: u64 = reg_fee.saturating_mul(2);
        Subtensor::<T>::add_balance_to_coldkey_account(&caller, deposit);

        assert_ok!(Subtensor::<T>::burned_register(
            RawOrigin::Signed(caller.clone()).into(),
            netuid,
            caller.clone()
        ));

        #[extrinsic_call]
        _(
            RawOrigin::Signed(caller.clone()),
            netuid,
            version,
            ip,
            port,
            ip_type,
            proto,
            p1,
            p2,
            cert.clone(),
        );
    }

    #[benchmark]
    fn set_identity() {
        let coldkey: T::AccountId = whitelisted_caller();
        let hotkey: T::AccountId = account("Alice", 0, 5);
        let name = b"n".to_vec();
        let url = vec![];
        let repo = vec![];
        let img = vec![];
        let disc = vec![];
        let descr = vec![];
        let add = vec![];

        Subtensor::<T>::create_account_if_non_existent(&coldkey, &hotkey);
        Subtensor::<T>::init_new_network(1, 1);
        let deposit: u64 = 1_000_000_000u64.saturating_mul(2);
        Subtensor::<T>::add_balance_to_coldkey_account(&coldkey, deposit);
        SubtokenEnabled::<T>::insert(1, true);

        assert_ok!(Subtensor::<T>::burned_register(
            RawOrigin::Signed(coldkey.clone()).into(),
            1,
            hotkey.clone()
        ));

        #[extrinsic_call]
        _(
            RawOrigin::Signed(coldkey.clone()),
            name.clone(),
            url.clone(),
            repo.clone(),
            img.clone(),
            disc.clone(),
            descr.clone(),
            add.clone(),
        );
    }

    #[benchmark]
    fn set_subnet_identity() {
        let coldkey: T::AccountId = whitelisted_caller();
        let netuid: u16 = 1;
        let name = b"n".to_vec();
        let repo = vec![];
        let contact = vec![];
        let url = vec![];
        let disc = vec![];
        let descr = vec![];
        let add = vec![];

        SubnetOwner::<T>::insert(netuid, coldkey.clone());
        SubtokenEnabled::<T>::insert(netuid, true);

        #[extrinsic_call]
        _(
            RawOrigin::Signed(coldkey.clone()),
            netuid,
            name.clone(),
            repo.clone(),
            contact.clone(),
            url.clone(),
            disc.clone(),
            descr.clone(),
            add.clone(),
        );
    }

    #[benchmark]
    fn set_tao_weights() {
        let netuid: u16 = 1;
        let hotkey: T::AccountId = account("A", 0, 6);
        let dests = vec![0u16];
        let weights = vec![0u16];
        let version: u64 = 1;

        Subtensor::<T>::init_new_network(netuid, 1);

        #[extrinsic_call]
        _(
            RawOrigin::None,
            netuid,
            hotkey.clone(),
            dests.clone(),
            weights.clone(),
            version,
        );
    }

    #[benchmark]
    fn swap_hotkey() {
        let coldkey: T::AccountId = whitelisted_caller();
        let old: T::AccountId = account("A", 0, 7);
        let new: T::AccountId = account("B", 0, 8);
        Owner::<T>::insert(&old, &coldkey);
        let cost: u64 = Subtensor::<T>::get_key_swap_cost();
        Subtensor::<T>::add_balance_to_coldkey_account(&coldkey, cost);

        #[extrinsic_call]
        _(
            RawOrigin::Signed(coldkey.clone()),
            old.clone(),
            new.clone(),
            None,
        );
    }

    #[benchmark]
    fn try_associate_hotkey() {
        let coldkey: T::AccountId = whitelisted_caller();
        let hot: T::AccountId = account("A", 0, 1);

        #[extrinsic_call]
        _(RawOrigin::Signed(coldkey.clone()), hot.clone());
    }

    #[benchmark]
    fn unstake_all() {
        let coldkey: T::AccountId = whitelisted_caller();
        let hotkey: T::AccountId = account("A", 0, 14);
        Subtensor::<T>::create_account_if_non_existent(&coldkey, &hotkey);

        #[extrinsic_call]
        _(RawOrigin::Signed(coldkey.clone()), hotkey.clone());
    }

    #[benchmark]
    fn unstake_all_alpha() {
        let coldkey: T::AccountId = whitelisted_caller();
        let hotkey: T::AccountId = account("A", 0, 15);
        Subtensor::<T>::create_account_if_non_existent(&coldkey, &hotkey);

        #[extrinsic_call]
        _(RawOrigin::Signed(coldkey.clone()), hotkey.clone());
    }
}
