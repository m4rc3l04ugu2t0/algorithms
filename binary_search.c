#include <stdio.h>
#include <stdlib.h>

const char NUMS[] = {-5, -3, 0, 1, 3, 8, 12};

int search_binary(int target) {
    int low = 0;
    int high = sizeof(NUMS) / sizeof(NUMS[0]) - 1;

    while (low <= high) {
        int mid = (low + high) / 2;

        if (NUMS[mid] == target) {
            return mid;
        } else if (NUMS[mid] < target) {
            low = mid + 1;
        } else {
            high = mid - 1;
        }
    }

    return -1;
}

int main(int argc, char *argv[]) {
    if (argc < 2) {
        printf("Use: %s <number>\n", argv[0]);
        return 1;
    }

    char *endptr;
    long target = strtol(argv[1], &endptr, 10);

    if (*endptr != '\0' || argv[1] == endptr) {
        printf("Inavalid input: %s\n", argv[1]);
        return 1;
    }

    int result = search_binary(target);

    if (result != -1) {
        printf("Number %ld found in index %d.\n", target, result);
    } else {
        printf("Number %ld not found.\n", target);
    }

    return 0;
}

