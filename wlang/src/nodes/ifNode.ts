import context from "../context.ts";
import Position from "../position.ts";
import NullValue from "../values/nullValue.ts";
import Value from "../values/value.ts";
import Node from "./node.ts";

export default class IfNode extends Node {
    conditionScopePairs: [Node, Node][];

    constructor(start: Position, conditionScopePairs: [Node, Node][]) {
        const last = conditionScopePairs[conditionScopePairs.length - 1][1];
        super(start, last.rightPos);
        this.conditionScopePairs = conditionScopePairs;
    }

    isEqualTo(node: Node): boolean {
        if (node instanceof IfNode) {
            return this.conditionScopePairs.length == node.conditionScopePairs.length &&
                this.conditionScopePairs.every((p, i) =>
                    p[0].isEqualTo(node.conditionScopePairs[i][0]) && p[1].isEqualTo(node.conditionScopePairs[i][1])
                );
        }
        return false;
    }
    visit(context: context): Value {
        for (let i = 0; i<this.conditionScopePairs.length; i++){
            const pair = this.conditionScopePairs[i];
            if (pair[0].visit(context).isTruthy(pair[0].leftPos, pair[0].rightPos)){
                pair[1].doReturnValue()
                const value = pair[1].visit(context)
                return value
            }
        }
        return new NullValue()
    }
}
