use soroban_sdk::contracterror;

#[contracterror]
#[derive(Copy, Clone, Debug, Eq, PartialEq, PartialOrd, Ord)]
#[repr(u32)]
pub enum VaultError {
    /// Returned by initialize-dependent getters and stake/unstake flows when
    /// the admin, token, or other required contract state has not been stored
    /// yet, and by `deploy_to_yield()` / `withdraw_from_yield()` when no yield
    /// protocol has been registered via `set_yield_protocol()`.
    NotInitialized = 1,
    /// Returned by initialize() when the vault has already been initialized,
    /// and by `enact_proposal()` when the proposal has already been enacted.
    AlreadyInitialized = 2,
    /// Returned by admin-only entrypoints that call `admin::require_admin()`
    /// and by `rescue_token()` / `slash()` when the supplied admin address does
    /// not match the stored admin.
    Unauthorized = 3,
    /// Returned by staking, unstaking, and amount-setting calls when a
    /// caller supplies zero or a negative amount where that is not allowed.
    ZeroAmount = 4,
    /// Returned by `withdraw()`, `unstake()`, and `unstake_all()` when the
    /// caller tries to burn more shares than they own.
    InsufficientShares = 5,
    /// Returned by staking, unstaking, and admin-yield entrypoints that require
    /// the pool to be unpaused.
    VaultPaused = 6,
    /// Reserved for token-validation failures during initialization or future
    /// token checks; no current public function returns this variant.
    InvalidToken = 7,
    /// Returned by staking, unstaking, claim, slash, preview, and reward math
    /// helpers when checked arithmetic or share conversion fails.
    ArithmeticError = 8,
    /// Returned by `withdraw()`, `unstake()`, and `unstake_all()` when the
    /// requested share amount exceeds the configured per-transaction limit.
    WithdrawalLimitExceeded = 9,
    /// Returned by `set_early_exit_penalty_bps()` when the admin sets a value
    /// above the supported cap.
    InvalidPenaltyBps = 10,
    /// Returned by `deposit()`, `stake()`, and `stake_for()` when the resulting
    /// position would fall below the configured minimum stake.
    BelowMinimumStake = 11,
    /// Returned by `set_boost_schedule()` when more than five boost tiers are
    /// supplied.
    TooManyBoostTiers = 12,
    /// Returned by `set_boost_schedule()` when a tier multiplier is below the
    /// base rate or the tier ledgers are not strictly increasing.
    InvalidBoostSchedule = 13,
    /// Returned by `claim()`, `stake_and_claim()`, and `claim_epoch_rewards()`
    /// when the reward pool does not hold enough tokens to pay the claim, and
    /// by `withdraw_from_yield()` when the requested amount exceeds what is
    /// currently tracked as deployed to the yield protocol.
    InsufficientRewardPool = 14,
    /// Returned by `revoke_delegate()` when the caller revokes the wrong
    /// delegate, and by `stake_for()` when the caller is not the approved
    /// delegate for the beneficiary.
    NotADelegate = 15,
    /// Returned by `rescue_token()` when the admin tries to rescue the stake
    /// token itself.
    CannotRescueStakeToken = 16,
    /// Returned by `rescue_token()` when the admin tries to rescue the
    /// registered reward token.
    CannotRescueRewardToken = 17,
    /// Returned by position-dependent flows such as `unstake_all()`,
    /// `claimable_since()`, `position_age_ledgers()`, `time_since_last_claim()`,
    /// `request_unstake()`, `execute_unstake()`, `slash()`, `transfer_position()`,
    /// `merge_positions()`, and `flag_frozen()` when the user has no active
    /// stake or unbonding position. Also returned by `create_proposal()` and
    /// `vote()` when the caller has no active position, and by `vote()` /
    /// `enact_proposal()` when the given proposal id does not exist.
    PositionNotFound = 18,
    /// Returned by `deposit()`, `stake()`, `stake_for()`, and `stake_and_claim()`
    /// when whitelist enforcement is enabled and the staker or beneficiary is
    /// not approved.
    NotWhitelisted = 19,
    /// Returned by `withdraw()` and `unstake()` when cooldown is enabled, and
    /// by `execute_unstake()` when the cooldown has not finished yet.
    UseCooldownFlow = 20,
    /// Returned by `set_unstake_fee_bps()` when the fee exceeds 500 bps (5%).
    UnstakeFeeTooHigh = 21,
    /// Returned by `batch_position_query()` when more than 20 addresses are supplied.
    BatchTooLarge = 22,
    /// Returned by `vote()` when the caller has already voted on the given
    /// proposal.
    TooManyStakers = 23,
    /// Returned by `transfer_position()` when the recipient already has an
    /// active staking position.
    RecipientAlreadyStaking = 24,
    /// Returned by `start_boost_campaign()` when a boost campaign is already active.
    CampaignAlreadyActive = 25,
    /// Returned by `end_boost_campaign()` when there is no active boost campaign
    /// to cancel.
    NoCampaignActive = 26,
    /// Returned by `set_leaderboard_size()` when the requested leaderboard cap
    /// exceeds 20.
    LeaderboardSizeTooLarge = 27,
    /// Returned by `view_all_positions()` when `page_size` is 0 or greater than 20.
    PageSizeTooLarge = 28,
    /// Returned by staking entrypoints when KYC enforcement is enabled and the
    /// staker is not approved.
    KycNotApproved = 29,
    /// Returned by `deposit()`, `stake()`, `stake_for()`, `stake_and_claim()`,
    /// `pause()`, and `unpause()` after `emergency_stop()` has permanently
    /// stopped the contract.
    ContractStopped = 30,
    /// Returned by staking entrypoints when the new deposit would exceed the
    /// configured pool cap, and by `deploy_to_yield()` when the requested
    /// amount exceeds `available_for_yield()` (the 20% liquidity buffer).
    PoolCapReached = 31,
    /// Returned by `set_pool_description()` when the description exceeds 200
    /// characters.
    DescriptionTooLong = 32,
    /// Returned by `record_wave_activity()` when the supplied wave id is not
    /// greater than the last recorded wave.
    NonMonotonicWaveId = 33,
    /// Returned by `record_wave_activity()` when more than 50 active users are
    /// supplied in one call.
    TooManyActiveUsers = 34,
    /// Returned by `initialize()` when the admin or token address is invalid
    /// for this contract, such as matching the contract's own address.
    InvalidAddress = 35,
    /// Returned by `initialize()` and `set_reward_rate_bps()` when the reward
    /// APR exceeds the configured cap.
    RateTooHigh = 36,
    /// Returned by staking entrypoints when the user already holds the
    /// configured maximum number of active positions, and by
    /// `create_proposal()` when 10 governance proposals are already open.
    MaxPositionsReached = 37,
    /// Returned by `set_max_positions_per_user()` when the requested cap exceeds 10.
    MaxPositionsTooHigh = 38,
    /// Returned by `vote()` when the proposal's voting period has already
    /// ended, or the proposal has already been enacted. Also returned by
    /// `enact_proposal()` when the proposal has been vetoed (issue #241).
    BatchKycTooLarge = 39,
    /// Returned by `set_dynamic_fee_config()` when `base_fee_bps > max_fee_bps`
    /// or `utilization_threshold_bps` exceeds 10 000 (100%).
    InvalidRate = 40,
    /// Custom error message exceeds MAX_ERROR_MESSAGE_LENGTH (150 characters).
    MessageTooLong = 41,

