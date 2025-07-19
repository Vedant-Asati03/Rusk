# Example highlight queries for Kotlin

; Keywords
"abstract" @keyword
"annotation" @keyword
"as" @keyword
"break" @keyword
"by" @keyword
"catch" @keyword
"class" @keyword
"companion" @keyword
"const" @keyword
"constructor" @keyword
"continue" @keyword
"crossinline" @keyword
"data" @keyword
"do" @keyword
"dynamic" @keyword
"else" @keyword
"enum" @keyword
"external" @keyword
"false" @keyword
"final" @keyword
"finally" @keyword
"for" @keyword
"fun" @keyword
"get" @keyword
"if" @keyword
"import" @keyword
"in" @keyword
"infix" @keyword
"init" @keyword
"inline" @keyword
"inner" @keyword
"interface" @keyword
"internal" @keyword
"is" @keyword
"lateinit" @keyword
"noinline" @keyword
"null" @keyword
"object" @keyword
"open" @keyword
"operator" @keyword
"out" @keyword
"override" @keyword
"package" @keyword
"private" @keyword
"protected" @keyword
"public" @keyword
"reified" @keyword
"return" @keyword
"sealed" @keyword
"set" @keyword
"super" @keyword
"suspend" @keyword
"tailrec" @keyword
"this" @keyword
"throw" @keyword
"true" @keyword
"try" @keyword
"typealias" @keyword
"typeof" @keyword
"val" @keyword
"var" @keyword
"vararg" @keyword
"when" @keyword
"where" @keyword
"while" @keyword

; Built-in types
"Boolean" @type.builtin
"Byte" @type.builtin
"Short" @type.builtin
"Int" @type.builtin
"Long" @type.builtin
"Float" @type.builtin
"Double" @type.builtin
"Char" @type.builtin
"String" @type.builtin
"Array" @type.builtin
"List" @type.builtin
"Set" @type.builtin
"Map" @type.builtin
"Any" @type.builtin
"Nothing" @type.builtin
"Unit" @type.builtin

; Functions
(function_declaration (simple_identifier) @function)
(call_expression (simple_identifier) @function.call)

; Classes
(class_declaration (type_identifier) @type)

; Variables
(simple_identifier) @variable

; Constants
"true" @constant.builtin
"false" @constant.builtin
"null" @constant.builtin

; Strings
(string_literal) @string
(character_literal) @string

; Numbers
(integer_literal) @number
(real_literal) @number

; Comments
(line_comment) @comment
(block_comment) @comment

; Annotations
(annotation) @attribute

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
"===" @operator
"!==" @operator
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
".." @operator
"?:" @operator
"?." @operator
"!!" @operator

; Punctuation
"(" @punctuation.bracket
")" @punctuation.bracket
"[" @punctuation.bracket
"]" @punctuation.bracket
"{" @punctuation.bracket
"}" @punctuation.bracket
"<" @punctuation.bracket
">" @punctuation.bracket
";" @punctuation.delimiter
"," @punctuation.delimiter
"." @punctuation.delimiter
":" @punctuation.delimiter
"::" @punctuation.delimiter