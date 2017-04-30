//! Binary lambda calculus execution

use binary_encoding::{from_binary};
use lambda_encoding::{encode, decode};
use lambda_calculus::reduction::beta_full;
use self::Error::*;

/// An error that can occur during blc execution.
#[derive(Debug, PartialEq)]
pub enum Error {
    InvalidProgram
}

/// Executes a binary lambda calculus program, feeding it the given argument.
///
/// # Example
/// ```
/// use blc::execution::run;
///
/// let reverse = b"0001011001000110100000000001011100111110111100001011011110110000010";
///
/// assert_eq!(run(&*reverse, b"herp derp"), Ok("pred preh".into()));
/// ```
pub fn run(blc_program: &[u8], blc_argument: &[u8]) -> Result<String, Error> {
    let program = from_binary(blc_program);
    if program.is_err() { return Err(InvalidProgram) }
    let calculation = beta_full(program.unwrap().app(encode(blc_argument))); // safe

    Ok(decode(calculation))
}

#[cfg(test)]
mod test {
    use super::*;
    use binary_encoding::decompress;

    #[test]
    fn inflating() {
        // program code from http://www.ioccc.org/2012/tromp/inflate.Blc
        let inflate_compressed =
            [0x44, 0x44, 0x68, 0x16, 0x01, 0x79, 0x1a, 0x00, 0x16, 0x7f, 0xfb, 0xcb, 0xcf, 0xdf,
             0x65, 0xfb, 0xed, 0x0f, 0x3c, 0xe7, 0x3c, 0xf3, 0xc2, 0xd8, 0x20, 0x58, 0x2c, 0x0b,
             0x06, 0xc0];

        let inflate_blc = decompress(&inflate_compressed);
        let s_compressed = [0x1, 0x7a, 0x74];

        assert_eq!(run(&*inflate_blc, &s_compressed[..]).unwrap(), "000000010111101001110100".to_owned());
    }

    #[test]
    fn deflating() {
        // program code from http://www.ioccc.org/2012/tromp/deflate.Blc
        let deflate_compressed =
            [0x44, 0x68, 0x16, 0x05, 0x7e, 0x01, 0x17, 0x00, 0xbe, 0x55, 0xff, 0xf0, 0x0d, 0xc1,
             0x8b, 0xb2, 0xc1, 0xb0, 0xf8, 0x7c, 0x2d, 0xd8, 0x05, 0x9e, 0x09, 0x7f, 0xbf, 0xb1,
             0x48, 0x39, 0xce, 0x81, 0xce, 0x80];

        let deflate_blc = decompress(&deflate_compressed);
        let s_blc = b"00000001011110100111010";

        assert_eq!(run(&*deflate_blc, &s_blc[..]).unwrap().as_bytes(), [0x1, 0x7a, 0x74]);
    }

    #[test]
    fn brainfuck() {
        // program code from
        let bf_interpreter_compressed =
            [0x01, 0xa1, 0x51, 0x44, 0x02, 0xd5, 0x55, 0x84, 0x22, 0x30, 0x70, 0xb7, 0x00, 0xf0,
             0x32, 0xff, 0x7f, 0x85, 0xf9, 0xbf, 0x95, 0x6f, 0xe1, 0x5e, 0xc0, 0xee, 0x7d, 0x7f,
             0x00, 0x68, 0x54, 0xe5, 0xfb, 0xfd, 0x55, 0x58, 0xfd, 0x57, 0x45, 0xe0, 0xb6, 0xf0,
             0xfb, 0xeb, 0x07, 0xd6, 0x2f, 0xf0, 0xd7, 0x73, 0x6f, 0xe1, 0xc0, 0xbc, 0x14, 0xf1,
             0x1f, 0x2e, 0xff, 0x0b, 0x17, 0x66, 0x6f, 0xa1, 0x2f, 0xef, 0x5b, 0xe8, 0xff, 0x13,
             0xff, 0xcf, 0x20, 0x34, 0xca, 0xe1, 0x0b, 0xd0, 0xc8, 0x0a, 0xe5, 0x1f, 0xee, 0x99,
             0x6a, 0x5a, 0x7f, 0xff, 0xff, 0x0f, 0xff, 0x1f, 0xd0, 0x04, 0x9d, 0x87, 0xdb, 0x05,
             0x00, 0xab, 0x3b, 0xb7, 0x40, 0x23, 0xb0, 0xc0, 0xcc, 0x28, 0x10, 0x74, 0x0e, 0x6c];

        let bf_interpreter_blc = decompress(&bf_interpreter_compressed);
        let bf_hello = b"++++++++++[>+++++++>++++++++++>+++>+<<<<-]>++.>+.+++++++..\
                         +++.>++.<<+++++++++++++++.>.+++.------.--------.>+.>.]";

        assert_eq!(run(&bf_interpreter_blc, &bf_hello[..]), Ok("Hello World!".into()));
    }
}
