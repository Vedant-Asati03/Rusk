# Example highlight queries for Vim script

; Keywords
"if" @keyword
"elseif" @keyword
"else" @keyword
"endif" @keyword
"while" @keyword
"endwhile" @keyword
"for" @keyword
"endfor" @keyword
"function" @keyword
"endfunction" @keyword
"return" @keyword
"break" @keyword
"continue" @keyword
"try" @keyword
"catch" @keyword
"finally" @keyword
"endtry" @keyword
"throw" @keyword
"let" @keyword
"unlet" @keyword
"const" @keyword
"lockvar" @keyword
"unlockvar" @keyword
"call" @keyword
"execute" @keyword
"eval" @keyword
"echo" @keyword
"echon" @keyword
"echomsg" @keyword
"echoerr" @keyword
"echohl" @keyword
"silent" @keyword
"normal" @keyword
"set" @keyword
"setlocal" @keyword
"augroup" @keyword
"autocmd" @keyword
"command" @keyword
"map" @keyword
"noremap" @keyword
"nnoremap" @keyword
"vnoremap" @keyword
"inoremap" @keyword
"cnoremap" @keyword
"syntax" @keyword
"highlight" @keyword
"match" @keyword

; Built-in functions
"strlen" @function.builtin
"substitute" @function.builtin
"expand" @function.builtin
"exists" @function.builtin
"type" @function.builtin
"empty" @function.builtin
"join" @function.builtin
"split" @function.builtin
"getline" @function.builtin
"setline" @function.builtin
"search" @function.builtin
"searchpair" @function.builtin
"match" @function.builtin
"matchend" @function.builtin
"matchstr" @function.builtin
"has" @function.builtin
"range" @function.builtin
"line" @function.builtin
"col" @function.builtin
"virtcol" @function.builtin

; Variables
(option_name) @variable.builtin
(variable_name) @variable

; Strings
(string) @string

; Numbers
(number) @number

; Comments
(comment) @comment

; Operators
"==" @operator
"!=" @operator
">" @operator
"<" @operator
">=" @operator
"<=" @operator
"=~" @operator
"!~" @operator
"." @operator
".." @operator

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

; Special
"&" @punctuation.special
"$" @punctuation.special
"%" @punctuation.special
"#" @punctuation.special