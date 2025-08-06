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
/* 
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
*/

//operadores 
/* 
fn main() {
    let a = 10;
    let b = 5;

    println!("Soma: {}", a + b); // soma
    println!("Diferença: {}", a - b); // subtração
    println!("Produto: {}", a * b); // multiplicação
    println!("Divisão: {}", a / b); // divisão
    println!("Módulo: {}", a % b); // resto da divisão
    println!("Igual? {}", a == b); // igualdade
    println!("Maior? {}", a > b); // maior que
    println!("Menor? {}", a < b); // menor que
}
*/

//estruturas de repetição
/* 
fn main() {
    let mut contador = 0;

    loop { //loop infinito até usar break
        println!("Contando: {}", contador);
        contador += 1;
        if contador == 10 {
            break;
        }
    }
}
*/

//while
/* 
fn main() {
    let mut n = 0;
    while n < 3 {
        println!("n = {}", n);
        n += 1;
    }
}
*/

//for
/* 
fn main() {
    for i in 1..5 { // inclui o 1 e vai até 4
        println!("i = {}", i);
    }
}
*/

//arrays
/* 
fn main() {
    let array = [1, 2, 3]; // tamanho fixo
    let mut vetor = vec![10, 20, 30]; // tamanho dinâmico

    println!("Primeiro do array: {}", array[0]);
    vetor.push(40);
    println!("Último do vetor: {}", vetor[vetor.len() - 1]);
}
*/

//strings
/*
fn main() {
    let mut nome = String::from("Rust");
    nome.push_str ("Lang"); // concatena
    println!("{}", nome); // RustLang

    let tamanho = nome.len();
    println!("Tamanho: {}", tamanho);

}
*/

//funções
/* 
fn saudacao(nome: &str) {
    println!("Olá, {}", nome);
}

fn soma(a: i32, b: i32) -> i32 {
    a + b
}

fn main() {
    saudacao("Jonas");
    let resultado = soma(5, 7);
    println!("Resultado da soma: {}", resultado);
}
*/

//métodos
//métodos são funções associadas a structs ou enums
/* 
struct Pessoa {
    nome: String,
    idade: u8,
}

impl Pessoa {
    fn saudacao(&self) {
        println!("Olá, meu nome é {}", self.nome);
    }

    fn aniversario(&mut self) {
        self.idade += 1;
    }
}

fn main() {
    let mut p = Pessoa {
        nome: String::from("Otávio"),
        idade: 20,
    };

    p.saudacao();
    p.aniversario();
    println!("Nova idade: {}", p.idade);
}
*/

// Definição de "classe" Pessoa
/* 
fn main() {
    let pessoa = Pessoa {
        nome: String::from("Jonas"),
        idade: 30,
    };

    pessoa.saudacao(); // Chama o método saudacao
}
struct Pessoa {
    nome: String,
    idade: u8,
}

impl Pessoa {
    // Método (comportamento)
    fn saudacao(&self) {
        println!("Olá, meu nome é {} e tenho {} anos.", self.nome, self.idade);
    }
}
*/

//atributos são campos da struct
/* 
struct ContaBancaria {
    titular: String,
    saldo: f64, // atributo
}
*/

//exemplo com struct, atributos, métodos e "construtor"
struct Retangulo {
    largura: u32,
    altura: u32,
}

impl Retangulo {
    // "Construtor"
    fn new(largura: u32, altura: u32) -> Self {
        Self { largura, altura }
    }

    fn area(&self) -> u32 {
        self.largura * self.altura
    }
}

fn main() {
    let r = Retangulo::new(10, 5);
    println!("Área: {}", r.area());
}

