import { assert, assertEquals, AssertionError } from "https://deno.land/std@0.140.0/testing/asserts.ts";
import Context from "../src/context.ts";
import Interpreter from "../src/interpreter.ts";
import Lexer from "../src/lexer.ts";
import BinOpNode from "../src/nodes/binOpNode.ts";
import ForNode from "../src/nodes/forNode.ts";
import IfNode from "../src/nodes/ifNode.ts";
import Node from "../src/nodes/node.ts";
import NumberNode from "../src/nodes/numberNode.ts";
import ScopeNode from "../src/nodes/scopeNode.ts";
import UnOpNode from "../src/nodes/unOpNode.ts";
import VarAsignmentNode from "../src/nodes/varAssignNode.ts";
import VarRetrievalNode from "../src/nodes/varRetrievalNode.ts";
import WhileNode from "../src/nodes/whileNode.ts";
import Parser from "../src/parser.ts";
import Position from "../src/position.ts";
import { Token, TokenType } from "../src/tokens.ts";
import Value from "../src/values/value.ts";

// deno-lint-ignore no-explicit-any
export function assertTypeOf(object: any, type: any) {
    if (object.constructor != type) {
        throw new AssertionError(`Expected object of type ${object.constructor.name} to have type: ${type.name}`);
    }
}

export function assertMatchingAST(node: Node, expNode: Node) {
    if (!node.isEqualTo(expNode)) {
        throw new AssertionError(`Expected ${expNode} found ${node}`);
    }
}

export function assertMatchingTokens(tokenArray: Token[], expTokenArray: Token[]) {
    for (let i = 0; i < expTokenArray.length; i++) {
        if (i >= tokenArray.length) {
            throw new AssertionError(`Expected ${expTokenArray[i]}, Reached end of token array`);
        }
        if (!tokenArray[i].isType([expTokenArray[i].type]) || tokenArray[i].value != expTokenArray[i].value) {
            throw new AssertionError(`Expected ${expTokenArray[i]}, Found ${tokenArray[i]}`);
        }
    }
    if (tokenArray.length > expTokenArray.length) {
        throw new AssertionError(`Expected no more tokens, Found: ${tokenArray[expTokenArray.length]}`);
    }
}

export function parseResult(text: string): Node{
    const tokens = Lexer.tokensFromLine("<stdin>", text);
    return Parser.parseTokens(tokens);
}

export function assertParseResult(text: string, result: nodeSpec) {
    const expectedTree = makeASTUtility(result);
    assertMatchingAST(parseResult(text), expectedTree);
}

// deno-lint-ignore no-explicit-any
export function assertParseError(text: string, error: any, nextChar?: string) {
    try {
        const tokens = Lexer.tokensFromLine("<stdin>", text);
        Parser.parseTokens(tokens);
    } catch (e) {
        assertTypeOf(e, error);
        if (nextChar) assertEquals(e?.posStart?.nextChar, nextChar);
    }
}

export function assertEqualValues(value1: Value, value2: Value) {
    try {
        assert(value1.equals(value2));
    } catch {
        throw new AssertionError(`Expected ${value2} got ${value1}`)
    }
}

export function assertRuntimeResult(nodeSpec: nodeSpec, value: Value, inContext?: Context): Context {
    const node = makeASTUtility(nodeSpec);
    node.openScope();
    node.doReturnValue();
    const context = inContext ?? Interpreter.newGlobalContext();
    const outValue = node.evaluate(context);
    assertEqualValues(outValue, value);
    return context;
}

// deno-lint-ignore no-explicit-any
export function assertNodeError(nodeSpec: nodeSpec, error: any) {
    const node = makeASTUtility(nodeSpec);
    try {
        node.evaluate(Interpreter.newGlobalContext());
    } catch (e) {
        assertTypeOf(e, error);
    }
}

type binOpSpec = [nodeSpec, TokenType, nodeSpec];
type singleNodeSpec = [number | string];
type unOpSpec = [TokenType | string, nodeSpec];
type scopeSpec = [Record<string, (nodeSpec[] | [])>];
type nodeSpec = (scopeSpec | binOpSpec | singleNodeSpec | unOpSpec);

export function makeASTUtility(spec: nodeSpec): ScopeNode {
    const node = makeInnerNodes(spec);
    return new ScopeNode("global", [node]);
}

export function makeIfNode(specs: [nodeSpec, nodeSpec][]){
    const nodes: [Node, Node][] = specs.map(spec => [makeInnerNodes(spec[0]), makeInnerNodes(spec[1])])
    return new IfNode(Position.dummy, nodes);
}

export function makeForNode(specs: [nodeSpec, nodeSpec, nodeSpec, nodeSpec]){
    const nodes: Node[] = specs.map(spec => makeInnerNodes(spec))
    return new ForNode(nodes[0], nodes[1], nodes[2], nodes[3] as ScopeNode)
}

export function makeWhileNode(specs: [nodeSpec, nodeSpec], isDo: boolean){
    const nodes: Node[] = specs.map(spec => makeInnerNodes(spec))
    return new WhileNode(nodes[0], nodes[1] as ScopeNode, isDo)
}

function makeInnerNodes(spec: nodeSpec): Node {
    const dummyPos = new Position(0, 0, 0, "", "");
    if (spec.length == 3) {
        spec = spec as binOpSpec;
        return new BinOpNode(makeInnerNodes(spec[0]), new Token(spec[1], dummyPos, dummyPos), makeInnerNodes(spec[2]));
    }
    if (spec.length == 2) {
        spec = spec as unOpSpec;
        if (typeof spec[0] == "string") {
            return new VarAsignmentNode(
                new Token(TokenType.IDENTIFIER, dummyPos, dummyPos, spec[0]),
                makeInnerNodes(spec[1]),
            );
        }
        return new UnOpNode(new Token(spec[0], dummyPos, dummyPos), makeInnerNodes(spec[1]));
    }
    if (typeof spec[0] == "string") {
        return new VarRetrievalNode(new Token(TokenType.IDENTIFIER, dummyPos, dummyPos, spec[0]));
    }
    if (typeof spec[0] == "object") {
        const name = Object.keys(spec[0])[0];
        return new ScopeNode(name, spec[0][name].map(makeInnerNodes));
    }

    return new NumberNode(new Token(TokenType.NUMBER, dummyPos, dummyPos, spec[0]));
}

// deno-lint-ignore no-explicit-any
export function makeTokensUtility(...arr: [TokenType, any?][]) {
    const dummyPos = new Position(0, 0, 0, "", "");
    // deno-lint-ignore no-explicit-any
    return arr.map((args: [TokenType, any?]) => new Token(args[0], dummyPos, dummyPos, args[1]));
}
