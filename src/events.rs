use crate::storage::AdminAction;
use soroban_sdk::{symbol_short, Address, Env};

pub fn deposit(env: &Env, depositor: &Address, amount: i128, shares_minted: i128, ledger: u32) {
    let topics = (symbol_short!("deposit"), depositor);
    env.events()
        .publish(topics, (amount, shares_minted, ledger));
}

pub fn withdraw(
    env: &Env,
    withdrawer: &Address,
    shares_burned: i128,
    amount_returned: i128,
    ledger: u32,
) {
    let topics = (symbol_short!("withdraw"), withdrawer);
    env.events()
        .publish(topics, (shares_burned, amount_returned, ledger));
}

pub fn unpaused(env: &Env, admin: &Address, ledger: u32) {
    let topics = (symbol_short!("unpaused"), admin);
    env.events().publish(topics, (ledger,));
}

pub fn yield_added(env: &Env, admin: &Address, amount: i128) {
    let topics = (symbol_short!("yield_add"), admin);
    env.events()
        .publish(topics, (amount, env.ledger().sequence()));
}

pub fn admin_changed(env: &Env, old_admin: &Address, new_admin: &Address) {
    let topics = (symbol_short!("admin_set"), old_admin);
    env.events()
        .publish(topics, (new_admin, env.ledger().sequence()));
}

pub fn withdrawal_limit_updated(env: &Env, admin: &Address, new_limit: i128) {
    let topics = (symbol_short!("wd_limit"), admin);
    env.events()
        .publish(topics, (new_limit, env.ledger().sequence()));
}

/// Issue #202: emitted by `start_bootstrap()`.
pub fn bootstrap_started(
    env: &Env,
    initial_rate_bps: u32,
    base_rate_bps: u32,
    duration_ledgers: u32,
) {
    let topics = (symbol_short!("boot_str"),);
    env.events().publish(
        topics,
        (
            initial_rate_bps,
            base_rate_bps,
            duration_ledgers,
            env.ledger().sequence(),
        ),
    );
}

/// Issue #202: emitted the first time any call notices the bootstrap period
/// has elapsed and settles the rate to `base_rate_bps` permanently.
pub fn bootstrap_ended(env: &Env, base_rate_bps: u32) {
    let topics = (symbol_short!("boot_end"),);
    env.events()
        .publish(topics, (base_rate_bps, env.ledger().sequence()));
}

/// Issue #197: emitted once per recipient during a fee split.
pub fn fee_distributed(env: &Env, recipient: &Address, amount: i128, ledger: u32) {
    let topics = (symbol_short!("fee_dist"), recipient);
    env.events().publish(topics, (amount, ledger));
}

/// Issue #195: emitted by `queue_action()`.
pub fn action_queued(env: &Env, action_id: u32, executable_at: u32) {
    let topics = (symbol_short!("act_queue"),);
    env.events()
        .publish(topics, (action_id, executable_at, env.ledger().sequence()));
}

/// Issue #195: emitted by `execute_action()`.
pub fn action_executed(env: &Env, action_id: u32) {
    let topics = (symbol_short!("act_exec"),);
    env.events()
        .publish(topics, (action_id, env.ledger().sequence()));
}

pub fn rate_changed(env: &Env, old_rate_bps: u32, new_rate_bps: u32) {
    let topics = (symbol_short!("rate_chg"),);
    env.events().publish(
        topics,
        (old_rate_bps, new_rate_bps, env.ledger().sequence()),
    );
}

/// Issue #206: emitted by `rollback_last_rate_change()`.
pub fn rate_rolled_back(env: &Env, restored_rate: u32, discarded_rate: u32) {
    let topics = (symbol_short!("rate_rbk"),);
    env.events().publish(
        topics,
        (restored_rate, discarded_rate, env.ledger().sequence()),
    );
}

/// Issue #207: emitted alongside `stake`/`unstake` for cross-chain bridge
/// relayers, when bridge event emission is enabled.
pub fn bridge_packet(env: &Env, packet: &crate::storage::BridgePacket) {
    let topics = (symbol_short!("bridge_pk"), packet.source_address.clone());
    env.events().publish(
        topics,
        (
            packet.sequence,
            packet.amount,
            packet.action.clone(),
            packet.timestamp_ledger,
        ),
    );
}

/// Issue #209: emitted by `position_split()`.
pub fn position_split(env: &Env, user: &Address, original_amount: i128, split_amount: i128) {
    let topics = (symbol_short!("pos_split"), user);
    env.events().publish(
        topics,
        (original_amount, split_amount, env.ledger().sequence()),
    );
}

/// Issue #205: emitted by `swap_and_stake()` after the swap and stake both
/// succeed.
pub fn swapped_and_staked(
    env: &Env,
    user: &Address,
    input_token: &Address,
    input_amount: i128,
    stake_amount_received: i128,
) {
    let topics = (symbol_short!("swap_stk"), user);
    env.events().publish(
        topics,
        (
            input_token,
            input_amount,
            stake_amount_received,
            env.ledger().sequence(),
        ),
    );
}

