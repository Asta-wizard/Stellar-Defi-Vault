//! Reward vesting cliff (issue #287).
//!
//! A cliff blocks reward *accrual* entirely until a position has been staked
//! for a configurable minimum duration. Once the cliff is reached, rewards
//! unlock all at once and accrue **retroactively from `staked_at_ledger`**, not
//! from the cliff date — so the first claim after the cliff pays out
//! `cliff_ledgers` worth of rewards.
//!
//! This is distinct from the two neighbouring features it is easy to confuse
//! it with:
//!
//! * Issue #35's vesting schedule releases rewards that have *already been
//!   claimed* on a drip. This one decides whether rewards exist at all.
//! * Issue #201's anti-sybil minimum stake age gates *claiming*. This one
//!   gates *accrual*, so a position inside the cliff reports zero pending.
//!
//! # Storage
//!
//! `DataKey` sits at Soroban's 50-variant cap for `#[contracttype]` enums, so
//! this module uses raw `Symbol`-keyed storage, matching the pattern already
//! established in `balance.rs`.

use soroban_sdk::{contractimpl, symbol_short, Address, Env, Symbol};

use crate::admin;
use crate::errors::VaultError;
use crate::storage::DataKey;
use crate::vault::{VaultContract, VaultContractClient};

/// The ledger at which `user` last staked, if they hold a position.
///
/// Read straight from `DataKey::StakedAtLedger` because `balance.rs` exposes
/// no accessor for it — the same way `vault.rs` reads it.
fn staked_at_ledger(env: &Env, user: &Address) -> Option<u32> {
    env.storage()
        .persistent()
        .get::<_, u32>(&DataKey::StakedAtLedger(user.clone()))
}

/// Instance-storage key for the configured cliff length, in ledgers.
const CLIFF_KEY: Symbol = symbol_short!("vst_clff");

/// Persistent-storage key prefix recording that a user's cliff unlock has
/// already been announced, so the event fires exactly once per position.
const CLIFF_EVENT_KEY: Symbol = symbol_short!("clf_evt");

/// Read the configured cliff length. `0` means no cliff, which is the default
/// and preserves the pre-#287 behaviour for every existing pool.
pub fn get_cliff_ledgers(env: &Env) -> u32 {
    env.storage().instance().get(&CLIFF_KEY).unwrap_or(0)
}

/// The ledger at which `user`'s position clears the cliff.
///
/// Returns `None` when the user has no recorded stake — there is nothing to
/// unlock — and the stake ledger itself when no cliff is configured.
pub fn cliff_unlock_ledger_for(env: &Env, user: &Address) -> Option<u32> {
    let staked_at = staked_at_ledger(env, user)?;
    Some(staked_at.saturating_add(get_cliff_ledgers(env)))
}

/// Whether `user`'s position has cleared the cliff.
///
/// A position with no recorded stake ledger is treated as past the cliff: it
/// has no rewards to gate, and reporting `false` would make an unstaked
/// address look permanently locked.
pub fn is_past_cliff_for(env: &Env, user: &Address) -> bool {
    match cliff_unlock_ledger_for(env, user) {
        Some(unlock_at) => env.ledger().sequence() >= unlock_at,
        None => true,
    }
}

/// Zero out a reward figure while the position is inside its cliff.
///
/// This is the single chokepoint the reward path calls, so accrual and every
/// read-only query agree by construction rather than by both remembering to
/// apply the same rule.
pub fn apply_cliff(env: &Env, user: &Address, reward: i128) -> i128 {
    if is_past_cliff_for(env, user) {
        reward
    } else {
        0
    }
}

/// Emit `cliff_unlocked` the first time `user` interacts after clearing the
/// cliff, then remember that it fired.
///
/// Emitted lazily rather than on a timer because a contract cannot wake itself
/// at a ledger height; the alternative would be no event at all.
pub fn maybe_emit_cliff_unlocked(env: &Env, user: &Address, accrued_since: i128) {
    if get_cliff_ledgers(env) == 0 || !is_past_cliff_for(env, user) {
        return;
    }

    let key = (CLIFF_EVENT_KEY, user.clone());
    if env.storage().persistent().has(&key) {
        return;
    }
    env.storage().persistent().set(&key, &true);

    env.events().publish(
        (symbol_short!("cliff_ulk"), user.clone()),
        (accrued_since, env.ledger().sequence()),
    );
}

/// Clear the "already announced" marker so a fresh stake gets a fresh cliff.
///
/// Called from the staking path: re-staking restarts `staked_at_ledger`, so
/// the position re-enters its cliff and must be able to announce unlocking
/// again.
pub fn reset_cliff_marker(env: &Env, user: &Address) {
    env.storage()
        .persistent()
        .remove(&(CLIFF_EVENT_KEY, user.clone()));
}

#[contractimpl]
impl VaultContract {
    /// Set the reward vesting cliff, in ledgers. `0` disables it.
    ///
    /// Admin only. Takes effect immediately for every position, including ones
    /// already staked: the cliff is evaluated against `staked_at_ledger` on
    /// each read rather than snapshotted at stake time, so lengthening it can
    /// pull a position back under its cliff. That is deliberate — the
    /// alternative is a per-position copy that an admin cannot correct.
    pub fn set_vesting_cliff(env: Env, cliff_ledgers: u32) -> Result<(), VaultError> {
        admin::require_admin(&env)?;

        env.storage().instance().set(&CLIFF_KEY, &cliff_ledgers);

        let admin = admin::get_admin(&env)?;
        env.events().publish(
            (symbol_short!("clf_set"), admin),
            (cliff_ledgers, env.ledger().sequence()),
        );
        Ok(())
    }

    /// The configured cliff length in ledgers. `0` means no cliff.
    pub fn get_vesting_cliff(env: Env) -> u32 {
        get_cliff_ledgers(&env)
    }

    /// Whether `user`'s position has cleared its cliff.
    pub fn is_past_cliff(env: Env, user: Address) -> bool {
        is_past_cliff_for(&env, &user)
    }

    /// The ledger at which `user`'s position clears the cliff
    /// (`staked_at_ledger + cliff_ledgers`).
    ///
    /// Returns `0` when the user has no recorded stake.
    pub fn cliff_unlock_ledger(env: Env, user: Address) -> u32 {
        cliff_unlock_ledger_for(&env, &user).unwrap_or(0)
    }
}
