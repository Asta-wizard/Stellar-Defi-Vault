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

pub fn admin_action_add_yield(env: &Env, actor: &Address, amount: i128) {
    env.events().publish(
        (symbol_short!("adm_act"), AdminAction::AddYield),
        (actor.clone(), env.ledger().sequence(), amount),
    );
}

pub fn admin_action_set_boost_schedule(env: &Env, actor: &Address, num_tiers: u32) {
    env.events().publish(
        (symbol_short!("adm_act"), AdminAction::SetBoostSchedule),
        (actor.clone(), env.ledger().sequence(), num_tiers),
    );
}

pub fn admin_action_set_nft_contract(env: &Env, actor: &Address, nft_addr: &Address) {
    env.events().publish(
        (symbol_short!("adm_act"), AdminAction::SetNftContract),
        (actor.clone(), env.ledger().sequence(), nft_addr.clone()),
    );
}

pub fn admin_action_set_restake_window(env: &Env, actor: &Address, window: u32) {
    env.events().publish(
        (symbol_short!("adm_act"), AdminAction::SetRestakeWindow),
        (actor.clone(), env.ledger().sequence(), window),
    );
}

pub fn admin_action_set_reward_token(env: &Env, actor: &Address, token: &Address) {
    env.events().publish(
        (symbol_short!("adm_act"), AdminAction::SetRewardToken),
        (actor.clone(), env.ledger().sequence(), token.clone()),
    );
}

pub fn slash(env: &Env, admin: &Address, user: &Address, amount: i128) {
    let topics = (symbol_short!("slash"), admin);
    env.events()
        .publish(topics, (user.clone(), amount, env.ledger().sequence()));
}

pub fn position_transferred(env: &Env, from: &Address, to: &Address, amount: i128) {
    let topics = (symbol_short!("pos_xfer"), from);
    env.events()
        .publish(topics, (to.clone(), amount, env.ledger().sequence()));
}

/// Emitted by `transfer_position_with_rewards` (issue #134).
///
/// `pending_reward_estimate` is the pending reward computed at transfer time —
/// it is informational only; no settlement occurs.
pub fn position_transferred_with_rewards(
    env: &Env,
    from: &Address,
    to: &Address,
    amount: i128,
    pending_reward_estimate: i128,
    ledger: u32,
) {
    let topics = (symbol_short!("pos_xf_rw"), from);
    env.events().publish(
        topics,
        (to.clone(), amount, pending_reward_estimate, ledger),
    );
}

pub fn campaign_started(env: &Env, admin: &Address, multiplier_bps: u32, ends_at_ledger: u32) {
    let topics = (symbol_short!("camp_on"), admin);
    env.events().publish(
        topics,
        (multiplier_bps, ends_at_ledger, env.ledger().sequence()),
    );
}

pub fn campaign_ended(env: &Env, admin: &Address) {
    let topics = (symbol_short!("camp_off"), admin);
    env.events().publish(topics, (env.ledger().sequence(),));
}

/// Emitted when accrued rewards are automatically compounded back into stake.
pub fn auto_restaked(env: &Env, user: &Address, amount: i128) {
    let topics = (symbol_short!("auto_rst"), user);
    env.events()
        .publish(topics, (amount, env.ledger().sequence()));
}

/// Emitted when the contract automatically pauses because reward funding dropped too low.
pub fn auto_paused(env: &Env, reward_balance: i128, threshold: i128) {
    let topics = (symbol_short!("auto_ps"),);
    env.events()
        .publish(topics, (reward_balance, threshold, env.ledger().sequence()));
}

/// Emitted when a user claims staking rewards (via `claim` or `stake_and_claim`).
pub fn claimed(env: &Env, user: &Address, reward: i128, ledger: u32) {
    let topics = (symbol_short!("claimed"), user);
    env.events().publish(topics, (reward, ledger));
}

