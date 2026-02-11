 
 pub trait Identity {

        fn verify(&self)-> bool;

        fn describe(&self)-> String{
            String::from("This is a anonymous entity on the blockchain")
        }
 }



 #[derive(Debug)]

 struct  RegularUser {
    username: String,
    reputation: u8,
 }
 #[derive(Debug)]

 struct  AdminUser {
    username: String,
    admin_code : u32,
 }


impl Identity for RegularUser{
    fn verify(&self)-> bool {
        if self.reputation > 50{
            println!("{} is a verified user with reputation {}", self.username, self.reputation);
            true
        }
        else {
            println!("{} is not a verified user with reputation {}", self.username, self.reputation);
            false
         }
        }

        fn describe(&self)-> String {
            format!("Regular User: {}", self.username, )
         }
        }


impl  Identity for AdminUser{
        fn verify(&self)-> bool {
            if self.admin_code == 777{
                println!("{} is a verified admin user with code {}", self.username, self.admin_code);
                true
            }
            else {
                println!("{} is not a verified admin user with code {}", self.username, self.admin_code);
                false
            }
        }
}
    


    fn login_process(entity : &impl  Identity){
        println!("Login Attempt : ----");
        println!("{}", entity.describe());

        if entity.verify() {
  println!("Login successful");
        }

        
        else {
            println!("Login failed");
        }
    }





pub fn demo() {

    println!("\n-------------------------------------\n");

    println!("Module 01 : Traits Example");

    let jyotishman = RegularUser {
        username: String::from("Jyotishman"),
        reputation: 75,
    };

    let fake_admin =  AdminUser {
        username : String::from("FakeAdmin"),
        admin_code: 123,
    };

    let real_admin = AdminUser {
        username : String::from("Jefrey Epstein"),
        admin_code: 777,
    };


    login_process(&jyotishman);
    login_process(&fake_admin);
    login_process(&real_admin);


 println!("\n-------------------------------------\n");



}
