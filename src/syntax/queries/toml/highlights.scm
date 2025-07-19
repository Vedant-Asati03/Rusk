# Example highlight queries for TOML

; Keys
(bare_key) @property
(quoted_key) @property

; Values
(string) @string
(integer) @number
(float) @number
(boolean) @constant.builtin
(offset_date_time) @constant
(local_date_time) @constant
(local_date) @constant
(local_time) @constant

; Arrays
(array) @punctuation.bracket
"[" @punctuation.bracket
"]" @punctuation.bracket

; Tables
(table) @type
(table_array_element) @type

; Comments
(comment) @comment

; Punctuation
"=" @operator
"," @punctuation.delimiter
"." @punctuation.delimiter
"[[" @punctuation.bracket
"]]" @punctuation.bracket