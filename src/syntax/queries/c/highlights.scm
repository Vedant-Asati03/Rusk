# Example highlight queries for C

; Keywords
"auto" @keyword
"break" @keyword
"case" @keyword
"char" @keyword
"const" @keyword
"continue" @keyword
"default" @keyword
"do" @keyword
"double" @keyword
"else" @keyword
"enum" @keyword
"extern" @keyword
"float" @keyword
"for" @keyword
"goto" @keyword
"if" @keyword
"inline" @keyword
"int" @keyword
"long" @keyword
"register" @keyword
"restrict" @keyword
"return" @keyword
"short" @keyword
"signed" @keyword
"sizeof" @keyword
"static" @keyword
"struct" @keyword
"switch" @keyword
"typedef" @keyword
"union" @keyword
"unsigned" @keyword
"void" @keyword
"volatile" @keyword
"while" @keyword
"_Alignas" @keyword
"_Alignof" @keyword
"_Atomic" @keyword
"_Generic" @keyword
"_Noreturn" @keyword
"_Static_assert" @keyword
"_Thread_local" @keyword

; Built-in types
"bool" @type.builtin
"size_t" @type.builtin
"ssize_t" @type.builtin
"ptrdiff_t" @type.builtin
"wchar_t" @type.builtin
"int8_t" @type.builtin
"int16_t" @type.builtin
"int32_t" @type.builtin
"int64_t" @type.builtin
"uint8_t" @type.builtin
"uint16_t" @type.builtin
"uint32_t" @type.builtin
"uint64_t" @type.builtin
"FILE" @type.builtin

; Functions
(function_declarator declarator: (identifier) @function)
(call_expression function: (identifier) @function.call)

; Variables
(identifier) @variable

; Strings
(string_literal) @string
(char_literal) @string

; Numbers
(number_literal) @number

; Comments
(comment) @comment

; Preprocessor
(preproc_include) @include
(preproc_define) @define
(preproc_ifdef) @conditional
(preproc_ifndef) @conditional
(preproc_if) @conditional
(preproc_endif) @conditional

; Operators
"+" @operator
"-" @operator
"*" @operator
"/" @operator
"%" @operator
"++" @operator
"--" @operator
"=" @operator
"+=" @operator
"-=" @operator
"*=" @operator
"/=" @operator
"%=" @operator
"==" @operator
"!=" @operator
"<" @operator
">" @operator
"<=" @operator
">=" @operator
"&&" @operator
"||" @operator
"!" @operator
"&" @operator
"|" @operator
"^" @operator
"~" @operator
"<<" @operator
">>" @operator

; Punctuation
"(" @punctuation.bracket
")" @punctuation.bracket
"[" @punctuation.bracket
"]" @punctuation.bracket
"{" @punctuation.bracket
"}" @punctuation.bracket
";" @punctuation.delimiter
"," @punctuation.delimiter
"." @punctuation.delimiter
"->" @punctuation.delimiter