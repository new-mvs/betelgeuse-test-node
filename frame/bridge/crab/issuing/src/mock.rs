#![allow(dead_code)]

pub mod crab_issuing {
	// --- betelgeuse ---
	pub use crate::Event;
}

// --- crates ---
use codec::{Decode, Encode};
// --- substrate ---
use frame_support::{impl_outer_event, impl_outer_origin, parameter_types, weights::Weight};
use sp_io::TestExternalities;
use sp_runtime::{
	testing::{Header, H256},
	traits::{BlakeTwo256, IdentityLookup},
	Perbill, RuntimeDebug,
};
// --- betelgeuse ---
use crate::*;

pub type AccountId = u64;
pub type Balance = u128;

pub type System = frame_system::Module<Test>;
pub type Etp3 = betelgeuse_balances::Module<Test, Etp3Instance>;
pub type CrabIssuing = Module<Test>;

pub type CrabIssuingError = Error<Test>;
pub type Etp3Error = betelgeuse_balances::Error<Test, Etp3Instance>;

impl_outer_origin! {
	pub enum Origin for Test where system = frame_system {}
}

impl_outer_event! {
	pub enum Event for Test {
		frame_system <T>,
		betelgeuse_balances Instance0<T>,
		crab_issuing <T>,
	}
}

betelgeuse_support::impl_test_account_data! {}

#[derive(Clone, Eq, PartialEq)]
pub struct Test;
parameter_types! {
	pub const CrabIssuingModuleId: ModuleId = ModuleId(*b"da/crabi");
}
impl Trait for Test {
	type Event = Event;
	type ModuleId = CrabIssuingModuleId;
	type Etp3Currency = Etp3;
	type WeightInfo = ();
}

parameter_types! {
	pub const BlockHashCount: u64 = 250;
	pub const MaximumBlockWeight: Weight = 1024;
	pub const MaximumBlockLength: u32 = 2 * 1024;
	pub const MinimumPeriod: u64 = 5;
	pub const AvailableBlockRatio: Perbill = Perbill::one();
}
impl frame_system::Trait for Test {
	type BaseCallFilter = ();
	type Origin = Origin;
	type Call = ();
	type Index = u64;
	type BlockNumber = u64;
	type Hash = H256;
	type Hashing = BlakeTwo256;
	type AccountId = AccountId;
	type Lookup = IdentityLookup<Self::AccountId>;
	type Header = Header;
	type Event = Event;
	type BlockHashCount = ();
	type MaximumBlockWeight = ();
	type DbWeight = ();
	type BlockExecutionWeight = ();
	type ExtrinsicBaseWeight = ();
	type MaximumExtrinsicWeight = ();
	type MaximumBlockLength = ();
	type AvailableBlockRatio = ();
	type Version = ();
	type PalletInfo = ();
	type AccountData = AccountData<Balance>;
	type OnNewAccount = ();
	type OnKilledAccount = ();
	type SystemWeightInfo = ();
}

parameter_types! {
	pub const ExistentialDeposit: Balance = 0;
}
impl betelgeuse_balances::Trait<Etp3Instance> for Test {
	type Balance = Balance;
	type DustRemoval = ();
	type Event = Event;
	type ExistentialDeposit = ExistentialDeposit;
	type BalanceInfo = AccountData<Balance>;
	type AccountStore = System;
	type MaxLocks = ();
	type OtherCurrencies = ();
	type WeightInfo = ();
}

pub fn new_test_ext() -> TestExternalities {
	let mut t = frame_system::GenesisConfig::default()
		.build_storage::<Test>()
		.unwrap();

	betelgeuse_balances::GenesisConfig::<Test, Etp3Instance> {
		balances: (1..10)
			.map(|i: AccountId| vec![(i, 100 * i as Balance), (10 * i, 1000 * i as Balance)])
			.flatten()
			.collect(),
	}
	.assimilate_storage(&mut t)
	.unwrap();
	GenesisConfig {
		total_mapped_etp3: 4_000,
	}
	.assimilate_storage::<Test>(&mut t)
	.unwrap();

	t.into()
}

pub fn events() -> Vec<Event> {
	let events = System::events()
		.into_iter()
		.map(|evt| evt.event)
		.collect::<Vec<_>>();

	System::reset_events();

	events
}

pub fn crab_issuing_events() -> Vec<Event> {
	events()
		.into_iter()
		.filter(|e| matches!(e, Event::crab_issuing(_)))
		.collect()
}
