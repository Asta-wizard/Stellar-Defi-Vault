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
    /// notify_reward_added called with a zero or negative amount (issue #126).
    InvalidRewardAmount = 48,
    /// Returned when a new stake is attempted after `start_graceful_shutdown` has been called.
    PoolShuttingDown = 49,
    /// Reverts with NotInEpochMode error if pool is not configured for epoch mode.
    NotInEpochMode = 50,
}

/// Soroban caps every `#[contracterror]`/`#[contracttype]` enum at 50 variants
/// (`ScSpecUdtUnionV0::cases` is a `VecM<_, 50>` in stellar-xdr) — `VaultError`
/// above is already at exactly that cap, so new error cases for issues #205,
/// #206, and #209 can't be added to it. This second, separate error enum
/// holds just those new cases, plus mirrors of the handful of `VaultError`
/// cases the new functions can also hit (via the `From` impl below, so `?`
/// still works normally at call sites).
#[contracterror]
#[derive(Copy, Clone, Debug, Eq, PartialEq, PartialOrd, Ord)]
#[repr(u32)]
pub enum VaultExtError {
    /// Mirrors `VaultError::Unauthorized`.
    Unauthorized = 1,
    /// Mirrors `VaultError::NotInitialized`.
    NotInitialized = 2,
    /// Mirrors `VaultError::ZeroAmount`.
    ZeroAmount = 3,
    /// Mirrors `VaultError::ArithmeticError`.
    ArithmeticError = 4,
    /// Mirrors `VaultError::AlreadyInitialized` — returned by `import_state()`.
    AlreadyInitialized = 5,
    /// Returned by `set_insurance_rate_bps()` when `bps` exceeds 500 (5%)
    /// (issue #199).
    InvalidInsuranceRate = 6,
    /// Returned by `export_state()` when more than 100 positions would be
    /// exported (issue #203).
    TooManyPositions = 7,
    /// Returned by `set_fee_recipients()` when `recipients.len() > 5`
    /// (issue #197), and by `add_output_token()` when the output-token
    /// whitelist is already full (issue #244).
    TooManyRecipients = 8,
    /// Returned by `set_fee_recipients()` when the recipients' `share_bps`
    /// values don't sum to exactly 10 000 (issue #197).
    InvalidFeeAllocation = 9,
    /// Returned by `queue_action()` when 5 actions are already pending
    /// (issue #195).
    TooManyPendingActions = 10,
    /// Returned by `execute_action()`/`cancel_action()` when the given
    /// action id doesn't exist or was already executed/cancelled (issue
    /// #195).
    ActionNotFound = 11,
    /// Returned by `execute_action()` when called before `executable_at`
    /// (issue #195).
    ActionNotYetExecutable = 12,
    /// Returned by `initialize_multisig()` when `admins.len()` is 0 or > 5,
    /// or `threshold` is 0 or greater than `admins.len()` (issue #196).
    InvalidMultisigConfig = 13,
    /// Returned by `propose_action()` when 10 open proposals already exist
    /// (issue #196).
    TooManyProposals = 14,
    /// Returned by `propose_action()`/`approve_action()` when the caller is
    /// not one of the configured multisig admins (issue #196).
    NotAMultisigAdmin = 15,
    /// Returned by `approve_action()` when the caller already approved this
    /// proposal (issue #196).
    AlreadyApproved = 16,
    /// Returned by `execute_proposal()` when approvals are below the
    /// configured threshold, or the proposal id doesn't exist / was already
    /// executed (issue #196).
    ProposalNotReady = 17,
    /// Returned by `rollback_last_rate_change()` when there is no previous
    /// rate to restore, or it was already rolled back (issue #206).
    RollbackUnavailable = 18,
    /// Returned by `swap_and_stake()` when the DEX swap's output amount is
    /// below the caller-supplied `min_stake_amount` (issue #205).
    SlippageExceeded = 19,
    /// Returned by `swap_and_stake()` when `input_token` is not registered
    /// with a DEX router capable of swapping it to the stake token, or when
    /// no DEX router has been configured at all (issue #205). Also returned
    /// by `claim_in_token()` for an `output_token` that is not on the
    /// admin-managed whitelist (issue #244).
    UnsupportedInputToken = 20,
    /// Returned by `position_split()` when `split_amount` is not strictly
    /// between 0 and the caller's current position amount (issue #209).
    InvalidSplitAmount = 21,
    /// Returned by `claim_merkle_reward()` when the Merkle proof is invalid.
    MerkleInvalidProof = 22,
    /// Returned by `claim_merkle_reward()` when the user has already claimed
    /// for the given epoch.
    MerkleAlreadyClaimed = 23,
    /// Returned by `create_tournament()` when a tournament is already active.
    TournamentAlreadyExists = 24,
    /// Returned by `finalize_tournament()` when the tournament has not ended yet.
    TournamentNotEnded = 25,
    /// Returned by `finalize_tournament()` when no tournament exists.
    TournamentNotFound = 26,
    /// Returned by `compare_pools()` when more than 5 external pools are supplied.
    TooManyPools = 27,
    /// Returned by `execute_buyback()` when buyback is not enabled.
    BuybackNotEnabled = 28,
    /// Returned by `execute_buyback()` when accrued fees are below the threshold.
    BuybackThresholdNotMet = 29,
    /// Returned by stake/claim rate-limit checks (issue #201).
    RateLimitExceeded = 30,
    /// Returned by `start_bootstrap()` when `initial_rate < base_rate`.
    InvalidBootstrapConfig = 31,
    /// Returned by delegation chain operations when a cycle is detected (issue #200).
    CircularDelegation = 32,
    /// Returned when a delegation chain would exceed the maximum length (issue #200).
    ChainTooLong = 33,
    /// Returned by `issue_certificate` when the user's stake is below the minimum (issue #222).
    IneligibleForCertificate = 34,
    /// Returned by `set_reward_smoothing()` when the smoothing period exceeds
    /// `MAX_SMOOTHING_PERIOD_LEDGERS`, or when `min_amount` is negative
    /// (issue #235).
    InvalidSmoothingConfig = 35,
    /// Returned by `referral_tree_data()` when `max_level` exceeds
    /// `MAX_REFERRAL_TREE_DEPTH` (issue #236).
    ReferralDepthTooDeep = 36,
    /// Returned by `start_capacity_auction()` when an auction is already open,
    /// and by `place_bid()` / `finalize_capacity_auction()` when no auction
    /// exists (issue #237).
    AuctionNotFound = 37,
    /// Returned by `start_capacity_auction()` when `spots` is 0 or above
    /// `MAX_AUCTION_SPOTS`, or `duration_ledgers` is 0 (issue #237).
    InvalidAuctionConfig = 38,
    /// Returned by `start_capacity_auction()` when the previous auction has not
    /// been finalized yet (issue #237).
    AuctionAlreadyActive = 39,
    /// Returned by `place_bid()` after the auction window has closed, and by
    /// `place_bid()` / `finalize_capacity_auction()` on an already-finalized
    /// auction (issue #237).
    AuctionClosed = 40,
    /// Returned by `finalize_capacity_auction()` when called before the auction
    /// window has elapsed (issue #237).
    AuctionNotEnded = 41,
    /// Returned by `place_bid()` when the resulting bid is below the auction's
    /// `min_bid` (issue #237).
    BidBelowMinimum = 42,
    /// Returned by `place_bid()` when the auction already holds
    /// `MAX_AUCTION_BIDS` distinct bidders (issue #237).
    TooManyBids = 43,
    /// Returned by `create_lottery()` when an undrawn lottery already exists,
    /// and by `draw_lottery()` when no lottery is configured or it was
    /// already drawn (issue #239).
    LotteryAlreadyActive = 44,
    /// Returned by `draw_lottery()` when called before `draw_at_ledger`
    /// (issue #239).
    LotteryNotReady = 45,
    /// Returned by `add_milestone()` when 10 milestones are already
    /// configured (issue #238).
    TooManyMilestones = 46,
    /// Returned by `check_and_release()` when no oracle contract has been
    /// registered via `set_oracle_contract()` (issue #240).
    NoOracleConfigured = 47,
    /// Returned by `veto_proposal()` when the caller's pool share is below
    /// the configured veto threshold, or the veto threshold is unset (0),
    /// which disables the feature entirely (issue #241).
    BelowVetoThreshold = 48,
    /// Returned by `veto_proposal()` when the proposal already has a vetoer,
    /// or has already been enacted and so can no longer be vetoed (issue
    /// #241).
    AlreadyVetoed = 49,
    /// Returned by `set_veto_threshold_bps()` when `bps` exceeds 10 000
    /// (100%) (issue #241). Both error enums are at Soroban's 50-variant cap,
    /// so this doubles as the generic "basis-points value out of range" code —
    /// `start_matching_program()` also returns it for a `match_rate_bps` above
    /// 10 000 (issue #242).
    InvalidVetoThreshold = 50,
}

