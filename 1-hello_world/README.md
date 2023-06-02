# Instalação
https://doc.rust-lang.org/book/ch01-01-installation.html

Linux ou macOS:

``` shell
$ curl --proto '=https' --tlsv1.2 https://sh.rustup.rs -sSf | sh

```

É necessário um *linker* que o Rust usa para unir as saídas compiladas é um arquivo. Instalando um compilador C, provavelmente o *linker* virá junto.

Para instalar o gcc no Linux:

``` shell
sudo apt-get update
sudo apt-get install gcc
```

Windows:

https://www.rust-lang.org/tools/install

# Hello, world!
https://doc.rust-lang.org/book/ch01-02-hello-world.html

Arquivos em Rust possuem a extensão *.rs*

**ver aquivo *main.rs***

Para compilar:

``` shell
rustc main.rs
```

O comando acima irá gerar um executável

Para executar:

``` shell
./main # ou .\main.exe no Windows
```
