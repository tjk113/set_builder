#include <string.h>
#include <stdio.h>
#include <math.h>

#define MAX_OUTPUT_LEN 10000
#define MAX_NUM_LEN       20

_Bool element_filter(float x) {
    return ELEMENT_FILTER_CONDITION ? 1 : 0;
}

float element_operation(float x) {
    ELEMENT_OPERATION_DEFINITION;
    return x;
}

int main() {
    char output[MAX_OUTPUT_LEN] = {0};
    char* output_ptr = output;

    const float range_start = RANGE_START, range_end = RANGE_END;

    *output_ptr = '[';
    output_ptr++;
    for (int x = range_start; x <= range_end; x++) {
        if (!element_filter(x))
            continue;
        float output_num = element_operation(x);
        // Copy the stringified number to the output buffer
        char num_str[MAX_NUM_LEN];
        if (output_num == (int)output_num)
            sprintf(num_str, "%d", (int)output_num);
        else
            sprintf(num_str, "%.2f", output_num);
        strcpy(output_ptr, num_str);
        /* Add a comma separator and increment
        the pointer to the output buffer */
        sprintf(output_ptr + strlen(num_str), ", ");
        // Increment pointer
        output_ptr += strlen(num_str) + 2;
    }
    output_ptr -= 2;
    *output_ptr = ']';
    output_ptr++;
    *output_ptr = '\0';

    printf("%s", output);

    return 0;
}