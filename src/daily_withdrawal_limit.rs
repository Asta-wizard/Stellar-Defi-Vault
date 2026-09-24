//! Per-user rolling 24-hour withdrawal limit (issue #554).
//!
//! Complements (does not replace) the pool-wide single-transaction circuit
//! breaker: this caps the *cumulative* token amount one user can withdraw
//! within any rolling `LEDGERS_PER_DAY` window, limiting the damage a
//! compromised user key can do without affecting normal usage.
//!
//! - `set_daily_withdrawal_limit(admin, amount)` — admin-only; `0` disables.
//! - Every withdrawal path (`withdraw`, `unstake`, `unstake_all`) goes through
//!   `VaultContract::do_unstake`, which calls [`enforce_and_record`].
//! - `get_remaining_daily_limit(user)` — read-only headroom query.
//!
//! # Storage
//!
//! `DataKey` is at Soroban's 50-variant cap, so this uses raw `Symbol`-keyed
//! storage like `minimum_unstake_amount.rs`. Each user's recent withdrawals
//! are kept as `(ledger, amount)` entries; entries older than the window are
//! pruned on every read, so the log only ever holds the last 24 hours.
//!
//! # Error code
//!
//! `VaultError` is at the 50-variant cap, so `DailyLimitExceeded` lives in
//! `VaultCampaignError` with code 51 — outside `VaultError`'s 1..=50 range, so
//! a failed `withdraw`/`unstake` can't be misread as an unrelated
//! `VaultError` variant with the same number.

use soroban_sdk::{contractimpl, panic_with_error, symbol_short, Address, Env, Symbol, Vec};

use crate::admin;
use crate::errors::VaultCampaignError;
use crate::vault::{VaultContractClient, LEDGERS_PER_DAY};
use crate::VaultContract;

const DAILY_LIMIT_KEY: Symbol = symbol_short!("dwl_cap");
const WITHDRAW_LOG_KEY: Symbol = symbol_short!("dwl_log");

/// Configured per-user daily cap in token units (0 = disabled).
pub fn get_daily_limit(env: &Env) -> i128 {
    env.storage().instance().get(&DAILY_LIMIT_KEY).unwrap_or(0)
}

fn log_key(user: &Address) -> (Symbol, Address) {
    (WITHDRAW_LOG_KEY, user.clone())
}

/// The user's withdrawals still inside the rolling window, and their total.
fn recent_withdrawals(env: &Env, user: &Address) -> (Vec<(u32, i128)>, i128) {
    let now = env.ledger().sequence();
    let log: Vec<(u32, i128)> = env
        .storage()
        .persistent()
        .get(&log_key(user))
        .unwrap_or_else(|| Vec::new(env));

    let mut kept = Vec::new(env);
    let mut total: i128 = 0;
    for (ledger, amount) in log.iter() {
        // Ages out once a full LEDGERS_PER_DAY has elapsed since it happened.
        if now.saturating_sub(ledger) < LEDGERS_PER_DAY {
            kept.push_back((ledger, amount));
            total = total.saturating_add(amount);
        }
    }
    (kept, total)
}

/// Reject `amount` with `DailyLimitExceeded` if it would push the user's
/// rolling-window total above the cap; otherwise record it. No-op when the
/// limit is disabled.
pub fn enforce_and_record(env: &Env, user: &Address, amount: i128) {
    let cap = get_daily_limit(env);
    if cap <= 0 {
        return;
    }

    let (mut kept, used) = recent_withdrawals(env, user);
    if used.saturating_add(amount) > cap {
        panic_with_error!(env, VaultCampaignError::DailyLimitExceeded);
    }

    kept.push_back((env.ledger().sequence(), amount));
    let key = log_key(user);
    env.storage().persistent().set(&key, &kept);
    env.storage()
        .persistent()
        .extend_ttl(&key, LEDGERS_PER_DAY, LEDGERS_PER_DAY * 2);
}

#[contractimpl]
impl VaultContract {
    /// Admin-only: set the per-user rolling 24-hour withdrawal cap, in stake
    /// token units. `0` disables the limit; negative values are rejected.
    pub fn set_daily_withdrawal_limit(
        env: Env,
        admin_addr: Address,
        amount: i128,
    ) -> Result<(), VaultCampaignError> {
        admin_addr.require_auth();
        if admin_addr != admin::get_admin(&env)? {
            return Err(VaultCampaignError::Unauthorized);
        }
        if amount < 0 {
            return Err(VaultCampaignError::InvalidDailyLimit);
        }
        env.storage().instance().set(&DAILY_LIMIT_KEY, &amount);
        env.events()
            .publish((symbol_short!("dwl_set"),), amount);
        Ok(())
    }

    /// Current per-user daily withdrawal cap (0 = disabled).
    pub fn get_daily_withdrawal_limit(env: Env) -> i128 {
        get_daily_limit(&env)
    }

    /// How much more `user` can withdraw right now without exceeding the
    /// rolling 24-hour cap. Returns `i128::MAX` when the limit is disabled.
    pub fn get_remaining_daily_limit(env: Env, user: Address) -> i128 {
        let cap = get_daily_limit(&env);
        if cap <= 0 {
            return i128::MAX;
        }
        let (_, used) = recent_withdrawals(&env, &user);
        cap.saturating_sub(used).max(0)
    }
}
