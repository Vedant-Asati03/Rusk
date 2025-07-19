# Example highlight queries for TypeScript

; Keywords (extending JavaScript)
"abstract" @keyword
"as" @keyword
"declare" @keyword
"enum" @keyword
"implements" @keyword
"interface" @keyword
"namespace" @keyword
"private" @keyword
"protected" @keyword
"public" @keyword
"readonly" @keyword
"type" @keyword

; JavaScript keywords
"async" @keyword
"await" @keyword
"break" @keyword
"case" @keyword
"catch" @keyword
"class" @keyword
"const" @keyword
"continue" @keyword
"debugger" @keyword
"default" @keyword
"delete" @keyword
"do" @keyword
"else" @keyword
"export" @keyword
"extends" @keyword
"finally" @keyword
"for" @keyword
"function" @keyword
"if" @keyword
"import" @keyword
"in" @keyword
"instanceof" @keyword
"let" @keyword
"new" @keyword
"return" @keyword
"static" @keyword
"super" @keyword
"switch" @keyword
"this" @keyword
"throw" @keyword
"try" @keyword
"typeof" @keyword
"var" @keyword
"void" @keyword
"while" @keyword
"with" @keyword
"yield" @keyword

; Types
(type_identifier) @type
(predefined_type) @type.builtin

; Functions
(function_declaration name: (identifier) @function)
(function_expression name: (identifier)? @function)
(arrow_function) @function
(call_expression function: (identifier) @function)
(method_definition name: (property_identifier) @function.method)

; Variables and properties
(identifier) @variable
(property_identifier) @property

; Strings
(string) @string
(template_string) @string

; Numbers
(number) @number

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
"==" @operator
"===" @operator
"!=" @operator
"!==" @operator
"<" @operator
">" @operator
"<=" @operator
">=" @operator
"&&" @operator
"||" @operator
"!" @operator
"?" @operator
":" @operator
"=>" @operator

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