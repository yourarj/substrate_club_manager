use crate::{mock::*, pallet, Error};
use frame_support::{assert_noop, assert_ok};
use sp_runtime::DispatchError;

#[test]
fn test_fail_on_boarding_member_if_not_root() {
	new_test_ext().execute_with(|| {
		let club_id = 1004;
		let account_id = 1;

		// populate club
		pallet::Clubs::<Test>::insert(1004, ());

		// should throw error if not sudo
		assert_noop!(
			ClubManagerModule::member_on_board(Origin::signed(account_id), club_id, account_id),
			DispatchError::BadOrigin
		);

		assert!(pallet::Membership::<Test>::get(club_id, account_id).is_none());
	});
}

#[test]
fn test_fail_on_boarding_member_non_existent_club() {
	new_test_ext().execute_with(|| {
		let club_id = 1004;
		let account_id = 1;

		// should throw error if not sudo
		assert_noop!(
			ClubManagerModule::member_on_board(Origin::root(), club_id, account_id),
			Error::<Test>::ClubNotFound
		);

		assert!(pallet::Membership::<Test>::get(club_id, account_id).is_none());
	});
}

#[test]
fn test_succeed_on_boarding_member_to_club() {
	new_test_ext().execute_with(|| {
		let club_id = 1004;
		let account_id = 1;

		// populate club
		pallet::Clubs::<Test>::insert(1004, ());

		assert_ok!(ClubManagerModule::member_on_board(Origin::root(), club_id, account_id));

		// verify entry is made
		assert!(pallet::Membership::<Test>::get(club_id, account_id).is_some());
	});
}

#[test]
fn test_fail_re_on_boarding_member_to_club() {
	new_test_ext().execute_with(|| {
		let club_id = 1004;
		let account_id = 1;

		// populate club
		pallet::Clubs::<Test>::insert(1004, ());

		assert_ok!(ClubManagerModule::member_on_board(Origin::root(), club_id, account_id));

		// should throw error if member already exist
		assert_noop!(
			ClubManagerModule::member_on_board(Origin::root(), club_id, account_id),
			Error::<Test>::ExistingMember
		);
	});
}

#[test]
fn test_fail_off_boarding_member_from_club_if_not_root() {
	new_test_ext().execute_with(|| {
		let club_id = 8809;
		let account_id = 1;

		// populate club
		pallet::Clubs::<Test>::insert(8809, ());

		assert_ok!(ClubManagerModule::member_on_board(Origin::root(), club_id, account_id));

		// verify entry is made
		assert!(pallet::Membership::<Test>::get(club_id, account_id).is_some());

		// try off boarding already off boarded user
		assert_noop!(
			ClubManagerModule::member_off_board(Origin::signed(1), club_id, account_id),
			DispatchError::BadOrigin
		);

		// verify entry is made
		assert!(pallet::Membership::<Test>::get(club_id, account_id).is_some());
	});
}

#[test]
fn test_fail_off_boarding_member_if_club_does_not_exist() {
	new_test_ext().execute_with(|| {
		let club_id = 8809;
		let account_id = 1;

		// try off boarding already off boarded user
		assert_noop!(
			ClubManagerModule::member_off_board(Origin::root(), club_id, account_id),
			Error::<Test>::ClubNotFound
		);
	});
}

#[test]
fn test_succeed_off_boarding_member() {
	new_test_ext().execute_with(|| {
		let club_id = 8809;
		let account_id = 1;

		// populate club
		pallet::Clubs::<Test>::insert(8809, ());

		assert_ok!(ClubManagerModule::member_on_board(Origin::root(), club_id, account_id));

		// verify entry is made
		assert!(pallet::Membership::<Test>::get(club_id, account_id).is_some());

		// try off boarding already off boarded user
		assert_ok!(ClubManagerModule::member_off_board(Origin::root(), club_id, account_id));

		// verify entry is made
		assert!(pallet::Membership::<Test>::get(club_id, account_id).is_none());
	});
}

#[test]
fn test_fail_off_boarding_member_if_not_a_member() {
	new_test_ext().execute_with(|| {
		let club_id = 8809;
		let account_id = 1;

		// populate club
		pallet::Clubs::<Test>::insert(8809, ());

		// verify entry is made
		assert!(pallet::Membership::<Test>::get(club_id, account_id).is_none());

		// try off boarding already off boarded user
		assert_noop!(
			ClubManagerModule::member_off_board(Origin::root(), club_id, account_id),
			Error::<Test>::MemberNotFound
		);
	});
}
