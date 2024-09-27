import { InvalidOperatorError } from "../../src/errors.ts";
import { TokenType } from "../../src/tokens.ts";
import NumberValue from "../../src/values/numberValue.ts";
import { assertNodeError, assertRuntimeResult } from "../testHelpers.ts";

Deno.test("Unary Operator Node", async (t) => {
    await t.step("Negative numbers", () => {
        assertRuntimeResult([TokenType.MINUS, [1]], new NumberValue(-1))
    });

    await t.step("Bitwise not", () => {        
        assertRuntimeResult([TokenType.NOT, [1]], new NumberValue(-2))
    });

    await t.step("Invalid Operation Error", () => {
        assertNodeError([TokenType.DIVIDE, [2]], InvalidOperatorError)
    });
});
