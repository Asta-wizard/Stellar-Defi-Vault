use crate::storage::{
    AdminProposal, ChangelogEntry, ClaimWindow, DataKey, DayBucket, DynamicFeeConfig,
    FeeRecipient, GovernanceProposal, MultisigConfig, PendingAction, RateHistoryEntry,
    ReferralStats, StakePosition, VestingEntry,
};

use soroban_sdk::{symbol_short, Address, Env, String, Symbol, Vec};

pub fn get_shares(env: &Env, user: &Address) -> i128 {
    env.storage()
        .persistent()
        .get(&DataKey::ShareBalance(user.clone()))
        .unwrap_or(0)
}

pub fn set_shares(env: &Env, user: &Address, amount: i128) {
    env.storage()
        .persistent()
        .set(&DataKey::ShareBalance(user.clone()), &amount);
}

pub fn get_total_shares(env: &Env) -> i128 {
    env.storage()
        .instance()
        .get(&DataKey::TotalShares)
        .unwrap_or(0)
}

pub fn set_total_shares(env: &Env, total: i128) {
    env.storage().instance().set(&DataKey::TotalShares, &total);
}

pub fn get_total_deposited(env: &Env) -> i128 {
    env.storage()
        .instance()
        .get(&DataKey::TotalDeposited)
        .unwrap_or(0)
}

pub fn set_total_deposited(env: &Env, total: i128) {
    env.storage()
        .instance()
        .set(&DataKey::TotalDeposited, &total);
}

pub fn get_min_stake(env: &Env) -> i128 {
    env.storage()
        .instance()
        .get(&DataKey::MinStake)
        .unwrap_or(0)
}

pub fn set_min_stake(env: &Env, amount: i128) {
    env.storage().instance().set(&DataKey::MinStake, &amount);
}

pub fn get_reward_rate_bps(env: &Env) -> u32 {
    env.storage()
        .instance()
        .get(&DataKey::RewardRateBps)
        .unwrap_or(0)
}

pub fn set_reward_rate_bps(env: &Env, rate_bps: u32) {
    env.storage()
        .instance()
        .set(&DataKey::RewardRateBps, &rate_bps);
}

pub fn get_rate_history(env: &Env) -> Vec<(u32, u32)> {
    env.storage()
        .instance()
        .get(&DataKey::RateHistory)
        .unwrap_or(Vec::new(env))
}

pub fn set_rate_history(env: &Env, history: &Vec<(u32, u32)>) {
    env.storage().instance().set(&DataKey::RateHistory, history);
}

pub const MAX_RATE_HISTORY_ENTRIES: u32 = 50;

/// Maximum allowed reward rate in basis points (500% APR). Issue #72.
pub const MAX_RATE_BPS: u32 = 50_000;

pub fn get_reward_pool_balance(env: &Env) -> i128 {
    env.storage()
        .instance()
        .get(&DataKey::RewardPoolBalance)
        .unwrap_or(0)
}

pub fn set_reward_pool_balance(env: &Env, balance: i128) {
    env.storage()
        .instance()
        .set(&DataKey::RewardPoolBalance, &balance);
}

pub fn get_withdrawal_limit(env: &Env) -> Option<i128> {
    env.storage().instance().get(&DataKey::WithdrawalLimit)
}

pub fn set_withdrawal_limit(env: &Env, limit: i128) {
    env.storage()
        .instance()
        .set(&DataKey::WithdrawalLimit, &limit);
}

pub fn get_pool_cap(env: &Env) -> i128 {
    env.storage()
        .instance()
        .get(&symbol_short!("pool_cp"))
        .unwrap_or(0)
}

pub fn set_pool_cap(env: &Env, cap: i128) {
    env.storage()
        .instance()
        .set(&symbol_short!("pool_cp"), &cap);
}

pub fn get_unstake_fee_bps(env: &Env) -> u32 {
    env.storage()
        .instance()
        .get(&DataKey::UnstakeFeeBps)
        .unwrap_or(0)
}

pub fn set_unstake_fee_bps(env: &Env, bps: u32) {
    env.storage().instance().set(&DataKey::UnstakeFeeBps, &bps);
}

pub fn get_reward_checkpoint_ledger(env: &Env, user: &Address) -> Option<u32> {
    env.storage()
        .persistent()
        .get(&DataKey::RewardCheckpointLedger(user.clone()))
}

pub fn set_reward_checkpoint_ledger(env: &Env, user: &Address, ledger: u32) {
    env.storage()
        .persistent()
        .set(&DataKey::RewardCheckpointLedger(user.clone()), &ledger);
}

pub fn set_last_claim_ledger(env: &Env, user: &Address, ledger: u32) {
    env.storage()
        .persistent()
        .set(&DataKey::LastClaimLedger(user.clone()), &ledger);
}

pub fn get_accrued_reward(env: &Env, user: &Address) -> i128 {
    env.storage()
        .persistent()
        .get(&DataKey::AccruedReward(user.clone()))
        .unwrap_or(0)
}

pub fn set_accrued_reward(env: &Env, user: &Address, amount: i128) {
    env.storage()
        .persistent()
        .set(&DataKey::AccruedReward(user.clone()), &amount);
}

pub fn get_stake_history(env: &Env, user: &Address) -> Option<Vec<(u32, i128)>> {
    env.storage()
        .persistent()
        .get(&DataKey::StakeHistory(user.clone()))
}

pub fn set_stake_history(env: &Env, user: &Address, history: &Vec<(u32, i128)>) {
    env.storage()
        .persistent()
        .set(&DataKey::StakeHistory(user.clone()), history);
}

pub fn get_boost_schedule(env: &Env) -> Option<Vec<(u32, u32)>> {
    env.storage().instance().get(&DataKey::BoostSchedule)
}

