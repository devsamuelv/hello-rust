use std::io;

fn main() {
    println!("Input a name");

    let mut name = String::new();
    let mut array = [2; 1];

    io::stdin().read_line(&mut name).expect("Expected name");

    println!("Selected name {name}");

    iterate(&array);

    println!("EOL;\n");

    array = [3];

    iterate(&array);
}

fn iterate(array: &[i32]) {
    for i in array {
        println!("{i}")
    }
}
