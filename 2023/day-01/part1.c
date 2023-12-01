#include <stdio.h>
#include <stdlib.h>
#define MAX 60

int main() {
  FILE *input = fopen("input.txt", "r");
  char line[MAX];
  int digit1;
  int digit2;
  int lineValue;
  int finalValue = 0;

  // File check
  if (input == NULL) {
    printf("File couldn't open");
    return 0;
  }

  while (fgets(line, MAX, input)) {
    // First digit
    for (int i = 0; line[i] != '\n'; i++) {
      if (line[i] >= '0' && line[i] <= '9') {
        digit1 = atoi(&line[i]);

        // Get only first digit of the number
        while (digit1 > 10) {
          digit1 /= 10;
        }

        break;
      }
    }

    // Last digit
    for (int i = 0; line[i] != '\n'; i++) {
      if (line[i] >= '0' && line[i] <= '9') {
        digit2 = atoi(&line[i]);

        // Get only first digit of the number
        while (digit2 > 10) {
          digit2 /= 10;
        }
      }
    }

    lineValue = digit1 * 10 + digit2;
    printf("%d", lineValue);
    printf("\n");

    finalValue += lineValue;
  }

  printf("Final answer: %d\n", finalValue);
  fclose(input);
}
