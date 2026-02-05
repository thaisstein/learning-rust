# Rust by Example

## 1. Hello World
This is a simple "hello world" in Rust (and my first step going down the rabbit hole \o/) but there's an interesting catch.

`println!` might look like the C `printlf` function, but it's actually not a function - it's a *macro*. 

What this means is instead of being a pre-compiled "piece" of the code that we can access at runtime, it's like a "recipe". Whenever the compiler sees `!` it replaces that line with low-level code. There's a few advantages in doing this like:
- catching mistakes early (since we're accessing the macro's code during compilation)

- faster at runtime (we dont have to "go back" to the function)

- flexible (it can take any number of arguments)

### 1.1. Comments
Like C we can have line // or block comments (/* */).

### 1.2. Formatted print

- Rust’s `println!` macro supports positional arguments, allowing us to reference data by its index within the argument list. To resolve the "Fix the issue in the above code (see FIXME)" issue, I added "James" as the second argument (index = 1). This ensures the format string reffers the placeholder {} to the this value.

- In order to print "pi" with only 3 decimal cases, I defined pi with ` let pi = 3.141592;` and when referecing it in the placeholder according to the [documentation](https://doc.rust-lang.org/std/fmt/), I added `{:.3}`, specifying 3 decimal cases.
#### 1.2.1. Debug

## 2. Primitives

There's different types of **primitives** in Rust, which are the building blocks of the language. We call them primitives because, unlike variables, we are in control of how our data should be handled by the CPU choosing how many bits we need for each task. 

In the code, they are declared with `let` before their name and they live in a block (the space between opening and closing cursly braces), i.e., they do not exist outside that block.

- **Scalar Types:**

- **Integers** (signed, unsigned). Ex: `i8` is a signed integer where 8 is the # of bits; `usize` is an unsigned integer where "size" on a 64 bit machine would be 64. We can pre-define their size but if we don't, the default size is 32.
- **Floating points**: `f32` (carries 32 bits) and `f64` (carries 64 bits, more accurate than the former). Their default size is 64.
- **Char**: unicode character (I found it interesting that unlike C, chars in Rust have 4 bytes each which can be useful for writing accented letters, etc.)
- Bool (true or false)
- Unit type (represents an empty value, returns a `()`)

- **Compound Types:**

- Arrays (Like we know and adore. Holds elements from the same type and has fixed lenght).
- Tuples (('this', 'is', 'a', 'tuple', true)). Fixed lenght, can hold different types. 

Fun fact: Coming from C, the biggest shock is the lack of C-style loops. Turns out I can't print array elements with for (int i = 0; i<,10; i++). I looked up on why and it's because it's a common "off-by-one" errors/buffer overflows. So true. Way to go Rust.

- **On Type Annotation**

We can tell Rust the exact type we want to give a primitive (type annotation) in 2 different ways:
- Regular (before the variable's value): `let a_float: f64 = 1.0;`
- Suffix (after variable designation): `let an_integer = 5i32;`
We can also *not* tell Rust the type we want. It can be inferred from context like. In `primitive.rs`, `inferred_type` will be i64, even though I didn't specify it when declarying the primitive.

- **On Mutability:**

Note that I declared `inferred_type` with `let mut` before it. This happens because by default, Rust variables are **immutable**, i.e. we can't change them. With `mut` we are specifying we want to change this value somewhere along the code. 

What we *can't* do, even when using `mut`, is change the primitive's *type*, unless we redefine it. This can be done by declaring a primitive with the same name as a previous one using the `let` keyword again and is called **shadowing** (what I did with `mutable`).

### 2.1. Literals and operators

- When trying to change `1i32` to `1u32` I get the following compilation error:    `^^^^^^^^ attempt to compute `1_u32 - 2_u32`, which would overflow`

### 2.2. Tuples 


## 03. Custom types

Like C, we have custom datatypes:
- `struct`
- `enum`
While struct is an "and", enum is an "or"

`const` and `static` allows you to create values that exist for the entire duration of the program. `const` is a value fixed at compile-time (compiler copies and past its value where it's used) while `static` is a single instance of data on a specific spot in memory (when using it, program points to the exact memory address).
