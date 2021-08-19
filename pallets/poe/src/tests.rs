use crate::{mock::*, Error};
use frame_support::{assert_noop, assert_ok};
use super::*;
use frame_system::pallet_prelude::*;

const CLAIM_HASH: &str = "claim message";
const EXPECT_SENDER: u64 = 1;
const EXPECT_RECEIVER: u64 = 2;

#[test]
fn create_claim_test() {
    new_test_ext().execute_with(|| {
        let proof = CLAIM_HASH.as_bytes().to_vec();

        assert_ok!(PoeModule::create_claim(Origin::signed(EXPECT_SENDER), proof.clone()));
        assert_eq!(
            Proofs::<Test>::get(&proof),
            Some((1u64, frame_system::Pallet::<Test>::block_number()))
        );
    })
}

#[test]
fn create_claim_when_hash_oversize() {
	new_test_ext().execute_with(|| {
		let proof = "sllskdkkdfladfadfadfadfadfadfadf";

		assert_noop!(
			PoeModule::create_claim(Origin::signed(EXPECT_SENDER), proof.as_bytes().to_vec()),
			Error::<Test>::OutOfLimitSize
		);
	});
}

#[test]
fn create_claim_when_proof_exist() {
	new_test_ext().execute_with(|| {
		let proof = CLAIM_HASH.as_bytes().to_vec();

		let _ =PoeModule::create_claim(Origin::signed(EXPECT_SENDER), proof.clone());
		assert_noop!(
			PoeModule::create_claim(Origin::signed(EXPECT_SENDER), proof.clone()),
			Error::<Test>::ProofAlreadyClaimed
		);
	});
}

#[test]
fn revoke_claim(){
	new_test_ext().execute_with(|| {
		let proof = CLAIM_HASH.as_bytes().to_vec();
		let _ =PoeModule::create_claim(Origin::signed(EXPECT_SENDER), proof.clone());

		assert_ok!(PoeModule::revoke_claim(Origin::signed(EXPECT_SENDER), proof.clone()));
		assert_eq!(Proofs::<Test>::get(&proof), None);
	});
}

#[test]
fn revoke_claim_when_proof_not_exist() {
	new_test_ext().execute_with(|| {
		let proof = CLAIM_HASH.as_bytes().to_vec();
		assert_noop!(
			PoeModule::revoke_claim(Origin::signed(EXPECT_SENDER), proof.clone()),
			Error::<Test>::NoSuchProof
		);
	})
}

#[test]
fn transfer_claim() {
	new_test_ext().execute_with(|| {
		let proof = CLAIM_HASH.as_bytes().to_vec();
		let _ =PoeModule::create_claim(Origin::signed(EXPECT_SENDER), proof.clone());

		let receiver = ensure_signed(Origin::signed(EXPECT_RECEIVER)).unwrap();

		assert_ok!(PoeModule::transfer_claim(Origin::signed(EXPECT_SENDER), proof.clone(), receiver));
		assert_eq!(
			Proofs::<Test>::get(&proof),
			Some((EXPECT_RECEIVER, frame_system::Pallet::<Test>::block_number()))
		);
	});
}

#[test]
fn transfer_claim_when_not_owner(){
	new_test_ext().execute_with(|| {
		let proof = CLAIM_HASH.as_bytes().to_vec();
		let _ =PoeModule::create_claim(Origin::signed(EXPECT_SENDER), proof.clone());

		let receiver = ensure_signed(Origin::signed(EXPECT_RECEIVER)).unwrap();

		assert_noop!(
			PoeModule::transfer_claim(Origin::signed(3), proof.clone(), receiver),
			Error::<Test>::NotProofOwner
		);
	});

}
