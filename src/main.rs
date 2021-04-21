fn main() {
    let cpf = String::from("123.456.789-01");
    
    let digitos = cpf.as_str().chars()
        .filter(|&char| char.is_ascii_digit())
        .collect::<String>();

    println!("numeros dados: {}", digitos.len()); 
}
