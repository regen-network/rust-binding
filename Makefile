.PHONY: python python-old

# detect os and dll extension
DLL_EXT = ""
SYS = ""
ifeq ($(OS),Windows_NT)
	DLL_EXT = dll
	SYS = windows
else
	UNAME_S := $(shell uname -s)
	ifeq ($(UNAME_S),Linux)
		DLL_EXT = so
		SYS = linux
	endif
	ifeq ($(UNAME_S),Darwin)
		DLL_EXT = dylib
		SYS = macos
	endif
endif

ifeq ($(SYS),linux)

install-rust:
	cargo build --release
	sudo cp target/release/librust_binding.$(DLL_EXT) /usr/local/lib
	sudo ldconfig

else ifeq ($(SYS),macos)

install-rust:
	cargo build --release
	sudo cp target/release/librust_binding.$(DLL_EXT) /usr/local/lib

else

install-rust:
	echo "install-rust not implemented for $SYS"
	exit 1

endif

ensure-%:
	mkdir -p bind/$*
	cp deps/* bind/$*

python: ensure-python
	swig -python -outdir bind/python bind/python/rust_binding.i
	cd bind/python && python setup.py build_ext --inplace
# 	cd bind/python && python setup.py install
	@ echo -e "\nRunning example script:\n"
	cd bind/python && python example.py

python3: ensure-python3
	swig -python -outdir bind/python3 bind/python3/rust_binding.i
	cd bind/python3 && python3 setup.py build_ext --inplace && python3 example.py
