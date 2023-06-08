# Algumas anotações do que foi usado no capítulo 2 do Livro

Foi utilizada a biblioteca *io* definida na biblioteca padrão. Rust tem por padrão um conjunto de itens que são definidos na biblioteca padrão e que estão inseridos no escopo de todo programa. Esse conjunto é chamado de *prelude*.

Items que não estão no *prelude* precisam ser adicionados explicitamente ao escopo do programa, como foi feito com a biblioteca *io*

Sintaxe de declaração de função:

``` Rust
fn nome_funcao(){}
```

Sintaxe de criação de variável:

``` Rust
let apples = 5;
```

Em Rust, as variáveis são imutáveis por padrão, ou seja, uma vez atribuido um valor, ele não irá mudar.

Sintaxe para variáveis mutáveis:

``` Rust
let mut banana = 5;
```

A sintaxe *::* indica que *new* é uma função associada ao tipo *String* (análodo a função estática)

A linha a seguir exemplifica o uso de referências, característica muito importante e complexa da Rust Language:

``` Rust
io::stdin()
        .read_line(&mut guess)
        .expect("Failed to read line");
```

Acima, a funcionalidade input/output é utilizada para conctenar a entrada do usuário à referência passada.

Rust sempre usa referências, evitando a criação de cópias na memória. Referências são indicadas por *&*

Referências são imutáveis por padrão. Para que seja possível (e deixe explícito) que a refrência será alterada, é necessário explicitar que ela é mutável, como acontece acima (*&mut*).

*read_line* também retorna o tipo *Result* que serve para tratar possíveis erros (variante *Err*). *expect* é um método definido no tipo *Result*, e em caso de variante *Ok*, retorna o valor de dentro. Em caso de variante *Err* causa um crash

Se *Result* é retornado e não há tratameto para suas variantes, cargo irá mostrar um *warnning*

Interpolação de variável(*{variavel}*) e de expressão (*{ }*):

``` Rust
let x = 5;
let y = 10;

println!("x = {x} and y + 2 = {}", y + 2);
```

## Uso de crate externa

Crates são pacotes em Rust.

São definidas na seção *[dependencies]* no arquivo *Cargo.toml*

As dependencias são baixadas de *regsitry* que é uma cópia de dados de *Crates.io*, onde a comunidade coloca suas crates

As dependências são baixadas ao ultilzar o comando

``` shel
cargo build
```


## Sistema de tipagem forte e estática, mas com inferência de tipos

## Uso de shadowing