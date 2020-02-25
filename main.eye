proc do_a_thing(a: I32, b: I32): I32 {
    return a + b;
}

main proc do_things() {
    return do_a_thing(5, 2);
}