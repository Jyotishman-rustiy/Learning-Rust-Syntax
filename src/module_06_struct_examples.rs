#[derive(Debug)]
struct BlockchainUser {
    username: String,
    public_key: String,
    balance: u64,
    active: bool,
}

impl BlockchainUser {
    fn new(username: &str, public_key: &str, balance: u64) -> Self {
        Self {
            username: username.to_string(),
            public_key: public_key.to_string(),
            balance,
            active: true,
        }
    }

    fn display_info(&self) {
        println!(
            "Name: {}, Public Key: {}, Balance: {}, Active: {}",
            self.username, self.public_key, self.balance, self.active
        );
    }

    fn update_public_key(&mut self, public_key: &str) {
        self.public_key = public_key.to_string();
    }

    fn deactivate(&mut self) {
        self.active = false;
    }

    fn add_balance(&mut self, amount: u64) {
        self.balance += amount;
        println!("Added {} tokens to user: {}", amount, self.username);
    }
}

#[derive(Debug)]
struct  Transaction(u64, String);


#[derive(Debug)]
struct  Block{
    index : u32 ,
    miner : BlockchainUser,
    transaction_count : u32
}

pub fn demo() {
    println!("\n--- Rust Struct Demo ---\n");

    let mut user1 = BlockchainUser {
        username: "Dipak_Chadda".to_string(),
        public_key: "Fx505Dk3s20sfnytw733".to_string(),
        balance: 300,
        active: true,
    };

    println!("The Blockchain User is:\n{:#?}", user1);

    let mut new_user = BlockchainUser::new(
        "jyotishman",
        "OXo920sjks3s20fnatq3222",
        300,
    );

    new_user.display_info();
    new_user.add_balance(220);
    new_user.update_public_key("OXo920sjks3s20fnatq3222sexxy");

    user1.deactivate();

    println!("\n--- Updated Users ---\n");
    new_user.display_info();
    user1.display_info();


 println!("\n--- Rust Transaction Demo ---\n");

    let tx1 = Transaction(400, String::from("0xB23Dop")); 
    let tx2 = Transaction(400, String::from("0xB23Posxp")); 

    println!("Transaction 1 : {:#?}", tx1);
    println!("Transaction 2 : {}  tokens sent to {}", tx2.0, tx2.1);


    println!("\n--- Rust Struct on Struct  Demo ---\n");

    let mined_block = Block{
        index: 1,
        miner:new_user,
        transaction_count:2,
    };
    

    println!("Block Index {}, Miner {}, Transaction : {} , Active : {} ", mined_block.index, 
    mined_block.miner.username,
     mined_block.transaction_count, 
     mined_block.miner.active);

    println!("\n-------------------------------------\n");
}
