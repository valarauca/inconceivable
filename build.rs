extern crate rustc_version;
use rustc_version::version_meta;

fn main() {
    let version_info = match version_meta() {
        Ok(v) => v,
        Err(e) => panic!("could not identify rustc version. error:'{:?}'", e),
    };

    if version_info.semver.major != 1 {
        panic!("presently we only support version 1.X of rustc");
    }

    if version_info.semver.major == 1 && version_info.semver.minor >= 27 {
        // special configuration variable for compiler version 1.27
        // this is when `core::hint::unchecked_unreachable` was
        // stablized.
        println!("cargo:rustc-cfg=RUSTC_VERSION_GE_1_27");
    }
}
