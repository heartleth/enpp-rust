#include "engppstd.hpp"
int main(){auto  print_10 = [&](){return(print(10));};
;print_10();}