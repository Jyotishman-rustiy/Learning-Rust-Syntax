#[derive(Debug)]
enum TransactionType {
    Transfer,
    Mint,
    Burn,
    Stake,
}

enum  ContractEvent {
    ContractDeployed,
    ContractTerminated,
    TokenTransfer { from: String, to:String, amount: u64},
    OracleUpdate { price : f64 }
}


enum  TransactionStatus {
    Pending, 
    Confirmed(u32),
    Failed(String)
}


impl  TransactionStatus {
    fn display_status(&self) {
        match  self {
            TransactionStatus::Pending => println!("Tranaxtion failed"),
              TransactionStatus::Confirmed(block) => println!("Tranaxtion Confirmed in block {} ", block),
          TransactionStatus::Failed(error) => println!("Tranaxtion failed due to {}", error)
        }
        }
        
        }
    

enum  ContractLifeCycle {
    Initialization,
    Active {participants:u32},
    Paused ,
    Terminated,
}


impl  ContractLifeCycle {
    fn display_state(&self){
        match self {
            ContractLifeCycle::Initialization => println!("Smart Contract is being init"),
            ContractLifeCycle::Active { participants }=>{println!("Contract is active with {} participants .",participants)}
            ContractLifeCycle::Paused => println!("Contract is currently paused"),
            ContractLifeCycle::Terminated => println!("Contract lifecycle is terminated"),
 
        }
    }
}



pub  fn demo(){
     println!("\n--- Rust Enum Demo ---\n");

    let tx_type = TransactionType::Transfer;
    match tx_type {
        TransactionType::Transfer => println!("Processing a Token Transfer"),
        TransactionType::Burn => println!("Burning New tokens"),
        TransactionType::Mint => println!("Minting New tokens"),
        TransactionType::Stake =>  println!("Staking New tokens"),
    }

    println!("Tx_type is {:#?}", tx_type);



    let transfer_event = ContractEvent::TokenTransfer { 
        from: String::from("OxFMsh4smtSkm22"), 
        to: String::from("Sxmo33&sbMsx2291"), 
        amount: 400
    };

    let oracle_event = ContractEvent::OracleUpdate { price: 300.23 };


    match transfer_event{
        ContractEvent::ContractDeployed => println!("Smart Contract Deployed"),
        ContractEvent::ContractTerminated => println!("Smart contract terminated"),
        ContractEvent::TokenTransfer { from, to, amount } => {
            println!("Transfer of {} tokens from {}  to {} .", amount, from, to)

        }
        ContractEvent::OracleUpdate { price }=>{
            println!("Oracle is Updated by {}", price)
        }
    }


    match oracle_event {
    ContractEvent::OracleUpdate { price } => {
        println!("Oracle updated with new price: {}", price);
    }
    _ => {
        println!("Not an oracle-related event");
    }
}




let tx1 = TransactionStatus::Pending;
let tx2 = TransactionStatus::Confirmed(200);
let tx3  = TransactionStatus::Failed("Invalid Signature".to_string());


tx1.display_status();
tx2.display_status();
tx3.display_status();



let contract = ContractLifeCycle::Active { participants: 100 };
let paused_contract = ContractLifeCycle::Paused;
let terminated_contract = ContractLifeCycle::Terminated;


contract.display_state();
paused_contract.display_state();
terminated_contract.display_state();



    println!("\n-------------------------------------\n");

}