proc fib(n) {
    if n == 0 {
        return 0;
    } else if n == 1 {
        return 1;
    }

    return fib(n-1) + fib(n-2);
}

print fib(10);