/// Emitted by `initialize` so indexers can detect new pool deployments on-chain.
pub fn pool_initialized(
    env: &Env,
    admin: &Address,
    stake_token: &Address,
    reward_token: &Address,
    reward_rate_bps: u32,
) {
    let topics = (symbol_short!("init"),);
    env.events().publish(
        topics,
        (
            admin.clone(),
            stake_token.clone(),
            reward_token.clone(),
            reward_rate_bps,
        ),
    );
}

// ── Issue #101: frozen position event ─────────────────────────────────────────

/// Emitted when an admin flags a user's position as frozen via `flag_frozen`.
pub fn frozen_position(env: &Env, admin: &Address, user: &Address, frozen_at: u32) {
    let topics = (symbol_short!("frz_pos"), admin);
    env.events()
        .publish(topics, (user.clone(), frozen_at, env.ledger().sequence()));
}

// ── Issue #107: emergency stop event ─────────────────────────────────────────

/// Emitted once when `emergency_stop` is triggered. This event is permanent
/// and irreversible — the contract will never accept new stakes again.
pub fn stopped(env: &Env, admin: &Address) {
    let topics = (symbol_short!("stopped"), admin);
    env.events().publish(topics, env.ledger().sequence());
}

/// Emitted once when `start_graceful_shutdown` is triggered. New stakes are
/// permanently blocked; existing stakers can still unstake, claim, and
/// withdraw vested rewards normally.
pub fn shutdown_started(env: &Env, admin: &Address) {
    let topics = (symbol_short!("shutdown"), admin);
    env.events().publish(topics, env.ledger().sequence());
}

// ── Issue #97: pool description event ────────────────────────────────────────

/// Emitted when the admin updates the pool description.
pub fn description_updated(env: &Env, admin: &Address, description: &soroban_sdk::String) {
    let topics = (symbol_short!("desc_upd"), admin);
    env.events()
        .publish(topics, (description.clone(), env.ledger().sequence()));
}

/// Emitted when a user merges their staking positions.
pub fn positions_merged(env: &Env, user: &Address, count_merged: u32, total_amount: i128) {
    let topics = (symbol_short!("merge"), user);
    env.events().publish(
        topics,
        (count_merged, total_amount, env.ledger().sequence()),
    );
}

// ── Issue #118: relayer approval events ───────────────────────────────────────

/// Emitted when a user approves a relayer.
pub fn relayer_approved(env: &Env, user: &Address, relayer: &Address) {
    let topics = (symbol_short!("rlyr_set"), user);
    env.events()
        .publish(topics, (relayer.clone(), env.ledger().sequence()));
}

/// Emitted when a user revokes a relayer.
pub fn relayer_revoked(env: &Env, user: &Address, relayer: &Address) {
    let topics = (symbol_short!("rlyr_rev"), user);
    env.events()
        .publish(topics, (relayer.clone(), env.ledger().sequence()));
}

/// Emitted when a relayer claims rewards on behalf of a user.
pub fn claimed_on_behalf(env: &Env, relayer: &Address, user: &Address, reward: i128) {
    let topics = (symbol_short!("claimed"), user);
    env.events()
        .publish(topics, (reward, relayer.clone(), env.ledger().sequence()));
}

// ── Issue #126: yield source events ───────────────────────────────────────────

/// Emitted when a yield source notifies the contract of a new reward deposit.
pub fn reward_added(env: &Env, source: &Address, amount: i128) {
    let topics = (symbol_short!("rwd_add"), source);
    env.events()
        .publish(topics, (amount, env.ledger().sequence()));
}

/// Emitted when admin adds an address to the yield source whitelist.
pub fn yield_source_added(env: &Env, admin: &Address, source: &Address) {
    let topics = (symbol_short!("ys_add"), admin);
    env.events()
        .publish(topics, (source.clone(), env.ledger().sequence()));
}

/// Emitted when admin removes an address from the yield source whitelist.
pub fn yield_source_removed(env: &Env, admin: &Address, source: &Address) {
    let topics = (symbol_short!("ys_rem"), admin);
    env.events()
        .publish(topics, (source.clone(), env.ledger().sequence()));
}

