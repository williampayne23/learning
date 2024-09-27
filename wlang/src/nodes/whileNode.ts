import context from "../context.ts";
import NullValue from "../values/nullValue.ts";
import value from "../values/value.ts";
import Node from "./node.ts";
import ScopeNode from "./scopeNode.ts";

export default class WhileNode extends Node {

    condition: Node
    scope: ScopeNode
    isDo: boolean

    constructor(condition: Node, scope: ScopeNode, isDo: boolean) {
        super(condition.leftPos, scope.rightPos)
        this.condition = condition
        this.scope = scope
        this.isDo = isDo
    }

    isEqualTo(node: Node): boolean {
        if(node instanceof WhileNode){
            return this.condition.isEqualTo(node.condition) && this.scope.isEqualTo(node.scope)
        }
        return false
    }

    visit(context: context): value {
        if(this.isDo){
            do {
                this.scope.visit(context)
            } while (this.condition.visit(context).isTruthy(this.condition.leftPos, this.condition.rightPos))
        }else{
            while (this.condition.visit(context).isTruthy(this.condition.leftPos, this.condition.rightPos)) {
                this.scope.visit(context)
            }
        }
        return new NullValue()
    }

    toString(){
        return this.isDo? `do ${this.scope} while ${this.condition}` : `while ${this.condition}: ${this.scope}`
    }
}
