### indexing

debug un-buffered
  Indexed folder "../docs.gl/" of 1607 files costs 13.806569875s
  Saving to "assets/index.json" costs 2.329292625s
  cargo run index  8.53s user 7.40s system 94% cpu 16.802 total

release un-buffered
  Indexed folder "../docs.gl/" of 1607 files costs 8.76874575s
  Saving to "assets/index.json" costs 2.24307425s
  target/release/tf-idf-in-rust index  3.32s user 7.41s system 97% cpu 11.028 total

### search

debug
  8s
release
  2s
