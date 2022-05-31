CC = gcc
FLAGS = -g -z execstack -fno-stack-protector
BIN = output.bin
SOURCE = embedded
SHELLCODE = shellcode

all: $(BIN)

.PHONY: $(SOURCE) $(BIN) $(SHELLCODE) assemble clean

$(SOURCE):
	$(CC) $(FLAGS) -o $(SOURCE) $(SOURCE).c

$(BIN): $(SOURCE)
	./$(SOURCE)

$(SHELLCODE):
	$(CC) $(FLAGS) -o $(SHELLCODE) $(SHELLCODE).c
	./$(SHELLCODE)

assemble:
	nasm -f elf64 -o $(SOURCE).o $(SOURCE).nasm 
	ld -dynamic-linker /lib64/ld-linux-x86-64.so.2 -o $(SOURCE) -lc $(SOURCE).o

clean:
	$(RM) $(SOURCE) $(SOURCE).o $(BIN) $(SHELLCODE)
