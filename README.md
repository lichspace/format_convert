png转tga， 或者tga转png

### 打包
cargo build --release


```rust
// 白色作为透明叠加
let mut bottom = image::open(&src).unwrap().into_rgba8();
let mut top = image::open(&over).unwrap().into_rgba8();
println!("{:?} {:?}", bottom.dimensions(), top.dimensions());

let start = Instant::now();

for p in top.pixels_mut() {
    if p[0] == 255 && p[1] == 255 && p[2] == 255 {
        p[3] = 0;
    }
}

image::imageops::overlay(&mut bottom, &top, 0, 0);
bottom.save(dist).unwrap();

let duration = start.elapsed();
println!("convert time: {:?}", duration);
```