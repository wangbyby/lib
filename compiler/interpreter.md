Source Code -lexer-> Tokens --> abstract syntax tree
1. lexer Lexical Analysis
   1. 工作内容
        the lexer's job is not to tell us whether code makes sense, works or contains errors.
   2. REPL read eval print loop
    the REPL reads input, sends it to the interpreter for evaluation, prints the result/ouput of the interpreter and starts again. Read, Eval, Print, Loop.

- parser
    1. 定义
        input data(frequently text) --> a data structure -often some kind of parse tree, AST
    2. AST abstract syntax tree
    3. parsing a programming language 


       1. top-down parsing (Pratt Parsing)
            Partt parsing was invented as an alternative to parsers based on context-free grammars and th BNF

          1. recursive descent parsing
          2. Early parsing
          3. predictive parsing
       2. bottom-up parsing
    4. statements & expressions
        expressions produce values, statement don't
        1. statement
            1. let-statement

                let <identifier> = <expression>
            2. return-statement
        2. expression

    5. how partt parsing works
        

- tree representation
- evaluation
  - tree walking interpreter
  - JIT
  - represent vales
    - native types of the host languages
    - values/objects --> pointers 
    - mixed
        source code --> ast.IntegerLiteral --eval that AST node --> Object.Integer, saving the value inside out struct and passing around a reference to this struct



- 语言目标
  - 变量绑定
  - 数组
  - 哈希
  - String