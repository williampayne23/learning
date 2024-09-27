import Context from "../context.ts";
import Position from "../position.ts";
import NullValue from "../values/nullValue.ts";
import Value from "../values/value.ts";

export default abstract class Node {
    leftPos: Position;
    rightPos: Position;
    return = true;

    constructor(leftPos: Position, rightPos: Position) {
        this.rightPos = rightPos;
        this.leftPos = leftPos;
    }

    dontReturnValue() {
        this.return = false;
    }

    doReturnValue() {
        this.return = true;
    }

    evaluate(context: Context): Value {
        const value = this.visit(context);
        const out = this.return ? value : new NullValue();
        context.set("", out);
        return out;
    }

    abstract isEqualTo(node: Node): boolean;
    abstract visit(context: Context): Value;
}