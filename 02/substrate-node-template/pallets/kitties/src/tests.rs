use super::*;
use crate::{mock::*, Error};
use frame_support::{assert_noop, assert_ok};
#[test]
fn test_create_kitty_success() {
    new_test_ext().execute_with(|| {
        System::set_block_number(10);
        assert_ok!(KittiesModule::create_kitty(Origin::signed(1)));
    });
}

#[test]
fn test_transfer_kitty_success() {
    new_test_ext().execute_with(|| {
        System::set_block_number(10);
        KittiesModule::create_kitty(Origin::signed(1));
        let kittyid = <KittiesCount<Test>>::get().unwrap() - 1;

        assert_ok!(KittiesModule::transfer_kitty(Origin::signed(1), 2, kittyid));
        assert_eq!(Owner::<Test>::get(kittyid), Some(2));
    });
}
#[test]
fn test_transfer_kitty_failed_dueto_not_kitty_not_exist() {
    new_test_ext().execute_with(|| {
        System::set_block_number(10);
        assert_noop!(
            KittiesModule::transfer_kitty(Origin::signed(10), 2, u64::max_value()),
            Error::<Test>::KittyNotExist
        );
    });
}

#[test]
fn test_transfer_kitty_failed_dueto_not_kitty_owner() {
    new_test_ext().execute_with(|| {
        System::set_block_number(10);
        KittiesModule::create_kitty(Origin::signed(1));
        let next_kittyid = <KittiesCount<Test>>::get().unwrap();

        assert_noop!(
            KittiesModule::transfer_kitty(Origin::signed(10), 2, next_kittyid - 1),
            Error::<Test>::NotKittyOwner
        );
    });
}

#[test]
fn test_breed_kitty_success() {
    new_test_ext().execute_with(|| {
        System::set_block_number(10);
        let parent1 = create_kitty(1);
        let parent2 = create_kitty(1);
        assert_ok!(KittiesModule::breed_kitty(
            Origin::signed(1),
            parent1,
            parent2
        ));
    });
}

#[test]
fn test_breed_kitty_failed_dueto_kitty_not_exist() {
    new_test_ext().execute_with(|| {
        System::set_block_number(10);
        assert_noop!(
            KittiesModule::breed_kitty(Origin::signed(1), 1, 2),
            Error::<Test>::KittyNotExist
        );
    });
}

#[test]
fn test_breed_kitty_failed_dueto_kitty_same_parent() {
    new_test_ext().execute_with(|| {
        System::set_block_number(10);
        let parent1 = create_kitty(1);
        assert_noop!(
            KittiesModule::breed_kitty(Origin::signed(1), 0, 0),
            Error::<Test>::SameParentIndex
        );
    });
}

fn create_kitty(owner: u64) -> u64 {
    KittiesModule::create_kitty(Origin::signed(owner));
    <KittiesCount<Test>>::get().unwrap() - 1
}
