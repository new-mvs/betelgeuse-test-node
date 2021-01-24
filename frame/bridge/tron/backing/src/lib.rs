//! # Tron Backing Module

#![cfg_attr(not(feature = "std"), no_std)]

mod types {
	// --- betelgeuse ---
	#[cfg(feature = "std")]
	use crate::*;

	pub type AccountId<T> = <T as frame_system::Trait>::AccountId;

	#[cfg(feature = "std")]
	pub type Etp3Balance<T> = <Etp3Currency<T> as Currency<AccountId<T>>>::Balance;
	#[cfg(feature = "std")]
	pub type DnaBalance<T> = <DnaCurrency<T> as Currency<AccountId<T>>>::Balance;

	#[cfg(feature = "std")]
	type Etp3Currency<T> = <T as Trait>::Etp3Currency;
	#[cfg(feature = "std")]
	type DnaCurrency<T> = <T as Trait>::DnaCurrency;
}

// --- substrate ---
use frame_support::{
	decl_module, decl_storage,
	traits::{Currency, Get},
};
use sp_runtime::{traits::AccountIdConversion, ModuleId};
// --- betelgeuse ---
use types::*;

pub trait Trait: frame_system::Trait {
	type ModuleId: Get<ModuleId>;

	type Etp3Currency: Currency<AccountId<Self>>;
	type DnaCurrency: Currency<AccountId<Self>>;

	type WeightInfo: WeightInfo;
}

pub trait WeightInfo {}
impl WeightInfo for () {}

decl_storage! {
	trait Store for Module<T: Trait> as BetelgeuseTronBacking {}

	add_extra_genesis {
		config(backed_etp3): Etp3Balance<T>;
		config(backed_dna): DnaBalance<T>;
		build(|config| {
			let module_account = <Module<T>>::account_id();
			let _ = T::Etp3Currency::make_free_balance_be(
				&module_account,
				T::Etp3Currency::minimum_balance() + config.backed_etp3
			);
			let _ = T::DnaCurrency::make_free_balance_be(
				&module_account,
				config.backed_dna
			);
		});
	}
}

decl_module! {
	pub struct Module<T: Trait> for enum Call
	where
		origin: T::Origin
	{
		const ModuleId: ModuleId = T::ModuleId::get();
	}
}

impl<T: Trait> Module<T> {
	pub fn account_id() -> T::AccountId {
		T::ModuleId::get().into_account()
	}
}
