use super::*;
use crate::{mock::*, Error, Event};
use frame_support::{assert_noop, assert_ok, debug};
use frame_system::Config;
use frame_system::EventRecord;

//use super::pallet_kitties;
//use Kitties;
//use Kitties::{Event, Config};
//use sp_runtime::traits;
//use crate::tests::RuntimeCall::Kitties;

#[test]
fn it_works_for__kitties_default_value() {
    new_test_ext().execute_with(|| {
        run_to_block(1);
        run_to_block(2);
    });
}

#[test]
fn it_works_for_kitties_create() {
    new_test_ext().execute_with(|| {
        run_to_block(1);
        let alice: u64 = 0;
        let caller = <<Test as Config>::RuntimeOrigin>::signed(alice);
        // let alice_account_id = system::RawOrigin::<u64>::from(alice).into();

        assert_ok!(Kitties::create(caller));
        //assert_ok!(pallet_kitties::call::create(caller));

        // 非空
        let events = System::events();
        assert!(!events.is_empty());
        //获得最新的消息；
        // let new_events =events.into_iter().next();
        let new_events = events.into_iter().next().unwrap();
        // 判断消息的发出者是alice

        assert!(matches!(
            new_events.event,
            mock::RuntimeEvent::Kitties(Event::KittyCreated { creator, index, data })
                if creator == alice
        ));
        // 其他判断处理
    });
}

//#[should_panic(expected = "caller should signed!")]
//测试溢出，测试panic……
#[test]
fn it_works_for_kitties_create_failed() {
    new_test_ext().execute_with(|| {
        run_to_block(1);
        let alice: u64 = 22;
        let caller = <<Test as Config>::RuntimeOrigin>::none();
        // let alice_account_id = system::RawOrigin::<u64>::from(alice).into();
        let res = Kitties::create(caller);
        assert!(res.is_err());
        //    assert_ok!(pallet_kitties::call::create
        //    我们预期这个调用会返回一个错误
        //    Kitties::create(caller);
    });
}

#[test]
fn it_works_for_kitties_breed_ok() {
    new_test_ext().execute_with(|| {
        run_to_block(1);
        let alice: u64 = 0;
        // let alice_account_id = system::RawOrigin::<u64>::from(alice).into();
        let mut new_events;
        // 非空
        let mut events;
        // 连续调用两次：
        let caller = <<Test as Config>::RuntimeOrigin>::signed(alice);
        assert_ok!(Kitties::create(caller));
        events = System::events();
        assert!(!events.is_empty());
        new_events = events.into_iter().next().unwrap(); //第一个事件

        assert!(matches!(
            new_events.event,
            mock::RuntimeEvent::Kitties(Event::KittyCreated { creator, index, data })
                if creator == alice
        ));

        let caller2 = <<Test as Config>::RuntimeOrigin>::signed(alice);
        assert_ok!(Kitties::create(caller2));

        events = System::events();
        assert!(!events.is_empty());
        new_events = events.into_iter().next().unwrap(); //第二个事件
        assert!(matches!(
            new_events.event,
            mock::RuntimeEvent::Kitties(Event::KittyCreated { creator, index, data })
                if creator == alice
        ));

        let id_new = Kitties::get_next_new_id();
        //assert_ok!(pallet_kitties::call::create(caller));    ""

        // debug!(" id new is :  {id_new} ");
        let caller3 = <<Test as Config>::RuntimeOrigin>::signed(alice);
        //生产新的Kitty
        assert_ok!(Kitties::breed(caller3, id_new - 2, id_new - 1));

        //获得最新的消息；
        // let new_events =events.into_iter().next();

        events = System::events();
        assert!(!events.is_empty());
        new_events = events.into_iter().next().unwrap(); //第三个事件

        // 判断消息的发出者是alice
        // assert!(matches!(
        //   new_events.event,
        //   mock::RuntimeEvent::Kitties(Event::KittyBreeded { creator, kitty_1,kitty_2,index,data })
        // ));
        //   if creator == alice
        // 其他判断处理
        //   if (creator == alice)&&(kitty_1==id_new -1 )&&(kitty_2==id_new -2)
    });
}

#[test]
fn it_works_for_kitties_breed_failed() {
    new_test_ext().execute_with(|| {
        run_to_block(1);
        let alice: u64 = 0;
        let mut caller = <<Test as Config>::RuntimeOrigin>::signed(alice);
        // let alice_account_id = system::RawOrigin::<u64>::from(alice).into();

        // 连续调用两次：
        assert_ok!(Kitties::create(caller));

        caller = <<Test as Config>::RuntimeOrigin>::signed(alice);
        assert_ok!(Kitties::create(caller));

        let id_new = Kitties::get_next_new_id();
        //assert_ok!(pallet_kitties::call::create(caller));

        caller = <<Test as Config>::RuntimeOrigin>::none();

        let res = Kitties::breed(caller, id_new - 1, id_new - 2);
        assert!(res.is_err());

        // 判断其它的错误信息；ID, DATA, 等等……
    });
}

