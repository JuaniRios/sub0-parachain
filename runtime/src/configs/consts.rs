extern crate alloc;

use frame_support::weights::{constants::WEIGHT_REF_TIME_PER_SECOND, Weight};
use sp_core::parameter_types;
pub use sp_runtime::Perbill;

use crate::configs::types::{AccountId, Balance, BlockNumber};

#[docify::export]
mod block_times {
	/// This determines the average expected block time that we are targeting. Blocks will be
	/// produced at a minimum duration defined by `SLOT_DURATION`. `SLOT_DURATION` is picked up by
	/// `pallet_timestamp` which is in turn picked up by `pallet_aura` to implement `fn
	/// slot_duration()`.
	///
	/// Change this to adjust the block time.
	pub const MILLI_SECS_PER_BLOCK: u64 = 6000;

	// NOTE: Currently it is not possible to change the slot duration after the chain has started.
	// Attempting to do so will brick block production.
	pub const SLOT_DURATION: u64 = MILLI_SECS_PER_BLOCK;
}
pub use block_times::*;

// Time is measured by number of blocks.
pub const MINUTES: BlockNumber = 60_000 / (MILLI_SECS_PER_BLOCK as BlockNumber);
pub const HOURS: BlockNumber = MINUTES * 60;
#[allow(unused)]
pub const DAYS: BlockNumber = HOURS * 24;

// Unit = the base number of indivisible units for balances
pub const UNIT: Balance = 1_000_000_000_000;
pub const MILLI_UNIT: Balance = 1_000_000_000;
pub const MICRO_UNIT: Balance = 1_000_000;

/// The existential deposit. Set to 1/10 of the Connected Relay Chain.
pub const EXISTENTIAL_DEPOSIT: Balance = MILLI_UNIT;

/// We assume that ~5% of the block weight is consumed by `on_initialize` handlers. This is
/// used to limit the maximal weight of a single extrinsic.
pub const AVERAGE_ON_INITIALIZE_RATIO: Perbill = Perbill::from_percent(5);

/// We allow `Normal` extrinsics to fill up the block up to 75%, the rest can be used by
/// `Operational` extrinsics.
pub const NORMAL_DISPATCH_RATIO: Perbill = Perbill::from_percent(75);

#[docify::export(max_block_weight)]
/// We allow for 2 seconds of compute with a 6 second average block time.
pub const MAXIMUM_BLOCK_WEIGHT: Weight = Weight::from_parts(
	WEIGHT_REF_TIME_PER_SECOND.saturating_mul(2),
	cumulus_primitives_core::relay_chain::MAX_POV_SIZE as u64,
);

#[docify::export]
mod async_backing_params {
	/// Maximum number of blocks simultaneously accepted by the Runtime, not yet included
	/// into the relay chain.
	pub(crate) const UNINCLUDED_SEGMENT_CAPACITY: u32 = 3;
	/// How many parachain blocks are processed by the relay chain per parent. Limits the
	/// number of blocks authored per slot.
	pub(crate) const BLOCK_PROCESSING_VELOCITY: u32 = 1;
	/// Relay chain slot duration, in milliseconds.
	pub(crate) const RELAY_CHAIN_SLOT_DURATION_MILLIS: u32 = 6000;
}
pub(crate) use async_backing_params::*;

/// Deposit that must be provided for each occupied storage item.
pub const DEPOSIT_STORAGE_ITEM: Balance = 56 * MILLI_UNIT;
/// Deposit that must be provided for each occupied storage byte.
pub const DEPOSIT_STORAGE_BYTE: Balance = 100 * MICRO_UNIT;
pub const fn deposit(items: u32, bytes: u32) -> Balance {
	(items as Balance * DEPOSIT_STORAGE_ITEM + (bytes as Balance) * DEPOSIT_STORAGE_BYTE) / 100
}
parameter_types! {
	pub const AssetAccountDeposit: Balance = deposit(1, 18);
	pub const AssetDeposit: Balance = 10 * UNIT;
	pub RootOperatorAccountId: AccountId = AccountId::from([0xffu8; 32]);
	pub const MetadataDepositBase: Balance = deposit(1, 68);
	pub const MetadataDepositPerByte: Balance = deposit(0, 1);
	pub const AssetsStringLimit: u32 = 50;

}