pub fn set_boost_schedule(env: &Env, tiers: &Vec<(u32, u32)>) {
    env.storage().instance().set(&DataKey::BoostSchedule, tiers);
}

pub fn get_total_stakers(env: &Env) -> u32 {
    env.storage()
        .instance()
        .get(&DataKey::TotalStakers)
        .unwrap_or(0)
}

pub fn set_total_stakers(env: &Env, count: u32) {
    env.storage().instance().set(&DataKey::TotalStakers, &count);
}

pub fn get_total_rewards_paid(env: &Env) -> i128 {
    env.storage()
        .instance()
        .get(&DataKey::TotalRewardsPaid)
        .unwrap_or(0)
}

pub fn set_total_rewards_paid(env: &Env, amount: i128) {
    env.storage()
        .instance()
        .set(&DataKey::TotalRewardsPaid, &amount);
}

pub fn get_last_claim_ledger(env: &Env, user: &Address) -> u32 {
    env.storage()
        .persistent()
        .get(&DataKey::LastClaimLedger(user.clone()))
        .unwrap_or(0)
}

pub fn get_delegate(env: &Env, user: &Address) -> Option<Address> {
    env.storage()
        .persistent()
        .get(&DataKey::Delegate(user.clone()))
}

pub fn set_delegate(env: &Env, user: &Address, delegate: &Address) {
    env.storage()
        .persistent()
        .set(&DataKey::Delegate(user.clone()), delegate);
}

pub fn remove_delegate(env: &Env, user: &Address) {
    env.storage()
        .persistent()
        .remove(&DataKey::Delegate(user.clone()));
}

// ── Issue #38: slash treasury ────────────────────────────────────────────────

pub fn get_slash_treasury(env: &Env) -> Option<Address> {
    env.storage().instance().get(&symbol_short!("sl_tr"))
}

pub fn set_slash_treasury(env: &Env, treasury: &Address) {
    env.storage()
        .instance()
        .set(&symbol_short!("sl_tr"), treasury);
}

// ── Issue #39: reward token ───────────────────────────────────────────────────

pub fn get_reward_token(env: &Env) -> Option<Address> {
    env.storage().instance().get(&symbol_short!("rwd_tok"))
}

pub fn set_reward_token(env: &Env, token: &Address) {
    env.storage()
        .instance()
        .set(&symbol_short!("rwd_tok"), token);
}

// ── Issue #40: NFT contract ───────────────────────────────────────────────────

pub fn get_nft_contract(env: &Env) -> Option<Address> {
    env.storage().instance().get(&symbol_short!("nft_con"))
}

pub fn set_nft_contract(env: &Env, nft: &Address) {
    env.storage().instance().set(&symbol_short!("nft_con"), nft);
}

// ── Issue #41: restake grace window ──────────────────────────────────────────

pub fn get_restake_window(env: &Env) -> u32 {
    env.storage()
        .instance()
        .get(&symbol_short!("rst_wnd"))
        .unwrap_or(0)
}

pub fn set_restake_window(env: &Env, ledgers: u32) {
    env.storage()
        .instance()
        .set(&symbol_short!("rst_wnd"), &ledgers);
}

pub fn get_last_unstake_ledger(env: &Env, user: &Address) -> Option<u32> {
    env.storage()
        .persistent()
        .get(&DataKey::LastUnstakeLedger(user.clone()))
}

pub fn set_last_unstake_ledger(env: &Env, user: &Address, ledger: u32) {
    env.storage()
        .persistent()
        .set(&DataKey::LastUnstakeLedger(user.clone()), &ledger);
}

pub fn is_restaked(env: &Env, user: &Address) -> bool {
    env.storage()
        .persistent()
        .get(&DataKey::Restaked(user.clone()))
        .unwrap_or(false)
}

pub fn set_restaked(env: &Env, user: &Address, value: bool) {
    env.storage()
        .persistent()
        .set(&DataKey::Restaked(user.clone()), &value);
}

pub fn remove_restaked(env: &Env, user: &Address) {
    let key = DataKey::Restaked(user.clone());
    if env.storage().persistent().has(&key) {
        env.storage().persistent().remove(&key);
    }
}

// ── Issue #42: admin action count ────────────────────────────────────────────

pub fn get_admin_action_count(env: &Env) -> u32 {
    env.storage()
        .instance()
        .get(&symbol_short!("adm_cnt"))
        .unwrap_or(0)
}

pub fn increment_admin_action_count(env: &Env) {
    let count = get_admin_action_count(env);
    env.storage()
        .instance()
        .set(&symbol_short!("adm_cnt"), &(count + 1));
}

// ── Claim cap (issue #78) ─────────────────────────────────────────────────────

pub fn get_claim_cap(env: &Env) -> i128 {
    env.storage()
        .instance()
        .get(&symbol_short!("clm_cap"))
        .unwrap_or(0)
}

pub fn set_claim_cap(env: &Env, cap: i128) {
    env.storage()
        .instance()
        .set(&symbol_short!("clm_cap"), &cap);
}

pub fn get_claim_cap_window(env: &Env) -> u32 {
    env.storage()
        .instance()
        .get(&symbol_short!("clm_win"))
        .unwrap_or(0)
}

pub fn set_claim_cap_window(env: &Env, window_ledgers: u32) {
    env.storage()
        .instance()
        .set(&symbol_short!("clm_win"), &window_ledgers);
}

pub fn get_user_claim_window(env: &Env, user: &Address) -> Option<ClaimWindow> {
    env.storage()
        .persistent()
        .get(&DataKey::UserClaimWindow(user.clone()))
}

pub fn set_user_claim_window(env: &Env, user: &Address, window: &ClaimWindow) {
    env.storage()
        .persistent()
        .set(&DataKey::UserClaimWindow(user.clone()), window);
}

