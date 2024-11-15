package parser

import (
	"testing"
	"wl/ast"
	"wl/lexer"
)

var letTests = []struct {
	input              string
	valid              bool
	expectedIdentifier string
}{
	{"let x = 5;", true, "x"},
	{"let y = 10;", true, "y"},
	{"let foobar = 838383;", true, "foobar"},
	{"let foobar 838383;", false, "foobar"},
}

func TestLetStatements(t *testing.T) {
	for _, tt := range letTests {
		lex := lexer.New(tt.input)
		parse := New(lex)
		program := parse.ParseProgram()
		var errors = checkParserErrors(t, parse)
		if errors == tt.valid {
			t.Fatalf("checkParserErrors() returned %t", errors)
		}
		if errors {
			continue
		}
		if program == nil {
			t.Fatalf("ParseProgram() returned nil")
		}
		if len(program.Statements) != 1 {
			t.Fatalf("program.Statements does not contain 1 statement. got=%d",
				len(program.Statements))
		}
		stmt := program.Statements[0]
		if !testLetStatement(t, stmt, tt.expectedIdentifier) {
			return
		}
	}
}

func testLetStatement(t *testing.T, s ast.Statement, name string) bool {
	if s.TokenLiteral() != "let" {
		t.Errorf("s.TokenLiteral not 'let'. got=%q", s.TokenLiteral())
		return false
	}
	letStmt, ok := s.(*ast.LetStatement)
	if !ok {
		t.Errorf("s not *ast.LetStatement. got=%T", s)
		return false
	}
	if letStmt.Name.Value != name {
		t.Errorf("letStmt.Name.Value not '%s'. got=%s", name, letStmt.Name.Value)
		return false
	}
	if letStmt.Name.TokenLiteral() != name {
		t.Errorf("letStmt.Name.TokenLiteral() not '%s'. got=%s",
			name, letStmt.Name.TokenLiteral())
		return false
	}
	return true
}

var returnTests = []struct {
	input string
	valid bool
}{
	{"return 5;", true},
	{"return 10;", true},
}

func TestReturnStatements(t *testing.T) {
	for _, tt := range returnTests {
		lex := lexer.New(tt.input)
		parse := New(lex)
		program := parse.ParseProgram()
		var errors = checkParserErrors(t, parse)
		if errors == tt.valid {
			t.Fatalf("checkParserErrors() returned %t for code\n%s", errors, tt.input)
		}
		if errors {
			continue
		}
		if program == nil {
			t.Fatalf("ParseProgram() returned nil")
		}
		if len(program.Statements) != 1 {
			t.Fatalf("program.Statements does not contain 1 statement. got=%d",
				len(program.Statements))
		}
		stmt := program.Statements[0]
		returnStmt, ok := stmt.(*ast.ReturnStatement)
		if !ok {
			t.Errorf("stmt not *ast.ReturnStatement. got=%T", stmt)
			continue
		}
		if returnStmt.TokenLiteral() != "return" {
			t.Errorf("returnStmt.TokenLiteral not 'return', got %q",
				returnStmt.TokenLiteral())
		}
	}
}

func checkParserErrors(t *testing.T, p *Parser) bool {
	errors := p.Errors()
	if len(errors) == 0 {
		return false
	}
	t.Logf("parser has %d errors", len(errors))
	for _, msg := range errors {
		t.Logf("parser error: %q", msg)
	}
	return true
}
