UNAME := $(shell uname)

ifeq ($(UNAME), Linux)
ARCH := elf64
endif
ifeq ($(UNAME), Darwin)
ARCH := macho64
endif

tests/%.s: tests/%.snek src/main.rs
	cargo run -- $< tests/$*.s

tests/%.run: tests/%.s runtime/start.rs
	nasm -f $(ARCH) tests/$*.s -o tests/$*.o
	ar rcs tests/lib$*.a tests/$*.o
	# needed for macos only
	rustup target add x86_64-apple-darwin 
	rustc -L tests/ -lour_code:$* runtime/start.rs --target x86_64-apple-darwin -o tests/$*.run

	# needed for linux / windows
	# rustc -L tests/ -lour_code:$* runtime/start.rs -o tests/$*.run

.PHONY: test
test:
	cargo build
	cargo test

clean:
	rm -f tests/*.a tests/*.s tests/*.run tests/*.o

.PRECIOUS: tests/%.s