use std::fs::File;
use std::env;

extern crate serde_derive;
extern crate serde_json;
extern crate serde;

use serde_derive::Deserialize;

#[derive(Deserialize)]
struct Grammar {
    magic_number: String,
    major_version: i32,
    minor_version: i32,
    revision: i32,
    operand_kinds: Vec<OperandKinds>
}

#[derive(Deserialize)]
struct OperandKinds {
    category: String,
    kind: String,
    enumerants: Vec<Enumerants>
}


#[derive(Deserialize)]
struct Enumerants {
    enumerant: String,
    value: String,
    capabilities: Option<Vec<String>>,
    parameters: Option<Vec<Parameter>>,
    extensions: Option<Vec<String>>,
    version: Option<String>
}

#[derive(Deserialize)]
struct Parameter
{
    kind: String
}


fn main() {
    let args: Vec<String> = env::args().collect();
    let file = File::open(&args[1]).expect("file should open read only");
    let spv: Grammar = serde_json::from_reader(file).unwrap();//.expect("file should be proper JSON");

    println!("magic {} major {}", spv.magic_number, spv.major_version);

    for elem in spv.operand_kinds {
        println!("cat {} kind {}", elem.category, elem.kind);
    }

    //println!("enum class OperandType : unsigned int \n {{\t");

    // let operands = match json["operand_kinds"] {
    //     serde_json::Array(ref v) => {
    //             formatter.write_str("Array(")?;
    //             Debug::fmt(v, formatter)?;
    //             formatter.write_str(")")
    //         }
    //         _ => {
    //             formatter.write_str("Object(")?;
    //             Debug::fmt(v, formatter)?;
    //             formatter.write_str(")")
    //         }
    // };


    //let mut ofile = File::create("spvgrammer.inl").expect("unable to create file");
    //ofile.write_all(output.as_bytes()).expect("unable to write");

}
