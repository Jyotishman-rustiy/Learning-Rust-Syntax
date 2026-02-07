

#[derive(Debug)]
struct BlockchainUser {
    username: String,
    public_key : String,
    balance: u64,
    active : bool,
}




pub fn demo(){
      println!("\n--- Rust Struct  ---\n");
        
    let  user1 = BlockchainUser{
    username:"Dipak_Chadda".to_string(), 
    public_key:"Fx505Dk3s20sfnytw733".to_string(),
    balance:300, 
    active:true};


    println!("The BlockChain User is : {:#?}", user1);


    println!("\n-------------------------------------\n");
}