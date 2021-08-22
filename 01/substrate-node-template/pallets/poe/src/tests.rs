use crate::{Error, mock::*};
use frame_support::{assert_ok, assert_noop};
use super::*;

// Exercise-1
#[test]
fn test_create_claim_success(){
	new_test_ext().execute_with(|| {
		let claim = vec![0,1];

        assert_ok!(PoeModule::create_claim(Origin::signed(1), claim.clone()));

        assert_eq!(Proofs::<Test>::get(&claim), (1, frame_system::Module::<Test>::block_number()))
	});
}

#[test]
fn test_create_claim_failed_because_claim_already_exist(){
	new_test_ext().execute_with(|| {
		let claim = vec![0,1];

        let _ = PoeModule::create_claim(Origin::signed(1), claim.clone());

		assert_noop!( 
            PoeModule::create_claim(Origin::signed(1), claim.clone()),
            Error::<Test>::ProofAlreadyClaimed
        );
	});
}



#[test]
fn test_revoke_claim_success(){
    new_test_ext().execute_with(|| {
        let claim = vec![0,1];
        let _ = PoeModule::create_claim(Origin::signed(1), claim.clone());

        assert_ok!(PoeModule::revoke_claim(Origin::signed(1), claim.clone()));
    });
}

#[test]
fn test_revoke_claim_failed_because_claim_not_exist() {
    new_test_ext().execute_with(|| {
		let claim = vec![0,1];
        assert_noop!(
            PoeModule::revoke_claim(Origin::signed(1), claim.clone()),
            Error::<Test>::NoSuchProof
        );
    })
}

#[test]
fn test_revoke_claim_failed_because_sender_not_claim_owner() {
    new_test_ext().execute_with(|| {
		let claim = vec![0,1];
		let _ = PoeModule::create_claim(Origin::signed(1), claim.clone());
        assert_noop!(
            PoeModule::revoke_claim(Origin::signed(2), claim.clone()),
            Error::<Test>::NotProofOwner
        );
    })
}

#[test]
fn test_transfer_claim_success() {
    new_test_ext().execute_with(|| {
		let claim = vec![0,1];
		let _ = PoeModule::create_claim(Origin::signed(1), claim.clone());
        assert_ok!(PoeModule::transfer_claim(Origin::signed(1), claim.clone(), 2));
    })
}

#[test]
fn test_transfer_claim_failed_bacause_claim_not_exist() {
    new_test_ext().execute_with(|| {
		let claim = vec![0,1];
        assert_noop!(
			PoeModule::transfer_claim(Origin::signed(1), claim.clone(), 2),
			Error::<Test>::CalimNotExist
		);
    })
}

#[test]
fn test_transfer_claim_failed_bacause_not_claim_owner() {
    new_test_ext().execute_with(|| {
		let claim = vec![0,1];
		let _ = PoeModule::create_claim(Origin::signed(1), claim.clone());
        assert_noop!(
			PoeModule::transfer_claim(Origin::signed(2), claim.clone(), 3),
			Error::<Test>::NotProofOwner
		);
    })
}




// Exercise-2
#[test]
fn test_create_claim_failed_because_claim_length_too_large(){
	new_test_ext().execute_with(|| {
		let claim = vec![0,1,2,3,4,5,6,7];
		assert_noop!( 
            PoeModule::create_claim(Origin::signed(1), claim.clone()),
            Error::<Test>::ClaimLengthTooLarge
        );
	});
}