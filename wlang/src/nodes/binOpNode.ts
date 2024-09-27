import Context from "../context.ts";
import { Token } from "../tokens.ts";
import Value from "../values/value.ts";
import Node from "./node.ts";

export default class BinOpNode extends Node {
    token: Token;
    leftNode: Node;
    rightNode: Node;

    constructor(leftNode: Node, operatorToken: Token, rightNode: Node) {
        super(leftNode.leftPos, rightNode.rightPos);
        this.leftNode = leftNode;
        this.rightNode = rightNode;
        this.token = operatorToken;
    }

    visit(context: Context): Value {
        const leftVal = this.leftNode.visit(context);
        const rightVal = this.rightNode.visit(context);

        return leftVal.performBinOperation(rightVal, this.token);
    }

    isEqualTo(node: Node): boolean {
        if (node instanceof BinOpNode) {
            return this.leftNode.isEqualTo(node.leftNode) &&
                this.rightNode.isEqualTo(node.rightNode) &&
                this.token.type == node.token.type;
        }
        return false;
    }

    toString() {
        return `(${this.leftNode} ${this.token} ${this.rightNode})`;
    }
}