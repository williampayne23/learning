import { assert } from "https://deno.land/std@0.140.0/testing/asserts.ts";
import Interpreter from "../src/interpreter.ts";
import NullValue from "../src/values/nullValue.ts";
import NumberValue from "../src/values/numberValue.ts";
import { assertEqualValues } from "./testHelpers.ts";

Deno.test("Interpreter", async (t) => {
    const i = new Interpreter()

    await t.step("Simple execution", () => {
        const four = i.executeCode("", "2 + 2")
        assertEqualValues(four, new NumberValue(4))

        const nullValue = i.executeCode("", "null")
        assert(nullValue.isNull())
    })

    await t.step("Repl", async (t) => {
        const repl = i.repl()
        repl.next()

        await t.step("Multiline scope", () => {
            repl.next("{")
            repl.next("2")
            const done = repl.next("}").done
            assert(!done)
        })

        await t.step("Errors", () => {
            const value = repl.next("x")
            assertEqualValues(value.value, new NullValue())
        })
    })

    await t.step("File", async () => {
        let val = await i.executeFile("tests/testFile.wlang")
        assertEqualValues(val, new NumberValue(1))

        val = await i.executeFile("tests/testError.wlang")
        assertEqualValues(val, new NullValue())
    })


})