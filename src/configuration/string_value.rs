pub(crate) struct StringValue {
    pub hex: String,
    pub usr: String,
    pub units: &'static str,
}

macro_rules! fn_internal8 {
    ( $units:literal, $data:ident, $type:ident $(, $op:expr )? ) => {{
        let mut slice = $data;
        paste! {
            let value = slice.[<read_ $type>]().unwrap()$(. $op )?;
        }
        StringValue {
            hex: format!("{:#04x}", &value),
            usr: format!("{}", &value),
            units: $units,
        }
    }};
}

macro_rules! fn_internal16 {
    ( $units:literal, $data:ident, $type:ident $(, $op:expr )? $(; $fun:expr )? ) => {{
        let mut slice = $data;
        paste! {
            let value = slice.[<read_ $type>]::<LittleEndian>().unwrap()$(. $op )?;
        }
        StringValue {
            hex: format!("{:#06x}", &value),
            usr: format!("{}", &$( $fun )?(&value)),
            units: $units,
        }
    }};
}

macro_rules! fn_value_as_string_u8 {
    () => {
        fn value_as_string_u8(_: &ConfigurationParameter, data: &[u8]) -> StringValue {
            fn_internal8!( "", data, u8 )
        }
    };

    ( $units:literal ) => {
        fn_value_as_string_u8!($units, $units);
    };

    ( $name:literal, $units:literal ) => {
        paste! {
            fn [<value_as_string_ $name _u8>](_: &ConfigurationParameter, data: &[u8]) -> StringValue {
                fn_internal8!( $units, data, u8 )
            }
        }
    };
}

macro_rules! fn_value_as_string_i8 {
    ( $units:literal ) => {
        fn_value_as_string_i8!($units, $units);
    };

    ( $name:literal, $units:literal ) => {
        paste! {
            fn [<value_as_string_ $name _i8>](_: &ConfigurationParameter, data: &[u8]) -> StringValue {
                fn_internal8!( $units, data, i8, unsigned_abs() )
            }
        }
    };
}

macro_rules! fn_value_as_string_u16 {
    () => {
        fn value_as_string_u16(_: &ConfigurationParameter, data: &[u8]) -> StringValue {
            fn_internal16!( "", data, u16 )
        }
    };

    ( $units:literal ) => {
        paste! {
            fn [<value_as_string_ $units:lower _u16>](_: &ConfigurationParameter, data: &[u8]) -> StringValue {
                fn_internal16!( $units, data, u16 )
            }
        }
    };

    ( $units:literal, $fun:expr ) => {
        paste! {
            fn [<value_as_string_ $units:lower _u16>](_: &ConfigurationParameter, data: &[u8]) -> StringValue {
                fn_internal16!( "", data, u16; $fun )
            }
        }
    };
}

macro_rules! fn_value_as_string_i16 {
    ( $units:literal ) => {
        paste! {
            fn [<value_as_string_ $units:lower _i16>](_: &ConfigurationParameter, data: &[u8]) -> StringValue {
                fn_internal16!( $units, data, i16, unsigned_abs() )
            }
        }
    };
}

macro_rules! fn_value_as_string {
    ( $units:literal, $fun:expr ) => {
        paste! {
            fn [<value_as_string_ $units:lower>](param: &ConfigurationParameter, data: &[u8]) -> StringValue {
                $fun(&data, param.length)
            }
        }
    };
}

macro_rules! value_as_string {
    ( 1, ) => {
        value_as_string_u8
    };

    ( 2, ) => {
        value_as_string_u16
    };

    ( 1, $units:ident ) => {
        paste! { [<value_as_string_ $units _u8>] }
    };

    ( -1, $units:ident ) => {
        paste! { [<value_as_string_ $units _i8>] }
    };

    ( 2, $units:ident ) => {
        paste! { [<value_as_string_ $units _u16>] }
    };

    ( -2, $units:ident ) => {
        paste! { [<value_as_string_ $units _i16>] }
    };

    ( $len:literal, $units:ident ) => {
        paste! { [<value_as_string_ $units>] }
    };
}

pub(crate) use {
    fn_internal16, fn_internal8, fn_value_as_string, fn_value_as_string_i16, fn_value_as_string_i8,
    fn_value_as_string_u16, fn_value_as_string_u8, value_as_string,
};
