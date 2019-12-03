.PHONY: python python-old

DLL_EXT = ""
ifeq ($(OS),Windows_NT)
	DLL_EXT = dll
else
	UNAME_S := $(shell uname -s)
	ifeq ($(UNAME_S),Linux)
		DLL_EXT = so
	endif
	ifeq ($(UNAME_S),Darwin)
		DLL_EXT = dylib
	endif
endif

install-rust:
	cargo build --release
	# install it as a system package
	sudo cp target/release/librust_binding.$(DLL_EXT) /usr/local/lib
	sudo ldconfig

swig-python:
	mkdir -p bind/python
	cp deps/* bind/python
	swig -python -outdir bind/python bind/python/rust_binding.i

python: swig-python
	cd bind/python && python setup.py build_ext --inplace
	@ echo -e "\nRunning example script:\n"
	cd bind/python && python example.py
# 	cd bind/python && python setup.py install
