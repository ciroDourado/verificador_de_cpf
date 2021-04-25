#![allow(non_snake_case)]

use cpf::CPF;

#[test]
fn cpf_works() {
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
    

    ///////////
    println!();
    ///////////


    let cpfSemSeparadores = CPF::from("52998224725");
    println!("{}", cpfSemSeparadores.get());

    match cpfSemSeparadores.ehValido() {
        true  => println!("Este CPF é válido"),
        false => println!("O CPF foi dado incorretamente")
    }
    match cpfSemSeparadores.validarCPF() {
        Ok(resultado) => println!("{}", resultado),
        Err(motivo)   => println!("{}", motivo)
    }


    ///////////
    println!();
    ///////////


    let cpfEntreCaracteres = CPF::from("b529982d24725");
    println!("{}", cpfEntreCaracteres.get());

    match cpfEntreCaracteres.ehValido() {
        true  => println!("Este CPF é válido"),
        false => println!("O CPF foi dado incorretamente")
    }
    match cpfEntreCaracteres.validarCPF() {
        Ok(resultado) => println!("{}", resultado),
        Err(motivo)   => println!("{}", motivo)
    }


}
