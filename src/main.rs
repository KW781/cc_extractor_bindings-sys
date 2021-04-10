use std::process;
use std::io::raw::{c_int, c_long};
use std::mem::{self, MaybeUninit};
use std::ffi::CString;
use std::env;
use std::convert::TryInto;
use cc_extractor_bindings-sys;

//mod bindings;

fn main() {
    unsafe {
        let mut args = env::args()
            .map(|arg| CString::new(arg).map(CString::into_raw))
            .collect::<Result<Vec<_>, _>>()
            .expect("One or more of the arguments contains a null charactor");
        let argc = args.len().try_into().expect("Can't convert argc into c_int type");
        let argv = args.as_mut_ptr();

        cc_extractor_bindings-sys::setlocale(LC_ALL, ""); //support for non-English CCs

        let api_options = cc_extractor_bindings-sys::api_init_options();
        cc_extractor_bindings-sys::parse_configuration(api_options);

        let compile_ret = cc_extractor_bindings-sys::parse_parameters(api_options, argc, argv);

        for pointer in args {
            drop(CString::from_raw(pointer));
        }

        if compile_ret == EXIT_NO_INPUT_FILES as i32 {
            cc_extractor_bindings-sys::print_usage();
            fatal(EXIT_NO_INPUT_FILES, "(This help screen was shown because there were no input files)\n");
        }
        else if compile_ret == EXIT_WITH_HELP as i32{
            process:exit(EXIT_OK);
        }
        else if compile_ret != EXIT_OK as i32{
            process::exit(compile_ret);
        }

        let start_ret = cc_extractor_bindings-sys::api_start(*api_options);
        process::exit(start_ret);
    }
}