// ── Issue #157: pool name events ──────────────────────────────────────────────

/// Emitted when the admin sets or updates the pool name via `set_pool_name`.
// `set_pool_name` itself is not currently wired up; kept for a future feature.
#[allow(dead_code)]
pub fn pool_name_updated(env: &Env, admin: &Address, name: &soroban_sdk::String) {
    let topics = (symbol_short!("nm_upd"), admin);
    env.events()
        .publish(topics, (name.clone(), env.ledger().sequence()));
}

// ── Reward refill alert ───────────────────────────────────────────────────────

/// Emitted when the reward pool runway drops below 30 days after a claim.
///
/// Topics: `("rfil_alt",)`.
/// Data: `(reward_balance: i128, ledgers_until_empty: u32, ledger: u32)`.
pub fn refill_alert(env: &Env, reward_balance: i128, ledgers_until_empty: u32, ledger: u32) {
    let topics = (symbol_short!("rfil_alt"),);
    env.events()
        .publish(topics, (reward_balance, ledgers_until_empty, ledger));
}

/// Emitted when the admin sets or updates the emergency contact string.
pub fn emergency_contact_updated(env: &Env, admin: &Address, contact: &soroban_sdk::String) {
    let topics = (symbol_short!("emg_cnt"), admin);
    env.events()
        .publish(topics, (contact.clone(), env.ledger().sequence()));
}

// ── Issue #219: pause with reason ────────────────────────────────────────────

/// Emitted when the admin pauses the pool with a reason and message.
pub fn paused_with_reason(
    env: &Env,
    admin: &Address,
    reason: &crate::storage::PauseReason,
    message: &soroban_sdk::String,
    ledger: u32,
) {
    let topics = (symbol_short!("paused"), admin);
    env.events()
        .publish(topics, (*reason, message.clone(), ledger));
}

// ── Issue #218: pool-to-pool migration ──────────────────────────────────────

/// Emitted when a user migrates their position to a target pool.
pub fn migrated(
    env: &Env,
    user: &Address,
    amount: i128,
    rewards_settled: i128,
    target_pool: &Address,
    ledger: u32,
) {
    let topics = (symbol_short!("migrated"), user);
    env.events().publish(
        topics,
        (amount, rewards_settled, target_pool.clone(), ledger),
    );
}

/// Emitted when the migration target pool is set.
pub fn migration_target_set(env: &Env, admin: &Address, target_pool: &Address, ledger: u32) {
    let topics = (symbol_short!("mig_tgt"), admin);
    env.events().publish(topics, (target_pool.clone(), ledger));
}

// ── Issue #215: yield farming hook ────────────────────────────────────────────

/// Emitted when the admin registers the external yield protocol.
pub fn yield_protocol_set(env: &Env, admin: &Address, protocol: &Address) {
    let topics = (symbol_short!("yld_p_set"), admin);
    env.events()
        .publish(topics, (protocol.clone(), env.ledger().sequence()));
}

/// Emitted when idle stake tokens are deployed to the yield protocol.
pub fn yield_deployed(env: &Env, admin: &Address, amount: i128, ledger: u32) {
    let topics = (symbol_short!("yld_dep"), admin);
    env.events().publish(topics, (amount, ledger));
}

/// Emitted when tokens (plus any accrued yield) are withdrawn from the yield protocol.
pub fn yield_withdrawn(env: &Env, admin: &Address, amount_returned: i128, ledger: u32) {
    let topics = (symbol_short!("yld_wdrw"), admin);
    env.events().publish(topics, (amount_returned, ledger));
}

// ── Issue #216: governance voting ─────────────────────────────────────────────

/// Emitted when a staker creates a new governance proposal.
pub fn proposal_created(env: &Env, proposer: &Address, proposal_id: u32, ends_at: u32) {
    let topics = (symbol_short!("prop_new"), proposer);
    env.events().publish(topics, (proposal_id, ends_at));
}

