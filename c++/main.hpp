#ifndef MG_SQR_CPP_MAIN
#define MG_SQR_CPP_MAIN

#include <functional>

#define SIZE 3
#define MIN 1
#define MAX 1000

typedef unsigned int square_int;
typedef std::function<square_int (square_int, square_int)> generator;

inline int cpu_count();
void signalHandler(int signum);
void findSquare();
void populate(square_int (&square)[SIZE][SIZE], generator f);
square_int checkGenerate(square_int (&square)[SIZE][SIZE], generator f, int x, int y);
bool contains(square_int (&square)[SIZE][SIZE], square_int value);
void powerOf2(square_int (&square)[SIZE][SIZE]);
std::string to_string(square_int (&square)[SIZE][SIZE]);
inline bool equal(square_int (&a)[2], square_int (&b)[2]);
bool is_magic(square_int (&square)[SIZE][SIZE]);

#endif
