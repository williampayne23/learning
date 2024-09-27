import { assertEquals } from "https://deno.land/std@0.140.0/testing/asserts.ts";
import Position from "../../src/position.ts";
import { Token, TokenType } from "../../src/tokens.ts";
import BooleanValue from "../../src/values/booleanValue.ts";
import NullValue from "../../src/values/nullValue.ts";
import NumberValue from "../../src/values/numberValue.ts";
import { assertEqualValues } from "../testHelpers.ts";

Deno.test("Null value", async t => {
    await t.step("Null", () => {
        const value = new NullValue()
        assertEquals(value.isNull(), true)
        assertEquals(new NumberValue(1).isNull(), false)
        assertEqualValues(new NullValue(), new NullValue())
        assertEquals(value.toString(), "NULL")
        assertEqualValues(new NullValue().performBinOperation(new NullValue(), new Token(TokenType.EE, Position.dummy, Position.dummy)), new BooleanValue(true))
    })
});