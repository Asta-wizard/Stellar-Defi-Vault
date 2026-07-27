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
    pub duration_score: u32,
    pub consistency_score: u32,
    pub size_score: u32,
    pub streak_score: u32,
    pub total_score: u32,
}

/// Single entry in the on-chain changelog exposed by `get_changelog`.
#[contracttype]
#[derive(Clone, Debug, PartialEq)]
pub struct ChangelogEntry {
    pub change_type: String,
    pub old_value: i128,
    pub new_value: i128,
}

/// One entry in the rich reward-rate history exposed by `get_reward_rate_history` (issue #124).
#[contracttype]
#[derive(Clone, Debug, PartialEq)]
pub struct RateHistoryEntry {
    pub old_rate_bps: i128,
    pub new_rate_bps: i128,
    pub changed_at_ledger: u32,
    pub changed_by: Address,
}

/// Accumulated referral stats stored per referrer.
#[contracttype]
#[derive(Clone, Debug, PartialEq)]
pub struct ReferralStats {
    pub total_referred_stake: i128,
    pub referral_count: u32,
}

/// Result of `get_boost_tier_progress` showing where a user is toward the next boost tier.
#[contracttype]
#[derive(Clone, Debug, PartialEq)]
pub struct BoostTierProgress {
    pub current_tier: u32,
    pub current_multiplier_bps: i128,
    pub next_tier_in_ledgers: Option<u32>,
    pub next_multiplier_bps: Option<i128>,
}

/// One entry in the referral leaderboard returned by `referral_leaderboard`.
#[contracttype]
#[derive(Clone, Debug, PartialEq)]
pub struct ReferralLeaderboardEntry {
    pub referrer: Address,
    pub total_referred_stake: i128,
    pub referral_count: u32,
}

/// Comprehensive health snapshot of the pool returned by `pool_health_report`.
#[contracttype]
#[derive(Clone, Debug, PartialEq)]
pub struct PoolHealthReport {
    pub reward_token_balance: i128,
    pub total_staked: i128,
    pub total_stakers: u32,
    pub total_rewards_paid: i128,
    pub reward_rate_bps: i128,
    pub is_paused: bool,
    pub is_stopped: bool,
    pub uptime_ledgers: u32,
    pub estimated_daily_obligations: i128,
    pub is_solvent_7_days: bool,
}

/// Effective reward rate breakdown returned by `reward_multiplier_preview` (issue #181).
///
/// Each `*_multiplier_bps` field defaults to 10000 (1x) if that boost mechanism
/// is not active for the user. `effective_rate_bps` is the fully stacked rate.
#[contracttype]
#[derive(Clone, Debug, PartialEq)]
pub struct RewardMultiplierBreakdown {
    pub base_rate_bps: i128,
    pub tier_multiplier_bps: i128,
    pub campaign_multiplier_bps: i128,
    pub effective_rate_bps: i128,
}

/// Aggregate score used by `staking_efficiency_score`.
#[contracttype]
#[derive(Clone, Debug, PartialEq)]
pub struct StakingEfficiencyScore {
    pub total_claimed: i128,
    pub estimated_if_compounded: i128,
    pub efficiency_bps: i128,
}

/// Aggregated user state returned by `user_summary` (issue #103).
///
/// - `position`: 0 or 1 `StakePosition` entries; empty when user has no stake.
/// - `pending_reward`: rewards accrued but not yet claimed.
/// - `pool_share_bps`: user's share of the total pool in basis points (10000 = 100%).
///
/// Note: `position` uses `Vec<StakePosition>` (0-or-1 elements) because
/// `Option<ContractType>` is not supported in `#[contracttype]` structs in
/// soroban-sdk 21.x testutils mode.
#[contracttype]
#[derive(Clone, Debug, PartialEq)]
pub struct UserSummary {
    pub position: Vec<StakePosition>,
    pub pending_reward: i128,
    pub pool_share_bps: i128,
}

// ── Issue #105: stake/unstake history ────────────────────────────────────────

/// Discriminant for a stake history entry.
#[contracttype]
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum StakeAction {
    Stake,
    Unstake,
}

/// One entry in a user's recent staking activity log.
///
/// - `action`: whether the user staked or unstaked.
/// - `amount`: token amount involved (not shares).
/// - `ledger`: ledger sequence number at which the action was recorded.
#[contracttype]
#[derive(Clone, Debug, PartialEq)]
pub struct StakeHistoryEntry {
    pub action: StakeAction,
    pub amount: i128,
    pub ledger: u32,
}

