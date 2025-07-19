# Example highlight queries for Java

; Keywords
"abstract" @keyword
"assert" @keyword
"break" @keyword
"case" @keyword
"catch" @keyword
"class" @keyword
"const" @keyword
"continue" @keyword
"default" @keyword
"do" @keyword
"else" @keyword
"enum" @keyword
"extends" @keyword
"final" @keyword
"finally" @keyword
"for" @keyword
"goto" @keyword
"if" @keyword
"implements" @keyword
"import" @keyword
"instanceof" @keyword
"interface" @keyword
"native" @keyword
"new" @keyword
"package" @keyword
"private" @keyword
"protected" @keyword
"public" @keyword
"return" @keyword
"static" @keyword
"strictfp" @keyword
"super" @keyword
"switch" @keyword
"synchronized" @keyword
"this" @keyword
"throw" @keyword
"throws" @keyword
"transient" @keyword
"try" @keyword
"volatile" @keyword
"while" @keyword

; Primitive types
"boolean" @type.builtin
"byte" @type.builtin
"char" @type.builtin
"double" @type.builtin
"float" @type.builtin
"int" @type.builtin
"long" @type.builtin
"short" @type.builtin
"void" @type.builtin

; Type identifiers
(type_identifier) @type

; Functions/Methods
(method_declaration name: (identifier) @function)
(constructor_declaration name: (identifier) @constructor)
(method_invocation name: (identifier) @function.call)

; Variables
(identifier) @variable

; Strings
(string_literal) @string
(character_literal) @string

; Numbers
(decimal_integer_literal) @number
(hex_integer_literal) @number
(octal_integer_literal) @number
(binary_integer_literal) @number
(decimal_floating_point_literal) @number
(hex_floating_point_literal) @number

; Comments
(line_comment) @comment
(block_comment) @comment

; Annotations
(annotation) @annotation

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
">>>" @operator
"&=" @operator
"|=" @operator
"^=" @operator
"<<=" @operator
">>=" @operator
">>>=" @operator

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