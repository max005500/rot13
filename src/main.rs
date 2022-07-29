use std::env;
fn main() {
    let mut args: Vec<String> = env::args().collect();
    args[0] = String::from("");

    let oracion: String = args.join(" ");
    let _letras: Vec<char> = oracion
        .chars()
        .map(|x| match x {
            'a'..='m' | 'A'..='M' => ((x as u8) + 13) as char,
            'n'..='z' | 'N'..='Z' => ((x as u8) - 13) as char,
            _ => x,
        })
        .collect();

    let oracion: String = String::from_iter(_letras);
    print!("{}", oracion);
}
