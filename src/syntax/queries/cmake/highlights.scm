# Example highlight queries for CMake

; Commands
(normal_command) @function

; Keywords
"function" @keyword
"endfunction" @keyword
"macro" @keyword
"endmacro" @keyword
"if" @keyword
"elseif" @keyword
"else" @keyword
"endif" @keyword
"foreach" @keyword
"endforeach" @keyword
"while" @keyword
"endwhile" @keyword
"break" @keyword
"continue" @keyword
"return" @keyword

; Built-in commands
"add_executable" @function.builtin
"add_library" @function.builtin
"target_link_libraries" @function.builtin
"target_include_directories" @function.builtin
"find_package" @function.builtin
"find_library" @function.builtin
"find_path" @function.builtin
"find_file" @function.builtin
"find_program" @function.builtin
"include" @function.builtin
"include_directories" @function.builtin
"link_directories" @function.builtin
"set" @function.builtin
"unset" @function.builtin
"list" @function.builtin
"string" @function.builtin
"file" @function.builtin
"message" @function.builtin
"option" @function.builtin
"configure_file" @function.builtin
"install" @function.builtin

; Variables
(variable_ref) @variable
(variable) @variable

; Built-in variables
"CMAKE_SOURCE_DIR" @variable.builtin
"CMAKE_BINARY_DIR" @variable.builtin
"CMAKE_CURRENT_SOURCE_DIR" @variable.builtin
"CMAKE_CURRENT_BINARY_DIR" @variable.builtin
"CMAKE_INSTALL_PREFIX" @variable.builtin
"CMAKE_BUILD_TYPE" @variable.builtin
"CMAKE_CXX_STANDARD" @variable.builtin
"CMAKE_C_STANDARD" @variable.builtin

; Strings
(quoted_argument) @string
(bracket_argument) @string

; Comments
(line_comment) @comment
(bracket_comment) @comment

; Numbers
(unquoted_argument) @string
  (#match? @string "^[0-9]+$")

; Booleans
"TRUE" @constant.builtin
"FALSE" @constant.builtin
"ON" @constant.builtin
"OFF" @constant.builtin

; Punctuation
"(" @punctuation.bracket
")" @punctuation.bracket
"${" @punctuation.special
"}" @punctuation.special