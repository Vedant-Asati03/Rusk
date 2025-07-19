# Example highlight queries for R

; Keywords
"if" @keyword
"else" @keyword
"repeat" @keyword
"while" @keyword
"function" @keyword
"for" @keyword
"in" @keyword
"next" @keyword
"break" @keyword
"TRUE" @keyword
"FALSE" @keyword
"NULL" @keyword
"Inf" @keyword
"NaN" @keyword
"NA" @keyword
"NA_integer_" @keyword
"NA_real_" @keyword
"NA_character_" @keyword
"NA_complex_" @keyword

; Functions
(call name: (identifier) @function.call)
(function_definition name: (identifier) @function)

; Variables
(identifier) @variable

; Constants
"TRUE" @constant.builtin
"FALSE" @constant.builtin
"NULL" @constant.builtin
"Inf" @constant.builtin
"NaN" @constant.builtin

; Strings
(string) @string

; Numbers
(integer) @number
(float) @number
(complex) @number

; Comments
(comment) @comment

; Operators
"+" @operator
"-" @operator
"*" @operator
"/" @operator
"^" @operator
"%%" @operator
"%/%" @operator
"%*%" @operator
"%o%" @operator
"%x%" @operator
"%in%" @operator
"==" @operator
"!=" @operator
"<" @operator
">" @operator
"<=" @operator
">=" @operator
"&" @operator
"|" @operator
"&&" @operator
"||" @operator
"!" @operator
"=" @operator
"<-" @operator
"<<-" @operator
"->" @operator
"->>" @operator

; Punctuation
"(" @punctuation.bracket
")" @punctuation.bracket
"[" @punctuation.bracket
"]" @punctuation.bracket
"[[" @punctuation.bracket
"]]" @punctuation.bracket
"{" @punctuation.bracket
"}" @punctuation.bracket
";" @punctuation.delimiter
"," @punctuation.delimiter
"$" @punctuation.delimiter
"@" @punctuation.delimiter