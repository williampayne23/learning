import Context from "../context.ts";
import { NoNodeError } from "../errors.ts";
import NullValue from "../values/nullValue.ts";
import Value from "../values/value.ts";
import Node from "./node.ts";

export default class ScopeNode extends Node {
    name: string;
    nodes: Node[];
    isClosed = true; 

    constructor(name: string, nodes: Node[]) {
        if (nodes.length < 1) {
            throw new NoNodeError();
        }
        super(nodes[0].leftPos, nodes[nodes.length - 1].rightPos);
        this.name = name;
        this.nodes = nodes;
        this.return = false;
    }

    openScope(){
        this.isClosed = false;
    }

    isEqualTo(node: Node): boolean {
        if (node instanceof ScopeNode) {
            return node.name == this.name &&
                this.nodes.length == node.nodes.length &&
                this.nodes.every((n, i) => n.isEqualTo(node.nodes[i]));
        }
        return false
    }

    visit(context: Context): Value {
        const localContext = this.isClosed? new Context(this.name, context) : context;
        let value: Value = new NullValue();
        for (let i = 0; i < this.nodes.length; i++) {
            value = this.nodes[i].evaluate(localContext);
        }
        return value;
    }

    toString() {
        return `{${this.name}: ${this.nodes.map(n => n.toString()).join('; ')}}`;
    }
}