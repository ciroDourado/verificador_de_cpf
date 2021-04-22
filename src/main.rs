#![allow(non_snake_case)]

/*
        struct CPF {
            digitos: String
        } // fim da struct

        impl CPF {
            fn new() -> Self {
                CPF{ digitos: String::new() }
            } // fim do construtor

            fn from(&mut self, string: String) {

            }
        } // fim dos métodos para CPF
*/

fn main() {
    let cpfUsuario = String::from("529.982.247-25");
    
    match cpfValido(cpfUsuario) {
        true  => println!("Este cpf é válido"),
        false => println!("cpf inválido!")
    }
} // fim da main


// para um cpf ser válido, ele deve conter:
// - 11 dígitos
// - desses, os 9 primeiros devem formar os 2 últimos


// a função de verificar cpf deve retornar um bool,
// pois como ela própria diz: ou o cpf é válido, ou
// não é


fn cpfValido(cpf: String) -> bool {
    match cpf.is_empty() {
        false => tem11Digitos(cpf),
        _     => false,
    }
} // fim cpfValido


fn tem11Digitos(cpf: String) -> bool {
    let digitos = obterDigitos(cpf);
    match digitos.len() {
        11 => penultimoDigitoValido(digitos), 
        _  => false
    }
} // fim tem11Digitos


fn obterDigitos(cpf: String) -> String { 
    let digitos = cpf.as_str().chars()
        .filter(|&char| char.is_ascii_digit())
        .collect::<String>();
    digitos
} // fim obterDigitos


fn penultimoDigitoValido(cpf: String) -> bool {
    cpf.as_str().chars()
        .rev()
        .skip(2)
        .map(|char| char.to_digit(10).unwrap())
        .zip( (2..).into_iter() )
        .map(digitoVezesIndice)
        .sum::<u32>();
    true
} // fim penultimoDigitoValido


fn digitoVezesIndice(enupla: (u32, usize)) -> u32 {
    let digito = enupla.0;
    let indice = enupla.1 as u32;
    
    digito*indice
} // fim digitoVezesIndice

// demais métodos auxiliares devem ser dados, como:
// retornar uma instância de CPF formatado - Option
// retornar uma instância com apenas os dígitos - Option
