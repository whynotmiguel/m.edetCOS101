fn main() {
    let v = vec!['C','O','M','P','U','T','E','R'];

    let mut input1 = String::new();

    println!("Enter an idex value bttw (0 - 7)");
    std::io::stdin().read_line(&mut input1).expect("Failed to read input");
    let index:usize = input1.trim().parse().expect("Invalid Input");

    let ch: char = v[index];
    println!("{} is the character for index [{}]\n", ch, index);
}

