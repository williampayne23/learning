import { assertEquals } from "https://deno.land/std@0.140.0/testing/asserts.ts";
import Context from "../../src/context.ts";
import ScopeNode from "../../src/nodes/scopeNode.ts";
import { TokenType } from "../../src/tokens.ts";
import NumberValue from "../../src/values/numberValue.ts";
import { assertEqualValues, makeWhileNode } from "../testHelpers.ts";

Deno.test("While Node", async (t) => {
    await t.step("Executes with condition", () => {
        const context = new Context("test");
        context.set("x", new NumberValue(-1))
        const expWhileNode = new ScopeNode("", [makeWhileNode([[[2], TokenType.GT, ["x"]] ,[{test: [["x", [4]]]}]], false)])
        expWhileNode.openScope();
        expWhileNode.visit(context)
        assertEqualValues(context.get("x"), new NumberValue(4))
    })

    await t.step("Doesn't execute without condition", () => {
        const context = new Context("test");
        context.set("x", new NumberValue(-1))
        const expWhileNode = new ScopeNode("", [makeWhileNode([[[-3], TokenType.GT, ["x"]] ,[{test: [["x", [4]]]}]], false)])
        expWhileNode.openScope();
        expWhileNode.visit(context)
        assertEqualValues(context.get("x"), new NumberValue(-1))
    })

    await t.step("Print", () => {
        const whileNode = makeWhileNode([[1], [2]], false)
        assertEquals(whileNode.toString(), "while <NUMBER: 1>: <NUMBER: 2>")
    })
})

Deno.test("Do while Node", async (t) => {
    await t.step("Executes with condition", () => {
        const context = new Context("test");
        context.set("x", new NumberValue(-1))
        const expWhileNode = new ScopeNode("", [makeWhileNode([[[2], TokenType.GT, ["x"]] ,[{test: [["x", [4]]]}]], true)])
        expWhileNode.openScope();
        expWhileNode.visit(context)
        assertEqualValues(context.get("x"), new NumberValue(4))
    })

    await t.step("Executes once without condition", () => {
        const context = new Context("test");
        context.set("x", new NumberValue(-1))
        const expWhileNode = new ScopeNode("", [makeWhileNode([[[-3], TokenType.GT, ["x"]] ,[{test: [["x", [4]]]}]], true)])
        expWhileNode.openScope();
        expWhileNode.visit(context)
        assertEqualValues(context.get("x"), new NumberValue(4))
    })

    await t.step("Print", () => {
        const doWhileNode = makeWhileNode([[1], [2]], true)
        assertEquals(doWhileNode.toString(), "do <NUMBER: 2> while <NUMBER: 1>")
    })
})