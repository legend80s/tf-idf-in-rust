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

`gsls function linearly interpolation`

#### debug without cache df

INFO: searching gsls function linearly interpolation
read_term_freq_index_from_file costs: 457.0605ms
calculate tf-idf costs: 4.53351175s
sorting costs: 3.420166ms

INFO: searching gsls function linearly interpolation
read_term_freq_index_from_file costs: 460.264417ms
calculate tf-idf costs: 77.582ms
sorting costs: 3.376042ms

"../docs.gl/el3/smoothstep.xhtml" => 0.041195642
"../docs.gl/sl4/smoothstep.xhtml" => 0.036167063
"../docs.gl/es1/glHint.xhtml" => 0.019645251
"../docs.gl/el3/mix.xhtml" => 0.01881235
"../docs.gl/gl2/glHint.xhtml" => 0.014087771
"../docs.gl/sl4/mix.xhtml" => 0.013692923
"../docs.gl/gl2/glGetMaterial.xhtml" => 0.011883816
"../docs.gl/gl3/glBlitFramebuffer.xhtml" => 0.009733891
"../docs.gl/es3/glBlitFramebuffer.xhtml" => 0.009045778
"../docs.gl/el3/step.xhtml" => 0.008715669

#### debug with cache df

#### release  without cache df

  2s

#### release with cache

  123ms
INFO: searching glsl function hello linealy interpolation
read_term_freq_index_from_file costs: 100.570125ms
calculate tf-idf costs: 9.988667ms
sorting costs: 2.775084ms

"../docs.gl/el3/smoothstep.xhtml" => 0.041195642
"../docs.gl/sl4/smoothstep.xhtml" => 0.036167063
"../docs.gl/es1/glHint.xhtml" => 0.019645251
"../docs.gl/gl2/glHint.xhtml" => 0.014087771
"../docs.gl/el3/mix.xhtml" => 0.011210989
"../docs.gl/gl3/glBlitFramebuffer.xhtml" => 0.009733891
"../docs.gl/es2/glGetString.xhtml" => 0.009056542
"../docs.gl/es3/glBlitFramebuffer.xhtml" => 0.009045778
"../docs.gl/el3/step.xhtml" => 0.008715669
"../docs.gl/gl2/gluNurbsCallback.xhtml" => 0.008667868
