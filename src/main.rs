#![allow(non_snake_case)]

mod cpf;
use crate::cpf::CPF;

fn main() {
    let cpfValido = CPF::from("529.982.247-25");
    println!("{}", cpfValido.get());
    
    match cpfValido.ehValido() {
        true  => println!("Este CPF é válido"),
        false => println!("O CPF foi dado incorretamente")
    }
    match cpfValido.validarCPF() {
        Ok(resultado) => println!("{}", resultado),
        Err(motivo)   => println!("{}", motivo)
    }
    

    ///////////
    println!();
    ///////////


    let cpfInvalido = CPF::from("123.456.789-01");
    println!("{}", cpfInvalido.get());

    match cpfInvalido.ehValido() {
        true  => println!("Este CPF é válido"),
        false => println!("O CPF foi dado incorretamente")
    }
    match cpfInvalido.validarCPF() {
        Ok(resultado) => println!("{}", resultado),
        Err(motivo)   => println!("{}", motivo)
    }
} // fim da main
