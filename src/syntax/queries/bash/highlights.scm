# Example highlight queries for Bash/Shell

; Keywords
"if" @keyword
"then" @keyword
"else" @keyword
"elif" @keyword
"fi" @keyword
"case" @keyword
"esac" @keyword
"for" @keyword
"while" @keyword
"until" @keyword
"do" @keyword
"done" @keyword
"in" @keyword
"function" @keyword
"select" @keyword
"time" @keyword
"declare" @keyword
"local" @keyword
"export" @keyword
"readonly" @keyword
"unset" @keyword
"return" @keyword
"exit" @keyword
"break" @keyword
"continue" @keyword

; Built-in commands
"echo" @function.builtin
"printf" @function.builtin
"read" @function.builtin
"cd" @function.builtin
"pwd" @function.builtin
"ls" @function.builtin
"cp" @function.builtin
"mv" @function.builtin
"rm" @function.builtin
"mkdir" @function.builtin
"rmdir" @function.builtin
"chmod" @function.builtin
"chown" @function.builtin
"grep" @function.builtin
"sed" @function.builtin
"awk" @function.builtin
"sort" @function.builtin
"uniq" @function.builtin
"cut" @function.builtin
"tr" @function.builtin
"cat" @function.builtin
"head" @function.builtin
"tail" @function.builtin
"wc" @function.builtin
"find" @function.builtin
"which" @function.builtin
"type" @function.builtin
"alias" @function.builtin
"source" @function.builtin
"test" @function.builtin

; Variables
(variable_name) @variable
(parameter_expansion) @variable
(command_substitution) @variable

; Strings
(string) @string
(raw_string) @string
(ansi_c_quoting) @string

; Numbers
(number) @number

; Comments
(comment) @comment

; Functions
(function_definition name: (word) @function)

; Commands
(command name: (word) @function.call)

; File descriptors
(file_redirect) @operator

; Operators
"&&" @operator
"||" @operator
"|" @operator
"&" @operator
";" @operator
">" @operator
">>" @operator
"<" @operator
"<<" @operator
"=" @operator

; Punctuation
"(" @punctuation.bracket
")" @punctuation.bracket
"[" @punctuation.bracket
"]" @punctuation.bracket
"{" @punctuation.bracket
"}" @punctuation.bracket
"$" @punctuation.special
"#" @punctuation.special