pub fn pool_cap_updated(env: &Env, admin: &Address, new_cap: i128) {
    let topics = (symbol_short!("cap_upd"), admin);
    env.events()
        .publish(topics, (new_cap, env.ledger().sequence()));
}

pub fn position_opened(env: &Env, user: &Address, amount: i128) {
    let topics = (symbol_short!("pos_open"), user);
    env.events()
        .publish(topics, (amount, env.ledger().sequence()));
}

pub fn position_closed(env: &Env, user: &Address) {
    let topics = (symbol_short!("pos_clos"), user);
    env.events().publish(topics, (env.ledger().sequence(),));
}

// ── Issue #39: rescue token event ────────────────────────────────────────────

pub fn token_rescued(env: &Env, token: &Address, amount: i128, recipient: &Address) {
    let topics = (symbol_short!("tk_rescue"),);
    env.events().publish(
        topics,
        (
            token.clone(),
            amount,
            recipient.clone(),
            env.ledger().sequence(),
        ),
    );
}

// ── Issue #42: admin action audit events ─────────────────────────────────────

pub fn admin_action_set_reward_rate(env: &Env, actor: &Address, old_rate: u32, new_rate: u32) {
    env.events().publish(
        (symbol_short!("adm_act"), AdminAction::SetRewardRate),
        (actor.clone(), env.ledger().sequence(), old_rate, new_rate),
    );
}

pub fn admin_action_pause(env: &Env, actor: &Address) {
    env.events().publish(
        (symbol_short!("adm_act"), AdminAction::Pause),
        (actor.clone(), env.ledger().sequence()),
    );
}

pub fn admin_action_unpause(env: &Env, actor: &Address) {
    env.events().publish(
        (symbol_short!("adm_act"), AdminAction::Unpause),
        (actor.clone(), env.ledger().sequence()),
    );
}

pub fn admin_action_transfer_admin(env: &Env, actor: &Address, new_admin: &Address) {
    env.events().publish(
        (symbol_short!("adm_act"), AdminAction::TransferAdmin),
        (actor.clone(), env.ledger().sequence(), new_admin.clone()),
    );
}

pub fn admin_action_set_lock_period(env: &Env, actor: &Address, new_ledgers: u32) {
    env.events().publish(
        (symbol_short!("adm_act"), AdminAction::SetLockPeriod),
        (actor.clone(), env.ledger().sequence(), new_ledgers),
    );
}

pub fn admin_action_set_cap(env: &Env, actor: &Address, new_limit: i128) {
    env.events().publish(
        (symbol_short!("adm_act"), AdminAction::SetCap),
        (actor.clone(), env.ledger().sequence(), new_limit),
    );
}

pub fn admin_action_rescue_token(
    env: &Env,
    actor: &Address,
    token: &Address,
    amount: i128,
    recipient: &Address,
) {
    env.events().publish(
        (symbol_short!("adm_act"), AdminAction::RescueToken),
        (
            actor.clone(),
            env.ledger().sequence(),
            token.clone(),
            amount,
            recipient.clone(),
        ),
    );
}

pub fn admin_action_set_early_exit_penalty(env: &Env, actor: &Address, new_bps: u32) {
    env.events().publish(
        (symbol_short!("adm_act"), AdminAction::SetEarlyExitPenalty),
        (actor.clone(), env.ledger().sequence(), new_bps),
    );
}

pub fn admin_action_set_min_stake(env: &Env, actor: &Address, new_amount: i128) {
    env.events().publish(
        (symbol_short!("adm_act"), AdminAction::SetMinStake),
        (actor.clone(), env.ledger().sequence(), new_amount),
    );
}

pub fn admin_action_fund_reward_pool(env: &Env, actor: &Address, amount: i128) {
    env.events().publish(
        (symbol_short!("adm_act"), AdminAction::FundRewardPool),
        (actor.clone(), env.ledger().sequence(), amount),
    );
}

// ── Emergency Admin Management Events ────────────────────────────────────────

pub fn emergency_admin_set(env: &Env, new_emergency_admin: &Address) {
    let topics = (symbol_short!("emer_set"),);
    env.events().publish(
        topics,
        (new_emergency_admin.clone(), env.ledger().sequence()),
    );
}

pub fn emergency_admin_revoked(env: &Env) {
    let topics = (symbol_short!("emer_rvk"),);
    env.events().publish(topics, (env.ledger().sequence(),));
}

// ── Staker Waitlist Management Notifications ─────────────────────────────────

/// Emitted by `unstake` when capacity clears up to notify the next in queue (Acceptance Criteria #6).
pub fn capacity_available(env: &Env, freed_amount: i128, next_in_queue_address: &Address) {
    let topics = (symbol_short!("cap_av"),);
    env.events().publish(
        topics,
        (freed_amount, next_in_queue_address.clone(), env.ledger().sequence()),
    );
}
