// Highlight queries adapted from Lumen (jnsahaj/lumen) for improved
// recognition of keywords, types, function names, and operator disambiguation.

pub const TS_HIGHLIGHTS: &str = r#"
(comment) @comment
(string) @string
(template_string) @string
(number) @number
(true) @constant.builtin
(false) @constant.builtin
(null) @constant.builtin
(undefined) @constant.builtin
(regex) @string.special

["const" "let" "var" "function" "class" "interface" "type" "enum" "namespace" "module" "declare" "implements" "extends" "public" "private" "protected" "readonly" "static" "abstract" "async" "await" "return" "if" "else" "for" "while" "do" "switch" "case" "default" "break" "continue" "try" "catch" "finally" "throw" "new" "delete" "typeof" "instanceof" "in" "of" "as" "is" "import" "export" "from" "default" "void"] @keyword

(type_identifier) @type
(predefined_type) @type.builtin

(function_declaration name: (identifier) @function)
(method_definition name: (property_identifier) @function.method)
(call_expression function: (identifier) @function)
(call_expression function: (member_expression property: (property_identifier) @function.method))
(arrow_function) @function

(property_identifier) @property
(shorthand_property_identifier) @property
(shorthand_property_identifier_pattern) @property

["(" ")" "[" "]" "{" "}"] @punctuation.bracket
["." "," ";" ":"] @punctuation.delimiter
"#;

pub const TSX_HIGHLIGHTS: &str = r#"
(comment) @comment
(string) @string
(template_string) @string
(number) @number
(true) @constant.builtin
(false) @constant.builtin
(null) @constant.builtin
(undefined) @constant.builtin
(regex) @string.special

["const" "let" "var" "function" "class" "interface" "type" "enum" "namespace" "module" "declare" "implements" "extends" "public" "private" "protected" "readonly" "static" "abstract" "async" "await" "return" "if" "else" "for" "while" "do" "switch" "case" "default" "break" "continue" "try" "catch" "finally" "throw" "new" "delete" "typeof" "instanceof" "in" "of" "as" "is" "import" "export" "from" "default" "void"] @keyword

(type_identifier) @type
(predefined_type) @type.builtin

(function_declaration name: (identifier) @function)
(method_definition name: (property_identifier) @function.method)
(call_expression function: (identifier) @function)
(call_expression function: (member_expression property: (property_identifier) @function.method))
(arrow_function) @function

(property_identifier) @property
(shorthand_property_identifier) @property
(shorthand_property_identifier_pattern) @property

(jsx_element open_tag: (jsx_opening_element name: (identifier) @tag))
(jsx_element close_tag: (jsx_closing_element name: (identifier) @tag))
(jsx_self_closing_element name: (identifier) @tag)
(jsx_attribute (property_identifier) @attribute)

["(" ")" "[" "]" "{" "}"] @punctuation.bracket
["." "," ";" ":"] @punctuation.delimiter
"#;

pub const JS_HIGHLIGHTS: &str = r#"
(comment) @comment
(string) @string
(template_string) @string
(number) @number
(true) @constant.builtin
(false) @constant.builtin
(null) @constant.builtin
(undefined) @constant.builtin
(regex) @string.special

["const" "let" "var" "function" "class" "extends" "async" "await" "return" "if" "else" "for" "while" "do" "switch" "case" "default" "break" "continue" "try" "catch" "finally" "throw" "new" "delete" "typeof" "instanceof" "in" "of" "import" "export" "from" "default" "void"] @keyword

(function_declaration name: (identifier) @function)
(method_definition name: (property_identifier) @function.method)
(call_expression function: (identifier) @function)
(call_expression function: (member_expression property: (property_identifier) @function.method))
(arrow_function) @function

(property_identifier) @property
(shorthand_property_identifier) @property

["(" ")" "[" "]" "{" "}"] @punctuation.bracket
["." "," ";" ":"] @punctuation.delimiter
"#;

pub const RUST_HIGHLIGHTS: &str = r#"
; Comments
(line_comment) @comment
(block_comment) @comment
(outer_doc_comment_marker) @comment
(inner_doc_comment_marker) @comment
(doc_comment) @comment

; Strings and literals
(string_literal) @string
(raw_string_literal) @string
(char_literal) @string
(integer_literal) @number
(float_literal) @number
(boolean_literal) @constant.builtin

; Types
(type_identifier) @type
(primitive_type) @type.builtin

; Functions
(function_item (identifier) @function)
(function_signature_item (identifier) @function)
(call_expression function: (identifier) @function)
(call_expression function: (field_expression field: (field_identifier) @function.method))
(call_expression function: (scoped_identifier name: (identifier) @function))
(generic_function function: (identifier) @function)
(generic_function function: (scoped_identifier name: (identifier) @function))

; Macros
(macro_invocation macro: (identifier) @function.macro "!" @function.macro)
(macro_definition "macro_rules!" @function.macro)

