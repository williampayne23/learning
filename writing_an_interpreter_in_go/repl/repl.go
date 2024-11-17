package repl

import (
    "bufio"
    "fmt"
    "io"
    "wl/lexer"
    "wl/parser"
    "wl/token"
)

const PROMPT = ">> "

type EvalFunc func(string) string

func Start(in io.Reader, out io.Writer, eval EvalFunc) {
    scanner := bufio.NewScanner(in)

    for {
        fmt.Printf(PROMPT)
        scanned := scanner.Scan()
        if !scanned {
            return
        }

        line := scanner.Text()
        evaluated := eval(line)
        if evaluated != "" {
            fmt.Println(evaluated)
        }
    }
}


func Lexer_eval(input string) string {
    l := lexer.New(input)

    for tok := l.NextToken(); tok.Type != token.EOF; tok = l.NextToken() {
        fmt.Printf("%+v\n", tok)
    }

    return ""
}


func Parser_eval(input string) string {
    l := lexer.New(input)
    p := parser.New(l)

    program := p.ParseProgram()
    if len(p.Errors()) != 0 {
        fmt.Println("Parser has errors:")
        for _, msg := range p.Errors() {
            fmt.Println("\t" + msg)
        }
        return ""
    }

    return program.String()
}
