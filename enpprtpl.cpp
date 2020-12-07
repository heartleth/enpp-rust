#include "engppstd.hpp"
int main(){(each(std::vector<int>({64, 65, 66, 67, 68, 69, 70, }), ([&](auto...pp){return(println(pp...));})));}