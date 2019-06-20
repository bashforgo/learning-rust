use std::io;

fn main() {
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    println!("{}", input.trim());

    let mut output = String::new();
    for word in input.split_whitespace() {
        let lower_first = word.chars().next().unwrap().to_ascii_lowercase();
        match lower_first {
            'a' | 'e' | 'i' | 'o' | 'u' => {
                output += &format!("{}-hay", word);
            }
            'b' | 'c' | 'd' | 'f' | 'g' | 'h' | 'j' | 'k' | 'l' | 'm' | 'n' | 'p' | 'q' | 'r'
            | 's' | 't' | 'v' | 'w' | 'x' | 'y' | 'z' => {
                let (first, rest) = word.split_at(1);
                output += &format!("{}-{}ay", rest, first);
            }
            _ => panic!("space separated english words please"),
        }
        output += " ";
    }

    println!("{}", output);
}