/// Third error enum, for the same 50-variant reason `VaultExtError` exists:
/// both `VaultError` and `VaultExtError` are now at exactly the cap, so the
/// cases introduced by issues #258 (branding), #259 (staking insurance),
/// #260 (flash stake), #261 (stake-backed loans), #275 (reward Gini
/// coefficient), #276 (seasonal reward multiplier), #274 (staker bio), and
/// #298 (pool sunsetting workflow) live here, plus mirrors of the handful of
/// `VaultError` cases those functions can also hit (via the `From` impl
/// below, so `?` still works normally at call sites).
///
/// Note on `InvalidBrandingField`: a `#[contracterror]` variant cannot carry a
/// payload, so the offending field name is encoded in the variant itself —
/// `InvalidBrandingDisplayName`, `InvalidBrandingLogoHash`,
/// `InvalidBrandingWebsiteUrl`, `InvalidBrandingTwitterHandle` — rather than
/// as an inner `String` on a single generic variant.
#[contracterror]
#[derive(Copy, Clone, Debug, Eq, PartialEq, PartialOrd, Ord)]
#[repr(u32)]
pub enum VaultFeatureError {
    /// Mirrors `VaultError::Unauthorized`.
    Unauthorized = 1,
    /// Mirrors `VaultError::NotInitialized`.
    NotInitialized = 2,
    /// Mirrors `VaultError::ZeroAmount`.
    ZeroAmount = 3,
    /// Mirrors `VaultError::ArithmeticError`.
    ArithmeticError = 4,
    /// Mirrors `VaultError::PositionNotFound`.
    PositionNotFound = 5,
    /// Mirrors `VaultError::VaultPaused`.
    VaultPaused = 6,
    /// Mirrors `VaultError::InsufficientRewardPool`.
    InsufficientRewardPool = 7,

