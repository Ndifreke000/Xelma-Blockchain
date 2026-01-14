#![no_std]
use soroban_sdk::{contract, contractimpl, contracttype, Address, Env};

/// Storage keys for organizing data in the contract
/// Think of these as "labels" for different storage compartments
/// 
/// The #[contracttype] attribute tells Soroban this can be stored in the contract
#[contracttype]
#[derive(Clone)]
pub enum DataKey {
    /// Stores the balance for a specific user address
    Balance(Address),
    /// Stores the admin address (the person who can create rounds)
    Admin,
    /// Stores the currently active round
    ActiveRound,
}

/// Represents a prediction round
/// This stores all the information about an active betting round
#[contracttype]
#[derive(Clone, Debug, PartialEq)]
pub struct Round {
    /// The starting price of XLM when the round begins (in stroops)
    pub price_start: u128,
    /// The ledger number when this round ends
    /// Ledgers are like blocks in blockchain - they increment every ~5 seconds
    pub end_ledger: u32,
    /// Total vXLM in the "UP" pool (people betting price will go up)
    pub pool_up: i128,
    /// Total vXLM in the "DOWN" pool (people betting price will go down)
    pub pool_down: i128,
}

/// The main contract structure
/// This represents your vXLM (virtual XLM) token contract
#[contract]
pub struct VirtualTokenContract;

#[contractimpl]
impl VirtualTokenContract {
    /// Initializes the contract by setting the admin
    /// This should be called once when deploying the contract
    /// 
    /// # Parameters
    /// * `env` - The contract environment
    /// * `admin` - The address that will have admin privileges
    /// 
    /// # Security
    /// Only the admin can create rounds, so choose this address carefully!
    pub fn initialize(env: Env, admin: Address) {
        // Ensure admin authorizes this initialization
        admin.require_auth();
        
        // Check if admin is already set to prevent re-initialization
        if env.storage().persistent().has(&DataKey::Admin) {
            panic!("Contract already initialized");
        }
        
        // Store the admin address
        env.storage().persistent().set(&DataKey::Admin, &admin);
    }
    
    /// Creates a new prediction round
    /// Only the admin can call this function
    /// 
    /// # Parameters
    /// * `env` - The contract environment
    /// * `start_price` - The current XLM price in stroops (e.g., 1 XLM = 10,000,000 stroops)
    /// * `duration_ledgers` - How many ledgers (blocks) the round should last
    ///                        Example: 60 ledgers ≈ 5 minutes (since ledgers are ~5 seconds)
    /// 
    /// # How it works
    /// 1. Verifies the caller is the admin
    /// 2. Calculates when the round will end
    /// 3. Creates a new Round with empty betting pools
    /// 4. Stores it as the active round
    pub fn create_round(env: Env, start_price: u128, duration_ledgers: u32) {
        // Get the admin address from storage
        let admin: Address = env.storage()
            .persistent()
            .get(&DataKey::Admin)
            .expect("Admin not set - call initialize first");
        
        // Verify that the caller is the admin
        // This prevents random users from creating rounds
        admin.require_auth();
        
        // Get the current ledger number and calculate end ledger
        // Think of ledgers like block numbers - they keep incrementing
        let current_ledger = env.ledger().sequence();
        let end_ledger = current_ledger + duration_ledgers;
        
        // Create a new round with the given parameters
        let round = Round {
            price_start: start_price,
            end_ledger,
            pool_up: 0,    // No bets yet
            pool_down: 0,  // No bets yet
        };
        
        // Save the round as the active round
        env.storage().persistent().set(&DataKey::ActiveRound, &round);
    }
    
    /// Gets the currently active round
    /// 
    /// # Returns
    /// Option<Round> - Some(round) if there's an active round, None if not
    /// 
    /// # Use case
    /// Frontend can call this to display current round info to users
    pub fn get_active_round(env: Env) -> Option<Round> {
        env.storage().persistent().get(&DataKey::ActiveRound)
    }
    
    /// Gets the admin address
    /// 
    /// # Returns
    /// Option<Address> - Some(admin) if set, None if not initialized
    pub fn get_admin(env: Env) -> Option<Address> {
        env.storage().persistent().get(&DataKey::Admin)
    }
    
    /// Mints (creates) initial vXLM tokens for a user on their first interaction
    /// 
    /// # Parameters
    /// * `env` - The contract environment (provided by Soroban, gives access to storage, etc.)
    /// * `user` - The address of the user who will receive tokens
    /// 
    /// # How it works
    /// 1. Checks if user already has a balance
    /// 2. If not, gives them 1000 vXLM as a starting amount
    /// 3. Stores this balance in the contract's persistent storage
    pub fn mint_initial(env: Env, user: Address) -> i128 {
        // Verify that the user is authorized to call this function
        // This ensures only the user themselves can mint tokens for their account
        user.require_auth();
        
        // Create a storage key for this user's balance
        let key = DataKey::Balance(user.clone());
        
        // Check if the user already has a balance
        // get() returns an Option: Some(balance) if exists, None if not
        if let Some(existing_balance) = env.storage().persistent().get(&key) {
            // User already has tokens, return their existing balance
            return existing_balance;
        }
        
        // User is new! Give them 1000 vXLM as initial tokens
        // Note: We use 1000_0000000 because Stellar uses 7 decimal places (stroops)
        let initial_amount: i128 = 1000_0000000; // 1000 vXLM
        
        // Save the balance to persistent storage
        // This data will remain even after the transaction completes
        env.storage().persistent().set(&key, &initial_amount);
        
        // Return the newly minted amount
        initial_amount
    }
    
