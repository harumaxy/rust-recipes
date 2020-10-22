all: target/debug/libdouble.so
	gcc src/main.c -L ./target/debug/ -ldouble -o doublerun
	LD_library_path = ./target/debug/ ./doublerun


target/debug/libdouble.so: src/lib.rs Cargo.toml
	cargo build

clean:
	rm -rf target
	rm -rf doublerun