/// Emitted when a staker votes on a governance proposal.
pub fn proposal_voted(
    env: &Env,
    voter: &Address,
    proposal_id: u32,
    support: bool,
    weight: i128,
    ledger: u32,
) {
    let topics = (symbol_short!("prop_vote"), voter);
    env.events()
        .publish(topics, (proposal_id, support, weight, ledger));
}

/// Emitted when a governance proposal is enacted (whether or not it passed).
///
/// Topics: `("prop_enct",)`.
/// Data: `(id, parameter, new_value, total_votes, ledger)`.
pub fn proposal_enacted(
    env: &Env,
    id: u32,
    parameter: crate::storage::ProposableParam,
    new_value: i128,
    total_votes: i128,
    ledger: u32,
) {
    let topics = (symbol_short!("prop_enct"),);
    env.events()
        .publish(topics, (id, parameter, new_value, total_votes, ledger));
}

// ── Issue #182: webhook URL config ───────────────────────────────────────────

/// Emitted when the admin sets or clears the stake-event webhook URL.
///
/// `url` is `None` when the webhook has just been cleared.
pub fn webhook_url_updated(
    env: &Env,
    admin: &Address,
    url: &Option<soroban_sdk::String>,
    ledger: u32,
) {
    let topics = (symbol_short!("whk_url"), admin);
    env.events().publish(topics, (url.clone(), ledger));
}

// ── Issue #210: Merkle Reward Distribution ────────────────────────────────────

pub fn merkle_claimed(env: &Env, user: &Address, amount: i128, epoch: u32, ledger: u32) {
    let topics = (symbol_short!("mrk_clm"), user);
    env.events().publish(topics, (amount, epoch, ledger));
}

// ── Issue #211: Staking Tournament Competition ─────────────────────────────────

pub fn tournament_winner(env: &Env, winner: &Address, score: i128, prize_paid: i128, ledger: u32) {
    let topics = (symbol_short!("tour_win"),);
    env.events()
        .publish(topics, (winner, score, prize_paid, ledger));
}

// ── Issue #212: Buyback & Burn ────────────────────────────────────────────────

pub fn buyback_executed(env: &Env, fee_amount_used: i128, reward_tokens_burned: i128, ledger: u32) {
    let topics = (symbol_short!("buyback"),);
    env.events()
        .publish(topics, (fee_amount_used, reward_tokens_burned, ledger));
}

pub fn buyback_toggled(env: &Env, admin: &Address, enabled: bool) {
    let topics = (symbol_short!("buy_tog"), admin);
    env.events()
        .publish(topics, (enabled, env.ledger().sequence()));
}

pub fn merkle_root_published(env: &Env, admin: &Address, epoch: u32, total_claimable: i128) {
    let topics = (symbol_short!("mrk_pub"), admin);
    env.events()
        .publish(topics, (epoch, total_claimable, env.ledger().sequence()));
}

// ── Events referenced by existing feature branches ────────────────────────────

pub fn state_exported(env: &Env, admin: &Address, num_positions: u32, ledger: u32) {
    let topics = (symbol_short!("st_exp"), admin);
    env.events().publish(topics, (num_positions, ledger));
}

pub fn penalty_redistributed(env: &Env, total_amount: i128, recipient_count: u32, ledger: u32) {
    let topics = (symbol_short!("pen_rdst"),);
    env.events()
        .publish(topics, (total_amount, recipient_count, ledger));
}

pub fn insurance_deployed(env: &Env, amount: i128, new_balance: i128) {
    let topics = (symbol_short!("ins_dep"),);
    env.events()
        .publish(topics, (amount, new_balance, env.ledger().sequence()));
}

// ── Issue #231: halving occurred ──────────────────────────────────────────────

