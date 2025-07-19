# Example highlight queries for Make

; Targets
(rule) @function

; Variables
(variable_reference) @variable
(variable_assignment variable: (word) @variable)

; Built-in variables
"$@" @variable.builtin
"$<" @variable.builtin
"$^" @variable.builtin
"$?" @variable.builtin
"$*" @variable.builtin
"$+" @variable.builtin
"$|" @variable.builtin
"$$" @variable.builtin

; Directives
"include" @include
"sinclude" @include
"-include" @include
"ifeq" @conditional
"ifneq" @conditional
"ifdef" @conditional
"ifndef" @conditional
"else" @conditional
"endif" @conditional

; Functions
"$(shell" @function.builtin
"$(wildcard" @function.builtin
"$(patsubst" @function.builtin
"$(subst" @function.builtin
"$(strip" @function.builtin
"$(findstring" @function.builtin
"$(filter" @function.builtin
"$(sort" @function.builtin
"$(word" @function.builtin
"$(words" @function.builtin
"$(firstword" @function.builtin
"$(lastword" @function.builtin
"$(dir" @function.builtin
"$(notdir" @function.builtin
"$(suffix" @function.builtin
"$(basename" @function.builtin
"$(addsuffix" @function.builtin
"$(addprefix" @function.builtin
"$(join" @function.builtin
"$(foreach" @function.builtin
"$(if" @function.builtin
"$(call" @function.builtin
"$(eval" @function.builtin
"$(origin" @function.builtin
"$(flavor" @function.builtin

; Strings
(string) @string

; Comments
(comment) @comment

; Recipe lines
(recipe_line) @text

; Operators
"=" @operator
":=" @operator
"?=" @operator
"+=" @operator
"!=" @operator

; Punctuation
":" @punctuation.delimiter
";" @punctuation.delimiter
"(" @punctuation.bracket
")" @punctuation.bracket