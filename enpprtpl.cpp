#include "engppstd.hpp"
int main(){(async([&](auto...pp){return(println(20, pp...));})).wait();}