    /// Queries (reads) the current vXLM balance for a user
    /// 
    /// # Parameters
    /// * `env` - The contract environment
    /// * `user` - The address of the user whose balance we want to check
    /// 
    /// # Returns
    /// The user's balance as an i128 (128-bit integer)
    /// Returns 0 if the user has never received tokens
    pub fn balance(env: Env, user: Address) -> i128 {
        // Create the storage key for this user
        let key = DataKey::Balance(user);
        
        // Try to get the balance from storage
        // unwrap_or(0) means: if balance exists, use it; otherwise, return 0
        env.storage().persistent().get(&key).unwrap_or(0)
    }
    
    /// Internal helper function to update a user's balance
    /// The underscore prefix means this is a private/internal function
    /// 
    /// # Parameters
    /// * `env` - The contract environment
    /// * `user` - The address whose balance to update
    /// * `amount` - The new balance amount
    fn _set_balance(env: &Env, user: Address, amount: i128) {
        let key = DataKey::Balance(user);
        env.storage().persistent().set(&key, &amount);
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use soroban_sdk::{testutils::Address as _, Env};

    #[test]
    fn test_mint_initial() {
        // Create a test environment
        let env = Env::default();
        
        // Register our contract in the test environment
        // This deploys the contract to the test blockchain and returns its unique ID
        // Think of it as: installing your app on a test phone before you can use it
        // The () means we're not passing any initialization arguments
        let contract_id = env.register(VirtualTokenContract, ());
        
        // Create a client to interact with the contract
        let client = VirtualTokenContractClient::new(&env, &contract_id);
        
        // Generate a random test user address
        let user = Address::generate(&env);
        
        // Mock the authorization (in tests, we need to simulate user approval)
        env.mock_all_auths();
        
        // Call mint_initial for the user
        let balance = client.mint_initial(&user);
        
        // Verify the user received 1000 vXLM
        assert_eq!(balance, 1000_0000000);
        
        // Verify we can query the balance
        let queried_balance = client.balance(&user);
        assert_eq!(queried_balance, 1000_0000000);
    }
    
    #[test]
    fn test_mint_initial_only_once() {
        let env = Env::default();
        let contract_id = env.register(VirtualTokenContract, ());
        let client = VirtualTokenContractClient::new(&env, &contract_id);
        let user = Address::generate(&env);
        
        env.mock_all_auths();
        
        // First mint
        let first_mint = client.mint_initial(&user);
        assert_eq!(first_mint, 1000_0000000);
        
        // Try to mint again - should return existing balance, not mint more
        let second_mint = client.mint_initial(&user);
        assert_eq!(second_mint, 1000_0000000);
        
        // Balance should still be 1000, not 2000
        let balance = client.balance(&user);
        assert_eq!(balance, 1000_0000000);
    }
    
    #[test]
    fn test_balance_for_new_user() {
        let env = Env::default();
        let contract_id = env.register(VirtualTokenContract, ());
        let client = VirtualTokenContractClient::new(&env, &contract_id);
        let user = Address::generate(&env);
        
        // Query balance for a user who never minted
        let balance = client.balance(&user);
        
        // Should return 0
        assert_eq!(balance, 0);
    }
    
    #[test]
    fn test_initialize() {
        let env = Env::default();
        let contract_id = env.register(VirtualTokenContract, ());
        let client = VirtualTokenContractClient::new(&env, &contract_id);
        
        // Generate an admin address
        let admin = Address::generate(&env);
        
        env.mock_all_auths();
        
        // Initialize the contract
        client.initialize(&admin);
        
        // Verify admin is set
        let stored_admin = client.get_admin();
        assert_eq!(stored_admin, Some(admin));
    }
    
    #[test]
    #[should_panic(expected = "Contract already initialized")]
    fn test_initialize_twice_fails() {
        let env = Env::default();
        let contract_id = env.register(VirtualTokenContract, ());
        let client = VirtualTokenContractClient::new(&env, &contract_id);
        
        let admin = Address::generate(&env);
        
        env.mock_all_auths();
        
        // Initialize once
        client.initialize(&admin);
        
        // Try to initialize again - should panic
        client.initialize(&admin);
    }
    
    #[test]
    fn test_create_round() {
        let env = Env::default();
        let contract_id = env.register(VirtualTokenContract, ());
        let client = VirtualTokenContractClient::new(&env, &contract_id);
        
        // Set up admin
        let admin = Address::generate(&env);
        env.mock_all_auths();
        client.initialize(&admin);
        
        // Create a round
        let start_price: u128 = 1_5000000; // 1.5 XLM in stroops
        let duration: u32 = 60; // 60 ledgers
        
        client.create_round(&start_price, &duration);
        
        // Verify the round was created
        let round = client.get_active_round().expect("Round should exist");
        
        assert_eq!(round.price_start, start_price);
        assert_eq!(round.pool_up, 0);
        assert_eq!(round.pool_down, 0);
        
        // Verify end_ledger is set correctly (current ledger + duration)
        // Note: In tests, current ledger starts at 0
        assert_eq!(round.end_ledger, duration);
    }
    
    #[test]
    #[should_panic(expected = "Admin not set - call initialize first")]
    fn test_create_round_without_init_fails() {
        let env = Env::default();
        let contract_id = env.register(VirtualTokenContract, ());
        let client = VirtualTokenContractClient::new(&env, &contract_id);
        
        env.mock_all_auths();
        
        // Try to create round without initializing - should panic
        client.create_round(&1_0000000, &60);
    }
    
    #[test]
    fn test_get_active_round_when_none() {
        let env = Env::default();
        let contract_id = env.register(VirtualTokenContract, ());
        let client = VirtualTokenContractClient::new(&env, &contract_id);
        
        // No round created yet
        let round = client.get_active_round();
        
        assert_eq!(round, None);
    }
}
