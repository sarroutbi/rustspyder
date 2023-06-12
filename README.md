[![Rust](https://github.com/sarroutbi/rustspyder/actions/workflows/rust.yaml/badge.svg)](https://github.com/sarroutbi/rustspyder/actions/workflows/rust.yaml)

# rustspyder

## Contents

- [Introduction](#introduction)
- [Configuration](#configuration)
- [Web Parsing and Navigation](#web-parsing-and-navigation)
- [Filters](#filters)
- [Concepts](#concepts)
- [Dumps](#dumps)

## Introduction
A web crawling robot, written in Rust.

## Configuration
Ideally, all configuration for the robot will be performed through configuration files. The unique parameters to provide to binary should be:

1. verbosity
2. help
3. configuration file

Preferably, the options to binary will be provided in both short and long options (-h|--help; -v|--verbose; -c|--configuration-file).

## Web Parsing And Navigation
Initially, rustspyder is thought to be used with "plain HTML" web pages. Navigating heavy `javascripted` sites is not the use case, although ideally in later phases should be handled similarly to plain HTML sites. Robot will perform easy mechanisms to navigate through different pages by configuration provided, so that crawler will be able to navigate.

## Filters
Web crawler will provide filters so that once a web page is retrieved, a filter can be provided to keep some of the information in the site that could be useful, such as a model, a price, a description, etc. A set of common filters (for price parsing, HTML elimination, model or description parsing, etc) will be available for user.

## Concepts
Crawler will provide a mechanism to store "concepts" associated to filters. Normally, a concept will match a filter. "Price" concept with "filter" regexp
would be saved so that concept can be later used (stored in database, stored in file, displayed in terminal, whatever).

## Dumps
Dumps will be mechanisms to dump each of the concepts, together with its value, to a particular output artifact (file, database, whatever).

As stated initially, all this should be handled via one or several configuration files (it will depend on design requirements).

## Compilation

Compilation in *rustspyder* is executed through *cargo* tool, as usual in Rust:

```bash
$ cargo build
```
For compilation in release mode, use --release flag:

```bash
$ cargo build --release
```

## Usage

```bash

$ ./target/debug/rustspyder -h

Usage: rustspyder [OPTIONS]

Options:
  -v, --verbose
  -c, --config <CONFIG>
  -h, --help             Print help
  -V, --version          Print version
```

## Tests

*rustspyder* includes minimal tests to check stability.
At this moment, only CLI function tests exist.
In the future, unit tests will be included too.
Tests in *rustspyder* are executed through *cargo* tool, as usual in Rust:

```bash

$ cargo test
...
     Running tests/rustspyder-test.rs (target/debug/deps/rustspyder_test-cbe3bccecde916b2)

running 3 tests
test verbose_check_test ... ok
test config_test ... ok
test help_check_options_test ... ok

test result: ok. 3 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s
```

## Coverage Tests
This section describes how to execute coverage tests.

- Install *grcov* and *llvm-tools*: First of all, installation of appropriate tools needs to be done:

```bash
$ cargo install grcov
$ rustup component add llvm-tools-preview
```

- Then, appropriate compilation flags need to be exported:

```bash
$ export RUSTFLAGS="-Cinstrument-coverage"
$ export LLVM_PROFILE_FILE="rustspyder-%p-%m.profraw"
```

- Finally, execute tests via *cargo test*, generate report with *grcov* (in HTML mode for this example) and open it with your preferred browser:

```bash
$ cargo test
$ grcov . -s . --binary-path ./target/debug/ -t html --branch --ignore-not-existing -o ./target/debug/coverage/
$ firefox ./target/debug/coverage/index.html
```
