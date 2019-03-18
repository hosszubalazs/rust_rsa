# RSA key generation in Rust

[![Codacy Badge](https://api.codacy.com/project/badge/Grade/5d9e28f94c4d48e1824dde418bbd7692)](https://app.codacy.com/app/hosszubalazs/rust_rsa?utm_source=github.com&utm_medium=referral&utm_content=hosszubalazs/rust_rsa&utm_campaign=Badge_Grade_Settings)
[![Build Status](https://travis-ci.org/hosszubalazs/rust_rsa.svg?branch=master)](https://travis-ci.org/hosszubalazs/rust_rsa)

Example project for creating assymetric key cyphers in Rust based on the RSA algorithm.

The solution is heavily under development and is meant for educational purposes only. (mostly for myself) The current solution barely works for very low prime numbers.

## Kid-RSA

A bug-ridden early implementation of the RSA-like encryption, Kid-RSA, can be found in `kid_rsa.rs`. Please check the tests and the FIXME to see the current state.

Requirements:

- The private and public key are different.
- The message is smaller than n. The algortihm is mod n based, messages longer than n does not make sense.

## Development environment, runnin the code

Use the official guide to get your environment working: [Install Rust](https://www.rust-lang.org/tools/install)

Use standard `cargo` commands to build/test/run.

## Resources

Some resources used for this project:

- [RSA Algorithm in Cryptography](https://www.geeksforgeeks.org/rsa-algorithm-cryptography/)
- [Euler's totient function](https://simple.wikipedia.org/wiki/Euler%27s_totient_function)
- [RSA Algorithm](https://simple.wikipedia.org/wiki/RSA_algorithm)
- [Kid RSA](https://sites.math.washington.edu/~koblitz/crlogia.html)