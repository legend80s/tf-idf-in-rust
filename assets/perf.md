### indexing

#### debug un-buffered

  Indexed folder "../docs.gl/" of 1607 files costs 13.806569875s
  Saving to "assets/index.json" costs 2.329292625s
  cargo run index  8.53s user 7.40s system 94% cpu 16.802 total

#### release un-buffered

  Indexed folder "../docs.gl/" of 1607 files costs 8.76874575s
  Saving to "assets/index.json" costs 2.24307425s
  target/release/tf-idf-in-rust index  3.32s user 7.41s system 97% cpu 11.028 total

#### debug buffered

  Indexed folder "../docs.gl/" of 1607 files costs 7.058619625s
  Saving to "assets/index.json" costs 84.249042ms
  cargo run index  6.91s user 0.11s system 92% cpu 7.570 total

#### release buffered

Indexed folder "../docs.gl/" of 1607 files costs 1.188443625s
Saving to "assets/index.json" costs 8.790792ms
target/release/tf-idf-in-rust index  1.00s user 0.06s system 64% cpu 1.648 total

### search

debug
  8s
release
  2s
