#include <limits.h>

typedef struct Something
{
    char first;
    unsigned long long second;
} Something;

Something build_something()
{
    // same effect
    // struct Something s = {
    //     .first = 1,
    //     .second = ULONG_MAX,
    // };

    struct Something s;

    s.first = 1;
    s.second = ULONG_MAX;

    return s;
}
