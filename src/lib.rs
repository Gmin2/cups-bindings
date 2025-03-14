#[allow(non_upper_case_globals, non_camel_case_types, dead_code)]
mod bindings {
    include!(concat!(env!("OUT_DIR"), "/bindings.rs"));
}
