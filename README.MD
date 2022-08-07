# Random Number Guessing Game

## Introdução

Este projeto faz parte de minha jornada de aprendizado na linguagem Rust. O programa consiste de um jogo de adivinhação de número aleatório na CLI. O jogo aceitará entradas até que o número aleatório entre 1 e 100 seja descoberto.

## Conceitos

Durante o desenvolvimento deste programa, segui como referência o capítulo 2 do `The Book`, livro encontrado na documentação oficial da Rustlang. Os conceitos aplicados foram:

- Váriaveis mutáveis e imutáveis
- Cargo e gerenciamento de pacotes
- Uso da crate `rand` e seu trait `rng`
- Implementação do `Loop` de jogo
- Match statements juntamente com `std::cmp::Ordering`
- `std::io` e o método `stdin()`, com seu método relacionado `read_line()`
- Conversão de string para u32 utilizando o conceito de shadowing com os métodos `trim()` e `parse()`
- Criação de e tipagem de variáveis utilizando `String::new()`
- Tratamento de erros com `expect()` e match statements

## Como jogar

Para rodar o jogo, é necessário ter instalado o Rust e o Cargo, seu gerenciador de pacotes, o que pode ser realizado na página oficial da linguagem. Após isso, basta rodar dentro do diretório do jogo o comando `cargo run`
