target\debug\rustspvgen.exe spirv.core.grammar.json --defs > C:\Users\razor\Projects\SpvGenTwo\lib\include\spvgentwo\Spv.h
target\debug\rustspvgen.exe spirv.core.grammar.json --header > C:\Users\razor\Projects\SpvGenTwo\lib\include\spvgentwo\Grammar.h
target\debug\rustspvgen.exe spirv.core.grammar.json extinst.glsl.std.450.grammar.json extinst.opencl.std.100.grammar.json --cpp > C:\Users\razor\Projects\SpvGenTwo\lib\source\Grammar.cpp
target\debug\rustspvgen.exe extinst.glsl.std.450.grammar.json --defs > C:\Users\razor\Projects\SpvGenTwo\lib\include\spvgentwo\Glsl.h
target\debug\rustspvgen.exe extinst.opencl.std.100.grammar.json --defs > C:\Users\razor\Projects\SpvGenTwo\lib\include\spvgentwo\OpenCl.h