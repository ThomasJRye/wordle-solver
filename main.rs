use std::env;
use std::fs;

fn main() {
    let path = "index.txt";
    let mut contents = fs::read_to_string(path).expect("something wrong");

    let mut words = Vec::new();

    let mut iterator = 0;
    let mut lengthRead = 0;

    println!("With text:\n{}", &contents[0..20]);

    for i in contents.chars() {
        if i.is_alphabetic() {
            iterator += 1;
        } else {
            if iterator == 5 {
                words.push(&contents[lengthRead..lengthRead + iterator]);

                println!("{}", &contents[lengthRead..lengthRead + iterator]);
                println!("lengthRead: {}, iterator: {}", lengthRead, iterator);

                lengthRead += iterator;
                iterator = 0;
            } else {
                lengthRead += iterator;
                iterator = 0;
            }
        }
    }
}
