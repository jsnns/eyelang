proc some_nums() {
    return 100 + 25;
}

proc double() {
    return 2 * some_nums();
}

proc test() {
    print true;
    return true;
}

proc m() {
    proc add(a, b) {
        return a + b;
    }

    set i = 5;
    set j = 1;

    return add(i, j + 2);
}

print 2 - m();