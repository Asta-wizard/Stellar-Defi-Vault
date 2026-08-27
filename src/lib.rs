#![no_std]

mod admin;
mod balance;
mod errors;
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
pub mod combined_vesting; // issue #346 — cliff-then-linear combined reward vesting
pub mod commitment; // issue #288 — commit–reveal stake commitments
pub mod compound_optimizer; // issue #338 — active claim/restake interval optimizer
pub mod content_curation; // content curation stake-weighted voting
pub mod epoch_alignment; // issue #342 — calendar-style epoch boundary alignment
pub mod epoch_reward_cap; // per-epoch reward outflow cap with deferred overflow claims
pub mod insurance; // issue #289 — pool health insurance
pub mod keeper_registry; // approved-keeper registry with performance stats
pub mod nft_fractionalize; // NFT receipt fractionalization
pub mod partial_freeze; // issue #337 — partial position freeze
pub mod position_dna; // deterministic staking position fingerprint (position DNA)
pub mod price_oracle; // issue #290 — position price oracle
pub mod qr_metadata; // issue #324 — stake receipt QR metadata
pub mod reputation_decay; // reputation score time-decay mechanism
pub mod tvl_rate_rebalance; // issue #333 — TVL-tiered pool reward rate rebalancing
pub mod validator_rewards; // validator node reward integration
pub mod vesting_cliff; // issue #287 — reward vesting cliff

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
