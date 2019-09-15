use straightedge_primitives::{BlockNumber, Balance, AuraId, AccountId};
use grandpa::AuthorityId as GrandpaId;
use im_online::ed25519::{AuthorityId as ImOnlineId};
use primitives::crypto::UncheckedInto;
use hex_literal::hex;
use serde::{Deserialize, Serialize};
use serde_json::{Result};
use std::fs::File;
use std::io::Read;
use hex::FromHex;

#[derive(Serialize, Deserialize)]
struct Allocation {
    balances: Vec<(String, String)>,
    vesting: Vec<(String, u32, u32, String)>,
    validators: Vec<(String, String, String, String)>,
}

pub fn get_spec_allocation() 
	-> Result<(
		Vec<(AccountId, Balance)>,
		Vec<(AccountId, BlockNumber, BlockNumber, Balance)>,
	)> {
    let mut file = File::open("node/service/src/genesis.json").unwrap();
    let mut data = String::new();
    file.read_to_string(&mut data).unwrap();

	let json: Allocation = serde_json::from_str(&data)?;;
	let balances_json = json.balances;
	let vesting_json = json.vesting;

	let balances: Vec<(AccountId, Balance)> = balances_json.into_iter().map(|e| {
		return (
			<[u8; 32]>::from_hex(e.0).unwrap().unchecked_into(),
			e.1.to_string().parse::<Balance>().unwrap(),
		);
	}).collect();
	let vesting: Vec<(AccountId, BlockNumber, BlockNumber, Balance)> = vesting_json.into_iter().map(|e| {
		return (
			<[u8; 32]>::from_hex(e.0).unwrap().unchecked_into(),
			e.1.to_string().parse::<BlockNumber>().unwrap(),
			e.2.to_string().parse::<BlockNumber>().unwrap(),
			e.3.to_string().parse::<Balance>().unwrap()
		);
	}).collect();
	Ok((balances, vesting))
}

