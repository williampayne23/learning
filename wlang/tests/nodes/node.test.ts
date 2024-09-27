import { assert, assertEquals, assertThrows } from "https://deno.land/std@0.140.0/testing/asserts.ts";
import Context from "../../src/context.ts";
import { TokenType } from "../../src/tokens.ts";
import { assertMatchingAST, makeASTUtility, makeForNode, makeIfNode, makeWhileNode } from "../testHelpers.ts";

Deno.test("Nodes", async (t) => {

    const node = makeASTUtility([2])
    node.dontReturnValue()
    assert(node.evaluate(new Context("")).isNull())

    await t.step("Non matching nodes aren't equal", () => {
        assertThrows(() =>
            assertMatchingAST(makeASTUtility([[1], TokenType.PLUS, [2]]), makeASTUtility([TokenType.MINUS, [2]]))
        );
        assertThrows(() =>
            assertMatchingAST(makeASTUtility([TokenType.MINUS, [2]]), makeASTUtility([[1], TokenType.PLUS, [2]]))
        );

        assertThrows(() => assertMatchingAST(makeASTUtility([2]), makeASTUtility([[1], TokenType.PLUS, [2]])));
        assertThrows(() => assertMatchingAST(makeASTUtility(["d", [2]]), makeASTUtility([[1], TokenType.PLUS, [2]])));
        assertThrows(() => assertMatchingAST(makeASTUtility(["d"]), makeASTUtility([[1], TokenType.PLUS, [2]])));
        assertThrows(() => assertMatchingAST(makeASTUtility([{scope: [[2]]}]), makeASTUtility([2])));
        assertThrows(() => assertMatchingAST(makeIfNode([[["true"], [2]]]), makeASTUtility([2])));
        assertThrows(() => assertMatchingAST(makeWhileNode([[1], [2]], false), makeASTUtility([2])));
        assertThrows(() => assertMatchingAST(makeForNode([[1], [1], [1], [{if: [[1]]}]]), makeASTUtility([2])));
    });

    await t.step("String repr", () => {
        const expectedTree = makeASTUtility([[1], TokenType.PLUS, [TokenType.MINUS, [2]]]);
        assertEquals(expectedTree.toString(), "{global: (<NUMBER: 1> <PLUS> (<MINUS> <NUMBER: 2>))}");
    });
});
