import { DivideByZeroError } from "../errors.ts";
import { TokenType,Token } from "../tokens.ts";
import BooleanValue from "./booleanValue.ts";
import Value from "./value.ts";

export default class NumberValue extends Value {

    value: number;

    constructor(value: number) {
        super();
        this.value = value;
    }

    copy(): NumberValue {
        return new NumberValue(this.value);
    }

    allowedBinOperations = {
        [TokenType.MULTIPLY](a:Value, b: Value): Value {
            return new NumberValue(a.value * b.value);
        },
        [TokenType.DIVIDE](a:Value, b: Value, token: Token): Value {
            if (b.value == 0) {
                throw new DivideByZeroError(token.start, token.end)
            }
            return new NumberValue(a.value / b.value);
        },
        [TokenType.MINUS](a:Value, b: Value): Value {
            return new NumberValue(a.value - b.value);
        },
        [TokenType.PLUS](a:Value, b: Value): Value {
            return new NumberValue(a.value + b.value);
        },
        [TokenType.MODULUS](a:Value, b: Value): Value {
            return new NumberValue(a.value % b.value);
        },
        [TokenType.FLOORDIVIDE](a:Value, b: Value, token:Token): Value {
            token.type = TokenType.DIVIDE
            const out = a.performBinOperation(b, token)
            out.value = Math.floor(out.value);
            return out;
        },
        [TokenType.POW](a:Value, b: Value): Value {
            return new NumberValue(a.value ** b.value);
        },
        [TokenType.GT](a: Value, b:Value): Value {
            return new BooleanValue(a.value > b.value)
        },
        [TokenType.LT](a: Value, b:Value): Value {
            return new BooleanValue(a.value < b.value)
        },
        [TokenType.GTE](a: Value, b:Value): Value {
            return new BooleanValue(a.value >= b.value)
        },
        [TokenType.LTE](a: Value, b:Value): Value {
            return new BooleanValue(a.value <= b.value)
        },
        [TokenType.EE](a: Value, b:Value): Value {
            return new BooleanValue(a.value == b.value)
        },
        [TokenType.NEE](a: Value, b:Value): Value {
            return new BooleanValue(a.value != b.value)
        },
        [TokenType.AND](a: Value, b: Value): Value{
            return new NumberValue(a.value & b.value)
        },
        [TokenType.OR](a: Value, b: Value): Value{
            return new NumberValue(a.value | b.value)
        },
        [TokenType.XOR](a: Value, b: Value): Value{
            return new NumberValue(a.value ^ b.value)
        },
        [TokenType.BITLEFT](a: Value, b: Value): Value{
            return new NumberValue(a.value << b.value)
        },
        [TokenType.BITRIGHT](a: Value, b: Value): Value{
            return new NumberValue(a.value >> b.value)
        },
        [TokenType.BITRIGHTZERO](a: Value, b: Value): Value{
            return new NumberValue(a.value >>> b.value)
        }
    }

    allowedUnOperations = {
        [TokenType.MINUS](a: Value): Value {
            return new NumberValue(a.value * -1)
        },
        [TokenType.NOT](a: Value): Value{
            return new NumberValue(~a.value)
        }
    }

}