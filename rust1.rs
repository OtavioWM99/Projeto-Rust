//iniciando a função principal
/* 
fn main() {
    println!("Hello World")
}
*/

//imprimindo mensagens no terminal
/* 
fn main() {
    println!("Hello World!"); // imprime uma mensagem simples
    println!("Número: {}", 42); // usa {} como placeholder
    println!("Soma: {} + {} = {}", 2, 3, 2 + 3); // múltiplos placeholders
}
*/

//usando variáveis
//por padrão, variáveis são imutáveis
//para criar uma variável mutável, usamos "mut"
/* 
fn main() {
    let nome = "Jonas"; // variável imutável
    let mut idade = 25; // variável mutável
    println!("{} tem {} anos", nome, idade);
    
    idade += 1;
    println!("Agora tem {} anos", idade);
}
*/

//tipos de dados
//Rust é uma linguagem estática e fortemente tipada
/*
fn main() {
    let inteiro: i32 = 10;         // número inteiro de 32 bits
    let flutuante: f64 = 3.14;     // ponto flutuante de 64 bits
    let booleano: bool = true;     // verdadeiro ou falso
    let caractere: char = 'R';     // aspas simples para char
    let texto: &str = "Olá";       // string literal (imutável)
    let string: String = String::from("Rust!"); // string alocada dinamicamente
    
    println!("int: {}, float: {}, bool: {}, char: {}, texto: {}, string: {}", 
             inteiro, flutuante, booleano, caractere, texto, string);
}
*/

//lendo dados do usuário
//usando a biblioteca std::io para entrada de dados
/* 
use std::io;

fn main() {
    let mut entrada = String::new();
    println!("Digite seu nome:");

    io::stdin()
        .read_line(&mut entrada)
        .expect("Erro ao ler entrada");

    println!("Olá, {}", entrada); 
}
*/

//condicionais
//usando if, else if e else 
fn main() {
    let idade = 17;

    if idade >= 18 {
        println!("Maior de idade");
    } else if idade >= 16 {
        println!("Pode votar");
    } else {
        println!("Menor de idade");
    }
}

