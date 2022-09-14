#![cfg_attr(not(feature = "std"), no_std)]

/// Edit this file to define custom logic or remove it if it is not needed.
/// Learn more about FRAME and the core library of Substrate FRAME pallets:
/// <https://docs.substrate.io/reference/frame-pallets/>
pub use pallet::*;

#[cfg(test)]
mod mock;

#[cfg(test)]
mod tests;

#[cfg(feature = "runtime-benchmarks")]
mod benchmarking;

pub mod weights;
pub use weights::*;

#[frame_support::pallet]
pub mod pallet {

	use codec::MaxEncodedLen;
	use frame_support::{
		pallet_prelude::{Member, *},
		Parameter,
	};
	use frame_system::pallet_prelude::*;
	use sp_runtime::{app_crypto::MaybeHash, traits::AtLeast32BitUnsigned};

	use crate::WeightInfo;

	#[pallet::pallet]
	#[pallet::generate_store(pub(super) trait Store)]
	pub struct Pallet<T>(_);

	/// Configure the pallet by specifying the parameters and types on which it depends.
	#[pallet::config]
	pub trait Config: frame_system::Config {
		/// Event type
		type Event: From<Event<Self>> + IsType<<Self as frame_system::Config>::Event>;

		/// Club ID type
		type ClubId: AtLeast32BitUnsigned
			+ Copy
			+ MaxEncodedLen
			+ Parameter
			+ Member
			+ Default
			+ MaybeSerializeDeserialize
			+ MaybeHash;

		/// Type representing the weight of this pallet
		type WeightInfo: WeightInfo;
	}

	#[pallet::storage]
	#[pallet::getter(fn clubs)]
	// as need to lookup based on two keys using double map
	pub type Clubs<T: Config> = StorageMap<_, Blake2_128, T::ClubId, ()>;

	#[pallet::storage]
	#[pallet::getter(fn membership)]
	// as need to lookup based on two keys using double map
	pub type Membership<T: Config> =
		StorageDoubleMap<_, Blake2_128, T::ClubId, Blake2_128Concat, T::AccountId, ()>;

	// event being dispatched
	#[pallet::event]
	#[pallet::generate_deposit(pub(super) fn deposit_event)]
	pub enum Event<T: Config> {
		/// new club member on boarded
		ClubMemberOnBoarded(T::AccountId, T::ClubId),
		/// club member left the membership
		ClubMemberOffBoarded(T::AccountId, T::ClubId),
	}

	// genesis configuration
	#[pallet::genesis_config]
	pub struct GenesisConfig<T: Config> {
		pub existing_clubs: std::collections::HashMap<T::ClubId, Vec<T::AccountId>>,
	}

	#[cfg(feature = "std")]
	impl<T: Config> Default for GenesisConfig<T> {
		fn default() -> Self {
			Self { existing_clubs: Default::default() }
		}
	}

	// initialize chain state with input from genesis
	#[pallet::genesis_build]
	impl<T: Config> GenesisBuild<T> for GenesisConfig<T> {
		fn build(&self) {
			self.existing_clubs.iter().for_each(|(club_id, club_members)| {
				<Clubs<T>>::insert(club_id, ());
				club_members
					.into_iter()
					.for_each(|member_id| <Membership<T>>::insert(club_id, &member_id, ()));
			});
		}
	}

	#[pallet::error]
	pub enum Error<T> {
		// specified club not found
		ClubNotFound,
		/// Member does not exist.
		MemberNotFound,
		/// Member already exists.
		ExistingMember,
	}

	#[pallet::call]
	impl<T: Config> Pallet<T> {
		/// Adds a new member (account) to the given club. Can only be called by `root`.
		/// Throws a `DuplicateMember` error, if the member was already in the club.
		/// Emits an event on success.
		#[pallet::weight(T::WeightInfo::member_on_board(1_u32))]
		pub fn member_on_board(
			origin: OriginFor<T>,
			club_id: T::ClubId,
			account_id: T::AccountId,
		) -> DispatchResult {
			ensure_root(origin)?;
			//ensure it's a valid club id
			ensure!(<Clubs<T>>::contains_key(club_id), Error::<T>::ClubNotFound);
			// ensure user is NOT a member already
			ensure!(
				!<Membership<T>>::contains_key(club_id, &account_id),
				Error::<T>::ExistingMember
			);
			<Membership<T>>::insert(club_id, &account_id, ());
			Self::deposit_event(Event::ClubMemberOnBoarded(account_id, club_id));
			Ok(())
		}

		/// Removes the given member (account) from the club if existed, otherwise throws an error.
		/// Can only be called by `root`. Emits an event on success.
		#[pallet::weight(T::WeightInfo::member_off_board(1_u32))]
		pub fn member_off_board(
			origin: OriginFor<T>,
			club_id: T::ClubId,
			account_id: T::AccountId,
		) -> DispatchResult {
			ensure_root(origin)?;
			// ensure it's a valid club_id
			ensure!(<Clubs<T>>::contains_key(club_id), Error::<T>::ClubNotFound);
			// ensure user is member of club
			ensure!(
				<Membership<T>>::contains_key(club_id, &account_id),
				Error::<T>::MemberNotFound
			);
			<Membership<T>>::remove(club_id, &account_id);
			Self::deposit_event(Event::ClubMemberOffBoarded(account_id, club_id));
			Ok(())
		}
	}
}
