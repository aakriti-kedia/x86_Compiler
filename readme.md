A compiler which implements compiling and parsing functionality for basic unary, binary operations, let bindings, print, if-else, loop, break, function calls and heap memory allocations (arrays). 

It follows the following grammar

<prog> := <defn>* <expr>                (new!)
<defn> := (fun (<name> <name>*) <expr>) (new!)
<expr> :=
  | <number>
  | true
  | false
  | input
  | <identifier>
  | (let (<binding>+) <expr>)
  | (<op1> <expr>)
  | (<op2> <expr> <expr>)
  | (set! <name> <expr>)
  | (if <expr> <expr> <expr>)
  | (block <expr>+)
  | (loop <expr>)
  | (break <expr>)
  | (<name> <expr>*)                    (new!)

<op1> := add1 | sub1 | isnum | isbool | print (new!)
<op2> := + | - | * | < | > | >= | <= | =

<binding> := (<identifier> <expr>)


Explicit tests can be viewed in the test directory
Command: make test