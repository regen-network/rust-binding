# Building cross-language bindings

Links: 
* [Simple Python Tutorial](https://blog.jonharrington.org/rust-and-swig/)
* [Install Swig](http://www.swig.org/Doc4.0/Preface.html#Preface_unix_installation)
    * [Download Tarball](https://sourceforge.net/projects/swig/files/swig/swig-4.0.1/swig-4.0.1.tar.gz/download)
* [Swig Docs](http://swig.org/Doc4.0/Sections.html#Sections)
  * [Python](http://swig.org/Doc4.0/Python.html#Python)


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

Now, when I try `./configure`, I see more options. Let's rebuild with all enabled:

```shell script
./configure
make
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

