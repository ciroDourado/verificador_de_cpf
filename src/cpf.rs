#![allow(non_snake_case, dead_code)]

//     para um cpf ser válido, ele deve conter:
//     - 11 dígitos
//     - desses, os 9 primeiros devem formar os 2 últimos
//
//     a função de verificar cpf deve retornar um bool,
//     pois como ela própria diz: ou o cpf é válido, ou
//     não é
//
//     To-do: me desafiar a criar um verificador de string,
//     pro caso do usuário dar algo como a52$9%98sd2o24P7%2$5
//     e o programa dar como válido, somente pelo fato de que
//     os dígitos formam uma SEQUENCIA válida (529 982 247 25)
//         
//         ajuda: regex!!!
//         use ([0-9]{3}[\.]{1}){2}[0-9]{3}[-][0-9]{2}
//         para padroes do tipo 
//             000.000.000-00
//         ou use [0-9]{9}[-]?[0-9]{2}
//         para padroes do tipo
//             000000000-00
//             00000000000
//         no fim, a string de regex deve ser algo do tipo
//     (([0-9]{3}[\.]{1}){2}[0-9]{3}[-][0-9]{2})|([0-9]{9}[-]?[0-9]{2})
//
//     Urgente: implementar um verificador para casos em que
//     todos os algarismos são iguais - o que é inválido, mas
//     que o algoritmo deixa passar

use std::cmp::Ordering;

pub struct CPF {
    cpf: String
} // fim da struct

impl CPF {
    // dica: use em instâncias mutáveis
    pub fn new() -> Self {
        CPF { cpf: String::new() }
    } // fim do construtor 


    // dica: use em instâncias imutáveis
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
    } // fim do método ehValido


    pub fn validarCPF(&self) -> Result<&'static str, &'static str> {
        self.tem11Digitos()?;
            Ok("Todos os algarismos passam no teste")
    } // fim do método validarCPF


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
        } // fim obterDigito


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
} // fim charPara_u32


fn digitoVezesIndice(enupla: (u32, usize)) -> u32 {
    let digito = enupla.0;
    let indice = enupla.1 as u32;

    digito*indice
} // fim digitoVezesIndice
