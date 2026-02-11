
macro_rules! create_solana_user {
    ($name : expr) =>{
        println!("Creating a Solana user with the name: {}", $name);
    };

    ($name: expr, $balance:expr)=>{
        println!("Creating a Solana user with the name: {} and balance: {}", $name, $balance);
    };

    (BATCH : $($name: expr),*)=>{
        $(
                        println!("Batch Create : {} connected to devnet ", $name);
        )*

    };

    (SOLANA_BATCH : $($name: expr), *)=>{
        $(
                        println!("Batch Create : {} connected to mainnet ", $name);
        )*
    }

}





pub fn demo(){
    println!("This is the demo function for module_02_declarative_macros");
    create_solana_user!("Jyotishman Pathak",200);
    create_solana_user!("wallah", 1002);

    create_solana_user!(BATCH: "Alice", "Bob");
    create_solana_user!(SOLANA_BATCH:"Charlie", "Dave", "Eve");
    create_solana_user!(BATCH: "Frank", "Grace", "Heidi");


}