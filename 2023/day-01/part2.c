#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#define MAX 60

void replaceStr(char *target, const char *needle, const char *replacement) {
  char buffer[MAX] = {0};
  char *insert_point = &buffer[0];

  const char *tmp = target;
  size_t needle_len = strlen(needle);
  size_t repl_len = strlen(replacement);

  while (1) {
    const char *p = strstr(tmp, needle);

    // walked past last occurrence of needle; copy remaining part
    if (p == NULL) {
      strcpy(insert_point, tmp);
      break;
    }

    // copy part before needle
    memcpy(insert_point, tmp, p - tmp);
    insert_point += p - tmp;

    // copy replacement string
    memcpy(insert_point, replacement, repl_len);
    insert_point += repl_len;

    // adjust pointers, move on
    tmp = p + needle_len;
  }

  // write altered string back to target
  strcpy(target, buffer);
}

int compare(const void *x, const void *y) {
  char a = *((char *)x);
  char b = *((char *)y);

  if (a < b)
    return -1;
  else
    return 1;
}

void replaceWordNumbers(char *string) {
  char *wordNumbers[9] = {"one", "two",   "three", "four", "five",
                          "six", "seven", "eight", "nine"};
  char *numbers[9] = {"o1e", "t2o", "t3e", "f4r", "f5e",
                      "s6x", "s7n", "e8t", "n9e"};

  for (int i = 0; i < 9; i++) {
    replaceStr(string, wordNumbers[i], numbers[i]);
  }
}

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
    replaceWordNumbers(line);
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
