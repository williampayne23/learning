package lexer

import (
	"testing"

	"wl/token"
)

type testResult = []struct {
	expectedType    token.TokenType
	expectedLiteral string
}

var tests = []struct {
	input    string
	expected testResult
}{
	{
		input: `let five = 5;
let ten = 10;`,
		expected: testResult{
			{token.LET, "let"},
			{token.IDENT, "five"},
			{token.ASSIGN, "="},
			{token.INT, "5"},
			{token.SEMICOLON, ";"},
			{token.LET, "let"},
			{token.IDENT, "ten"},
			{token.ASSIGN, "="},
			{token.INT, "10"},
			{token.SEMICOLON, ";"},
			{token.EOF, ""},
		},
	},
	{
		input: `let add = fn(x, y) {
    x + y;
};`,
		expected: testResult{
			{token.LET, "let"},
			{token.IDENT, "add"},
			{token.ASSIGN, "="},
			{token.FUNCTION, "fn"},
			{token.LPAREN, "("},
			{token.IDENT, "x"},
			{token.COMMA, ","},
			{token.IDENT, "y"},
			{token.RPAREN, ")"},
			{token.LBRACE, "{"},
			{token.IDENT, "x"},
			{token.PLUS, "+"},
			{token.IDENT, "y"},
			{token.SEMICOLON, ";"},
			{token.RBRACE, "}"},
			{token.SEMICOLON, ";"},
			{token.EOF, ""},
		},
	},
	{
		input: `let result = add(five, ten);`,
		expected: testResult{
			{token.LET, "let"},
			{token.IDENT, "result"},
			{token.ASSIGN, "="},
			{token.IDENT, "add"},
			{token.LPAREN, "("},
			{token.IDENT, "five"},
			{token.COMMA, ","},
			{token.IDENT, "ten"},
			{token.RPAREN, ")"},
			{token.SEMICOLON, ";"},
			{token.EOF, ""},
		},
	},
	{
		input: `!-/*5;
5 < 10 > 5;`,
		expected: testResult{
			{token.BANG, "!"},
			{token.MINUS, "-"},
			{token.SLASH, "/"},
			{token.ASTERISK, "*"},
			{token.INT, "5"},
			{token.SEMICOLON, ";"},
			{token.INT, "5"},
			{token.LT, "<"},
			{token.INT, "10"},
			{token.GT, ">"},
			{token.INT, "5"},
			{token.SEMICOLON, ";"},
			{token.EOF, ""},
		},
	},
	{
		input: `if (5 < 10) {
    return true;
} else {
    return false;
}`,
		expected: testResult{
			{token.IF, "if"},
			{token.LPAREN, "("},
			{token.INT, "5"},
			{token.LT, "<"},
			{token.INT, "10"},
			{token.RPAREN, ")"},
			{token.LBRACE, "{"},
			{token.RETURN, "return"},
			{token.TRUE, "true"},
			{token.SEMICOLON, ";"},
			{token.RBRACE, "}"},
			{token.ELSE, "else"},
			{token.LBRACE, "{"},
			{token.RETURN, "return"},
			{token.FALSE, "false"},
			{token.SEMICOLON, ";"},
			{token.RBRACE, "}"},
			{token.EOF, ""},
		},
	},
	{
		input: `10 == 10;
10 != 9;`,
		expected: testResult{
			{token.INT, "10"},
			{token.EQ, "=="},
			{token.INT, "10"},
			{token.SEMICOLON, ";"},
			{token.INT, "10"},
			{token.NOT_EQ, "!="},
			{token.INT, "9"},
			{token.SEMICOLON, ";"},
			{token.EOF, ""},
		},
	},
    {
        input: `:w`,
        expected: testResult{
               {token.ILLEGAL, ":"},     
                {token.IDENT, "w"},
                {token.EOF, ""},
        },
    },
    {   
        input: `x++; y--;`,
        expected: testResult{
            {token.IDENT, "x"},
            {token.INCREMENT, "++"},
            {token.SEMICOLON, ";"},
            {token.IDENT, "y"},
            {token.DECREMENT, "--"},
            {token.SEMICOLON, ";"},
            {token.EOF, ""},
        },
    },
    {
        input: `<= >=`,
        expected: testResult{
            {token.LTE, "<="},
            {token.GTE, ">="},
            {token.EOF, ""},
        },
    },
}

func TestNextToken(t *testing.T) {
	for _, tt := range tests {
		SingleTest(t, tt.input, tt.expected)
	}
}

func SingleTest(t *testing.T, input string, output []struct {
	expectedType    token.TokenType
	expectedLiteral string
}) {
	l := New(input)

	for i, tt := range output {
		tok := l.NextToken()

		if tok.Type != tt.expectedType {
			t.Fatalf("tests[%d] - tokentype wrong. expected=%q, got=%q\n\t\tliteral=%q\nwhen reading\n%q",
				i, tt.expectedType, tok.Type, tok.Literal, input)
		}

		if tok.Literal != tt.expectedLiteral {
			t.Fatalf("tests[%d] - literal wrong. expected=%q, got=%q",
				i, tt.expectedLiteral, tok.Literal)
		}
	}

}
