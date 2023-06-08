# Hello, cargo!
https://doc.rust-lang.org/book/ch01-03-hello-cargo.html

Cargo é o construto do sistema e o gerenciador de pacotes do Rust.

Ele Já vem instalado junto com o Rust (se a instalação foi feita de outra maneira, é necessário verificar se foi instalado)

``` shell
cargo --version
```

## Criar projeto com Cargo

``` shell
cargo new hello_cargo
```

Com o comando acima, um projeto de nome *hello_cargo* e um diretório de mesmo nome são criados. Dentro do diretório, foi criado outro de nome *src* com o arquivo *main.rs* dentro, e o carquivo *Cargo.toml* (análogo ao *package.json*)

O aquivo *Cargo.toml* possui configurações do projeto e suas dependências:

``` toml
[package]
name = "hello_cargo"
version = "0.1.0"
edition = "2021"

# See more keys and their definitions at https://doc.rust-lang.org/cargo/reference/manifest.html

[dependencies]

```

Por fim, o arquivo *main.rs* é um hello_world como criado no capítulo anterior

## Construindo e executando um projeto Cargo

Construir:

``` shell
cargo build
```

O comando acima cria um executável em *target/debug/hello_cargo*, já que o build padrão é para debug

Esse comando também gera o arquivo *Cargo.lock* análogo ao *package-lock.json*

Para executa:

``` shell
./target/debug/hello_cargo
```

ou

```
cargo run
```

O comando acima também compila o projeto caso ele tenha sido modificado

```
cargo check
```

Verifica se o programa compila sem erros, mas sem precisar executar a etapa de construção novamente do projeto

### Construir para release

``` shell
cargo run --realease
```

Constrói com otimizações em *target/release*



Documentação Cargo: https://doc.rust-lang.org/cargo/