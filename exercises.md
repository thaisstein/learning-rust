### 1.2 Formatted print exercises

- Rust’s `println!` macro supports positional arguments, allowing us to reference data by its index within the argument list. To resolve the "Fix the issue in the above code (see FIXME)" issue, I added "James" as the second argument (index = 1). This ensures the format string reffers the placeholder {} to the this value.

- In order to print "pi" with only 3 decimal cases, I defined pi with ` let pi = 3.141592;` and when referecing it in the placeholder according to the [documentation](https://doc.rust-lang.org/std/fmt/), I added `{:.3}`, specifying 3 decimal cases.

### 2.1 Literals and operators

- When trying to change `1i32` to `1u32` I get the following compilation error:    `^^^^^^^^ attempt to compute `1_u32 - 2_u32`, which would overflow`

### 2.2 Tuples 

