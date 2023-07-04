#![cfg_attr(not(feature = "std"), no_std)]
pub use pallet::*;
#[frame_support::pallet]
pub mod pallet {
	use frame_support::pallet_prelude::*;
	use frame_system::pallet_prelude::*;

	#[pallet::pallet]
	pub struct Pallet<T>(_);
	#[pallet::config]
	pub trait Config: frame_system::Config {
		type RuntimeEvent: From<Event<Self>> + IsType<<Self as frame_system::Config>::RuntimeEvent>;
		#[pallet::constant]
		type Maxlength: Get<u32>;
	}
	#[pallet::event]
	#[pallet::generate_deposit(pub(super) fn deposit_event)]
	pub enum Event<T: Config> {
		SetName(T::AccountId, Vec<u8>),
	}
	#[pallet::error]
	pub enum Error<T> {
		TooLong,
	}
	#[pallet::storage]
	#[pallet::getter(fn name_storage)]
	pub type NameStorage<T: Config> =
		StorageMap<_, BlakeTwo256, T::AccountId, BoundedVec<u8, T::Maxlength>>;
	#[pallet::call]
	impl<T: Config> Pallet<T> {
		#[pallet::weight(1_000)]
		pub fn create_name(origin: OriginFor<T>, name: Vec<u8>) -> DispatchResult {
			let who = ensure_signed(origin);
			let bounded_name: BoundedVec<_, _> =
				name.clone.try_into().map_err(|_| Error::<T>::TooLong);
			NameStorage::<T>::insert(&who, bounded_name);
			Self::deposit_event(Event::SetName(who, name));
			Ok(())
		}
	}
}
