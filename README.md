# Building cross-language bindings

Links: 
* [Simple Python Tutorial](https://blog.jonharrington.org/rust-and-swig/)
* [Install Swig](http://www.swig.org/Doc4.0/Preface.html#Preface_unix_installation)
    * [Download Tarball](https://sourceforge.net/projects/swig/files/swig/swig-4.0.1/swig-4.0.1.tar.gz/download)
* [Swig Docs](http://swig.org/Doc4.0/Sections.html#Sections)
  * [Python](http://swig.org/Doc4.0/Python.html#Python)

## Organization

This is an attempt to pull out the ffi stuff into a sub-crate, and wasm into another.
Note the `cbindgen.toml` file in `lib/rust_ffi`. The `parse_deps = true` directive 
is essential to be able to expose headers for re-exported structs.

Nice example to refer to for this organization approach: https://github.com/Ortham/esplugin/tree/master/ffi

## Install Swig

First, [download the tarball](https://sourceforge.net/projects/swig/files/swig/swig-4.0.1/swig-4.0.1.tar.gz/download).

Then setup dependencies (Ubuntu):

```shell script
sudo apt-get install build-essential libpcre3-dev
tar xzvf swig-4.0.1.tar.gz
cd swig-4.0.1
./configure
make
```

For MacOS, run the following in place of `sudo apt-get` line:

```shell script
brew install git automake pcre boost
```

Note that this showed the following on my system:

```
The SWIG test-suite and examples are configured for the following languages:
go perl5 python 
```

It seems I need to install more packages to enable other languages. There is a list of how to [enable
more languages here](https://github.com/swig/swig/wiki/Getting-Started#linux---ubuntu). Here I will
just add Java, PHP, and Ruby support for now, as well as Python 3:

```shell script
sudo apt-get install -y python3-dev 2to3 # Python 3
sudo apt-get install -y default-jdk-headless # Java
sudo apt-get install -y php-cli php-dev
sudo apt-get install -y ruby ruby-dev
```

Now, when I try `./configure`, I see more options (`go java perl5 php python ruby`) Let's rebuild with all enabled:

```shell script
./configure
make
make install
```

## Build the Rust Library

```shell script
cargo build --release
ls -l target/rust_binding.h 
ls -l target/release/lib*
```

If you are surprised by the size (2MB) of `librust_binding.so`, you can 
run `strip target/release/librust_binding.so` (on Linux) and trim it down to 10KB,
which is all that is really there.

## Build python bindings

Running `make python` will compile the proper bindings. However, there seems to be some issue with the linkage.
My current workaroud is:

```shell script
cd python
LD_LIBRARY_PATH=`pwd` python
>>> import rust_binding
>>> rust_binding.sum(2, 4)
6
```

This is a [known problem](https://stackoverflow.com/questions/36827871/cython-unable-to-find-shared-object-file) 
and either setting `LD_LIBARY_PATH` or trying  [setup.py install](https://github.com/gipit/gippy/issues/123)
seem to be the only options. Let's try the second.

```shell script
virtualenv ~/foo
source ~/foo/bin/activate
cd bind/python
python setup.py install
```

Same issue :(

Current approach is to copy the *.so file to `/usr/local/lib` and call `ldconfig`. Basically, 
installing our rust library on the system. Which I guess is okay? After that all the swig bindings just work
as they can find our new "system" library. This is the same approach as libpng or libgzip, or many others.

Now this works:

```shell script
make install-rust
make python
```

## Handling Structs

It is not hard to expose structs over swig, just preface them with

```rust
#[repr(C)]
#[derive(Copy,Clone)]
pub struct Foo { // ...
```

You can now access the struct in the calling library and work with it. You can use it as an argument or return value for 
functions as well. However... if you implement methods on it, they will not be exposed... 
not even if you ask cbindgen to export "C++" style.

## Handling ints

When exposing any rust int types (i32, u64, etc), swig was happy, but python would complain
about unknown types. The first approach was to cast the `extern "C"` functions so they only
expose `libc::int_t`. However, once we exposed structs, the accessors would get `i32` and
complain hard when trying to access the struct directly. 

Adding `%include "stdint.i"` to the top of `rust_bindings.i` will allow swig to expose all the
various rust int types in one standard type that the python runtime can interact with. I assume
this will also work with other languages :cross_fingers:

## Exposing C++ style methods

[Swig does have nice support for C++ methods](http://swig.org/Doc4.0/Python.html#Python_nn20).
However, that requires you to actually have C++ methods. After spending some time trying
to get `cbindgen` to expose methods, I dug in deeper to see who had managed to expose
C++ method type interfaces from rust, such that they can be used by a caller.

I did find an article of someone who [demonstrates how to expose proper C++ types from rust](https://karroffel.gitlab.io/post/2019-05-15-rust/).
It is a quite interesting an advanced technique, but in the end an utter headache on the FFI front in comparison to "C" exports
(which are super painless). Here is an example of the code (from the above link):

```rust
// src/ffi/mod.rs
use cpp::cpp;

use crate::Adder;

cpp!{{

Adder::Adder() {
    this->internal =
        rust!(Adder_constructor [] -> *mut Adder as "void *" {
            let b = Box::new(Adder::default());
            Box::into_raw(b)
        });
}

Adder::~Adder() {
    rust!(Adder_destructor [internal: *mut Adder as "void *"] {
        let _b = unsafe {
            Box::from_raw(internal)
        };
    });
}

void Adder::add(int64_t value) {
    rust!(Adder_add [
        internal: &mut Adder as "void *",
        value: i64 as "int64_t
    ] {
        internal.add(value);
    });
}

int64_t Adder::tell() const {
    return rust!(Adder_tell [
        internal: &mut Adder as "void *"
    ] -> i64 as "int64_t" {
        internal.tell()
    });
}

}}
```

Given that, I will stick with the standard C api and pass around structs and functions that manipulate them.

## Passing function pointers

There was also discussion on how to pass function pointer from the target language (eg. python/go)
into the rust library. There is a very nice stack overflow answer about passing one function
*defined in the rust lib* to another function *also defined in the C header*.

https://stackoverflow.com/questions/22923696/how-to-wrap-a-c-function-which-takes-in-a-function-pointer-in-python-using-swi

Note it is about returning using one function defined in C as an argument to another function defined in C, which ends up
requiring a few swig directives to get it to work. No talk of passing a pure 
python function into the C code.

## TODO

* Try javascript
* Organize project
* Try Java