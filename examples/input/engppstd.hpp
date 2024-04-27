
#ifndef _ENPPSTD_
#define _ENPPSTD_
#include <algorithm>
#include <iostream>
#include <fstream>
#include <vector>
#include <string>
#include <thread>
#include <chrono>
#include <regex>
#include <tuple>
#include <map>
#ifdef __cpp_lib_ranges
#include <ranges>
#endif
#define jthread(t) std::thread((t)).join()
#define dthread(t) std::thread((t)).detach()
typedef char i1;typedef short i2;typedef long i4;typedef long long i8;
typedef unsigned char u1;typedef unsigned short u2;typedef unsigned long u4;typedef unsigned long long u8;
typedef float f4;typedef double f8;typedef long double ld;
typedef const char ci1;typedef const unsigned char cu1;typedef const short ci2;typedef const long ci4;typedef const long long ci8;
typedef const unsigned short cu2;typedef const unsigned long cu4;typedef const unsigned long long cu8;
typedef const float cf4;typedef const double cf8;typedef const long double cld;
using std::vector;using std::string;using std::stoi;
template<class F, class...T>void get_time(F f, T...a) { auto st = std::chrono::system_clock::now(); f(a...); std::chrono::duration<double>t = std::chrono::system_clock::now() - st; std::cout << t.count() << " second(s) spent." << std::endl; }
template<class...T>void println(T...arg) { (std::cout << ... << arg); std::cout << std::endl; }
template<class...T>void print(T...arg) { (std::cout << ... << arg); }
std::string input_line(std::string a = "") { std::string b; std::cout << a; getline(std::cin, b); return b; }
std::string input(std::string a = "") { std::string b; std::cout << a; std::cin >> b; return b; }
std::string static_input(int etag, std::string a = "") { static std::map<int, std::string>memoi; if (memoi.count(etag)) { return memoi[etag]; } std::string b; std::cout << a; std::cin >> b; memoi.insert(std::make_pair(etag, b)); return b; }
std::string static_input_line(int etag, std::string a = "") { static std::map<int, std::string>memoi; if (memoi.count(etag)) { return memoi[etag]; } std::string b; std::cout << a; getline(std::cin, b); memoi.insert(std::make_pair(etag, b)); return b; }
template<class...T>auto tup(T...arg)->std::tuple<T...> { return std::tuple<T...>(arg...); }
template<class T>class __folder {
public:T c; template<class E>__folder& operator<< (E a) { c.push_back(a); return*this; }};
template<class T, class...R>class __gft { public:typedef T CORE; };
template<class...T>std::vector<typename __gft<T...>::CORE> vec(T...arg) { __folder<std::vector<typename __gft<T...>::CORE>> r; return (r << ... << arg).c; }
template <class T>std::string to_string(T a) { std::stringstream k; k << a; return k.str(); }
template<class T, class F>T map(T c, F f) { T g(c); std::transform(c.begin(), c.end(), g.begin(), f); return g; }
template<class T, class F>void each(T c, F f) { std::for_each(c.begin(), c.end(), f); }
std::vector<i4>until(i4 b, i4 e) { std::vector<i4>v; for (int i = b; i <= e; i++)v.push_back(i); return v; }
#ifdef __cpp_lib_ranges
namespace srv = std::ranges::views;
namespace sr = std::ranges;
using namespace srv;
using namespace sr;
#endif
#endif
