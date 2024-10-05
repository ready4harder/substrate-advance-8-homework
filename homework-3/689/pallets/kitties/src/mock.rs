use crate as pallet_kitties;
use frame_support::traits::Hooks;
use frame_support::{
    derive_impl,
    traits::{ConstU16, ConstU32, ConstU64, ConstU128},
    weights::Weight,
};
use sp_core::H256;
use sp_runtime::{
    traits::{BlakeTwo256, IdentityLookup},
    BuildStorage,
};
use pallet_balances;

type Block = frame_system::mocking::MockBlock<Test>;
type Balance = u128;

// Configure a mock runtime to test the pallet.
frame_support::construct_runtime!(
    pub enum Test
    {
        System: frame_system,
        Kitties: pallet_kitties,
        Balances: pallet_balances,
        Random: pallet_insecure_randomness_collective_flip,
    }
);

#[derive_impl(frame_system::config_preludes::TestDefaultConfig)]
impl frame_system::Config for Test {
    type AccountData = pallet_balances::AccountData<Balance>;
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
    type OnNewAccount = ();
    type OnKilledAccount = ();
    type SystemWeightInfo = ();
    type SS58Prefix = ConstU16<42>;
    type OnSetCode = ();
    type MaxConsumers = frame_support::traits::ConstU32<16>;
}

impl pallet_balances::Config for Test {
    type Balance = Balance;
    type DustRemoval = ();
    type RuntimeEvent = RuntimeEvent;
    type ExistentialDeposit = ConstU128<1>;
    type AccountStore = System;
    type WeightInfo = pallet_balances::weights::SubstrateWeight<Test>;
    type MaxLocks = ConstU32<50>;
    type MaxReserves = ();
    type ReserveIdentifier = [u8; 8];
    type FreezeIdentifier = ();
    type MaxFreezes = ConstU32<0>;
    type RuntimeHoldReason = ();
    type RuntimeFreezeReason = ();
}

impl pallet_kitties::Config for Test {
    type RuntimeEvent = RuntimeEvent;
    type WeightInfo = ();
    type Randomness = Random;

    type Currency = Balances;
    type BidMargin = ConstU128<100>;
    type KittyCost = ConstU128<500>;    
}

impl pallet_insecure_randomness_collective_flip::Config for Test {}

// Build genesis storage according to the mock runtime.
pub fn new_test_ext() -> sp_io::TestExternalities {
    sp_tracing::try_init_simple();
    let mut storage = frame_system::GenesisConfig::<Test>::default()
        .build_storage()
        .unwrap();

    // This will cause some initial issuance
    pallet_balances::GenesisConfig::<Test> {
        balances: vec![(1, 5_000), (2, 5_000), (3, 5_000)],
    }
    .assimilate_storage(&mut storage)
    .ok();

    let mut ext = sp_io::TestExternalities::new(storage);
    ext.execute_with(|| System::set_block_number(1));
    ext        
}

pub fn run_to_block(n: u64) {
    while System::block_number() < n {
        log::info!("current block: {:?}", System::block_number());
        Kitties::on_finalize(System::block_number());
        System::on_finalize(System::block_number());
        System::set_block_number(System::block_number() + 1);
        System::on_initialize(System::block_number());
        Kitties::on_initialize(System::block_number());
        Kitties::on_idle(System::block_number(), Weight::default());
    }
}
