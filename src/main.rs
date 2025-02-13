use std::io::{stdin, stdout, Write};

fn main() {
    let message = String::from("Escolha uma opção:
    [1] Comparar colunas de duas planilhas\n");
    let res = input(message);
    println!("{}", res)
}

fn input(question: String) -> String {
    let mut s = String::new();
    print!("{}", question.to_string());

    let _ = stdout().flush();
    stdin().read_line(&mut s).expect("Algo deu errado");

    if let Some('\n') = s.chars().next_back() {
        s.pop();
    }

    if let Some('\r') = s.chars().next_back() {
        s.pop();
    }

    s
}
