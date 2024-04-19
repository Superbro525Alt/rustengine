// #[macro_export]
macro_rules! assert_is_type {
    ($t:ty, $i:ident: $ti:ty) => {
        const _: () = {
            fn dummy(v: $t) {
                let _: $ti = v.$i;
            }
        };
    };
}
