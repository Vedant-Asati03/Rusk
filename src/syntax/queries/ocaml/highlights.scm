# Example highlight queries for OCaml

; Keywords
"and" @keyword
"as" @keyword
"assert" @keyword
"asr" @keyword
"begin" @keyword
"class" @keyword
"constraint" @keyword
"do" @keyword
"done" @keyword
"downto" @keyword
"else" @keyword
"end" @keyword
"exception" @keyword
"external" @keyword
"false" @keyword
"for" @keyword
"fun" @keyword
"function" @keyword
"functor" @keyword
"if" @keyword
"in" @keyword
"include" @keyword
"inherit" @keyword
"initializer" @keyword
"land" @keyword
"lazy" @keyword
"let" @keyword
"lor" @keyword
"lsl" @keyword
"lsr" @keyword
"lxor" @keyword
"match" @keyword
"method" @keyword
"mod" @keyword
"module" @keyword
"mutable" @keyword
"new" @keyword
"nonrec" @keyword
"object" @keyword
"of" @keyword
"open" @keyword
"or" @keyword
"private" @keyword
"rec" @keyword
"sig" @keyword
"struct" @keyword
"then" @keyword
"to" @keyword
"true" @keyword
"try" @keyword
"type" @keyword
"val" @keyword
"virtual" @keyword
"when" @keyword
"while" @keyword
"with" @keyword

; Built-in types
"bool" @type.builtin
"char" @type.builtin
"float" @type.builtin
"int" @type.builtin
"string" @type.builtin
"unit" @type.builtin
"list" @type.builtin
"array" @type.builtin
"option" @type.builtin

; Functions
(let_binding pattern: (value_name) @function)
(function_expression @function)

; Variables
(value_name) @variable

; Constants
"true" @constant.builtin
"false" @constant.builtin

; Strings
(string) @string
(character) @string

; Numbers
(number) @number

; Comments
(comment) @comment

; Operators
"+" @operator
"-" @operator
"*" @operator
"/" @operator
"mod" @operator
"=" @operator
"<>" @operator
"<" @operator
">" @operator
"<=" @operator
">=" @operator
"&&" @operator
"||" @operator
"not" @operator
"::" @operator
"@" @operator
"^" @operator
"!" @operator
":=" @operator

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
"|" @punctuation.delimiter