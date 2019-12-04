#!/usr/bin/env python

"""
setup.py file for SWIG example
"""

from os import getcwd

from distutils.core import setup, Extension


rust_module = Extension('_rust_ffi',
                           sources=['rust_ffi_wrap.c'],
                           libraries=['rust_ffi'],
                           )

setup (name = 'rust',
       version = '0.1',
       author      = "Regen",
       description = """Sample swig example""",
       ext_modules = [rust_module],
       py_modules = [],
       )