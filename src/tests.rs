// This file is part of Substrate.

// Copyright (C) 2020-2021 Parity Technologies (UK) Ltd.
// SPDX-License-Identifier: Apache-2.0

// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
// 	http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

use crate::*;
use crate as example_offchain_worker;
use std::sync::Arc;
use codec::Decode;
use frame_support::{assert_ok, parameter_types};
use sp_core::{
	H256,
	offchain::{OffchainWorkerExt, TransactionPoolExt, testing},
	sr25519::Signature,
};

use sp_keystore::{
	{KeystoreExt, SyncCryptoStore},
	testing::KeyStore,
};
use sp_runtime::{
	RuntimeAppPublic,
	testing::{Header, TestXt},
	traits::{
		BlakeTwo256, IdentityLookup, Extrinsic as ExtrinsicT,
		IdentifyAccount, Verify,
	},
};

type UncheckedExtrinsic = frame_system::mocking::MockUncheckedExtrinsic<Test>;
type Block = frame_system::mocking::MockBlock<Test>;

// For testing the module, we construct a mock runtime.
frame_support::construct_runtime!(
	pub enum Test where
		Block = Block,
		NodeBlock = Block,
		UncheckedExtrinsic = UncheckedExtrinsic,
	{
		System: frame_system::{Pallet, Call, Config, Storage, Event<T>},
		Example: example_offchain_worker::{Pallet, Call, Storage, Event<T>, ValidateUnsigned},
	}
);

parameter_types! {
	pub const BlockHashCount: u64 = 250;
	pub BlockWeights: frame_system::limits::BlockWeights =
		frame_system::limits::BlockWeights::simple_max(1024);
}
impl frame_system::Config for Test {
	type BaseCallFilter = ();
	type BlockWeights = ();
	type BlockLength = ();
	type DbWeight = ();
	type Origin = Origin;
	type Call = Call;
	type Index = u64;
	type BlockNumber = u64;
	type Hash = H256;
	type Hashing = BlakeTwo256;
	type AccountId = sp_core::sr25519::Public;
	type Lookup = IdentityLookup<Self::AccountId>;
	type Header = Header;
	type Event = Event;
	type BlockHashCount = BlockHashCount;
	type Version = ();
	type PalletInfo = PalletInfo;
	type AccountData = ();
	type OnNewAccount = ();
	type OnKilledAccount = ();
	type SystemWeightInfo = ();
	type SS58Prefix = ();
	type OnSetCode = ();
}

type Extrinsic = TestXt<Call, ()>;
type AccountId = <<Signature as Verify>::Signer as IdentifyAccount>::AccountId;

impl frame_system::offchain::SigningTypes for Test {
	type Public = <Signature as Verify>::Signer;
	type Signature = Signature;
}

impl<LocalCall> frame_system::offchain::SendTransactionTypes<LocalCall> for Test where
	Call: From<LocalCall>,
{
	type OverarchingCall = Call;
	type Extrinsic = Extrinsic;
}

impl<LocalCall> frame_system::offchain::CreateSignedTransaction<LocalCall> for Test where
	Call: From<LocalCall>,
{
	fn create_transaction<C: frame_system::offchain::AppCrypto<Self::Public, Self::Signature>>(
		call: Call,
		_public: <Signature as Verify>::Signer,
		_account: AccountId,
		nonce: u64,
	) -> Option<(Call, <Extrinsic as ExtrinsicT>::SignaturePayload)> {
		Some((call, (nonce, ())))
	}
}

parameter_types! {
	pub const GracePeriod: u64 = 5;
	pub const UnsignedInterval: u64 = 128;
	pub const UnsignedPriority: u64 = 1 << 20;
}

impl Config for Test {
	type Event = Event;
	type AuthorityId = crypto::TestAuthId;
	type Call = Call;
	type GracePeriod = GracePeriod;
	type UnsignedInterval = UnsignedInterval;
	type UnsignedPriority = UnsignedPriority;
}

