# Build
CARGO = cargo b
RELEASE = release
FLAGS = --$(RELEASE)
DIST = dist
BUILD = target
BIN = morbius

# Assembly build
ASM = asm_src

# Debugging
OBJDUMP = objdump
KEYFLAGS = -s -j .nsp0
NONCEFLAGS = -s -j .nsp1
FIRSTFLAGS = -s -j .lbss
FUNCFLAGS = -s -j .hash

.PHONY: $(BIN) debug run release clean

all: $(BIN)

$(BIN):
	$(CARGO) $(FLAGS)

run:
	$(BUILD)/$(RELEASE)/$(BIN)

release:
	$(DIST)/$(BIN)

debug:
	$(OBJDUMP) $(KEYFLAGS) $(BUILD)/$(RELEASE)/$(BIN)
	$(OBJDUMP) $(NONCEFLAGS) $(BUILD)/$(RELEASE)/$(BIN)
	$(OBJDUMP) $(FIRSTFLAGS) $(BUILD)/$(RELEASE)/$(BIN)
	$(OBJDUMP) $(FUNCFLAGS) $(BUILD)/$(RELEASE)/$(BIN)

clean:
	$(RM) -r $(BUILD)
	make -C $(ASM) clean
