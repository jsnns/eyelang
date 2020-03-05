define fib to be {
    if n is 0 {
        return 0;
    } else if n is 1 {
        return 1;
    } else {
        return fib(n-1) + fib(n-2);
    }
} given (n);

// prints '55' to stdout
print run fib given (10);