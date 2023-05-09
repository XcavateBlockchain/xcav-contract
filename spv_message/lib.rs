#![cfg_attr(not(feature = "std"), no_std)]

#[ink::contract]
mod spv_message {
    use ink::{
        prelude::{string::String, vec::Vec},
        storage::Mapping,
    };

    #[derive(scale::Decode, scale::Encode)]
    #[cfg_attr(
        feature = "std",
        derive(scale_info::TypeInfo, ink::storage::traits::StorageLayout)
    )]
    pub struct Message {
        user: AccountId,
        message: String,
    }

    #[derive(scale::Decode, scale::Encode)]
    #[cfg_attr(feature = "std", derive(scale_info::TypeInfo))]
    pub enum SpvMessageError {
        AllNftsSoldAlready,
        SPVHasntCreatedYet,
    }

    #[ink(storage)]
    #[derive(Default)]
    pub struct SpvMessage {
        total_nft: i32,
        sold_nft: i32,
        nfts_owners: Vec<AccountId>,
        message_id: i32,
        message: Mapping<i32, Message>,
    }

    impl SpvMessage {
        #[ink(constructor)]
        pub fn new() -> Self {
            let instance = Self::default();
            instance
        }

        #[ink(message)]
        pub fn update_sold_nft(&mut self) -> Result<(), SpvMessageError> {
            if self.total_nft < 100 {
                self.sold_nft += 1;
                Ok(())
            } else {
                return Err(SpvMessageError::AllNftsSoldAlready);
            }
        }

        #[ink(message)]
        pub fn add_nfts_owners(&mut self, nft_owner: AccountId) -> Result<(), SpvMessageError> {
            if self.nfts_owners.len() < 100 {
                self.nfts_owners.push(nft_owner);
                Ok(())
            } else {
                return Err(SpvMessageError::AllNftsSoldAlready);
            }
        }

        #[ink(message)]
        pub fn send_message(&mut self) -> Result<(), SpvMessageError> {
            if self.sold_nft == self.total_nft {
                for owner in self.nfts_owners.clone() {
                    let message = Message {
                        user: owner,
                        message: String::from("SPV has created successfully"),
                    };

                    self.message.insert(self.message_id, &message);
                    self.message_id += 1;
                }
            } else {
                return Err(SpvMessageError::SPVHasntCreatedYet);
            }
            Ok(())
        }
    }
}
