PY = python3
CC = gcc
BIN = morbius_extract
FLAGS = -g -z execstack -fno-stack-protector
SHELLCODE = shellcode
DUMP = dump
XPL = xpl.py

all: $(SHELLCODE)

.PHONY: $(DUMP) $(SHELLCODE) run xpl clean

$(DUMP):
	objcopy $(BIN) /dev/null --dump-section .hash=$(DUMP)

$(SHELLCODE):
	$(CC) $(FLAGS) -o $(SHELLCODE) $(SHELLCODE).c

run:
	./$(SHELLCODE)

xpl:
	$(PY) $(XPL)

clean:
	$(RM) $(SHELLCODE)
