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
fn main() {
    let nome = "Jonas"; // variável imutável
    let mut idade = 25; // variável mutável
    println!("{} tem {} anos", nome, idade);
    
    idade += 1;
    println!("Agora tem {} anos", idade);
}

