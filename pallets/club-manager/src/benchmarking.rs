//! Benchmarking setup for pallet-club_manager

use super::*;

#[allow(unused)]
use crate::Pallet as ClubManagerModule;
use frame_benchmarking::{account, benchmarks};
use frame_system::RawOrigin;

benchmarks! {
	member_on_board {
		let a in 0 .. 100_u32;
		let b in 0 .. 2_u32;
		let acc: T::AccountId = account("test", 0, a);
		let club_id = T::ClubId::from(b);
		// add club
		Clubs::<T>::insert(club_id, ());
	}: _(RawOrigin::Root, club_id, account("test", 0, a))
	verify {
		assert!(Membership::<T>::get(club_id, acc).is_some());
	}

	member_off_board {
		let a in 0 .. 100_u32;
		let b in 0 .. 2_u32;
		let acc: T::AccountId = account("test", 0, a);
		let club_id = T::ClubId::from(b);
		// add respective club
		Clubs::<T>::insert(club_id, ());
		// on board member
		ClubManagerModule::<T>::member_on_board(RawOrigin::Root.into(), club_id, acc.clone()).unwrap();
	}: _(RawOrigin::Root, club_id, acc.clone())
	verify {
		assert!(Membership::<T>::get(club_id, acc).is_none());
	}

	impl_benchmark_test_suite!(ClubManagerModule, crate::mock::new_test_ext(), crate::mock::Test);
}
