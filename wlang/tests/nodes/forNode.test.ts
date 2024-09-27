import Context from "../../src/context.ts";
import ScopeNode from "../../src/nodes/scopeNode.ts";
import { TokenType } from "../../src/tokens.ts";
import NumberValue from "../../src/values/numberValue.ts";
import { assertEqualValues, makeForNode } from "../testHelpers.ts";

Deno.test("For Node", async (t) => {
    await t.step("Executes with condition", () => {
        const context = new Context("test");
        context.set("x", new NumberValue(-1))
        const expForNode = new ScopeNode("", [makeForNode([["x", [1]], [[3], TokenType.GT, ["x"]], [3], [{for: [["x", [4]]]}]])])
        expForNode.openScope();
        expForNode.visit(context)
        assertEqualValues(context.get("x"), new NumberValue(4))
    })

    await t.step("Doesn't execute without condition", () => {
        const context = new Context("test");
        context.set("x", new NumberValue(-1))
        const expForNode = new ScopeNode("", [makeForNode([[3], [[1], TokenType.GT, [2]], [1], [{for: [["x", [0]]]}]])])
        expForNode.openScope();
        expForNode.visit(context)
        assertEqualValues(context.get("x"), new NumberValue(-1))
    })
})