#![cfg_attr(not(feature = "std"), no_std)]

#[ink::contract]
mod transfer_contract {

    #[ink(storage)]
    pub struct TransferContract {
        multisig_wallet_address: AccountId,
    }

    impl Default for TransferContract {
        fn default() -> Self {
            TransferContract {
                multisig_wallet_address: [0; 32].into(),
            }
        }
    }

    impl TransferContract {
        #[ink(constructor)]
        pub fn new(multisig_wallet_address: AccountId) -> Self {
            let mut instance = Self::default();
            instance.multisig_wallet_address = multisig_wallet_address;
            instance
        }

        #[ink(message)]
        pub fn transfer_value(&mut self, nft_price: u64) {
            let transfer_value = (nft_price * 1) / 100;
            self.env()
                .transfer(self.multisig_wallet_address, transfer_value as u128)
                .unwrap_or_default();
        }
    }
}
