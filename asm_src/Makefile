# Compilers
CC = gcc
FLAGS = -g -z execstack -fno-stack-protector

# Files
BIN = output.bin
SOURCE = embedded
SHELLCODE = shellcode

all: $(BIN)

.PHONY: $(SOURCE) $(BIN) $(SHELLCODE) clean

$(SOURCE):
	$(CC) $(FLAGS) -o $(SOURCE) $(SOURCE).c

$(BIN): $(SOURCE)
	./$(SOURCE)

$(SHELLCODE):
	$(CC) $(FLAGS) -o $(SHELLCODE) $(SHELLCODE).c
	./$(SHELLCODE)

clean:
	$(RM) $(SOURCE) $(BIN) $(SHELLCODE)
