# Example highlight queries for LaTeX

; Commands
(command_name) @function

; Environments
(begin) @keyword
(end) @keyword
(environment_name) @type

; Math
(math_environment) @markup.math
(inline_formula) @markup.math

; Comments
(comment) @comment

; Text formatting
(text_mode) @text

; Citations and references
(citation) @reference
(label) @label

; Sectioning
"\\section" @markup.heading
"\\subsection" @markup.heading
"\\subsubsection" @markup.heading
"\\chapter" @markup.heading
"\\part" @markup.heading

; Packages
"\\usepackage" @include
"\\documentclass" @include

; Special characters
"\\\\" @constant.character.escape
"\\&" @constant.character.escape
"\\%" @constant.character.escape
"\\$" @constant.character.escape
"\\#" @constant.character.escape
"\\_" @constant.character.escape
"\\{" @constant.character.escape
"\\}" @constant.character.escape

; Punctuation
"{" @punctuation.bracket
"}" @punctuation.bracket
"[" @punctuation.bracket
"]" @punctuation.bracket