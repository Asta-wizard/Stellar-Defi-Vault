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
pub mod bulk_supply_rewards; // issue #334 — bulk reward-pool top-up
pub mod commitment; // issue #288 — commit–reveal stake commitments
pub mod content_curation; // content curation stake-weighted voting
pub mod insurance; // issue #289 — pool health insurance
pub mod nft_fractionalize; // NFT receipt fractionalization
pub mod partial_freeze; // issue #337 — partial position freeze
pub mod price_oracle; // issue #290 — position price oracle
pub mod qr_metadata; // issue #324 — stake receipt QR metadata
pub mod reputation_decay; // reputation score time-decay mechanism
pub mod validator_delegation; // issue #332 — validator set delegation
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
