
build: 
	@cargo build --release

clean: 
	@rm -rf players/bin/*
	
asm:
	@./target/release/assembler $(ARGS)
run:
	@./target/release/vm $(ARGS)
disasm:
	@./target/release/disassemler $(ARGS)