    // ── Issue #258: branding ────────────────────────────────────────────────
    /// `set_branding()`: `display_name` exceeds `MAX_BRANDING_NAME_LEN` (50).
    InvalidBrandingDisplayName = 8,
    /// `set_branding()`: `logo_hash` exceeds `MAX_BRANDING_LOGO_LEN` (64).
    InvalidBrandingLogoHash = 9,
    /// `set_branding()`: `website_url` exceeds `MAX_BRANDING_URL_LEN` (200).
    InvalidBrandingWebsiteUrl = 10,
    /// `set_branding()`: `twitter_handle` exceeds `MAX_BRANDING_TWITTER_LEN` (16).
    InvalidBrandingTwitterHandle = 11,

    // ── Issue #259: staking insurance ───────────────────────────────────────
    /// `set_insurance_product()`: `premium_bps` above 10 000, or
    /// `max_coverage_per_user` is negative.
    InvalidInsuranceProduct = 12,
    /// `purchase_insurance()` before any `set_insurance_product()` call.
    InsuranceProductNotSet = 13,
    /// `purchase_insurance()` when the caller already holds an active policy.
    InsuranceAlreadyActive = 14,
    /// `cancel_insurance()` / `declare_shortfall()` for a user with no policy.
    InsurancePolicyNotFound = 15,
    /// `declare_shortfall()` when the insurance fund cannot cover the total
    /// coverage owed to `affected_users`.
    InsuranceFundInsufficient = 16,
    /// `declare_shortfall()` when `affected_users.len()` exceeds
    /// `MAX_SHORTFALL_USERS` (50) — split the payout across several calls.
    TooManyAffectedUsers = 23,

    // ── Issue #275: reward Gini coefficient ─────────────────────────────────
    /// `get_reward_gini_coefficient()`: more than 100 active stakers. Named
    /// identically to (but distinct from) `VaultError::TooManyStakers`, which
    /// is already used for an unrelated case (`vote()` double-voting) — both
    /// `VaultError` and `VaultExtError` are at their 50-variant cap so this
    /// domain's own error lives here instead.
    TooManyStakers = 24,

    // ── Issue #276: seasonal reward multiplier ──────────────────────────────
    /// `add_season()`: `starts_at >= ends_at`, or `multiplier_bps` is zero.
    InvalidSeasonConfig = 25,
    /// `add_season()`: 10 seasons are already scheduled.
    TooManySeasons = 26,
    /// `add_season()`: the requested range overlaps an already-scheduled
    /// season — only one season may be active at a time.
    SeasonOverlap = 27,
    /// `remove_season()`: no season exists at the given index.
    SeasonNotFound = 28,

    // ── Issue #274: staker bio ───────────────────────────────────────────────
    /// `set_staker_bio()`: `bio` exceeds `MAX_BIO_LEN` (160 characters).
    BioTooLong = 29,

    // ── Issue #298: pool sunsetting workflow ────────────────────────────────
    /// Returned by every sunset-workflow entrypoint when called from the
    /// wrong `SunsetState` (transitions are one-way and only valid from a
    /// specific prior state), and by `start_force_resolution()` when called
    /// before the grace period configured in `announce_sunset()` has elapsed.
    InvalidSunsetTransition = 30,
    /// `close_pool()`: at least one staker still holds an active position —
    /// every position must be resolved (via `force_resolve_position()` or a
    /// voluntary `unstake()`) before the pool can close.
    PositionsStillActive = 31,

