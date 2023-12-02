#include <stdio.h>
#include <stdlib.h>
#include <string.h>

#define MAX 200

typedef struct game {
  int id;
  int maxRed;
  int maxGreen;
  int maxBlue;
} game;

int main() {
  FILE *input = fopen("input.txt", "r");
  char line[MAX];
  game game[101] = {0};
  int finalAnswer = 0;

  // File check
  if (input == NULL) {
    printf("File couldn't open\n");
    return 0;
  }

  for (int lineIdx = 1; fgets(line, MAX, input); lineIdx++) {
    // Index that refers to the semicolon
    int indexAfterGame = (int)(strchr(line, ':') - line) + 2;

    game[lineIdx].id = lineIdx;

    // Remove Game xx: from line
    for (int i = 0; line[i + indexAfterGame] != '\n'; i++) {
      line[i] = line[i + indexAfterGame];
    }

    char *color = strtok(line, " ");
    char *number = strtok(NULL, " ");

    int firstLoop = 1;
    while (color != NULL) {
      // On the first loop the values are flipped
      if (firstLoop) {
        char *tmp = color;
        color = number;
        number = tmp;
      }

      // Insert the maximum red amount in game.maxRed
      if (color[0] == 'r' && atoi(number) > game[lineIdx].maxRed) {
        game[lineIdx].maxRed = atoi(number);
      }

      // Insert the maximum green amount in game.maxGreen
      if (color[0] == 'g' && atoi(number) > game[lineIdx].maxGreen) {
        game[lineIdx].maxGreen = atoi(number);
      }

      // Insert the maximum blue amount in game.maxBlue
      if (color[0] == 'b' && atoi(number) > game[lineIdx].maxBlue) {
        game[lineIdx].maxBlue = atoi(number);
      }

      // Loop to next substring between 2 spaces
      number = strtok(NULL, " ");
      color = strtok(NULL, " ");
      firstLoop = 0;
    }

    finalAnswer +=
        game[lineIdx].maxRed * game[lineIdx].maxGreen * game[lineIdx].maxBlue;
  }
  printf("Final Answer: %d\n", finalAnswer);

  fclose(input);
}
