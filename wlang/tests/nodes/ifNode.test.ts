import Interpreter from "../../src/interpreter.ts";
import NullValue from "../../src/values/nullValue.ts";
import NumberValue from "../../src/values/numberValue.ts";
import { assertEqualValues, makeIfNode } from "../testHelpers.ts";

Deno.test("If Node", () => {
    const interpreter = new Interpreter()
    let expIfNode = makeIfNode([[["true"], [2]]]) 
    let value = expIfNode.evaluate(interpreter.context)
    assertEqualValues(value, new NumberValue(2))

    expIfNode = makeIfNode([[["false"], [2]]]) 
    value = expIfNode.evaluate(interpreter.context)
    assertEqualValues(value, new NullValue())
})