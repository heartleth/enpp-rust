@echo off
cargo build --release > NUL
copy .\target\release\engpp.exe C:\engpp > NUL
engpp main
g++ -o out -std=c++20 main.cpp
.\out