    /// Returned by epoch-mode entrypoints when the contract is in the wrong mode.
    EpochModeConflict = 42,
    /// Returned when a vesting queue already holds the maximum supported entries.
    VestingQueueFull = 43,
    /// Returned when a vesting withdrawal is requested but nothing has matured yet.
    NothingToWithdraw = 44,
    /// Returned when an epoch cannot be finalized because the configured
    /// window has not elapsed, and by `enact_proposal()` when the voting
    /// period has not ended yet.
    EpochNotFinalized = 45,
    /// Caller is not an approved relayer for the target user (issue #118).
    RelayerNotApproved = 46,
    /// Caller is not on the yield source whitelist (issue #126).
    NotYieldSource = 47,
    
    // --- Emergency Waitlist Error Variants ---
    
    /// Returned when a user tries to join the waitlist but the pool still has free capacity.
    PoolNotFull = 48,
    /// Returned when a user tries to join the waitlist but it has reached its 100-entry capacity limit.
    WaitlistFull = 49,
    
    /// Fallback mapping variant for authorization error conditions.
    NotAuthorized = 52,

    // --- Reward waterfall / transfer cooldown / stake quota / slash dispute
    // (issues #341, #340, #339, #336) ---
    //
    // NOTE: this enum was already over Soroban's ~50-case `#[contracterror]`
    // cap (52 variants) *before* these 7 were added — confirmed by testing
    // the pre-existing 52-variant enum alone, which already panics the
    // `#[contracterror]` macro at build time. That's part of the same
    // unrelated pre-existing corruption described in the PR: `VaultError`
    // used to be three separate enums (see the "Legacy Workspace Mappings"
    // variants above) that got collapsed into one. Adding 7 more here
    // doesn't introduce a new failure mode — the type already can't build —
    // but it does mean whoever splits this enum back apart to actually fix
    // the cap should fold these 7 in wherever the rest end up.
    /// Returned by `unstake` / `check_transfer_cooldown` when the caller
    /// received their position via a transfer and is still inside the
    /// configured transfer cooldown window. Call
    /// `get_transfer_cooldown_remaining()` for how many ledgers remain.
    TransferCooldownActive = 53,
    /// Returned by quota-gated entrypoints (`create_proposal`,
    /// `submit_content`, `create_poll`) when the caller has no operation
    /// quota remaining in the current epoch.
    QuotaExhausted = 54,
    /// Returned by `dispute_slash` when the pool already has 5 open disputes.
    TooManyOpenDisputes = 55,
    /// Returned by `dispute_slash` when the dispute window for the given
    /// slash has already elapsed, and by `resolve_dispute` when the dispute's
    /// voting deadline has not been reached yet. Also returned by
    /// `vote_on_mutual_loss` (voting after the deadline) and
    /// `resolve_mutual_loss_claim` (resolving before it) in
    /// `mutual_insurance_pool.rs`, issue #366.
    DisputeWindowClosed = 56,
    /// Returned by `dispute_slash` / `vote_on_dispute` / `resolve_dispute`
    /// when the referenced slash or dispute id does not exist.
    DisputeNotFound = 57,
    /// Returned by `resolve_dispute` when the dispute has already been
    /// resolved.
    DisputeAlreadyResolved = 58,
    /// Returned by `vote_on_dispute` when the caller has already voted on
    /// the given dispute, or has no active position (so no vote weight).
    /// Also returned by `vote_on_mutual_loss` for the same two reasons
    /// (`mutual_insurance_pool.rs`, issue #366).
    AlreadyVotedOrNoWeight = 59,