/// Emitted when a reward calculation crosses a halving boundary during
/// claim or stake operations.
///
/// Not yet wired into `reward_between_ledgers`: that walk splits segments at
/// halving boundaries without a hook to emit from. Kept so the halving schedule
/// has an event to publish once one exists.
#[allow(dead_code)]
pub fn halving_occurred(env: &Env, halving_count: u32, effective_rate_bps: i128, ledger: u32) {
    let topics = (symbol_short!("halving"),);
    env.events()
        .publish(topics, (halving_count, effective_rate_bps, ledger));
}

// ── Issue #222: staking certificate events ────────────────────────────────────

pub fn certificate_issued(
    env: &Env,
    user: &Address,
    certificate_id: u32,
    valid_until: u32,
    ledger: u32,
) {
    let topics = (symbol_short!("cert_iss"), user);
    env.events()
        .publish(topics, (certificate_id, valid_until, ledger));
}

// ── Issue #233: pool activation events ────────────────────────────────────────

pub fn pool_activated(env: &Env, total_staked: i128, threshold: i128, ledger: u32) {
    let topics = (symbol_short!("pool_act"),);
    env.events()
        .publish(topics, (total_staked, threshold, ledger));
}

pub fn pool_deactivated(env: &Env, total_staked: i128, threshold: i128, ledger: u32) {
    let topics = (symbol_short!("pool_dact"),);
    env.events()
        .publish(topics, (total_staked, threshold, ledger));
}

// ── Issue #234: reward activation events ──────────────────────────────────────

/// Emitted the first time pool TVL reaches the configured minimum pool size,
/// which is the ledger reward accrual starts from.
pub fn rewards_activated(env: &Env, total_staked: i128, min_pool_size: i128, ledger: u32) {
    let topics = (symbol_short!("rwd_act"),);
    env.events()
        .publish(topics, (total_staked, min_pool_size, ledger));
}

/// Emitted when the admin changes the minimum pool size required for rewards.
pub fn min_pool_size_set(env: &Env, admin: &Address, old_amount: i128, new_amount: i128) {
    let topics = (symbol_short!("minpool"), admin);
    env.events()
        .publish(topics, (old_amount, new_amount, env.ledger().sequence()));
}

// ── Issue #235: reward smoothing events ───────────────────────────────────────

/// Emitted when a large yield addition is scheduled for linear release instead
/// of being credited to the pool immediately.
pub fn smoothing_scheduled(
    env: &Env,
    total_amount: i128,
    start_ledger: u32,
    duration_ledgers: u32,
) {
    let topics = (symbol_short!("smth_sch"),);
    env.events()
        .publish(topics, (total_amount, start_ledger, duration_ledgers));
}

/// Emitted when a slice of a smoothing schedule is credited to the pool.
pub fn smoothing_released(env: &Env, amount: i128, cumulative_released: i128, total_amount: i128) {
    let topics = (symbol_short!("smth_rel"),);
    env.events().publish(
        topics,
        (
            amount,
            cumulative_released,
            total_amount,
            env.ledger().sequence(),
        ),
    );
}

// ── Issue #237: capacity auction events ───────────────────────────────────────

pub fn auction_started(env: &Env, spots: u32, min_bid: i128, ends_at: u32) {
    let topics = (symbol_short!("auct_st"),);
    env.events()
        .publish(topics, (spots, min_bid, ends_at, env.ledger().sequence()));
}

pub fn bid_placed(env: &Env, bidder: &Address, amount: i128, total_bid: i128) {
    let topics = (symbol_short!("bid_plcd"), bidder);
    env.events()
        .publish(topics, (amount, total_bid, env.ledger().sequence()));
}

pub fn auction_won(env: &Env, bidder: &Address, amount: i128, rank: u32) {
    let topics = (symbol_short!("auct_won"), bidder);
    env.events()
        .publish(topics, (amount, rank, env.ledger().sequence()));
}

pub fn bid_refunded(env: &Env, bidder: &Address, amount: i128) {
    let topics = (symbol_short!("bid_rfnd"), bidder);
    env.events()
        .publish(topics, (amount, env.ledger().sequence()));
}

