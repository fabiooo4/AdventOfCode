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
  FILE *input = fopen("input.txt", "r");
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
    int partsMult;

    int parts;
    for (int j = 0; line[i][j] != '\n'; j++) {
      if (line[i][j] == '*') {
        partsMult = 1;
        parts = 0;

        // Count parts
        // Left
        if (isDigit(line[i][j - 1])) {
          int tmp = 0;
          int offset = 1;
          parts++;

          while (isDigit(line[i][j - offset])) {
            tmp = atoi(&line[i][j - offset]);
            offset++;
          }
          partsMult *= tmp;
        }

        // Right
        if (isDigit(line[i][j + 1])) {
          parts++;
          partsMult *= atoi(&line[i][j + 1]);
        }

        // Top
        if (isDigit(line[i - 1][j])) {
          int tmp = 0;
          int offset = 0;
          parts++;

          while (isDigit(line[i - 1][j - offset])) {
            tmp = atoi(&line[i - 1][j - offset]);
            offset++;
          }
          partsMult *= tmp;
        }

        // Top left
        if (!isDigit(line[i - 1][j]) && isDigit(line[i - 1][j - 1])) {
          int tmp = 0;
          int offset = 1;
          parts++;

          while (isDigit(line[i - 1][j - offset])) {
            tmp = atoi(&line[i - 1][j - offset]);
            offset++;
          }
          partsMult *= tmp;
        }

        // Top right
        if (!isDigit(line[i - 1][j]) && isDigit(line[i - 1][j + 1])) {
          parts++;
          partsMult *= atoi(&line[i - 1][j + 1]);
        }

        // Bottom
        if (isDigit(line[i + 1][j])) {
          int tmp = 0;
          int offset = 0;
          parts++;

          while (isDigit(line[i + 1][j - offset])) {
            tmp = atoi(&line[i + 1][j - offset]);
            offset++;
          }
          partsMult *= tmp;
        }

        // Bottom left
        if (!isDigit(line[i + 1][j]) && isDigit(line[i + 1][j - 1])) {
          int tmp = 0;
          int offset = 1;
          parts++;

          while (isDigit(line[i + 1][j - offset])) {
            tmp = atoi(&line[i + 1][j - offset]);
            offset++;
          }
          partsMult *= tmp;
        }

        // Bottom right
        if (!isDigit(line[i + 1][j]) && isDigit(line[i + 1][j + 1])) {
          parts++;
          partsMult *= atoi(&line[i + 1][j + 1]);
        }

        if (parts == 2) {
          finalAnswer += partsMult;
          printf("%d %d\n", parts, partsMult);
        } else {
          partsMult = 1;
        }
      }
    }
  }
  printf("Final answer: %d\n", finalAnswer);

  fclose(input);
}
