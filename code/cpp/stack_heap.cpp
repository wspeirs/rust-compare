#define unused __attribute ((unused))
int main(unused int argc, unused char **argv)
// BEGIN_CODE
{
    // single char allocated on the stack
    char a = 'a';

    // single char allocated on the heap
    // char pointer allocated on the stack
    char *b = new char('b');

    // array of 23 chars allocated on the heap
    // char pointer allocated on the stack
    char *c = new char[23];


    // heap allocated char is freed
    delete b;

    // heap allocated array is freed
    delete[] c;
}
// END_CODE

