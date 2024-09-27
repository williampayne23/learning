import { assertEquals, assertThrows } from "https://deno.land/std@0.140.0/testing/asserts.ts";
import Position from "../../src/position.ts";
import { Token, TokenType } from "../../src/tokens.ts";
import BooleanValue from "../../src/values/booleanValue.ts";
import { assertEqualValues } from "../testHelpers.ts";

Deno.test("Boolean Value", async (t) => {
    await t.step("Boolean string Repr", () => {
        const value = new BooleanValue(false);
        assertEquals(value?.toString(), "false");
    });

    await t.step("Copy", () => {
        const value = new BooleanValue(false);
        assertEqualValues(value, value.copy())
        assertThrows(() => {assertEquals(value, value.copy())})
    })

    type operation = ([boolean, TokenType, boolean] | [TokenType, boolean]);

    function assertLogicalOperation(operation: operation, expResult: boolean) {
        const dummyPos = new Position(0, 0, 0, "", "");
        if (operation.length == 3) {
            const opResult = new BooleanValue(operation[0])
                .performBinOperation(
                    new BooleanValue(operation[2]),
                    new Token(operation[1], dummyPos, dummyPos),
                );
            assertEqualValues(opResult, new BooleanValue(expResult));
        }
        if(operation.length == 2){
            const opResult = new BooleanValue(operation[1])
                .performUnOperation(new Token(operation[0], dummyPos, dummyPos),);
            assertEqualValues(opResult, new BooleanValue(expResult));
        }
    }

    await t.step("and", () => {
        assertLogicalOperation([true, TokenType.AND, true], true);
        assertLogicalOperation([false, TokenType.AND, true], false);
        assertLogicalOperation([false, TokenType.AND, false], false);
    });

    await t.step("or", () => {
        assertLogicalOperation([true, TokenType.OR, true], true);
        assertLogicalOperation([false, TokenType.OR, true], true);
        assertLogicalOperation([false, TokenType.OR, false], false);
    });

    await t.step("or", () => {
        assertLogicalOperation([true, TokenType.XOR, true], false);
        assertLogicalOperation([false, TokenType.XOR, true], true);
        assertLogicalOperation([false, TokenType.XOR, false], false);
    });

    await t.step("not", () => {
        assertLogicalOperation([TokenType.NOT, true], false);
        assertLogicalOperation([TokenType.NOT, false], true);
    });
});
