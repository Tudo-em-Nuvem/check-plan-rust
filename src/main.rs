use std::io::{stdin, stdout, Write};
mod utils;
use utils::faturamento::gerar_planilha_faturamento;

fn main() {
    loop {
        let message = String::from("
Escolha uma opção:
[1] - Gerar planilha de faturamento\n");

        let option = input(String::from(message));
        handling(option);
    }
    
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

fn handling(option: String) {
    match option.as_str() {
        "1" => {
            println!("Gerando planilha de faturamento...");
            gerar_planilha_faturamento();
        }
        _ => {
            println!("Opção inválida. Tente novamente.");
        }
    }
}