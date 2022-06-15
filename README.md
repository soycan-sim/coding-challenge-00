# Intra

The intergalactic numeric translator.

# Build instructions

1. Follow the instructions on [rust-lang.org](https://www.rust-lang.org/learn/get-started) to install the Rust programming language.
2. Run `cargo build` to build the library and executable. The first compilation can take a few minutes, as cargo will have to download and build all dependencies.
3. The executable will be located in `./target/debug/`, but it can also be ran with `cargo run`.
4. (Optional) Run `cargo test` to run all unit and integration tests.
5. (Optional) Run `cargo build --release` to compile an optimized build.

# Usage

Use `cargo run -- --help` to list the available options.
```
$ cargo run -- --help
intra 0.1.0
Simone Walter <waltersz@protonmail.com>
The intergalactic numeric translator

USAGE:
    intra [OPTIONS] [PATH]

ARGS:
    <PATH>    File to read from. Defaults to stdin

OPTIONS:
    -h, --help               Print help information
    -o, --output <OUTPUT>    File to write to. Defaults to stdout. Ignored if in interactive mode
    -V, --version            Print version information
```

Run with `cargo run -- ./tests/test.txt` to run the test cases in [tests/test.txt](./tests/test.txt). This will print to stdout.
The expected output is located in [tests/test.out.txt](./tests/test.out.txt).

Add the option `-o FILE` to print the output to a text file.

If an input file isn't provided, `intra` will be ran in interactive mode. Press CTRL-C or CTRL-D to exit.

```
$ cargo run
> glob is I
> glib is V
> how much is glib glob?
glib glob is 6
> what is glob?
I have no idea what you are talking about
>
^C
```

# Implementation details

Each line of input is read by an interpreter that will compare it to 4 known regexes.
When one of them is recognized, the interpreter will act accordingly by doing one of the following:
- add new word and its translation to the map of known words
- add new item and its price to the map of known items
- translate an intergalactic numeral to a roman numeral, convert it to decimal and print it out
- translate a numeral, lookup an item, calculate the total price and print it out

Roman numerals are stored as strings and validated with a regex when created.
They can be currently only be converted to unsigned 32-bit integers, but other integer conversion are trivial.

The language database is stored as a map of string -> char.

Prices are handled in a decimal format provided by the [`rust_decimal` crate](https://github.com/paupino/rust-decimal).

The interpreter recognizes different kinds of errors in queries, but in the executable they're all printed out as
"I have no idea what you are talking about". An error doesn't abort the program.

## Assumptions 

Whitespaces in input can be replaced by any number of any unicode whitespaces, but words must be separated by at least one whitespace.

Key words such as "how much" and "how many" are case-insensitive. "Credits" is also case-insensitive.
Intergalactic numerals must always be lowercase, and items must always be capitalized.
The reasoning for this is that the question "How many credits is glob glob Gold?" is recognized by a regex,
but if numbers were case-insensitive, the regex for recognizing numbers would greedily match "Gold" as well.
Without abandoning regex, I could either reserve the last word for the item or require that all items be capitalized.
With the first option, only one-word items could be sold. I decided for the second option,
as I myself wouldn't want to live in a world where I can't buy Soy milk with my glob glob credits.

## Future

If intra was required to scale, I would change the language hashmap and the item hashmap into SQL databases.
Hashmaps work for testing and are fast, but they don't scale well since they're stored in-memory.

Instead if SQL wasn't necessary but the language and item maps were required to be stored on disk,
[serde](https://crates.io/crates/serde) can be used to add de-/serialization to a type with a single line of code.

If more complicated queries were required, I would rewrite the interpreter to use a context-free grammar instead of regular grammar.
Regex is great for short queries whose form is known, but trying to handle language detection for hundreds of cases with regex would be impossible.
For even more complicated queries, a more complicated natural speech model would have to be used.

I wrote my own roman numeral validation and conversion, but in the future it should be handled by an external library.
One candidate could be [septem](https://crates.io/crates/septem). I only wrote my own roman numerals because I thought
it was simple enough to do in a day, but not trivial. I was right, it took me a few tries to get the validation regex right.

# License

This project is licensed under the [MIT license](./LICENSE).