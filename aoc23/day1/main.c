#include <ctype.h>
#include <stdint.h>
#include <stdio.h>

struct word {
    int len;
    uint64_t word;
};

const static int kNumDigitWords = 9;

const static struct word kDigitWords[] = {
    {.len = 3, .word = 0x6F6E65},
    {.len = 3, .word = 0x74776F},
    {.len = 5, .word = 0x7468726565},
    {.len = 4, .word = 0x666F7572},
    {.len = 4, .word = 0x66697665},
    {.len = 3, .word = 0x736978},
    {.len = 5, .word = 0x736576656E},
    {.len = 5, .word = 0x6569676874},
    {.len = 4, .word = 0x6E696E65},
};

int word_digit(uint64_t buf) {
    for (int d = 0; d < kNumDigitWords; d++) {
        int bits_of_word = kDigitWords[d].len * 8;
        uint64_t mask = (1ULL << bits_of_word) - 1;
        if (kDigitWords[d].word == (buf & mask)) {
            return d + 1;
        }
    }
    return -1;
}

int digit(uint64_t buf) {
    int c = buf & 0xff;
    if (isdigit(c)) {
        return c - '0';
    }
    return word_digit(buf);
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

int part_two() {
    int c;
    int sum = 0;
    int first = -1, last = -1;
    uint64_t buf = 0;
    while ((c = getc(stdin)) != EOF) {
        buf = (buf << 8) | c;
        int d = digit(buf);
        if (d > 0) {
            first = (first >= 0) ? first : d;
            last = d;
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
    printf("Result: %d\n", part_two());
    return 0;
}