//! # Crab Issuing Module

#![cfg_attr(not(feature = "std"), no_std)]

#[cfg(test)]
mod mock;
#[cfg(test)]
mod tests;

mod types {
	// --- betelgeuse ---
	use crate::*;

	pub type MappedEtp3 = u128;

	pub type AccountId<T> = <T as frame_system::Trait>::AccountId;

	pub type Etp3Balance<T> = <Etp3Currency<T> as Currency<AccountId<T>>>::Balance;

	type Etp3Currency<T> = <T as Trait>::Etp3Currency;
}

// --- substrate ---
use frame_support::{
	decl_error, decl_event, decl_module, decl_storage,
	traits::{Currency, Get},
};
use sp_runtime::{traits::AccountIdConversion, ModuleId};
// --- betelgeuse ---
use types::*;

pub trait Trait: frame_system::Trait {
	type Event: From<Event<Self>> + Into<<Self as frame_system::Trait>::Event>;

	type ModuleId: Get<ModuleId>;

	type Etp3Currency: Currency<AccountId<Self>>;

	type WeightInfo: WeightInfo;
}

pub trait WeightInfo {}
impl WeightInfo for () {}

decl_event! {
	pub enum Event<T>
	where
		AccountId = AccountId<T>,
		Etp3Balance = Etp3Balance<T>,
	{
		/// Dummy Event. [who, swapped *CRING*, burned Mapped *RING*]
		DummyEvent(AccountId, Etp3Balance, MappedEtp3),
	}
}

decl_error! {
	pub enum Error for Module<T: Trait> {
	}
}

decl_storage! {
	trait Store for Module<T: Trait> as BetelgeuseCrabIssuing {
		pub TotalMappedEtp3 get(fn total_mapped_etp3) config(): MappedEtp3;
	}

	add_extra_genesis {
		build(|config| {
			let _ = T::Etp3Currency::make_free_balance_be(
				&<Module<T>>::account_id(),
				T::Etp3Currency::minimum_balance(),
			);

			TotalMappedEtp3::put(config.total_mapped_etp3);
		});
	}
}

decl_module! {
	pub struct Module<T: Trait> for enum Call
	where
		origin: T::Origin
	{
		type Error = Error<T>;

		const ModuleId: ModuleId = T::ModuleId::get();

		fn deposit_event() = default;
	}
}

impl<T: Trait> Module<T> {
	pub fn account_id() -> T::AccountId {
		T::ModuleId::get().into_account()
	}
}
