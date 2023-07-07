#![cfg_attr(not(feature = "std"), no_std)]
pub use pallet::*;
#[frame_support::pallet]
pub mod pallet {
	use frame_support::pallet_prelude::*;
	use frame_system::pallet_prelude::*;
	use scale_info::prelude::vec::Vec;
	#[derive(Clone, PartialEq, Eq, Encode, Decode, TypeInfo)]
	pub struct StudentInfo {
		mssv: u32,
		full_name: Vec<u8>,
		class: Vec<u8>,
		age: u8,
		school: Vec<u8>,
	}

	type MSSV = u32;
	#[pallet::pallet]
	pub struct Pallet<T>(_);
	#[pallet::config]
	pub trait Config: frame_system::Config {
		type RuntimeEvent: From<Event<Self>> + IsType<<Self as frame_system::Config>::RuntimeEvent>;
		#[pallet::constant]
		type MaxLength: Get<u32>;
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
	// pub type NameStorage<T: Config> =
	// StorageMap<_, Blake2_128Concat, T::AccountId, BoundedVec<u8, T::MaxLength>>;
	pub type Students<T: Config> = StorageMap<_, Blake2_128Concat, MSSV, StudentInfo>;
	#[pallet::call]
	impl<T: Config> Pallet<T> {
		#[pallet::call_index(0)]
		#[pallet::weight({50_000})]
		pub fn create_a_student(
			origin: OriginFor<T>,
			mssv: u32,
			name: Vec<u8>,
			class: Vec<u8>,
			age: u8,
			school: Vec<u8>,
		) -> DispatchResult {
			let who = ensure_signed(origin)?;
			// let bounded_name: BoundedVec<u8, T::MaxLength> =
			// 	name.clone().try_into().map_err(|_| Error::<T>::TooLong)?;
			let student = StudentInfo { mssv, full_name: name.clone(), class, age, school };
			Students::<T>::insert(&mssv, student);
			Self::deposit_event(Event::SetName(who, name));
			Ok(())
		}
	}
}
