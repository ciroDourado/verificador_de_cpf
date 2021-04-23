#![allow(non_snake_case, dead_code)]


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
         match self.obterDigitos().len() {
             11 => self.oPenultimoEhValido(),
             _  => Err("Não possui todos os 11 dígitos")
         }
    } // fim do método privado tem11Digitos


        fn obterDigitos(&self) -> String {
            let digitos = self.get().as_str().chars()
                .filter(|&char| char.is_ascii_digit())
                .collect::<String>();
            digitos
        } // fim obterDigitos


    fn oPenultimoEhValido(&self) -> Result<(), &'static str> {
        let penultimo = self.obterDigito(9);

        Err("O CPF não gera o penúltimo algarismo")
    } // fim do método privado oPenultimoEhValido


        fn obterDigito(&self, posicao: usize) -> u32 {
            self.get().as_str().chars()
                .nth(posicao).unwrap()
                .to_digit(10).unwrap()
        } // fim obterPenultimoDigito


    fn oUltimoEhValido(&self) -> Result<(), &'static str> {
        let ultimo = self.obterDigito(10);

        Err("O CPF não gera o último algarismo")
    } // fim do método privado oUltimoEhValido


} // fim dos métodos


fn main() {
    let cpfUsuario = CPF::from("529.982.247-25");

    match cpfUsuario.ehValido() {
        true  => println!("Este CPF é válido"),
        false => println!("O CPF foi dado incorretamente")
    }

    println!("{}", match cpfUsuario.validarCPF() {
        Ok(resultado) => resultado,
        Err(motivo)   => motivo
    });

    match cpfUsuario.validarCPF() {
        Ok(resultado) => println!("{}", resultado),
        Err(motivo)   => println!("{}", motivo)
    }

} // fim da main


//     para um cpf ser válido, ele deve conter:
//     - 11 dígitos
//     - desses, os 9 primeiros devem formar os 2 últimos
//
//     a função de verificar cpf deve retornar um bool,
//     pois como ela própria diz: ou o cpf é válido, ou
//     não é


// fn primeiroVerificador(cpf: String) -> u32 {
//     let elementos = digitosVezesIndices(cpf,2);
//     let soma = elementos.iter().sum::<u32>();
//
//     let resultado = (soma*10) % 11;
//     resultado
// } // fim primeiroVerificador
//
//
// fn digitosVezesIndices(cpf: String, skip: usize) -> Vec<u32> {
//     cpf.as_str().chars()
//         .rev().skip(skip)
//         .map(|char| char.to_digit(10).unwrap())
//         .zip( (2..).into_iter() )
//         .map(digitoVezesIndice)
//         .collect::<Vec<u32>>()
// } // fim digitosVezesIndices
//
//
// fn digitoVezesIndice(enupla: (u32, usize)) -> u32 {
//     let digito = enupla.0;
//     let indice = enupla.1 as u32;
//
//     digito*indice
// } // fim digitoVezesIndice
//
//
// fn segundoVerificador(cpf: String) -> u32 {
//     let elementos = digitosVezesIndices(cpf,1);
//     let soma = elementos.iter().sum::<u32>();
//
//     let resultado = (soma*10) % 11;
//     resultado
// } // fim primeiroVerificador
