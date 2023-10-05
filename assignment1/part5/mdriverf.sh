KRUSTFLAGS="--emit metadata" make KBUILD_MODPOST_WARN=1 CONFIG_SAMPLE_RUST_MTEST=n SUBDIRS=./samples/rust/ modules -j16 
mv ~/cpsc-429-Principles-of-System-Design/linux-cs429-fall-2023/librust_mdriverf.rmeta ~/cpsc-429-Principles-of-System-Design/linux-cs429-fall-2023/samples/rust/librust_mdriverf.rmeta
