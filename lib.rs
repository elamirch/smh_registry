#![cfg_attr(not(feature = "std"), no_std, no_main)]

#[ink::contract]
mod smh_registry {
    use ink::storage::Mapping;
    use ink::prelude::string::String;

    #[ink(storage)]
    #[derive(Default)]
    pub struct SmhRegistry {
        domains: Mapping<String, bool>,
    }

    impl SmhRegistry {
        #[ink(constructor)]
        pub fn new() -> Self {
            Self { domains: Mapping::default() }
        }

        #[ink(message)]
        pub fn register_if_free(&mut self, domain: String) -> bool {
            if self.domains.get(&domain).is_some() {
                return false
            }
            self.domains.insert(&domain, &true);
            true
        }
    }
}