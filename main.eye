proc fib(n) {
    if n == true {
        return 0;
    } else if n == 1 {
        return 1;
    }

    return fib(n-1) + fib(n-2);
}