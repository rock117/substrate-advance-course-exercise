#![cfg_attr(not(feature = "std"), no_std)]

/// Edit this file to define custom logic or remove it if it is not needed.
/// Learn more about FRAME and the core library of Substrate FRAME pallets:
/// <https://substrate.dev/docs/en/knowledgebase/runtime/frame>
pub use pallet::*;

#[cfg(test)]
mod mock;

#[cfg(test)]
mod tests;

#[cfg(feature = "runtime-benchmarks")]
mod benchmarking;

#[frame_support::pallet]
pub mod pallet {
    use frame_support::dispatch::fmt::Debug;
    use frame_support::{
        dispatch::DispatchResult, pallet_prelude::*, sp_runtime::traits::Hash,
        traits::tokens::currency::Currency, traits::tokens::ExistenceRequirement,
        traits::Randomness,
    };

    use frame_support::debug;
    use frame_system::pallet_prelude::*;
    use sp_core::H256;
    use sp_io::hashing::{blake2_128, blake2_256, twox_128, twox_256, twox_64};
    use sp_runtime::print;
    use sp_runtime::traits::AtLeast32Bit;
    use sp_runtime::traits::MaybeDisplay;
    use sp_runtime::DispatchErrorWithPostInfo;

    /// Configure the pallet by specifying the parameters and types on which it depends.
    // #[pallet::config]
    // pub trait Config: frame_system::Config {
    // 	/// Because this pallet emits events, it depends on the runtime's definition of an event.
    // 	type Event: From<Event<Self>> + IsType<<Self as frame_system::Config>::Event>;
    // }
    pub type KittyIndex = u64;
    #[pallet::config]
    pub trait Config: pallet_balances::Config + frame_system::Config {
        type Event: From<Event<Self>> + IsType<<Self as frame_system::Config>::Event>;
        type Randomness: Randomness<H256, Self::BlockNumber>;
        // type KittyIndex: Parameter
        // + Member
        // + MaybeSerializeDeserialize
        // + Debug
        // + Default
        // + MaybeDisplay
        // + AtLeast32Bit
        // + sp_std::hash::Hash
        // + Copy;
    }

    #[pallet::pallet]
    #[pallet::generate_store(pub(super) trait Store)]
    pub struct Pallet<T>(_);

    #[pallet::storage]
    #[pallet::getter(fn kitties_count)]
    pub(super) type KittiesCount<T: Config> = StorageValue<_, Option<u64>, ValueQuery>;

    #[pallet::storage]
    #[pallet::getter(fn get_nonce)]
    pub(super) type Nonce<T: Config> = StorageValue<_, u64, ValueQuery>;

    #[pallet::storage]
    #[pallet::getter(fn kitties)]
    pub(super) type Kitties<T: Config> =
        StorageMap<_, Blake2_128Concat, KittyIndex, Option<Kitty>, ValueQuery>;

    #[pallet::storage]
    #[pallet::getter(fn owner)]
    pub(super) type Owner<T: Config> =
        StorageMap<_, Blake2_128Concat, KittyIndex, Option<T::AccountId>, ValueQuery>;

    // Pallets use events to inform users when important changes are made.
    // https://substrate.dev/docs/en/knowledgebase/runtime/events
    #[pallet::event]
    #[pallet::metadata(T::AccountId = "AccountId")]
    #[pallet::generate_deposit(pub(super) fn deposit_event)]
    pub enum Event<T: Config> {
        KittyCreated(T::AccountId, u64),
        PriceSet(T::AccountId, T::Hash, T::Balance),
        KittyTransferred(T::AccountId, T::AccountId, KittyIndex),
        Bought(T::AccountId, T::AccountId, T::Hash, T::Balance),
        KittyBreeded(T::AccountId, KittyIndex, KittyIndex),
    }

    // Errors inform users that something went wrong.
    #[pallet::error]
    pub enum Error<T> {
        /// Errors should have helpful documentation associated with them.
        StorageOverflow,
        KittyAlreadyExist,
        KittyCountOverflow,
        NotKittyOwner,
        KittyIndexInvalid,
        SameParentIndex,
        KittyNotExist,
    }

    // Dispatchable functions allows users to interact with the pallet and invoke state changes.
    // These functions materialize as "extrinsics", which are often compared to transactions.
    // Dispatchable functions must be annotated with a weight and must return a DispatchResult.

    #[derive(Clone, Encode, Decode, Default, PartialEq)]
    pub struct Kitty(pub [u8; 16]);

    #[pallet::call]
    impl<T: Config> Pallet<T> {
        #[pallet::weight(10)]
        pub fn create_kitty(origin: OriginFor<T>) -> DispatchResultWithPostInfo {
            let sender = ensure_signed(origin)?;
            let dna = Self::random_hash(&sender);
            Self::create(sender, dna)
        }

