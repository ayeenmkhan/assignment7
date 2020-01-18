pub mod banks{
    pub mod branch {
       #[derive(Debug)]
        enum Bank{
           Bankname(String),
            City(String),
            BranchCode(u8),
        }   
        pub fn  mybank(){
        let bank_name= Bank::Bankname(String::from("Bank Al falah"));
        println!("My Bank name is {:#?}!",bank_name);
        }
    }
}