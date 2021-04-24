#![allow(
    non_snake_case,
    dead_code,
    unused_variables
)]


//     para um cpf ser válido, ele deve conter:
//     - 11 dígitos
//     - desses, os 9 primeiros devem formar os 2 últimos
//
//     a função de verificar cpf deve retornar um bool,
//     pois como ela própria diz: ou o cpf é válido, ou
//     não é

use std::cmp::Ordering;

pub struct CPF {
    cpf: String
} // fim da struct

impl CPF {
    // for mutable instances
    pub fn new() -> Self {
        CPF { cpf: String::new() }
    } // fim do construtor


    // for immutable instances
    pub fn from(digitos: &'static str) -> Self {
        CPF { cpf: String::from(digitos) }
    } // fim do construtor


    pub fn set(&mut self, digitos: &'static str) {
        self.cpf = String::from(digitos);
    } // fim do método set digitos


    pub fn get(&self) -> String {
        self.cpf.clone()
    } // fim do método get digitos


    pub fn ehValido(&self) -> bool {
        self.validarCPF().is_ok()
    } // fim do método CPF é válido


    pub fn validarCPF(&self) -> Result<&'static str, &'static str> {
        self.tem11Digitos()?;
            Ok("O CPF dado é valido")
    } // fim do método CPF é válido


    fn tem11Digitos(&self) -> Result<(), &'static str> {
        let numeroDeDigitos = self.obterDigitos().len();

        match numeroDeDigitos.cmp(&11) {
            Ordering::Equal   => self.oPenultimoEhValido(),
            Ordering::Less    => Err("Menos que 11 dígitos"),
            Ordering::Greater => Err("Mais que 11 dígitos")
        }
    } // fim do método privado tem11Digitos


        fn obterDigitos(&self) -> String {
            let digitos = self.get().as_str().chars()
                .filter(|&char| char.is_ascii_digit())
                .collect::<String>();
            digitos
        } // fim obterDigitos


    fn oPenultimoEhValido(&self) -> Result<(), &'static str> {
        let indice: usize = 9;
        let penultimo = self.obterDigito(indice);

        match penultimo == self.gerarVerificador(indice) {
            true  => self.oUltimoEhValido(),
            false => Err("O CPF não gera o penúltimo algarismo")
        }
    } // fim do método privado oPenultimoEhValido


        fn obterDigito(&self, posicao: usize) -> u32 {
            self.obterDigitos().as_str().chars()
                .nth(posicao).unwrap()
                .to_digit(10).unwrap()
        } // fim obterPenultimoDigito


    fn oUltimoEhValido(&self) -> Result<(), &'static str> {
        let indice: usize = 10;
        let ultimo = self.obterDigito(indice);

        match ultimo == self.gerarVerificador(indice) {
            true  => Ok(()),
            false => Err("O CPF não gera o último algarismo")
        }
    } // fim do método privado oUltimoEhValido


        fn gerarVerificador(&self, indice: usize) -> u32 {
            let elementos = self.digitosVezesIndices(11 - indice);
            let soma = elementos.iter().sum::<u32>();
            let resto = (soma*10) % 11;

            match resto == 10 {
                true  => 0,
                false => resto
            }
        } // fim do método privado gerarVerificador


        fn digitosVezesIndices(&self, quantidade: usize) -> Vec<u32> {
            let elementos = self.obterDigitos().as_str().chars()
                .rev().skip(quantidade)
                .map(charPara_u32)
                .zip( (2..).into_iter() )
                .map(digitoVezesIndice)
                .collect::<Vec<u32>>();
            elementos
        } // fim digitosVezesIndices

} // fim dos métodos


fn charPara_u32(algarismo: char) -> u32 {
    algarismo.to_digit(10).unwrap()
} // fim digitoVezesIndice


fn digitoVezesIndice(enupla: (u32, usize)) -> u32 {
    let digito = enupla.0;
    let indice = enupla.1 as u32;

    digito*indice
} // fim digitoVezesIndice


fn main() {
    let cpfUsuario = CPF::from("529.982.247-25");

    match cpfUsuario.ehValido() {
        true  => println!("Este CPF é válido"),
        false => println!("O CPF foi dado incorretamente")
    }

    // os blocos de código abaixo são apenas
    // tentativas de visualizar melhor qual
    // forma de escrever é a mais clara/legível

    // println!("{}", match cpfUsuario.validarCPF() {
    //     Ok(resultado) => resultado,
    //     Err(motivo)   => motivo
    // });

    match cpfUsuario.validarCPF() {
        Ok(resultado) => println!("{}", resultado),
        Err(motivo)   => println!("{}", motivo)
    }

    // let validacao = match cpfUsuario.validarCPF() {
    //     Ok(resultado) => resultado,
    //     Err(motivo)   => motivo
    // };
    // println!("{}", validacao);

} // fim da main