// ── Token decimals (reward normalization) ─────────────────────────────────────

/// Default decimal precision for Stellar tokens. Most tokens use 7 places,
/// but this is only a fallback for pools initialized without explicit values.
pub const DEFAULT_TOKEN_DECIMALS: u32 = 7;

pub fn get_stake_decimals(env: &Env) -> u32 {
    env.storage()
        .instance()
        .get(&symbol_short!("stk_dec"))
        .unwrap_or(DEFAULT_TOKEN_DECIMALS)
}

pub fn set_stake_decimals(env: &Env, decimals: u32) {
    env.storage()
        .instance()
        .set(&symbol_short!("stk_dec"), &decimals);
}

pub fn get_reward_decimals(env: &Env) -> u32 {
    env.storage()
        .instance()
        .get(&symbol_short!("rwd_dec"))
        .unwrap_or(DEFAULT_TOKEN_DECIMALS)
}

pub fn set_reward_decimals(env: &Env, decimals: u32) {
    env.storage()
        .instance()
        .set(&symbol_short!("rwd_dec"), &decimals);
}

// ── All-stakers list (issue #95) ──────────────────────────────────────────────

pub fn get_all_stakers(env: &Env) -> Vec<Address> {
    env.storage()
        .instance()
        .get(&DataKey::AllStakers)
        .unwrap_or(Vec::new(env))
}

pub fn set_all_stakers(env: &Env, stakers: &Vec<Address>) {
    env.storage().instance().set(&DataKey::AllStakers, stakers);
}

// ── Share math ────────────────────────────────────────────────────────────────

/// Convert a deposit amount to shares using current vault ratio.
/// First deposit: 1:1. Subsequent: proportional to existing pool.
pub fn amount_to_shares(total_shares: i128, total_deposited: i128, amount: i128) -> Option<i128> {
    if total_shares == 0 || total_deposited == 0 {
        Some(amount)
    } else {
        amount
            .checked_mul(total_shares)?
            .checked_div(total_deposited)
    }
}

/// Convert shares to the underlying token amount.
pub fn shares_to_amount(total_shares: i128, total_deposited: i128, shares: i128) -> Option<i128> {
    if total_shares == 0 {
        Some(0)
    } else {
        shares
            .checked_mul(total_deposited)?
            .checked_div(total_shares)
    }
}

pub fn get_reward_remainder(env: &Env, user: &Address) -> i128 {
    env.storage()
        .persistent()
        .get(&DataKey::RewardRemainder(user.clone()))
        .unwrap_or(0)
}

pub fn set_reward_remainder(env: &Env, user: &Address, amount: i128) {
    env.storage()
        .persistent()
        .set(&DataKey::RewardRemainder(user.clone()), &amount);
}

// ── Issue #69: last updated ledger ───────────────────────────────────────────
// Uses symbol_short! to avoid pushing DataKey over the contracttype variant limit.

pub fn get_last_updated_ledger(env: &Env) -> u32 {
    env.storage()
        .instance()
        .get(&symbol_short!("lst_upd"))
        .unwrap_or(0)
}

pub fn set_last_updated_ledger(env: &Env, ledger: u32) {
    env.storage()
        .instance()
        .set(&symbol_short!("lst_upd"), &ledger);
}

// ── Issue #97: pool description ──────────────────────────────────────────────

pub fn get_pool_description(env: &Env) -> Option<soroban_sdk::String> {
    env.storage().instance().get(&symbol_short!("pool_desc"))
}

pub fn set_pool_description(env: &Env, description: &soroban_sdk::String) {
    env.storage()
        .instance()
        .set(&symbol_short!("pool_desc"), description);
}

// ── Issue #99: staking streak ────────────────────────────────────────────────

pub fn get_last_recorded_wave(env: &Env) -> Option<u32> {
    env.storage().instance().get(&symbol_short!("last_wave"))
}

pub fn set_last_recorded_wave(env: &Env, wave_id: u32) {
    env.storage()
        .instance()
        .set(&symbol_short!("last_wave"), &wave_id);
}

pub fn get_user_streak(env: &Env, user: &Address) -> Option<crate::storage::StakeStreak> {
    let key = (Symbol::new(env, "strk"), user.clone());
    env.storage().persistent().get(&key)
}

pub fn set_user_streak(env: &Env, user: &Address, streak: &crate::storage::StakeStreak) {
    let key = (Symbol::new(env, "strk"), user.clone());
    env.storage().persistent().set(&key, streak);
}

// ── Issue #135: per-user cumulative claimed counter ───────────────────────────
// Uses a tuple key to avoid exhausting the DataKey enum's contracttype limit.

pub fn get_user_total_claimed(env: &Env, user: &Address) -> i128 {
    let key = (Symbol::new(env, "t_claimed"), user.clone());
    env.storage().persistent().get(&key).unwrap_or(0)
}

// Not currently wired into any claim path; kept for a future accounting feature.
#[allow(dead_code)]
pub fn add_user_total_claimed(env: &Env, user: &Address, amount: i128) {
    let current = get_user_total_claimed(env, user);
    let key = (Symbol::new(env, "t_claimed"), user.clone());
    env.storage().persistent().set(&key, &(current + amount));
}

// ── Issue #114: on-chain admin changelog ─────────────────────────────────────
// Key "chlg" (4 chars, short symbol) stored in instance storage.

pub fn get_changelog(env: &Env) -> Vec<ChangelogEntry> {
    env.storage()
        .instance()
        .get(&symbol_short!("chlg"))
        .unwrap_or(Vec::new(env))
}

pub fn set_changelog(env: &Env, log: &Vec<ChangelogEntry>) {
    env.storage().instance().set(&symbol_short!("chlg"), log);
}

