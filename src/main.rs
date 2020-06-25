use std::fs::File;
use std::env;

extern crate serde_derive;
extern crate serde_json;
extern crate serde;

use serde_derive::Deserialize;
use std::collections::HashSet;
use std::collections::HashMap;
use std::collections::BTreeSet;
use std::collections::BTreeMap;

#[derive(Debug)]
enum Extension
{
    glslstd450,
    opencl100
}

#[derive(Deserialize)]
struct Grammar {
    copyright: Vec<String>,
    magic_number: Option<String>, // spv
    major_version: Option<u32>, // spv
    minor_version: Option<u32>, // spv
    version: Option<u32>, // glsl & opencl
    revision: u32, // both
    instructions: Vec<Instruction>, // both
    operand_kinds: Option<Vec<OperandKinds>> // spv
}

#[derive(Deserialize)]
struct Instruction
{
    opname: String,
    class: Option<String>,
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

fn spv_defs(spv: Grammar)
{
    let operand_kinds = spv.operand_kinds.unwrap_or(Vec::new());

    for line in spv.copyright {
        println!("// {}", line); 
    }

    println!("#pragma once\n");
    println!("namespace spvgentwo::spv\n{{");

    println!("\tusing Id = unsigned int;");
    println!("\tstatic constexpr unsigned int MagicNumber = {};", spv.magic_number.unwrap_or("0x07230203".to_string()));
    let version: u32 = spv.major_version.unwrap_or_default() << 16 | spv.minor_version.unwrap_or_default() << 8;
    println!("\tstatic constexpr unsigned int Version = {};", version);
    println!("\tstatic constexpr unsigned int Revision = {};", spv.revision);
    println!("\tstatic constexpr unsigned int OpCodeMask = 0xffff;");
    println!("\tstatic constexpr unsigned int WordCountShift = 16;");

    let mut extensions = BTreeSet::new();

    // scann for extensions
    for op in &operand_kinds {
        match op.enumerants.as_ref()  {
            Some(v) => {
                for enumval in v {                    
                    for ext in enumval.extensions.as_ref() {
                        for e in ext {
                            if extensions.contains(e) == false{
                                extensions.insert(e);
                            }
                        }
                    }
                } 
            },
            None => {}
        }  
    }

    // value enums
    for op in &operand_kinds {
        if op.category == "ValueEnum" || op.category != "BitEnum" {
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
                    println!("\t\tMax = 0x7fffffff");
                    println!("\t}};");    
                },
                None => {}
            }  
        }
        else if op.category == "BitEnum"
        {
            match op.enumerants.as_ref()  {
                Some(v) => {
                    println!("\tenum class {}Mask : unsigned\n\t{{", op.kind);
                    for enumval in v {
                        if enumval.enumerant == "None"{
                            print!("\t\tMask{} = ", enumval.enumerant);
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

                    println!("\tenum class {}Shift : unsigned\n\t{{", op.kind);
                    let mut shift = 0;
                    for enumval in v {
                        println!("\t\t{} = {},", enumval.enumerant, shift); shift += 1
                    }
                    println!("\t\tMax = 0x7fffffff");
                    println!("\t}};");   
                },
                None => {}
            }
        }        
    }

    // opcode enum
    println!("\tenum class Op : unsigned\n\t{{");
    for instr in &spv.instructions
    {
        println!("\t\t{} = {},", instr.opname, instr.opcode);

        for ext in instr.extensions.as_ref(){
            for e in ext{
                if extensions.contains(e) == false{
                    extensions.insert(e);
                }
            }
        }
    }
    println!("\t\tMax = 0x7fffffff");
    println!("\t}};");

    // extensions enum
    {
        println!("\tenum class Extension : unsigned\n\t{{");
        let mut i = 0;
        for ext in &extensions
        {
            println!("\t\t{} = {},", ext, i); i += 1;
        }
        println!("\t\tMax = 0x7fffffff");
        println!("\t}};");
    }

    // extension names array
    println!("\tstatic constexpr const char* ExtensionNames[] =\n\t{{");
    for ext in &extensions
    {
        println!("\t\t\"{}\",", ext);
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
                Some(ops) => {    
                    let mut res = false;                
                    for operand in ops {
                        if operand.kind == "IdResult" {res = true; break;}
                    }
                    if res == false {println!("\t\tcase Op::{}: return false;", instr.opname);  }
                }
                None => { println!("\t\tcase Op::{}: return false;", instr.opname);  }
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
            match &instr.operands {
                Some(ops) => {
                    let mut res = false;
                    for operand in ops {
                        if operand.kind == "IdResultType" { res = true; break;}
                    }
                    if res == false {
                        println!("\t\tcase Op::{}: return false;", instr.opname);
                    }
                }
                None => {println!("\t\tcase Op::{}: return false;", instr.opname);}
            }            
        }

        println!("\t\t}}");
        println!("\t}}");
    }

