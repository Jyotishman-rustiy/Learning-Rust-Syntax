

pub fn basic_if_else(){
    let transaction_amount:i32 = 100;

    if transaction_amount >0 {
        println!("Transaction is Valid");


    }else if  transaction_amount>0 {
        println!("Invalid Transaction : Negative Amount");
    }else {
        println!("Transaction amount is zero : No transfer");

    }
}

pub fn match_example(day: u8) {
    let task = match day {
        1 => "Block production",
        2 => "Transaction validation",
        3 => "State pruning",
        4 => "Snapshot generation",
        5 => "Reward distribution",
        6 => "Governance voting",
        7 => "Network maintenance",
        _ => "Invalid day task",
    };

    println!("Day {} task: {}", day, task);
}

pub fn while_loop_example() {
    let mut pending_transactions = 0;
    while pending_transactions <5 {
        println!(
            "Processing Transaction number : {}",
            pending_transactions +1
        );
        pending_transactions +=1;
    }
    println!("All transaction processed")
}

pub fn for_loop_examples(){
    let staking_rewards =[10,20,30,40,50];

    for i in staking_rewards.iter(){
        println!("Validator  reward {}", i)
    }
     for block in 1..5{
        println!("Validator  reward {}", block)
    }
}

pub fn infinite_loop_examples(){
    let mut attempts =0;
    loop{
        println!("Checking blockchain state... attemptss {}", attempts +1);
        attempts +=1;

        if attempts ==3{
            println!("Breaking the loop after 3 attempts");
            break;
        }
    }
}

pub fn match_pattern_example(number:i32){
    match number {
        1 => println!("Executing token transfer"),
        2|3|5|4 =>  println!("Executing a prime validator operation"),
        10..=21 => println!("Performing governance action of block of {} ", number),
        _=> println!("Unrecongnised operation "),
    }

}

pub fn let_if_example(repetation_score : i32) {
    let reputation_level : &str = if repetation_score >= 90{
        "Higher Reputation"
    }else if repetation_score>= 70 {
        "Good Reputation"
    }else if repetation_score>=60 {
        "Average reputation"
    }else {
        "Poor Repuation"
    };

    println!("Repuation Score : {} , Repuation Level : {}" , repetation_score, reputation_level);
}


pub fn match_return_example(status_code:i32)-> &'static str{
match  status_code {
    200 => "Transaction Successfull",
     404 => "Transaction Not Found",
      500 => "Blockchain Error  ",
       _ => "Unknown Error",
   
}


}
pub fn demo(){
      println!("\n--- Rust Conntrol Flow  ---\n");
    basic_if_else();
    match_example(5);
    while_loop_example();
    for_loop_examples();
    infinite_loop_examples();

    match_pattern_example(20);
    let_if_example(80);
   let status_message =  match_return_example(500);
    println!("Status Message : {}", status_message);
      println!("\n-------------------------------------\n");

}