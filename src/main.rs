use std::io;
use clap::Parser;

mod encryption;
mod cli_handler;

fn main() {
    let mut arg_len = (std::env::args()).len();

    loop {
        let mut input_text: String = String::new();
        let mut decider: String = String::new();
        let mut mode: String = String::new();
        let mut key: String = String::new();
        let mut recursion: String = String::new();    

        // Start
        if arg_len <= 1 {
            println!("Input Text");
            io::stdin()
                .read_line(&mut input_text)
                .expect("Couldn't read line");

            println!("Encrypt | Decrypt");
            io::stdin()
                .read_line(&mut decider)
                .expect("Couldn't read line");

            println!("Default | ASCII");
            io::stdin()
                .read_line(&mut mode)
                .expect("Couldn't read line");

            println!("Key");
            io::stdin()
                .read_line(&mut key)
                .expect("Couldn't read line");

            println!("Recursion");
            io::stdin()
                .read_line(&mut recursion)
                .expect("Couldn't read line");
        }
        else {
            let args = cli_handler::CliOutput::parse();

            input_text = match std::fs::read_to_string(args.input_file){
                Ok(text) => text,
                Err(_) => {
                    println!("Could not read file at path");
                    break;
                }
            };

            decider = args.decider;
            mode = args.mode;
            key = args.key;
            recursion = args.recursion;
            
            println!("Input: {} \nMode: {}\nKey: {}\nRecursion: {}", input_text, mode, key, recursion);
        }

        arg_len = 0;

        // Parse To Integer 
        let key: i32 = match key.trim().parse(){
            Ok(num) => num,
            Err(_) => {
                println!("Invalid Key");
                0
            },
        };

        let recursion: i32 = match recursion.trim().parse(){
            Ok(num) => num,
            Err(_) => {
                println!("Invalid Recursion");
                0
            },
        };

        println!("Decider: {}", decider.to_lowercase().as_str());

        let output: String = match mode.trim().to_lowercase().as_str(){
            "default" => match decider.trim().to_lowercase().as_str() {
                "encrypt" => encryption::encrypt(&input_text.trim().to_string().to_uppercase(), key, recursion),
                "decrypt" => encryption::decrypt(&input_text.trim().to_string().to_uppercase(), key, recursion),
                _ => {
                    println!("Please use either 'Encrypt' or 'Decrypt'");
                    continue;
                }
            },
            "ascii" => match decider.trim().to_lowercase().as_str() {
                "encrypt" => encryption::ascii_encrypt(&input_text.trim().to_string(), key.try_into().unwrap(), recursion),
                "decrypt" => encryption::ascii_decrypt(&input_text.trim().to_string(), key.try_into().unwrap(), recursion),
                _ => {
                    println!("Please use either 'Encrypt' or 'Decrypt'");
                    continue;
                }
            }
            _ => {
                println!("Please use either 'Default' or 'ASCII'");
                continue;
            }
        };

        println!("Output: {}", output);        
    }
}