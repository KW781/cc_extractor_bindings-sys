# cc_extractor_bindings-sys

Rust bindings for CCExtractor

Simply include ccextractor alongside the source directory, then run the project with cargo and it should work.

Note:
Just the main() function in main.rs has been written (from ccextractor.c), not all of main.rs has been completed.

So far just lib_ccx.h, list.h and ccx_common_options.h has been included in the wrapper header. Over time as required, more header files can be included easily in the wrapper.

Right now these bindings don't account for blacklisting/whitelisting functions or variable names. These are to be added in later when required. 
