# Varint-rs   
[\[API Documentation\]](http://techern.github.io/Varint-rs)  
##Variable-length integer implementation in Rust

[![Build Status](https://travis-ci.org/Techern/Varint-rs.svg?branch=master)](https://travis-ci.org/Techern/Varint-rs)  [![Crates.io](https://img.shields.io/crates/v/varint.svg)](https://crates.io/crates/varint)
[![Coverage Status](https://coveralls.io/repos/Techern/Varint-rs/badge.svg?branch=master&service=github)](https://coveralls.io/github/Techern/Varint-rs?branch=master)

###Notes for 0.9

 * Reading and writing 64-bit Varints is currently unsupported, but will be added in 1.0 after testing current functionality
 * If you're using the io-operations feature for IoOperations 0.2 integration, please note that it will be refactored in the next release.
 * Real-world tests are still in progress. Theoretically, however, everything **should** work.

###List of feature flags

 * ```io-operations``` - Preliminary integration with the IO-Operations library

###Examples

```rust
extern crate varint;
use varint::{ VarintRead, VarintWrite }; //Using IO-Operations? Replace with VarintReader and VarintWriter, the functions are the same

use std::io::Cursor; //Currently supports Cursor<Vec<u8>> and TcpStream, but should be okay to implement in any Read/Write trait

let mut vector = Cursor::new(vec![0u8; 0]);

assert!(vector.write_signed_varint_32(2346784).is_ok()); //You can check this however you like. Try! could work. I'll check it out in Netherrack and update this by 1.0

//Do whatever you need to do.
vector.set_position(0); //If you're using a TcpStream, you'd probably switch sides and into a different codebase. This is just a quick example before I fall asleep :)

assert_eq!(2346784, vector.read_signed_varint_32().unwrap()); //You could also use try! here. Again, I'll test it in a real world project and update later

```
