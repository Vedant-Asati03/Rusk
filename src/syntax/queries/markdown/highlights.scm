# Example highlight queries for Markdown

; Headings
(atx_heading) @markup.heading
(setext_heading) @markup.heading

; Emphasis
(emphasis) @markup.italic
(strong_emphasis) @markup.bold

; Links
(link) @markup.link.url
(image) @markup.link.url
(autolink) @markup.link.url

; Code
(code_span) @markup.raw
(indented_code_block) @markup.raw
(fenced_code_block) @markup.raw

; Lists
(list_marker_plus) @punctuation.special
(list_marker_minus) @punctuation.special
(list_marker_star) @punctuation.special
(list_marker_dot) @punctuation.special
(list_marker_parenthesis) @punctuation.special

; Quotes
(block_quote) @markup.quote

; Tables
(pipe_table_header) @markup.heading
(pipe_table_delimiter_row) @punctuation.delimiter
(pipe_table_row) @markup

; HTML in Markdown
(html_block) @markup.raw.html
(html_tag) @markup.raw.html

; Horizontal rules
(thematic_break) @punctuation.special

; Text
(paragraph) @markup