// ── Issue #104: interface detection ──────────────────────────────────────────

/// Feature interface identifiers for `supports_interface`.
///
/// `Base` is always supported. All others are only true when the corresponding
/// feature is compiled into this deployment.
#[contracttype]
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum InterfaceId {
    Base,
    Lockup,
    Whitelist,
    Compounding,
    EpochMode,
    VestingSchedule,
}

/// Result of a `can_unstake` pre-flight check (issue #98).
#[contracttype]
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum UnstakeCheckResult {
    /// The unstake would succeed.
    Ok,
    /// The user has no active staking position.
    NoPosition,
    /// The user's position is smaller than the requested amount (in token units).
    InsufficientAmount,
    /// The pool is currently paused.
    PoolPaused,
    /// The lock-up period has not yet elapsed (early exit penalty would apply).
    StillLocked,
}

/// Per-user staking streak data (issue #99).
#[contracttype]
#[derive(Clone, Debug, PartialEq)]
pub struct StakeStreak {
    pub current_streak: u32,
    pub longest_streak: u32,
    pub last_active_wave: u32,
}

#[contracttype]
#[derive(Clone, Debug, PartialEq)]
pub struct VestingEntry {
    pub amount: i128,
    pub claimable_at_ledger: u32,
}

#[contracttype]
#[derive(Clone, Debug, PartialEq)]
pub struct EpochState {
    pub epoch_number: u32,
    pub started_at: u32,
    pub reward_pool: i128,
    pub total_staked_snapshot: i128,
}

/// Governance checkpoint: total staked recorded at a specific ledger (issue snapshot_total_staked).
#[contracttype]
#[derive(Clone, Debug, PartialEq)]
pub struct TotalStakedSnapshot {
    pub total_staked: i128,
    pub ledger: u32,
}

/// Issue #166: aggregated contract/token addresses for integration setup.
#[contracttype]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ContractAddresses {
    pub contract: Address,
    pub admin: Address,
    pub stake_token: Address,
    pub reward_token: Address,
}

/// One day-bucket in the rolling 7-day activity heatmap.
///
/// `day_index` = `current_ledger / LEDGERS_PER_DAY` (see `vault::LEDGERS_PER_DAY`).
/// Each counter is incremented once per matching user action within that day.
#[contracttype]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct DayBucket {
    pub day_index: u32,
    pub stake_count: u32,
    pub unstake_count: u32,
    pub claim_count: u32,
}

// ── Issue #219: pause reason ─────────────────────────────────────────────────

/// Reason categories for pool pauses (issue #219).
#[contracttype]
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum PauseReason {
    Maintenance,
    SecurityIncident,
    RateReconfiguration,
    CapAdjustment,
    Other,
}

/// Full pause state stored when the pool is paused (issue #219).
#[contracttype]
#[derive(Clone, Debug, PartialEq)]
pub struct PauseInfo {
    pub reason: PauseReason,
    pub message: String,
    pub paused_at: u32,
}

// ── Issue #217: tax reporting ────────────────────────────────────────────────

/// Annual tax report summary for a user (issue #217).
#[contracttype]
#[derive(Clone, Debug, PartialEq)]
pub struct TaxReport {
    pub user: Address,
    pub ledger_from: u32,
    pub ledger_to: u32,
    pub total_rewards_claimed: i128,
    pub average_stake_amount: i128,
    pub claim_count: u32,
}

// ── Issue #220: rounding policy ──────────────────────────────────────────────

/// Rounding behaviour for sub-unit division in share and reward math (issue #220).
#[contracttype]
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum RoundingPolicy {
    Floor,
    Ceiling,
    Nearest,
}

// ── Issue #216: governance voting ────────────────────────────────────────────

/// Pool parameter a governance proposal can change (issue #216).
#[contracttype]
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum ProposableParam {
    RewardRate,
    UnstakeFee,
    PoolCap,
    MinStake,
}

/// A governance proposal to change a pool parameter, created via
/// `create_proposal` and resolved via `enact_proposal` (issue #216).
///
/// Voting power is the staker's token amount at the time of the `vote` call,
/// not adjusted retroactively if the staker's position changes afterward.
#[contracttype]
#[derive(Clone, Debug, PartialEq)]
pub struct GovernanceProposal {
    pub id: u32,
    pub parameter: ProposableParam,
    pub new_value: i128,
    pub votes_for: i128,
    pub votes_against: i128,
    pub ends_at: u32,
    pub enacted: bool,
}

