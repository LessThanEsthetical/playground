#include <stdio.h>
#include <stdbool.h>

/*int main(void)
{
    int a, b;
    scanf("%d %d", &a, &b);
    bool c = a < b;

    printf("c = %d\n", c);

    if (c) 
    {
        printf("a less than b\n");
    }
    else 
    {
        printf("a greater than or equal to b\n");
    }
    return 0;
}*/

/* int main(void)
{
    bool is_locked = true;
    bool can_edit = !is_locked;

    printf("is_locked = %d\n", is_locked);
    printf("can_edit = %d\n", can_edit);
    return 0;
} */

/* int main(void)
{
    int a = -2 > 5;          // 0
    int b = 0 < 7;           // 1
    int c = 0 == 0;          // 1
    int d = a && b || c;        // 1
    
    printf("a = %d \n", a);
    printf("b = %d \n", b);
    printf("c = %d \n", c);
    printf("d = %d \n", d);

    return 0;
} */ 

/* int main(void)
{
    int a = 19;
    int b = 10;
    bool c = a < b;

    printf("c = %d\n", c);

    (a < b) ? printf("a is less than b\n") : printf("a is greater than or equal to b\n");
    
    return 0;
} */

/* int main(void)
{
    int choice;
    
    printf("choose action: 1 is hello, 2 is bye: ");
    scanf("%d", &choice);

    switch (choice) 
    {
        case 1:
            printf("hello!\n");
            break;

        case 2:
            printf("bye!\n");
            break;

        default:
            printf("any of them.\n");
            break;
    }

    return 0;
} */

/* int main(void)
{
    unsigned char var = 153; // 10011001
    unsigned char var2 = ~var; // 01100110

    // I like your code
    printf("var = %d\n", var);
    printf("var2 = %d\n", var2);

    return 0;
} */
