#include "engppstd.hpp"
int main(){(each(vec(1, 2, 3), ([&](auto  e){return(print(e));})));}