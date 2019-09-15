// Copyright 2018 Commonwealth Labs, Inc.
// This file is part of Straightedge.

// Straightedge is free software: you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.

// Straightedge is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
// GNU General Public License for more details.

// You should have received a copy of the GNU General Public License
// along with Straightedge.  If not, see <http://www.gnu.org/licenses/>

use primitives::{Pair, Public};
use straightedge_primitives::{AccountId, AuraId, Balance};
use im_online::ed25519::{AuthorityId as ImOnlineId};
use straightedge_runtime::{
	GrandpaConfig, BalancesConfig, ContractsConfig, ElectionsConfig, DemocracyConfig, CouncilConfig,
	AuraConfig, IndicesConfig, SessionConfig, StakingConfig, SudoConfig, TreasuryRewardConfig,
	SystemConfig, ImOnlineConfig, WASM_BINARY, Perbill, SessionKeys, StakerStatus, AuthorityDiscoveryConfig,
};
use straightedge_runtime::constants::{time::DAYS, currency::DOLLARS, currency::MILLICENTS};
use straightedge_runtime::{IdentityConfig, SignalingConfig};
pub use straightedge_runtime::GenesisConfig;
use substrate_service;
use substrate_telemetry::TelemetryEndpoints;
use grandpa::AuthorityId as GrandpaId;
use crate::fixtures::*;
use sr_primitives::{
	traits::{One},
};
use core::convert::TryInto;

const STAGING_TELEMETRY_URL: &str = "wss://telemetry.polkadot.io/submit/";
const DEFAULT_PROTOCOL_ID: &str = "str8";

/// Specialised `ChainSpec`.
pub type ChainSpec = substrate_service::ChainSpec<GenesisConfig>;

pub fn straightedge_mainnet() -> ChainSpec {
	match ChainSpec::from_json_file(std::path::PathBuf::from("mainnet/genesis.json")) {
		Ok(spec) => spec,
		Err(e) => panic!(e),
	}	
}

