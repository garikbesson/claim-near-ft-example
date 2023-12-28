use near_contract_standards::fungible_token::core::ext_ft_core::ext;
use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};
use near_sdk::json_types::U128;
use near_sdk::{env, near_bindgen, log, Gas, AccountId, Promise, PromiseError};

const FT_CONTRACT: &str = "tttest.tkn.near";
const CLAIMING_AMOUNT: U128 = U128(1);

const YOCTO_NEAR: u128 = 1;
const TGAS: u64 = 1_000_000_000_000;

// Define the contract structure
#[near_bindgen]
#[derive(BorshDeserialize, BorshSerialize)]
pub struct Contract {
  ft_contract: AccountId
}

impl Default for Contract {
  // The default trait with which to initialize the contract
  fn default() -> Self {
    Self {
      ft_contract: FT_CONTRACT.parse().unwrap()
    }
  }
}

// Implement the contract structure
#[near_bindgen]
impl Contract {
  #[payable]
  pub fn claim(&mut self) -> Promise {
    assert_eq!(env::attached_deposit(), 1, "Requires attached deposit of exactly 1 yoctoNEAR");

    let receiver_id = env::signer_account_id();

    let promise = ext(self.ft_contract.clone())
      .with_attached_deposit(YOCTO_NEAR)
      .ft_transfer(receiver_id, CLAIMING_AMOUNT, None);

    return promise.then( // Create a promise to callback query_greeting_callback
      Self::ext(env::current_account_id())
      .with_static_gas(Gas(30*TGAS))
      .external_call_callback()
    )
  }

  #[private] // Public - but only callable by env::current_account_id()
  pub fn external_call_callback(&self, #[callback_result] call_result: Result<(), PromiseError>) {
    // Check if the promise succeeded
    if call_result.is_err() {
      log!("There was an error contacting external contract");
    }
  }
}
