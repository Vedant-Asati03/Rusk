# Example highlight queries for Python

; Keywords
"and" @keyword
"as" @keyword
"assert" @keyword
"break" @keyword
"class" @keyword
"continue" @keyword
"def" @keyword
"del" @keyword
"elif" @keyword
"else" @keyword
"except" @keyword
"finally" @keyword
"for" @keyword
"from" @keyword
"global" @keyword
"if" @keyword
"import" @keyword
"in" @keyword
"is" @keyword
"lambda" @keyword
"nonlocal" @keyword
"not" @keyword
"or" @keyword
"pass" @keyword
"raise" @keyword
"return" @keyword
"try" @keyword
"while" @keyword
"with" @keyword
"yield" @keyword

; Built-in functions
(call
  function: (identifier) @function.builtin
  (#any-of? @function.builtin
    "abs" "all" "any" "bin" "bool" "bytearray" "bytes" "callable" "chr"
    "classmethod" "compile" "complex" "delattr" "dict" "dir" "divmod"
    "enumerate" "eval" "exec" "filter" "float" "format" "frozenset"
    "getattr" "globals" "hasattr" "hash" "help" "hex" "id" "input"
    "int" "isinstance" "issubclass" "iter" "len" "list" "locals"
    "map" "max" "memoryview" "min" "next" "object" "oct" "open"
    "ord" "pow" "print" "property" "range" "repr" "reversed" "round"
    "set" "setattr" "slice" "sorted" "staticmethod" "str" "sum" "super"
    "tuple" "type" "vars" "zip"))

; Functions
(function_definition name: (identifier) @function)
(call function: (identifier) @function)

; Variables
(identifier) @variable

; Strings
(string) @string
(f_string) @string

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
"//" @operator
"%" @operator
"**" @operator
"=" @operator
"+=" @operator
"-=" @operator
"*=" @operator
"/=" @operator
"//=" @operator
"%=" @operator
"**=" @operator
"==" @operator
"!=" @operator
"<" @operator
">" @operator
"<=" @operator
">=" @operator
"<<" @operator
">>" @operator
"&" @operator
"|" @operator
"^" @operator
"~" @operator

; Punctuation
"(" @punctuation.bracket
")" @punctuation.bracket
"[" @punctuation.bracket
"]" @punctuation.bracket
"{" @punctuation.bracket
"}" @punctuation.bracket
"," @punctuation.delimiter
":" @punctuation.delimiter
";" @punctuation.delimiter
"." @punctuation.delimiter