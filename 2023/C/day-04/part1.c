#include <stdio.h>
#include <stdlib.h>
#include <string.h>

#define LINE_LENGTH 118
#define NUMBERS 25
#define WINNING_NUMBERS 10

int main() {
  FILE *input = fopen("input.txt", "r");
  char line[LINE_LENGTH];
  char lineCopy[LINE_LENGTH];
  int numbers[NUMBERS];
  int winningNumbers[WINNING_NUMBERS];
  int points;
  int finalAnswer = 0;

  if (input == NULL) {
    printf("File couldn't open");
    return 0;
  }

  for (int i = 1; fgets(line, LINE_LENGTH, input); i++) {
    points = 0;

    // Index that refers to the semicolon
    int indexAfterCard = (int)(strchr(line, ':') - line) + 2;

    // Remove Game xx: from line
    for (int j = 0; line[j] != '\n'; j++) {
      line[j] = line[j + indexAfterCard];
    }

    for (int i = 0; i < LINE_LENGTH; i++) {
      lineCopy[i] = line[i];
    }

    char *token = strtok(lineCopy, "|");

    // Insert all winning numbers into its array
    char *winningNumberToken = strtok(token, " ");

    for (int k = 0; winningNumberToken != NULL; k++) {
      winningNumbers[k] = atoi(winningNumberToken);
      winningNumberToken = strtok(NULL, " ");
    }

    // Loop to next token
    token = strtok(line, "|");
    token = strtok(NULL, "|");

    // Insert all numbers into its array
    char *numberToken = strtok(token, " ");

    for (int k = 0; numberToken != NULL; k++) {
      numbers[k] = atoi(numberToken);
      numberToken = strtok(NULL, " ");
    }

    for (int i = 0; i < WINNING_NUMBERS; i++) {
      printf("%2i ", winningNumbers[i]);

      for (int j = 0; j < NUMBERS; j++) {
        if (winningNumbers[i] == numbers[j]) {
          if (points == 0) {
            points++;
          } else {
            points *= 2;
          }
        }
      }
    }
    printf(" ");

    for (int i = 0; i < NUMBERS; i++) {
      printf("%2i ", numbers[i]);
    }

    printf("\n");

    finalAnswer += points;
  }

  printf("Final Answer: %d\n", finalAnswer);

  fclose(input);
}
