# Example highlight queries for XML

; Elements
(element (start_tag (name) @tag))
(element (end_tag (name) @tag))
(self_closing_tag (name) @tag)

; Attributes
(attribute (name) @attribute)
(attribute (quoted_attribute_value) @string)

; Processing instructions
(processing_instruction) @keyword

; XML declaration
(xml_decl) @keyword

; DOCTYPE
(doctype_decl) @keyword

; Comments
(comment) @comment

; CDATA
(cdata_section) @string

; Text content
(content) @text

; Entity references
(entity_ref) @constant.character.escape
(char_ref) @constant.character.escape

; Punctuation
"<" @punctuation.bracket
">" @punctuation.bracket
"</" @punctuation.bracket
"/>" @punctuation.bracket
"<?" @punctuation.bracket
"?>" @punctuation.bracket
"=" @operator