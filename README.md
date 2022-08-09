# merge-junit
> Merging JUnit XML files.

## Installation

Via npm:
```sh
$ npm i merge-junit
$ npx merge-junit --version
merge-junit 0.1.4
```

Downloading binaries (e.g Linux):
```sh
$ curl -L https://github.com/tobni/merge-junit/releases/download/v0.1.4/merge-junit-v0.1.4-x86_64-unknown-linux-musl.tar.gz | tar -xz
$ ./merge-junit-v0.1.4-x86_64-unknown-linux-musl/merge-junit --version
merge-junit 0.1.4
```

From source:
```sh
$ git clone https://github.com/tobni/merge-junit
$ cd merge-junit
$ cargo build --release
$ ./target/release/merge-junit --version
merge-junit 0.1.4
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