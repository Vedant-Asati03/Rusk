# Example highlight queries for Swift

; Keywords
"associatedtype" @keyword
"class" @keyword
"deinit" @keyword
"enum" @keyword
"extension" @keyword
"fileprivate" @keyword
"func" @keyword
"import" @keyword
"init" @keyword
"inout" @keyword
"internal" @keyword
"let" @keyword
"open" @keyword
"operator" @keyword
"private" @keyword
"protocol" @keyword
"public" @keyword
"static" @keyword
"struct" @keyword
"subscript" @keyword
"typealias" @keyword
"var" @keyword
"break" @keyword
"case" @keyword
"continue" @keyword
"default" @keyword
"defer" @keyword
"do" @keyword
"else" @keyword
"fallthrough" @keyword
"for" @keyword
"guard" @keyword
"if" @keyword
"in" @keyword
"repeat" @keyword
"return" @keyword
"switch" @keyword
"where" @keyword
"while" @keyword
"as" @keyword
"catch" @keyword
"false" @keyword
"is" @keyword
"nil" @keyword
"rethrows" @keyword
"super" @keyword
"self" @keyword
"Self" @keyword
"throw" @keyword
"throws" @keyword
"true" @keyword
"try" @keyword

; Access modifiers
"public" @keyword.modifier
"private" @keyword.modifier
"fileprivate" @keyword.modifier
"internal" @keyword.modifier
"open" @keyword.modifier

; Types
"Int" @type.builtin
"Double" @type.builtin
"Float" @type.builtin
"Bool" @type.builtin
"String" @type.builtin
"Character" @type.builtin
"Array" @type.builtin
"Dictionary" @type.builtin
"Set" @type.builtin
"Optional" @type.builtin
"Any" @type.builtin
"AnyObject" @type.builtin

; Functions
(function_declaration name: (simple_identifier) @function)
(call_expression function: (simple_identifier) @function.call)

; Variables
(simple_identifier) @variable

; Constants
"true" @constant.builtin
"false" @constant.builtin
"nil" @constant.builtin

; Strings
(line_string_literal) @string
(multi_line_string_literal) @string

; Numbers
(integer_literal) @number
(real_literal) @number

; Comments
(comment) @comment
(multiline_comment) @comment

; Attributes
(attribute) @attribute

; Operators
"+" @operator
"-" @operator
"*" @operator
"/" @operator
"%" @operator
"=" @operator
"+=" @operator
"-=" @operator
"*=" @operator
"/=" @operator
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
"??" @operator

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
"->" @punctuation.delimiter