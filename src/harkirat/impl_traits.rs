

pub trait Wallet {
  fn  balance(&self) -> f64;

  fn chain(&self) -> String {
    String::from("SOLANA")
  }

}


struct  MetamaskWallet {
    eth_balance : f64,
    address : String,
    chain : String,
}

struct PhantomWallet {
    sol_balance : f64,
    address : String,
    chain : String,
}



impl  Wallet for MetamaskWallet {
        fn  balance(&self) -> f64 {
            println!("Metamask Wallet Address: {}, Chain: {}", self.address, self.chain);
            self.eth_balance
            
        }

        fn chain(&self) -> String {
            if self.chain != "SOLANA" {
                println!("Warning: This wallet is not on the SOLANA chain. {}", self.chain);

            } else {
                    println!("This wallet is on the SOLANA chain. {}",self.chain);
            }
        
            self.chain.clone()
        }
    }


impl  Wallet for PhantomWallet {
    fn  balance(&self) -> f64 {
        println!("The balance of Wallet is {} SOL , the Address is : {} , chain : {}", self.sol_balance, self.address, self.chain);
        self.sol_balance
    }

}


pub fn check_portfolio(wallet : &impl Wallet){
    println!("Checking portfolio for wallet on chain: {}", wallet.chain());
    println!("Current balance: {}", wallet.balance());
}

pub fn demo(){
    let metamask_wallet = MetamaskWallet {
        eth_balance: 2.5,
        address: String::from("0x1234567890abcdef"),
        chain: String::from("ETHEREUM"),
    };

    let phantom_wallet = PhantomWallet {
        sol_balance: 10.0,
        address: String::from("Phantom12345"),
        chain: String::from("SOLANA"),
    };

    check_portfolio(&metamask_wallet);
    check_portfolio(&phantom_wallet);
}