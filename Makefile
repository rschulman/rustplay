LD=i686-elf-ld
RUSTC=rustc
NASM=nasm
CLANG=clang

all: main.bin

.SUFFIXES: .o .bc .rs .asm

.rs.bc:
	$(RUSTC) -O --target i686-intel-linux -Z no-landing-pads -o $@ --emit=bc $<

.bc.o:
	$(CLANG) -target x86_64-intel-linux -ffreestanding -c $<

.asm.o:
	$(NASM) -f elf $< -o $@

main.bin: linker.ld boot.o main.o
	$(LD) -o $@ -T $^
