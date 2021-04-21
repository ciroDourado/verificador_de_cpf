fn main() {
    let cpf = String::from("123.456.789-01");
    
    let digitos = cpf.as_str().chars()
        .filter(|&char| char.is_ascii_digit())
        .collect::<String>();

    println!("numeros dados: {}", digitos.len()); 
} // fim da main


// para um cpf ser válido, ele deve conter:
// - 11 dígitos
// - desses, os 9 primeiros devem formar os 2 últimos


// a função de verificar cpf deve retornar um bool,
// pois como ela própria diz: ou o cpf é válido, ou
// não é


// demais métodos auxiliares devem ser dados, como:
// retornar uma instância de CPF formatado - Option
// retornar uma instância com apenas os dígitos - Option
