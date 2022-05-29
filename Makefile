# Build
CARGO = cargo b
RELEASE = release
FLAGS = --$(RELEASE)
BUILD = target
BIN = aibohphobia

# Debugging
OBJDUMP = objdump
KEYFLAGS = -s -j .nsp0
NONCEFLAGS = -s -j .nsp1
FIRSTFLAGS = -s -j .gnu.version
FUNCFLAGS = -s -j .ctors

.PHONY: $(BIN) debug run

all: $(BIN)

$(BIN):
	$(CARGO) $(FLAGS)

run:
	$(BUILD)/$(RELEASE)/$(BIN)

debug:
	$(OBJDUMP) $(KEYFLAGS) $(BUILD)/$(RELEASE)/$(BIN)
	$(OBJDUMP) $(NONCEFLAGS) $(BUILD)/$(RELEASE)/$(BIN)
	$(OBJDUMP) $(FIRSTFLAGS) $(BUILD)/$(RELEASE)/$(BIN)
	$(OBJDUMP) $(FUNCFLAGS) $(BUILD)/$(RELEASE)/$(BIN)

clean:
	$(RM) -r $(BUILD)
