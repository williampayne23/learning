import Context from "../context.ts";
import { UndefinedVariableError } from "../errors.ts";
import position from "../position.ts";
import { Token, TokenType } from "../tokens.ts";
import Value from "../values/value.ts";
import Node from "./node.ts";

export default class VarRetrievalNode extends Node {
    identifier: string;

    constructor(identifierToken: Token) {
        super(identifierToken.start, identifierToken.end);
        this.identifier = identifierToken.value as string;
    }

    isEqualTo(node: Node): boolean {
        if (node instanceof VarRetrievalNode) {
            return node.identifier == this.identifier;
        }
        return false;
    }

    visit(context: Context): Value {
        try {
            return context.get(this.identifier);
        } catch {
            throw new UndefinedVariableError(this.identifier, context, this.leftPos, this.rightPos);
        }
    }

    toString() {
        return `(${this.identifier})`;
    }

    static NULL = (start: position, end: position) => new VarRetrievalNode(new Token(TokenType.IDENTIFIER, start, end, "null"))
    static TRUE = (start: position, end: position) => new VarRetrievalNode(new Token(TokenType.IDENTIFIER, start, end, "true"))
    static FALSE = (start: position, end: position) => new VarRetrievalNode(new Token(TokenType.IDENTIFIER, start, end, "false"))
}