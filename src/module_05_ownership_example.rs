
pub fn ownership_example(){
    let token_owner = String::from("Haresh");

    let new_token_owner = &token_owner;

    println!("The new Token Owner contains : {}", new_token_owner );
    println!("If ownership is not given to one the scope will be gone from token_owner");
    println!("\n-------------------------------------\n");


}

pub fn cloning_example(){
    let name = String::from("Namrata");

    let clone_name = name.clone();

    println!("both values have same values now 1st : {} , 2nd : {}", name, clone_name);



}


pub fn borrowing_example(){
    let contract_state = String::from("Contract Active ");
    print_borrow(&contract_state);
    println!("\n After borrowing Contract State :{} ", contract_state)


}

pub fn print_borrow(contract: &String){
    println!("\nBorrow Contract State : {}", contract)
}

pub fn mutable_borrowing_example(){
    let mut contract_state_mutable = String::from("Contract Pending");

    modify_state(&mut contract_state_mutable);

        println!("After modifying , Contract State : {}", contract_state_mutable )

}

pub fn modify_state(contract:  &mut String){
    contract.push_str("and now active ");

}

pub fn demo(){
    println!("\n--- Rust Ownership  ---\n");
    ownership_example();
   

    cloning_example();
  println!("\n-------------------------------------\n");

    borrowing_example();
  println!("\n-------------------------------------\n");

    mutable_borrowing_example();
  
  println!("\n-------------------------------------\n");

  println!("Read Dangling Reference ------")
}