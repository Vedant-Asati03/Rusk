# Example highlight queries for YAML

; Keys
(block_mapping_pair key: (flow_node) @property)
(block_mapping_pair key: (plain_scalar) @property)
(flow_mapping (_ key: (flow_node) @property))

; Strings
(double_quote_scalar) @string
(single_quote_scalar) @string
(literal_scalar) @string
(folded_scalar) @string

; Numbers
(plain_scalar) @number
  (#match? @number "^[0-9]+$")

; Booleans
(plain_scalar) @constant.builtin
  (#match? @constant.builtin "^(true|false|yes|no|on|off)$")

; Null
(plain_scalar) @constant.builtin
  (#match? @constant.builtin "^(null|~)$")

; Comments
(comment) @comment

; Anchors and aliases
(anchor) @label
(alias) @label

; Tags
(tag) @type

; Document markers
"---" @punctuation.delimiter
"..." @punctuation.delimiter

; List markers
"-" @punctuation.delimiter

; Punctuation
":" @punctuation.delimiter
"," @punctuation.delimiter
"[" @punctuation.bracket
"]" @punctuation.bracket
"{" @punctuation.bracket
"}" @punctuation.bracket