.PHONY: python python-old

rust:
	cargo build --release
	cp target/release/librust_binding.so deps

swig-python: rust
	mkdir -p bind/python
	cp deps/* bind/python
	swig -python -outdir bind/python bind/python/rust_binding.i

python:

python-old:
	gcc -fPIC -Wno-register -Wall -c -o python/rust_binding_wrap.o python/rust_binding_wrap.c -Ipython/ `python2-config --includes`
	gcc -Wno-deprecated-register -shared -o python/_rust_binding.so python/rust_binding_wrap.o -Lpython/ -lrust_binding -lpython
