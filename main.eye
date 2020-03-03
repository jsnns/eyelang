proc fib(n) {
    if n == 0 {
        return 0;
    } else if n == 1 {
        return 1;
    }

    set val = fib(n-1) + fib(n-2);
    return val;
}

proc add(a, b) {
    return a + b;
}

set add_ = add;

print add_(1, 2);