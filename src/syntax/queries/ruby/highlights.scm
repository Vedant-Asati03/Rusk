# Example highlight queries for Ruby

; Keywords
"alias" @keyword
"and" @keyword
"begin" @keyword
"break" @keyword
"case" @keyword
"class" @keyword
"def" @keyword
"defined?" @keyword
"do" @keyword
"else" @keyword
"elsif" @keyword
"end" @keyword
"ensure" @keyword
"false" @keyword
"for" @keyword
"if" @keyword
"in" @keyword
"module" @keyword
"next" @keyword
"nil" @keyword
"not" @keyword
"or" @keyword
"redo" @keyword
"rescue" @keyword
"retry" @keyword
"return" @keyword
"self" @keyword
"super" @keyword
"then" @keyword
"true" @keyword
"undef" @keyword
"unless" @keyword
"until" @keyword
"when" @keyword
"while" @keyword
"yield" @keyword
"require" @keyword
"include" @keyword
"extend" @keyword
"attr_reader" @keyword
"attr_writer" @keyword
"attr_accessor" @keyword
"private" @keyword
"protected" @keyword
"public" @keyword

; Classes and modules
(class name: (constant) @type)
(module name: (constant) @type)

; Methods
(method name: (identifier) @function)
(call method: (identifier) @function.call)

; Constants
(constant) @constant

; Variables
(identifier) @variable
(instance_variable) @variable.instance
(class_variable) @variable.class
(global_variable) @variable.global

; Strings
(string) @string
(symbol) @string.special
(regex) @string.regex

; Numbers
(integer) @number
(float) @number

; Comments
(comment) @comment

; Interpolation
(interpolation) @embedded

; Operators
"+" @operator
"-" @operator
"*" @operator
"/" @operator
"%" @operator
"**" @operator
"==" @operator
"!=" @operator
"<" @operator
">" @operator
"<=" @operator
">=" @operator
"<=>" @operator
"&&" @operator
"||" @operator
"!" @operator
"&" @operator
"|" @operator
"^" @operator
"~" @operator
"<<" @operator
">>" @operator
"=" @operator
"+=" @operator
"-=" @operator
"*=" @operator
"/=" @operator
"%=" @operator
"**=" @operator

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
"::" @punctuation.delimiter
":" @punctuation.delimiter