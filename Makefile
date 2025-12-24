
build: 
	@cargo build --release --features gui 

clean: 
	@rm -rf players/bin/*

run: 
	@./target/release/core $(ARGS)


asm:
	@./target/release/assembler $(ARGS)
vm:
	@./target/release/vm $(ARGS)
disasm:
	@./target/release/disassemler $(ARGS)