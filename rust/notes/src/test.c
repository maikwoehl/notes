#include <stdlib.h>
#include <stdio.h>
#include <string.h>

char** test(char **message) {
    *message = "Test\0";
    return message;
}

int main(void) {
    char *text = "Hello, world!";
    printf("String is '%s'\n", text);

    char** message = test(&text);
    printf("String is '%c'\n", *(text+2));
    printf("String is '%s'\n", message);
}

