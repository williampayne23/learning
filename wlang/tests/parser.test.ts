import { UnexpectedEndOfFile, UnexpectedTokenError } from "../src/errors.ts";
import ScopeNode from "../src/nodes/scopeNode.ts";
import { TokenType } from "../src/tokens.ts";
import { assertMatchingAST, assertParseError, assertParseResult, makeForNode, makeIfNode, makeWhileNode, parseResult } from "./testHelpers.ts";

Deno.test("Parser", async (t) => {
    await t.step("Simple parse", () => {
        assertParseResult("1 + 2", [[1], TokenType.PLUS, [2]]);
    });

    await t.step("Newlines", () => {
        assertParseResult("last", [""]);
        assertParseResult("$", [""]);
        assertParseResult("1\n", [1]);
        assertParseResult("1;", [1]);
        assertParseResult("#Hello", ["null"]);
    });

    await t.step("If", () => {
        let expIfNode = new ScopeNode("global", [makeIfNode([
            [["true"], [2]],
            [["true"], [3]],
            [["true"], [4]]
        ])]);
        assertMatchingAST(parseResult("if true then 2 elif true then 3 else 4"), expIfNode)

        expIfNode = new ScopeNode("global", [makeIfNode([
            [["true"], [{if: [[2]]}]],
            [["true"], [{elif: [[3]]}]],
            [["true"], [{else: [[4]]}]]
        ])]);
        assertMatchingAST(parseResult("if true {2} elif true {3} else {4}"), expIfNode)

        //Checking some syntax errors are caught

        assertParseError("if 2 2", UnexpectedTokenError)
        assertParseError("if 2 then 3 elif 3 3", UnexpectedTokenError)

        assertParseError("if 2 {} elif 3 3", UnexpectedTokenError)
        assertParseError("if 2 {} elif 3 {} else 10", UnexpectedTokenError)
    });

    await t.step("For", () => {
        const expForNode = new ScopeNode("global", [makeForNode([[2], [[1], TokenType.GT, [2]], [3], [{for: [[3]]}]])])
        assertMatchingAST(parseResult("for (2; 1 > 2; 3) {3}"), expForNode)
    })

    await t.step("While and Do While", () => {
        const expWhileNode = new ScopeNode("global", [makeWhileNode([[[1], TokenType.GT, [2]], [{while: [[3]]}]], false)])
        assertMatchingAST(parseResult("while (1 > 2) {3}"), expWhileNode)

        const expDoWhileNode = new ScopeNode("global", [makeWhileNode([[[1], TokenType.GT, [2]], [{"do-while": [[3]]}]], true)])
        assertMatchingAST(parseResult("do {3} while (1 > 2)"), expDoWhileNode)

        assertParseError("do {4} x", UnexpectedTokenError)
    })

    await t.step("Local Scope", () => {
        assertParseResult("{\n\n2;\n\n}", [{ anonymous: [[2]] }]);
        assertParseResult("{\n\n2;\n\n\n2;}", [{ anonymous: [[2], [2]] }]);
        assertParseResult("{}", [{ anonymous: [["null"]] }])
    });

    await t.step("Order of Operations", async (t) => {
        await t.step("Brackets", () => {
            assertParseResult("1 + 2 - 4 * 3", [
                [[1], TokenType.PLUS, [2]],
                TokenType.MINUS,
                [[4], TokenType.MULTIPLY, [3]],
            ]);

            assertParseResult("1 + (2 - 4) * 3", [
                [1],
                TokenType.PLUS,
                [
                    [[2], TokenType.MINUS, [4]],
                    TokenType.MULTIPLY,
                    [3],
                ],
            ]);
        });

        await t.step("Indicies", () => {
            assertParseResult("1 + 2 ** 3", [
                [1],
                TokenType.PLUS,
                [
                    [2],
                    TokenType.POW,
                    [3],
                ],
            ]);

            assertParseResult("2 ** 3 * 3", [
                [[2], TokenType.POW, [3]],
                TokenType.MULTIPLY,
                [3],
            ]);
        });

        await t.step("Mult, Divide, Modulo", () => {
            assertParseResult("1 + 2 % 3", [
                [1],
                TokenType.PLUS,
                [
                    [2],
                    TokenType.MODULUS,
                    [3],
                ],
            ]);

            assertParseResult("2 % 3 + 3", [
                [[2], TokenType.MODULUS, [3]],
                TokenType.PLUS,
                [3],
            ]);

            assertParseResult("1 + 2 // 3", [
                [1],
                TokenType.PLUS,
                [
                    [2],
                    TokenType.FLOORDIVIDE,
                    [3],
                ],
            ]);

            assertParseResult("2 // 3 + 3", [
                [[2], TokenType.FLOORDIVIDE, [3]],
                TokenType.PLUS,
                [3],
            ]);

            assertParseResult("1 + 2 * 3", [
                [1],
                TokenType.PLUS,
                [
                    [2],
                    TokenType.MULTIPLY,
                    [3],
                ],
            ]);

            assertParseResult("2 * 3 + 3", [
                [[2], TokenType.MULTIPLY, [3]],
                TokenType.PLUS,
                [3],
            ]);

            assertParseResult("1 + 2 / 3", [
                [1],
                TokenType.PLUS,
                [
                    [2],
                    TokenType.DIVIDE,
                    [3],
                ],
            ]);

            assertParseResult("2 / 3 + 3", [
                [[2], TokenType.DIVIDE, [3]],
                TokenType.PLUS,
                [3],
            ]);
        });
    });

    await t.step("Unary Operator", async (t) => {
        await t.step("Single minus", () => {
            assertParseResult("-2", [TokenType.MINUS, [2]]);
        });

        await t.step("Multiple minus", () => {
            assertParseResult("---2", [TokenType.MINUS, [TokenType.MINUS, [TokenType.MINUS, [2]]]]);
        });

        await t.step("Single not", () => {
            assertParseResult("!2", [TokenType.NOT, [2]]);
        });

        await t.step("Multiple not", () => {
            assertParseResult("!!!2", [TokenType.NOT, [TokenType.NOT, [TokenType.NOT, [2]]]]);
        });
    });

    await t.step("Variables", async (t) => {
        await t.step("Successful assignment", () => {
            assertParseResult("let x = 4", ["x", [4]]);
        });

        await t.step("Successful retrieval", () => {
            assertParseResult("x", ["x"]);
        });
    });

    await t.step("Unexpected Tokens", async (t) => {
        await t.step("No close bracket", () => {
            assertParseError("(1", UnexpectedEndOfFile, "");
        });

        await t.step("No expr in bracket", () => {
            assertParseError("()", UnexpectedTokenError, "");
        });

        await t.step("Minus with no factor", () => {
            assertParseError("-+", UnexpectedTokenError, "+");
        });

        await t.step("Let but no identifier", () => {
            assertParseError("let 4", UnexpectedTokenError);
        });

        await t.step("Let but no equals", () => {
            assertParseError("let x 4", UnexpectedTokenError);
        });

        await t.step("At end of file", () => {
            assertParseError("1 2", UnexpectedTokenError, "2");
        });
    });
});
