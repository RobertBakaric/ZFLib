# ZFLib
Fast(Q/A) compression library

[![License](https://img.shields.io/badge/license-MIT-blue.svg)](https://github.com/RobertBakaric/susq-rust/blob/master/LICENSE)

Short description

## Installation

To install lztq, first install Rust (> v1.38) and g++ (> v8.0.0 ). lzt is currently tested on Rust 1.39.0, but it is likely to work on other subsequent versions as well.

To install library:

```
cargo install ltzq
```

To create binary:

```
cargo build ltzq
```

## Usage
```

  lztq -h

#compress

  lztq -i file.fq -o file.fq.lzt -a c

#decompress
  lztq -i file.fq.lzt -a d -o file.fq

#extract (random access)
  lztq -i file.fq.lzt -a e -o file.fq -l list.csv -d bi
  lztq -i file.fq.lzt -a e -o file.fq -l "rand(15)" -d fwd


```


## License

The software is licensed under the  [MIT license](http://opensource.org/licenses/MIT).
