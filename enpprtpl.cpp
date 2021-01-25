#include "engppstd.hpp"
int main(){auto  sum = ([&](auto  e){return((fold(e, ([&](auto  a, auto  b){return(((a + b)));}))));});
;println(sum(vec(1, 3, 5, 7)));}