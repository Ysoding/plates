# plates

A tiny  lib to generate human-readable hash from a string in the style of license plates.

## Use
```rust

let a = plates::PlateCode::encode("hello world123", true);
println!("{}", a); // 🍔 藏G·NBSWY 🦘 鲁H·3DPEB 🛗 沪B·3W64T 💣 粤M·MMQYT 🍨 津K·EMYAA ✂️
println!("{}", plates::PlateCode::decode(&a)); // hello world123

let b = plates::PlateCode::encode("hello world456", false);
println!("{}", b); // 苏Y·NBSWY 琼Z·3DPEB 湘B·3W64T 甘B·MMQ2D 桂V·KNQAA
println!("{}", plates::PlateCode::decode(&b)); // hello world456

println!("{}", plates::PlateCode::hash("test", true)); // 🧼 鲁E·L75AL ⚰️
println!("{}", plates::PlateCode::hash("test", false)); // 鲁E·L75AL
```


## Credit

forked from Project [Generate readable hash or encoding in the style of license plates 📦 鄂A·22391](https://github.com/Leizhenpeng/platecode)
