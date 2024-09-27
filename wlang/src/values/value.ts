import { InvalidOperatorError } from "../errors.ts";
import position from "../position.ts";
import { Token, TokenType } from "../tokens.ts";
import BooleanValue from "./booleanValue.ts";

export default abstract class Value {
    // deno-lint-ignore no-explicit-any
    value: any;
    allowedBinOperations: Partial<Record<TokenType, (a:Value, b:Value, token:Token) => Value>> = {}
    allowedUnOperations: Partial<Record<TokenType, (a:Value, token:Token) => Value>> = {}
    declareIsAllowed(token: Token) {
        if (!(token.type in this.allowedBinOperations)) {
            throw new InvalidOperatorError(token, this);
        }
    }

    isTruthy(start: position, end: position): boolean{
        return this.performBinOperation(new BooleanValue(true), new Token(TokenType.EE, start, end)).value
    }

    performBinOperation(b: Value, token: Token): Value{
        const operation = this.allowedBinOperations[token.type]
        if(operation == null){
            throw new InvalidOperatorError(token, this);
        }
        b?.declareIsAllowed(token)
        return operation(this, b, token)
    }

    performUnOperation(token: Token): Value{
        const operation = this.allowedUnOperations[token.type]
        if(operation == null){
            throw new InvalidOperatorError(token, this);
        }
        return operation(this, token)
    }

    isNull() {
        return false;
    }

    toString(): string {
        return `${this.value}`;
    }

    equals(value: Value){
        return this.value == value.value
    }

    abstract copy(): Value;
}