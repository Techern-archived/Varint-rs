# Varint-rs
##Variable-length integer implementation in Rust

[![Build Status](https://travis-ci.org/Techern/Varint-rs.svg?branch=master)](https://travis-ci.org/Techern/Varint-rs)  [![Crates.io](https://img.shields.io/crates/v/varint.svg)](https://crates.io/crates/varint)
[![Coverage Status](https://coveralls.io/repos/Techern/Varint-rs/badge.svg?branch=master&service=github)](https://coveralls.io/github/Techern/Varint-rs?branch=master)

###Notes for 0.9

 * Reading and writing 64-bit Varints is currently unsupported, but will be added in 1.0 after testing current functionality
 * If you're using the io-operations feature for IoOperations 0.2 integration, please note that it will be refactored in the next release.
 * Real-world tests are still in progress. Theoretically, however, everything **should** work.