/// This is the allocation that will fit into the "balances" collection
/// of the "balances" module. The total should be 5e26 - 1e26 or 400 million EDG,
/// since we have also allocated 1e26 to the validators below.
pub fn get_genesis_allocation() -> Vec<(AccountId, Balance)> {
	return vec![(
		hex!["1ec5e3d9a77ac81d6da0290c04d003bbcb04af8c4902bd59dbf9be4dfa47234f"].unchecked_into(),
		5000000000000000000000000,
	), (
		hex!["720967cda4c9097924d705695b62dfb6dc6dbeade65b5575abf5c4ca38e50503"].unchecked_into(),
		5000000000000000000000000,
	), (
		hex!["de90c8b070c0a63fbf52655af7492dc8e7d985334a4c60c02bc2f59424ff1430"].unchecked_into(),
		5000000000000000000000000,
	), (
		hex!["9665bd715c72b686c2557fe11e6cd54924adef62dc1f52cf43a503f363cf843c"].unchecked_into(),
		5000000000000000000000000,
	), (
		hex!["464c96a206e310511a27acc92b2e410a14bd83cb8788b522d0cee03f0d285862"].unchecked_into(),
		5000000000000000000000000,
	), (
		hex!["34c71b1e42e910b94b8cbb2c960873bd4bf0db6e80afdf41cdc52acd91d6393f"].unchecked_into(),
		5000000000000000000000000,
	), (
		hex!["d02002915139ac3e4552c5006f92cccfbf8b02cb4d4ca1993a69d63368cc1f0c"].unchecked_into(),
		5000000000000000000000000,
	), (
		hex!["143f9f9019fa62919ed6da39b8f147cb52501438e1e9e7a82d22d7b49df18e59"].unchecked_into(),
		5000000000000000000000000,
	), (
		hex!["4e7de9c8f3564fe5cc5057de51c41b40a7da801d22c6ee5aa57f8bb2838ae857"].unchecked_into(),
		5000000000000000000000000,
	), (
		hex!["ce64070e4dffe61183241dca3e922b65ecd509430a3e283fab5c143532f79d3e"].unchecked_into(),
		5000000000000000000000000,
	), (
		hex!["c4fff312e8e224c9f1380d235c1a601d9f00213e6b4f4c86250f768563cb6f2a"].unchecked_into(),
		5000000000000000000000000,
	), (
		hex!["3a9fcaef453185230c2d777fa7dda16a745f6840b0cbc28f3e8c2ab07e533d3b"].unchecked_into(),
		5000000000000000000000000,
	), (
		hex!["e280d0fbfbcbdc070526d0997d0bfec3b3527b036bf5a68f87e6cc0422ced302"].unchecked_into(),
		5000000000000000000000000,
	), (
		hex!["865d38926f2344541912b464e2bb910ef72f2c9d447bc26f996b46f67605b85b"].unchecked_into(),
		5000000000000000000000000,
	), (
		hex!["6ea38f9aeb405131cfcd8e542d72cd2a3f3e56427b9162298d8e7f3529a6c34e"].unchecked_into(),
		5000000000000000000000000,
	), (
		hex!["968a20a9c04a662831afd93d8ad834f69c6aeb37e9017b4b739fd94502aae543"].unchecked_into(),
		5000000000000000000000000,
	), (
		hex!["fce2e25267c2f45932ca1cc9834d2db0b5438804af69a8af16bade1ca00c160d"].unchecked_into(),
		5000000000000000000000000,
	), (
		hex!["86cdb7761daa3c8917719b057e83ca0377de8ca599624c4ff7bd541ab056427d"].unchecked_into(),
		5000000000000000000000000,
	), (
		hex!["dc1ec6087624728ec118c4b4cd8e58bfe8c977195aac172473262242fba5d160"].unchecked_into(),
		5000000000000000000000000,
	), (
		hex!["a0c53fe2dd2bdcb3b3da9612b5a185243057ae81a7e6092a0788672cc99f8b72"].unchecked_into(),
		5000000000000000000000000,
	), (
		hex!["3401ae0dc64bf4bae104413d1e68380cd8f9f75b753b870b39352bc25e05c865"].unchecked_into(),
		5000000000000000000000000,
	), (
		hex!["62f118b23e00a02f0e64b2cc5491226c24b7781fd60ca7ac330011173f6d4b68"].unchecked_into(),
		5000000000000000000000000,
	), (
		hex!["0211cc0925ac6dd2d14fbdb63766d235d2a3c159d4c1dd3a260f3f179b05ce41"].unchecked_into(),
		5000000000000000000000000,
	), (
		hex!["d6a6eb11c5bb7b37de7ebefdcb723016bf525f5af016aff323ed8b46e182e063"].unchecked_into(),
		5000000000000000000000000,
	), (
		hex!["2a36057030d2b7034eb0da5188729859780d151bdd3a64295ddcf5514e21a679"].unchecked_into(),
		4999980000000000000000000,
	), (
		hex!["92c3225d346f99794812069a16cb91d979137d1534fff5795dacbaede7369f1d"].unchecked_into(),
		10000000000000000000,
	), (
		hex!["02456d652d95f78a97ff18e9f1170b3478f3644c4283f58312f2bb98f3ffe74e"].unchecked_into(),
		10000000000000000000,
	), (
		hex!["765169492c492ee29f2af9af46f9e1b117aa0283b73a4361ae12ace1c41a6c72"].unchecked_into(),
		150000000000000000000000000,
	), (
		hex!["ec80b8b78a2b283f0a48712c8446241cf5f36d2f480559cdc73253981963f402"].unchecked_into(),
		25000000000000000002363,
	),

	// moose key
	(
		hex!["063b8cece2f86e568e7e18017988ef4b970dda9275f6f90c2fc6a9388cefb44d"].unchecked_into(),
		31243750000000000000000000,
	),

	// edgeware genesis validators unstaked
	(
		hex!["e049bb3d3369ddf42e07a51e8eeadb660a618fc3cea63dc728a2b79683873855"].unchecked_into(),
		10000000000000000000000000,
	), (
		hex!["b204d6aa8c13f1f274f5f849cb09ef4f3f2641acc3dec99f905e444784fbb247"].unchecked_into(),
		10000000000000000000000000,
	), (
		hex!["84f5ab01391781936ac0447d2e801fa6531cbe478ae96c5583fef9e101001123"].unchecked_into(),
		10000000000000000000000000,
	), ( 
		hex!["6e70fa41746fd3e728e2abe2bed9c957c9ef1a5e67304ff0a0841551bcd4d621"].unchecked_into(),
		10000000000000000000000000,
	), (
		hex!["aea65488616f3fa5dc0bb7b4a3a803185bf05a8e89c076f9abc18021694ebb38"].unchecked_into(),
		10000000000000000000000000,
	), (
		hex!["743ea391454673ddacfd7b344790b7889ab8702cd06208347fb3c24ab9dcbe14"].unchecked_into(),
		10000000000000000000000000,
	), (
		hex!["84798309f2d264c5bb5f3c651ddf5080c3494013b201859e62bd9352b32c1148"].unchecked_into(),
		10000000000000000000000000,
	), (
		hex!["3e0dbf4f6e126100f2cc8c6b4c62fffa263387edd2894e7ab7fa3a7f876a9437"].unchecked_into(),
		10000000000000000000000000,
	), (
		hex!["f2a644bf322abbe9d5fc0861056ad5220cdf64b15a3218a116d45f91c709bd79"].unchecked_into(),
		10000000000000000000000000,
	), (
		hex!["ca5b161dbda0460526d45b8bca2265b3f17d1d26463219f7c05fa4e8f1bf6f4d"].unchecked_into(),
		10000000000000000000000000,
	)]
}

