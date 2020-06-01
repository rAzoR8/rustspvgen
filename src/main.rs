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
    instructions: Vec<Instruction>,
    operand_kinds: Vec<OperandKinds>
}

#[derive(Deserialize)]
struct Instruction
{
    opname: String,
    class: String,
    opcode: u32,
    operands: Option<Vec<Operand>>
}

#[derive(Deserialize)]
struct Operand
{
    kind: String,
    quantifier: Option<String>,
    name: Option<String>
}

#[derive(Deserialize)]
struct OperandKinds {
    category: String,
    kind: String,
    enumerants: Option<Vec<Enumerants>>
}

#[derive(Deserialize)]
enum EnumValue
{
    hex(String),
    int(u32)
}

#[derive(Deserialize)]
struct Enumerants {
    enumerant: String,
    value: serde_json::Value,
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
    let spv: Grammar = serde_json::from_reader(file).expect("file should be proper JSON");

    println!("magic {} major {}", spv.magic_number, spv.major_version);

    // for elem in spv.instructions {
    //     println!("name {} opcode {}", elem.opname, elem.opcode);
    // }

    for elem in spv.operand_kinds {
        match elem.enumerants.as_ref()
        {
            Some(v) =>
            {
                println!("enum class {} : unsigned int \n{{", elem.kind);
                for enumval in v {
                    print!("\t{} = ",enumval.enumerant);
                    match &enumval.value
                    {
                        serde_json::Value::Number(x) => {print!("{},\n",x)},
                        serde_json::Value::String(s) => {print!("{},\n",s)}
                        _ => {}
                    }
                }
                println!("}};");

            },
            None => {}
        }
    }

    //let mut ofile = File::create("spvgrammer.inl").expect("unable to create file");
    //ofile.write_all(output.as_bytes()).expect("unable to write");
}
