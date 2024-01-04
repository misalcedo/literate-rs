## Expressions
```tortuga
x = [0, 100]
z = (0, 100)
y = [
    2 + 2
]
```


```
{x, y, z} = { 1#1.0, x,y}(0)
{x, y | z} = { 1#1.0, x,y}(0)
1 + 2

X = 3

x.0 = 4.0

x._ = 4.5
x.y = 4.2

f(x, y) = 2 * 3

f(n = [0, 100]) = n / 100
f(x.0) = [
    a = 2^5
    g(b) = a + b
]

f(x) = 2*x^2 + 3*x - 5

f((x, y)) = y - x

02 + .20
```

## Factorial
Calculate the factorial of an integer recursively:

```
factorial(n = 0) = 1
factorial(n.0 > 0) = n * factorial(n - 1)
```

## Fibonnaci
```tortuga
fibonacci(n <= 1) = n
fibonacci(n) = [
    fibonacci(n - 2) + fibonacci(n - 1)
]
```

##
Empty heading