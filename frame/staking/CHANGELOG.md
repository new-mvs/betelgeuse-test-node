# CHANGELOG(v2.0.0.alpha.3)

## Core

Some concepts should have some explaination for the changing from substrate

### power

power is a mixture of etp3 and dna.

+ *RING*: `power = etp3_ratio * POWER_COUNT / 2`
+ *KTON*: `power = dna_ratio * POWER_COUNT / 2`

We use `currency_to_power` and `power_of` to calculcate `power`.

### rebond

We doesn't support `rebond` currently now.

### withdraw

What should happen after all balances being unbonded?(the locked balance)


## Moudle
### delete `withdraw_unbond`

+ **withdraw_unbond**: Remove all associated data of a stash account from the staking system.

Betelgeuse has `active_balance` and `active_deposit_balance`, we calculate `normal_balance` by `active_balance - active_deposit_balance`, the `normal_balance` is **free to transfer**, so we don't need the `withdraw_unbond` function actually.

### delete `slashable_balance_of`

+ **slashable_balance_of**: The total balance that can be slashed from a stash account as of right now.

We use `power_of` and `stake_of` instead of `slashable_balance_of`:

+ **power_of**: The total power that can be slashed from a stash account as of right now.
+ **stake_of**: The `active_etp3` and `active_dna` from a stash account.

**For if an account is slashale:**

Just use `power_of`, if the return `power` is zero, the target account is not slashable.

**For the amount of slashable balances:**

The slashable balances actually mean `active-etp3` and `active-dna` in betelgeuse's staking
process, we can use `Staking::ledger(controller)` to get a `StakingLedger` which contains
the `active-etp3` and `active-dna` the `controller` have.

## Structs

### Exposure

A snapshot of the stake backing a single validator in the system.

> betelgeuse

```rust
#[derive(PartialEq, Eq, PartialOrd, Ord, Clone, Encode, Decode, Default, RuntimeDebug)]
pub struct Exposure<AccountId, Etp3Balance, DnaBalance>
where
	Etp3Balance: HasCompact,
	DnaBalance: HasCompact,
{
	#[codec(compact)]
	pub own_etp3_balance: Etp3Balance,
	#[codec(compact)]
	pub own_dna_balance: DnaBalance,
	pub own_power: Power,
	pub total_power: Power,
	pub others: Vec<IndividualExposure<AccountId, Etp3Balance, DnaBalance>>,
}
```

> substrate

```rust
#[derive(PartialEq, Eq, PartialOrd, Ord, Clone, Encode, Decode, Default, RuntimeDebug)]
pub struct Exposure<AccountId, Balance: HasCompact> {
	/// The total balance backing this validator.
	#[codec(compact)]
	pub total: Balance,
	/// The validator's own stash that is exposed.
	#[codec(compact)]
	pub own: Balance,
	/// The portions of nominators stashes that are exposed.
	pub others: Vec<IndividualExposure<AccountId, Balance>>,
}
```

### IndividualExposure

The amount of exposure (to slashing) than an individual nominator has.

> betelgeuse

```rust
#[derive(PartialEq, Eq, PartialOrd, Ord, Clone, Encode, Decode, RuntimeDebug)]
pub struct IndividualExposure<AccountId, Etp3Balance, DnaBalance>
where
	Etp3Balance: HasCompact,
	DnaBalance: HasCompact,
{
	who: AccountId,
	#[codec(compact)]
	etp3_balance: Etp3Balance,
	#[codec(compact)]
	dna_balance: DnaBalance,
	power: Power,
}
```

> substrate
```rust
#[derive(PartialEq, Eq, PartialOrd, Ord, Clone, Encode, Decode, RuntimeDebug)]
pub struct IndividualExposure<AccountId, Balance: HasCompact> {
	/// The stash account of the nominator in question.
	who: AccountId,
	/// Amount of funds exposed.
	#[codec(compact)]
	value: Balance,
}
```


### StakingLedger

The ledger of a (bonded) stash.

+ annotated `rebond`

Currently we don't have this requirement.

> betelgeuse
```rust
#[derive(PartialEq, Eq, Clone, Default, Encode, Decode, RuntimeDebug)]
pub struct StakingLedger<AccountId, Etp3Balance, DnaBalance, BlockNumber, Timestamp>
where
	Etp3Balance: HasCompact,
	DnaBalance: HasCompact,
{
	pub stash: AccountId,
  #[codec(compact)]
	pub active_etp3: Etp3Balance,
  #[codec(compact)]
	pub active_deposit_etp3: Etp3Balance,
	#[codec(compact)]
	pub active_dna: DnaBalance,
	pub deposit_items: Vec<TimeDepositItem<Etp3Balance, Timestamp>>,
	pub etp3_staking_lock: StakingLock<Etp3Balance, BlockNumber>,
	pub dna_staking_lock: StakingLock<DnaBalance, BlockNumber>,
}
```

> substrate

```rust
#[derive(PartialEq, Eq, Clone, Encode, Decode, RuntimeDebug)]
pub struct StakingLedger<AccountId, Balance: HasCompact> {
	pub stash: AccountId,
	/// The total amount of the stash's balance that we are currently accounting for.
	/// It's just `active` plus all the `unlocking` balances.
	#[codec(compact)]
	pub total: Balance,
	/// The total amount of the stash's balance that will be at stake in any forthcoming
	/// rounds.
	#[codec(compact)]
	pub active: Balance,
	/// Any balance that is becoming free, which may eventually be transferred out
	/// of the stash (assuming it doesn't get slashed first).
	pub unlocking: Vec<UnlockChunk<Balance>>,
}
```
