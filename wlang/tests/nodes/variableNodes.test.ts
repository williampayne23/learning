import { UndefinedVariableError } from "../../src/errors.ts";
import NumberValue from "../../src/values/numberValue.ts";
import { assertEqualValues, assertNodeError, assertRuntimeResult } from "../testHelpers.ts";


Deno.test("Variable Assignment", () => {
    const context = assertRuntimeResult(["x", [3]], new NumberValue(3))
    assertEqualValues(context.get("x"), new NumberValue(3))
})


Deno.test("Variable Retrieval", () => {
    const context = assertRuntimeResult(["x", [3]], new NumberValue(3))
    assertRuntimeResult(["x"], new NumberValue(3), context)
    assertNodeError(["y"], UndefinedVariableError)
})