/// The generation script for the recommended mainnet genesis configuration
pub fn straightedge_mainnet_config_genesis() -> GenesisConfig {
	// genesis allocation
	let genesis_allocation: Vec<(AccountId, Balance)> = get_genesis_allocation();
	// genesis authorities for mainnet
	let genesis_authorities: Vec<(
		AccountId,
		AccountId,
		AuraId,
		Balance,
		GrandpaId,
		ImOnlineId
	)> = get_genesis_mainnet_validators();
	let initial_lockdrop_authorities: Vec<(
		AccountId,
		AccountId,
		AuraId,
		Balance,
		GrandpaId,
		ImOnlineId
	)> = get_lockdrop_mainnet_validators();
	// lockdrop spec
	let spec = get_spec_allocation().unwrap();
	let lockdrop_balances = spec.0;
	let lockdrop_vesting = spec.1;
	// genesis root key
	let root_key = get_mainnet_root_key();
	// genesis identity verifiers
	let identity_verifiers = get_mainnet_identity_verifiers();
	// initial election members, should only have 1 commonwealth account at start
	let election_members = get_mainnet_election_members();
	// session keys contain genesis authorities
	let session_keys = genesis_authorities.iter().map(|x| (x.0.clone(), session_keys(x.2.clone(), x.4.clone(), x.5.clone())))
		.chain(initial_lockdrop_authorities.iter().map(|x| (x.0.clone(), session_keys(x.2.clone(), x.4.clone(), x.5.clone()))))
		.collect::<Vec<_>>();

	GenesisConfig {
		system: Some(SystemConfig {
			code: WASM_BINARY.to_vec(),
			changes_trie_config: Default::default(),
		}),
		balances: Some(BalancesConfig {
			balances: genesis_allocation.iter().map(|x| (x.0.clone(), x.1.clone()))
				.chain(genesis_authorities.iter().map(|x| (x.0.clone(), x.3.clone()))) // stash accounts
				.chain(genesis_authorities.iter().map(|x| (x.1.clone(), CONTROLLER_ENDOWMENT))) // controller accounts
				.chain(lockdrop_balances.iter().map(|x| (x.0.clone(), x.1.clone()))) // lockdrop accounts
				.collect(),
			vesting: genesis_authorities.iter().map(|x| (x.0.clone(), 1314000, 1, x.3.clone()))
				.chain(genesis_allocation.iter().map(|x| (x.0.clone(), 1314000, 1, x.1.clone())))
				.chain(lockdrop_balances.iter().map(|x| (x.0.clone(), 438000, 1, CONTROLLER_ENDOWMENT)))
				.collect(),
		}),
		indices: Some(IndicesConfig {
			ids: genesis_allocation.iter().map(|x| x.0.clone())
				.chain(genesis_authorities.iter().map(|x| x.0.clone()))
				.chain(genesis_authorities.iter().map(|x| x.1.clone()))
				.chain(lockdrop_balances.iter().map(|x| x.0.clone()))
				.collect::<Vec<_>>(),
		}),
		session: Some(SessionConfig {
			keys: session_keys,
		}),
		staking: Some(StakingConfig {
			current_era: 0,
			validator_count: 60,
			minimum_validator_count: 0,
			stakers: genesis_authorities.iter().map(|x| (
				x.0.clone(),
				x.1.clone(),
				// Ensure stakers have some non-bonded balance
				x.3.clone() - CONTROLLER_ENDOWMENT,
				StakerStatus::Validator
			)).chain(initial_lockdrop_authorities.iter().map(|x| (
				x.0.clone(),
				x.1.clone(),
				x.3.clone(),
				StakerStatus::Validator
			))).collect(),
			invulnerables: vec![],
			slash_reward_fraction: Perbill::from_percent(0),
			.. Default::default()
		}),
		democracy: Some(DemocracyConfig::default()),
		collective_Instance1: Some(CouncilConfig {
			members: election_members.iter().map(|x| x.clone()).collect(),
			phantom: Default::default(),
		}),
		elections: Some(ElectionsConfig {
			members: election_members.iter().map(|x| (x.clone(), 6 * 28 * DAYS)).collect(),
			desired_seats: 6,
			presentation_duration: (3 * DAYS).try_into().unwrap(),
			term_duration: (6 * 28 * DAYS).try_into().unwrap(),
		}),
		contracts: Some(ContractsConfig {
			current_schedule: Default::default(),
			gas_price: 1 * MILLICENTS,
		}),
		sudo: Some(SudoConfig {
			key: root_key,
		}),
		im_online: Some(ImOnlineConfig {
			keys: vec![],
		}),
		aura: Some(AuraConfig {
			authorities: vec![],
		}),
		grandpa: Some(GrandpaConfig {
			authorities: vec![],
		}),
		authority_discovery: Some(AuthorityDiscoveryConfig{
			keys: vec![],
		}),
		identity: Some(IdentityConfig {
			verifiers: identity_verifiers,
			expiration_length: (7 * DAYS).try_into().unwrap(),
			registration_bond: 1 * DOLLARS,
		}),
		signaling: Some(SignalingConfig {
			voting_length: (14 * DAYS).try_into().unwrap(),
			proposal_creation_bond: 100 * DOLLARS,
		}),
		treasury_reward: Some(TreasuryRewardConfig {
			current_payout: 95 * DOLLARS,
			minting_interval: One::one(),
		}),
	}
}


