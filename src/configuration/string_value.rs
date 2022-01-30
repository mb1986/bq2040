pub(crate) struct StringValue {
    pub hex: String,
    pub usr: String,
    pub units: &'static str,
}

macro_rules! fn_value_as_string_u8 {
    () => {
        fn_value_as_string_u8!("", "");
    };

    ( $units:literal ) => {
        fn_value_as_string_u8!($units, $units);
    };

    ( $name:literal, $units:literal ) => {
        paste! {
            fn [<value_as_string_ $name _u8>](_: &ConfigurationParameter, data: &[u8]) -> StringValue {
                let mut slice = data;
                let value = slice.read_u8().unwrap();
                StringValue {
                    hex: format!("{:#04x}", &value),
                    usr: format!("{}", &value),
                    units: $units,
                }
            }
        }
    };
}

macro_rules! fn_value_as_string_i8 {
    () => {
        fn_value_as_string_i8!("", "");
    };

    ( $units:literal ) => {
        fn_value_as_string_i8!($units, $units);
    };

    ( $name:literal, $units:literal ) => {
        paste! {
            fn [<value_as_string_ $name _i8>](_: &ConfigurationParameter, data: &[u8]) -> StringValue {
                let mut slice = data;
                let value = slice.read_i8().unwrap().unsigned_abs();
                StringValue {
                    hex: format!("{:#04x}", &value),
                    usr: format!("{}", &value),
                    units: $units,
                }
            }
        }
    };
}

macro_rules! fn_value_as_string_u16 {
    () => { fn_value_as_string_u16!(""); };

    ( $units:literal ) => {
        paste! {
            fn [<value_as_string_ $units:lower _u16>](_: &ConfigurationParameter, data: &[u8]) -> StringValue {
                let mut slice = data;
                let value = slice.read_u16::<LittleEndian>().unwrap();
                StringValue {
                    hex: format!("{:#06x}", &value),
                    usr: format!("{}", &value),
                    units: $units,
                }
            }
        }
    };

    ( $units:literal, $fun:expr ) => {
        paste! {
            fn [<value_as_string_ $units:lower _u16>](_: &ConfigurationParameter, data: &[u8]) -> StringValue {
                let mut slice = data;
                let value = slice.read_u16::<LittleEndian>().unwrap();
                let str_val = $fun(&value);
                StringValue {
                    hex: format!("{:#06x}", &value),
                    usr: format!("{}", &str_val),
                    units: "",
                }
            }
        }
    };
}

macro_rules! fn_value_as_string_i16 {
    () => { fn_value_as_string_i16!(""); };

    ( $units:literal ) => {
        paste! {
            fn [<value_as_string_ $units:lower _i16>](_: &ConfigurationParameter, data: &[u8]) -> StringValue {
                let mut slice = data;
                let value = slice.read_i16::<LittleEndian>().unwrap().unsigned_abs();
                StringValue {
                    hex: format!("{:#06x}", &value),
                    usr: format!("{}", &value),
                    units: $units,
                }
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
        value_as_string__u8
    };

    ( 2, ) => {
        value_as_string__u16
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
    fn_value_as_string, fn_value_as_string_i16, fn_value_as_string_i8, fn_value_as_string_u16,
    fn_value_as_string_u8, value_as_string,
};
