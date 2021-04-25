# verificador_de_cpf

### Resumo:<br>
O código deste repositório é uma ferramenta para verificar uma String que representa o CPF de uma pessoa.<br>
RegEx foram usadas para validar CPF's nos seguintes padrões:<br>
* xxxxxxxxxxx
* xxxxxxxxx-xx
* xxx.xxx.xxx-xx

Qualquer coisa fora disso, com mais ou menos dígitos, será desconsiderado.<br>


### Utilizando a Crate (módulo):<br>
Caso queira usar como dependência em um projeto, apenas inclua em seu Cargo.toml:
```
...

[dependencies]
cpf = {git="https://github.com/ciroDourado/verificador_de_cpf"}
```
E em seu código-fonte:
```
use cpf::CPF;
```
