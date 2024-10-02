// Desafio de projeto 02 - Calculadora de Partidas Rankeadas
// Dev: Rychards Matos
// Data: 1/10/2024
// Linguagem: Rust

use std::io;

fn get_hero_name() -> String {
    // Personagem
    println!("Digite o nome do Heroi:");
    let mut name_hero = String::new(); // Nome do Heroi
    io::stdin()
        .read_line(&mut name_hero)
        .expect("Falhar ao ler nick do heroi");
    name_hero.trim().to_string() //Removendo os espaços em branco
}
fn get_victory() -> u32 {
    // Vitorias
    println!("Digite o total de vitorias:");
    let mut victory_input = String::new();
    io::stdin()
        .read_line(&mut victory_input)
        .expect("Falhar ao ler numeros de vitorias");
    victory_input
        .trim()
        .parse()
        .expect("Por favor, digite um numero valido")
}

fn get_default() -> u32 {
    // Derrotas
    println!("Digite o total de Derrotas:");
    let mut default_input = String::new();
    io::stdin()
        .read_line(&mut default_input)
        .expect("Falhar ao ler numeros de derrotas");
    default_input
        .trim()
        .parse()
        .expect("Por favor, digite um numero valido")
}

fn levels(result: u32) -> &'static str {
    // Função dos ranks
    if result <= 10 {
        "Ferro"
    } else if result <= 20 {
        "Bronze"
    } else if result <= 50 {
        "Prata"
    } else if result <= 80 {
        "Ouro"
    } else if result <= 90 {
        "Diamante"
    } else if result <= 100 {
        "Lendário"
    } else {
        "Imortal"
    }
}

fn main() {
    let name_hero = get_hero_name(); // Chamando função get hero name
    let victory = get_victory(); // Chamando função get victory
    let default = get_default(); // Chamando função get default

    let result = victory.saturating_sub(default); // Metodo para resultado não ficar negativo ou abaixo de zero.
    let level = levels(result);

    println!(
        "O Herói {} tem de saldo de {} vitórias é está no nível {}",
        name_hero, result, level
    );
}
