import Context from "../context.ts";
import { Token } from "../tokens.ts";
import Value from "../values/value.ts";
import Node from "./node.ts";

export default class UnOpNode extends Node {
    token: Token;
    node: Node;

    constructor(operator: Token, node: Node) {
        super(operator.start, node.rightPos);
        this.token = operator;
        this.node = node;
        this.leftPos = operator.start;
        this.rightPos = node.rightPos;
    }

    visit(context: Context): Value {
        const value = this.node.visit(context) as Value;
        return value.performUnOperation(this.token);
    }

    isEqualTo(node: Node): boolean {
        if (node instanceof UnOpNode) {
            return this.node.isEqualTo(node.node) &&
                this.token.type == node.token.type;
        }
        return false;
    }

    toString() {
        return `(${this.token} ${this.node})`;
    }
}