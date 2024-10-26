# Todo 
- Implement the Traits for Term and Operation Expression::members


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

