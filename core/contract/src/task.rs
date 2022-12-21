use crate::Contract;
use crate::ContractExt;

use near_sdk::serde::Serialize;
use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};
use near_sdk::{env, log, near_bindgen, AccountId, Promise, Balance};
use near_sdk::json_types::U128;
//use phpify::array::array_unshift;

pub const STORAGE_COST: u128 = 1_000_000_000_000_000_000_000;

#[near_bindgen]
#[derive(BorshDeserialize, BorshSerialize, Serialize)]
#[serde(crate = "near_sdk::serde")]
pub struct Task {
  pub account_id: AccountId, 
  pub payable_amount: U128,
  pub task_id: u128,
  
}

#[near_bindgen]
impl Contract {
  #[payable] // Public - People can attach money
  pub fn setup_task(&mut self, task_payment: u128) -> u128 {
    // Get who is calling the method and how much $NEAR they attached
    let client: AccountId = env::predecessor_account_id();
    let setup_fee: Balance = env::attached_deposit();

    //let mut donated_so_far = self.donations.get(&donor).unwrap_or(0);

    assert!(setup_fee > STORAGE_COST, "Attach at least {} yoctoNEAR", STORAGE_COST);
    assert!(task_payment > STORAGE_COST* 10, "Too Little Being Offered");
    
    
    let new_task_id: u128 = self.get_task_count() + 1;
    self.task_count = new_task_id;
    self.quotes.insert(new_task_id, task_payment);
    self.clients.insert(new_task_id, client.clone());
    self.applicants.insert(new_task_id, Vec::new());
    
    log!("Thank you {} for creating a task worth {}!", client.clone(), task_payment);
    
    log!("Showing first quote: {}", self.quotes.get(&new_task_id).unwrap());

    log!("Showing first client: {}", self.clients.get(&new_task_id).unwrap());

    // Return the total amount donated so far
    return new_task_id;
  }

  #[payable] // Public - People can attach money
  pub fn apply_for_task(&mut self, task_id: u128) -> bool {
    // Get who is calling the method and how much $NEAR they attached
    let applicant: AccountId = env::predecessor_account_id();

    assert!(&applicant != self.clients.get(&task_id).unwrap(), "You CANNOT apply for your own task!");
    
    let application_fee: Balance = env::attached_deposit();

    //let mut donated_so_far = self.donations.get(&donor).unwrap_or(0);

    assert!(application_fee > STORAGE_COST, "Attach at least {} yoctoNEAR", STORAGE_COST);
    
    
    
    
    let mut applicant_vec = self.applicants.get(&task_id).unwrap().clone();

    //array_unshift(&mut applicant_vec, &applicant);

    applicant_vec.push(applicant.clone());

    self.applicants.remove(&task_id);
    self.applicants.insert(task_id, (*applicant_vec).to_vec());
    
    
    log!("Thank you for applying!");
    
    

    // Return the total amount donated so far
    return true;
  }

  #[payable] // Public - People can attach money
  pub fn assign_task(&mut self, task_id: u128, assignee: AccountId) -> bool {
    // Get who is calling the method and how much $NEAR they attached
    let caller: AccountId = env::predecessor_account_id();
    assert!(&caller == self.clients.get(&task_id).unwrap());

    let pre_payment: Balance = env::attached_deposit();

    //let mut donated_so_far = self.donations.get(&donor).unwrap_or(0);

    assert!(&pre_payment > self.quotes.get(&task_id).unwrap(), "Not enough to cover the quoted price: {}", self.quotes.get(&task_id).unwrap());
    
    self.assignees.insert(task_id, assignee.clone());

    log!("Thank you for choosing {} as your assignee", assignee.clone());
    
    return true;
  }

  #[payable] // Public - People can attach money
  pub fn client_review_task(&mut self, task_id: u128, judgement: bool) -> bool {
    // Get who is calling the method and how much $NEAR they attached
    let caller: AccountId = env::predecessor_account_id();
    assert!(&caller == self.clients.get(&task_id).unwrap());

    //let pre_payment: Balance = env::attached_deposit();


    
    self.client_review.insert(task_id, judgement.clone());

    log!("Thank you for reviewing Task ID: {}", task_id.clone());
    
    return true;
  }

  #[payable] // Public - People can attach money
  pub fn assignee_challenge_task(&mut self, task_id: u128) -> bool {
    // Get who is calling the method and how much $NEAR they attached
    let caller: AccountId = env::predecessor_account_id();
    assert!(&caller == self.assignees.get(&task_id).unwrap());

    //let pre_payment: Balance = env::attached_deposit();

    assert!(!self.client_review.get(&task_id).unwrap());
    
    self.assignee_challenge.insert(task_id, true);

    log!("You challenge for Task ID: {} has been received. You will be notified shortly.", task_id.clone());
    
    return true;
  }

  #[payable] // Public - People can attach money
  pub fn validate_task(&mut self, task_id: u128, judgement: bool) -> bool {
    // Get who is calling the method and how much $NEAR they attached
    let caller: AccountId = env::predecessor_account_id();
    //let setup_fee: Balance = env::attached_deposit();

    assert!(caller == env::current_account_id(),"You are not authorized to call this method");

    if judgement{
        Promise::new(self.assignees.get(&task_id).unwrap().clone()).transfer(*self.quotes.get(&task_id).unwrap());
        let _r1 = self.clients.remove(&task_id);
        let _r2 = self.quotes.remove(&task_id);
        let _r3 = self.assignees.remove(&task_id);
    }
    else {
        Promise::new(self.clients.get(&task_id).unwrap().clone()).transfer(*self.quotes.get(&task_id).unwrap());
        let _r1 = self.clients.remove(&task_id);
        let _r2 = self.quotes.remove(&task_id);
        let _r3 = self.assignees.remove(&task_id);
    }
    
    
    log!("Task Closed successfully with status: {}", judgement);
    
    
    return true;
  }

  // Public - get donation by account ID
  pub fn get_client_for_task(&self, task_id: u128) -> AccountId {
    return self.clients.get(&task_id).unwrap().clone();
  }

  pub fn get_assignee_for_task(&self, task_id: u128) -> AccountId {
    return self.assignees.get(&task_id).unwrap().clone();
  }

  pub fn get_quote_for_task(&self, task_id: u128) -> u128 {
    return self.quotes.get(&task_id).unwrap().clone();
  }

  pub fn get_applicants_for_task(&self, task_id: u128) -> Vec<AccountId> {
    return self.applicants.get(&task_id).unwrap().clone();
  }

}