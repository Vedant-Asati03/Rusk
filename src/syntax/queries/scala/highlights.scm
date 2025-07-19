# Example highlight queries for Scala

; Keywords
"abstract" @keyword
"case" @keyword
"catch" @keyword
"class" @keyword
"def" @keyword
"do" @keyword
"else" @keyword
"extends" @keyword
"false" @keyword
"final" @keyword
"finally" @keyword
"for" @keyword
"forSome" @keyword
"if" @keyword
"implicit" @keyword
"import" @keyword
"lazy" @keyword
"match" @keyword
"new" @keyword
"null" @keyword
"object" @keyword
"override" @keyword
"package" @keyword
"private" @keyword
"protected" @keyword
"return" @keyword
"sealed" @keyword
"super" @keyword
"this" @keyword
"throw" @keyword
"trait" @keyword
"try" @keyword
"true" @keyword
"type" @keyword
"val" @keyword
"var" @keyword
"while" @keyword
"with" @keyword
"yield" @keyword

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
"Unit" @type.builtin
"Nothing" @type.builtin
"Any" @type.builtin
"AnyRef" @type.builtin
"AnyVal" @type.builtin

; Functions
(function_definition name: (identifier) @function)
(call_expression function: (identifier) @function.call)

; Classes and objects
(class_definition name: (identifier) @type)
(object_definition name: (identifier) @type)
(trait_definition name: (identifier) @type)

; Variables
(identifier) @variable

; Constants
"true" @constant.builtin
"false" @constant.builtin
"null" @constant.builtin

; Strings
(string_literal) @string
(symbol_literal) @string.special

; Numbers
(integer_literal) @number
(floating_point_literal) @number

; Comments
(comment) @comment

; Operators
"+" @operator
"-" @operator
"*" @operator
"/" @operator
"%" @operator
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
"=" @operator
"+=" @operator
"-=" @operator
"*=" @operator
"/=" @operator
"%=" @operator
"<-" @operator
"=>" @operator

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
"::" @punctuation.delimiter