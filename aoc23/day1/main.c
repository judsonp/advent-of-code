#include <ctype.h>
#include <stdio.h>

int digit(int c) {
    return c - '0';
}

int part_one() {
    int c;
    int sum = 0;
    int first = -1, last = -1;
    while ((c = getc(stdin)) != EOF) {
        if (isdigit(c)) {
            first = (first >= 0) ? first : digit(c);
            last = digit(c);
        }
        if (c == '\n') {
            sum += first * 10 + last;
            first = -1, last = -1;
        }
    }
    if (first >= 0) {
        sum += first * 10 + last;
    }
    return sum;
}

int main(int argc, char ** argv) {
    printf("Result: %d\n", part_one());
    return 0;
}