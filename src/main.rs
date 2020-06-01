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
    major_version: u32,
    minor_version: u32,
    revision: u32,
    instructions: Vec<Instruction>,
    operand_kinds: Vec<OperandKinds>
}

#[derive(Deserialize)]
struct Instruction
{
    opname: String,
    class: String,
    opcode: u32,
    operands: Option<Vec<Operand>>,
    capabilities: Option<Vec<String>>,
    extensions: Option<Vec<String>>,
    version: Option<String>
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

    let defs = args.len() > 2 && args[2] == "--defs";
    let header = args.len() > 2 && args[2] == "--header";
    let cpp = args.len() > 2 && args[2] == "--cpp";
    let spv: Grammar = serde_json::from_reader(file).expect("file should be proper JSON");

    println!("// Auto generated - do not modify");

    if defs
    {
        println!("#pragma once\n");
        println!("namespace spvgentwo::spv\n{{");

        println!("\tusing Id = unsigned int;");
        println!("\tstatic constexpr unsigned int MagicNumber = {};", spv.magic_number);
        let version: u32 = spv.major_version << 16 | spv.minor_version << 8;
        println!("\tstatic constexpr unsigned int Version = {};", version);
        println!("\tstatic constexpr unsigned int Revision = {};", spv.revision);
        println!("\tstatic constexpr unsigned int WordCountShift = 16;");

        for op in &spv.operand_kinds {
            match op.enumerants.as_ref()  {
                Some(v) => {
                    println!("\tenum class {} : unsigned\n\t{{", op.kind);
                    for enumval in v {
                        if op.kind == "Dim" && enumval.enumerant.len() == 2{
                            print!("\t\tDim{} = ", enumval.enumerant);
                        }else{
                            print!("\t\t{} = ", enumval.enumerant);
                        }
                        match &enumval.value
                        {
                            serde_json::Value::Number(x) => {print!("{},\n", x)},
                            serde_json::Value::String(s) => {print!("{},\n", s)}
                            _ => {}
                        }
                    }
                    println!("\t}};");    
                },
                None => {}
            }          
        }

        println!("\tenum class Op : unsigned\n\t{{");
        for instr in &spv.instructions
        {
            println!("\t\t{} = {},", instr.opname, instr.opcode);
        }
        println!("\t}};");

        // HasResultAndType
        {
            println!("\tinline void HasResultAndType(Op opcode, bool *hasResult, bool *hasResultType) {{");
            println!("\t\t*hasResult = *hasResultType = false;");
            println!("\t\tswitch (opcode) {{");
            println!("\t\tdefault: /* unknown opcode */ break;");

            let mut opcodes = HashSet::new();
            for instr in &spv.instructions
            {
                if opcodes.contains(&instr.opcode) { continue; }
                opcodes.insert(instr.opcode);

                match &instr.operands
                {
                    Some(ops) =>
                    {
                        let mut res_type = false;
                        let mut res = false;
                        for operand in ops {
                            if operand.kind == "IdResultType" { res_type = true;}
                            if operand.kind == "IdResult" { res = true;}
                            if res && res_type {break;}
                        }
                        if res || res_type {
                            println!("\t\tcase Op::{}: *hasResult = {}; *hasResultType = {}; break;", instr.opname, res, res_type);
                        }
                    }
                    None => {}
                }            
            }

            println!("\t\t}}");
            println!("\t}}");
        }

        // HasResult
        {
            println!("\tinline constexpr bool HasResult(Op opcode) {{");
            println!("\t\tswitch (opcode) {{");
            println!("\t\tdefault: return true; // majority of instructions has a result");

            let mut opcodes = HashSet::new();        

            for instr in &spv.instructions
            {
                if opcodes.contains(&instr.opcode) { continue; }
                opcodes.insert(instr.opcode);
                match &instr.operands
                {
                    Some(ops) =>
                    {
                        let mut res = false;
                        for operand in ops {
                            if operand.kind == "IdResult" { res = true; break;}
                        }
                        if res == false {
                            println!("\t\tcase Op::{}: return false;", instr.opname);
                        }
                    }
                    None => {}
                }            
            }

            println!("\t\t}}");
            println!("\t}}");
        }

        // HasResultType
        {
            println!("\tinline constexpr bool HasResultType(Op opcode) {{");
            println!("\t\tswitch (opcode) {{");
            println!("\t\tdefault: return true; // majority of instructions has a result type");

            let mut opcodes = HashSet::new();        

            for instr in &spv.instructions
            {
                if opcodes.contains(&instr.opcode) { continue; }
                opcodes.insert(instr.opcode);
                match &instr.operands
                {
                    Some(ops) =>
                    {
                        let mut res = false;
                        for operand in ops {
                            if operand.kind == "IdResultType" { res = true; break;}
                        }
                        if res == false {
                            println!("\t\tcase Op::{}: return false;", instr.opname);
                        }
                    }
                    None => {}
                }            
            }

            println!("\t\t}}");
            println!("\t}}");
        }

        println!("}} // spvgentwo::spv");
    }    
    else if header
    {
        println!("#pragma once\n");

        println!("#include \"Vector.h\"");
        println!("#include \"HashMap.h\"");
        println!("#include \"Spv.h\"\n");

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
                println!("\t\tVector<spv::Capability> capabilities;");
                println!("\t\tVector<const char*> extensions;");
                println!("\t\tunsigned int version;");
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
            //if instr.class != "@exclude" && instr.class != "Reserved"
            {
                let ver: u32 = match instr.version
                {
                    Some(s) => {
                        //if s == "None" { continue }
                        let vec: Vec<&str> = s.split(".").collect();
                        if vec.len() == 2 {
                            let major: u32 = vec.first().unwrap().to_string().parse().unwrap();
                            let minor: u32 = vec.last().unwrap().to_string().parse().unwrap();
                            (major << 16) | (minor << 8)
                        }else {0} // fail case
                    },
                    None => 0
                };

                println!("\tm_instructions.emplaceUnique(spv::Op::{}, Instruction{{\"{}\", spv::Op::{}, _pAllocator, _pAllocator, _pAllocator, {}}});", instr.opname, instr.opname, instr.opname, ver);
            }
        }
        println!("}};"); // constructor
    }

    //let mut ofile = File::create("spvgrammer.inl").expect("unable to create file");
    //ofile.write_all(output.as_bytes()).expect("unable to write");
}
