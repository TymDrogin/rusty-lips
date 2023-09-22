use std::io;

#[allow(non_snake_case)]
fn READ(input: String) -> String {
    input
}
#[allow(non_snake_case)]
fn EVAL(input: String) -> String {
    input
}
#[allow(non_snake_case)]
fn PRINT(input: String) -> String {
    input
}
fn rep(input: String) -> String {
    let ast = READ(input);
    let result = EVAL(ast);
    let output = PRINT(result);
    output
}

fn main() {
    loop {
        let mut input = String::new();
        print!("user>");

        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read line");

        if input == "EoF" {
            break;
        }
        let result = rep(input);
        print!("{}", result);
    }
}








