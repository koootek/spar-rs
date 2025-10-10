# spar - simple args
inspired by [flag.h](https://github.com/tsoding/flag.h)

This crate supports Rust edition >=2018

## How to use
1. Copy `spar.rs` to your project.
2. Import spar to your main file. (`mod spar;`)
3. Create a new flag by calling `flag_*`. It returns a static borrow to a Flag. WARNING: YOU are responsible for duplicate flag names.
4. Call `spar::parse_args(&mut std::env::args())`.
5. Access the flag value using borrowed Flag from the third step.

## Flag syntax
1. Boolean

    \-name
2. Numbers

    \-name value
3. String

    \-name value | \-name "value"
4. Ignoring flags

    \-/name [value]

