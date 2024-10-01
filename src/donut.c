#include "../include/pros/screen.h"

static char do_char[21][40] = {
        "             k;double sin()             ",
        "         ,cos();main(){float A=         ",
};

void donut()
{
        int i = 0;
        while (i < 2) {
                screen_print(0, i, do_char[1]);
        }
}
