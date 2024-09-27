import { Token } from "../tokens.ts";
import NumberValue from "../values/numberValue.ts";
import Value from "../values/value.ts";
import Node from "./node.ts";

export default class NumberNode extends Node {
    value: NumberValue;

    constructor(token: Token) {
        super(token.start, token.end);
        this.value = new NumberValue(parseFloat(token.value as string));
    }

    visit(): Value {
        return this.value;
    }

    isEqualTo(node: Node): boolean {
        if (node instanceof NumberNode) {
            return this.value.value == node.value.value;
        }
        return false;
    }

    toString() {
        return `<NUMBER: ${this.value}>`;
    }
}