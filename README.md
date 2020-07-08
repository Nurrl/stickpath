## Stickpath

Follow the path, not the white rabbit Alice ! :rabbit:

### Some explanations on what I did:

- The project was made to work with rust **stable**.

- Usage of the crate `derive_more`:
I decided to use this crate for error management without reimplementing the **Display** trait on my errors manually, it provides graceful ways to define errors.

- The project was formated with `cargo fmt` before commit.

- The project features a number of **0** unsafe `unwrap()`.

### How to test:

- **<-** Represents user inputs to `stdin`.
- **->** Represents program output to `stdout`.
