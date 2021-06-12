use std::io;

pub(crate) fn read_from_stdin() -> Vec<String> {
    let mut incoming: Vec<String> = vec![];

    let mut input = String::new();
    match io::stdin().read_line(&mut input) {
        Ok(_n) => {
            incoming.push(input.trim_end().parse().unwrap())
        }
        Err(error) => println!("error: {}", error)
    }
    return incoming
}
