# Example highlight queries for HTML

; HTML Tags
(element (start_tag (tag_name) @tag))
(element (end_tag (tag_name) @tag))
(self_closing_tag (tag_name) @tag)

; Attributes
(attribute (attribute_name) @attribute)
(attribute (quoted_attribute_value) @string)

; DOCTYPE
(doctype) @keyword

; Comments
(comment) @comment

; Text content
(text) @text

; Special tags
(script_element (start_tag (tag_name) @tag.special))
(script_element (end_tag (tag_name) @tag.special))
(style_element (start_tag (tag_name) @tag.special))
(style_element (end_tag (tag_name) @tag.special))

; Entity references
(entity_reference) @constant.character.escape

; Punctuation
"<" @punctuation.bracket
">" @punctuation.bracket
"</" @punctuation.bracket
"/>" @punctuation.bracket
"=" @operator