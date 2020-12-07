pub static STDLIB :&[u8] = b"
#ifndef _ENPPSTD_
#define _ENPPSTD_
#include <type_traits>
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
typedef char i1; typedef short i2; typedef long i4; typedef long long i8;
typedef unsigned char u1; typedef unsigned short u2; typedef unsigned long u4; typedef unsigned long long u8;
typedef float f4; typedef double f8; typedef long double ld;
typedef const char ci1; typedef const unsigned char cu1; typedef const short ci2; typedef const long ci4; typedef const long long ci8;
typedef const unsigned short cu2; typedef const unsigned long cu4; typedef const unsigned long long cu8;
typedef const float cf4; typedef const double cf8; typedef const long double cld;
using std::vector; using std::string; using std::stoi; using namespace std::string_literals;
template<class F, class...T>void get_time(F f, T...a) {
auto st = std::chrono::system_clock::now(); f(a...); std::chrono::duration<double>t = std::chrono::system_clock::now() - st; std::cout << t.count() << \" second(s) spent.\" << std::endl;}
std::string input_line(std::string a = \"\") { std::string b; std::cout << a; getline(std::cin, b); return b; }
std::string input(std::string a = \"\") { std::string b; std::cout << a; std::cin >> b; return b; }
std::string static_input(int etag, std::string a = \"\") { static std::map<int, std::string>memoi; if (memoi.count(etag)) { return memoi[etag]; } std::string b; std::cout << a; std::cin >> b; memoi.insert(std::make_pair(etag, b)); return b; }
std::string static_input_line(int etag, std::string a = \"\") { static std::map<int, std::string>memoi; if (memoi.count(etag)) { return memoi[etag]; } std::string b; std::cout << a; getline(std::cin, b); memoi.insert(std::make_pair(etag, b)); return b; }
template<class...T>auto tup(T...arg)->std::tuple<T...> { return std::tuple<T...>(arg...); }
template<class T>class __folder {
public:T c; template<class E>__folder& operator<< (E a) { c.push_back(a); return*this; }};
template<class...T>void print(const T&...arg) { (std::cout << ... << arg); }
template<class...T>void println(const T&...arg) { (std::cout << ... << arg); std::cout << std::endl; }
template<class T, class...R>class __gft { public:typedef T CORE; };
template<class...T>std::vector<typename __gft<T...>::CORE> vec(T...arg) { __folder<std::vector<typename __gft<T...>::CORE>> r; return (r << ... << arg).c; }
template<class T>std::string to_string(T a) { std::stringstream k; k << a; return k.str(); }
template<class T, class F>auto map(T c, F f)->std::vector<typename T::value_type> { std::vector<typename T::value_type>g(c.begin(), c.end()); for (auto& i : g) { i = f(i); }return g; }
template<class T, class F>void each(T c, F f) { std::for_each(c.begin(), c.end(), f); }
template<class T, class F>auto filter(T c, F f)->std::vector<typename T::value_type> { std::vector<typename T::value_type>a; for (const auto& i : c)if (f(i))a.push_back(i); return a; }
template<typename T, typename F>auto integrate(T c, F f)->std::vector<decltype(f(*c.begin(), *c.begin()))> { typedef typename T::value_type vt; auto iter = c.begin(); vt rdc = *iter; std::vector<vt>ret;ret.push_back(rdc); iter++;
for (; iter != c.end(); iter++) { rdc = f(rdc, *iter); ret.push_back(rdc); }return ret; }
template<typename T, typename F, typename vt = typename T::value_type>auto fold(const T & c, const F & f)->decltype(f(*c.begin(), *c.begin())) { auto iter = c.begin(); vt rdc = *iter; iter++; std::vector<vt>ret; for (; iter != c.end(); iter++) { rdc = f(rdc, *iter); }return rdc; }
template<typename T, typename F, typename vt>auto bfold(const vt & d, const T & c, const F & f)->decltype(f(vt(), *c.begin())) { auto iter = c.begin(); vt rdc = d; std::vector<vt>ret; for (; iter != c.end(); iter++) { rdc = f(rdc, *iter); }return rdc; }
template<class T1, class T2>std::vector<typename T1::value_type> cat(T1 a, T2 b) { std::vector<typename T1::value_type> ret(a.begin(), a.end()); ret.insert(ret.end(), b.begin(), b.end()); return ret; }
class range {private:int start; int End; int diff; public:typedef int value_type;typedef const int& const_reference;typedef int&reference;
range(int _end) { start = 0; End = _end; diff = 1; }
range(int _start, int _end, int _diff = 1) { start = _start; End = _end; diff = _diff; }
class iterator {
private:int _diff; public:int _val; iterator(int v, int d) :_val(v), _diff(d) {}
auto operator++()->iterator& { _val += _diff; return *this; }
inline int operator*() { return _val; }
int operator==(iterator i) { return (i._val == _val); }
int operator!=(iterator i) { return (i._val >= _val + _diff); }};
inline auto begin()->iterator { return iterator(start, diff); }
inline auto end()->iterator { return iterator(End + diff, diff); }};
inline range until(i4 a, i4 b) { return range(a, b); }
#ifdef __cpp_lib_ranges
namespace srv = std::ranges::views;
namespace sr = std::ranges;
using namespace srv;
using namespace sr;
#endif
#endif
";