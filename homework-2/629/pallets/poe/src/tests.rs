use super::*;
use crate::{mock::*, Error, Event as PoeEvent, Proofs};
use frame_support::{assert_noop, assert_ok, BoundedVec, traits::Get, assert_err};

use sp_runtime::DispatchError;
use crate::mock::PoeModule;
use sp_runtime::traits::ConstU32;

// 公共逻辑
// 0.1 失败-> 存证未签名
// 0.2 失败-> 创建的存证超过长度
// 0.3 失败-> 和设置的存证最大长度不相等

// 测一下创建Claim超长的
#[test]
fn claim_fails_with_too_long_claim() {
    new_test_ext().execute_with(|| {
        let alice = 1;
        let max_len = <<Test as Config>::MaxClaimLength as Get<u32>>::get();
        // 放了6位
        let claim: Result<BoundedVec<u8, ConstU32<5>>, _> = BoundedVec::try_from(vec![0; (max_len + 1) as usize]);
        assert!(claim.is_err());
    });
}

#[test]
fn claim_fails_with_unsigned_origin() {
    new_test_ext().execute_with(|| {
        let claim = BoundedVec::try_from(vec![0, 1]).unwrap();
        let bob = 2;


        // 使用未签名的origin
        // ensure_signed pub fn ensure_signed<OuterOrigin, AccountId>(o: OuterOrigin) -> Result<AccountId, BadOrigin>

        assert_noop!(
            PoeModule::create_claim(RuntimeOrigin::none(), claim.clone()),
            DispatchError::BadOrigin
        );

        assert_noop!(
            PoeModule::revoke_claim(RuntimeOrigin::none(), claim.clone()),
            DispatchError::BadOrigin
        );

        assert_noop!(
            PoeModule::transfer_claim(RuntimeOrigin::none(), claim.clone(), bob),
            DispatchError::BadOrigin
        );
    });
}

#[test]
fn claim_fails_with_max_claim_not_equal() {
    new_test_ext().execute_with(|| {
        assert_eq!(<<Test as Config>::MaxClaimLength as Get<u32>>::get(), 5);
    });
}


// 1. 创建存证，验证
// 1.1 成功创建存证，并且事件触发
// 1.1 失败-> 创建已存在的存证
#[test]
fn create_claim_works() {
    new_test_ext().execute_with(|| {
        let alice = 3;
        let claim = BoundedVec::try_from(vec![0, 1]).unwrap();
        assert_ok!(PoeModule::create_claim(RuntimeOrigin::signed(alice),claim.clone()));

        assert_eq!(
            Proofs::<Test>::get(&claim),
            Some((alice, frame_system::Pallet::<Test>::block_number()))
        );

        println!("len {:?}", System::events().len());

        // 验证只有一个事件被触发（第一次成功创建时的事件）
        // System::assert_last_event(PoeEvent::ClaimCreated(alice, claim).into())
        // System::assert_last_event(RuntimeEvent::PoeModule(PoeEvent::ClaimCreated(alice, claim)).into());
    });
}

#[test]
fn create_claim_failed_when_claim_already_exist() {
    new_test_ext().execute_with(|| {
        let claim = BoundedVec::try_from(vec![1, 2, 3]).unwrap();
        let alice = 1;
        let first_create = PoeModule::create_claim(RuntimeOrigin::signed(alice), claim.clone());
        assert_noop!(
            PoeModule::create_claim(RuntimeOrigin::signed(alice),claim.clone()),
            Error::<Test>::ProofAlreadyExist
        );
    })
}


// 2.   撤销测试，验证
// 2.1 成功撤销，并且事件触发
// 2.1 失败->撤销的凭证不存在
// 2.2 失败->撤销的凭证否为本人
#[test]
fn revoke_claim_should_work() {
    new_test_ext().execute_with(|| {
        let claim = BoundedVec::try_from(vec![1, 2, 3]).unwrap();
        let alice = 1;
        // 创建一个claim
        assert_ok!(PoeModule::create_claim(RuntimeOrigin::signed(alice), claim.clone()));

        assert_ok!(PoeModule::revoke_claim(RuntimeOrigin::signed(alice), claim.clone()));


        // 验证事件是否被正确触发
        // System::assert_last_event(<Test as Config>::RuntimeEvent::PoeModule(PoeEvent::ClaimRevoked(alice, claim.clone()).into()));

        // 验证claim已被撤销
        assert_eq!(Proofs::<Test>::get(&claim), None);
    })
}

#[test]
fn revoke_claim_failed_when_claim_is_not_exist() {
    new_test_ext().execute_with(|| {
        let claim = BoundedVec::try_from(vec![1, 2, 3]).unwrap();
        let alice = 1;
        assert_noop!(
            PoeModule::revoke_claim(RuntimeOrigin::signed(alice), claim.clone()),
            Error::<Test>::ClaimNotExist
        );
    })
}

#[test]
fn revoke_claim_failed_with_wrong_owner() {
    new_test_ext().execute_with(|| {
        let claim = BoundedVec::try_from(vec![1, 2, 3]).unwrap();
        let alice = 1;
        let tony = 2;

        // 账户1创建claim
        assert_ok!(PoeModule::create_claim(RuntimeOrigin::signed(alice), claim.clone()));

        // 账户2尝试撤销claim
        assert_noop!(
            PoeModule::revoke_claim(RuntimeOrigin::signed(tony), claim.clone()),
            Error::<Test>::NotClaimOwner
        );
    })
}


// 3. 转移存证的例子
// 3.1 正常情况能否转移
// 3.2 失败->转移不存在的声明
// 3.2 失败->非所有者的转移，不是所有者的，不能转移
// 3.5 失败->边界的问题，转出的和转入的都是自己


#[test]
fn transfer_claim_works() {
    new_test_ext().execute_with(|| {
        let claim = BoundedVec::try_from(vec![0, 1]).unwrap();
        let alice = 1;
        let tony = 2;

        let res = PoeModule::create_claim(RuntimeOrigin::signed(alice), claim.clone());

        // 验证下转移成功
        assert_ok!(PoeModule::transfer_claim(crate::mock::RuntimeOrigin::signed(alice), claim.clone(), tony));

        // 验证是否为被转移人拥有
        assert_ok!(PoeModule::proofs(RuntimeOrigin::signed(tony), claim.clone()));
    });
}

#[test]
fn transfer_claim_fails_when_claim_not_exist() {
    new_test_ext().execute_with(|| {
        let claim = BoundedVec::try_from(vec![0, 1]).unwrap();
        let alice = 1;
        let tony = 2;

        // 验证下转移成功
        assert_noop!(
        PoeModule::transfer_claim(crate::mock::RuntimeOrigin::signed(alice), claim.clone(), tony),
        Error::<Test>::ClaimNotExist
        );
    });
}


#[test]
fn transfer_claim_fails_when_not_owner() {
    new_test_ext().execute_with(|| {
        let claim = BoundedVec::try_from(vec![0, 1]).unwrap();
        let alice = 1;
        let tony = 2;
        let charlie = 3;

        // Alice创建声明
        assert_ok!(PoeModule::create_claim(RuntimeOrigin::signed(alice), claim.clone()));

        // tony尝试转移Alice的声明
        assert_noop!(
            PoeModule::transfer_claim(RuntimeOrigin::signed(tony), claim.clone(), charlie),
            Error::<Test>::NotClaimOwner
        );
    });
}












