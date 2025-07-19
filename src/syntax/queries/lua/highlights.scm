# Example highlight queries for Lua

; Keywords
"and" @keyword
"break" @keyword
"do" @keyword
"else" @keyword
"elseif" @keyword
"end" @keyword
"false" @keyword
"for" @keyword
"function" @keyword
"goto" @keyword
"if" @keyword
"in" @keyword
"local" @keyword
"nil" @keyword
"not" @keyword
"or" @keyword
"repeat" @keyword
"return" @keyword
"then" @keyword
"true" @keyword
"until" @keyword
"while" @keyword

; Functions
(function_declaration name: (identifier) @function)
(function_call name: (identifier) @function.call)

; Variables
(identifier) @variable

; Constants
"true" @constant.builtin
"false" @constant.builtin
"nil" @constant.builtin

; Strings
(string) @string

; Numbers
(number) @number

; Comments
(comment) @comment

; Operators
"+" @operator
"-" @operator
"*" @operator
"/" @operator
"//" @operator
"%" @operator
"^" @operator
"#" @operator
"=" @operator
"==" @operator
"~=" @operator
"<" @operator
">" @operator
"<=" @operator
">=" @operator
"and" @operator
"or" @operator
"not" @operator
".." @operator

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