    // IsTypeOp
    {
        println!("\tinline constexpr bool IsTypeOp(Op opcode) {{");
        println!("\t\tswitch (opcode) {{");
        println!("\t\tdefault: return false; // majority of instructions are not types");

        let mut opcodes = HashSet::new();        

        for instr in &spv.instructions
        {
            if instr.opname.starts_with("OpType") {
                if opcodes.contains(&instr.opcode) { continue; }
                opcodes.insert(instr.opcode);
                println!("\t\tcase Op::{}: return true;", instr.opname);
            }
        }

        println!("\t\t}}");
        println!("\t}}");
    }

    // IsConstantOp
    {
        println!("\tinline constexpr bool IsConstantOp(Op opcode) {{");
        println!("\t\tswitch (opcode) {{");
        println!("\t\tdefault: return false; // majority of instructions are not constants");

        let mut opcodes = HashSet::new();        

        for instr in &spv.instructions
        {
            if instr.opname.starts_with("OpConstant") {
                if opcodes.contains(&instr.opcode) { continue; }
                opcodes.insert(instr.opcode);
                println!("\t\tcase Op::{}: return true;", instr.opname);
            }
        }

        println!("\t\t}}");
        println!("\t}}");
    }

    // IsSpecConstantOp
    {
        println!("\tinline constexpr bool IsSpecConstantOp(Op opcode) {{");
        println!("\t\tswitch (opcode) {{");
        println!("\t\tdefault: return false; // majority of instructions are not spec constants");

        let mut opcodes = HashSet::new();        

        for instr in &spv.instructions
        {
            if instr.opname.starts_with("OpSpecConstant") {
                if opcodes.contains(&instr.opcode) { continue; }
                opcodes.insert(instr.opcode);
                println!("\t\tcase Op::{}: return true;", instr.opname);
            }
        }

        println!("\t\t}}");
        println!("\t}}");
    }

    println!("}} // spvgentwo::spv");
}

