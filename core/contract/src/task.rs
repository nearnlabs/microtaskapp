use crate::Contract;
use crate::ContractExt;

use near_sdk::serde::Serialize;
use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};
use near_sdk::{env, log, near_bindgen, AccountId, Promise, Balance};
use near_sdk::json_types::U128;


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
  #[payable] 
  pub fn setup_task(&mut self, task_payment: u128) -> u128 {
    // Any potential client can set up a microtask
    let client: AccountId = env::predecessor_account_id();
    let setup_fee: Balance = env::attached_deposit();

    

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

    
    return new_task_id;
  }

  #[payable] 
  pub fn apply_for_task(&mut self, task_id: u128) -> bool {
    // An eligible user can apply for said task
    let applicant: AccountId = env::predecessor_account_id();

    assert!(&applicant != self.clients.get(&task_id).unwrap(), "You CANNOT apply for your own task!");
    
    let application_fee: Balance = env::attached_deposit();


    assert!(application_fee > STORAGE_COST, "Attach at least {} yoctoNEAR", STORAGE_COST);
    
    
    
    
    let mut applicant_vec = self.applicants.get(&task_id).unwrap().clone();

    

    applicant_vec.push(applicant.clone());

    self.applicants.remove(&task_id);
    self.applicants.insert(task_id, (*applicant_vec).to_vec());
    
    
    log!("Thank you for applying!");
    
    

    
    return true;
  }

  #[payable] 
  pub fn assign_task(&mut self, task_id: u128, assignee: AccountId) -> bool {
    // Used by the client to assign the microtask to their preferred applicant. Potential fee
    // for the task is moved from the client to the smart contract
    let caller: AccountId = env::predecessor_account_id();
    assert!(&caller == self.clients.get(&task_id).unwrap());

    let pre_payment: Balance = env::attached_deposit();

    

    assert!(&pre_payment > self.quotes.get(&task_id).unwrap(), "Not enough to cover the quoted price: {}", self.quotes.get(&task_id).unwrap());
    
    self.assignees.insert(task_id, assignee.clone());

    log!("Thank you for choosing {} as your assignee", assignee.clone());
    
    return true;
  }

  #[payable] 
  pub fn client_review_task(&mut self, task_id: u128, judgement: bool) -> bool {
    // Client review given in the form of a true (task completed satisfactorily) or
    // false (task not completed) flag
    let caller: AccountId = env::predecessor_account_id();
    assert!(&caller == self.clients.get(&task_id).unwrap());

    //let pre_payment: Balance = env::attached_deposit();


    
    self.client_review.insert(task_id, judgement.clone());

    log!("Thank you for reviewing Task ID: {}", task_id.clone());
    
    return true;
  }

  #[payable] 
  pub fn assignee_challenge_task(&mut self, task_id: u128) -> bool {
    // Provision for the assignee to challenge the client review. A panel of validators
    // decide in such cases if the client judgement is to be overruled.
    let caller: AccountId = env::predecessor_account_id();
    assert!(&caller == self.assignees.get(&task_id).unwrap());

    //let pre_payment: Balance = env::attached_deposit();

    assert!(!self.client_review.get(&task_id).unwrap());
    
    self.assignee_challenge.insert(task_id, true);

    log!("You challenge for Task ID: {} has been received. You will be notified shortly.", task_id.clone());
    
    return true;
  }

  #[payable] 
  pub fn validate_task(&mut self, task_id: u128, judgement: bool) -> bool {
    // The app team deciding to either give the payment to the assignee or refund the client
    // depending on client review, assignee challenge, and the validators' feedback.
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