// ── Issue #207: cross-chain bridge relayer hook ──────────────────────────────

/// Packet data emitted alongside `stake`/`unstake`, shaped for cross-chain
/// bridge relayers (issue #207). Supplementary only — never read by, or
/// required for, core stake/unstake logic.
#[contracttype]
#[derive(Clone, Debug, PartialEq)]
pub struct BridgePacket {
    pub sequence: u64,
    pub source_address: Address,
    pub amount: i128,
    pub action: String,
    pub timestamp_ledger: u32,
}

// ── Issue #197: fee splitting ─────────────────────────────────────────────────

/// One recipient of a proportional fee split (issue #197).
#[contracttype]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct FeeRecipient {
    pub address: Address,
    pub share_bps: u32,
}

// ── Issue #195: timelocked admin actions ─────────────────────────────────────

/// A queued admin action awaiting its timelock delay (issue #195).
///
/// Execution support (see `execute_action()` in vault.rs) is implemented for
/// `AdminAction::SetRewardRate` and `AdminAction::Pause`/`Unpause` — the
/// issue's own notes call these out as "the primary candidates" for
/// timelock. Queuing any other `AdminAction` is allowed (the queue/cancel
/// lifecycle itself is generic), but executing one reverts with
/// `ActionNotFound` rather than silently doing nothing, since safely
/// decoding+dispatching arbitrary opaque `Bytes` for every admin action in
/// this contract is a much larger dispatcher than this issue's stated scope.
#[contracttype]
#[derive(Clone, Debug, PartialEq)]
pub struct PendingAction {
    pub id: u32,
    pub action_type: AdminAction,
    pub params: soroban_sdk::Bytes,
    pub queued_at: u32,
    pub executable_at: u32,
}

// ── Issue #196: multi-sig admin ──────────────────────────────────────────────

/// Multisig configuration: up to 5 admin addresses and an M-of-N approval
/// threshold (issue #196). Additive and optional — see the scope note on
/// `initialize_multisig()` in vault.rs for why this does not replace the
/// existing single `Admin` key.
#[contracttype]
#[derive(Clone, Debug, PartialEq)]
pub struct MultisigConfig {
    pub admins: Vec<Address>,
    pub threshold: u32,
}

/// A multisig proposal awaiting approvals (issue #196). Named
/// `AdminProposal` rather than the issue's literal `Proposal` to avoid
/// confusion with the existing, unrelated `GovernanceProposal` (staker
/// voting on pool parameters, issue #216) already in this file.
///
/// Same execution-dispatch scope note as `PendingAction` above: only
/// `AdminAction::SetRewardRate` and `Pause`/`Unpause` are actually
/// executable once threshold is reached; other action types can be
/// proposed and approved but revert on execution.
#[contracttype]
#[derive(Clone, Debug, PartialEq)]
pub struct AdminProposal {
    pub id: u32,
    pub action_type: AdminAction,
    pub params: soroban_sdk::Bytes,
    pub approvals: Vec<Address>,
    pub executed: bool,
}

// ── Missing types for existing feature branches ──────────────────────────────

/// Export payload for migrating pool state to a new contract (issue #203).
#[contracttype]
#[derive(Clone, Debug, PartialEq)]
pub struct MigrationExport {
    pub admin: Address,
    pub stake_token: Address,
    pub reward_token: Address,
    pub reward_rate_bps: u32,
    pub total_staked: i128,
    pub total_stakers: u32,
    pub paused: bool,
    pub all_positions: Vec<(Address, StakePosition)>,
}

/// Bootstrap configuration for phased reward rate (issue #202).
#[contracttype]
#[derive(Clone, Debug, PartialEq)]
pub struct BootstrapConfig {
    pub initial_rate: u32,
    pub base_rate: u32,
    pub started_at: u32,
    pub duration: u32,
}

/// A delegation chain: beneficiary + list of delegates (issue #200).
#[contracttype]
#[derive(Clone, Debug, PartialEq)]
pub struct DelegationChain {
    pub beneficiary: Address,
    pub delegates: Vec<Address>,
}

/// Storage footprint report (issue #208).
#[contracttype]
#[derive(Clone, Debug, PartialEq)]
pub struct StorageUsageReport {
    pub instance_keys: u32,
    pub persistent_position_count: u32,
    pub persistent_other_keys: u32,
    pub estimated_total_bytes: u32,
}

