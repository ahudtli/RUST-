#![cfg_attr(not(feature = "std"), no_std)]

pub use pallet::*;


#[cfg(test)]
mod mock;

#[cfg(test)]
mod tests;

#[frame_support::pallet]
pub mod pallet {
	use frame_support::{dispatch::DispatchResult, pallet_prelude::*, traits::{Randomness,Currency,ReservableCurrency,ExistenceRequirement}};
	use frame_system::pallet_prelude::*;
	use codec::{Encode,Decode};
	use sp_io::hashing::blake2_128;	
//	use sp_runtime::{traits::{AtLeast32BitUnsigned, MaybeSerializeDeserialize, CheckedAdd, Zero, One, Bounded }};
//    	use core::convert::TryInto;

	#[derive(Encode,Decode)]
	pub struct Kitty(pub [u8;16]);

	type KittyIndex = u32;

	pub type BalanceOf<T> = <<T as Config>::Currency as Currency<<T as frame_system::Config>::AccountId>>::Balance;

	#[pallet::config]
	pub trait Config: frame_system::Config {
		type Event: From<Event<Self>> + IsType<<Self as frame_system::Config>::Event>;
		type Randomness:Randomness<Self::Hash>;
		type Currency: Currency<Self::AccountId>+ ReservableCurrency<Self::AccountId>;
}

	#[pallet::pallet]
	#[pallet::generate_store(pub(super) trait Store)]
	pub struct Pallet<T>(_);

	#[pallet::event]
	#[pallet::metadata(T::AccountId = "AccountId")]
	#[pallet::generate_deposit(pub(super) fn deposit_event)]
	pub enum Event<T: Config> {
		KittyCreate(T::AccountId, KittyIndex),
		KittyTransfer(T::AccountId, T::AccountId, KittyIndex),
		KittyBUY(T::AccountId, T::AccountId, BalanceOf<T>, KittyIndex),
	}

	#[pallet::storage]
	#[pallet::getter(fn kitties_count)]
	pub type KittiesCount<T> = StorageValue<_,u32>;

	#[pallet::storage]
        #[pallet::getter(fn kitties)]
        pub type Kitties<T> = StorageMap<_,Blake2_128Concat,KittyIndex,Option<Kitty>,ValueQuery>;

        #[pallet::storage]
        #[pallet::getter(fn owner)]
        pub type Owner<T:Config> = StorageMap<_,Blake2_128Concat,KittyIndex,Option<T::AccountId>,ValueQuery>;

	#[pallet::error]
	pub enum Error<T> {
		KittiesCountOverflow,
		NotOwner,
		SameParentIndex,
		InvalidKittyIndex,
		}

	#[pallet::hooks]
	impl<T: Config> Hooks<BlockNumberFor<T>> for Pallet<T> {}

	#[pallet::call]
	impl<T:Config> Pallet<T> {
		#[pallet::weight(0)]
		///创建小猫并抵押token
		pub fn create(origin: OriginFor<T>,balance: BalanceOf<T>,)->DispatchResultWithPostInfo{
			let who = ensure_signed(origin)?;
		
			let kitty_id = match Self::kitties_count(){
				Some(id) =>{
					ensure!(id !=KittyIndex::max_value(),Error::<T>::KittiesCountOverflow);
					id
				},
				None=>{
					1
				}
			};
		
			let dna = Self::random_value(&who);
	
			Kitties::<T>::insert(kitty_id,Some(Kitty(dna)));
			Owner::<T>::insert(kitty_id,Some(who.clone()));
			KittiesCount::<T>::put(kitty_id + 1);
			
			let reserve: u64 = 100;
            		T::Currency::reserve(&who, balance)?;

			Self::deposit_event(Event::KittyCreate(who,kitty_id));
			
			Ok(().into())
		}

		///交易小猫
		#[pallet::weight(0)]
		pub fn transfer(origin:OriginFor<T>, new_owner:T::AccountId, kitty_id:KittyIndex)->DispatchResultWithPostInfo{
			let who = ensure_signed(origin)?;
			let kitty = Self::kitties(kitty_id).ok_or(Error::<T>::InvalidKittyIndex)?;
			ensure!(Some(who.clone())==Owner::<T>::get(kitty_id),Error::<T>::NotOwner);
			Owner::<T>::insert(kitty_id,Some(new_owner.clone()));
			Self::deposit_event(Event::KittyTransfer(who,new_owner,kitty_id));

			Ok(().into())
		}
		///繁殖
		#[pallet::weight(0)]
		pub fn breed(origin: OriginFor<T>, kitty_id_1:KittyIndex, kitty_id_2:KittyIndex)->DispatchResultWithPostInfo{
			let who = ensure_signed(origin)?;
			ensure!(kitty_id_1 != kitty_id_2,Error::<T>::SameParentIndex);
			
			let kitty1 = Self::kitties(kitty_id_1).ok_or(Error::<T>::InvalidKittyIndex)?;
			let kitty2 = Self::kitties(kitty_id_2).ok_or(Error::<T>::InvalidKittyIndex)?;

			let kitty_id = match Self::kitties_count(){
				Some(id)=>{
					ensure!(id != KittyIndex::max_value(),Error::<T>::KittiesCountOverflow);
					id
				}
				None=>{
					1
				}
			};

			let dna_1 = kitty1.0;
			let dna_2 = kitty2.0;

			let selector = Self::random_value(&who);
			let mut new_dna = [0u8;16];

			for i in 0..dna_1.len(){
				new_dna[i] = ((selector[i] & dna_1[i]) | (!selector[i] & dna_2[i]));
			}
			
			Kitties::<T>::insert(kitty_id,Some(Kitty(new_dna)));
                        Owner::<T>::insert(kitty_id,Some(who.clone()));
                        KittiesCount::<T>::put(kitty_id + 1);			
			Self::deposit_event(Event::KittyCreate(who,kitty_id));

			Ok(().into())
		}

		///买卖小猫
		#[pallet::weight(0)]
		pub fn kitty_buy(origin:OriginFor<T>, new_owner:T::AccountId, kitty_id:KittyIndex, balance:BalanceOf<T>)->DispatchResultWithPostInfo{
			let who = ensure_signed(origin)?;
                        ensure!(Some(who.clone())==Owner::<T>::get(kitty_id),Error::<T>::NotOwner);
			T::Currency::transfer(&who, &new_owner, balance, ExistenceRequirement::AllowDeath)?;
                        Owner::<T>::insert(kitty_id,Some(new_owner.clone()));
                        Self::deposit_event(Event::KittyBUY(who, new_owner, balance, kitty_id));

                        Ok(().into())
		}

	}

	impl<T:Config> Pallet<T> {
		fn random_value(sender:&T::AccountId)->[u8;16]{
			let payload = (
				T::Randomness::random_seed(),
				&sender,
				<frame_system::Pallet<T>>::extrinsic_index(),
			);
			payload.using_encoded(blake2_128)
        	}
	}
}
