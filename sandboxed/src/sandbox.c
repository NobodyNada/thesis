int VAL = 5;

int sandboxed(int i) { return i * 3 * VAL; }

int sandboxed2(int i) { return i * 2 * VAL; }

int *foo(int *p) { return p; }

void nop() {}
