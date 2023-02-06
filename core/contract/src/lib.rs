use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};
use near_sdk::{env, log, near_bindgen, AccountId};
//use near_sdk::collections::{UnorderedMap};
use std::collections::BTreeMap;

//use phpify::array::array_unshift;

mod task;

#[near_bindgen]
#[derive(BorshDeserialize, BorshSerialize)]
pub struct Contract {
  pub task_count: u128,
  pub clients: BTreeMap<u128, AccountId>,
  pub quotes: BTreeMap<u128, u128>,
  pub applicants: BTreeMap<u128, Vec<AccountId>>,
  pub assignees: BTreeMap<u128, AccountId>,
  pub client_review: BTreeMap<u128, bool>,
  pub assignee_challenge: BTreeMap<u128, bool>,
}

impl Default for Contract {
  fn default() -> Self {
    Self{
      task_count: 0,
      clients: BTreeMap::new(),
      quotes: BTreeMap::new(),
      applicants: BTreeMap::new(),
      assignees: BTreeMap::new(),
      client_review: BTreeMap::new(),
      assignee_challenge: BTreeMap::new(),
    }
  }
}

#[near_bindgen]
impl Contract {
  #[init]
  #[private] // Public - but only callable by env::current_account_id()
  pub fn init() -> Self {
    assert!(!env::state_exists(), "Already initialized");
    Self {
        task_count: 0,
        clients: BTreeMap::new(),
        quotes: BTreeMap::new(),
        applicants: BTreeMap::new(),
        assignees: BTreeMap::new(),
        client_review: BTreeMap::new(),
        assignee_challenge: BTreeMap::new(),
    }
  }

  // Public - client list getter
  pub fn get_client_list(&self) -> BTreeMap<u128, AccountId> {
    self.clients.clone()
  }

  // Public - quote list getter
  pub fn get_quote_list(&self) -> BTreeMap<u128, u128> {
    self.quotes.clone()
  }

  // Public - assignee list getter
  pub fn get_assignee_list(&self) -> BTreeMap<u128, AccountId> {
    self.assignees.clone()
  }

  pub fn get_task_count(&self) -> u128{
    self.task_count.clone()

  }
  
}

#[cfg(test)]
mod tests {
  use super::*;
  use near_sdk::testing_env;
  use near_sdk::test_utils::VMContextBuilder;
  use near_sdk::Balance;

  //const BENEFICIARY: &str = "beneficiary";
  const NEAR: u128 = 1000000000000000000000000;

  #[test]
  fn initializes() {
      let contract = Contract::init();
      assert_eq!(contract.task_count, 0);
  }

  #[test]
  fn check_quote() {
      let mut contract = Contract::init();

      // Make a donation
      set_context("client_a", 1*NEAR);
      contract.setup_task(NEAR*3);
      let first_quote = contract.get_quote_for_task(contract.task_count);

      log!("{} tasks", contract.task_count);
      // Check the donation was recorded correctly
      //assert_eq!(contract.task_count, 1);
      assert_eq!(first_quote, NEAR*3,"assertion failed");

      
  }

  #[test]
  #[should_panic]
  fn check_quote_too_less() {
      let mut contract = Contract::init();

      // Make a donation
      set_context("client_a", NEAR/1000);
      contract.setup_task(NEAR*3);
      let first_quote = contract.get_quote_for_task(contract.task_count);

      log!("{} tasks", contract.task_count);
      // Check the donation was recorded correctly
      //assert_eq!(contract.task_count, 1);
      assert_eq!(first_quote, NEAR*3,"assertion failed");

      
  }

  #[test]
  fn check_client() {
      let mut contract = Contract::init();

      // Make a donation
      set_context("client_a", 1*NEAR);
      contract.setup_task(NEAR*3);
      let first_client = contract.get_client_for_task(contract.task_count);

      log!("{} client", contract.get_client_for_task(contract.task_count));
      // Check the donation was recorded correctly
      //assert_eq!(contract.task_count, 1);
      assert_eq!(first_client, AccountId::new_unchecked("client_a".to_string()),"assertion failed 2");

      
  }

  #[test]
  fn check_task_count() {
      let mut contract = Contract::init();

      // Make a donation
      set_context("client_b", 1*NEAR);
      contract.setup_task(NEAR*2);
      //let first_quote = contract.get_quote_for_task(contract.task_count);

      log!("{} tasks", contract.task_count);
      // Check the donation was recorded correctly
      assert_eq!(contract.task_count, 1);
      //assert_eq!(first_quote, NEAR*3,"assertion failed");

      
  }

  #[test]
  fn check_application() {
      let mut contract = Contract::init();

      // Make a donation
      set_context("client_a", 1*NEAR);
      contract.setup_task(NEAR*5);
      //let first_quote = contract.get_quote_for_task(contract.task_count);

      set_context("client_b", 1*NEAR);
      let res = contract.apply_for_task(1);

      //log!("{} tasks", contract.task_count);
      // Check the donation was recorded correctly
      //assert_eq!(contract.task_count, 1);
      assert_eq!(true, res,"assertion failed 3");

      
  }


