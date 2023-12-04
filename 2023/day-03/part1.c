#include <stdio.h>
#include <stdlib.h>

#define LINE_LENGTH 142
#define FILE_LENGTH 140

int isDigit(char c) {
  if (c >= '0' && c <= '9') {
    return 1;
  }
  return 0;
}

int main() {
  FILE *input = fopen("input.txt", "r"); // in test.txt part1: 413 part2: 6756
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

    int leftSum = 0;

    int top = 0;
    int topLeft = 0;
    int topRight = 0;

    int bottom = 0;
    int bottomLeft = 0;
    int bottomRight = 0;

    for (int j = 0; line[i][j] != '\n'; j++) {
      // If current character is a symbol print it
      if (!isDigit(line[i][j]) && line[i][j] != '.' && line[i][j] != '\n') {

        // If character to the left or the right of the symbol is a number add
        // it to the sum
        int offset = 1;
        while (isDigit(line[i][j - offset])) {
          leftSum = atoi(&line[i][j - offset]);
          offset++;
        }
        horizontalSum += leftSum;
        leftSum = 0;
        if (isDigit(line[i][j + 1])) {
          horizontalSum += atoi(&line[i][j + 1]);
        }

        // Top
        if (isDigit(line[i - 1][j])) {
          int offset = 0;
          while (isDigit(line[i - 1][j - offset])) {
            top = atoi(&line[i - 1][j - offset]);
            offset++;
          }
          topSum += top;
          top = 0;
        } else {
          // Top left
          int offset = 1;
          while (isDigit(line[i - 1][j - offset])) {
            topLeft = atoi(&line[i - 1][j - offset]);
            offset++;
          }
          topSum += topLeft;
          topLeft = 0;

          // Top right
          offset = 1;
          if (isDigit(line[i - 1][j + offset])) {
            topRight += atoi(&line[i - 1][j + offset]);
            offset++;
          }
        }

        // Bottom
        if (isDigit(line[i + 1][j])) {
          int offset = 0;
          while (isDigit(line[i + 1][j - offset])) {
            bottom = atoi(&line[i + 1][j - offset]);
            offset++;
          }
          bottomSum += bottom;
          bottom = 0;
        } else {
          // Bottom left
          int offset = 1;
          while (isDigit(line[i + 1][j - offset])) {
            bottomLeft = atoi(&line[i + 1][j - offset]);
            offset++;
          }
          bottomSum += bottomLeft;
          bottomLeft = 0;

          // Top right
          offset = 1;
          if (isDigit(line[i + 1][j + offset])) {
            bottomRight += atoi(&line[i + 1][j + offset]);
            offset++;
          }
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
