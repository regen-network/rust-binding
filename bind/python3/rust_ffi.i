// this is important to use any non int_t types
%include "stdint.i"

%module rust_ffi
%{
#include "rust_ffi.h"
%}
%include "rust_ffi.h"