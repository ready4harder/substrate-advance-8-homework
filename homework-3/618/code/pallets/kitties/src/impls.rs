use frame_support::pallet_macros::pallet_section;

/// Define the implementation of the pallet, like helper functions.
#[pallet_section]
mod impls {
    impl<T: Config> Pallet<T> {
        // get a random 256.
        fn random_value(who: &T::AccountId) -> [u8; 16] {
            let nonce = frame_system::Pallet::<T>::account_nonce(&who);
            // let nonce_u32: u32 = nonce as u32;
            // generate a random value based on account and its nonce
            let nonce_u32: u32 = TryInto::try_into(nonce).ok().expect("nonce is u64; qed");
            let a: BlockNumberFor<T> = TryFrom::try_from(nonce_u32)
                .ok()
                .expect("nonce is u32; qed");
            let payload=(
                T::Randomness::random_seed(),
                a,
                <frame_system::Pallet<T>>::extrinsic_index(),
            );
            let hash =payload.using_encoded(sp_io::hashing::blake2_128);
            hash
        }

        // breed on kitty based on both paraent kitties
        fn breed_kitty(who: &T::AccountId, kitty_1: [u8; 16], kitty_2: [u8; 16]) -> [u8; 16] {
            let selector = Self::random_value(&who);

            let mut data = [0u8; 16];
            for i in 0..kitty_1.len() {
                // 0 choose kitty2, and 1 choose kitty1
                data[i] = (kitty_1[i] & selector[i]) | (kitty_2[i] & !selector[i]);
            }
            data
        }
        fn create_with_stake(owner:&T::AccountId,v: [u8; 16])-> DispatchResult{
            // 从存储中获取
            let kitty_id=NextKittyId::<T>::get();
            // 与balance交互
            // 获取kitty的stake
            let stake=T::KittyStake::get();
            T::Currency::reserve(&owner,stake).map_err(|_|Error::<T>::NotEnoughForStaking)?;

            // 链上存储，用对象存储
            Kitties::<T>::insert(kitty_id,Kitty(v));
            KittyOwner::<T>::insert(kitty_id,owner.clone());
            // 下一个加1，可能溢出
            let next_kitty_id=kitty_id.checked_add(1).ok_or(Error::<T>::KittyIdOverflow)?;
            // 更新
            NextKittyId::<T>::put(next_kitty_id);
            
            Self::deposit_event(Event::KittyCreated { 
                creator:owner.clone(),
                index:kitty_id,
                data:v,
             });
             Ok(())
        }
    }
}
