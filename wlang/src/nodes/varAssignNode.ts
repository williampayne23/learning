import Context from "../context.ts";
import { Token } from "../tokens.ts";
import Value from "../values/value.ts";
import Node from "./node.ts";

export default class VarAsignmentNode extends Node {
    identifier: string;
    assignNode: Node;

    constructor(identifierToken: Token, node: Node) {
        super(identifierToken.start, node.rightPos);
        this.identifier = identifierToken.value as string;
        this.assignNode = node;
    }

    visit(context: Context): Value {
        const value = this.assignNode.visit(context);
        context.set(this.identifier, value);
        return value;
    }

    isEqualTo(node: Node): boolean {
        if (node instanceof VarAsignmentNode) {
            return (this.identifier == node.identifier && this.assignNode.isEqualTo(node.assignNode));
        }
        return false;
    }

    toString() {
        return `(${this.identifier} = ${this.assignNode})`;
    }
}