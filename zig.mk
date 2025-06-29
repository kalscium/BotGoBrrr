# Makefile for zig files
ZIG:=zig
ZIG_BIN_DIR:=zig-out/bin

ELF_DEPS+=$(BINDIR)/userlib.zig.o

$(BINDIR)/userlib.zig.o: build.zig $(SRCDIR)/*.zig
	-$(VV)mkdir -p $(dir $@)
	$(call test_output_2,Compiled ZIG userlib ,$(ZIG) build -Dtoggle-arcade -Dlog-velocity -Dbenchmark -Doptimize=ReleaseSmall obj,$(OK_STRING))
	$(VV)mv $(ZIG_BIN_DIR)/userlib.zig.o $@
