Esse Capítulo trata das similaridades entre Rust e outras Linguagens
https://rust-book.cs.brown.edu/ch03-00-common-programming-concepts.html

# Variáveis e mutabilidade

Variáveis são imutáveis por padrão. Uma vez atribuído um valor, não será possível alterar.

``` Rust
let x = 5;
println!("The value of x is: {x}");
let x = 6;
println!("The value of x is: {x}");
```

 O código acima dá erro de compilação porque o compilador Rust verifica se as variáveis imutáveis continuam imutáveis por todo o código

 Para que uma variável seja mutável, precisa utilizar a palavra-chave ```mut``` :

``` Rust
let x = 5;
println!("The value of x is: {x}");
let x = 6;
println!("The value of x is: {x}");
```

## Constantes

Declaradas com ```cosnt``` no lugar de ```let```

**ORIGATÓRIO** informar o tipo de dado da constante

Não permitem uso de ```mut```

Podem ser declaradas no escopo glogal, diferentemente de ```let```

Por convensão, utiliza-se letras maiúsculas e underlines

Podem ser o resultado de uma expressão simples, não definida em tempo de computação. Mais sobre isso [aqui](https://doc.rust-lang.org/reference/const_eval.html)

``` Rust
const THREE_HOURS_IN_SECONDS:u32 = 60 * 60 * 3;
```

## Sombreamento (Shadowing)

Redeclarar(sobrear) um variável

Sombreamento dentro de um escopo só vale dentre daquele escopo ou para escopos mais internos

``` Rust
fn main(){
    let x = 5;

    let x = x + 1; // Sombreando o primeiro x

    {
        let x = x * 2; // Sombreando novamente, mas em um novo escopo
        println!("The value of x is: {x}"); // x aqui tem valor de 12
    }

    println!("The value of x is: {x}"); // Aqui o x volta a ter o valor 6
}
```

Um sobreamento pode mudar o tipo da variável:

``` Rust
fn main() {
    let spaces = "     ";
    let spaces = spaces.len(); // Sombreando e mudando o tipo de dados da variável
}
```

# Tipos de dados

Rust é uma linguagem **estaticamente tipada** e também pode **inferir** o tipo de acordo com o valor atribuído e o uso.

Em casos onde uma expressão pode assumir vários valores, é necessário informa o tipoe:

``` Rust
let guess: u32 = "42".parse().expect("Not a number");
```

## Tipos escalares

Representam um único valor: **integers**, **float-point numbers**, **Booleans** e **characters**

### Tipos inteiros

Número sem uma componente fracional

Tipos de inteiros:

| Length | Signed | Unsigned |
| ------ | ------ | -------- |
| 8-bit	 |   i8   |    u8    |
| 16-bit |	i16   |    u16   |
| 32-bit |	i32   |	   u32   |
| 64-bit |	i64	  |    u64   |
| 128-bit|	i128  |    u128  |
| arch	 |  isize |	  usize  |

Inteiros literais em Rust:

| Number literals | Example |
| --------------- | ------- |
| Decimal         | 98_222  |
|  Hex	          | 0xff    |
| Octal           | 0o77    |
| Binary	      | 0b1111_0000 |
| Byte (u8 only)  |	b'A'    |

O tipo padrão de interiro é **i32**

### Tipo ponto-flutuante

Exitem dois tipos: **f32** (32 bits) e **f64** (64 bits)

**f64** é o tipo flutuante padrão

Ambos são **signed**

``` Rust
fn main() {
    let x = 2.0; // f64

    let y: f32 = 3.0; // f32
}
```

#### Operações numéricas 

```Rust
fn main() {
    // addition
    let sum = 5 + 10;

    // subtraction
    let difference = 95.5 - 4.3;

    // multiplication
    let product = 4 * 30;

    // division
    let quotient = 56.7 / 32.2;
    let floored = 2 / 3; // Results in 0

    // remainder
    let remainder = 43 % 5;
}
```

### Tipo Booleano

Dois valores possíveis: **true** e **false**

Tamanho: **1 byte**

Especificado utilizando ```bool```

``` Rust
fn main() {
    let t = true;

    let f: bool = false; // with explicit type annotation
}
```

### O tipo Character

Tamanho: **4 bytes**

Definido com aspa simples (aspas duplas são para strings)

Representa: **Unicode scalar Values** (vai além das representações ASCII)

## Tipos compostos

Agrupa múltiplos valores em um

**tuples** e **arrays**

### O tipo Tuple

Possuem tamanho fixo: uma vez declarada, não aumentam nem diminuem

Declaradas como valores separados por vírgula dentre de parenteses

O nome da variável é a associada a toda ela

Para aceesar os valores independentes utiliza-se o *destructuring* ou o ponto

``` Rust
fn main() {
    let tup: (i32, f64, u8) = (500, 6.4, 1);

    let (x, y, z) = tup;

    let first = tup.1;
    let second = tup.2;
    let third = tup.3;
}
```

A tupla sem valor possui um nome especial: *unit*. Esse valor e seu tipo correspondente são ambos escritos da mesma forma: ```()```

### O tipo array

Todos os valores devem ser do mesmo tipo

Tamanho fixo após a declaração

```Rust
fn main() {
    let months = ["January", "February", "March", "April", "May", "June", "July",
              "August", "September", "October", "November", "December"];

    let a: [i32, 5] = [1, 2, 3, 4, 5]; // explicitando o tipo

    let first = a[0];
    let second = a[1];

    let b = [3; 5]; // o mesmo que: let b = [3, 3, 3, 3, 3];
}
```

Para um array com tamanho variável, é melhor usar *vector* ([capítulo 8](https://rust-book.cs.brown.edu/ch08-01-vectors.html))

# Funções

Utiliza *snake_case* como convensão para nomeação de funções e variáveis, ou seja: letras minúculas separadas por underline

Definição de função com ```fn```:

``` Rust
fn main() {
    another_function();
}

fn another_function() {
    println!("Function called");
}
```

Os tipos de cada parâmetros da função devem ser especificados:`

``` Rust
fn main() {
    another_function(5);
}

fn another_function(x: i32) {
    println!("The value of x is: {x}");
}
```

## Expressões vs declarações

Expressões evoluem para um valor

Declarações, como declarações de variáveis e funções, não retornam nenhum valor (diferentemente de C e Ruby)

Chamada de função, chamada de macro e um escopo criado por chaves são exemplos de expressões

Uma expressão pode compor uma declaração:

``` Rust
fn main() {
    let y = {
        let x = 5;
        x + 1 // Expressões não possuem ; no final
    };
}
```

## Funções com valores de retorno

``` Rust
fn five() -> u32{
    5 //valore retornado não tem ;
}

fn main() {
    let x = five();
    println!("{x}");
}
```

É necessário especificar o tipo do valor retornardo

```return``` pode ser usado, principalmente para early return

# Controle e fluxo

## if

``` Rust
fn main() {
    let number = 5;

    if number > 5 {
        println!("number is greater than 5");
    } else if number < 5 {
        println!("number is less than 5");
    } else {
        println!("number is equal to 5");
    }
}
```

### if em declarações let

``` Rust
fn main() {
    let condition = true;
    let x = if condition {5} else {6};

    println!("The value of x is: {x}");
}
```

Os valores dos blocos `if` e `else` devem ser do mesmo tipo

## Repetição com loops

Rust tem 3 tipos de loos: `loop`, `while` e `for`

### loop

Repete a execução do bloco de códigos indefinidament até que o programa seja encerrado, ou até que a instrução `break` seja executada

`continue` pode ser utilizada para pular uma iteração

``` Rust
fn main() {
    loop {
        println!("Again!");
    }
}
```

#### Retornado valores pelo

Basta adcionar a expressão que dere ser retornada após o `break`:

``` Rust
fn main() {
    let mut counter = 0;

    let result = loop {
        counter += 1;

        if counter == 10 {
            break counter * 2;
        }
    }

    println!("The result is: {result}");
}
```

#### loop com labels

É possível definir labels para loop's. Útil para especificar a qual loop as instruções `break` e `continue` devem ser aplicadas quando há loop's aninhados

``` Rust
fn main() {
    let mut count = 0;

    'counting_up: loop {
        println!("count: {count}");
        let mut remaining = 10;

        loop {
            println!("remaining = {remaining}");

            if remaining == 9 {
                break;
            }
            if count == 2 {
                break 'counting_up;
            }
            remaining -= 1;
        }

        count += 1;
    }
}
```

## Repetição com while

``` Rust
fn main() {
    let mut dec_count = 5;

    while dec_count > 0 {
        println!("dec_count: {dec_count}");

        dec_count -= 1;

    }
}
```

## Repetição com for

### for em uma coleção

``` Rust
fn main() {
    let a = [10, 20, 30, 40, 50];

    for element in a {
        println!("The value is: {element}");
    }
}
```

### for em um intervalo (Range)

```Rust
fn main() {
    let a = [10, 20, 30, 40, 50];

    for index in (1..4).rev() {
        println!("On index {index}: {}", a[index]);
    }
}
```
