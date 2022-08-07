# merge-junit
> Merging JUnit XML files.

## Installation

From source:

```sh
$ git clone https://github.com/tobni/merge-junit
$ cd merge-junit
$ cargo build --release
$ ./target/release/merge-junit --version
<VERSION>
```

## Usage example

Some test runners run in parallel and produce many small JUnit reports.
Stitching them together to feed a GUI / reporting application can increase readability.

```sh
$ merge-junit *.xml -o merged-tests.xml
```

## Development setup

Uses `rustfmt` and `clippy`.

```sh
$ git clone https://github.com/tobni/merge-junit
$ cd merge-junit
$ cargo test
```