# Example highlight queries for Haskell

; Keywords
"as" @keyword
"case" @keyword
"class" @keyword
"data" @keyword
"default" @keyword
"deriving" @keyword
"do" @keyword
"else" @keyword
"hiding" @keyword
"if" @keyword
"import" @keyword
"in" @keyword
"infix" @keyword
"infixl" @keyword
"infixr" @keyword
"instance" @keyword
"let" @keyword
"module" @keyword
"newtype" @keyword
"of" @keyword
"qualified" @keyword
"then" @keyword
"type" @keyword
"where" @keyword

; Built-in types
"Bool" @type.builtin
"Char" @type.builtin
"Double" @type.builtin
"Float" @type.builtin
"Int" @type.builtin
"Integer" @type.builtin
"IO" @type.builtin
"Maybe" @type.builtin
"Either" @type.builtin
"String" @type.builtin

; Functions
(function) @function
(variable) @function.call

; Variables
(variable) @variable

; Constants
"True" @constant.builtin
"False" @constant.builtin

; Strings
(string) @string
(char) @string

; Numbers
(integer) @number
(float) @number

; Comments
(comment) @comment

; Operators
"+" @operator
"-" @operator
"*" @operator
"/" @operator
"^" @operator
"**" @operator
"==" @operator
"/=" @operator
"<" @operator
">" @operator
"<=" @operator
">=" @operator
"&&" @operator
"||" @operator
"!" @operator
"." @operator
"$" @operator
"++" @operator
"!!" @operator
"<-" @operator
"->" @operator
"=>" @operator
"|" @operator
"::" @operator

; Punctuation
"(" @punctuation.bracket
")" @punctuation.bracket
"[" @punctuation.bracket
"]" @punctuation.bracket
"{" @punctuation.bracket
"}" @punctuation.bracket
";" @punctuation.delimiter
"," @punctuation.delimiter