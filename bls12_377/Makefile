
.PHONY: libfpc.a

libfpc.a: src/fpc.c
	rm -f libfpc.a fpc.o
	gcc -march=native -O2 -c src/fpc.c
	ar -rsc libfpc.a fpc.o
	rm -f fpc.o
	touch src/fp.rs

test: libfpc.a
	RUSTFLAGS="-L `pwd`" cargo test

bench: libfpc.a
	RUSTFLAGS="-L `pwd`" cargo bench -- Fp_m

both: libfpc.a
	RUSTFLAGS="-L `pwd`" cargo test
	RUSTFLAGS="-L `pwd`" cargo bench -- Fp_m

rsync:
	rsync -ar --progress --exclude=target --exclude='*.a' . tinkerboard:bls-embedded/bls12_377
