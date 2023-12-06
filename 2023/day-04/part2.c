#include <stdio.h>
#include <stdlib.h>
#include <string.h>

#define LINE_LENGTH 50
#define NUMBERS 8
#define WINNING_NUMBERS 5

int main() {
  FILE *input = fopen("input.txt", "r");
  char line[LINE_LENGTH];
  char lineCopy[LINE_LENGTH];
  int numbers[NUMBERS];
  int winningNumbers[WINNING_NUMBERS];
  int points;
  int finalAnswer = 1;

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

    for (int j = 0; j < LINE_LENGTH; j++) {
      lineCopy[j] = line[j];
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

    for (int c = 0; c < WINNING_NUMBERS; c++) {
      for (int j = 0; j < NUMBERS; j++) {
        if (winningNumbers[c] == numbers[j]) {
          points++;
        }
      }
    }

    for (int j = 0; j < points; j++) {
      finalAnswer += 1;
    }

    printf("%d: %d\n", i, points);
  }

  printf("Final Answer: %d\n", finalAnswer);

  fclose(input);
}
