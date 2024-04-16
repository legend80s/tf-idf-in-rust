# TF-IDF in Rust 🦀

A local search engine use TF-IDF algorithm in Rust!

the searching materials are from <https://github.com/BSVino/docs.gl>

## Technical Details

It contains:

- a `Lexer` for word segmentation use `xml-rs` and
- `model` for calculating TF-IDF.
- a web interface for searching use `tiny_http` with input and results dispaying and a `/api/search` API.

And it's a CLI app hand-made without `clap` and with subcommands:

- index - Indexing tf-idf into json file.
- serve - Start a local server with web interface for searching.

## Start

1 git clone this repo
2 `cargo run index`
3 `cargo run serve`
