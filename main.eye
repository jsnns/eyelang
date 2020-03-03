proc fib(n) {
    if n == 0 {
        return 0;
    } else if n == 1 {
        return 1;
    }

    set val = fib(n-1) + fib(n-2);
    return val;
}

// val should not exist
print val;

print fib(10);