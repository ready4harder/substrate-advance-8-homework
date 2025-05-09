use crate as pallet_kitties;
use frame_support::traits::Hooks;
use frame_support::{
    derive_impl,parameter_types,
    traits::{ConstU16,ConstU32, ConstU64,ConstU128},
    weights::Weight,
};
#[warn(unused_imports)]
use sp_core::{sr25519::Signature, H256};
use sp_runtime::{
    traits::{BlakeTwo256, Extrinsic as ExtrinsicT, IdentifyAccount, IdentityLookup, Verify},
    testing::{Header, TestXt},
    BuildStorage,
};
type Block = frame_system::mocking::MockBlock<Test>;
type Balance = u128;
// Configure a mock runtime to test the pallet.
frame_support::construct_runtime!(
    pub enum Test
    {
        System: frame_system,
        Balances: pallet_balances,
        PalletKitties: pallet_kitties,
        Random: pallet_insecure_randomness_collective_flip,
    }
);

#[derive_impl(frame_system::config_preludes::TestDefaultConfig)]
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
    type AccountId = sp_core::sr25519::Public;
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

#[derive_impl(pallet_balances::config_preludes::TestDefaultConfig)]
impl pallet_balances::Config for Test {
    type Balance = Balance;
    type ExistentialDeposit = ConstU128<500>;
    type AccountStore = System;
}

pub type Extrinsic = TestXt<RuntimeCall, ()>;
type AccountId = <<Signature as Verify>::Signer as IdentifyAccount>::AccountId;


impl<LocalCall> frame_system::offchain::SendTransactionTypes<LocalCall> for Test
where
    RuntimeCall: From<LocalCall>,
{
    type OverarchingCall = RuntimeCall;
    type Extrinsic = Extrinsic;
}

impl frame_system::offchain::SigningTypes for Test {
    type Public = <Signature as Verify>::Signer;
    type Signature = Signature;
}
impl<LocalCall> frame_system::offchain::CreateSignedTransaction<LocalCall> for Test
where
    RuntimeCall: From<LocalCall>,
{
    fn create_transaction<C: frame_system::offchain::AppCrypto<Self::Public, Self::Signature>>(
        call: RuntimeCall,
        _public: <Signature as Verify>::Signer,
        _account: AccountId,
        nonce: u64,
    ) -> Option<(RuntimeCall, <Extrinsic as ExtrinsicT>::SignaturePayload)> {
        Some((call, (nonce, ())))
    }
}

parameter_types! {
    pub const UnsignedPriority: u64 = 1 << 20;
}
impl pallet_kitties::Config for Test {
    type RuntimeEvent = RuntimeEvent;
    type WeightInfo = ();
    type Randomness = Random;
    type Currency = Balances;
    type MaxKittiesBidPerBlock = ConstU32<10>;
    type MaxKittiesOwned = ConstU32<10>;
    type MinBidBlockSpan = ConstU64<20>;
    type StakeAmount = ConstU128<200>;
    type MinBidIncrement = ConstU128<500>;
    type MaxPrices = ConstU32<10>;
}

impl pallet_insecure_randomness_collective_flip::Config for Test {}

// Build genesis storage according to the mock runtime.
pub fn new_test_ext() -> sp_io::TestExternalities {
    sp_tracing::try_init_simple();
    let mut storage = frame_system::GenesisConfig::<Test>::default()
        .build_storage()
        .unwrap();

    pallet_balances::GenesisConfig::<Test> {
        balances: vec![
            (
                sp_core::sr25519::Public::from_raw([1u8; 32]),
                10_000_000_000,
            ),
            (
                sp_core::sr25519::Public::from_raw([2u8; 32]),
                10_000_000_000,
            ),
            (
                sp_core::sr25519::Public::from_raw([3u8; 32]),
                10_000_000_000,
            ),
        ],
    }
        .assimilate_storage(&mut storage)
        .unwrap();

    let mut ext = sp_io::TestExternalities::new(storage);
    ext.execute_with(|| System::set_block_number(1));
    ext
}

pub fn run_to_block(n: u64) {
    while System::block_number() < n {
        log::info!("current block: {:?}", System::block_number());
        PalletKitties::on_finalize(System::block_number());
        System::on_finalize(System::block_number());
        System::set_block_number(System::block_number() + 1);
        System::on_initialize(System::block_number());
        PalletKitties::on_initialize(System::block_number());
        PalletKitties::on_idle(System::block_number(), Weight::default());
    }
}
