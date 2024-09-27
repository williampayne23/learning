import Interpreter from "./interpreter.ts";
import { parse } from "https://deno.land/std@0.140.0/flags/mod.ts";

main();

async function main() {
    const interpreter = new Interpreter();

    const args = parse(Deno.args);

    if (args["_"].length != 0) {
        for (const i in args["_"]) {
            const file = args["_"][i] as string;
            await interpreter.executeFile(file)
        }
    }

    if (!(args["r"] || args["repl"] || args["_"].length == 0)) {
        return;
    }

    const repl = interpreter.repl();
    repl.next();
    let line = prompt(">") ?? "";
    while (!repl.next(line).done) {
        line = prompt(">") ?? "";
    }
}
