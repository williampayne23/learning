import { assertThrows } from "https://deno.land/std@0.140.0/testing/asserts.ts";
import NumberValue from "../src/values/numberValue.ts";
import { assertEqualValues, assertMatchingTokens, assertTypeOf, makeTokensUtility } from "./testHelpers.ts";


Deno.test("Test Helpers", async t => {
    await t.step("Not matching types", () => {
        assertThrows(() => assertTypeOf(3, ""))
    })

    await t.step("Longer token list isn't equal", () => {
        assertThrows(() => 
        assertMatchingTokens(makeTokensUtility([9]), makeTokensUtility()))
    })

    await t.step("Shorter token list isn't equal", () => {
        assertThrows(() => 
        assertMatchingTokens(makeTokensUtility(), makeTokensUtility([9])))
    })

    await t.step("Non matching token type isn't equal", () => {
        assertThrows(() => 
        assertMatchingTokens(makeTokensUtility([9]), makeTokensUtility([8])))
    })

    await t.step("Non matching value isn't equal", () => {
        assertThrows(() => assertEqualValues(new NumberValue(2), new NumberValue(3)))
    })
})