/// Straightedge mainnet generator
pub fn straightedge_mainnet_config() -> Result<ChainSpec, String> {
	let boot_nodes = vec![
		"/ip4/159.65.223.215/tcp/30333/p2p/QmTVhTDnxBBjAGmgGXmha4zJ1E8CUPoeC4Rmi3LWyoLVVB".to_string(), // Sunny
		"/ip4/134.209.244.243/tcp/30333/p2p/QmbkJhL1Drr67BzGv8HxTnXeJfU9jKvj7LH6pp5A4Atoq6".to_string(), // Chris
		"/ip4/35.157.118.166/tcp/30333/p2p/QmdfPKgYx61H2X6xGiwfdMCswekxxoYB4E9askgfLegVK5".to_string(), // Julien
		"/ip4/165.227.18.43/tcp/30333/p2p/QmVs8tjmd6ShJjcgQ2QoMfCMJc8mKqr5oe2rLWPnNqSxtF".to_string(), // maskofice
		"/ip4/178.128.231.97/tcp/30333/p2p/Qmf6WAJRB4Bhr7WMRcmZaDJtgScAKMoyAjmTg6LWT4A78Q".to_string(), // briancohen
		"/ip4/3.15.223.164/tcp/30333/p2p/QmY4ZGbFCwXP29u1k1gsf9prCThvMaHiYXovCrBvpDq6A6".to_string(), // florian
	];

	let data = r#"
		{
			"tokenDecimals": 18,
			"tokenSymbol": "STR"
		}"#;
	let properties = serde_json::from_str(data).unwrap();

	Ok(ChainSpec::from_genesis(
		"Straightedge",
		"straightedgeXX",
		straightedge_mainnet_config_genesis,
		boot_nodes,
		Some(TelemetryEndpoints::new(vec![(STAGING_TELEMETRY_URL.to_string(), 0)])),
		Some(DEFAULT_PROTOCOL_ID),
		None,
		properties,
	))
}


fn session_keys(aura: AuraId, grandpa: GrandpaId, im_online: ImOnlineId) -> SessionKeys {
	SessionKeys { aura, grandpa, im_online }
}

/// Helper function to generate a crypto pair from seed
pub fn get_from_seed<TPublic: Public>(seed: &str) -> <TPublic::Pair as Pair>::Public {
	TPublic::Pair::from_string(&format!("//{}", seed), None)
		.expect("static values are valid; qed")
		.public()
}

/// Helper function to generate stash, controller and session key from seed
pub fn get_authority_keys_from_seed(seed: &str) -> (AccountId, AccountId, AuraId, GrandpaId, ImOnlineId) {
	(
		get_from_seed::<AccountId>(&format!("{}//stash", seed)),
		get_from_seed::<AccountId>(seed),
		get_from_seed::<AuraId>(seed),
		get_from_seed::<GrandpaId>(seed),
		get_from_seed::<ImOnlineId>(seed),
	)
}