/// The mainnet commonwealth validator pubkeys and staked balances.
/// We give each of these 9999990000000000000000000 balance, which
/// is 1e19 less than 1e25, so that we reserve 1e19 for the controllers
/// of these accounts (enough balance for txs and existential balance)
/// 
/// The total of the Commonwealth Mainnet validators balances is thus 1e26 or 20%
/// i.e 10 * ((1e25 - 1e19) + 1e19) = 1e26 or 20% of 5e26
pub const CONTROLLER_ENDOWMENT: Balance = 10000000000000000000;
pub fn get_genesis_mainnet_validators() -> Vec<(AccountId, AccountId, AuraId, Balance, GrandpaId, ImOnlineId)> {
	return vec![( // sunny
		// 5EHuW1nXmHARKrFTbA7xsk66qNWGB374H34D3TZew8U7SQ1Y
		hex!["6289da9f0a3d164f829d9b30dd43cc87c3513848dd0dec309d88f09a501b492b"].unchecked_into(),
		// 5HR5z8aPqwiXLNzNuUhpt9Uz1ri8jM8qBfBjKz11X2M2kHwC
		hex!["ecb8912c7fcaa6ff472d91d130309a9c25c7a0c975dfad513837dc85dcb34d71"].unchecked_into(),
		// 5CsWm3BeQ76kJ2sJLRkN5zTJsQnxLncQgpgTsQn7tZb3afXn
		hex!["23b32d8c70a57b0a7fe360f88bfd3b3262985d4054caee06ee3d381bc5caed1b"].unchecked_into(),
		// Initial bonded stake
		(15621875000000000000000000 - CONTROLLER_ENDOWMENT),
		// 5DBTeCLgaARpYzPL8jSCPrkuPgnvkYXtkPKfCaUnQWCPB6xY
		hex!["31631964511baf8181c9fcad6e4e3327c8ae499112259554305fbee10d3ba929"].unchecked_into(),
		// 5C6K36YHugikBYbstsyUjnkH7SeuzqxwJ5WBcfQCS9JfBeKb
		hex!["0139abce81b2d960169f38211f54223d4d84a7b14d7d3da618fc80ff4208f643"].unchecked_into(),
	), ( // chris
		// 5EPaGtby97rA3vax5UBjiNeX118gifit8WQtqguihCEQYrjP
		hex!["66dc9ae207a8b0e744f5c8e1f1114a29563584dee24cd8b1c0b3bd209f682e31"].unchecked_into(),
		// 5DHb8bRWtufbcARHdvtAKcaGzf4PnwWQ2j1AHxJN7fC91qpL
		hex!["360fc8af30165160b0439293e1f1ede7dc8dc97ddd5c724dc36f517fbfa0f244"].unchecked_into(),
		// 5EaSKWrphEqBhVaYRcPGUTxRhPkrf18N3QW6bCTtPQsgUvK6
		hex!["6f2586c365d0474ad95fff0f10cf80eb458442cb818e0bdf2b508a713a4fd496"].unchecked_into(),
		// Initial bonded stake
		(15621875000000000000000000 - CONTROLLER_ENDOWMENT),
		// 5ET1puzhKdFsuUu2WjVpXxvrze2k1TryinLLB7YgBMpnzpCg
		hex!["697c5b7dcb4bcfa4b662ed7c16611a08ad23ed79980cef7b03cde055a58f2380"].unchecked_into(),
		// 5Fu2Tqi2PMNuuSFNgGKF27z8oQbMhGXUPBGKGExccqMxZNMc
		hex!["a98f1d51f014243e086e18217acdcca68671dfdf587bc517e2e28f477089b545"].unchecked_into(),
	), ( // maskofice
		// 5Hh5oUsTVy5XLJJk6noKtUnNpzif9GkDu5SUKKjB1hLq1cku
		hex!["f8ebe01bf9f4f91fadfc5a1c038f9fb91c10ff526229f88309a241a82ec1b20a"].unchecked_into(),
		// 5FsXycTP24dEJerKQUmnQRJkcZBjGobgwr1fyUprko4nDAzm
		hex!["a86bf96f34d5e6ba3b001d05d7691ee544a70c3be9b1b5a5b9e3bf404aaa8a56"].unchecked_into(),
		// 5GuxF5zxb3jrEFRAQWtHWMDoPK43skcHRQw71JmY5QEk4xnU
		hex!["d6806352509711f3755a87ab5c53a1078b8ca17c2fe1375d3b781bcd3a9f0926"].unchecked_into(),
		// Initial bonded stake
		(15621875000000000000000000 - CONTROLLER_ENDOWMENT),
		// 5ERhh5ZuMPcmdN61VE9FfDnEGnMqzGms3uvknZEkbXD2e7V5
		hex!["687c10581a35f71df519ba11dcdf19334ce4c409ea020e589fe034e043a0a2f4"].unchecked_into(),
		// 5ENjT8nPAuMz3bHRBcuXZbYsQjYcrdZYwpFqxobFd4iA8Bs4
		hex!["6638403a8fb33408bd06cbd03286c6edae553b048f05881e067567bc3c69077d"].unchecked_into(),
	), ( // briancohen
		// 5D4t6Ue7PqotZL2pKsSi1h7zBv6Tb4o5dXGrvj3zxQyrwvyY
		hex!["2c5eb2a9a2ad85eba59bb0509a601b59cf578fd2edc82766fe24425235ce050d"].unchecked_into(),
		// 5FRdMPPUnnt6fsZLNvFz3YMgrVDWSypCAAFzo2kJTzQpQLmu
		hex!["94a9b06b3d776007b2e920fee64354ff8165524660bf4eb645576bda0fdd7655"].unchecked_into(),
		// 5CCBG5HSmuCzXBsQNZVRWcLwgKLAXPRASkyqZdKQCsHvRFrp
		hex!["05b2f766417d7bd9cd49828cd81e5bb70f3af9a7b0cf036ce735da88585ab6cd"].unchecked_into(),
		// Initial bonded stake
		(15621875000000000000000000 - CONTROLLER_ENDOWMENT),
		// 5D8xu6iEuGUfvkRX3635E9RksqAxNBXcDueqQjtv66RXrxpU
		hex!["2f7bd9c375d4e13216980da3e932e8317205d92dc6a52b4b70572f0dea77df92"].unchecked_into(),
		// 5F6gK4diV35AWJ86GCLboteSGfjKFsgE2spc2gckUu51vsd9
		hex!["8635fd91de3e210baddfa9c8f4fad050dc68852c4fa5a8fa7aea739fadb2f32b"].unchecked_into(),
	), ( // julien
		// 5DcfNHUdoBJmDj6XUDJtqS2A7L9EntKKRANU4yvkPyBryzqw
		hex!["449bb4d6d41bad287732655b3cb9700cece0776cc3c71372c833e6777054a270"].unchecked_into(),
		// 5EjWd1S5PZjdVrqaGJG8D2dkZax8CrGzBnT2Py8M18XxgG2h
		hex!["7611372cafdd5beb97b3816d643f7ff40f8c117166a58c00ade0d0b42b817f25"].unchecked_into(),
		// 5GRXt7pHygChp4rWpm2cbXWvpjYswQdpfrDf92eUHBB6HmWr
		hex!["c0d38228d2a149ea1e4e75912578a68f400c6ac8c55970fe7e058aca61cf1747"].unchecked_into(),
		// Initial bonded stake
		(31243750000000000000000000 - CONTROLLER_ENDOWMENT),
		// 5G7niGTdvB6DNznmnWrDEmqyKkJcpuUYnEGzkMQqGZpoehrJ
		hex!["b34b038fafa91da6332063bc453a21ca14bd054e12a7ed2bd2b819e7a7ac2208"].unchecked_into(),
		// 5CMZdw5MG68Harf6TGvsMaPcLtfXQHYQp1HtWSVoUsDA4KBr
		hex!["0cdb8096f2310217e14d377e87b142d9ca6d5061108e01becea52817d3142a05"].unchecked_into(),
	)];
}


