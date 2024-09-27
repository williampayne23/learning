import Context from "./context.ts";
import Position from "./position.ts";
import { Token, TokenType } from "./tokens.ts";
import Value from "./values/value.ts";

export class WLANGError {
    details: string;
    posStart: Position;
    posEnd: Position;

    constructor(details: string, startChar: Position, endChar: Position) {
        this.details = details;
        this.posStart = startChar;
        this.posEnd = endChar;
    }

    toString() {
        if(this.posStart !== undefined){
            let out =  `Error: ${this.details} at col ${this.posStart.col}:\n`;
            const lines = this.posStart.text.split("\n")
            const line = lines[this.posStart.line - 1]
            if(!line){
                return out
            }
            out += line + "\n"
            let underlineLength = this.posEnd.col - this.posStart.col
            underlineLength = underlineLength && underlineLength >= 0? underlineLength : 1;
            out += " ".repeat(this.posStart.col) + "^".repeat(underlineLength)
            return out
        }
    }
}

export class IllegalCharacterError extends WLANGError {
    constructor(posStart: Position, posEnd: Position) {
        super("Illegal character " + posStart.nextChar, posStart, posEnd);
    }
}

export class UnexpectedTokenError extends WLANGError {
    
    token: Token
    expectedTokens: TokenType[]

    constructor(token: Token, start: Position, end: Position, expectedTokens: TokenType[]) {
        const text = `Unexpected token. Expected: ${expectedTokens.map(e => TokenType[e]).join(",")} received ${token}`;
        super(text, start, end);
        this.token = token;
        this.expectedTokens = expectedTokens
        if(token.isType([TokenType.EOF])){
            
            return new UnexpectedEndOfFile(start, end, expectedTokens)
        }
    }

    static createFromSingleToken(token: Token, expectedTokens: TokenType[]): UnexpectedTokenError{
        return new UnexpectedTokenError(token, token.start.copy(), token.end.copy(), expectedTokens)
    }
}

export class UnexpectedEndOfFile extends WLANGError {

    token: Token
    expectedTokens: TokenType[]


    constructor(start: Position, end: Position, expectedTokens: TokenType[]){
        super(`Reached end of file. Expected: ${expectedTokens.map(e => TokenType[e]).join(",")}`, start, end)
        this.token = new Token(TokenType.EOF, start, end)
        this.expectedTokens = expectedTokens
    }
}

export class InvalidOperatorError extends WLANGError {
    constructor(token: Token, value: Value){
        const text = `Invalid operation. cannot perform operation ${token} on ${value}`
        super(text, token.start, token.end)
    }
}

export class DivideByZeroError extends WLANGError {
    constructor(start: Position, end: Position){
        super("Divide by zero error", start, end)
    }
}

export class NoNodeError extends WLANGError {
    constructor(){
        super("No node", new Position(0, 0, 0, "", ""), new Position(0, 0, 0, "", ""))
    }
}

export class UndefinedVariableError extends WLANGError {
    constructor(varName: string, context: Context, start: Position, end: Position){
        super(`No variable ${varName} in ${context.name}`, start, end)
    }
}
