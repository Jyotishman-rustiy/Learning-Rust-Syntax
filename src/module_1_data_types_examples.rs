pub fn primitive_data_types() {
    // Integers
    let token_supply: u64 = 1_000_000_000_000_000_000;
    let block_number: i64 = -123456789;

    println!("Token Supply (u64): {}", token_supply);
    println!("Block Number (i64): {}", block_number);

    // Floating point
    let token_price: f32 = 3.14;
    let transaction_fees: f64 = 0.0000001;

    println!("Token price (f32): {}", token_price);
    println!("Transaction fees (f64): {}", transaction_fees);

    // Boolean
    let is_minted: bool = true;
    println!("Is token minted: {}", is_minted);

    // Character
    let symbol: char = '₹';
    let rocket: char = '🚀';

    println!("Currency symbol: {}", symbol);
    println!("Launch icon: {}", rocket);

    //&str

    let wallet_address : &str = "ox127283476294dhBDHS";
    let contract_name : String = String::from("Decentralised Exchange");

    println!("Wallet Address : {}", wallet_address);
    println!("Contract Name: {}", contract_name);

    // Tuple
    let transaction: (u32, f64, bool) = (1, 2.5, true);
    println!("Transaction tuple: {:?}", transaction);

    // Array
    let validators: [u8; 3] = [1, 2, 3];
    println!("Validators: {:?}", validators);

    // Unit type
    let nothing: () = ();
    println!("Unit type: {:?}", nothing);
}


pub fn solana_arithmetic_demo() {
    // 1 SOL = 1_000_000_000 lamports
    let mut sender_balance: u64 = 5_000_000_000;
    let mut receiver_balance: u64 = 1_000_000_000;

    let transfer_amount: u64 = 2_000_000_000;
    let transaction_fee: u64 = 5_000;

    println!("--- Initial State ---");
    println!("Sender balance: {} lamports", sender_balance);
    println!("Receiver balance: {} lamports", receiver_balance);

    // 🔁 Transfer arithmetic (SUB + ADD)
    sender_balance = sender_balance - transfer_amount - transaction_fee;
    receiver_balance = receiver_balance + transfer_amount;

    println!("\n--- After Transfer ---");
    println!("Sender balance: {} lamports", sender_balance);
    println!("Receiver balance: {} lamports", receiver_balance);

    // 🧮 Mint arithmetic (ADD)
    let mut total_supply: u64 = 10_000_000_000;
    let mint_amount: u64 = 1_000_000_000;

    total_supply = total_supply + mint_amount;

    println!("\n--- Mint Operation ---");
    println!("Total supply after mint: {} lamports", total_supply);

    // 🔥 Burn arithmetic (SUB)
    let burn_amount: u64 = 500_000_000;
    total_supply = total_supply - burn_amount;

    println!("\n--- Burn Operation ---");
    println!("Total supply after burn: {} lamports", total_supply);

    // ➗ Rent exemption calculation (DIV)
    let rent_exempt_lamports: u64 = 890_880;
    let months: u64 = 12;

    let monthly_rent: u64 = rent_exempt_lamports / months;

    println!("\n--- Rent Calculation ---");
    println!("Monthly rent: {} lamports", monthly_rent);
}

pub fn solana_logical_demo() {
    let is_signer: bool = true;
    let is_writable: bool = true;
    let is_owner: bool = true;

    let account_balance: u64 = 3_000_000_000;
    let required_balance: u64 = 1_000_000_000;

    let is_rent_exempt: bool = account_balance >= required_balance;

    println!("--- Account Validation ---");

    // AND (&&) — all conditions must be true
    if is_signer && is_writable && is_owner {
        println!("Account passed authority checks");
    } else {
        println!("Account failed authority checks");
    }

    // AND with comparison
    if is_signer && is_rent_exempt {
        println!("Signer account is rent exempt");
    }

    // OR (||) — one condition is enough
    let is_system_program: bool = false;
    let is_token_program: bool = true;

    if is_system_program || is_token_program {
        println!("Allowed program invoked");
    }

    // NOT (!) — invert condition
    if !is_writable {
        println!("Account is read-only");
    } else {
        println!("Account is writable");
    }

    // Equality / Inequality
    let expected_owner: &str = "TokenProgram";
    let actual_owner: &str = "TokenProgram";

    if actual_owner == expected_owner {
        println!("Account owner verified");
    }

    if actual_owner != "SystemProgram" {
        println!("Account is not system-owned");
    }
}


pub fn variable_shadowing_and_conversion(){
    let account_balance :i32 = 500;
    println!("Initial account balance {}", account_balance);

    let account_balance :i32 = account_balance +100;
    println!("Updated account balance {} ", account_balance);

    let gas_fee:f64 = 5.0025;
    let gas_fee_int : i32 = gas_fee as i32;

    println!("Gas fee (f64): {}, Converted to lamports: {}", gas_fee, gas_fee_int);


    let block_height :i32 = 123456;
    let block_height_str: String = block_height.to_string();


    println!("Block height {}, Converted to string {}", block_height, block_height_str);

}

pub fn mutability_example(){
   
    let mut user_balance :i32 = 500;
    println!("Before transaction balance = {} ", user_balance);
    user_balance -= 50;
    println!("After transaction User Balance = {} " , user_balance)
    
}

pub fn tuple_destructing_example(){
    let transaction_info = ("transfer", 200, 0.0003);
    let (tx_type , tx_amount, tx_fee) = transaction_info;

    println!(
        "Transaction Type : {}, Amount : {} , Fees : {}",

        tx_type,tx_amount, tx_fee
    );
   println!(
        "Transaction Type : {}, Amount : {} , Fees : {}",

      transaction_info.0, transaction_info.1, transaction_info.2,
    );


    println!("The transaction info is : {:?} ", transaction_info)
}



pub fn demo() {
        println!("\n--- Rust Primitive Data Types Demo ---\n");
        primitive_data_types();
        println!("\n-------------------------------------\n");
        solana_arithmetic_demo();
        println!("\n-------------------------------------\n");
        solana_logical_demo();
        println!("\n-------------------------------------\n");
        variable_shadowing_and_conversion();
        println!("\n-------------------------------------\n");
        mutability_example();
        println!("\n-------------------------------------\n");
        tuple_destructing_example();
}