  #[test]
  #[should_panic]
  fn check_self_application() {
      let mut contract = Contract::init();

      // Make a donation
      set_context("client_a", 1*NEAR);
      contract.setup_task(NEAR*5);
      //let first_quote = contract.get_quote_for_task(contract.task_count);

      
      let res = contract.apply_for_task(1);

      //log!("{} tasks", contract.task_count);
      // Check the donation was recorded correctly
      //assert_eq!(contract.task_count, 1);
      assert_eq!(true, res,"assertion failed 3");

      
  }

  #[test]
  fn check_applicant() {
      let mut contract = Contract::init();

      // Make a donation
      set_context("client_a", 1*NEAR);
      contract.setup_task(NEAR*5);
      //let first_quote = contract.get_quote_for_task(contract.task_count);

      set_context("client_b", 1*NEAR);
      let _res = contract.apply_for_task(1);

      let applicant_list = contract.get_applicants_for_task(contract.task_count);

      //log!("{} tasks", contract.task_count);
      // Check the donation was recorded correctly
      //assert_eq!(contract.task_count, 1);
      assert_eq!(applicant_list[0], AccountId::new_unchecked("client_b".to_string()));

      
  }

  #[test]
  fn check_assignee() {
      let mut contract = Contract::init();

      // Make a donation
      set_context("client_a", 1*NEAR);
      contract.setup_task(NEAR*5);
      //let first_quote = contract.get_quote_for_task(contract.task_count);

      set_context("client_b", 1*NEAR);
      let _res1 = contract.apply_for_task(1);

      set_context("client_c", 1*NEAR);
      let _res2 = contract.apply_for_task(1);

      let applicant_list = contract.get_applicants_for_task(contract.task_count);
      let chosen_one = &applicant_list[1];
      set_context("client_a", 6*NEAR);
      let _res = contract.assign_task(1, chosen_one.clone());
      //log!("{} tasks", contract.task_count);
      // Check the donation was recorded correctly
      //assert_eq!(contract.task_count, 1);
      assert_eq!(chosen_one.clone(), AccountId::new_unchecked("client_c".to_string()), "Assertion Failed 5");

      
  }

  #[test]
  fn check_validation() {
      let mut contract = Contract::init();

      // Make a donation
      set_context("client_a", 1*NEAR);
      contract.setup_task(NEAR*5);
      //let first_quote = contract.get_quote_for_task(contract.task_count);

      set_context("client_b", 1*NEAR);
      let _res1 = contract.apply_for_task(1);

      set_context("client_c", 1*NEAR);
      let _res2 = contract.apply_for_task(1);

      let applicant_list = contract.get_applicants_for_task(contract.task_count);
      let chosen_one = &applicant_list[1];
      set_context("client_a", 6*NEAR);
      let _res3 = contract.assign_task(1, chosen_one.clone());
      //log!("{} tasks", contract.task_count);
      // Check the donation was recorded correctly
      //assert_eq!(contract.task_count, 1);
      let caller = env::current_account_id();
      set_context_2(caller, 10*NEAR);
      let res = contract.validate_task(1, true);

      assert_eq!(res, true, "Assertion Failed 6");

      
  }

  #[test]
  fn check_client_review() {
      let mut contract = Contract::init();

      // Make a donation
      set_context("client_a", 1*NEAR);
      contract.setup_task(NEAR*5);
      //let first_quote = contract.get_quote_for_task(contract.task_count);

      set_context("client_b", 1*NEAR);
      let _res1 = contract.apply_for_task(1);

      set_context("client_c", 1*NEAR);
      let _res2 = contract.apply_for_task(1);

      let applicant_list = contract.get_applicants_for_task(contract.task_count);
      let chosen_one = &applicant_list[1];
      set_context("client_a", 6*NEAR);
      let _res3 = contract.assign_task(1, chosen_one.clone());
      let res = contract.client_review_task(1, true);
      

      assert_eq!(res, true, "Assertion Failed 6");

      
  }

  #[test]
  fn check_assignee_challenge() {
      let mut contract = Contract::init();

      // Make a donation
      set_context("client_a", 1*NEAR);
      contract.setup_task(NEAR*5);
      //let first_quote = contract.get_quote_for_task(contract.task_count);

      set_context("client_b", 1*NEAR);
      let _res1 = contract.apply_for_task(1);

      set_context("client_c", 1*NEAR);
      let _res2 = contract.apply_for_task(1);

      let applicant_list = contract.get_applicants_for_task(contract.task_count);
      let chosen_one = &applicant_list[1];
      set_context("client_a", 6*NEAR);
      let _res3 = contract.assign_task(1, chosen_one.clone());
      let _res4 = contract.client_review_task(1, false);
      set_context("client_c", 1*NEAR);
      let res = contract.assignee_challenge_task(1);
      

      assert_eq!(res, true, "Assertion Failed 7");

      
  }

  // Auxiliar fn: create a mock context
  fn set_context(predecessor: &str, amount: Balance) {
    let mut builder = VMContextBuilder::new();
    builder.predecessor_account_id(predecessor.parse().unwrap());
    builder.attached_deposit(amount);

    testing_env!(builder.build());
  }

  fn set_context_2(predecessor: AccountId, amount: Balance) {
    let mut builder = VMContextBuilder::new();
    builder.predecessor_account_id(predecessor);
    builder.attached_deposit(amount);

    testing_env!(builder.build());
  }
}