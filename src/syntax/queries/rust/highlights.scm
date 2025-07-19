# Example highlight queries for Rust

; Keywords
(async) @keyword
(await) @keyword
(break) @keyword
(const) @keyword
(continue) @keyword
(crate) @keyword
(dyn) @keyword
(else) @keyword
(enum) @keyword
(extern) @keyword
(fn) @keyword
(for) @keyword
(if) @keyword
(impl) @keyword
(in) @keyword
(let) @keyword
(loop) @keyword
(match) @keyword
(mod) @keyword
(move) @keyword
(mut) @keyword
(pub) @keyword
(ref) @keyword
(return) @keyword
(self) @keyword
(static) @keyword
(struct) @keyword
(super) @keyword
(trait) @keyword
(type) @keyword
(union) @keyword
(unsafe) @keyword
(use) @keyword
(where) @keyword
(while) @keyword

; Types
(primitive_type) @type.builtin
(type_identifier) @type

; Functions
(function_item name: (identifier) @function)
(call_expression function: (identifier) @function)

; Variables
(identifier) @variable

; Strings
(string_literal) @string
(raw_string_literal) @string
(char_literal) @string

; Numbers
(integer_literal) @number
(float_literal) @number

; Comments
(line_comment) @comment
(block_comment) @comment

; Operators
"+" @operator
"-" @operator
"*" @operator
"/" @operator
"=" @operator
"==" @operator
"!=" @operator
"<" @operator
">" @operator
"<=" @operator
">=" @operator
"&&" @operator
"||" @operator
"!" @operator

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