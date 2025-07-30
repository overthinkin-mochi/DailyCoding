# problema

## traducido

>[!NOTE]
>Este problema fue traducido con claude y transformado de python a rust.

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

---

## Original

Good morning! Here's your coding interview problem for today.

This problem was asked by Jane Street.

cons(a, b) constructs a pair, and car(pair) and cdr(pair) returns the first and last element of that pair. For example, car(cons(3, 4)) returns 3, and cdr(cons(3, 4)) returns 4.

Given this implementation of cons:

```python
def cons(a, b):
    def pair(f):
        return f(a, b)
    return pair
```

Implement car and cdr
