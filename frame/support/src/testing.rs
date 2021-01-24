#[macro_export]
macro_rules! impl_test_account_data {
	() => {
		pub type Etp3Instance = betelgeuse_balances::Instance0;
		pub type DnaInstance = betelgeuse_balances::Instance1;

		$crate::impl_account_data! {
			struct AccountData<Balance>
			for
				Etp3Instance,
				DnaInstance
			where
				Balance = Balance
			{}
		}
	};
}
