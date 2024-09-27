import Context from "../context.ts";
import context from "../context.ts";
import NullValue from "../values/nullValue.ts";
import Value from "../values/value.ts";
import Node from "./node.ts";
import ScopeNode from "./scopeNode.ts";

export default class ForNode extends Node {
    assignment: Node;
    condition: Node;
    increment: Node;
    scope: ScopeNode;

    constructor(assignment: Node, condition: Node, increment: Node, scope: ScopeNode) {
        super(assignment.leftPos, scope.rightPos);
        this.assignment = assignment;
        this.condition = condition;
        this.increment = increment;
        this.scope = scope;
        this.scope.openScope();
    }

    isEqualTo(node: Node): boolean {
        if (node instanceof ForNode) {
            return this.assignment.isEqualTo(node.assignment) &&
                this.condition.isEqualTo(node.condition) &&
                this.increment.isEqualTo(node.increment) &&
                this.scope.isEqualTo(node.scope);
        }
        return false;
    }
    visit(context: context): Value {
        const forContext = new Context("for", context)
        this.assignment.visit(forContext)
        while (this.condition.visit(forContext).isTruthy(this.condition.leftPos, this.condition.rightPos)){
            this.scope.visit(forContext)
            this.increment.visit(forContext)
        }
        return new NullValue()
    }
}
