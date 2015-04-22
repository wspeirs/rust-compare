#define unused __attribute ((unused))

int main(unused int argc, unused char **argv) {
    // unsigned values
    unsigned char a;
    unsigned short b;
    unsigned int c;
    unsigned long d;
    unsigned long long e;

    // Boolean values
    bool f;

    // signed values
    char g;
    short h;
    int i;
    long j;
    long long k;

    // floating point values
    float l;
    double m;

    return 0;
}