// ── Issue #210: Merkle Reward Distribution ────────────────────────────────────

#[contracttype]
#[derive(Clone, Debug, PartialEq)]
pub struct MerkleRoot {
    pub root: soroban_sdk::Bytes,
    pub epoch: u32,
    pub total_claimable: i128,
    pub published_at: u32,
}

// ── Issue #231: Halving Schedule ──────────────────────────────────────────────

#[contracttype]
#[derive(Clone, Debug, PartialEq)]
pub struct HalvingConfig {
    pub interval_ledgers: u32,
    pub started_at: u32,
    pub halving_count: u32,
    pub floor_rate_bps: i128,
}

// ── Issue #222: Staking Certificate ───────────────────────────────────────────

#[contracttype]
#[derive(Clone, Debug, PartialEq)]
pub struct StakingCertificate {
    pub holder: Address,
    pub min_amount_staked: i128,
    pub staked_since: u32,
    pub issued_at: u32,
    pub valid_until: u32,
    pub certificate_id: u32,
}

// ── Issue #211: Staking Tournament Competition ─────────────────────────────────

#[contracttype]
#[derive(Clone, Debug, PartialEq)]
pub struct Tournament {
    pub prize_pool: i128,
    pub starts_at: u32,
    pub ends_at: u32,
    pub winner: soroban_sdk::Bytes,
    pub finalized: bool,
}

// ── Issue #221: Cross-Pool Reward Comparison ──────────────────────────────────

#[contracttype]
#[derive(Clone, Debug, PartialEq)]
pub struct PoolComparison {
    pub pool_address: Address,
    pub reward_rate_bps: i128,
    pub reachable: bool,
}

// ── Issue #235: Reward Smoothing ──────────────────────────────────────────────

/// A large reward addition being released linearly over `duration_ledgers`
/// (issue #235).
///
/// `released` is the cumulative amount already credited to `total_deposited` by
/// `release_smoothed_yield()`. The schedule is finished once
/// `released == total_amount`; the tokens themselves are transferred into the
/// contract up front when the schedule is created, so a finished schedule owes
/// nothing.
#[contracttype]
#[derive(Clone, Debug, PartialEq)]
pub struct SmoothingSchedule {
    pub total_amount: i128,
    pub released: i128,
    pub start_ledger: u32,
    pub duration_ledgers: u32,
}

/// Read-only snapshot of the active smoothing schedule returned by
/// `reward_smoothing()` (issue #235).
///
/// All fields are zero when no schedule has ever been created. `releasable_now`
/// is what a `release_smoothed_yield()` call at this ledger would credit;
/// `unreleased` is everything still locked, including `releasable_now`.
#[contracttype]
#[derive(Clone, Debug, PartialEq)]
pub struct SmoothingStatus {
    pub total_amount: i128,
    pub released: i128,
    pub releasable_now: i128,
    pub unreleased: i128,
    pub start_ledger: u32,
    pub duration_ledgers: u32,
    pub ledgers_remaining: u32,
}

// ── Issue #236: Referral Tree Visualization ───────────────────────────────────

/// One node in the flattened referral hierarchy returned by
/// `referral_tree_data()` (issue #236).
///
/// The tree is returned as a flat, breadth-first `Vec` rather than a nested
/// structure because `#[contracttype]` cannot express a recursive type. Rebuild
/// the hierarchy client-side by joining `parent` against `address`.
///
/// - `level`: 0 for the queried root, 1 for its direct referrals, up to 3.
/// - `parent`: the address that referred this node. For the root (level 0) this
///   is the root's own address, since `Option<Address>` is not usable inside a
///   `#[contracttype]` struct in soroban-sdk 21.x testutils mode (same
///   constraint that shapes `UserSummary::position`).
/// - `referred_count` / `total_referred_stake`: this node's own referral stats.
#[contracttype]
#[derive(Clone, Debug, PartialEq)]
pub struct ReferralTreeNode {
    pub address: Address,
    pub parent: Address,
    pub level: u32,
    pub referred_count: u32,
    pub total_referred_stake: i128,
}

// ── Issue #237: Capacity Auction ──────────────────────────────────────────────

