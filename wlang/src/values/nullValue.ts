import { TokenType } from "../tokens.ts";
import BooleanValue from "./booleanValue.ts";
import Value from "./value.ts";

export default class NullValue extends Value {

    allowedBinOperations = {
        [TokenType.EE](a: Value, b: Value): Value {
            return new BooleanValue(b.isNull() && a.isNull())
        }
    }

    toString(): string {
        return "NULL";
    }
    isNull() {
        return true;
    }
    copy() {
        return new NullValue();
    }

    equals(value: Value){
        return value.isNull();
    }
}