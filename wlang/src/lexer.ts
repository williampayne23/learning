import { IllegalCharacterError } from "./errors.ts";
import Position from "./position.ts";
import { Keywords, Token, TokenType } from "./tokens.ts";

type TokenArgs = [TokenType, (string | number | undefined)]
export default class Lexer {
  lastPos: Position
  pos: Position
  tokens: Token[] = [];

  constructor(file: string, text: string) {
    this.pos = new Position(0, 0, 1, file, text)
    this.lastPos = this.pos.copy()
  }

  advance(): string {
    return this.pos.advance()
  }

  success(type: TokenType, value?: string|number) {
    const token = new Token(type, this.lastPos.copy(), this.pos.copy(), value)
    this.tokens.push(token);
    this.lastPos = this.pos.copy()
    return token
  }

  parseNumber(): TokenArgs {
    let number = "";
    let decimalCount = 0;
    while (
      "0123456789.".includes(this.pos.nextChar) && this.pos.nextChar != ""
    ) {
      if (this.pos.nextChar == ".") {
        decimalCount++;
        if (decimalCount > 1) {
          break;
        }
      }
      number += this.pos.nextChar;
      this.advance();
    }
    return [TokenType.NUMBER, parseFloat(number)];
  }

  parseIdentifierOrKeyword(): TokenArgs {
    let name = "";
    while (/[a-zA-Z_]/.test(this.pos.nextChar)) {
      name += this.pos.nextChar;
      this.advance();
    }
    if (name in Keywords) {
      return [TokenType.KEYWORD, name];
    }
    return [TokenType.IDENTIFIER, name];
  }

  static tokensFromLine(file: string, line: string): Token[] {
    const lexer = new Lexer(file, line);
    let comment = false
    while (lexer.pos.nextChar !== "") {
      if(comment){
        if(lexer.pos.nextChar == "#"){
          lexer.advance();
          if(lexer.pos.nextChar == "#"){
            lexer.advance();
            if(lexer.pos.nextChar == "#"){
              lexer.advance();
              comment = false
            }
          }
        }
        lexer.advance()
        continue
      }
      if(lexer.pos.nextChar == "#"){
        lexer.advance();
        if(lexer.pos.nextChar == "#"){
          lexer.advance();
          if(lexer.pos.nextChar == "#"){
            lexer.advance();
            comment = true
          }
        }else{
          while (lexer.pos.nextChar != "\n" && lexer.pos.nextChar != "" ){
            lexer.advance();
          } 
        }
      }else if (lexer.pos.nextChar == "+") {
        lexer.advance();
        lexer.success(TokenType.PLUS);
      } else if (lexer.pos.nextChar == "-") {
        lexer.advance();
        lexer.success(TokenType.MINUS);
      } else if (lexer.pos.nextChar == "/") {
        lexer.advance();
        if(lexer.pos.nextChar == "/"){
          lexer.advance();
          lexer.success(TokenType.FLOORDIVIDE)
          continue
        }
        lexer.success(TokenType.DIVIDE);
      } else if (lexer.pos.nextChar == "*") {
        lexer.advance();
        if(lexer.pos.nextChar == "*"){
          lexer.advance();
          lexer.success(TokenType.POW)
          continue
        }
        lexer.success(TokenType.MULTIPLY);
      } else if (lexer.pos.nextChar == "%") {
        lexer.advance();
        lexer.success(TokenType.MODULUS);
      } else if (lexer.pos.nextChar == "(") {
        lexer.advance();
        lexer.success(TokenType.OPENPAR);
      } else if (lexer.pos.nextChar == ")") {
        lexer.advance();
        lexer.success(TokenType.CLOSEPAR);
      } else if (lexer.pos.nextChar == "{") {
        lexer.advance();
        lexer.success(TokenType.OPENBRACE);
      } else if (lexer.pos.nextChar == "}") {
        lexer.advance();
        lexer.success(TokenType.CLOSEBRACE);
      } else if (lexer.pos.nextChar == "=") {
        lexer.advance();
        if(lexer.pos.nextChar == "="){
          lexer.advance()
          lexer.success(TokenType.EE)
          continue
        }
        lexer.success(TokenType.EQ);
      } else if(lexer.pos.nextChar == "<"){
        lexer.pos.advance();
        if(lexer.pos.nextChar == "<"){
          lexer.advance()
          lexer.success(TokenType.BITLEFT)
          continue
        }
        if(lexer.pos.nextChar == "="){
          lexer.advance()
          lexer.success(TokenType.LTE)
          continue
        }
        lexer.success(TokenType.LT)
      } else if(lexer.pos.nextChar == ">"){
        lexer.advance();
        if(lexer.pos.nextChar == ">"){
          lexer.advance()
          if(lexer.pos.nextChar == ">"){
            lexer.advance()
            lexer.success(TokenType.BITRIGHTZERO)
            continue
          }
          lexer.success(TokenType.BITRIGHT)
          continue
        }
        if(lexer.pos.nextChar == "="){
          lexer.advance()
          lexer.success(TokenType.GTE)
          continue
        }
        lexer.success(TokenType.GT)
      } else if(lexer.pos.nextChar == "&"){
        lexer.advance()
        lexer.success(TokenType.AND)
      } else if(lexer.pos.nextChar == "|"){
        lexer.advance()
        lexer.success(TokenType.OR)
      } else if(lexer.pos.nextChar == "^"){
        lexer.advance()
        lexer.success(TokenType.XOR)
      }else if(lexer.pos.nextChar == "$"){
        lexer.advance()
        lexer.success(TokenType.KEYWORD, "last")
      }else if (lexer.pos.nextChar == "!") {
        const char = lexer.advance();
        if(char == "="){
          lexer.advance();
          lexer.success(TokenType.NEE)
          continue
        }
        lexer.success(TokenType.NOT);
      } else if ("1234567890".includes(lexer.pos.nextChar) && lexer.pos.nextChar != "") {
        lexer.success(...lexer.parseNumber());
      } else if (/[a-zA-Z]/.test(lexer.pos.nextChar)) {
        lexer.success(...lexer.parseIdentifierOrKeyword());
      } else if ("\n".includes(lexer.pos.nextChar)) {
        lexer.advance();
        lexer.success(TokenType.NEWLINE);
      } else if (";".includes(lexer.pos.nextChar)) {
        lexer.advance();
        lexer.success(TokenType.TEMINAL);
      } else if (" \t".includes(lexer.pos.nextChar)) {
        lexer.advance();
        lexer.lastPos = lexer.pos.copy()
        continue;
      } else {
        throw new IllegalCharacterError(lexer.lastPos.copy(), lexer.pos.copy());
      }
    }
    lexer.success(TokenType.EOF)
    return lexer.tokens;
  }
}
