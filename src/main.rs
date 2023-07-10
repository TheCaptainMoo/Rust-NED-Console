use std::io;

static ALPHABET: &str = "ABCDEFGHIJKLMNOPQRSTUVWXYZ";

fn main() {
    loop {
        let mut input_text: String = String::new();
        let mut decider: String = String::new();
        let mut key: String = String::new();
        let mut recursion: String = String::new();    

        // Start
        println!("Input Text");
        io::stdin()
            .read_line(&mut input_text)
            .expect("Couldn't read line");

        println!("Encrypt | Decrypt");
        io::stdin()
            .read_line(&mut decider)
            .expect("Couldn't read line");

        println!("Key");
        io::stdin()
            .read_line(&mut key)
            .expect("Couldn't read line");

        println!("Recursion");
        io::stdin()
            .read_line(&mut recursion)
            .expect("Couldn't read line");

        // Parse To Integer 
        let key: i32 = match key.trim().parse(){
            Ok(num) => num,
            Err(_) => 0,
        };

        let recursion: i32 = match recursion.trim().parse(){
            Ok(num) => num,
            Err(_) => 0,
        };

        println!("Decider: {}", decider.to_lowercase().as_str());

        match decider.trim().to_lowercase().as_str() {
            "encrypt" => encrypt(&input_text.trim().to_string().to_uppercase(), key, recursion),
            "decrypt" => decrypt(&input_text.trim().to_string().to_uppercase(), key, recursion),
            _ => {
                println!("Please use either 'Encrypt' or 'Decrypt'");
                continue;
            }
        };
    }
}

fn encrypt(text: &String, addition_key: i32, recursion: i32){
    println!("Encrypting '{}' with a key of: {} and a recursion of: {}", text, addition_key.to_string(), recursion.to_string());

    let mut encrypt_text: String = text.clone();
    let mut letter_index: i32;
    let mut number_out: Vec<i32> = Vec::new();
    let mut split_out: String = String::from("");
    let mut punctuation: String = String::from("");
    
    let mut i: i32 = 0;
    while i < recursion{
        {
            let mut j: usize = 0;
            while j < encrypt_text.len(){
                let char: char = match encrypt_text.chars().nth(j) {
                    Some(c) => c,
                    None => {
                        println!("End of chars");
                        break;
                    }
                };

                letter_index = match ALPHABET.find(char) {
                    Some(index) => index.try_into().unwrap(),
                    None => -1
                };

                println!("{}", letter_index);

                number_out.push(letter_index);

                if letter_index == -1{
                    punctuation.push(char);
                }

                j += 1;
            }
        }
        {
            let mut j: usize = 0;
            while j < number_out.len().try_into().unwrap(){
                if number_out[j] != -1{
                    number_out[j] += addition_key;
                    println!("Number: {}", number_out[j]);
                    split_out.push_str(number_out[j].to_string().trim());
                }
                else{
                    println!("Punctuation Detected");
                    split_out.push('-');
                }

                j += 1;
            }
        }

        encrypt_text.clear();
        let mut index: usize = 0;
        let mut punctuation_index: usize = 0;

        while index < split_out.len(){
            let c = match split_out.chars().nth(index) {
                Some(c) => c,
                None => {
                    println!("End of chars");
                    break;
                }
            };

            println!("{}", c);

            if c != '-'{
                let c_i32: i32 = match c.to_string().trim().parse() {
                    Ok(num) => num,
                    Err(_) => {
                        println!("Character is not a number");
                        break;
                    }
                };

                encrypt_text.push(match ALPHABET.chars().nth(c_i32.try_into().unwrap()) {
                    Some(c) => c,
                    None => '?'
                });
            }
            else{
                encrypt_text.push(match punctuation.chars().nth(punctuation_index) {
                    Some(c) => c,
                    None => '?'
                });
                punctuation_index += 1;
            }

            index += 1;
        }

        println!("Encrypt Text: {}", encrypt_text);

        println!("Recursion {} : {}", i, encrypt_text);

        split_out.clear();
        number_out.clear();

        i += 1;
    }

    println!("OUTPUT: {}", encrypt_text);

}

fn decrypt(text: &String, subtraction_key: i32, recursion: i32){
    println!("Decrypting '{}' with a key of: {} and a recursion of: {}", text, subtraction_key.to_string(), recursion.to_string());

    let mut decrypt_text: String = text.clone();
    let mut number_out: String = String::from("");
    let mut punctuation: String = String::from("");
    let mut join_out: Vec<i32> = Vec::new();

    let mut i: i32 = 0;  
    while i < recursion{
        {
            let mut j: usize = 0;
            while j < decrypt_text.len(){
                let c: char = match decrypt_text.chars().nth(j) {
                    Some(c) => {
                        println!("{}", c);
                        c
                    },
                    None => {
                        println!("End of chars");
                        break;
                    }
                };

                

                let alphabet_index: i32 = match ALPHABET.find(c) {
                    Some(_) => 1,
                    None => -1
                };

                if alphabet_index == -1 {
                    punctuation.push(c);
                    number_out.push_str("-1");
                }
                else{
                    number_out.push_str(match ALPHABET.find(c){
                        Some(c) => c,
                        None => {
                            println!("Cannot find letter in alphabet.");
                            break;
                        }
                    }.to_string().trim());
                }

                j += 1;
            }
        }

        println!("Replaced Output: {}", number_out);

        decrypt_text.clear();
        let loop_length: i32 = number_out.len().try_into().unwrap();

        println!("Loop Length: {}", loop_length);

        let mut index: usize = 0;

        {
            let mut j: usize = 0;
            while j < loop_length.try_into().unwrap() {
                let sub_str = match number_out.get(j..(j+2)){
                    Some(str) => str,
                    None => {
                        println!("Substring doesn't exist.");
                        break;
                    }
                };

                println!("Substring: {} .. {} | {}", j, (j+2), sub_str);
                //println!("Number Out: {}", number_out);

                join_out.push(match sub_str.to_string().trim().parse(){
                    Ok(num) => num,
                    Err(_) => {
                        println!("Number cannot be parsed."); 
                        break;
                    } 
                });

                if !sub_str.starts_with('-'){
                    //println!("Join Out: {}", join_out[j/2]);
                    join_out[j/2] -= subtraction_key;
                    println!("Subtracted Join Out: {}", join_out[j/2]);

                    decrypt_text.push(match ALPHABET.chars().nth(join_out[j/2].try_into().unwrap()){
                        Some(c) => c,
                        None => {
                            println!("Cannot locate position in alphabet.");
                            break;
                        }
                    });
                }
                else{
                    decrypt_text.push(match punctuation.chars().nth(index){
                        Some(c) => c,
                        None => {
                            println!("Cannot location punctionation at specified index.");
                            break;
                        }
                    });

                    index += 1;
                }

                j += 2;
            }
        }

        join_out.clear();
        number_out.clear();

        i += 1;
    }

    println!("OUTPUT: {}", decrypt_text);

}