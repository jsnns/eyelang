// define test to be {
//     print "Test";
// }

// define a to be 10;

// define z to be 9;

// if z is 10 {
//     print "yay";
// } else if z is 5 {
//     print "nay";
// }

// run test;

// define add to be { return a + b; } given (a, b);

// print run add given (1, 2);
// print a;

define add_num to be {
    return a + b;
} given (a, b);

print run add_num given (1, 2);

define fib to be {
    if n is 0 {
        return 0;
    } else if n is 1 {
        return 1;
    } else {
        return fib(n-1) + fib(n-2);
    }
} given (n);

print run fib given (10);