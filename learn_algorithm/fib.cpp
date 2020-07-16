#include <iostream>
#include <utility>

using namespace std;

pair<uint64_t, uint64_t> Fib(size_t n)
{
    // return f_{n},  f_{n + 1}, 
    if (n > 0){

        auto PF = Fib(n / 2);
        auto t0 = PF.first;
        auto t1 = PF.first;
        if(n % 2 == 1)
            return {t0 * t0 + t1 * t1, (2 * t0 + t1) * t1};
        else
            return {(2 * t1 - t0) * t0, t0 * t0  + t1 * t1};
    }
    return {0, 1};// return f_0, f_1
}

int main()
{
    size_t n = 10;
    auto ret = Fib(n);
    cout << "fib(" << n << ")" << ret.first  << endl;
    return 0;
}

