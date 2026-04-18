#![no_std]
use soroban_sdk::{contract, contractimpl, contracttype, Address, Env, String};

#[contracttype]
pub enum DataKey {
    Status(Address),
}

#[contract]
pub struct StatusContract;

#[contractimpl]
impl StatusContract {
    /// Sets a status for the caller.
    pub fn set_status(env: Env, user: Address, text: String) {
        // Require authorization from the user
        user.require_auth();

        // Store the text
        let key = DataKey::Status(user.clone());
        env.storage().persistent().set(&key, &text);
    }

    /// Retrieves the status for a user.
    pub fn get_status(env: Env, user: Address) -> String {
        let key = DataKey::Status(user);
        env.storage().persistent().get(&key).unwrap_or(String::from_str(&env, "No status yet"))
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use soroban_sdk::{testutils::Address as _, Address, Env, String};

    #[test]
    fn test() {
        let env = Env::default();
        env.mock_all_auths();
        
        let contract_id = env.register_contract(None, StatusContract);
        let client = StatusContractClient::new(&env, &contract_id);

        let user = Address::generate(&env);
        let text = String::from_str(&env, "Hello Web3!");

        client.set_status(&user, &text);
        
        let fetched_text = client.get_status(&user);
        assert_eq!(fetched_text, text);
    }
}
