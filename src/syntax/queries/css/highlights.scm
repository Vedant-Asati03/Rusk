# Example highlight queries for CSS

; Selectors
(tag_name) @tag
(class_selector) @property
(id_selector) @property
(attribute_selector) @property
(pseudo_class_selector) @property
(pseudo_element_selector) @property

; Properties
(property_name) @property

; Values
(property_value) @value
(integer_value) @number
(float_value) @number
(color_value) @constant
(string_value) @string
(url_value) @string

; Units
(unit) @type

; Keywords
"@import" @keyword
"@media" @keyword
"@keyframes" @keyword
"@font-face" @keyword
"@charset" @keyword
"@namespace" @keyword
"@supports" @keyword
"@page" @keyword

; Important
"!important" @keyword

; Functions
(function_name) @function

; Comments
(comment) @comment

; Punctuation
"{" @punctuation.bracket
"}" @punctuation.bracket
"(" @punctuation.bracket
")" @punctuation.bracket
"[" @punctuation.bracket
"]" @punctuation.bracket
";" @punctuation.delimiter
"," @punctuation.delimiter
":" @punctuation.delimiter

; Colors
"red" @constant
"blue" @constant
"green" @constant
"white" @constant
"black" @constant
"yellow" @constant
"orange" @constant
"purple" @constant
"pink" @constant
"brown" @constant
"gray" @constant
"grey" @constant