// ── Issue #115: last reward rate change ledger ────────────────────────────────
// Key "lrcl" (4 chars, short symbol) stored in instance storage.

pub fn get_last_rate_change_ledger(env: &Env) -> u32 {
    env.storage()
        .instance()
        .get(&symbol_short!("lrcl"))
        .unwrap_or(0)
}

pub fn set_last_rate_change_ledger(env: &Env, ledger: u32) {
    env.storage()
        .instance()
        .set(&symbol_short!("lrcl"), &ledger);
}

// ── Issue #116: per-user vesting entries ─────────────────────────────────────
// Key ("vest", user) stored in persistent storage (same pattern as streak).

pub fn get_vesting_entries(env: &Env, user: &Address) -> Vec<VestingEntry> {
    let key = (Symbol::new(env, "vest"), user.clone());
    env.storage()
        .persistent()
        .get(&key)
        .unwrap_or(Vec::new(env))
}

pub fn set_vesting_entries(env: &Env, user: &Address, entries: &Vec<VestingEntry>) {
    let key = (Symbol::new(env, "vest"), user.clone());
    env.storage().persistent().set(&key, entries);
}

// ── Issue #117: pool initialization ledger ───────────────────────────────────
// Key "inal" (4 chars, short symbol) stored in instance storage.

pub fn get_initialized_at_ledger(env: &Env) -> Option<u32> {
    env.storage().instance().get(&symbol_short!("inal"))
}

pub fn set_initialized_at_ledger(env: &Env, ledger: u32) {
    env.storage()
        .instance()
        .set(&symbol_short!("inal"), &ledger);
}

// ── Custom error messages (frontend metadata) ────────────────────────────────
// Not currently wired into any public entrypoint; kept for a future feature.

/// Maximum length for custom error messages (150 characters).
#[allow(dead_code)]
pub const MAX_ERROR_MESSAGE_LENGTH: u32 = 150;

/// Maximum number of custom error messages stored (20 messages).
#[allow(dead_code)]
pub const MAX_ERROR_MESSAGES: u32 = 20;

/// Get list of all error codes that have custom messages set.
#[allow(dead_code)]
pub fn get_error_message_codes(env: &Env) -> Vec<u32> {
    let key = symbol_short!("err_codes");
    env.storage()
        .persistent()
        .get(&key)
        .unwrap_or(Vec::new(env))
}

/// Set the list of error codes that have custom messages.
#[allow(dead_code)]
fn set_error_message_codes(env: &Env, codes: &Vec<u32>) {
    let key = symbol_short!("err_codes");
    env.storage().persistent().set(&key, codes);
}

/// Get custom error message for a specific error code.
/// Uses tuple key to avoid DataKey enum limit.
#[allow(dead_code)]
pub fn get_error_message(env: &Env, error_code: u32) -> Option<soroban_sdk::String> {
    let key = (Symbol::new(env, "err_msg"), error_code);
    env.storage().persistent().get(&key)
}

/// Set custom error message for a specific error code.
/// Enforces MAX_ERROR_MESSAGES limit by removing oldest when full.
/// Uses tuple key to avoid DataKey enum limit.
#[allow(dead_code)]
pub fn set_error_message(env: &Env, error_code: u32, message: &soroban_sdk::String) {
    // Store the message using tuple key
    let key = (Symbol::new(env, "err_msg"), error_code);
    env.storage().persistent().set(&key, message);

    // Update the codes list
    let mut codes = get_error_message_codes(env);

    // Check if this error code is already in the list
    let mut found = false;
    for i in 0..codes.len() {
        if codes.get(i).unwrap() == error_code {
            found = true;
            break;
        }
    }

    // If not found, add it to the list
    if !found {
        // If at capacity, remove the oldest (first) entry
        if codes.len() >= MAX_ERROR_MESSAGES {
            let oldest_code = codes.get(0).unwrap();
            let oldest_key = (Symbol::new(env, "err_msg"), oldest_code);
            env.storage().persistent().remove(&oldest_key);
            codes.remove(0);
        }
        codes.push_back(error_code);
        set_error_message_codes(env, &codes);
    }
}

// ── Issue #113: auto-restake toggle ───────────────────────────────────────────
// Key ("auto_rst", user) stored in persistent storage (same pattern as streak).

pub fn get_auto_restake(env: &Env, user: &Address) -> bool {
    let key = (Symbol::new(env, "auto_rst"), user.clone());
    env.storage().persistent().get(&key).unwrap_or(false)
}

pub fn set_auto_restake(env: &Env, user: &Address, enabled: bool) {
    let key = (Symbol::new(env, "auto_rst"), user.clone());
    env.storage().persistent().set(&key, &enabled);
}

// ── Issue #124: rich reward-rate history ─────────────────────────────────────
// Key "rwd_rth" stored in instance storage.

pub const MAX_RICH_RATE_HISTORY: u32 = 20;

pub fn get_reward_rate_history(env: &Env) -> Vec<RateHistoryEntry> {
    env.storage()
        .instance()
        .get(&symbol_short!("rwd_rth"))
        .unwrap_or(Vec::new(env))
}

pub fn set_reward_rate_history(env: &Env, history: &Vec<RateHistoryEntry>) {
    env.storage()
        .instance()
        .set(&symbol_short!("rwd_rth"), history);
}

// ── Referral system ───────────────────────────────────────────────────────────
// Tuple-keyed persistent storage to avoid growing the DataKey enum.

pub fn get_referrer_of(env: &Env, user: &Address) -> Option<Address> {
    let key = (Symbol::new(env, "ref_of"), user.clone());
    env.storage().persistent().get(&key)
}

pub fn set_referrer_of(env: &Env, user: &Address, referrer: &Address) {
    let key = (Symbol::new(env, "ref_of"), user.clone());
    env.storage().persistent().set(&key, referrer);
}

