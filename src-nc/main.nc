#include <stdio.h>
#include <stdlib.h>
#include <string.h>

int main()
{
    char *test = malloc(5);
    defer free(test);

    test.strcpy("Hello, world!\n");

    printf("%s", test);
}
