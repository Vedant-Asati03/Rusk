# Example highlight queries for Dart

; Keywords
"abstract" @keyword
"as" @keyword
"assert" @keyword
"async" @keyword
"await" @keyword
"break" @keyword
"case" @keyword
"catch" @keyword
"class" @keyword
"const" @keyword
"continue" @keyword
"covariant" @keyword
"default" @keyword
"deferred" @keyword
"do" @keyword
"dynamic" @keyword
"else" @keyword
"enum" @keyword
"export" @keyword
"extends" @keyword
"extension" @keyword
"external" @keyword
"factory" @keyword
"false" @keyword
"final" @keyword
"finally" @keyword
"for" @keyword
"Function" @keyword
"get" @keyword
"hide" @keyword
"if" @keyword
"implements" @keyword
"import" @keyword
"in" @keyword
"interface" @keyword
"is" @keyword
"late" @keyword
"library" @keyword
"mixin" @keyword
"new" @keyword
"null" @keyword
"on" @keyword
"operator" @keyword
"part" @keyword
"required" @keyword
"rethrow" @keyword
"return" @keyword
"set" @keyword
"show" @keyword
"static" @keyword
"super" @keyword
"switch" @keyword
"sync" @keyword
"this" @keyword
"throw" @keyword
"true" @keyword
"try" @keyword
"typedef" @keyword
"var" @keyword
"void" @keyword
"while" @keyword
"with" @keyword
"yield" @keyword

; Built-in types
"bool" @type.builtin
"double" @type.builtin
"int" @type.builtin
"num" @type.builtin
"String" @type.builtin
"List" @type.builtin
"Map" @type.builtin
"Set" @type.builtin
"Object" @type.builtin
"dynamic" @type.builtin
"void" @type.builtin

; Functions
(function_signature name: (identifier) @function)
(function_expression_body (identifier) @function.call)

; Classes
(class_definition name: (identifier) @type)

; Variables
(identifier) @variable

; Constants
"true" @constant.builtin
"false" @constant.builtin
"null" @constant.builtin

; Strings
(string_literal) @string
(string_interpolation) @string

; Numbers
(decimal_integer_literal) @number
(hex_integer_literal) @number
(decimal_floating_point_literal) @number

; Comments
(comment) @comment

; Operators
"+" @operator
"-" @operator
"*" @operator
"/" @operator
"~/" @operator
"%" @operator
"++" @operator
"--" @operator
"=" @operator
"+=" @operator
"-=" @operator
"*=" @operator
"/=" @operator
"~/=" @operator
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
"??" @operator
"?" @operator
":" @operator

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
"?." @punctuation.delimiter