pub fn auction_finalized(env: &Env, winner_count: u32, total_staked: i128, refunded: i128) {
    let topics = (symbol_short!("auct_fin"),);
    env.events().publish(
        topics,
        (
            winner_count,
            total_staked,
            refunded,
            env.ledger().sequence(),
        ),
    );
}

// ── Issue #232: position expired event ────────────────────────────────────────

pub fn position_expired(env: &Env, user: &Address, expired_at_ledger: u32, current_ledger: u32) {
    let topics = (symbol_short!("pos_exp"), user);
    env.events()
        .publish(topics, (expired_at_ledger, current_ledger));
}

pub fn dynamic_fee_config_updated(
    env: &Env,
    admin: &Address,
    base_fee_bps: u32,
    max_fee_bps: u32,
    threshold_bps: u32,
) {
    let topics = (symbol_short!("dfeecfg"), admin);
    env.events().publish(
        topics,
        (
            base_fee_bps,
            max_fee_bps,
            threshold_bps,
            env.ledger().sequence(),
        ),
    );
}

// ── Issue #239: stake-weighted lottery ────────────────────────────────────────

pub fn lottery_drawn(
    env: &Env,
    winner: &Address,
    prize_amount: i128,
    random_seed: u64,
    ledger: u32,
) {
    let topics = (symbol_short!("lot_drwn"), winner);
    env.events()
        .publish(topics, (prize_amount, random_seed, ledger));
}

// ── Issue #238: loyalty milestone badges ──────────────────────────────────────

pub fn milestone_achieved(
    env: &Env,
    user: &Address,
    milestone_id: u32,
    milestone_name: &soroban_sdk::String,
    ledger: u32,
) {
    let topics = (symbol_short!("mstn_ach"), user);
    env.events()
        .publish(topics, (milestone_id, milestone_name.clone(), ledger));
}

// ── Issue #240: oracle-triggered lock-up release ──────────────────────────────

pub fn condition_triggered(
    env: &Env,
    user: &Address,
    asset_id: &soroban_sdk::String,
    oracle_price: i128,
    trigger_price: i128,
    ledger: u32,
) {
    let topics = (symbol_short!("cond_trg"), user);
    env.events().publish(
        topics,
        (asset_id.clone(), oracle_price, trigger_price, ledger),
    );
}

// ── Issue #241: governance proposal veto ──────────────────────────────────────

pub fn proposal_vetoed(
    env: &Env,
    user: &Address,
    proposal_id: u32,
    user_stake_bps: u32,
    ledger: u32,
) {
    let topics = (symbol_short!("prop_veto"), user);
    env.events()
        .publish(topics, (proposal_id, user_stake_bps, ledger));
}

// ── Issue #242: stake matching program ────────────────────────────────────────

pub fn stake_matched(
    env: &Env,
    user: &Address,
    stake_amount: i128,
    match_amount: i128,
    ledger: u32,
) {
    let topics = (symbol_short!("stk_mtch"), user);
    env.events()
        .publish(topics, (stake_amount, match_amount, ledger));
}

// ── Issue #243: unstake insurance policy ──────────────────────────────────────

pub fn insurance_purchased(
    env: &Env,
    user: &Address,
    premium_paid: i128,
    position_amount: i128,
    ledger: u32,
) {
    let topics = (symbol_short!("ins_purc"), user);
    env.events()
        .publish(topics, (premium_paid, position_amount, ledger));
}

// ── Issue #244: multi-currency claim ──────────────────────────────────────────

pub fn reward_claimed_in_token(
    env: &Env,
    user: &Address,
    reward_amount: i128,
    output_token: &Address,
    output_amount: i128,
    ledger: u32,
) {
    let topics = (symbol_short!("clm_tok"), user);
    env.events().publish(
        topics,
        (reward_amount, output_token.clone(), output_amount, ledger),
    );
}
