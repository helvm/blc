# blc

**blc** is an implementation of the
[binary lambda calculus](https://esolangs.org/wiki/Binary_lambda_calculus).

## Binary lambda calculus basics

Binary lambda calculus (BLC) is a minimal, purely functional programming language based on a binary
encoding of the untyped [lambda calculus](https://en.wikipedia.org/wiki/Lambda_calculus) with
[De Bruijn indices](https://en.wikipedia.org/wiki/De_Bruijn_index).

Lambda terms have the following representation in BLC:

| term        | lambda | BLC            |
--------------|--------|----------------|
| abstraction | λM     | 00M            |
| application | MN     | 01MN           |
| variable    | i      | 1<sup>i</sup>0 |

Since BLC programs are basically lambda calculus terms, they can be applied to other terms. In
order for them to be applicable to binary (but not BLC-encoded) input, it has to be lambda-encoded
first. Bytestrings are lambda-encoded as
[single-pair lists](https://en.wikipedia.org/wiki/Church_encoding#One_pair_as_a_list_node) of bytes
and bytes are lambda-encoded as single-pair lists of lambda-encoded bits.

Bits 0 and 1 are lambda-encoded as
[Church booleans](https://en.wikipedia.org/wiki/Church_encoding#Church_Booleans):

| bit | lambda      | BLC     |
|-----|-------------|---------|
|  0  | λλ2 (true)  | 0000110 |
|  1  | λλ1 (false) | 000010  |

Example: BLC-encoding steps for a byte representing the ASCII/UTF-8 encoded letter 'a':

| encoding  | representation |
|-----------|----------------|
| decimal   | 96             |
| binary    | 01100001       |
| lambda    | λ1(<b>λλ2</b>)(λ1(<b>λλ1</b>)(λ1(<b>λλ1</b>)(λ1(<b>λλ2</b>)(λ1(<b>λλ2</b>)(λ1(<b>λλ2</b>)(λ1(<b>λλ2</b>)(λ1(<b>λλ1</b>)(λλ1)))))))) |
| BLC (hex) | 16 16 0c 2c 10 b0 42 c1 85 83 0b 06 16 0c 2c 10 41 00 |

## [Documentation](https://docs.rs/blc)

## Example BLC program

```
extern crate blc;

use blc::*;
use blc::encoding::binary::decompress;
use blc::execution::Input;

fn repeat() {
    let code_compressed = [0x16, 0x46, 0x80, 0x05, 0xbc, 0xbc, 0xfd, 0xf6, 0x80];
    let code_blc        = decompress(&code_compressed);

    assert_eq!(
        run(&*code_blc, Input::Bytes(&*b"hurr")).unwrap(),
        "hurrhurr"
    );
}
```
