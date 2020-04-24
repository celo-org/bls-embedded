AR=ar
CC=gcc
CXX=g++
CPPFLAGS=-march=armv7-m -mcpu=cortex-m3 -mthumb -O3 -funroll-all-loops -fconserve-stack
#CPPFLAGS=-march=native -O2 -funroll-all-loops

all: libfpc.a
	touch src/fp.rs

libfpc.a: libfpc.a(fpc.o fp_mont.o fp_asm.o fq_asm.o)

%.o: src/%.cpp
	$(CXX) $(CPPFLAGS) $(CXXFLAGS) -c $< -o $@

%.o: src/%.c
	$(CC) $(CPPFLAGS) $(CFLAGS) -c $< -o $@

%.o: src/%.s
	$(CC) $(CPPFLAGS) $(CFLAGS) -c $< -o $@

%.o: src/%.S
	$(CC) $(CPPFLAGS) $(CFLAGS) -c $< -o $@

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
