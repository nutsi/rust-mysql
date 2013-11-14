RUSTC=rustc

dummy1 := $(shell mkdir build/ build/lib build/examples 2> /dev/null)

all: lib

lib:
	$(RUSTC) -o build/lib/mysql src/mysql.rc --lib

examples:
	$(RUSTC) -Lbuild/lib -o build/examples/first_example example/first_example.rs

clean:
	rm -rf build

