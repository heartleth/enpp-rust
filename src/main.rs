pub mod transpile;
pub use transpile::*;
pub use parser::*;

//                      /\                 ____
//                   /    |               /   |
//                 /      |            /      |
//                |       /_______   /        |
//             ___\    __/    -   `-^-,      /__-___
//           -     .---                \ ___/       `--_
//         /      //        /V\\         \              \
//       /       //    \ \/ \  /     \    |              |
//       |       |//  \ /`\_     _/   /  /               |
//       |         `--_=\/ \    __-_,, _| \             /
//       \           / |\\`/|  `   \ /    |           /
//         \         | \\ ``:_.    _-```  /        _/
//           -_       -__---___--``     -<_____--``
//              `-------=`__-|  |-_____
//                 __-=```_ ^\``/     /``\
//                /   |  |  \ `//    /     \
//              /     |      | v   //\      \
//            /      | |  |  | /  /   \      \
//         _-      /``===_=__=____=____ \     \
// =-- _ __-     /     |____________\    \     \
//  ```=___---/-`     /  /     |   |  \   \   __-
//                  /    /     |    \   \  \_-.  `---=
//                /     /      |     \    \    \_<---=`
//              /       /      |      \     \
//            /        |       ^      _\___--\
//            \\___   |=========`````` __---``
//                 `````-----------````

pub fn main() {
    println!("처리중...");
    let v = &String::from("stoi: input + stoi: input");
    println!("처리끝ㅇㅇ : {}", parser::value_parse(&v, 0));
}