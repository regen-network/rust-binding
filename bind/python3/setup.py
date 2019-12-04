#!/usr/bin/env python3

"""
setup.py file for SWIG example
"""

from os import getcwd

from distutils.core import setup, Extension


rust_module = Extension('_rust_binding',
                           sources=['rust_binding_wrap.c'],
                           libraries=['rust_binding'],
                           )

setup (name = 'rust',
       version = '0.1',
       author      = "Regen",
       description = """Sample swig example""",
       ext_modules = [rust_module],
       py_modules = [],
       )