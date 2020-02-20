
.PHONY: libfpc.a

libfpc.a: src/fpc.cpp
	rm -f libfpc.a fpc.o
	gcc -march=armv7-m -mcpu=cortex-m3 -mthumb -O2 \
	 -funroll-all-loops \
	 -c src/fpc.cpp
#	gcc -march=native -O2 \
		-funroll-all-loops \
		-c src/fpc.cpp
	ar -rsc libfpc.a fpc.o
	rm -f fpc.o
	touch src/fp.rs

test: libfpc.a
	RUSTFLAGS="-L `pwd`" cargo test

bench: libfpc.a
	RUSTFLAGS="-L `pwd`" cargo bench -- Fp_m

bench2: libfpc.a
	RUSTFLAGS="-L `pwd`" cargo bench -- G2Projective_s

bench1: libfpc.a
	RUSTFLAGS="-L `pwd`" cargo bench -- G1Projective_s



rsync:
	rsync -ar --progress --exclude=target --exclude='*.a' . tinkerboard:bls-embedded/bls12_377
