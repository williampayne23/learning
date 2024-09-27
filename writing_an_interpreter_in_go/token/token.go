package token

type TokenType string

const (
	ILLEGAL = "ILLEGAL"
	EOF     = "EOF"

	// Identifiers + literals
	IDENT = "IDENT" // add, foobar, x, y, ...
	INT   = "INT"   // 1343456

	// Operators
	ASSIGN   = "="
	PLUS     = "+"
	MINUS    = "-"
	BANG     = "!"
	ASTERISK = "*"
	SLASH    = "/"

    DECREMENT = "--"
    INCREMENT = "++"

	LT = "<"
    LTE = "<="
	GT = ">"
    GTE = ">="

	EQ     = "=="
	NOT_EQ = "!="

	// Delimiters
	COMMA     = ","
	SEMICOLON = ";"

	LPAREN = "("
	RPAREN = ")"
	LBRACE = "{"
	RBRACE = "}"

	// Keywords
	FUNCTION = "FUNCTION"
	LET      = "LET"
	TRUE     = "TRUE"
	FALSE    = "FALSE"
	IF       = "IF"
	ELSE     = "ELSE"
	RETURN   = "RETURN"
)

type Token struct {
	Type    TokenType
	Literal string
}

var Keywords = map[string]TokenType{
	"fn":     FUNCTION,
	"let":    LET,
	"true":   TRUE,
	"false":  FALSE,
	"if":     IF,
	"else":   ELSE,
	"return": RETURN,
}

var Symbols = map[string]TokenType{
    "=":  ASSIGN,
    "+":  PLUS,
    "-":  MINUS,
    "!":  BANG,
    "*":  ASTERISK,
    "/":  SLASH,
    "<":  LT,
    ">":  GT,
    ",":  COMMA,
    ";":  SEMICOLON,
    "(":  LPAREN,
    ")":  RPAREN,
    "{":  LBRACE,
    "}":  RBRACE,
}

var MultiCharSymbols = map[string]TokenType{
    "<=": LTE,
    ">=": GTE,
    "==": EQ,
    "!=": NOT_EQ,
    "++": INCREMENT,
    "--": DECREMENT,
}