fn grammar_header(spv: Grammar)
{
    let operand_kinds = spv.operand_kinds.unwrap_or(Vec::new());

    println!("#pragma once\n");

    println!("#include \"Vector.h\"");
    println!("#include \"HashMap.h\"");
    println!("#include \"Spv.h\"\n");

    println!("namespace spvgentwo\n{{");

    println!("class Grammar\n{{");
    println!("\tpublic:");
        println!("\tenum class Extension : unsigned short\n\t{{");
            println!("\t\tCore = 0,");
            println!("\t\tGlsl = 1,");
            println!("\t\tOpenCl = 2,");  
        println!("\t}};");

        let mut categories = HashSet::new();
        println!("\tenum class OperandCategory : unsigned short\n\t{{");
        for elem in &operand_kinds {
            if !categories.contains(&elem.category)
            {
                categories.insert(&elem.category);
                println!("\t\t{},",elem.category);
            }
        }
        println!("\t}};");

        println!("\tenum class OperandKind : unsigned short\n\t{{");
        for elem in &operand_kinds {
            println!("\t\t{},",elem.kind);
        }
        println!("\t}};");

        // operand name lookup tables
        for op in &operand_kinds {
            if op.category == "ValueEnum" || op.category == "BitEnum" {
                match op.enumerants.as_ref()  {
                    Some(v) => {
                        println!("\tstatic constexpr const char* {}Names[] =\n\t{{", op.kind);
                        for enumval in v {
                            if op.kind == "Dim" && enumval.enumerant.len() == 2{
                                println!("\t\t\"Dim{}\",", enumval.enumerant);
                            }else{
                                println!("\t\t\"{}\",", enumval.enumerant);
                            }
                        }
                        println!("\t}};");    
                    },
                    None => {}
                }  
            }        
        }

        println!("\tstatic constexpr const char* const* OperandKindNames[] =\n\t{{");
        for op in &operand_kinds {
            if op.category == "ValueEnum" || op.category == "BitEnum" {
                println!("\t\t{}Names,", op.kind);
            }
        }
        println!("\t}};"); 

        println!("\tenum class Quantifier\n\t{{");
            println!("\t\tZeroOrOne, // zero or one");
            println!("\t\tZeroOrAny, // zero or any");
            println!("\t\tOne, // exactly once");
        println!("\t}};");

        println!("\tstruct Operand\n\t{{");
            println!("\t\tOperandKind kind;");
            println!("\t\tOperandCategory category;");        
            println!("\t\tconst char* name;");
            println!("\t\tQuantifier quantifier;");
        println!("\t}};");

        println!("\tstruct Instruction\n\t{{");
            println!("\t\tconst char* name;");
            println!("\t\tVector<Operand> operands;");
            println!("\t\tVector<spv::Capability> capabilities;");
            println!("\t\tVector<spv::Extension> extensions;");
            println!("\t\tunsigned int version;");
        println!("\t}};");

        println!("\t\tGrammar(IAllocator* _pAllocator);");
        println!("\t\tconst Instruction* getInfo(unsigned int _opcode, Extension _extension = Extension::Core) const;");
    println!("\tprivate:");
        println!("\t\tHashMap<Hash64, Instruction> m_instructions;");
    println!("}};");

    println!("}} // spvgentwo"); // namespace
}

fn print_instruction(instr: &Instruction, kind_categories: &std::collections::HashMap<&std::string::String, &std::string::String>, shift: u32)
{
    let ver: u32 = match &instr.version
    {
        Some(s) => {
            let vec: Vec<&str> = s.split(".").collect();
            if vec.len() == 2 {
                let major: u32 = vec.first().unwrap().to_string().parse().unwrap();
                let minor: u32 = vec.last().unwrap().to_string().parse().unwrap();
                (major << 16) | (minor << 8)
            }else {0} // fail case
        },
        None => 0
    };

    let has_props = instr.operands.is_some() || instr.capabilities.is_some() || instr.extensions.is_some();
    if has_props {
        println!("\t{{");
        print!("\t\tauto& instr = ");
    } else {print!("\t");}

    print!("m_instructions.emplaceUnique({}ull << 32u |{}u, Instruction{{\"{}\", _pAllocator, _pAllocator, _pAllocator, {}}})", shift, instr.opcode, instr.opname, ver);
    
    if has_props {
        println!(".kv.value;");
    } else {println!(";");}
    
    if instr.operands.is_some(){
        for op in instr.operands.as_ref().unwrap() {
            let category = kind_categories[&op.kind];
            let quantifier = match &op.quantifier {Some(s) => if s == "?" { "Quantifier::ZeroOrOne"} else if s == "*" {"Quantifier::ZeroOrAny"} else {"Quantifier::One"}, None => "Quantifier::One"};
            let name =  match op.name {Some(ref s) => s, None => if op.kind == "IdResultType" { "ResultType"} else if op.kind == "IdResult" { "Result" } else {""}};
            println!("\t\tinstr.operands.emplace_back(OperandKind::{}, OperandCategory::{}, \"{}\",{});", &op.kind, category, name.replace("\n", ""), quantifier);
        }
    }

    if instr.capabilities.is_some(){
        for cap in instr.capabilities.as_ref().unwrap() {
            println!("\t\tinstr.capabilities.emplace_back(spv::Capability::{});", cap);
        }
    }

    if instr.extensions.is_some(){
        for ext in instr.extensions.as_ref().unwrap() {
            println!("\t\tinstr.extensions.emplace_back(spv::Extension::{});", ext);
        }
    }

    if has_props {
        println!("\t}}")
    }
}

