use soroban_sdk::{contracttype, Address, String, Vec};

/// Storage keys for all persistent and instance state in the vault.
///
/// Instance keys (fast, small): Admin, Token, TotalShares, TotalDeposited,
/// MinStake, RewardRateBps, RewardPoolBalance, BoostSchedule, Paused,
/// WithdrawalLimit, LockPeriod, EarlyExitPenaltyBps, TotalStakers,
/// TotalRewardsPaid, WhitelistEnabled, CooldownPeriod,
/// UnstakeFeeBps, AllStakers, InactivityThreshold, Changelog,
/// LastRateChangeLedger, InitializedAtLedger.
///
/// Persistent keys (per-user, long-lived): ShareBalance, StakeHistory,
/// RewardCheckpointLedger, LastClaimLedger, AccruedReward, StakedAtLedger,
/// Delegate, Whitelisted, UnbondingPosition, UserClaimWindow, FrozenAt,
/// VestingEntries.
#[contracttype]
#[derive(Clone)]
pub enum DataKey {
    Admin,
    Token,
    TotalShares,
    TotalDeposited,
    MinStake,
    RewardRateBps,
    RewardPoolBalance,
    BoostSchedule,
    ShareBalance(Address),
    StakeHistory(Address),
    RewardCheckpointLedger(Address),
    LastClaimLedger(Address),
    AccruedReward(Address),
    Paused,
    WithdrawalLimit,
    LockPeriod,
    EarlyExitPenaltyBps,
    StakedAtLedger(Address),
    TotalStakers,
    TotalRewardsPaid,
    Delegate(Address),
    LastUnstakeLedger(Address),
    TotalEverClaimed,
    Restaked(Address),
    WhitelistEnabled,
    Whitelisted(Address),
    CooldownPeriod,
    UnbondingPosition(Address),
    RewardRemainder(Address),
    UserClaimWindow(Address),
    UnstakeFeeBps,
    AllStakers,
    RateHistory,
    BoostCampaign,
    Leaderboard,
    LeaderboardSize,
    // Issue #101: frozen positions
    InactivityThreshold,
    FrozenAt(Address),
    KycRequired,
    KycApproved(Address),
    Stopped,
    ShuttingDown,
    FirstStakedAt(Address),
    // Task 2: Vesting
    VestingPeriod,
    VestingEntries(Address),
    // Task 3: Epoch Mode
    EpochMode,
    CurrentEpoch,
    EpochLedgers,
    EpochRewardPerEpoch,
    EpochRewardFactor(u32),
    // NOTE: DataKey is already at Soroban's 50-variant cap for
    // #[contracttype] enums (ScSpecUdtUnionV0::cases is VecM<_, 50> in
    // stellar-xdr) — adding a 51st variant makes the #[contracttype] derive
    // panic at compile time. New storage introduced by issues #205/#206/
    // #207/#209 uses raw Symbol-keyed storage instead (see balance.rs),
    // matching the pattern already established here for the same reason
    // (e.g. symbol_short!("pool_cp"), symbol_short!("sl_tr"), the
    // Symbol::new(env, "prop")/"voted" tuple keys above).
}

/// Storage key for an individual epoch snapshot.
///
/// Soroban's enum contracttype support is stricter for tuple variants, so we
/// keep the address and epoch together in a dedicated struct instead of a
/// multi-field enum variant.
#[contracttype]
#[derive(Clone, Debug, PartialEq)]
pub struct UserEpochSnapshotKey {
    pub user: Address,
    pub epoch: u32,
}

/// Issue #42: enum of all admin actions for the audit log.
#[contracttype]
#[derive(Clone, Debug, PartialEq)]
pub enum AdminAction {
    SetRewardRate,
    Pause,
    Unpause,
    TransferAdmin,
    SetLockPeriod,
    SetCap,
    Slash,
    RescueToken,
    SetEarlyExitPenalty,
    SetMinStake,
    FundRewardPool,
    AddYield,
    SetBoostSchedule,
    SetNftContract,
    SetRestakeWindow,
    SetRewardToken,
}

#[contracttype]
#[derive(Clone, Debug, PartialEq)]
pub struct UnbondingPosition {
    pub amount: i128,
    pub unbonding_since: u32,
}

#[contracttype]
#[derive(Clone, Debug, PartialEq)]
pub struct VaultState {
    pub total_shares: i128,
    pub total_deposited: i128,
    pub paused: bool,
}

#[contracttype]
#[derive(Clone, Debug, PartialEq)]
pub struct PoolStats {
    pub total_staked: i128,
    pub total_stakers: u32,
    pub reward_rate_bps: i128,
    pub reward_token_balance: i128,
    pub paused: bool,
    pub total_rewards_paid: i128,
}

#[contracttype]
#[derive(Clone, Debug, PartialEq)]
pub struct RewardTier {
    pub max_amount: i128,
    pub rate_bps: i128,
}

