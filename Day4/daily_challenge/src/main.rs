/*
¡Buenos días! Aquí tienes tu problema de entrevista de codificación para hoy.

Este problema fue planteado por Jane Street.

La función cons(a, b) construye un par, y car(pair) y cdr(pair) devuelven el
primer y el último elemento de ese par, respectivamente. Por ejemplo,
car(cons(3, 4)) devuelve 3, y cdr(cons(3, 4)) devuelve 4.

Dada esta implementación de cons en Rust:
Rust

```rust

fn cons<A, B>(a: A, b: B) -> Box<dyn Fn(Box<dyn Fn(A, B) -> R>) -> R>
where
    A: 'static,
    B: 'static,
    R: 'static, // Asume que R es el tipo de retorno general
{
    Box::new(move |f| {
        f(a, b)
    })
}
```

Implementa car y cdr.

*/

// this was a more appropiated representation
fn cons<T: Clone + 'static>(a: T, b: T) -> Box<dyn Fn(Box<dyn Fn(T, T) -> T>) -> T> {
    Box::new(move |f| f(a.clone(), b.clone()))
}

fn car<T: Clone + 'static>(pair: Box<dyn Fn(Box<dyn Fn(T, T) -> T>) -> T>) -> T {
    pair(Box::new(|a, _b| a))
}

fn cdr<T: Clone + 'static>(pair: Box<dyn Fn(Box<dyn Fn(T, T) -> T>) -> T>) -> T {
    pair(Box::new(|_a, b| b))
}
fn main() {
    let pair1 = cons(4, 5);
    let pair2 = cons(4, 5);

    let first_digit = car(pair1);
    let last_digit = cdr(pair2);

    println!("{}", first_digit);
    println!("{}", last_digit);
}
