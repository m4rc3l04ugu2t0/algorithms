#include <stdio.h>
#include <stdlib.h>
#include <string.h>

const char NUMS[] = {-5, -3, 0, 1, 3, 8, 12};

int search_binary(int target) {
  int low = 0;
  int high = strlen(NUMS);

  while (low <= high) {
    int mid = (low + high) / 2;

    if (NUMS[mid] == target) {
      return mid;
    } else if (NUMS[mid] < target) {
      low = mid + 1;
    } else {
      high = mid -1;
    }
  }


  return -1;
}

int main(int argc, char *argv[]) {
  if (argc < 2) {
    return 1;
  } else {
    char *endptr;
    long target = strtol(argv[1], &endptr, 10);

    if (*endptr != '\0') {
      printf("invalid");
      return 1;
    }

    search_binary(target);
  }
  return 0;
}