pub fn get_lockdrop_mainnet_validators() -> Vec<(AccountId, AccountId, AuraId, Balance, GrandpaId, ImOnlineId)> {
	return vec![(
		hex!["aeb995615b36c72bd22637ea15d926386b4f51bf74fb79eae26fd24fda410806"].unchecked_into(),
		hex!["947a28f326b8501d02d9af7371ec209f940b9a0225f4eb7eb56a1db6820ba53a"].unchecked_into(),
		hex!["4b9fca841bb618be3680684657cea5a6d833e5509b6d7788154ce20bac9e3d36"].unchecked_into(),
		150000000000000000000000,
		hex!["ae6a216639c5564b83a2aaa409f3cddf2d8e81dc28bcabfb4545718a523be9e9"].unchecked_into(),
		hex!["92ab61796c8751b46d542539066f9b5eb9723e7f99ea016a382259327fe629a1"].unchecked_into(),
	)];
}

pub fn get_mainnet_identity_verifiers() -> Vec<AccountId> {
	return vec![
		// 5DLp1hjsmq8WUfSQjPe5bLHRVNBDfUdARtpbWtQEHVR2Q5RY
		hex!["3884e1d006f07973128059a261782b096b0fed5daa33126a09e4edd6f659fc4f"].unchecked_into(),
	];
}

pub fn get_mainnet_root_key() -> AccountId {
	// 5He8UmeQxPPrx15Di1eBJKPTmwFaQ1bq22a7bQtJ6RExVGA6
	return hex!["f6ab26f31d3e66fb0eb651b385110c4d723039f56ea0cb98fdb11f60fdbfae6a"].unchecked_into();
}

pub fn get_mainnet_election_members() -> Vec<AccountId> {
	return vec![
		// 5DLp1hjsmq8WUfSQjPe5bLHRVNBDfUdARtpbWtQEHVR2Q5RY
		hex!["3884e1d006f07973128059a261782b096b0fed5daa33126a09e4edd6f659fc4f"].unchecked_into(),
	];
}