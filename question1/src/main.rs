mod banks{
    pub mod branch {
       #[derive(Debug)]
        struct Bank{
            bankname:String,
            city:String,
            branch_code: u8
        }   
        pub fn  bankname(){
            let mybank= Bank{
                bankname:"soneri".to_string(),
                city:"Islamabad".to_string(),
                branch_code:0224
            };
        println!("My Bank info is {:#?}",mybank);
        }
    }
}

use crate::banks::branch;
fn main() {
    println!("Module function call here");
    branch::bankname();
}
