use std::fs::File;
use std::env;

extern crate serde_derive;
extern crate serde_json;
extern crate serde;

use serde_derive::Deserialize;
use std::collections::HashSet;
use std::collections::HashMap;

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

    let header = args.len() > 2 && args[2] == "--header";
    let spv: Grammar = serde_json::from_reader(file).expect("file should be proper JSON");

    println!("// Auto generated - do not modify");

    // for elem in spv.instructions {
    //     println!("name {} opcode {}", elem.opname, elem.opcode);
    // }

    
    if header
    {
        println!("#pragma once\n");

        println!("#include \"Vector.h\"");
        println!("#include \"HashMap.h\"");
        println!("#include \"SpvDefines.h\"\n");

        println!("namespace spvgentwo\n{{");

        println!("class Grammar\n{{");

            let mut categories = HashSet::new();
            println!("\tenum class OperandCategory : unsigned short\n\t{{");
            for elem in &spv.operand_kinds {
                if !categories.contains(&elem.category)
                {
                    categories.insert(&elem.category);
                    println!("\t\t{},",elem.category);
                }
            }
            println!("\t}};");

            println!("\tenum class OperandKind : unsigned short\n\t{{");
            for elem in &spv.operand_kinds {
                println!("\t\t{},",elem.kind);
            }
            println!("\t}};");

            println!("\tenum class Quantifier\n\t{{");
                println!("\t\tOptional,");
                println!("\t\tAnyCount");   
            println!("\t}};");

            println!("\tstruct Operand\n\t{{");
                println!("\t\tOperandKind kind;");
                println!("\t\tOperandCategory category;");        
                println!("\t\tconst char* name;");
                println!("\t\tQuantifier quantifier;");
            println!("\t}};");

            println!("\tstruct Instruction\n\t{{");
                println!("\t\tconst char* name;");
                println!("\t\tspv::Op opcode;");
                println!("\t\tVector<Operand> operands;");
            println!("\t}};");

        println!("\tpublic:");
            println!("\t\tGrammar(IAllocator* _pAllocator);");
        println!("\tprivate:");
            //println!("\tVector<Instruction> m_instructions;");
            println!("\t\tHashMap<spv::Op, Instruction> m_instructions;");
        println!("}};");

        println!("}} // spvgentwo"); // namespace
    }
    else // cpp
    {
        println!("#include \"spvgentwo/Grammar.h\"\n");
        println!("using namespace spvgentwo;\n");

        let mut kinds = HashMap::new();
        for elem in &spv.operand_kinds {
            kinds.insert(&elem.kind, &elem.category);
        }

        println!("Grammar::Grammar(IAllocator* _pAllocator) : m_instructions(_pAllocator)\n{{");
        for instr in spv.instructions
        {
            println!("\tm_instructions.emplaceUnique(spv::Op::{}, Instruction{{\"{}\", spv::Op::{}, _pAllocator}});", instr.opname, instr.opname, instr.opname);
        }
        println!("}};"); // constructor
    }

    //let mut ofile = File::create("spvgrammer.inl").expect("unable to create file");
    //ofile.write_all(output.as_bytes()).expect("unable to write");
}
