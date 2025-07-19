# Example highlight queries for Go

; Keywords
"break" @keyword
"case" @keyword
"chan" @keyword
"const" @keyword
"continue" @keyword
"default" @keyword
"defer" @keyword
"else" @keyword
"fallthrough" @keyword
"for" @keyword
"func" @keyword
"go" @keyword
"goto" @keyword
"if" @keyword
"import" @keyword
"interface" @keyword
"map" @keyword
"package" @keyword
"range" @keyword
"return" @keyword
"select" @keyword
"struct" @keyword
"switch" @keyword
"type" @keyword
"var" @keyword

; Built-in types
"bool" @type.builtin
"byte" @type.builtin
"complex64" @type.builtin
"complex128" @type.builtin
"error" @type.builtin
"float32" @type.builtin
"float64" @type.builtin
"int" @type.builtin
"int8" @type.builtin
"int16" @type.builtin
"int32" @type.builtin
"int64" @type.builtin
"rune" @type.builtin
"string" @type.builtin
"uint" @type.builtin
"uint8" @type.builtin
"uint16" @type.builtin
"uint32" @type.builtin
"uint64" @type.builtin
"uintptr" @type.builtin

; Functions
(function_declaration name: (identifier) @function)
(method_declaration name: (field_identifier) @function.method)
(call_expression function: (identifier) @function)

; Variables
(identifier) @variable

; Strings
(interpreted_string_literal) @string
(raw_string_literal) @string
(rune_literal) @string

; Numbers
(int_literal) @number
(float_literal) @number
(imaginary_literal) @number

; Comments
(comment) @comment

; Operators
"+" @operator
"-" @operator
"*" @operator
"/" @operator
"%" @operator
"&" @operator
"|" @operator
"^" @operator
"<<" @operator
">>" @operator
"&^" @operator
"+=" @operator
"-=" @operator
"*=" @operator
"/=" @operator
"%=" @operator
"&=" @operator
"|=" @operator
"^=" @operator
"<<=" @operator
">>=" @operator
"&^=" @operator
"&&" @operator
"||" @operator
"<-" @operator
"++" @operator
"--" @operator
"==" @operator
"<" @operator
">" @operator
"=" @operator
"!" @operator
"!=" @operator
"<=" @operator
">=" @operator
":=" @operator
"..." @operator

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
":" @punctuation.delimiter