fn grammar_cpp(spv: Grammar, glsl: Grammar, opencl: Grammar)
{
    let operand_kinds = spv.operand_kinds.unwrap_or(Vec::new());

    println!("#include \"spvgentwo/Grammar.h\"");
    println!("#include \"spvgentwo/Glsl.h\"");
    println!("#include \"spvgentwo/OpenCl.h\"");
    println!("using namespace spvgentwo;\n");

    let mut kind_categories = HashMap::new();
    for elem in &operand_kinds {
        kind_categories.insert(&elem.kind, &elem.category);
    }

    let mut unique_instructions = BTreeMap::new();

    for instr in &spv.instructions
    {        
        let entry = &unique_instructions.entry(instr.opcode).or_insert(instr);
        // filter out / replace vendor extension instructions with their ratified versions
        if entry.opname != instr.opname && (instr.opname.ends_with("KHR") || instr.opname.ends_with("EXT")){
            unique_instructions.insert(instr.opcode, instr);
        }
    }

    println!("Grammar::Grammar(IAllocator* _pAllocator) : m_instructions(_pAllocator, {})\n{{", unique_instructions.len() + glsl.instructions.len() + opencl.instructions.len());
    for (opcode, instr) in unique_instructions
    {
        print_instruction(instr, &kind_categories, 0);
    }

    for instr in glsl.instructions
    {
        print_instruction(&instr, &kind_categories, 1);
    }

    for instr in opencl.instructions
    {
        print_instruction(&instr, &kind_categories, 2);
    }

    println!("}};"); // constructor

    println!("const Grammar::Instruction* Grammar::getInfo(unsigned int _opcode, Extension _extension) const\n{{");
        println!("\tHash64 hash = static_cast<Hash64>(_extension) << 32u | _opcode;");
        println!("\treturn m_instructions.get(hash);");
    println!("}};"); // getInfo
}

fn ext_defs(spv: Grammar, ext: Extension)
{
    for line in spv.copyright {
        println!("// {}", line); 
    }

    println!("#pragma once\n");
    println!("namespace spvgentwo::{:?}\n{{", ext);

    println!("\tstatic constexpr unsigned int Version = {};", spv.version.unwrap_or_default());
    println!("\tstatic constexpr unsigned int Revision = {};", spv.revision);

    println!("\tenum class Op : unsigned\n\t{{");
    for instr in &spv.instructions
    {
        println!("\t\t{} = {},", instr.opname, instr.opcode);
    }
    println!("\t\tMax = 0x7fffffff");
    println!("\t}};");

    println!("}} // spvgentwo::{:?}", ext);
}

fn main() {
    let args: Vec<String> = env::args().collect();

    let mut spv: Option<Grammar> = None;
    let mut glsl: Option<Grammar> = None;
    let mut opencl: Option<Grammar> = None;

    let mut defs = false;
    let mut header = false;
    let mut cpp = false;

    for arg in args
    {
        if arg.ends_with("spirv.core.grammar.json") {
            spv = serde_json::from_reader(File::open(arg).expect("file should open read only")).expect("file should be proper JSON");
        } else if arg.ends_with("extinst.glsl.std.450.grammar.json") {
            glsl = serde_json::from_reader(File::open(arg).expect("file should open read only")).expect("file should be proper JSON"); 
        } else if arg.ends_with("extinst.opencl.std.100.grammar.json") {
            opencl = serde_json::from_reader(File::open(arg).expect("file should open read only")).expect("file should be proper JSON"); 
        } else if arg == "--defs" {
            defs = true;
        } else if arg == "--header" {
            header = true;
        } else if arg == "--cpp" {
            cpp = true;
        }
    }
    println!("// Auto generated - do not modify");

    if defs {
        if spv.is_some() { spv_defs(spv.unwrap()); } else
        if glsl.is_some() { ext_defs(glsl.unwrap(), Extension::glslstd450); } else
        if opencl.is_some() { ext_defs(opencl.unwrap(), Extension::opencl100); } 
    }    
    else if header && spv.is_some() {
        grammar_header(spv.unwrap());
    }
    else if cpp && spv.is_some() && glsl.is_some() && opencl.is_some() {
        grammar_cpp(spv.unwrap(), glsl.unwrap(), opencl.unwrap());
    }
}
