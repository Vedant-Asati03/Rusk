# Example highlight queries for PHP

; PHP tags
"<?php" @tag
"<?=" @tag
"?>" @tag

; Keywords
"abstract" @keyword
"and" @keyword
"array" @keyword
"as" @keyword
"break" @keyword
"callable" @keyword
"case" @keyword
"catch" @keyword
"class" @keyword
"clone" @keyword
"const" @keyword
"continue" @keyword
"declare" @keyword
"default" @keyword
"die" @keyword
"do" @keyword
"echo" @keyword
"else" @keyword
"elseif" @keyword
"empty" @keyword
"enddeclare" @keyword
"endfor" @keyword
"endforeach" @keyword
"endif" @keyword
"endswitch" @keyword
"endwhile" @keyword
"eval" @keyword
"exit" @keyword
"extends" @keyword
"final" @keyword
"finally" @keyword
"for" @keyword
"foreach" @keyword
"function" @keyword
"global" @keyword
"goto" @keyword
"if" @keyword
"implements" @keyword
"include" @keyword
"include_once" @keyword
"instanceof" @keyword
"insteadof" @keyword
"interface" @keyword
"isset" @keyword
"list" @keyword
"namespace" @keyword
"new" @keyword
"or" @keyword
"print" @keyword
"private" @keyword
"protected" @keyword
"public" @keyword
"require" @keyword
"require_once" @keyword
"return" @keyword
"static" @keyword
"switch" @keyword
"throw" @keyword
"trait" @keyword
"try" @keyword
"unset" @keyword
"use" @keyword
"var" @keyword
"while" @keyword
"xor" @keyword
"yield" @keyword

; Types
"bool" @type.builtin
"int" @type.builtin
"float" @type.builtin
"string" @type.builtin
"array" @type.builtin
"object" @type.builtin
"resource" @type.builtin
"null" @type.builtin
"mixed" @type.builtin
"void" @type.builtin

; Functions
(function_definition name: (name) @function)
(function_call_expression function: (name) @function.call)
(method_call_expression name: (name) @function.method)

; Variables
(variable_name) @variable

; Constants
(name) @constant
"true" @constant.builtin
"false" @constant.builtin
"null" @constant.builtin

; Strings
(string) @string
(heredoc) @string
(nowdoc) @string

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
"%" @operator
"**" @operator
"=" @operator
"+=" @operator
"-=" @operator
"*=" @operator
"/=" @operator
"%=" @operator
"**=" @operator
".=" @operator
"==" @operator
"!=" @operator
"===" @operator
"!==" @operator
"<" @operator
">" @operator
"<=" @operator
">=" @operator
"<=>" @operator
"&&" @operator
"||" @operator
"!" @operator
"and" @operator
"or" @operator
"xor" @operator
"&" @operator
"|" @operator
"^" @operator
"~" @operator
"<<" @operator
">>" @operator
"++" @operator
"--" @operator
"." @operator
"->" @operator
"=>" @operator
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
":" @punctuation.delimiter
"$" @punctuation.special