        #[pallet::weight(10)]
        pub fn transfer_kitty(
            origin: OriginFor<T>,
            dest: T::AccountId,
            kitty_id: KittyIndex,
        ) -> DispatchResultWithPostInfo {
            let sender: T::AccountId = ensure_signed(origin)?;
            ensure!(
                Kitties::<T>::contains_key(kitty_id),
                Error::<T>::KittyNotExist
            );
            ensure!(
                Some(sender.clone()) == Owner::<T>::get(kitty_id),
                Error::<T>::NotKittyOwner
            );
            Owner::<T>::insert(kitty_id, Some(dest.clone()));
            Self::deposit_event(Event::KittyTransferred(sender, dest, kitty_id));
            Ok(().into())
        }

        #[pallet::weight(10)]
        pub fn breed_kitty(
            origin: OriginFor<T>,
            kitty1: KittyIndex,
            kitty2: KittyIndex,
        ) -> DispatchResultWithPostInfo {
            let sender = ensure_signed(origin)?;
            ensure!(
                Kitties::<T>::contains_key(kitty1),
                Error::<T>::KittyNotExist
            );
            ensure!(
                Kitties::<T>::contains_key(kitty2),
                Error::<T>::KittyNotExist
            );
            ensure!(kitty1 != kitty2, Error::<T>::SameParentIndex);
            let dna = Self::generate_dna_from_2kitties(sender.clone(), kitty1, kitty2)?;
            Self::create(sender, dna)
        }

        #[pallet::weight(10)]
        pub fn buy_kitty(
            origin: OriginFor<T>,
            kitty_id: KittyIndex,
            kitty_price: T::Balance,
        ) -> DispatchResultWithPostInfo {
            let buyer = ensure_signed(origin)?;
            let kitty_owner = Owner::<T>::get(kitty_id).ok_or(Error::<T>::KittyNotExist)?;
            ensure!(&buyer != &kitty_owner, Error::<T>::NotKittyOwner);

            <pallet_balances::Pallet<T> as Currency<_>>::transfer(
                &buyer,
                &kitty_owner,
                kitty_price,
                ExistenceRequirement::KeepAlive,
            )?;

            Ok(().into())
        }

        #[pallet::weight(10)]
        pub fn sell_kitty(
            origin: OriginFor<T>,
            buyer: T::AccountId,
            kitty_id: KittyIndex,
            kitty_price: T::Balance,
        ) -> DispatchResultWithPostInfo {
            let seller = ensure_signed(origin)?;
            ensure!(
                Some(seller.clone()) == Owner::<T>::get(kitty_id),
                Error::<T>::NotKittyOwner
            );

            <pallet_balances::Pallet<T> as Currency<_>>::transfer(
                &buyer,
                &seller,
                kitty_price,
                ExistenceRequirement::KeepAlive,
            )?;

            Ok(().into())
        }
    }

    impl<T: Config> Pallet<T> {
        fn random_hash(sender: &T::AccountId) -> [u8; 16] {
            let payload = (
                T::Randomness::random_seed(),
                sender,
                <frame_system::Pallet<T>>::extrinsic_index(),
            );
            payload.using_encoded(blake2_128)
        }

        fn create(kitty_owner: T::AccountId, dna: [u8; 16]) -> DispatchResultWithPostInfo {
            let kitty_id = match Self::kitties_count() {
                Some(count) => {
                    ensure!(
                        count != KittyIndex::max_value(),
                        Error::<T>::KittyCountOverflow
                    );
                    count
                }
                None => 0,
            };
           

            Kitties::<T>::insert(kitty_id, Some(Kitty(dna)));
            Owner::<T>::insert(kitty_id, Some(kitty_owner.clone()));
            <KittiesCount<T>>::put(Some(kitty_id + 1));
            Self::deposit_event(Event::KittyCreated(kitty_owner, kitty_id));
            Ok(().into())
        }

        fn generate_dna_from_2kitties(
            sender: T::AccountId,
            kitty1: KittyIndex,
            kitty2: KittyIndex,
        ) -> Result<[u8; 16], Error<T>> {
            let parent1 = Self::kitties(kitty1).ok_or(Error::<T>::KittyIndexInvalid)?;
            let parent2 = Self::kitties(kitty2).ok_or(Error::<T>::KittyIndexInvalid)?;
            let parent1_dna = parent1.0;
            let parent2_dna = parent2.0;
            let base_dna = Self::random_hash(&sender);
            let mut dna = [0u8; 16];
            for i in 0..dna.len() {
                dna[i] = (base_dna[i] & parent1_dna[i]) | (!base_dna[i] & parent2_dna[i])
            }
            Ok(dna)
        }
    }
}
