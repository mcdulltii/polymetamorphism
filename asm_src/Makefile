SOURCE = embedded

all: assemble

.PHONY: assemble clean

assemble:
	nasm -f elf64 -o $(SOURCE).o $(SOURCE).nasm 
	ld -dynamic-linker /lib64/ld-linux-x86-64.so.2 -o $(SOURCE) -lc $(SOURCE).o

clean:
	$(RM) $(SOURCE) $(SOURCE).o
