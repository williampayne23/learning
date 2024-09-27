import { assertEquals } from "https://deno.land/std@0.140.0/testing/asserts.ts";
import { IllegalCharacterError } from "../src/errors.ts";
import Lexer from "../src/lexer.ts";
import { TokenType } from "../src/tokens.ts";
import { assertMatchingTokens, assertTypeOf, makeTokensUtility } from "./testHelpers.ts";

Deno.test("Lexer", async (t) => {

    await t.step("Simple arithmatic", () => {
        const expectedResult = makeTokensUtility(
            [TokenType.OPENPAR],
            [TokenType.NUMBER, 1],
            [TokenType.PLUS],
            [TokenType.NUMBER, 2],
            [TokenType.MINUS],
            [TokenType.NUMBER, 3],
            [TokenType.MULTIPLY],
            [TokenType.NUMBER, 4],
            [TokenType.DIVIDE],
            [TokenType.NUMBER, 5],
            [TokenType.POW],
            [TokenType.FLOORDIVIDE],
            [TokenType.MODULUS],
            [TokenType.CLOSEPAR],
            [TokenType.TEMINAL],
            [TokenType.NEWLINE],
            [TokenType.NOT],
            [TokenType.KEYWORD, "last"],
            [TokenType.AND],
            [TokenType.OR],
            [TokenType.XOR],
            [TokenType.GT],
            [TokenType.LT],
            [TokenType.GTE],
            [TokenType.LTE],
            [TokenType.EE],
            [TokenType.NEE],
            [TokenType.BITLEFT],
            [TokenType.BITRIGHT],
            [TokenType.BITRIGHTZERO],
            [TokenType.EOF],
        );
        const tokens = Lexer.tokensFromLine("<stdin>", "(1+2-3*4 / 5 ** // % ) ; \n ! $ & | ^ > < >= <= == != << >> >>>");
        assertMatchingTokens(tokens, expectedResult);
    });

    await t.step("Comments", () => {
        let tokens = Lexer.tokensFromLine("<stdin>", "# Hello");
        assertMatchingTokens(tokens, makeTokensUtility([TokenType.EOF]))


        tokens = Lexer.tokensFromLine("<stdin>", "###\nHello\n###");
        assertMatchingTokens(tokens, makeTokensUtility([TokenType.EOF]))
    });

    await t.step("Illegal character error", () => {
        try{
            Lexer.tokensFromLine("<stdin>", ".");
        } catch (e){
            assertTypeOf(e, IllegalCharacterError);
        }
    });

    await t.step("Illegal character error (too many decimals)", () => {
        try{
            Lexer.tokensFromLine("<stdin>", "1.222.");
        } catch (e){
            assertTypeOf(e, IllegalCharacterError);
        }
    });

    await t.step("Identifiers", () => {
        const expectedResult = makeTokensUtility(
            [TokenType.IDENTIFIER, "seven"],
            [TokenType.EOF],
        );
        const tokens = Lexer.tokensFromLine("<stdin>", "seven");
        assertMatchingTokens(
            tokens,
            expectedResult,
        );
    });

    await t.step("Keywords", () => {
        const expectedResult = makeTokensUtility(
            [TokenType.KEYWORD, "let"],
            [TokenType.EOF],
        );
        const tokens = Lexer.tokensFromLine("<stdin>", "let");
        assertMatchingTokens(
            tokens,
            expectedResult,
        );
    });


    await t.step("String Repr", () => {
        const lex = Lexer.tokensFromLine("<stdin>", "1").toString();
        assertEquals(lex, "<NUMBER: 1>,<EOF>");
    });
});