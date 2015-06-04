#include <memory>

using namespace std;

#define unused __attribute ((unused))
int main(unused int argc, unused char **argv)
// BEGIN_CODE
{
    // memory freed when ptr_a goes out of scope
    unique_ptr<char> ptr_a(new char('a'));

    // reference counted pointer
    shared_ptr<char> ptr_b(new char('b'));

    // pointer that does not increase reference counts
    weak_ptr<char> ptr_c = ptr_b;

    // memory freed when ptr_d goes out of scope
    // differs from unique_ptr in copy semantics
    auto_ptr<char> ptr_d(new char('d'));
}
// END_CODE

