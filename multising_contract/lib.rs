#![cfg_attr(not(feature = "std"), no_std)]

#[ink::contract]
mod multising_contract {
    use ink::prelude::vec::Vec;
    use ink::storage::Mapping;
    pub type TransactionId = i32;

    #[derive(scale::Decode, scale::Encode)]
    #[cfg_attr(
        feature = "std",
        derive(scale_info::TypeInfo, ink::storage::traits::StorageLayout)
    )]
    pub struct WalletTransaction {
        nft_owners: Vec<AccountId>,
    }

    #[derive(scale::Decode, scale::Encode)]
    #[cfg_attr(
        feature = "std",
        derive(scale_info::TypeInfo, ink::storage::traits::StorageLayout)
    )]
    pub enum MultisingContractError {
        AllNftsNotSold,
    }

    #[ink(storage)]
    pub struct MultisingContract {
        deployer: AccountId,
        total_nfts: i32,
        sold_nfts: i32,
        is_all_nft_sold: bool,
        transaction_id: TransactionId,
        wallet_transactions: Mapping<TransactionId, WalletTransaction>,
    }

    impl Default for MultisingContract {
        fn default() -> MultisingContract {
            Self {
                deployer: [0; 32].into(),
                total_nfts: Default::default(),
                sold_nfts: Default::default(),
                is_all_nft_sold: Default::default(),
                transaction_id: Default::default(),
                wallet_transactions: Mapping::default(),
            }
        }
    }

    impl MultisingContract {
        #[ink(constructor)]
        pub fn new(total_nfts: i32) -> Self {
            let mut instance = Self::default();
            instance.total_nfts = total_nfts;
            instance
        }

        #[ink(message)]
        pub fn update_is_sold(&mut self) -> Result<(), MultisingContractError> {
            if self.total_nfts == self.sold_nfts {
                self.is_all_nft_sold = true;
                Ok(())
            } else {
                return Err(MultisingContractError::AllNftsNotSold);
            }
        }

        #[ink(message)]
        pub fn add_nft_owner_to_wallet(&mut self, nft_owner: AccountId) {
            let mut nft_owners = Vec::new();
            nft_owners.push(nft_owner);

            let w_transaction = WalletTransaction { nft_owners };
            self.wallet_transactions
                .insert(self.transaction_id, &w_transaction);
            self.transaction_id += 1;
            self.sold_nfts += 1;
        }
    }
}
