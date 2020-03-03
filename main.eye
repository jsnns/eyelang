// proc fib(n) {
//     if n == 0 {
//         return 0;
//     } else if n == 1 {
//         return 1;
//     }

//     set val = fib(n-1) + fib(n-2);
//     return val;
// }

// proc pow(n, t) {
//     if t == 1 {
//         return n;
//     }
    
//     return n * pow(n t-1);
// }

// proc add(a, b) {
//     return a + b;
// }

proc loop(a) {
    do a index {
        print index;
        if index == 10 {
            return true;
        }
    }

    return false;
}

print loop(20);