import { assertEquals } from "https://deno.land/std@0.140.0/testing/asserts.ts";
import { WLANGError } from "../src/errors.ts";
import Position from "../src/position.ts";

Deno.test("Error", async t => {
    await t.step("String repr", () => {
        const dummyPos = new Position(7, 1, 2, "test", "Hello\nWorld")
        dummyPos.advance()
        assertEquals(dummyPos.nextChar, "r")
        const dummyPosEnd = dummyPos.copy()
        dummyPosEnd.advance()
        let error = new WLANGError("Fake Error", dummyPos, dummyPosEnd)
        assertEquals(error.toString(), `Error: Fake Error at col ${2}:\nWorld\n  ^`)

        dummyPos.advance()
        error = new WLANGError("Fake Error", dummyPos, dummyPosEnd)
        assertEquals(error.toString(), `Error: Fake Error at col ${3}:\nWorld\n   ^`)

        error = new WLANGError("Fake Error", new Position(0, 0, 1, "", ""), new Position(0, 0, 1, "", ""))
        assertEquals(error.toString(), `Error: Fake Error at col 0:\n`)
    })
})