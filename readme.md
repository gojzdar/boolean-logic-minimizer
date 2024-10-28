# Capabilities 
Given a list of minterms, provide the minimized equivalent expression.

# Vision
Far future:

```
$ f := ~x1 + ~x2 ~x3 x4 
$ .table x 
x1 | x2 | x3 | x4 | f(x1, x2, x3, x4)
<truth table for f> 

$ .minimize f 
<minimal DNF of f> 

$ .minterms f 
<list f's minterms>

$ .maxterms f 
<list f's maxterms> 

$ .displaylatex 
Output switched to latex expression 

$ .reduce a => b 
\lnot a \vee b 

$ g := x1 & 1 
$ .simplify g 
x_{1} 

$ .printAST f 
+
    ~
        x1 
    ^
        ~
            x2 
        ~
            x3 
        x4 
```

## Operators
- (t) 
    - if t == const -> const 
    - if t == var -> var 
    - else -> (expr) 
- ~{t}
    - if t == const -> ~const 
    - if t == var -> ~var 
    - else ~{ expr }
- {t1} | {t2} 
- {t1} & {t2}
    - if t1 == t2 == var -> term{t1, t2}
    - else &{t1, t2} 
- {t1} ^ {t2} XOR 
- {t1} = {t2} XNOR
- {t1} => {t2} IMPL 

- {t1} {t2} CONCAT 
    - if t1 == t2 == var -> term {t1, t2}
    - else PARSING ERROR 

## Possible AST optimizations

### Term equality 
NOT 
    NOT 
        ... 
Evaluates to 
...

--- 
OR 
    var a 
    var a 
Evaluates to 
var a 

OR 
    var a 
    ~ 
        var a 
    ... 
Evaluates to 
1 



### Known constants 
AND 
    0 
    ... 
Evaluates to 0 

---
XOR 
    1 

    a 
Evaluates to 
~a 

---
XOR  
    0 
    ... 
Evalues to 
XOR 
    ... 

---



### Chainable operation 
OR 
    OR 
        ...1 
    OR 
        ...2  
Is the same as 
OR 
    ...1 
    ...2 