#[test]
fn should_submit_signed_transaction_on_chain2() {
	const PHRASE: &str = "news slush supreme milk chapter athlete soap sausage put clutch what kitten";

	let (offchain, offchain_state) = testing::TestOffchainExt::new();
	let (pool, pool_state) = testing::TestTransactionPoolExt::new();
	let keystore = KeyStore::new();

	SyncCryptoStore::sr25519_generate_new(
		&keystore,
		crate::crypto::Public::ID,
		Some(&format!("{}/hunter1", PHRASE))
	).unwrap();

	let mut t = sp_io::TestExternalities::default();
	t.register_extension(OffchainWorkerExt::new(offchain));
	t.register_extension(TransactionPoolExt::new(pool));
	t.register_extension(KeystoreExt(Arc::new(keystore)));

	// David: If without this line, it panicked 
	// at 'No `response` provided for request with id: HttpRequestId(0)', 
	// primitives\core\src\offchain\testing.rs:307:17
	//
	// The JSON which should have been fetched from the web are ugly hardcoded in this fuction,
	// I do use built-in http module to fetch, but that seems to require this function
	// to complement, to work tegether, seemingly playing a role of an expected mock.
	//
	// I don't know how to avoid this, I don't believe I should touch anything 
	// outside example_offchain_worker, since the stacktrace shows it's related to testing.rs. 
	address_oracle_response2(&mut offchain_state.write());

	// price_oracle_response(&mut offchain_state.write());// Original
	t.execute_with(|| {
		// when
		// Example::fetch_price_and_send_signed().unwrap(); // Original
		Example::fetch_addresses_and_send_signed();// David:
		// then
		let tx = pool_state.write().transactions.pop().unwrap();
		// assert!(pool_state.read().transactions.is_empty());// Original
		let tx = Extrinsic::decode(&mut &*tx).unwrap();
		// assert_eq!(tx.signature.unwrap().0, 0);// Original
		// assert_eq!(tx.call, Call::Example(crate::Call::submit_price(15523)));// Original
	});
}

fn address_oracle_response2(state: &mut testing::OffchainState) {
	state.expect_request(testing::PendingRequest {
		method: "GET".into(),
		// uri: "https://min-api.cryptocompare.com/data/price?fsym=BTC&tsyms=USD".into(),
		uri: "https://bitkeys.work/file/subocw/datasource.txt".into(),
		response: Some(br#"{"sc-address": "5DeeNqcAcaHDSed2HYnqMDK7JHcvxZ5QUE9EKmjc5snvU6wF", "pair-url": "[\"https://bitkeys.work/file/subocw/datasource1.txt\",\"https://bitkeys.work/file/subocw/datasource2.txt\",\"https://bitkeys.work/file/subocw/datasource3.txt\",\"https://bitkeys.work/file/subocw/datasource4.txt\"]"}"#.to_vec()),
		sent: true,
		..Default::default()
	});
	state.expect_request(testing::PendingRequest {
		method: "GET".into(),
		uri: "https://bitkeys.work/file/subocw/datasource1.txt".into(),
		response: Some(br#"{"address":"address1","drec":4321,"drep":1234,"rrec":5678,"rrep":8765}"#.to_vec()),
		sent: true,
		..Default::default()
	});
	state.expect_request(testing::PendingRequest {
		method: "GET".into(),
		uri: "https://bitkeys.work/file/subocw/datasource2.txt".into(),
		response: Some(br#"{"address":"address2","drec":1234,"drep":4321,"rrec":8765,"rrep":5678}"#.to_vec()),
		sent: true,
		..Default::default()
	});
	state.expect_request(testing::PendingRequest {
		method: "GET".into(),
		uri: "https://bitkeys.work/file/subocw/datasource3.txt".into(),
		response: Some(br#"{"address":"address3","drec":1111,"drep":2222,"rrec":3333,"rrep":4444}"#.to_vec()),
		sent: true,
		..Default::default()
	});
	state.expect_request(testing::PendingRequest {
		method: "GET".into(),
		uri: "https://bitkeys.work/file/subocw/datasource4.txt".into(),
		response: Some(br#"{"address":"address4","drec":2222,"drep":1111,"rrec":4444,"rrep":3333}"#.to_vec()),
		sent: true,
		..Default::default()
	});
}