#include "engppstd.hpp"
int main(){(each((integrate(std::vector<int>({1, 3, 5, 7, 9, }), ([&](auto  a, auto  b){return(((a + b)));}))), [&](auto...pp){return(println(pp...));}));}