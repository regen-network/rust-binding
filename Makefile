.PHONY: python

python:
	mkdir -p python
	cp target/release/librust_binding.so python/librust_binding.so
	cp target/rust_binding.h python/rust_binding.h
	cp rust_binding.i python/
	swig -python -outdir python python/rust_binding.i
	gcc -fPIC -Wno-register -Wall -c -o python/rust_binding_wrap.o python/rust_binding_wrap.c -Ipython/ `python2-config --includes`
	gcc -Wno-deprecated-register -shared -o python/_rust_binding.so python/rust_binding_wrap.o -Lpython/ -lrust_binding -lpython
