#include "cmark/cmark.h"

extern int VAL;

int sandboxed(int i);
int sandboxed2(int i);
void nop();

const int *foo(int *p);
