#![no_std]

mod admin;
pub mod admin_succession; // designated heir admin activated on prolonged inactivity
pub mod anti_dump_claim_cooldown; // issue #365 — cooldown after large reward claims
mod balance;
pub mod errors;
mod events;
pub mod example_consumer;
pub mod interface;
pub mod nft;
mod storage;
mod vault;

// Features added as their own modules rather than inside `vault.rs`. Soroban
// supports several `#[contractimpl]` blocks for one contract type, and
// `nft.rs` already establishes the pattern, so each of these keeps its storage
// keys, types, and entrypoints together instead of appending to a 25k-line
// file. `DataKey` is at Soroban's 50-variant cap for `#[contracttype]` enums,
// so all of them use raw `Symbol`-keyed storage as `balance.rs` does.
pub mod boost_activation_age; // issue #401 — minimum position age before boost multipliers activate
pub mod capacity_forecast; // issue #402 — TVL capacity-cap arrival forecast from stake inflow
pub mod charitable_donation_routing; // issue #419 — charitable donation routing
pub mod collusion_detector; // issue #406 — coordinated stake/unstake pattern detector
pub mod combined_vesting; // issue #346 — cliff-then-linear combined reward vesting
pub mod comfort_score; // issue #399 — personalized pool comfort score for a user's risk profile
pub mod commitment; // issue #288 — commit–reveal stake commitments
pub mod competitive_season; // periodic leaderboard-resetting competitive seasons
pub mod compliance_report; // issue #409 — regulatory compliance report generator
pub mod compound_optimizer; // issue #338 — active claim/restake interval optimizer
pub mod content_curation; // content curation stake-weighted voting
pub mod daily_token_velocity_limiter; // issue #411 — pool-wide daily reward outflow cap
pub mod dex_limit_order_buyback; // issue #421 — limit order buyback mechanism
pub mod epoch_alignment; // issue #342 — calendar-style epoch boundary alignment
pub mod epoch_reward_cap; // per-epoch reward outflow cap with deferred overflow claims
pub mod governance_power_decay; // issue #404 — governance vote weight decay for long-inactive voters
pub mod insurance; // issue #289 — pool health insurance
pub mod keeper_registry; // approved-keeper registry with performance stats
pub mod minimum_reserve_ratio; // issue #405 — minimum reward-reserve ratio floor
pub mod mutual_insurance_pool; // issue #366 — peer mutual insurance pool
pub mod nft_fractionalize; // NFT receipt fractionalization
pub mod nft_redeem; // issue #410 — burn-and-redeem NFT-triggered position exit
pub mod partial_freeze; // issue #337 — partial position freeze
pub mod performance_league_table; // issue #373 — cross-pool performance league table
pub mod pool_clone_factory; // issue #412 — deploy new pool instances from this contract as template
pub mod pool_presale; // issue #369 — pool pre-sale reserved staking spots
pub mod position_dna; // deterministic staking position fingerprint (position DNA)
pub mod position_heartbeat; // issue #414 — periodic check-ins to maintain boosted reward status
pub mod position_shadow_clone; // issue #420 — read-only shadow clone of a position
pub mod price_oracle; // issue #290 — position price oracle
pub mod qr_metadata; // issue #324 — stake receipt QR metadata
pub mod reputation_decay; // reputation score time-decay mechanism
pub mod staker_sentiment_index;
pub mod staking_covenant; // issue #413 — on-chain commitment to pool terms by each staker
pub mod sub_unit_reward_accumulator; // issue #367 — fractional reward carry-forward below minimum transfer
pub mod tvl_rate_rebalance; // issue #333 — TVL-tiered pool reward rate rebalancing
pub mod twa_reward_rate; // issue #400 — time-weighted average reward rate for smoother pending-reward estimates
pub mod validator_rewards; // validator node reward integration
pub mod vesting_cliff; // issue #287 — reward vesting cliff
pub mod xlm_wrapper_integration; // issue #372 — auto-wrap native XLM to wXLM before staking // issue #422 — composite sentiment index

pub use nft::StakeReceiptNFT;
pub use vault::VaultContract;

#[cfg(test)]
mod test;

#[cfg(test)]
mod test_content_curation;

#[cfg(test)]
mod test_integration;

#[cfg(test)]
mod test_nft_fractionalize;

#[cfg(test)]
mod test_reputation_decay;

#[cfg(test)]
mod test_validator_rewards;

#[cfg(test)]
mod test_features_287_290;

#[cfg(test)]
mod test_reward_waterfall;

#[cfg(test)]
mod test_transfer_cooldown;

#[cfg(test)]
mod test_stake_quota;

#[cfg(test)]
mod test_slash_dispute;

#[cfg(test)]
mod test_nft_redeem;

#[cfg(test)]
mod test_compliance_report;

#[cfg(test)]
mod test_position_var;

#[cfg(test)]
mod test_staker_diversity;

#[cfg(test)]
mod test_comfort_score;

#[cfg(test)]
mod test_twa_reward_rate;

#[cfg(test)]
mod test_boost_activation_age;

#[cfg(test)]
mod test_capacity_forecast;

#[cfg(test)]
mod test_issues_419_422; // issue #419, #420, #421, #422
