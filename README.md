# rustspvgen
CLI tool to generate C++ / SPIR-V enumerations and tables for [SpvGenTwo](https://github.com/rAzoR8/SpvGenTwo)

Consumes SPIR-V machine-readable JSON grammars:
* [spirv.core.grammar.json](https://github.com/KhronosGroup/SPIRV-Headers/blob/master/include/spirv/unified1/spirv.core.grammar.json)
* [extinst.opencl.std.100.grammar.json](https://github.com/KhronosGroup/SPIRV-Headers/blob/master/include/spirv/unified1/extinst.opencl.std.100.grammar.json)
* [extinst.glsl.std.450.grammar.json](https://github.com/KhronosGroup/SPIRV-Headers/blob/master/include/spirv/unified1/extinst.glsl.std.450.grammar.json)

I was to lazy to write the generated text to the file direclty, so the tool just prints it to stdout and you have to pipe it into a file.

* *--defs* generates a header like `spirv.hpp11` but with some extras
* *--header* generates Grammar.h
* *--cpp* generates Grammar.cpp (the main lookup table for SpvGenTwo)

Example usage:
```
rustspvgen.exe spirv.core.grammar.json --defs > C:\Users\Fabian\Projects\Proto\SpvGenTwo\lib\include\spvgentwo\Spv.h
rustspvgen.exe spirv.core.grammar.json --header > C:\Users\Fabian\Projects\Proto\SpvGenTwo\lib\include\spvgentwo\Grammar.h
rustspvgen.exe spirv.core.grammar.json extinst.glsl.std.450.grammar.json extinst.opencl.std.100.grammar.json --cpp > C:\Users\Fabian\Projects\Proto\SpvGenTwo\lib\source\Grammar.cpp
rustspvgen.exe extinst.glsl.std.450.grammar.json --defs > C:\Users\Fabian\Projects\Proto\SpvGenTwo\lib\include\spvgentwo\Glsl.h
rustspvgen.exe extinst.opencl.std.100.grammar.json --defs > C:\Users\Fabian\Projects\Proto\SpvGenTwo\lib\include\spvgentwo\OpenCl.h
```

Generated files can be found here: https://github.com/rAzoR8/SpvGenTwo/blob/feature/grammar/lib/include/spvgentwo/Spv.h
https://github.com/rAzoR8/SpvGenTwo/blob/feature/grammar/lib/include/spvgentwo/Glsl.h
https://github.com/rAzoR8/SpvGenTwo/blob/feature/grammar/lib/include/spvgentwo/OpenCl.h