use std::sync::Once;

fn main() {
    static V8_INIT: Once = Once::new();

    V8_INIT.call_once(move || {
        let flags = String::from(concat!(
            " --no-validate-asm",
            " --turbo-fast-api-calls",
            " --harmony-temporal",
            " --js-float16array",
            " --js-explicit-resource-management",
        ));

        v8::V8::set_flags_from_string(&flags);

        let platform = if cfg!(any(test, feature = "v8_unsafe_use_unprotected_platform")) {
            // Use unprotected platform for unit testing
            v8::new_unprotected_default_platform(0, false)
        } else {
            v8::new_default_platform(0, false)
        }
        .make_shared();
        v8::V8::initialize_platform(platform);
        v8::V8::initialize();
    });

    println!("v8 version:{}", v8::VERSION_STRING);
}

#[cfg(test)]
mod tests {
    #[test]
    pub fn test1() {}
}
