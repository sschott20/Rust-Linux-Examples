KRUSTFLAGS="--extern rust_mdriverf=./samples/rust/librust_mdriverf.rmeta" make KBUILD_MODPOST_WARN=1 SUBDIRS=./samples/rust/ modules -j16
