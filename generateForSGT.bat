target\debug\rustspvgen.exe spirv.core.grammar.json --defs > ..\SpvGenTwo\lib\include\spvgentwo\Spv.h
target\debug\rustspvgen.exe spirv.core.grammar.json --header > ..\SpvGenTwo\lib\include\spvgentwo\Grammar.h
target\debug\rustspvgen.exe spirv.core.grammar.json extinst.glsl.std.450.grammar.json extinst.opencl.std.100.grammar.json --cpp > ..\SpvGenTwo\lib\source\Grammar.cpp
target\debug\rustspvgen.exe extinst.glsl.std.450.grammar.json --defs > ..\SpvGenTwo\lib\include\spvgentwo\Glsl.h
target\debug\rustspvgen.exe extinst.opencl.std.100.grammar.json --defs > ..\SpvGenTwo\lib\include\spvgentwo\OpenCl.h