#[test]
fn it_works_for_kitties_transfer_ok() {
    new_test_ext().execute_with(|| {
        run_to_block(1);
        let alice: u64 = 0;
        let caller = <<Test as Config>::RuntimeOrigin>::signed(alice);

        //先创建一个；
        assert_ok!(Kitties::create(caller.clone()));
        // 非空
        let mut events = System::events();
        assert!(!events.is_empty());
        //获得最新的消息；
        let mut new_events = events.into_iter().next().unwrap();

        // 判断消息的发出者是alice
        assert!(matches!(
            new_events.event,
            mock::RuntimeEvent::Kitties(Event::KittyCreated { creator, index, data })
                if creator == alice
        ));

        //转移Kitty,
        let bob: u64 = 3;
        let to = <<Test as Config>::RuntimeOrigin>::signed(bob);

        let id_new = Kitties::get_next_new_id(); //最新的ID
        assert_ok!(Kitties::transfer(caller.clone(), bob, id_new - 1));

        events = System::events();
        assert!(!events.is_empty());
        //获得最新的消息；
        new_events = events.into_iter().next().unwrap();

        //判断消息的发出者是alice
        assert!(matches!(
            new_events.event,
            mock::RuntimeEvent::Kitties(Event::Transfered { from, to, kitty_id })
        ));
        // if (from == alice)
        // 其他判断处理 &&(to==    )&&(kitty_id== )
        //   if creator == alice
        // 其他判断处理
        //
    });
}

#[test]
fn it_works_for_kitties_transfer_failed() {
    new_test_ext().execute_with(|| {
        run_to_block(1);
        let alice: u64 = 0;
        let mut caller = <<Test as Config>::RuntimeOrigin>::signed(alice);
        //let alice_account_id = system::RawOrigin::<u64>::from(alice).into();

        //连续调用两次：
        assert_ok!(Kitties::create(caller));

        caller = <<Test as Config>::RuntimeOrigin>::signed(alice);
        assert_ok!(Kitties::create(caller));

        let id_new = Kitties::get_next_new_id();
        //assert_ok!(pallet_kitties::call::create(caller));

        caller = <<Test as Config>::RuntimeOrigin>::none();

        let res = Kitties::breed(caller, id_new - 1, id_new - 2);
        assert!(res.is_err());

        // 判断其它的错误信息；ID, DATA, 等等……
        // 非空
        //let events = System::events();
        //assert!(!events.is_empty());
        //获得最新的消息；
        // let new_events =events.into_iter().next();
        //let new_events = events.into_iter().next().unwrap();
        // 其他判断处理
        //   if (creator == alice)&&(kitty_1==id_new -1 )&&(kitty_2==id_new -2)
    });
}

#[test]
fn it_works_for_kitties_sale_ok() {
    new_test_ext().execute_with(|| {
        run_to_block(1);
        let alice: u64 = 0;
        let caller = <<Test as Config>::RuntimeOrigin>::signed(alice);

        //先创建一个；
        assert_ok!(Kitties::create(caller.clone()));
        // 非空
        let mut events = System::events();
        assert!(!events.is_empty());
        //获得最新的消息；
        let mut new_events = events.into_iter().next().unwrap();

        // 判断消息的发出者是alice
        assert!(matches!(
            new_events.event,
            mock::RuntimeEvent::Kitties(Event::KittyCreated { creator, index, data })
                if creator == alice
        ));

        //let caller2 = <<Test as Config>::RuntimeOrigin>::signed(alice);
        //转移Kitty,
        let bob: u64 = 12;

        let to = <<Test as Config>::RuntimeOrigin>::signed(bob);

        let id_new = Kitties::get_next_new_id();

        assert_ok!(Kitties::transfer(caller.clone(), bob, id_new - 1));

        events = System::events();
        assert!(!events.is_empty());

        //获得最新的消息；
        new_events = events.into_iter().next().unwrap();

        //判断消息的发出者是alice
        assert!(matches!(
        new_events.event,
          mock::RuntimeEvent::Kitties(Event::Transfered { from, to, kitty_id })
             if (from == alice)
         ));
        // 其他判断处理 &&(to==    )&&(kitty_id== )
        //   if creator == alice
        // 其他判断处理
        //
    });
}



#[test]
fn it_works_for_kitties_sale_failed() {
    new_test_ext().execute_with(|| {
        run_to_block(1);
        let alice: u64 = 0;
        let mut caller = <<Test as Config>::RuntimeOrigin>::signed(alice);
        //let alice_account_id = system::RawOrigin::<u64>::from(alice).into();

        //连续调用两次：
        assert_ok!(Kitties::create(caller));

        caller = <<Test as Config>::RuntimeOrigin>::signed(alice);
        assert_ok!(Kitties::create(caller));

        let id_new = Kitties::get_next_new_id();
        //assert_ok!(pallet_kitties::call::create(caller));

        caller = <<Test as Config>::RuntimeOrigin>::none();

        let res = Kitties::breed(caller, id_new - 1, id_new - 2);
        assert!(res.is_err());

        // 判断其它的错误信息；ID, DATA, 等等……
        // 非空
        //let events = System::events();
        //assert!(!events.is_empty());
        //获得最新的消息；
        // let new_events =events.into_iter().next();
        //let new_events = events.into_iter().next().unwrap();
        // 其他判断处理
        //   if (creator == alice)&&(kitty_1==id_new -1 )&&(kitty_2==id_new -2)
    });
}
