# WLANG
A programming language intepreter written in typescript using Deno

## Install:

```
git clone https://github.com/williampayne23/wlang
cd wlang
```

## Usage

Start the REPL (Read, Evaluate, Print loop) using `yarn start`. 

Execute files with `yarn start FILENAME_1 FILENAME_2 FILENAME_3`

If you want to enter the REPL after executing the file you can with `yarn start FILENAMES --repl` or `yarn start FILENAMES -r`


## Syntax
### Commenting

All lines beginning with a single `#` will be ignored.
Typing `###` will start a multiline comment until the next occurance of `###`

### Simple arithmatic: 

`1 + 1` (= `2`)

`1 - 1` (= `0`)

`1 * 1` (= `1`)

`1 / 1` (= `1`)

`2 ** 3` (= `8`)

`3 % 2` (= `1`) (Modulo operator)

`3 // 2` (= `1`) (Floor division operator)

You can use parentheses to change order of operations.

### Logical Operations and comparisons

`a | b` Logical or bitwise or (depending on type)

`a & b` Logical or bitwise and (depending on type)

`a ^ b` Logical or bitwise xor (depending on type)

`! a`    Logical or bitwise not (depending on type)

`a > b` a is greater than b

`a < b` a is less than b

`a >= b` a is greater than or equal to b

`a <= b` a is less than or equal to b

`a == b` a is equal to b

`a != b` a is not equal to b

### Variable assignment:

Declare a variable with:

`let x = 20` (= `20`)

Variable declarations return the value the variable was assigned to.

Use a variable:

`x + 3` (= `23`)

### Semicolon:

Executing a line of WLANG will return a value unless you use a semicolon

`let x = 20` (= `20`)
`let x = 20;`(= undefined)

When you execute a file or a line of REPL the last returned value is printed.

You can access the last return value with the `last` keyword or `$` operator.

E.g:

```
12 + 2
last
```

Will return 14.

### Scoping

You can define a scope with curly braces `{ }` anything within that scope will have access to it's own variables

```
let x = 3
{
        x (Will return errror)
        x = 4;
        x (Will return 4)
}
x (Will return 3)

```

### Control flow

#### If statements

If, else if statements, and else statements are written like this:

```
if condition {
expression
} elif condition {
expression
} else {
expression
}
```

You can also write a one line ternary statement which returns the result of whatever code is executed

```
if condition then expression elif condition expression else expression
```

#### For loop

The for loop will evaluate expression1 before the loop starts, it will run the loop so long as the comparison evaluates to true, and at the end of every loop iteration it will run expression 2. 

```
for (expression1; comparison; expression2){
        LOGIC
}
```

#### While and do while loop

The while loop will execute so long as the comparative expression evaluates to true.


```
while (comp-expr) {
        LOGIC
}

The do-while loop will execute so long as the comparative expression evaluates to true at the end of every loop.

do {
        LOGIC
} while (comp-expr)
```
## Grammar

At the highest level, the parser will first attempt to find a scope with EOF marking the end of the scope

        global-scope    
                : (expression (SEMICOLON | NEWLINE | EOF) NEWLINE*)+ EOF

        scope    
                : (expression SEMICOLON? ( NEWLINE* | CLOSEBRACE))+ CLOSEBRACE

        expression  
                : OPENBRACE scope
                : KEYWORD:let assignment
        TODO    : KEYWORD:function function
                : KEYWORD:if if  
                : KEYWORD:for for 
                : KEYWORD:while while 
                : KEYWORD:do do-while 
                : comp-expr ((AND | OR) comp-expr)*

        function
        TODO    : OPENPAREN IDENTIFIER (IDENTIFIER? (COMMA IDENTIFIER)*) CLOSEPAREN scope
        TODO    : OPENPAREN IDENTIFIER (IDENTIFIER? (COMMA IDENTIFIER)*) CLOSEPAREN ARROW expression

        if      
                : expression scope (elif expression scope)* (else scope)?
                : expression then expression (elif expression then expression)* (else expression)?

        for     
                : OPENPAREN KEYWORD:let asignment; comp-expr; assignment CLOSEPAREN scope

        while   
                : OPENPAREN comp-expr CLOSEPAREN scope
        
        do-while
                : scope while OPENPAREN comp-expr CLOSEPAREN


        asignment
                : IDENTIFIER EQ expression

        comp-expr
                : NOT comp-expr
                : arith-expr ((LT | GT | LTE | GTE | EE | NE) arith-expr)

        arith-expr
                : term ((PLUS | MINUS ) term)*
        
        term        
                : power ((MULT | DIV | FLOORDIV | MODULO) power)*
            
        power       
                : factor (POW) factor

        factor      
                : NUMBER
        TODO    : function-call
                : IDENTIFIER
                : KEYWORD:last | !!
                : (MINUS) factor
                : (LEFTPAREN) expr (RIGHTPAREN)

        function-call
        TODO    : IDENTIFIER OPENPAREN expression? (COMMA expression)* CLOSEPAREN 




