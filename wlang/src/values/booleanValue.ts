import { TokenType } from "../tokens.ts";
import Value from "./value.ts";

export default class BooleanValue extends Value {
    value: boolean

    constructor(value: boolean){
        super()
        this.value = value
    }

    allowedBinOperations = {
        [TokenType.AND](a: Value, b: Value): Value{
            return new BooleanValue(a.value && b.value)
        },
        [TokenType.OR](a: Value, b: Value): Value{
            return new BooleanValue(a.value || b.value)
        },
        [TokenType.XOR](a: Value, b: Value): Value{
            return new BooleanValue((a.value ^ b.value) == 1)
        },
        [TokenType.EE](a: Value, b:Value): Value {
            return new BooleanValue(a.value == b.value)
        },
    }

    allowedUnOperations = {
        [TokenType.NOT](a: Value): Value{
            return new BooleanValue(!a.value)
        }
    }

    toString(){
        return `${this.value}`
    }

    copy(): Value {
        return new BooleanValue(this.value)
    }

}