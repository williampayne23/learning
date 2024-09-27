package lexer

import (
	"strings"
	"wl/token"
)

type Lexer struct {
	input        string
	position     int  // current position in input (points to current char)
	readPosition int  // current reading position in input (after current char)
	ch           byte // current char under examination
}

func New(input string) *Lexer {
	l := &Lexer{input: input}
	l.readChar()
	return l
}

func (l *Lexer) readChar() {
	if l.readPosition >= len(l.input) {
		l.ch = 0
	} else {
		l.ch = l.input[l.readPosition]
	}
	l.position = l.readPosition
	l.readPosition += 1
}

func (l *Lexer) NextToken() token.Token {
	l.skipWhitespace()
	if tok, ok := tryMultiCharSymbol(l, l.ch); ok {
        l.readChar()
		return tok
	}

	if tok, ok := trySingleCharSymbol(l); ok {
        l.readChar()
		return tok
	}

	if tok, ok := tryKeywordOrIdent(l); ok {
        l.readChar()
		return tok
	}

	if tok, ok := tryNumber(l); ok {
        l.readChar()
		return tok
	}

    if tok, ok := tryEOF(l); ok {
        l.readChar()
        return tok
    }

    // If we reach this point, we have an illegal character
    tok := token.Token{Type: token.ILLEGAL, Literal: string(l.ch)}
    l.readChar()
    return tok
}

func newToken(tokenType token.TokenType, ch byte) token.Token {
	return token.Token{Type: tokenType, Literal: string(ch)}
}

func (l *Lexer) readIdentifier() string {
	position := l.position
	for isIdentifierCharacter(l.ch) {
		l.readChar()
	}
	return l.input[position:l.position]
}

const NonAsciiIdentifierCharacters = "_"

func isIdentifierCharacter(ch byte) bool {
	return 'a' <= ch && ch <= 'z' || 'A' <= ch && ch <= 'Z' || strings.ContainsRune(NonAsciiIdentifierCharacters, rune(ch))
}

func (l *Lexer) skipWhitespace() {
	for l.ch == ' ' || l.ch == '\t' || l.ch == '\n' || l.ch == '\r' {
		l.readChar()
	}
}

func (l *Lexer) readNumber() string {
	position := l.position
	for isDigit(l.ch) {
		l.readChar()
	}
	return l.input[position:l.position]
}

func isDigit(ch byte) bool {
	return '0' <= ch && ch <= '9'
}

func (l *Lexer) peekChar(n int) byte {
	if l.readPosition - 1 +n >= len(l.input) {
		return 0
	} else {
		return l.input[l.readPosition - 1 + n]
	}
}

func tryMultiCharSymbol(l *Lexer, ch byte) (token.Token, bool) {
	// Try to read a multi-character symbol
	possibleMultiCharSymbols := possibleMultiCharSymbols(ch)
    longestSymbol := token.Token{Type: token.ILLEGAL, Literal: ""}
    longestSymbolLength := 0
	for symbol, tok := range possibleMultiCharSymbols {
        symbol_length := len(symbol)
        if symbol_length < longestSymbolLength {
            continue
        }
        if l.readPosition + symbol_length - 1> len(l.input) {
            continue
        }
        lookahead := l.input[l.position:l.readPosition + symbol_length - 1] 
        if lookahead == symbol {
            longestSymbol = token.Token{Type: tok, Literal: symbol}
            longestSymbolLength = symbol_length
        }
	}
    if longestSymbolLength > 0 {
        for i := 1; i < longestSymbolLength; i++ {
            l.readChar()
        }
        return longestSymbol, true
    }
	return token.Token{Type: token.ILLEGAL, Literal: string(ch)}, false
}

func possibleMultiCharSymbols(ch byte) map[string]token.TokenType {
	// Return the set of possible multi-character symbols that start with the given character
	// Initialize the set of possible multi-character symbols
	possibleMultiCharSymbols := make(map[string]token.TokenType)
	for symbol, tok := range token.MultiCharSymbols {
		if symbol[0] == ch {
			possibleMultiCharSymbols[symbol] = tok
		}
	}
	return possibleMultiCharSymbols
}

func trySingleCharSymbol(l *Lexer) (token.Token, bool) {
	// Try to read a single-character symbol
	if tok, ok := token.Symbols[string(l.ch)]; ok {
		return token.Token{Type: tok, Literal: string(l.ch)}, true
	}
	return token.Token{Type: token.ILLEGAL, Literal: string(l.ch)}, false
}

func tryKeywordOrIdent(l *Lexer) (token.Token, bool) {
    // Try to read a keyword or identifier
    position := l.position
    if !isIdentifierCharacter(l.ch) {
        return token.Token{Type: token.ILLEGAL, Literal: string(l.ch)}, false
    }

    for isIdentifierCharacter(l.peekChar(1)) {
        l.readChar()
    }
    ident := l.input[position:l.position + 1] 
    if tok, ok := token.Keywords[ident]; ok {
        return token.Token{Type: tok, Literal: ident}, true
    }
    return token.Token{Type: token.IDENT, Literal: ident}, true
}

func tryNumber(l *Lexer) (token.Token, bool) {
    // Try to read a number
    position := l.position
    if !isDigit(l.ch) {
        return token.Token{Type: token.ILLEGAL, Literal: string(l.ch)}, false
    }

    // We peek to avoid reading the next character. When we do
    // reach a non digit out pointer will be at the last digit
    // Which matches the expected behavior
    for isDigit(l.peekChar(1)) {
        l.readChar()
    }
    return token.Token{Type: token.INT, Literal: l.input[position:l.position + 1]}, true
}

func tryEOF(l *Lexer) (token.Token, bool) {
    // Try to read EOF
    if l.ch == 0 {
        return token.Token{Type: token.EOF, Literal: ""}, true
    }
    return token.Token{Type: token.ILLEGAL, Literal: string(l.ch)}, false
}
