#include <stdio.h>

typedef struct PuzzleLine
{
    char bin[12];
    char check;
} PuzzleLine;

const int PUZZLE_LEN = 1000;

void resetPuzzle(PuzzleLine* puzzle) {
    for (int i = 0; i < PUZZLE_LEN; i++) {
        puzzle[i].check = 1;
    }
}

int to_num(char* bin, int count) {
    int out = 0;
    for (int i = 0; i < count; i++)
    {
        out <<= 1;
        if (bin[i] == '1') {
            out++;
        }
    }
    return out;
}

int main() {
    // part2
    FILE* f = fopen("03.txt", "r");
    PuzzleLine puzzle[1000];
    int len = fread(puzzle, 1, sizeof(puzzle), f);
    printf("%d\n", len);
    int oxyg = 0;
    int co2 = 0;
    for (int i = 0; i < 12; i++) {
        int total_count = 0;
        int set_count = 0;
        for (int j = 0; j < PUZZLE_LEN; j++) {
            if (puzzle[j].check) {
                total_count++;
                if (puzzle[j].bin[i] == '1') {
                    set_count++;
                }
            }
        }
        int char_to_keep = set_count >= total_count - set_count ? '1' : '0';
        int match_count = 0;
        int last_matched = 0;
        for (int j = 0; j < PUZZLE_LEN; j++) {
            if (puzzle[j].check) {
                if (puzzle[j].bin[i] == char_to_keep) {
                    match_count++;
                    last_matched = j;
                } else {
                    puzzle[j].check = 0;
                }
            }
        }
        if (match_count == 1) {
            oxyg = to_num(puzzle[last_matched].bin, 12);
        }
    }
    resetPuzzle(puzzle);
    for (int i = 0; i < 12; i++) {
        int total_count = 0;
        int set_count = 0;
        for (int j = 0; j < PUZZLE_LEN; j++) {
            if (puzzle[j].check) {
                total_count++;
                if (puzzle[j].bin[i] == '1') {
                    set_count++;
                }
            }
        }
        int char_to_keep = set_count < total_count - set_count ? '1' : '0';
        int match_count = 0;
        int last_matched = 0;
        for (int j = 0; j < PUZZLE_LEN; j++) {
            if (puzzle[j].check) {
                if (puzzle[j].bin[i] == char_to_keep) {
                    match_count++;
                    last_matched = j;
                } else {
                    puzzle[j].check = 0;
                }
            }
        }
        if (match_count == 1) {
            co2 = to_num(puzzle[last_matched].bin, 12);
        }
    }
    printf("%d, %d, %d\n", oxyg, co2, oxyg * co2);
    return 0;
}