/// Aggregate user stats used by `user_stats`.
///
/// - `position_amount`: the user's current position size expressed in token units.
/// - `pending_reward`: rewards accrued but not yet claimed.
/// - `staked_at_ledger`: the ledger sequence when the position was first opened.
/// - `last_claim_ledger`: the most recent ledger at which rewards were claimed.
#[contracttype]
#[derive(Clone, Debug, PartialEq)]
pub struct UserStats {
    pub position_amount: i128,
    pub pending_reward: i128,
    pub staked_at_ledger: u32,
    pub last_claim_ledger: u32,
}

/// Active boost campaign set by admin (#48).
///
/// - `multiplier_bps`: reward multiplier stacked on top of tier multipliers (10000 = 1x).
/// - `starts_at_ledger`: ledger when the campaign was activated.
/// - `ends_at_ledger`: ledger after which the campaign no longer applies.
#[contracttype]
#[derive(Clone, Debug, PartialEq)]
pub struct CampaignInfo {
    pub multiplier_bps: u32,
    pub starts_at_ledger: u32,
    pub ends_at_ledger: u32,
}

/// A single entry in the staking leaderboard (#46).
#[contracttype]
#[derive(Clone, Debug, PartialEq)]
pub struct LeaderboardEntry {
    pub staker: Address,
    pub amount: i128,
}

/// Type alias for the leaderboard vector used in storage and queries.
#[allow(dead_code)]
pub type Leaderboard = Vec<LeaderboardEntry>;

/// Current stake position for a user.
///
/// - `amount`: the user's current position size expressed in token units.
/// - `staked_at_ledger`: the ledger sequence when the position was first opened.
/// - `last_claim_ledger`: the most recent ledger at which rewards were claimed.
#[contracttype]
#[derive(Clone, Debug, PartialEq)]
pub struct StakePosition {
    pub amount: i128,
    pub staked_at_ledger: u32,
    pub last_claim_ledger: u32,
}

/// Snapshot of all pool-level configuration returned by `get_pool_config`.
///
/// Allows frontends to fetch all settings in a single RPC call instead of
/// querying each key individually.
///
/// - `admin`: current admin address.
/// - `stake_token`: token accepted for staking and used to pay rewards.
/// - `reward_token`: same as `stake_token` (single-token vault).
/// - `reward_rate_bps`: annual reward rate in basis points.
/// - `paused`: whether deposits and withdrawals are currently paused.
#[contracttype]
#[derive(Clone, Debug, PartialEq)]
pub struct PoolConfig {
    pub admin: Address,
    pub stake_token: Address,
    pub reward_token: Address,
    pub reward_rate_bps: u32,
    pub paused: bool,
}

/// Contract metadata returned by `contract_metadata`.
#[contracttype]
#[derive(Clone, Debug, PartialEq)]
pub struct ContractMetadata {
    pub name: String,
    pub version: String,
    pub description: String,
}

/// Per-user reward claim window used to enforce the optional claim cap.
///
/// - `claimed_in_window`: cumulative rewards claimed by this user in the current window.
/// - `window_started_at`: ledger sequence at which the current window began.
///
/// The window resets automatically when `current_ledger > window_started_at + window_ledgers`.
/// Any unclaimed remainder is deferred to the next window — it is not lost.
#[contracttype]
#[derive(Clone, Debug, PartialEq)]
pub struct ClaimWindow {
    pub claimed_in_window: i128,
    pub window_started_at: u32,
}

/// Dynamic unstake fee model configured via `set_dynamic_fee_config` (issue #213).
///
/// Below `utilization_threshold_bps` pool utilization, the unstake fee is
/// `base_fee_bps`. Above it, the fee interpolates linearly up to
/// `max_fee_bps` at 100% utilization. While a config is set, `unstake` uses
/// this dynamic fee instead of the static fee from `set_unstake_fee_bps`.
#[contracttype]
#[derive(Clone, Debug, PartialEq)]
pub struct DynamicFeeConfig {
    pub base_fee_bps: u32,
    pub max_fee_bps: u32,
    pub utilization_threshold_bps: u32,
}

/// Composite staker reputation score returned by `get_reputation_score` (issue #214).
///
/// Each sub-score is bounded to 2500 basis points; `total_score` is the sum,
/// bounded to 10 000 (100%). All zero for an address with no active position.
#[contracttype]
#[derive(Clone, Debug, PartialEq)]
pub struct ReputationScore {
    pub consistency_score: u32,
    pub volume_score: u32,
    pub tenure_score: u32,
    pub total_score: u32,
}

/// Waitlist entry struct representing a prospective staker in the queue (Acceptance Criteria #1).
#[contracttype]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct WaitlistEntry {
    pub user: Address,
    pub intended_amount: i128,
    pub joined_waitlist_at: u32,
}