pub fn get_referral_stats(env: &Env, referrer: &Address) -> ReferralStats {
    let key = (Symbol::new(env, "ref_st"), referrer.clone());
    env.storage()
        .persistent()
        .get(&key)
        .unwrap_or(ReferralStats {
            total_referred_stake: 0,
            referral_count: 0,
        })
}

pub fn set_referral_stats(env: &Env, referrer: &Address, stats: &ReferralStats) {
    let key = (Symbol::new(env, "ref_st"), referrer.clone());
    env.storage().persistent().set(&key, stats);
}

pub fn get_referral_registry(env: &Env) -> Vec<Address> {
    env.storage()
        .instance()
        .get(&symbol_short!("ref_reg"))
        .unwrap_or(Vec::new(env))
}

pub fn set_referral_registry(env: &Env, registry: &Vec<Address>) {
    env.storage()
        .instance()
        .set(&symbol_short!("ref_reg"), registry);
}
// ── Issue #118: relayer approval ─────────────────────────────────────────────
pub fn get_approved_relayer(env: &Env, user: &Address) -> Option<Address> {
    let key = (Symbol::new(env, "app_rlyr"), user.clone());
    env.storage().persistent().get(&key)
}

pub fn set_approved_relayer(env: &Env, user: &Address, relayer: &Address) {
    let key = (Symbol::new(env, "app_rlyr"), user.clone());
    env.storage().persistent().set(&key, relayer);
}

pub fn remove_approved_relayer(env: &Env, user: &Address) {
    let key = (Symbol::new(env, "app_rlyr"), user.clone());
    env.storage().persistent().remove(&key);
}

// ── Issue #126: yield source whitelist ───────────────────────────────────────
pub fn is_yield_source(env: &Env, source: &Address) -> bool {
    let key = (Symbol::new(env, "y_source"), source.clone());
    env.storage().persistent().get(&key).unwrap_or(false)
}

pub fn set_yield_source(env: &Env, source: &Address, approved: bool) {
    let key = (Symbol::new(env, "y_source"), source.clone());
    if approved {
        env.storage().persistent().set(&key, &true);
    } else {
        env.storage().persistent().remove(&key);
    }
}

pub fn get_total_rewards_added(env: &Env) -> i128 {
    let key = (Symbol::new(env, "tot_rwds"),);
    env.storage().instance().get(&key).unwrap_or(0)
}

pub fn set_total_rewards_added(env: &Env, total: i128) {
    let key = (Symbol::new(env, "tot_rwds"),);
    env.storage().instance().set(&key, &total);
}

// ── Issue #180: lifetime protocol fee revenue counter ────────────────────────
// Uses a direct symbol key (rather than a `DataKey` variant) because the
// `DataKey` union is already at the 50-case XDR spec limit.

pub fn get_protocol_fee_collected(env: &Env) -> i128 {
    env.storage()
        .instance()
        .get(&symbol_short!("prot_fee"))
        .unwrap_or(0)
}

pub fn add_protocol_fee_collected(env: &Env, amount: i128) {
    let total = get_protocol_fee_collected(env) + amount;
    env.storage()
        .instance()
        .set(&symbol_short!("prot_fee"), &total);
}

// ── Issue #182: off-chain webhook URL config ─────────────────────────────────
// Uses a direct symbol key for the same reason as above.

pub fn get_webhook_url(env: &Env) -> Option<String> {
    env.storage().instance().get(&symbol_short!("whk_url"))
}

pub fn set_webhook_url(env: &Env, url: &String) {
    env.storage().instance().set(&symbol_short!("whk_url"), url);
}

pub fn clear_webhook_url(env: &Env) {
    env.storage().instance().remove(&symbol_short!("whk_url"));
}

// ── Activity heatmap (7-day rolling buckets) ────────────────────────────────

pub fn get_activity_log(env: &Env) -> Vec<DayBucket> {
    env.storage()
        .instance()
        .get(&symbol_short!("act_log"))
        .unwrap_or(Vec::new(env))
}

pub fn set_activity_log(env: &Env, log: &Vec<DayBucket>) {
    env.storage().instance().set(&symbol_short!("act_log"), log);
}

// ── Issue #217: per-user claim history for tax reporting ─────────────────────

pub const MAX_CLAIM_HISTORY: u32 = 100;

pub fn get_claim_history(env: &Env, user: &Address) -> Vec<(u32, i128)> {
    let key = (Symbol::new(env, "clm_hist"), user.clone());
    env.storage()
        .persistent()
        .get(&key)
        .unwrap_or(Vec::new(env))
}

pub fn set_claim_history(env: &Env, user: &Address, history: &Vec<(u32, i128)>) {
    let key = (Symbol::new(env, "clm_hist"), user.clone());
    env.storage().persistent().set(&key, history);
}

// ── Issue #219: pause info ───────────────────────────────────────────────────

pub fn get_pause_info(env: &Env) -> Option<crate::storage::PauseInfo> {
    env.storage().instance().get(&symbol_short!("ps_info"))
}

pub fn set_pause_info(env: &Env, info: &crate::storage::PauseInfo) {
    env.storage()
        .instance()
        .set(&symbol_short!("ps_info"), info);
}

pub fn clear_pause_info(env: &Env) {
    env.storage().instance().remove(&symbol_short!("ps_info"));
}

// ── Issue #218: migration target ─────────────────────────────────────────────

pub fn get_migration_target(env: &Env) -> Option<Address> {
    env.storage().instance().get(&symbol_short!("mig_tgt"))
}

pub fn set_migration_target(env: &Env, target: &Address) {
    env.storage()
        .instance()
        .set(&symbol_short!("mig_tgt"), target);
}

// ── Issue #220: rounding policy ──────────────────────────────────────────────

