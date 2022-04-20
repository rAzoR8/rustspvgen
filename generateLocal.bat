target\debug\rustspvgen.exe spirv.core.grammar.json --defs > generated\Spv.h
target\debug\rustspvgen.exe extinst.glsl.std.450.grammar.json --defs > generated\Glsl.h
target\debug\rustspvgen.exe extinst.opencl.std.100.grammar.json --defs > generated\OpenCl.h