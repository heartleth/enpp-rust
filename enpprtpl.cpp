#include "engppstd.hpp"
int main(){println((fold(vec(1, 2, 3, 4, 5), ([&](auto  ...a){return(((... + a)));}))));}