/// Helper function to create GenesisConfig for testing
pub fn development_genesis(
	initial_authorities: Vec<(AccountId, AccountId, AuraId, GrandpaId, ImOnlineId)>,
	root_key: AccountId,
	endowed_accounts: Option<Vec<AccountId>>,
	initial_verifiers: Option<Vec<AccountId>>,
) -> GenesisConfig {
	let initial_verifiers: Vec<AccountId> = initial_verifiers.unwrap_or_else(|| {
		vec![
			get_authority_keys_from_seed("Alice").1,
		]
	});

	let endowed_accounts: Vec<AccountId> = endowed_accounts.unwrap_or_else(|| {
		vec![
			get_authority_keys_from_seed("Alice").1,
			get_authority_keys_from_seed("Bob").1,
			get_authority_keys_from_seed("Charlie").1,
			get_authority_keys_from_seed("Dave").1,
			get_authority_keys_from_seed("Eve").1,
			get_authority_keys_from_seed("Ferdie").1,
		]
	});

	const ENDOWMENT: Balance = 10_000_000 * DOLLARS;
	const STASH: Balance = 100 * DOLLARS;
	let desired_seats: u32 = (endowed_accounts.len() / 2 - initial_authorities.len()) as u32;

	GenesisConfig {
		system: Some(SystemConfig {
			code: WASM_BINARY.to_vec(),
			changes_trie_config: Default::default(),
		}),
		balances: Some(BalancesConfig {
			balances: endowed_accounts.iter().cloned()
				.map(|k| (k, ENDOWMENT))
				.chain(initial_authorities.iter().map(|x| (x.0.clone(), STASH)))
				.collect(),
			vesting: vec![],
		}),
		indices: Some(IndicesConfig {
			ids: endowed_accounts.iter().cloned()
				.chain(initial_authorities.iter().map(|x| x.0.clone()))
				.collect::<Vec<_>>(),
		}),
		session: Some(SessionConfig {
			keys: initial_authorities.iter().map(|x|
				(x.0.clone(), session_keys(x.2.clone(), x.3.clone(), x.4.clone()))
			).collect::<Vec<_>>(),
		}),
		staking: Some(StakingConfig {
			current_era: 0,
			validator_count: 7,
			minimum_validator_count: 4,
			stakers: initial_authorities.iter().map(|x| (x.0.clone(), x.1.clone(), STASH, StakerStatus::Validator)).collect(),
			invulnerables: initial_authorities.iter().map(|x| x.0.clone()).collect(),
			slash_reward_fraction: Perbill::from_percent(10),
			.. Default::default()
		}),
		democracy: Some(DemocracyConfig::default()),
		collective_Instance1: Some(CouncilConfig {
			members: initial_authorities.iter().map(|x| x.1.clone()).collect(),
			phantom: Default::default(),
		}),
		elections: Some(ElectionsConfig {
			members: initial_authorities.iter().map(|x| (x.1.clone(), 1000000)).collect(),
			desired_seats: desired_seats,
			presentation_duration: (1 * DAYS).try_into().unwrap(),
			term_duration: (28 * DAYS).try_into().unwrap(),
		}),
		contracts: Some(ContractsConfig {
			current_schedule: Default::default(),
			gas_price: 1 * MILLICENTS,
		}),
		sudo: Some(SudoConfig {
			key: root_key,
		}),
		im_online: Some(ImOnlineConfig {
			keys: vec![],
		}),
		aura: Some(AuraConfig {
			authorities: vec![],
		}),
		grandpa: Some(GrandpaConfig {
			authorities: vec![],
		}),
		authority_discovery: Some(AuthorityDiscoveryConfig{
			keys: vec![],
		}),
		identity: Some(IdentityConfig {
			verifiers: initial_verifiers,
			expiration_length: (1 * DAYS).try_into().unwrap(), // 1 days
			registration_bond: 1 * DOLLARS,
		}),
		signaling: Some(SignalingConfig {
			voting_length: (3 * DAYS).try_into().unwrap(), // 7 days
			proposal_creation_bond: 100 * DOLLARS,
		}),
		treasury_reward: Some(TreasuryRewardConfig {
			current_payout: 158 * DOLLARS,
			minting_interval: One::one(),
		}),
	}
}

fn development_config_genesis() -> GenesisConfig {
	development_genesis(
		vec![
			get_authority_keys_from_seed("Alice"),
		],
		get_authority_keys_from_seed("Alice").0,
		None,
		None,
	)
}

/// Development config (single validator Alice)
pub fn development_config() -> ChainSpec {
	ChainSpec::from_genesis(
		"Development",
		"dev",
		development_config_genesis,
		vec![],
		None,
		Some(DEFAULT_PROTOCOL_ID),
		None,
		None)
}

fn local_development_genesis() -> GenesisConfig {
	development_genesis(
		vec![
			get_authority_keys_from_seed("Alice"),
			get_authority_keys_from_seed("Bob"),
		],
		get_authority_keys_from_seed("Alice").0,
		None,
		None,
	)
}

/// Local testnet config (multivalidator Alice + Bob)
pub fn local_testnet_config() -> ChainSpec {
	ChainSpec::from_genesis(
		"Local Testnet",
		"local_testnet",
		local_development_genesis,
		vec![],
		None,
		Some(DEFAULT_PROTOCOL_ID),
		None,
		None)
}
