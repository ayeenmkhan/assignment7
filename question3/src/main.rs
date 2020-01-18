use myPackage;
use std::io; 
fn main() {
    loop {
        println!("Enter First number to Add");
        
        let mut num1 = String::new();
    
         io::stdin().read_line(&mut num1)
                .expect("Failed to read line");
    
        //let guess: u32 =  guess.trim().parse().unwrap();
             let num1: u32 = match num1.trim().parse() {
                 Ok(num) => num,
                 Err(_) => continue,
            };
            println!("You entered: {}", num1);

            println!("Enter Second number to Add");
        
            let mut num2 = String::new();
        
             io::stdin().read_line(&mut num2)
                    .expect("Failed to read line");
        
            //let guess: u32 =  guess.trim().parse().unwrap();
                 let num2: u32 = match num2.trim().parse() {
                     Ok(num) => num,
                     Err(_) => continue,
                };
                println!("You entered: {}", num2);
                     

                myPackage::add(num1,num2);
            break;
    
        }  
}
