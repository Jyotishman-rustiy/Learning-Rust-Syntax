use std::collections::HashMap;


pub  fn array_example(){
    let block_hashes:[u64; 5] = [10,20,22200,42200,2100299];
    println!("Block Hashes Array : {:?}", block_hashes);
    println!("First block hash: {} ", block_hashes[0]);
    println!("Last block hash : {} ", block_hashes[block_hashes.len() -1]);


    let slice = &block_hashes[1..4];
     let slice_22 = &block_hashes[1..=4];
    println!("The values from 1 to 4 are {:?}", slice);
    println!("The values from 1 to =4 are {:?}", slice_22)
}


pub fn vector_example() {
    let mut transaction_ids = vec!["tx1", "tx2", "tx322"];

    transaction_ids.push("kkss2211tsx");
      transaction_ids.push("tsx4422");

      println!("After adding transaction : {:?}", transaction_ids);


      transaction_ids.pop();
            println!("After removing transaction : {:?}", transaction_ids);


    for tx in &transaction_ids {
        println!("Transaction Ids : {}", tx);
    }


    if let Some(first_tx)= transaction_ids.get(0){
        println!("first transaction ID: {}", first_tx);

    }
}

pub fn tuple_example(){
    let user_info= ("Slavia", 30, 2.5);
    println!("User Info in Tuple : {:?}", user_info);

    println!("Username : {} ", user_info.0);
        println!("Age : {} ", user_info.1);
    println!("SOL Balance: {} ", user_info.2);

}


pub fn hashmaps(){
    let mut balance_of: HashMap<&str, u32> = HashMap::new();


    balance_of.insert("sjhdfjshdfknj1089573klndsklfnsijdfhdfjoshdf", 100);
    balance_of.insert("kajfmms8725389734520982", 200);  
      balance_of.insert("fmms872538ssww9734520982", 2020);


    println!("User Balances HashMap : {:#?}", balance_of);


    match balance_of.get("kajfmms8725389734520982") {
        Some(balance) => println!("Sage balance is : {}", balance),
        None => println!("No balance Found for this user")
    }

    for (user, balance) in &balance_of {
        println!("{}'s Balance  {}", user, balance);
    }



    balance_of.entry("sjhdfjshdfknj1089573klndsklfnsijdfhdfjoshdf").and_modify(|balance| *balance -= 50);
    println!("Updated Rage wallet Balance {:#?}" , balance_of);


    balance_of.remove("kajfmms8725389734520982");
    println!("After Removing the wallet : {:#?}", balance_of)


}


pub fn nested_data_structures_example(){
    let mut user_transaction: HashMap<&str, Vec<&str>> = HashMap::new();

    user_transaction.insert("8w74598kdmhfksjfh8297334mm", vec!["tx1", "tx2", "tx3"]);
        user_transaction.insert("FJzbMdYzCfhfvyNWbyVuf48NBJUCf3j3rQQ1h5WKKtiQ", vec!["tx3222q", "tx402hash", "tx39s2xnh9js"]);

        println!("User Transaction HashMap : {:#?} ", user_transaction);



        if let Some(transaction) = user_transaction.get("FJzbMdYzCfhfvyNWbyVuf48NBJUCf3j3rQQ1h5WKKtiQ"){
            println!("Users Transaction : {:#?}", transaction)
        }

        if let Some(transaction )= user_transaction.get_mut("FJzbMdYzCfhfvyNWbyVuf48NBJUCf3j3rQQ1h5WKKtiQ"){
            transaction.push("txlops789");
        }

        println!("User transaction : {:#?}", user_transaction)


}

pub fn demo(){
            println!("\n--- Data Structure Examples---\n");
            array_example();
            vector_example();
            tuple_example();
            hashmaps();
            nested_data_structures_example();
            println!("\n-------------------------------------\n");
}