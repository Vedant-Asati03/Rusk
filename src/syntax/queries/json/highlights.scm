# Example highlight queries for JSON

; Objects
(object) @punctuation.bracket
"{" @punctuation.bracket
"}" @punctuation.bracket

; Arrays
(array) @punctuation.bracket
"[" @punctuation.bracket
"]" @punctuation.bracket

; Keys
(pair key: (string) @property)

; Values
(string) @string
(number) @number
(true) @constant.builtin
(false) @constant.builtin
(null) @constant.builtin

; Punctuation
"," @punctuation.delimiter
":" @punctuation.delimiter

; Escape sequences
(escape_sequence) @constant.character.escape