    // --- Admin action nonce / governance comment thread (issues #374, #375) ---

    /// Returned by `execute_admin_action_with_nonce` when the supplied nonce
    /// does not match the admin's next expected nonce — either a stale,
    /// already-consumed value (a replayed transaction) or one issued too far
    /// ahead. Call `admin_action_nonce()` for the correct value.
    NonceMismatch = 60,
    /// Returned by `post_proposal_comment` when the comment text exceeds
    /// `proposal_comment_thread::MAX_COMMENT_LENGTH`.
    CommentTooLong = 61,
    // --- Pool pre-sale (issue #369) ---
    /// Returned by `reserve_presale_spot`, `redeem_presale_reservation`, and
    /// `cancel_presale` when no pre-sale has been started, or the active one
    /// was already cancelled.
    PresaleNotActive = 60,
    /// Returned by `start_presale` when a pre-sale is already active, and by
    /// `reserve_presale_spot` once the pre-sale's `opens_at` ledger has been
    /// reached (reservations are only accepted before the pool opens).
    PresaleReservationClosed = 61,
    /// Returned by `redeem_presale_reservation` when called before the
    /// pre-sale's `opens_at` ledger.
    PresaleNotYetOpen = 62,
    /// Returned by `reserve_presale_spot` when the reservation would exceed
    /// `max_reservation_per_user`.
    PresaleReservationExceedsMax = 63,
    /// Returned by `redeem_presale_reservation` when the caller has no
    /// reservation (or reserved zero).
    NoPresaleReservation = 64,
    /// Returned by `redeem_presale_reservation` when the reservation was
    /// already redeemed.
    PresaleReservationAlreadyRedeemed = 65,

    // --- Mutual insurance pool (issue #366) ---
    /// Returned by `enable_mutual_insurance`-gated entrypoints
    /// (`join_mutual_insurance`, `claim_reward_with_mutual_contribution`,
    /// `file_mutual_loss_claim`) and by `disable_mutual_insurance` when the
    /// pool has never been configured, or has been disabled.
    MutualInsuranceNotActive = 66,
    /// Returned by `claim_reward_with_mutual_contribution` and
    /// `file_mutual_loss_claim` when the caller has not opted in via
    /// `join_mutual_insurance`.
    NotAMutualMember = 67,
    /// Returned by `vote_on_mutual_loss` and `resolve_mutual_loss_claim`
    /// when the given loss-event id does not exist.
    LossEventNotFound = 68,
    /// Returned by `vote_on_mutual_loss` and `resolve_mutual_loss_claim`
    /// when the referenced loss event has already been resolved.
    LossEventAlreadyResolved = 69,

    /// Returned by `claim_with_anti_dump_cooldown` (issue #365) when the
    /// caller is still inside the cooldown window started by a prior claim
    /// that exceeded the configured threshold.
    ClaimCooldownActive = 70,

    /// Returned by `sign_covenant` (issue #413) when the supplied
    /// `terms_hash` does not match the currently published pool terms.
    TermsMismatch = 71,
    /// Returned by `stake_with_covenant` (issue #413) when the caller has
    /// not signed the currently published pool terms.
    CovenantRequired = 72,
}
