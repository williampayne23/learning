import { InvalidOperatorError } from "../../src/errors.ts";
import { TokenType } from "../../src/tokens.ts";
import { assertNodeError } from "../testHelpers.ts";


Deno.test("Binary Operator Node", async t => {
    await t.step("Invalid Operation Error", () => {
        assertNodeError([[2], TokenType.IDENTIFIER, [2]], InvalidOperatorError)
        assertNodeError([[2], TokenType.DIVIDE, ["false"]], InvalidOperatorError)
    });
})
