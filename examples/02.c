#include <stdio.h>

int main() {
    FILE* f = fopen("02.txt", "r");
    int posx = 0;
    int posy = 0;
    while (1) {
        int c = fgetc(f);
        if (c == -1) {
            break;
        }
        // expect f, u, d
        int mode = c;
        while (fgetc(f) != ' ');
        int num = 0;
        while (1) {
            c = fgetc(f);
            if (c == '\n') break;
            num *= 10;
            num += c - '0';
        }
        if (mode == 'f') {
            posx += num;
        } else if (mode == 'u') {
            posy -= num;
        } else {
            posy += num;
        }
    }
    printf("%d, %d: %d\n", posx, posy, posx * posy);
    return 0;
}
