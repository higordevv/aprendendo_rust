fn main() {
    let  x = 5;
    println!("O valor de x é: {}", x);
    x = 6;
    println!("O valor de x é: {}", x);
}
// EXEMPLO COM MUTABILIDADE
// fn main() {
//     let mut x = 5;
//     println!("O valor de x é: {}", x);
//     x = 6;
//     println!("O valor de x é: {}", x);
// }

/*
Shadowing

fn main() {
    let x = 5;

    let x = x + 1;

    let x = x * 2;

    println!("O valor de x é: {}", x);
}

output: 

$ cargo run
   Compiling variaveis v0.1.0 (file:///projects/variaveis)
    Finished dev [unoptimized + debuginfo] target(s) in 0.31 secs
     Running `target/debug/variaveis`
O valor de x é: 12

A declaração "shadowing" permite a criação de uma nova variável com o mesmo nome de uma já existente, enquanto a utilização de "let" impede a reatribuição de uma variável declarada como "imutável".
*/