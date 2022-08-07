# merge-junit
> Merging JUnit XML files.

## Installation

From source:

```sh
$ git clone https://github.com/tobni/merge-junit
$ cd merge-junit
$ cargo build --release
$ ./target/release/merge-junit --version
merge-junit 0.1.0
```

## Usage example

Some test runners run in parallel and produce many small JUnit reports.
Stitching them together to feed a GUI / reporting application can increase readability.

```sh
$ merge-junit *.xml -o merged-tests.xml
```

## Development setup

Uses [`rustfmt`](https://github.com/rust-lang/rustfmt) and [`clippy`](https://github.com/rust-lang/rust-clippy).

```sh
$ git clone https://github.com/tobni/merge-junit
$ cd merge-junit
$ cargo test
```