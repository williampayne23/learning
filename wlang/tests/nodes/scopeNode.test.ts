import Context from "../../src/context.ts";
import { NoNodeError } from "../../src/errors.ts";
import ScopeNode from "../../src/nodes/scopeNode.ts";
import {  assertTypeOf } from "../testHelpers.ts";

Deno.test("Scope", async (t) => {
    await t.step("No node error", () => {
        try{
            new ScopeNode("scope", []).evaluate(new Context(""))
        }catch (e) {
            assertTypeOf(e, NoNodeError)
        }
    });
});