    // ── Issue #260: flash stake ─────────────────────────────────────────────
    /// `set_flash_stake_fee_bps()`: `bps` above 10 000.
    InvalidFlashStakeFee = 17,

    // ── Issue #261: stake-backed loans ──────────────────────────────────────
    /// `set_loan_config()`: `max_ltv_bps` or `interest_rate_bps` above 10 000.
    InvalidLoanConfig = 18,
    /// `borrow()` before any `set_loan_config()` call.
    LoanConfigNotSet = 19,
    /// `repay()` / `liquidate_loan()` / `get_loan()` flows for a user with no
    /// outstanding loan.
    LoanNotFound = 20,
    /// `borrow()` when the requested amount would push total debt above
    /// `position.amount * max_ltv_bps / 10000`.
    ExceedsMaxLtv = 21,
    /// `liquidate_loan()` when the borrower's LTV is still below
    /// `LIQUIDATION_LTV_BPS` (9 000).
    LoanNotLiquidatable = 22,

    // ── Issue #286: debt NFT collateral ──────────────────────────────────
    /// `mint_debt_nft()`: user already has an outstanding debt NFT.
    PositionCollateralized = 32,
    /// `mint_debt_nft()`: face_value exceeds the user's staking position.
    FaceValueExceedsPosition = 33,
    /// `burn_debt_nft()` / `transfer_debt_nft()` / `get_debt_nft()`: NFT not found.
    DebtNftNotFound = 34,
    /// `transfer_debt_nft()`: caller is not the current holder.
    NotNftHolder = 35,

    // ── Issue #285: cross-pool yield detector ────────────────────────────
    /// `set_competitor_pools()`: more than 10 competitor addresses.
    TooManyCompetitors = 36,

    // ── Issue #283: position AMM ──────────────────────────────────────────
    /// `create_swap_offer()`: caller already has 5 open offers.
    TooManyOpenOffers = 37,
    /// `accept_swap_offer()` / `cancel_swap_offer()`: offer id not found.
    OfferNotFound = 38,
    /// `accept_swap_offer()`: offer has expired.
    OfferExpired = 39,
    /// `accept_swap_offer()`: caller is not the requested counterparty.
    NotRequestedCounterparty = 40,
    /// `create_swap_offer()`: requested swap amount exceeds position size.
    InsufficientAmount = 41,

    // ── Issue #284: reward prediction market ──────────────────────────────
    /// `open_prediction_market()`: a market is already open.
    MarketAlreadyOpen = 42,
    /// `place_bet()`: no market is currently open.
    NoMarketOpen = 43,
    /// `place_bet()`: betting window has closed.
    BettingClosed = 44,
    /// `resolve_market()`: target ledger has not been reached yet.
    MarketNotReady = 45,
    /// `claim_prediction_winnings()`: market not yet resolved.
    MarketNotResolved = 46,
    /// `claim_prediction_winnings()`: user already claimed.
    AlreadyClaimed = 47,
    /// `place_bet()`: caller is not an active staker.
    NotActiveStaker = 48,

    // ── Issue #284: used by open_prediction_market validation ──────────────
    /// `open_prediction_market()`: target_ledger in the past or closes_at >= target_ledger.
    InvalidRate = 49,
}

impl From<VaultError> for VaultExtError {
    fn from(err: VaultError) -> Self {
        match err {
            VaultError::Unauthorized => VaultExtError::Unauthorized,
            VaultError::NotInitialized => VaultExtError::NotInitialized,
            VaultError::ZeroAmount => VaultExtError::ZeroAmount,
            VaultError::ArithmeticError => VaultExtError::ArithmeticError,
            // Any other VaultError reaching here (shouldn't happen given how
            // these functions are written) maps to the closest generic case.
            _ => VaultExtError::Unauthorized,
        }
    }
}

impl From<VaultError> for VaultFeatureError {
    fn from(err: VaultError) -> Self {
        match err {
            VaultError::Unauthorized => VaultFeatureError::Unauthorized,
            VaultError::NotInitialized => VaultFeatureError::NotInitialized,
            VaultError::ZeroAmount => VaultFeatureError::ZeroAmount,
            VaultError::ArithmeticError => VaultFeatureError::ArithmeticError,
            VaultError::PositionNotFound => VaultFeatureError::PositionNotFound,
            VaultError::VaultPaused => VaultFeatureError::VaultPaused,
            VaultError::InsufficientRewardPool => VaultFeatureError::InsufficientRewardPool,
            // Any other VaultError reaching here (shouldn't happen given how
            // these functions are written) maps to the closest generic case.
            _ => VaultFeatureError::Unauthorized,
        }
    }
}
