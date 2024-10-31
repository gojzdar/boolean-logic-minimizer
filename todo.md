# Todo 

## Split AND OR XOR into own objects
### Problem 
More work, complexity 

Pros: 
- each can independantly implement traits 
- explicit operator trait objects can exist 
- cleaner code for implementing Operator traits 
- could allow more generic optimizations, like is_associative for reducing tree depth, without duplicating code 

Cons:
- some complexity 
- not really a priority, until more operators are added 
- uglier code in some cases 

## Sort items in commutative expressions 
- constants << vars << terms << (operations sorted by the same fn, then depth) << functions 


## Impl variables that hold expressions 
Variables can hold expressions like: 
`a := x1`
Could be used the same ways as functions.


## Implement functions 
Operations, that have named arguments with local scope. 
Should be easy to implement 

**be careful about var visibility**

Pros: 
- reusable placeholder expressions 
- could be defined as expressions with unbounded variables 
    - so any letter holding an expression could be a function 
    - `p = x1`, would allow for: `p(x1=1)` that would evaluate as `p = 1`
    - there could be problems when combined with global variables 
        - solution 1: `p[x1=1]` distinguishes a function call and makes the parser easier to write
        - solution 2: all functions require `f` name prefix, so `fa` is a valid name but `a` isn't. 
            - still requires you to somehow specify the arguments passed into the function 
    - so saying a:= x1 would create a function. 
    - decide if function arguments can be expressions / functions 
        - if no: simple, just constants 
        - if yes: can be expressions 

## Implement forall, exists, exists unique 
- evaluate() is a loop through 0/1 value for each binded variable 


Pros: 
- allows for simple SAT solvers, where you just say Exists(a,b,c,d) f(a,b,c,d)
- allows for: A1, A2, ... |= B checkers 
    - Forall(vars in A1, A2, ...) A1 & A2 & ... => B 
- not too difficult to implement Proof of concept, if not counting parser work 

Cons: 
- lots of work 
- needs better parser 
    - needs support for: 
        - Expression lists: Exp1, Exp2, ... 
        - parse time operators: (vars in {expression list})
- not a priority 





# Done 
## Change AST simplification 
### Problem 
The current Simplify trait can't work, since the Expression::member types would have to modify the parent that holds them. 

Solutions:
1. Make the simplification consume the expression::member and return its replacement. 
2. Make the simplification a trait of Expression itself 
    - less modular 

### Solution 
Chose option 1. Probably not most memory or time efficient, unless rust compiler does magic. 
But it is elegant, Expression implements Simplify for free and requires least code change