/// An auction allocating a limited number of pool spots to the highest bidders
/// (issue #237).
///
/// A bid is denominated in stake tokens and is escrowed in the contract when
/// placed. On `finalize_capacity_auction()` the top `spots` bids are converted
/// into real staking positions and every losing bid is refunded in full.
#[contracttype]
#[derive(Clone, Debug, PartialEq)]
pub struct CapacityAuction {
    pub spots: u32,
    pub min_bid: i128,
    pub started_at: u32,
    pub ends_at: u32,
    pub finalized: bool,
    pub total_escrowed: i128,
}

/// A single bid in the capacity auction (issue #237).
#[contracttype]
#[derive(Clone, Debug, PartialEq)]
pub struct AuctionBid {
    pub bidder: Address,
    pub amount: i128,
    pub placed_at: u32,
}

// ── Issue #239: stake-weighted lottery ────────────────────────────────────────

/// A prize pool funded by the admin and drawn among active stakers, weighted
/// by their staked amount (issue #239).
///
/// `winner` uses `Vec<Address>` (0 or 1 elements) rather than `Option<Address>`
/// since `Option<Address>` is not usable inside a `#[contracttype]` struct in
/// soroban-sdk 21.x testutils mode (same constraint noted on `UserSummary`
/// and `ReferralTreeNode` above).
#[contracttype]
#[derive(Clone, Debug, PartialEq)]
pub struct LotteryConfig {
    pub prize_amount: i128,
    pub draw_at_ledger: u32,
    pub drawn: bool,
    pub winner: Vec<Address>,
}

// ── Issue #238: loyalty milestone badges ──────────────────────────────────────

/// The staker metric a milestone is evaluated against (issue #238).
#[contracttype]
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum MilestoneCondition {
    StakeDurationLedgers,
    TotalStakedAmount,
    ClaimCount,
    TotalRewardsClaimed,
}

/// An admin-configured staking achievement. Evaluated lazily by
/// `check_milestones()`, which records a non-transferable badge in
/// `UserMilestones` the first time a staker crosses `threshold` (issue #238).
#[contracttype]
#[derive(Clone, Debug, PartialEq)]
pub struct Milestone {
    pub id: u32,
    pub name: String,
    pub condition_type: MilestoneCondition,
    pub threshold: i128,
}

// ── Issue #240: oracle-triggered lock-up release ──────────────────────────────

/// Direction the oracle price must move relative to `trigger_price` to
/// satisfy a `PriceCondition` (issue #240).
#[contracttype]
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum TriggerDirection {
    Below,
    Above,
}

/// A user-configured price condition that, once satisfied, waives their
/// position's early-exit lock-up penalty (issue #240).
#[contracttype]
#[derive(Clone, Debug, PartialEq)]
pub struct PriceCondition {
    pub trigger_price: i128,
    pub asset_id: String,
    pub direction: TriggerDirection,
}

// ── Issue #242: stake matching program ────────────────────────────────────────

/// An admin-funded program that matches every user stake with an extra
/// contribution, bounded by a per-user cap and a total program budget.
///
/// `total_budget` is transferred into the contract up front by
/// `start_matching_program()`; `budget_used` is the portion already credited to
/// stakers. Unused budget is returned to the admin by `end_matching_program()`.
#[contracttype]
#[derive(Clone, Debug, PartialEq)]
pub struct MatchingProgram {
    pub match_rate_bps: u32,
    pub per_user_cap: i128,
    pub total_budget: i128,
    pub budget_used: i128,
    pub active: bool,
}

/// Per-user running total of matched contributions received (issue #242).
#[contracttype]
#[derive(Clone, Debug, PartialEq)]
pub struct UserMatchingStats {
    pub total_matched: i128,
}

// ── Issue #245: staking cohort analytics ──────────────────────────────────────

/// Aggregate metrics for one weekly staker cohort (issue #245).
///
/// A cohort groups every address whose first stake landed in the same week:
/// `cohort_id = staked_at_ledger / (LEDGERS_PER_DAY * 7)`.
///
/// - `member_count`: addresses that have ever joined this cohort.
/// - `active_members`: members that still hold an open position — the ratio of
///   this to `member_count` is the cohort's retention rate.
/// - `avg_position`: `total_staked / active_members`, or 0 when none remain.
#[contracttype]
#[derive(Clone, Debug, PartialEq)]
pub struct CohortStats {
    pub cohort_id: u32,
    pub member_count: u32,
    pub total_staked: i128,
    pub avg_position: i128,
    pub total_rewards_claimed: i128,
    pub active_members: u32,
}
