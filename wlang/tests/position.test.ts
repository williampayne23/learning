import { assertEquals } from "https://deno.land/std@0.140.0/testing/asserts.ts";
import Position from "../src/position.ts";

Deno.test("Position", async t => {
    await t.step("Line counting", () => {
        const position = new Position(0, 0, 1, "", "\n\n\n")
        position.advance()
        position.advance()
        position.advance()
        assertEquals(position.line, 3);
    });
})