use std::io;

fn main() {
    let mut alunos: Vec<(String, String, Vec<f32>)> = Vec::new();

    loop {        
        println!("-----------------------");
        println!("1. Cadastrar aluno");
        println!("2. Alterar aluno");
        println!("3. Excluir aluno");
        println!("4. Listar aluno");
        println!("5. Sair do programa");
        println!("Digite sua opção: ");
        println!("-----------------------");        

        let mut opcao = String::new();
        io::stdin().read_line(&mut opcao).expect("Falha ao ler a opção");
        let opcao: u32 = match opcao.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        match opcao {
            1 => {
                let mut nome = String::new();
                let mut matricula = String::new();
                let mut notas: Vec<f32> = Vec::new();

                println!("Iniciando cadastro de aluno ");
                println!("Digite o nome do aluno: ");
                io::stdin().read_line(&mut nome).unwrap();
                println!("Digite a matrícula do aluno: ");
                io::stdin().read_line(&mut matricula).unwrap();
                println!("Digite as notas do aluno: (ou 'fim' para concluir)");
                let mut nota = String::new();
                loop {                    
                    io::stdin().read_line(&mut nota).unwrap();
                    if nota.contains("fim") {
                        break;
                    }
                    let nota: f32 = match nota.trim().parse() {
                        Ok(num) => num,
                        Err(_) => continue,
                    };
                    notas.push(nota);
                }
                alunos.push((nome.trim().to_string(), matricula.trim().to_string(), notas));
            },
            2 => {
                println!("Iniciando alteração de aluno");
            },
            3 => {
                println!("Iniciando exclusão de aluno ");
            },
            4 => {
                println!("{:?}",alunos)
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