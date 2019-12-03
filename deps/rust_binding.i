// this is important to use any non int_t types
%include "stdint.i"

%module rust_binding
%{
#include "rust_binding.h"
%}
%include "rust_binding.h"