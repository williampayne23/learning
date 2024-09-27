import { assertAlmostEquals, assertEquals } from "https://deno.land/std@0.140.0/testing/asserts.ts";
import { DivideByZeroError } from "../../src/errors.ts";
import Position from "../../src/position.ts";
import { Token, TokenType } from "../../src/tokens.ts";
import NumberValue from "../../src/values/numberValue.ts";
import { assertTypeOf } from "../testHelpers.ts";

Deno.test("Number Value", async t => {


    
    type operation = ([number, TokenType, number] | [TokenType, number]);

    function assertOperation(operation: operation, expResult: number | boolean) {
        const dummyPos = new Position(0, 0, 0, "", "");
        if (operation.length == 3) {
            const opResult = new NumberValue(operation[0])
                .performBinOperation(
                    new NumberValue(operation[2]),
                    new Token(operation[1], dummyPos, dummyPos),
                );
            if(typeof expResult == "number") assertAlmostEquals(opResult.value, new NumberValue(expResult).value);
            else assertEquals(opResult.value, expResult)
        }
        if(operation.length == 2 && typeof expResult == "number" ){
            const opResult = new NumberValue(operation[1])
                .performUnOperation(new Token(operation[0], dummyPos, dummyPos),);
            assertAlmostEquals(opResult.value, new NumberValue(expResult).value);
        }
    }
    await t.step("Arithmatic", async (t) => {
        await t.step("Addition", () => {
            assertOperation([1, TokenType.PLUS, 1], 2)
            assertOperation([1, TokenType.PLUS, -1], 0)
            assertOperation([1, TokenType.PLUS, 1.1], 2.1)
        });

        await t.step("Subtraction", () => {
            assertOperation([1, TokenType.MINUS, 1], 0)
            assertOperation([1, TokenType.MINUS, -1], 2)
            assertOperation([1, TokenType.MINUS, 1.1], -0.1)
        });

        await t.step("Multiplication", () => {
            assertOperation([1, TokenType.MULTIPLY, 2], 2)
            assertOperation([1, TokenType.MULTIPLY, -1], -1)
            assertOperation([2, TokenType.MULTIPLY, 1.1], 2.2)
        });

        await t.step("Division", () => {
            assertOperation([1, TokenType.DIVIDE, 3], 1/3)
            assertOperation([1, TokenType.DIVIDE, -1], -1)
            assertOperation([4.2, TokenType.DIVIDE, 2], 2.1)
            try {
                assertOperation([1, TokenType.DIVIDE, 0], 0)
            }catch (e){
                assertTypeOf(e, DivideByZeroError)
            }
        });

        await t.step("Floor Division", () => {
            assertOperation([1, TokenType.FLOORDIVIDE, 3], 0)
            assertOperation([1, TokenType.FLOORDIVIDE, -1], -1)
            assertOperation([4.2, TokenType.FLOORDIVIDE, 2], 2)
            try {
                assertOperation([1, TokenType.FLOORDIVIDE, 0], 0)
            }catch (e){
                assertTypeOf(e, DivideByZeroError)
            }
        });

        await t.step("Power", () => {
            assertOperation([2, TokenType.POW, 3], 8)
            assertOperation([2, TokenType.POW, -1], 0.5)
            assertOperation([3, TokenType.POW, 2], 9)
        });
        

        await t.step("Modulus", () => {
            assertOperation([5, TokenType.MODULUS, 2], 1)
            assertOperation([3, TokenType.MODULUS, 2], 1)
            assertOperation([18, TokenType.MODULUS, 5], 3)
        });
    });

    await t.step("Logical", async (t) => {
        await t.step("Bitwise and", () => {
            assertOperation([5, TokenType.AND, 2], 5 & 2)
        });
        await t.step("Bitwise Or", () => {
            assertOperation([2, TokenType.OR, 1], 3)
        });
        await t.step("Bitwise Xor", () => {
            assertOperation([5, TokenType.XOR, 2], 5 ^ 2)
        });
        await t.step("Bitshift Left", () => {
            assertOperation([5, TokenType.BITLEFT, 2], 5 << 2)
        });
        await t.step("Bitshift right", () => {
            assertOperation([5, TokenType.BITRIGHT, 2], 5 >> 2)
            assertOperation([3, TokenType.BITRIGHTZERO, 2], 3 >>> 2)
        });
    });

    await t.step("Comparison", async (t) => {
        await t.step("Less than", () => {
            assertOperation([1, TokenType.LT, 2], true)
            assertOperation([2, TokenType.LT, 1], false)
            assertOperation([2, TokenType.LT, 2], false)
        });

        await t.step("Greater than", () => {
            assertOperation([1, TokenType.GT, 2], false)
            assertOperation([2, TokenType.GT, 1], true)
            assertOperation([2, TokenType.GT, 2], false)
        });

        await t.step("Less than or equal to", () => {
            assertOperation([1, TokenType.LTE, 2], true)
            assertOperation([2, TokenType.LTE, 1], false)
            assertOperation([2, TokenType.LTE, 2], true)
        });

        await t.step("Greater than or equal to", () => {
            assertOperation([1, TokenType.GTE, 2], false)
            assertOperation([2, TokenType.GTE, 1], true)
            assertOperation([2, TokenType.GTE, 2], true)
        });

        await t.step("Equal to", () => {
            assertOperation([1, TokenType.EE, 2], false)
            assertOperation([2, TokenType.EE, 1], false)
            assertOperation([2, TokenType.EE, 2], true)
        });

        await t.step("Not equal to", () => {
            assertOperation([1, TokenType.NEE, 2], true)
            assertOperation([2, TokenType.NEE, 1], true)
            assertOperation([2, TokenType.NEE, 2], false)
        });
    });

    await t.step("Number string Repr", () => {
        const value = new NumberValue(-1)
        assertEquals(value?.toString(), "-1");
    });
});