
.PHONY: libfpc.a

libfpc.a: src/fpc.cpp
	rm -f libfpc.a fpc.o
	gcc -O2 -c src/fpc.cpp
	ar -rsc libfpc.a fpc.o
	rm -f fpc.o
	touch src/fp.rs

test: libfpc.a
	RUSTFLAGS="-L `pwd`" cargo test

bench: libfpc.a
	RUSTFLAGS="-L `pwd`" cargo bench -- Fp_m

