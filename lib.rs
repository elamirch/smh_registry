#![cfg_attr(not(feature = "std"), no_std, no_main)]

#[ink::contract]
mod smh_registry {
    use ink::storage::Mapping;
    use ink::prelude::string::String;

    #[ink(event)]
    pub struct DomainRegistered {
        #[ink(topic)]
        domain: String,
        #[ink(topic)]
        contract_address: AccountId,
    }

    #[ink(event)]
    pub struct DomainExtentionCreated {
        #[ink(topic)]
        extention: String,
    }

    #[ink(storage)]
    #[derive(Default)]
    pub struct SmhRegistry {
        domains: Mapping<String, AccountId>,
        extention: String
    }

    impl SmhRegistry {
        #[ink(constructor)]
        pub fn new(extention: String) -> Self {
            let instance = Self {
                domains: Mapping::default(),
                extention: extention.clone(),
            };
            instance.env().emit_event(DomainExtentionCreated {
                extention
            });
            instance
        }

        #[ink(message)]
        pub fn register_if_free(&mut self, domain: String, contract_address: AccountId) -> bool {
            if self.domains.get(&domain).is_some() {
                return false
            }
            self.domains.insert(&domain, &contract_address);
            self.env().emit_event(DomainRegistered {
                domain,
                contract_address,
            });
            true
        }
    }
}