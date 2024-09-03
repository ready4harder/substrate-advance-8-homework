use super::*;
use crate::{mock::*, Error};
use frame_support::{assert_noop, assert_ok, BoundedVec};

// 测试创建声明是否正常工作
#[test]
fn create_claim_works() {
	new_test_ext().execute_with(|| {
		// 创建一个包含[0, 1]的有界向量作为声明
		let claim = BoundedVec::try_from(vec![0, 1]).unwrap();
		// 断言创建声明操作成功执行
		assert_ok!(PoeModule::create_claim(RuntimeOrigin::signed(1), claim.clone()));

		// 验证存储中的声明信息是否正确
		assert_eq!(
			Proofs::<Test>::get(&claim),
			Some((1, frame_system::Pallet::<Test>::block_number()))
		);
		// 验证最大声明长度是否为10
		assert_eq!(<<Test as Config>::MaxClaimLength as Get<u32>>::get(), 10);
	})
}

// 测试创建已存在的声明是否会失败
#[test]
fn create_claim_failed_when_claim_already_exist() {
	new_test_ext().execute_with(|| {
		let claim = BoundedVec::try_from(vec![0, 1]).unwrap();
		// 首次创建声明
		let _ = PoeModule::create_claim(RuntimeOrigin::signed(1), claim.clone());

		// 断言再次创建相同声明会失败，并返回ProofAlreadyExist错误
		assert_noop!(
			PoeModule::create_claim(RuntimeOrigin::signed(1), claim.clone()),
			Error::<Test>::ProofAlreadyExist
		);
	})
}

// 测试撤销声明是否正常工作
#[test]
fn revoke_claim_works() {
	new_test_ext().execute_with(|| {
		let claim = BoundedVec::try_from(vec![0, 1]).unwrap();
		// 首先创建一个声明
		let _ = PoeModule::create_claim(RuntimeOrigin::signed(1), claim.clone());

		// 断言撤销声明操作成功执行
		assert_ok!(PoeModule::revoke_claim(RuntimeOrigin::signed(1), claim.clone()));
	})
}

// 测试撤销不存在的声明是否会失败
#[test]
fn revoke_claim_failed_when_claim_is_not_exist() {
	new_test_ext().execute_with(|| {
		let claim = BoundedVec::try_from(vec![0, 1]).unwrap();

		// 断言撤销不存在的声明会失败，并返回ClaimNotExist错误
		assert_noop!(
			PoeModule::revoke_claim(RuntimeOrigin::signed(1), claim.clone()),
			Error::<Test>::ClaimNotExist
		);
	})
}

// 测试非所有者撤销声明是否会失败
#[test]
fn revoke_claim_failed_with_wrong_owner() {
	new_test_ext().execute_with(|| {
		let claim = BoundedVec::try_from(vec![0, 1]).unwrap();
		// 账户1创建声明
		let _ = PoeModule::create_claim(RuntimeOrigin::signed(1), claim.clone());

		// 断言账户2（非所有者）撤销声明会失败，并返回NotClaimOwner错误
		assert_noop!(
			PoeModule::revoke_claim(RuntimeOrigin::signed(2), claim.clone()),
			Error::<Test>::NotClaimOwner
		);
	})
}

// 测试转移声明是否正常工作
#[test]
fn transfer_claim_works() {
	new_test_ext().execute_with(|| {
		let claim = BoundedVec::try_from(vec![0, 1]).unwrap();
		// 账户1创建声明
		let _ = PoeModule::create_claim(RuntimeOrigin::signed(1), claim.clone());

		// 断言账户1将声明转移给账户2成功执行
		assert_ok!(PoeModule::transfer_claim(RuntimeOrigin::signed(1), claim.clone(), 2));

		// 验证存储中的声明所有权是否已更新为账户2
		let bounded_claim =
			BoundedVec::<u8, <Test as Config>::MaxClaimLength>::try_from(claim.clone()).unwrap();
		assert_eq!(
			Proofs::<Test>::get(&bounded_claim),
			Some((2, frame_system::Pallet::<Test>::block_number()))
		);
	})
}

// 测试转移不存在的声明是否会失败
#[test]
fn transfer_claim_failed_when_claim_is_not_exist() {
	new_test_ext().execute_with(|| {
		let claim = BoundedVec::try_from(vec![0, 1]).unwrap();

		// 断言转移不存在的声明会失败，并返回ClaimNotExist错误
		assert_noop!(
			PoeModule::transfer_claim(RuntimeOrigin::signed(1), claim.clone(), 2),
			Error::<Test>::ClaimNotExist
		);
	})
}

// 测试非所有者转移声明是否会失败
#[test]
fn transfer_claim_failed_with_wrong_owner() {
	new_test_ext().execute_with(|| {
		let claim = BoundedVec::try_from(vec![0, 1]).unwrap();
		// 账户1创建声明
		let _ = PoeModule::create_claim(RuntimeOrigin::signed(1), claim.clone());

		// 断言账户2（非所有者）转移声明会失败，并返回NotClaimOwner错误
		assert_noop!(
			PoeModule::transfer_claim(RuntimeOrigin::signed(2), claim.clone(), 3),
			Error::<Test>::NotClaimOwner
		);
	})
}
