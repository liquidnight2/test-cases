


### Cross-Compile zu windows
> apt-get install	binutils-mingw-w64-x86-64
>
> apt-get install	g++-mingw-w64-x86_64



### Add to Cargo.toml:
```
[target.x86_64-pc-windows-gnu]
linker = "/usr/bin/x86_64-w64-mingw32-gcc"
ar = "/usr/bin/x86_64-w64-mingw32-ar"
```

> rustup target add x86_64-pc-windows-gnu


> cargo build --target x86_64-pc-windows-gnu

> wine   target/x86_64-pc-windows-gnu/debug/hellocross.exe