pub fn get_rounding_policy(env: &Env) -> crate::storage::RoundingPolicy {
    env.storage()
        .instance()
        .get(&symbol_short!("rnd_pol"))
        .unwrap_or(crate::storage::RoundingPolicy::Floor)
}

pub fn set_rounding_policy(env: &Env, policy: &crate::storage::RoundingPolicy) {
    env.storage()
        .instance()
        .set(&symbol_short!("rnd_pol"), policy);
}

// ── Issue #215: yield farming hook ────────────────────────────────────────────

pub fn get_yield_protocol(env: &Env) -> Option<Address> {
    env.storage().instance().get(&symbol_short!("yld_prot"))
}

pub fn set_yield_protocol(env: &Env, protocol: &Address) {
    env.storage()
        .instance()
        .set(&symbol_short!("yld_prot"), protocol);
}

pub fn get_yield_deployed(env: &Env) -> i128 {
    env.storage()
        .instance()
        .get(&symbol_short!("yld_dep"))
        .unwrap_or(0)
}

pub fn set_yield_deployed(env: &Env, amount: i128) {
    env.storage()
        .instance()
        .set(&symbol_short!("yld_dep"), &amount);
}

// ── Issue #216: governance voting ─────────────────────────────────────────────

pub fn get_next_proposal_id(env: &Env) -> u32 {
    env.storage()
        .instance()
        .get(&symbol_short!("prop_nid"))
        .unwrap_or(0)
}

pub fn set_next_proposal_id(env: &Env, id: u32) {
    env.storage()
        .instance()
        .set(&symbol_short!("prop_nid"), &id);
}

pub fn get_open_proposal_count(env: &Env) -> u32 {
    env.storage()
        .instance()
        .get(&symbol_short!("prop_opn"))
        .unwrap_or(0)
}

pub fn set_open_proposal_count(env: &Env, count: u32) {
    env.storage()
        .instance()
        .set(&symbol_short!("prop_opn"), &count);
}

pub fn get_proposal(env: &Env, id: u32) -> Option<GovernanceProposal> {
    let key = (Symbol::new(env, "prop"), id);
    env.storage().persistent().get(&key)
}

pub fn set_proposal(env: &Env, id: u32, proposal: &GovernanceProposal) {
    let key = (Symbol::new(env, "prop"), id);
    env.storage().persistent().set(&key, proposal);
}

pub fn has_voted(env: &Env, proposal_id: u32, voter: &Address) -> bool {
    let key = (Symbol::new(env, "voted"), proposal_id, voter.clone());
    env.storage().persistent().get(&key).unwrap_or(false)
}

pub fn set_voted(env: &Env, proposal_id: u32, voter: &Address) {
    let key = (Symbol::new(env, "voted"), proposal_id, voter.clone());
    env.storage().persistent().set(&key, &true);
}

// ── Issue #206: single-level reward-rate rollback ────────────────────────────
// DataKey is already at Soroban's 50-variant cap (see the note in
// storage.rs), so these use raw Symbol keys instead of a DataKey variant —
// the same workaround this file already uses for symbol_short!("pool_cp"),
// symbol_short!("sl_tr"), etc.

pub fn get_previous_rate(env: &Env) -> Option<u32> {
    env.storage().instance().get(&symbol_short!("prevrate"))
}

pub fn set_previous_rate(env: &Env, rate_bps: u32) {
    env.storage()
        .instance()
        .set(&symbol_short!("prevrate"), &rate_bps);
}

pub fn clear_previous_rate(env: &Env) {
    env.storage().instance().remove(&symbol_short!("prevrate"));
}

// ── Issue #207: cross-chain bridge relayer hook ──────────────────────────────

pub fn get_bridge_packet_sequence(env: &Env) -> u64 {
    env.storage()
        .instance()
        .get(&symbol_short!("brpktseq"))
        .unwrap_or(0)
}

pub fn set_bridge_packet_sequence(env: &Env, seq: u64) {
    env.storage()
        .instance()
        .set(&symbol_short!("brpktseq"), &seq);
}

pub fn is_bridge_enabled(env: &Env) -> bool {
    env.storage()
        .instance()
        .get(&symbol_short!("brenabld"))
        .unwrap_or(false)
}

pub fn set_bridge_enabled(env: &Env, enabled: bool) {
    env.storage()
        .instance()
        .set(&symbol_short!("brenabld"), &enabled);
}

// ── Issue #209: additive split positions ─────────────────────────────────────

pub fn get_split_positions(env: &Env, user: &Address) -> Vec<StakePosition> {
    let key = (Symbol::new(env, "splitpos"), user.clone());
    env.storage()
        .persistent()
        .get(&key)
        .unwrap_or(Vec::new(env))
}

pub fn set_split_positions(env: &Env, user: &Address, positions: &Vec<StakePosition>) {
    let key = (Symbol::new(env, "splitpos"), user.clone());
    env.storage().persistent().set(&key, positions);
}

// ── Issue #205: DEX router used by swap_and_stake ────────────────────────────

pub fn get_dex_router(env: &Env) -> Option<Address> {
    env.storage().instance().get(&symbol_short!("dexroutr"))
}

pub fn set_dex_router(env: &Env, router: &Address) {
    env.storage()
        .instance()
        .set(&symbol_short!("dexroutr"), router);
}

// ── Issue #163: lifetime total-ever-staked counter ───────────────────────────

pub fn get_total_ever_staked(env: &Env) -> i128 {
    env.storage()
        .instance()
        .get(&symbol_short!("everstk"))
        .unwrap_or(0)
}

pub fn set_total_ever_staked(env: &Env, total: i128) {
    env.storage()
        .instance()
        .set(&symbol_short!("everstk"), &total);
}

// ── Issue #197: fee splitting ─────────────────────────────────────────────────

