#include <stdio.h>
#include <stdlib.h>

#define LINE_LENGTH 14
#define FILE_LENGTH 12

int isDigit(char c) {
  if (c >= '0' && c <= '9') {
    return 1;
  }
  return 0;
}

int main() {
  FILE *input = fopen("test.txt", "r"); // in test.txt part1: 413 part2: 6756
  char line[FILE_LENGTH][LINE_LENGTH];
  int finalAnswer = 0;

  // File check
  if (input == NULL) {
    printf("File couldn't open\n");
    return 0;
  }

  int lineIdx = 0;
  while (fgets(line[lineIdx], LINE_LENGTH, input)) {
    lineIdx++;
  }

  for (int i = 0; i < FILE_LENGTH; i++) {
    int horizontalSum = 0;
    int topSum = 0;
    int bottomSum = 0;

    int top = 0;
    int topLeft = 0;
    int topRight = 0;

    int bottom = 0;
    int bottomLeft = 0;
    int bottomRight = 0;
    for (int j = 0; j < LINE_LENGTH; j++) {
      // If current character is a symbol print it
      if (!isDigit(line[i][j]) && line[i][j] != '.' && line[i][j] != '\n') {

        // If character to the left or the right of the symbol is a number add
        // it to the sum
        int offset = 1;
        while (isDigit(line[i][j - offset])) {
          horizontalSum = atoi(&line[i][j - offset]);
          offset++;
        }
        if (isDigit(line[i][j + 1])) {
          horizontalSum += atoi(&line[i][j + 1]);
        }

        // Top
        offset = 0;
        while (i != 0 && isDigit(line[i - 1][j - offset])) {
          top = atoi(&line[i - 1][j - offset]);
          offset++;
        }

        // Top left
        offset = 0;
        while (i != 0 && !isDigit(line[i - 1][j]) &&
               isDigit(line[i - 1][j - offset - 1])) {
          topLeft = atoi(&line[i - 1][j - offset - 1]);
          offset++;
        }
        top += topLeft;
        topLeft = 0;

        // Top right
        offset = 0;
        if (i != 0 && !isDigit(line[i - 1][j]) &&
            isDigit(line[i - 1][j + offset + 1])) {
          topRight = atoi(&line[i - 1][j + offset + 1]);
          offset++;
        }

        // Bottom
        offset = 0;
        while (i != LINE_LENGTH - 1 && isDigit(line[i + 1][j - offset])) {
          bottom = atoi(&line[i + 1][j - offset]);
          offset++;
        }

        // Bottom left
        offset = 0;
        while (i != LINE_LENGTH - 1 && !isDigit(line[i + 1][j]) &&
               isDigit(line[i + 1][j - offset - 1])) {
          bottomLeft = atoi(&line[i + 1][j - offset - 1]);
          offset++;
        }

        // Bottom Right
        offset = 0;
        if (i != LINE_LENGTH - 1 && !isDigit(line[i + 1][j]) &&
            isDigit(line[i + 1][j + offset + 1])) {
          bottomRight = atoi(&line[i + 1][j + offset + 1]);
          offset++;
        }
      }
    }
    topSum += top + topLeft + topRight;
    bottomSum += bottom + bottomLeft + bottomRight;
    printf("%d %d %d\n", topSum, horizontalSum, bottomSum);
    finalAnswer += topSum + horizontalSum + bottomSum;
  }
  printf("Final answer: %d\n", finalAnswer);

  fclose(input);
}
