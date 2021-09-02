use crate::{Error, mock::*};
use frame_support::{assert_ok, assert_noop};
use super::*;


#[test]
fn  create_kitty_succeed() {
    new_test_ext().execute_with(|| {
        assert_eq!(Balances::free_balance(&1),100);
        assert_eq!(Balances::reserved_balance(&1), 0);
        assert_ok!(KittiesModule::create(Origin::signed(1), 10 ));
        assert_eq!(Balances::free_balance(&1), 90);
        assert_eq!(Balances::reserved_balance(&1), 10);
    });
}


#[test]
fn  transfer_kitty_succeed() {
    new_test_ext().execute_with(|| {
        KittiesModule::create(Origin::signed(1),100);
        assert_ok!(KittiesModule::transfer(Origin::signed(1), 2, 1 ));
    });
}


#[test]
fn  transfer_kitty_faild_when_invalidKittyIndex() {
    new_test_ext().execute_with(|| {
        KittiesModule::create(Origin::signed(3), 100 );
	assert_noop!(
		KittiesModule::transfer(Origin::signed(3), 2, 8 ),
		Error::<Test>::InvalidKittyIndex
        );
    });
}

#[test]
fn  transfer_kitty_faild_when_notOwner() {
    new_test_ext().execute_with(|| {
        KittiesModule::create(Origin::signed(1), 100 );

	assert_noop!(
		KittiesModule::transfer(Origin::signed(2), 4, 3),
		Error::<Test>::NotOwner    
    	);
    });
}
