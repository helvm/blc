//! **blc** is an implementation of the
//! [binary lambda calculus](https://esolangs.org/wiki/Binary_lambda_calculus) in Rust.
//! ## Binary lambda calculus basics
//!
//! Binary lambda calculus (BLC) is a minimal, purely functional programming language based on a binary
//! encoding of the untyped [lambda calculus](https://en.wikipedia.org/wiki/Lambda_calculus) with
//! [De Bruijn indices](https://en.wikipedia.org/wiki/De_Bruijn_index).
//!
//! Lambda terms have the following representation in BLC:
//!
//! | term        | lambda | BLC            |
//! --------------|--------|----------------|
//! | abstraction | λM     | 00M            |
//! | application | MN     | 01MN           |
//! | variable    | i      | 1<sup>i</sup>0 |
//!
//! Since BLC programs are basically lambda calculus terms, they can be applied to other terms. In
//! order to be applicable to binary (but not BLC-encoded) input, it has to be lambda-encoded first.
//! Bytestrings are lambda-encoded as
//! [Church lists](https://en.wikipedia.org/wiki/Church_encoding#One_pair_as_a_list_node) of bytes
//! and bytes are lambda-encoded as Church lists of lambda-encoded bits.
//!
//! Bits 0 and 1 are lambda-encoded as
//! [Church booleans](https://en.wikipedia.org/wiki/Church_encoding#Church_Booleans):
//!
//! | bit | lambda      | BLC     |
//! |-----|-------------|---------|
//! | 0   | λλ2 (true)  | 0000110 |
//! | 1   | λλ1 (false) | 000010  |
//!
//! Example: BLC-encoding steps for a byte representing the UTF-encoded letter 'a':
//!
//! | encoding | representation |
//! |----------|----------------|
//! | UTF-8    | 96 (decimal)   |
//! | binary   | 01100001       |
//! | lambda   | λ1(<b>λλ2</b>)(λ1(<b>λλ1</b>)(λ1(<b>λλ1</b>)(λ1(<b>λλ2</b>)(λ1(<b>λλ2</b>)(λ1(<b>λλ2</b>)(λ1(<b>λλ2</b>)(λ1(<b>λλ1</b>)(λλ1)))))))) |
//! | BLC      | 0001011000010110<b>0000110</b>00010110<b>000010</b>00010110<b>000010</b>00010110<b>0000110</b>00010110<b>0000110</b>00010110<b>0000110</b>00010110<b>0000110</b>00010110<b>000010</b>000010000010 |

extern crate lambda_calculus;

pub mod lambda_encoding;
pub mod binary_encoding;
pub mod execution;
