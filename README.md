# merge-junit
> Merging JUnit XML files.

## Installation

OS X & Linux:

```sh
$ git clone https://github.com/tobni/merge-junit
$ cd merge-junit
$ cargo build --release
$ ./target/release/merge-junit --version
<VERSION>
```

## Usage example

Some test runners run in parallel produce many small JUnit reports. 
Stitching them together to feed e.g. a GUI can increase readability.

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