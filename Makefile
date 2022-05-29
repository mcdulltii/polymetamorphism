# Build
CARGO = cargo b
RELEASE = release
FLAGS = --$(RELEASE)
BUILD = target
BIN = aibohphobia

# Debugging
OBJDUMP = objdump
KEYFLAGS = -s -j .unixb
FUNCFLAGS = -s -j .reloc

.PHONY: $(BIN) debug run

all: $(BIN)

$(BIN):
	$(CARGO) $(FLAGS)

run: debug
	$(BUILD)/$(RELEASE)/$(BIN)

debug:
	$(OBJDUMP) $(KEYFLAGS) $(BUILD)/$(RELEASE)/$(BIN)
	$(OBJDUMP) $(FUNCFLAGS) $(BUILD)/$(RELEASE)/$(BIN)

clean:
	$(RM) -r $(BUILD)
