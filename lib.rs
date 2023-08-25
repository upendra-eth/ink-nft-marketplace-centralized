#![cfg_attr(not(feature = "std"), no_std)]
#![feature(min_specialization)]

#[openbrush::contract]
pub mod marketplace_psp37 {
    use ink::{
        codegen::{EmitEvent, Env},
        env::balance,
        prelude::vec::Vec,
    };

    // imports from openbrush
    use openbrush::contracts::psp37::extensions::batch::*;
    use openbrush::contracts::psp37::extensions::burnable::*;
    use openbrush::contracts::psp37::extensions::metadata::*;
    use openbrush::contracts::psp37::extensions::mintable::*;
    use openbrush::storage::Mapping;
    use openbrush::traits::{Storage, String};

    #[ink(storage)]
    #[derive(Default, Storage)]
    pub struct Contract {
        #[storage_field]
        psp37: psp37::Data,
        #[storage_field]
        metadata: metadata::Data,

        // Fields of current contract
        /// mapping from token id to `token_uri`
        token_uris: Mapping<Id, String>,

        /// A unique identifier for the tokens which have been minted (and are therefore
        /// supported) by this contract.
        next_id: u32,
    }

    /// Event emitted when a token transfer occurs.
    #[ink(event)]
    #[ink(anonymous)]
    pub struct Transfer {
        #[ink(topic)]
        _from: Option<AccountId>,
        #[ink(topic)]
        _to: Option<AccountId>,
        #[ink(topic)]
        _id: Id,
        #[ink(topic)]
        _amount: Balance,
    }

    #[ink(event)]
    pub struct Amount {
        #[ink(topic)]
        _amount: Balance,
    }

    /// Event emitted when a token Batch transfer occurs.
    #[ink(event)]
    pub struct TransferBatch {
        #[ink(topic)]
        _from: Option<AccountId>,
        #[ink(topic)]
        _to: Option<AccountId>,
        #[ink(topic)]
        _ids_amounts: Vec<(Id, Balance)>,
    }

    /// Event emitted when a token approve occurs.
    #[ink(event)]
    #[ink(anonymous)]
    pub struct Approval {
        #[ink(topic)]
        _owner: AccountId,
        #[ink(topic)]
        _operator: AccountId,
        #[ink(topic)]
        _id: Option<Id>,
        #[ink(topic)]
        _value: Balance,
    }

    /// Event emitted when a set_token_uri occurs.
    #[ink(event)]
    pub struct SetTokenUri {
        #[ink(topic)]
        _id: Id,
        #[ink(topic)]
        _token_uri: String,
    }

    // Section contains default implementation without any modifications
    impl PSP37 for Contract {}
    impl PSP37Batch for Contract {}
    impl PSP37Burnable for Contract {
        #[ink(message)]
        fn burn(
            &mut self,
            from: AccountId,
            ids_amounts: Vec<(Id, Balance)>,
        ) -> Result<(), PSP37Error> {
            if from != self.env().caller() {
                return Err(PSP37Error::NotAllowed);
            }
            self._burn_from(from, ids_amounts)
        }
    }
    impl PSP37Mintable for Contract {
        /// only ids which has been created can be minted to increase supply
        #[ink(message)]
        fn mint(
            &mut self,
            to: AccountId,
            ids_amounts: Vec<(Id, Balance)>,
        ) -> Result<(), PSP37Error> {
            for (id, amount) in ids_amounts.clone() {
                if id >= Id::U32(self.next_id) {
                    return Err(PSP37Error::NotAllowed);
                }
            }
            self._mint_to(to, ids_amounts)
        }
    }
    impl PSP37Metadata for Contract {}

    impl psp37::Internal for Contract {
        fn _emit_transfer_event(
            &self,
            _from: Option<AccountId>,
            _to: Option<AccountId>,
            _id: Id,
            _amount: Balance,
        ) {
            self.env().emit_event(Transfer {
                _from,
                _to,
                _id,
                _amount,
            });

            self.env().emit_event(Amount { _amount });
        }
        fn _emit_transfer_batch_event(
            &self,
            _from: Option<AccountId>,
            _to: Option<AccountId>,
            _ids_amounts: Vec<(Id, Balance)>,
        ) {
            self.env().emit_event(TransferBatch {
                _from,
                _to,
                _ids_amounts,
            });
        }
        fn _emit_approval_event(
            &self,
            _owner: AccountId,
            _operator: AccountId,
            _id: Option<Id>,
            _value: Balance,
        ) {
            self.env().emit_event(Approval {
                _owner,
                _operator,
                _id,
                _value,
            });
        }
    }

    impl Contract {
        #[ink(constructor)]
        pub fn new() -> Self {
            let mut _instance = Self::default();
            _instance
        }

        pub fn _emit_set_token_uri_event(&self, _id: Id, _token_uri: String) {
            self.env().emit_event(SetTokenUri { _id, _token_uri });
        }

        fn set_token_uri(&mut self, id: Id, _token_uri: String) -> Result<(), PSP37Error> {
            self.token_uris.insert(&id, &_token_uri);
            self._emit_set_token_uri_event(id, _token_uri);

            Ok(())
        }

        #[ink(message)]
        pub fn get_token_uri(&self, id: Id) -> Option<String> {
            self.token_uris.get(&id)
        }

        pub fn remove_token_uri(&mut self, id: &Id) -> Result<(), PSP37Error> {
            self.token_uris.remove(&id);
            Ok(())
        }

        #[ink(message)]
        pub fn create_nft(
            &mut self,
            to: AccountId,
            _amounts: Balance,
            _token_uri: String,
        ) -> Result<(), PSP37Error> {
            self.set_token_uri(Id::U32(self.next_id), _token_uri);
            let mut ids_amounts: Vec<(Id, Balance)> = Vec::new();
            // Add tuples to the vector
            ids_amounts.push((Id::U32(self.next_id), _amounts));
            self._mint_to(to, ids_amounts);
            self.next_id += 1;
            Ok(())
        }
    }
}
