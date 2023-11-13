/*
    Required Imports For Class
    Provided from assignment documentation
*/
use std::env;
pub use array2::Array2;
mod codec;
pub mod format;
pub mod value_conversion;
use codec::{compress, decompress};

fn main() {
    let args: Vec<String> = env::args().collect();
    let argnum = args.len();
    assert!(argnum == 2 || argnum == 3);
    let filename = args.iter().nth(2).unwrap();
    match args[1].as_str() {
        "-c" => compress(Some(filename)),
        "-d" => decompress(Some(filename)),
        _ => {
            eprintln!("Usage: rpeg -d [filename]\nrpeg -c [filename]")
        }
    }
}