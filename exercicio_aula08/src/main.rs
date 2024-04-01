use std::io::{self, Write};

fn main() {
    loop {
        print!("\n");
        println!("-----------------------");
        println!("1. Cadastrar aluno");
        println!("2. Alterar aluno");
        println!("3. Excluir aluno");
        println!("4. Listar aluno");
        println!("5. Sair do programa");
        println!("Digite sua opção: ");
        println!("-----------------------");
        io::stdout().flush().unwrap(); // é por conta do loop, para limpar o buffer de saída

        let mut opcao = String::new();
        io::stdin().read_line(&mut opcao).unwrap();
        let opcao: u32 = match opcao.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        match opcao {
            1 => {
                print!("Iniciando cadastro de aluno ");               
            },
            2 => {
                println!("Iniciando alteração de aluno");               
            },
            3 => {
                print!("Iniciando exclusão de aluno ");                
            },
            4 => {
                print!("listando alunos ");                
            },
            5 => {
                println!("Saindo do programa...");
                break;
            },
            _ => {
                println!("Opção inválida!");
            },
        }
    }
}