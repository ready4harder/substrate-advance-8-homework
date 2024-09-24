use crate as pallet_kitties;
use frame_support::traits::{ Hooks};
use frame_support::{
    derive_impl,
    traits::{ConstU16, ConstU64,ConstU128},
    weights::Weight,
};
use sp_core::{ H256};
use sp_runtime::{
	traits::{BlakeTwo256, IdentityLookup},
	BuildStorage,
};

type Block = frame_system::mocking::MockBlock<Test>;
type Balance = u128;
frame_support::construct_runtime!(
    pub enum Test
    {
        System: frame_system,
        KittiesModule: pallet_kitties,
        Random: pallet_insecure_randomness_collective_flip,
        Balances:pallet_balances,
    }
);



#[derive_impl(frame_system::config_preludes::TestDefaultConfig as frame_system::DefaultConfig)]
impl frame_system::Config for Test {
	type BaseCallFilter = frame_support::traits::Everything;
    type BlockWeights = ();
    type BlockLength = ();
    type DbWeight = ();
    type RuntimeOrigin = RuntimeOrigin;
    type RuntimeCall = RuntimeCall;
    type Nonce = u64;
    type Hash = H256;
    type Hashing = BlakeTwo256;
    type AccountId = u64;
    type Lookup = IdentityLookup<Self::AccountId>;
    type Block = Block;
    type RuntimeEvent = RuntimeEvent;
    type BlockHashCount = ConstU64<250>;
    type Version = ();
    type PalletInfo = PalletInfo;
    type AccountData = pallet_balances::AccountData<Balance>;
    type OnNewAccount = ();
    type OnKilledAccount = ();
    type SystemWeightInfo = ();
    type SS58Prefix = ConstU16<42>;
    type OnSetCode = ();
    type MaxConsumers = frame_support::traits::ConstU32<16>;
}
impl pallet_insecure_randomness_collective_flip::Config for Test {}

#[derive_impl(pallet_balances::config_preludes::TestDefaultConfig)]
impl pallet_balances::Config for Test {
	type Balance = Balance;
	// type DustRemoval = ();
	// type RuntimeEvent = RuntimeEvent;
	type ExistentialDeposit = ConstU128<1>;
	type AccountStore = System;
	// type WeightInfo = ();
	// type MaxLocks = ();
	// type MaxReserves = frame_support::traits::ConstU32<50>;
	// type ReserveIdentifier = [u8; 8];
	// type FreezeIdentifier = ();
	// type MaxFreezes = ();
	// type RuntimeHoldReason = ();
	// type RuntimeFreezeReason = ();
}

impl pallet_kitties::Config for Test {
    type RuntimeEvent = RuntimeEvent;
    type WeightInfo = ();
    type Randomness = Random;

    type Currency = Balances;
    type KittyStake = ConstU128<200>;
}

pub fn new_test_ext() -> sp_io::TestExternalities {
        sp_tracing::try_init_simple();
		let mut t = frame_system::GenesisConfig::<Test>::default().build_storage().unwrap();
		pallet_balances::GenesisConfig::<Test> {
			balances:{
				vec![
					(0, 10_000_000_000),
					(1, 10_000_000_000),
					(2, 10_000_000_000),
					(3 ,10_000_000_000),
				]
			}
		}
		.assimilate_storage(&mut t)
		.unwrap();

		let mut ext = sp_io::TestExternalities::new(t);
		ext.execute_with(|| System::set_block_number(1));
		ext
}

pub fn run_to_block(n: u64) {
    while System::block_number() < n {
        log::info!("current block: {:?}", System::block_number());
        KittiesModule::on_finalize(System::block_number());
        System::on_finalize(System::block_number());
        System::set_block_number(System::block_number() + 1);
        System::on_initialize(System::block_number());
        KittiesModule::on_initialize(System::block_number());
        KittiesModule::on_idle(System::block_number(), Weight::default());
    }
}