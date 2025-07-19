# Example highlight queries for C++

; C++ Keywords (extending C)
"alignas" @keyword
"alignof" @keyword
"and" @keyword
"and_eq" @keyword
"asm" @keyword
"auto" @keyword
"bitand" @keyword
"bitor" @keyword
"bool" @keyword
"break" @keyword
"case" @keyword
"catch" @keyword
"char" @keyword
"char16_t" @keyword
"char32_t" @keyword
"class" @keyword
"compl" @keyword
"concept" @keyword
"const" @keyword
"constexpr" @keyword
"const_cast" @keyword
"continue" @keyword
"decltype" @keyword
"default" @keyword
"delete" @keyword
"do" @keyword
"double" @keyword
"dynamic_cast" @keyword
"else" @keyword
"enum" @keyword
"explicit" @keyword
"export" @keyword
"extern" @keyword
"false" @keyword
"float" @keyword
"for" @keyword
"friend" @keyword
"goto" @keyword
"if" @keyword
"inline" @keyword
"int" @keyword
"long" @keyword
"mutable" @keyword
"namespace" @keyword
"new" @keyword
"noexcept" @keyword
"not" @keyword
"not_eq" @keyword
"nullptr" @keyword
"operator" @keyword
"or" @keyword
"or_eq" @keyword
"private" @keyword
"protected" @keyword
"public" @keyword
"register" @keyword
"reinterpret_cast" @keyword
"requires" @keyword
"return" @keyword
"short" @keyword
"signed" @keyword
"sizeof" @keyword
"static" @keyword
"static_assert" @keyword
"static_cast" @keyword
"struct" @keyword
"switch" @keyword
"template" @keyword
"this" @keyword
"thread_local" @keyword
"throw" @keyword
"true" @keyword
"try" @keyword
"typedef" @keyword
"typeid" @keyword
"typename" @keyword
"union" @keyword
"unsigned" @keyword
"using" @keyword
"virtual" @keyword
"void" @keyword
"volatile" @keyword
"wchar_t" @keyword
"while" @keyword
"xor" @keyword
"xor_eq" @keyword

; STL types
"std::string" @type.builtin
"std::vector" @type.builtin
"std::map" @type.builtin
"std::set" @type.builtin
"std::unique_ptr" @type.builtin
"std::shared_ptr" @type.builtin
"std::weak_ptr" @type.builtin

; Functions
(function_declarator declarator: (identifier) @function)
(call_expression function: (identifier) @function.call)
(template_function declarator: (identifier) @function)

; Classes and namespaces
(class_specifier name: (type_identifier) @type)
(namespace_definition name: (identifier) @namespace)

; Variables
(identifier) @variable

; Strings
(string_literal) @string
(char_literal) @string
(raw_string_literal) @string

; Numbers
(number_literal) @number

; Comments
(comment) @comment

; Preprocessor
(preproc_include) @include
(preproc_define) @define

; Operators
"+" @operator
"-" @operator
"*" @operator
"/" @operator
"%" @operator
"++" @operator
"--" @operator
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
"::" @operator

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
"->" @punctuation.delimiter
"::" @punctuation.delimiter