pub fn get_fee_recipients(env: &Env) -> Vec<FeeRecipient> {
    env.storage()
        .instance()
        .get(&symbol_short!("feerecip"))
        .unwrap_or(Vec::new(env))
}

pub fn set_fee_recipients(env: &Env, recipients: &Vec<FeeRecipient>) {
    env.storage()
        .instance()
        .set(&symbol_short!("feerecip"), recipients);
}

// ── Issue #195: timelocked admin actions ─────────────────────────────────────

pub fn get_timelock_delay(env: &Env) -> u32 {
    env.storage()
        .instance()
        .get(&symbol_short!("tlockdly"))
        .unwrap_or(0)
}

pub fn set_timelock_delay(env: &Env, ledgers: u32) {
    env.storage()
        .instance()
        .set(&symbol_short!("tlockdly"), &ledgers);
}

pub fn get_pending_actions(env: &Env) -> Vec<PendingAction> {
    env.storage()
        .instance()
        .get(&symbol_short!("pendacts"))
        .unwrap_or(Vec::new(env))
}

pub fn set_pending_actions(env: &Env, actions: &Vec<PendingAction>) {
    env.storage()
        .instance()
        .set(&symbol_short!("pendacts"), actions);
}

pub fn next_action_id(env: &Env) -> u32 {
    let id: u32 = env
        .storage()
        .instance()
        .get(&symbol_short!("actidctr"))
        .unwrap_or(0);
    env.storage()
        .instance()
        .set(&symbol_short!("actidctr"), &(id + 1));
    id
}

// ── Issue #196: multi-sig admin ──────────────────────────────────────────────

pub fn get_multisig_config(env: &Env) -> Option<MultisigConfig> {
    env.storage().instance().get(&symbol_short!("msigcfg"))
}

pub fn set_multisig_config(env: &Env, config: &MultisigConfig) {
    env.storage()
        .instance()
        .set(&symbol_short!("msigcfg"), config);
}

pub fn get_admin_proposals(env: &Env) -> Vec<AdminProposal> {
    env.storage()
        .instance()
        .get(&symbol_short!("msigprop"))
        .unwrap_or(Vec::new(env))
}

pub fn set_admin_proposals(env: &Env, proposals: &Vec<AdminProposal>) {
    env.storage()
        .instance()
        .set(&symbol_short!("msigprop"), proposals);
}

pub fn next_proposal_id(env: &Env) -> u32 {
    let id: u32 = env
        .storage()
        .instance()
        .get(&symbol_short!("propidct"))
        .unwrap_or(0);
    env.storage()
        .instance()
        .set(&symbol_short!("propidct"), &(id + 1));
    id
}

// ── Missing helper functions for existing vault features ──────────────────────

pub fn get_stake_rate_limit(env: &Env) -> u32 {
    env.storage()
        .instance()
        .get(&symbol_short!("stk_rtl"))
        .unwrap_or(0)
}

pub fn set_stake_rate_limit(env: &Env, limit: u32) {
    env.storage()
        .instance()
        .set(&symbol_short!("stk_rtl"), &limit);
}

pub fn get_last_stake_ledger(env: &Env, user: &Address) -> Option<u32> {
    let key = (Symbol::new(env, "lst_stk"), user.clone());
    env.storage().persistent().get(&key)
}

pub fn set_last_stake_ledger(env: &Env, user: &Address, ledger: u32) {
    let key = (Symbol::new(env, "lst_stk"), user.clone());
    env.storage().persistent().set(&key, &ledger);
}

pub fn get_claim_rate_limit(env: &Env) -> u32 {
    env.storage()
        .instance()
        .get(&symbol_short!("clm_rtl"))
        .unwrap_or(0)
}

pub fn set_claim_rate_limit(env: &Env, limit: u32) {
    env.storage()
        .instance()
        .set(&symbol_short!("clm_rtl"), &limit);
}

pub fn get_last_claim_action_ledger(env: &Env, user: &Address) -> Option<u32> {
    let key = (Symbol::new(env, "lst_clm"), user.clone());
    env.storage().persistent().get(&key)
}

pub fn set_last_claim_action_ledger(env: &Env, user: &Address, ledger: u32) {
    let key = (Symbol::new(env, "lst_clm"), user.clone());
    env.storage().persistent().set(&key, &ledger);
}

// ── Issue #198: penalty redistribution ───────────────────────────────────────

pub fn get_penalty_redistribution_mode(env: &Env) -> bool {
    env.storage()
        .instance()
        .get(&symbol_short!("pen_rdst"))
        .unwrap_or(false)
}

pub fn set_penalty_redistribution_mode(env: &Env, enabled: bool) {
    env.storage()
        .instance()
        .set(&symbol_short!("pen_rdst"), &enabled);
}

// ── Issue #199: insurance fund ────────────────────────────────────────────────

pub fn set_insurance_rate_bps(env: &Env, bps: u32) {
    env.storage()
        .instance()
        .set(&symbol_short!("ins_rate"), &bps);
}

pub fn get_insurance_rate_bps(env: &Env) -> u32 {
    env.storage()
        .instance()
        .get(&symbol_short!("ins_rate"))
        .unwrap_or(0)
}

pub fn get_insurance_fund_balance(env: &Env) -> i128 {
    env.storage()
        .instance()
        .get(&symbol_short!("ins_fund"))
        .unwrap_or(0)
}

pub fn set_insurance_fund_balance(env: &Env, amount: i128) {
    env.storage()
        .instance()
        .set(&symbol_short!("ins_fund"), &amount);
}

// ── Issue #200: delegation chain ──────────────────────────────────────────────

pub fn get_delegation_chain(env: &Env, user: &Address) -> Option<crate::storage::DelegationChain> {
    let key = (Symbol::new(env, "dchain"), user.clone());
    env.storage().persistent().get(&key)
}