; Fields and properties
(field_identifier) @variable.member
(shorthand_field_identifier) @variable.member

; Labels and lifetimes
(lifetime (identifier) @label)

; Parameters
(parameter (identifier) @variable.parameter)

; Modules
(mod_item name: (identifier) @module)
(scoped_identifier path: (identifier) @module)

; Special
(self) @variable.builtin
(crate) @keyword
(super) @keyword
(mutable_specifier) @keyword

; Keywords
"as" @keyword
"async" @keyword
"await" @keyword
"break" @keyword
"const" @keyword
"continue" @keyword
"dyn" @keyword
"else" @keyword
"enum" @keyword
"extern" @keyword
"fn" @keyword
"for" @keyword
"if" @keyword
"impl" @keyword
"in" @keyword
"let" @keyword
"loop" @keyword
"match" @keyword
"mod" @keyword
"move" @keyword
"pub" @keyword
"ref" @keyword
"return" @keyword
"static" @keyword
"struct" @keyword
"trait" @keyword
"type" @keyword
"unsafe" @keyword
"use" @keyword
"where" @keyword
"while" @keyword

; Operators
"*" @operator
"&" @operator
"=" @operator
"+" @operator
"-" @operator
"%" @operator
"<" @operator
">" @operator
"==" @operator
"!=" @operator
"<=" @operator
">=" @operator
"&&" @operator
"||" @operator
"+=" @operator
"-=" @operator
"*=" @operator
"/=" @operator
".." @operator
"..=" @operator
"=>" @operator
"->" @operator
"?" @operator

(binary_expression "/" @operator)
(unary_expression "!" @operator)

"(" @punctuation.bracket
")" @punctuation.bracket
"[" @punctuation.bracket
"]" @punctuation.bracket
"{" @punctuation.bracket
"}" @punctuation.bracket
"::" @punctuation.delimiter
":" @punctuation.delimiter
"#;

pub const PYTHON_HIGHLIGHTS: &str = r#"
(comment) @comment
(string) @string
(escape_sequence) @string.special

(integer) @number
(float) @number
(none) @constant.builtin
(true) @constant.builtin
(false) @constant.builtin

(type (identifier) @type)
(attribute attribute: (identifier) @property)

(function_definition name: (identifier) @function)
(call function: (identifier) @function)
(call function: (attribute attribute: (identifier) @function.method))
(decorator) @function
(decorator (identifier) @function)

"as" @keyword
"assert" @keyword
"async" @keyword
"await" @keyword
"break" @keyword
"class" @keyword
"continue" @keyword
"def" @keyword
"del" @keyword
"elif" @keyword
"else" @keyword
"except" @keyword
"finally" @keyword
"for" @keyword
"from" @keyword
"global" @keyword
"if" @keyword
"import" @keyword
"lambda" @keyword
"nonlocal" @keyword
"pass" @keyword
"raise" @keyword
"return" @keyword
"try" @keyword
"while" @keyword
"with" @keyword
"yield" @keyword
"match" @keyword
"case" @keyword
"and" @operator
"or" @operator
"not" @operator
"in" @operator
"is" @operator
"#;

pub const GO_HIGHLIGHTS: &str = r#"
(comment) @comment
(interpreted_string_literal) @string
(raw_string_literal) @string
(rune_literal) @string

(int_literal) @number
(float_literal) @number
(true) @constant.builtin
(false) @constant.builtin
(nil) @constant.builtin

(type_identifier) @type
(type_spec name: (type_identifier) @type)

(function_declaration name: (identifier) @function)
(method_declaration name: (field_identifier) @function.method)
(call_expression function: (identifier) @function)
(call_expression function: (selector_expression field: (field_identifier) @function.method))

(field_identifier) @property

(package_identifier) @module

"break" @keyword
"case" @keyword
"chan" @keyword
"const" @keyword
"continue" @keyword
"default" @keyword
"defer" @keyword
"else" @keyword
"fallthrough" @keyword
"for" @keyword
"func" @keyword
"go" @keyword
"goto" @keyword
"if" @keyword
"import" @keyword
"interface" @keyword
"map" @keyword
"package" @keyword
"range" @keyword
"return" @keyword
"select" @keyword
"struct" @keyword
"switch" @keyword
"type" @keyword
"var" @keyword
"#;

pub const JSON_HIGHLIGHTS: &str = r#"
(string) @string
(number) @number
(true) @constant.builtin
(false) @constant.builtin
(null) @constant.builtin
(pair key: (string) @property)

"[" @punctuation.bracket
"]" @punctuation.bracket
"{" @punctuation.bracket
"}" @punctuation.bracket
":" @punctuation.delimiter
"," @punctuation.delimiter
"#;

pub const BASH_HIGHLIGHTS: &str = r#"
(comment) @comment
(string) @string
(raw_string) @string
(number) @number
(command_name) @function
(variable_name) @variable
"#;
