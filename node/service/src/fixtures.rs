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

pub fn get_allocation() 
	-> Result<(
		Vec<(AccountId, Balance)>,
		Vec<(AccountId, BlockNumber, BlockNumber, Balance)>,
		Vec<(AccountId, AccountId, AuraId, Balance, GrandpaId, ImOnlineId)>
	)> {
    let mut file = File::open("node/service/src/genesis.json").unwrap();
    let mut data = String::new();
    file.read_to_string(&mut data).unwrap();

	let json: Allocation = serde_json::from_str(&data)?;;
	let balances_json = json.balances;
	let vesting_json = json.vesting;
	let validators_json = json.validators;

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
	let validators: Vec<(AccountId, AccountId, AuraId, Balance, GrandpaId, ImOnlineId)> = validators_json.into_iter().map(|e| {
		return (
			<[u8; 32]>::from_hex(e.0).unwrap().unchecked_into(),
			<[u8; 32]>::from_hex(e.1).unwrap().unchecked_into(),
			<[u8; 32]>::from_hex(e.2.clone()).unwrap().unchecked_into(),
			e.3.to_string().parse::<Balance>().unwrap(),
			<[u8; 32]>::from_hex(e.2.clone()).unwrap().unchecked_into(),
			<[u8; 32]>::from_hex(e.2).unwrap().unchecked_into(),
		);
	}).collect();
	Ok((balances, vesting, validators))
}

pub fn get_genesis_validators() -> Vec<(AccountId, AccountId, AuraId, Balance, GrandpaId, ImOnlineId)> {
	return vec![(
		// 5DvWxEcMP66DgHigGm2eTTg4pPueDMMDS5F67ixK2WpCTKMU
		hex!["5239cc265b2d7ac6dad6b640a28a64ce5e09b7de22fd0549c2d282d461da260e"].unchecked_into(),
		// 5EkGSct2SPojcFF6fX6EvF3xbaW5aq3oEW2ujJLjTKk8pexP
		hex!["76a4bad1d5fe37ba60dcc9160f3b0fb1822c64f0e92f2171c358a0b7e59aef37"].unchecked_into(),
		// 5D8sbkeQpqoAXt7E4WcNTBEK3sn4CGp67HRBJRQsqXFfsXB5
		hex!["2f6a032ba0dbdcac7fa68607533971ba399a9a06002978b4c071f87334d153b0"].unchecked_into(),
		50000000000000000000000000,
		// 5GywTGqF81sdGsynZG7hr8DgibrCDsdvNN9mGCQhf7CNqHpv
		hex!["d98ab5ea66c0ee4d443b3b6f896daf9c7fefb4d1d2eeca7653ffae84557cf5f3"].unchecked_into(),
		// 5D8sbkeQpqoAXt7E4WcNTBEK3sn4CGp67HRBJRQsqXFfsXB5
		hex!["2f6a032ba0dbdcac7fa68607533971ba399a9a06002978b4c071f87334d153b0"].unchecked_into(),
	), (
		// 5G3h5wZrWiKwnYUbSCuM5EDzrSjyYWzjbocJYxH4Q83Q4VfD
		hex!["b02b17ab762a49749169c2bba23ea7e381dfd3daccd0ed4dd557f1de25ee0e71"].unchecked_into(),
		// 5E7hMvzKjcG31jjK7GhD5Xz9ZrKJSRGcct65tNDhccZkehMF
		hex!["5ac08904abd6cb603e582180dc8106898eb1c8a401cf89a06d221f61fee5df46"].unchecked_into(),
		// 5G2yXYTi1JsgbvkRcnzMe6EABrBpi3g1w4QtWe1W4XHmJxpW
		hex!["af9f319aa910050d7bed99f5ee2ba4e25429ac9e7746b94edcdf154b8a901a3c"].unchecked_into(),
		50000000000000000000000000,
		// 5EJbF7phraRrRmSi6a9P9RKVfHZTtwC29qdu7TdaZxE9vJdZ
		hex!["630fa435592579438ffe7a5e6c074617c972a09ff3850bdc25cebaeca40b5c13"].unchecked_into(),
		// 5G2yXYTi1JsgbvkRcnzMe6EABrBpi3g1w4QtWe1W4XHmJxpW
		hex!["af9f319aa910050d7bed99f5ee2ba4e25429ac9e7746b94edcdf154b8a901a3c"].unchecked_into(),
	)];
}

pub fn get_more_endowed() -> Vec<AccountId> {
	return vec![
		// 5GbmRGiV3roZkekSVHigiRonQnauNg5SB3DEnjF1a3VaW2uY
		hex!["c8a18852ebde806e011b4df37861a7b9b3960eea3383d8c5938a2feb6585ca60"].unchecked_into(),
		// 5DaJBK1GzL8ppRvGFGZzCtvXGUzEuqed2dQCp9bQmSWBcovH
		hex!["42cde2ea2ebad078c18be12d567bde59fe019243ddf670c9b46b30cbe0210d44"].unchecked_into(),
		// 5CexFLkSDHFn6jWSaiQs6s3QURig5rPoTCWecdysVxWRD8Jo
		hex!["1a1ec9b100da17f4062f5b23d4c442d6845ac913ff7d6d1ef5357be688b4ef16"].unchecked_into(),
		// 5HNVCZustH928S8mWbQWPcNfEQ5zipccQTiCDbYDrdSDno4f
		hex!["eabcfa4431091ab5742e21164a2e24b0e0d6f3ab96018c002b0188c213272d47"].unchecked_into(),
		// 5CrLFRMME6MbaFXrTUuHZ6VcS3RFHNwtqiP52f8BnEQTARVB
		hex!["22CC8CEE58420B7FF445DC9D6AFAEFC33B658A5F3A26322BA8DAB2D3FB6D2F1F"].unchecked_into(),
	];
}

pub fn get_identity_verifiers() -> Vec<AccountId> {
	return vec![
		// 5DXp7vdd8uS5HnodyNzXhE7oGrJWh7PpVa3DzwoAtRH66SMv
		hex!["40e8f152a7015fb3867e7c108514029942ef9004602d0f3a5f8061a54dfa6f35"].unchecked_into(),
	];
}

pub fn get_root_key() -> AccountId {
	// 5DXp7vdd8uS5HnodyNzXhE7oGrJWh7PpVa3DzwoAtRH66SMv
	return hex!["40e8f152a7015fb3867e7c108514029942ef9004602d0f3a5f8061a54dfa6f35"].unchecked_into();
}

