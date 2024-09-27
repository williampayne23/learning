import Context from "./context.ts";
import { UnexpectedEndOfFile } from "./errors.ts";
import Lexer from "./lexer.ts";
import Parser from "./parser.ts";
import { TokenType } from "./tokens.ts";
import BooleanValue from "./values/booleanValue.ts";
import NullValue from "./values/nullValue.ts";
import NumberValue from "./values/numberValue.ts";
import Value from "./values/value.ts";
export default class Interpreter {

    context: Context;

    constructor(context?: Context){
        this.context = context ?? Interpreter.newGlobalContext()
    }

    static newGlobalContext(): Context{
        const globalContext = new Context("global")
        globalContext.set("true", new BooleanValue(true))
        globalContext.set("false", new BooleanValue(false))
        globalContext.set("null", new NullValue())
        globalContext.set("pi", new NumberValue(Math.PI))
        globalContext.set("tau", new NumberValue(Math.PI/2))
        return globalContext
    }

    async executeFile(fileName: string): Promise<Value>{
        try {
        const text = await Deno.readTextFile(fileName);
        console.log(text)
        return this.executeCode(fileName, text);
        } catch (e) {
            console.error(`${e}`)
        }
        return new NullValue();
    }

    executeCode(source: string, code: string) : Value {
        //Lexer
        const tokens = Lexer.tokensFromLine(source, code);
        //Parser
        const parseRes = Parser.parseTokens(tokens);
        //Interpreter
        const result = parseRes.evaluate(this.context);
        if(!result.isNull()) console.log(`${result}`);
        return result
    }

    * repl(): Generator< Value, Value, string>{
        let line: string = yield new NullValue();
        while (true) {
            try{
                const val = this.executeCode("repl", line)
                line = yield val
            } catch (e) {
                if(e instanceof UnexpectedEndOfFile && (e as UnexpectedEndOfFile).expectedTokens.includes(TokenType.NEWLINE)){
                    const extendLine = yield new NullValue();
                    line = line + '\n' + extendLine;
                    continue
                }
                console.log(`${e}`);
                return new NullValue()
            }
        }
    }
}