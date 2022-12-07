import tkmandel
from timeit import timeit
print(tkmandel.add2(3, 4))
print(tkmandel.add3(3, 4, 5))
test = tkmandel.Bahd(5)
print(test)
print(test.square())

def fib(n):
    if n <= 2:
        return n
    return fib(n-1) + fib(n-2)

for i in range(1, 40):
    print("Pyth: ", i , timeit(lambda: fib(i), number=5))
    print("Rust: ", i , timeit(lambda: tkmandel.fib(i), number=5))