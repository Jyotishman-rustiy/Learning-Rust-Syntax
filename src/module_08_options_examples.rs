
pub  fn options_basic_examples(){
    let user_balance : Option<u64> = Some(1000);
    let user_address : Option<&str> = Some("Skm7729smaf226sh6s6aopc");

    // let no_balance :Option<u64> = None;
    

    match user_balance {
        Some(balance )=> println!("User has a balance of : {} tokens", balance),
        None => println!("No balance found for this account "),
    }
    

    match  user_address {
        Some(address)=> println!("Found blockchain address {}", address),
        None => println!("No address found")
    }
}

pub fn if_let_example(){
    let some_balance: Option<u64> = Some(1000);

    if let Some(balance)= some_balance {
        println!("Using `if let` : User has a balance of {} tokens ", balance);
    }else {
        println!("No balance found for the user")
    }
}

pub fn unwrap_example(){
    let user_balance: Option<u64> = Some(1000);
   
    // let no_balance: Option<u64> = None;
    let balance:u64 =  user_balance.unwrap();
    println!("Unwrapped balance {}", balance);

}

pub fn unwrap_or_example(){
    let no_balance :Option<u64>= None;

    let balance = no_balance.unwrap_or(0);
    println!("No balance found , so using default : {} " , balance);
}



pub fn demo(){
options_basic_examples();
if_let_example();
unwrap_example();
unwrap_or_example();
}