pub fn set_delegation_chain(env: &Env, user: &Address, chain: &crate::storage::DelegationChain) {
    let key = (Symbol::new(env, "dchain"), user.clone());
    env.storage().persistent().set(&key, chain);
}

pub fn remove_delegation_chain(env: &Env, user: &Address) {
    let key = (Symbol::new(env, "dchain"), user.clone());
    env.storage().persistent().remove(&key);
}

// ── Issue #202: bootstrap ─────────────────────────────────────────────────────

pub fn set_bootstrap_config(env: &Env, config: &crate::storage::BootstrapConfig) {
    env.storage()
        .instance()
        .set(&symbol_short!("bootcfg"), config);
}

pub fn get_bootstrap_config(env: &Env) -> Option<crate::storage::BootstrapConfig> {
    env.storage().instance().get(&symbol_short!("bootcfg"))
}

pub fn clear_bootstrap_config(env: &Env) {
    env.storage().instance().remove(&symbol_short!("bootcfg"));
}

// ── Issue #213: dynamic fee config ────────────────────────────────────────────

pub fn set_dynamic_fee_config(env: &Env, config: &DynamicFeeConfig) {
    env.storage()
        .instance()
        .set(&symbol_short!("dfeecfg"), config);
}

pub fn get_dynamic_fee_config(env: &Env) -> Option<DynamicFeeConfig> {
    env.storage().instance().get(&symbol_short!("dfeecfg"))
}

// ── Issue #214: user claim count (for reputation consistency score) ────────────

pub fn get_user_claim_count(env: &Env, user: &Address) -> u32 {
    let key = (Symbol::new(env, "clm_cnt"), user.clone());
    env.storage().persistent().get(&key).unwrap_or(0)
}

pub fn increment_user_claim_count(env: &Env, user: &Address) {
    let current = get_user_claim_count(env, user);
    let key = (Symbol::new(env, "clm_cnt"), user.clone());
    env.storage().persistent().set(&key, &(current + 1));
}

// ── Issue #210: Merkle Reward Distribution ────────────────────────────────────

pub fn set_merkle_root(env: &Env, root: &crate::storage::MerkleRoot) {
    env.storage()
        .instance()
        .set(&symbol_short!("merkle"), root);
}

pub fn get_merkle_root(env: &Env) -> Option<crate::storage::MerkleRoot> {
    env.storage().instance().get(&symbol_short!("merkle"))
}

pub fn is_merkle_claimed(env: &Env, user: &Address, epoch: u32) -> bool {
    let key = (Symbol::new(env, "mrk_clm"), user.clone(), epoch);
    env.storage().persistent().get(&key).unwrap_or(false)
}

pub fn set_merkle_claimed(env: &Env, user: &Address, epoch: u32) {
    let key = (Symbol::new(env, "mrk_clm"), user.clone(), epoch);
    env.storage().persistent().set(&key, &true);
}

// ── Issue #211: Staking Tournament Competition ────────────────────────────────

pub fn set_tournament(env: &Env, tournament: &crate::storage::Tournament) {
    env.storage()
        .instance()
        .set(&symbol_short!("tourney"), tournament);
}

pub fn get_tournament(env: &Env) -> Option<crate::storage::Tournament> {
    env.storage().instance().get(&symbol_short!("tourney"))
}

pub fn remove_tournament(env: &Env) {
    env.storage().instance().remove(&symbol_short!("tourney"));
}

pub fn get_tournament_score(env: &Env, user: &Address) -> i128 {
    let key = (Symbol::new(env, "tour_scr"), user.clone());
    env.storage().persistent().get(&key).unwrap_or(0)
}

pub fn set_tournament_score(env: &Env, user: &Address, score: i128) {
    let key = (Symbol::new(env, "tour_scr"), user.clone());
    env.storage().persistent().set(&key, &score);
}

pub fn get_tournament_participants(env: &Env) -> Vec<Address> {
    env.storage()
        .instance()
        .get(&symbol_short!("tour_par"))
        .unwrap_or(Vec::new(env))
}

pub fn set_tournament_participants(env: &Env, participants: &Vec<Address>) {
    env.storage()
        .instance()
        .set(&symbol_short!("tour_par"), participants);
}

pub fn add_tournament_participant(env: &Env, user: &Address) {
    let mut participants = get_tournament_participants(env);
    let mut exists = false;
    for i in 0..participants.len() {
        if participants.get(i).unwrap() == *user {
            exists = true;
            break;
        }
    }
    if !exists {
        participants.push_back(user.clone());
        set_tournament_participants(env, &participants);
    }
}

// ── Issue #212: Buyback & Burn ────────────────────────────────────────────────

pub fn buyback_enabled(env: &Env) -> bool {
    env.storage()
        .instance()
        .get(&symbol_short!("bybk_enb"))
        .unwrap_or(false)
}

pub fn set_buyback_enabled(env: &Env, enabled: bool) {
    env.storage()
        .instance()
        .set(&symbol_short!("bybk_enb"), &enabled);
}

pub fn get_buyback_threshold(env: &Env) -> i128 {
    env.storage()
        .instance()
        .get(&symbol_short!("bybk_thr"))
        .unwrap_or(0)
}

pub fn set_buyback_threshold(env: &Env, amount: i128) {
    env.storage()
        .instance()
        .set(&symbol_short!("bybk_thr"), &amount);
}

pub fn get_total_tokens_burned(env: &Env) -> i128 {
    env.storage()
        .instance()
        .get(&symbol_short!("tot_burn"))
        .unwrap_or(0)
}

pub fn add_tokens_burned(env: &Env, amount: i128) {
    let total = get_total_tokens_burned(env) + amount;
    env.storage()
        .instance()
        .set(&symbol_short!("tot_burn"), &total);
}
