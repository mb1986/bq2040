use super::StringValue;

pub(crate) struct ConfigurationParameter {
    pub name: &'static str,
    pub description: &'static str,
    pub address: u8,
    pub length: u8,
    pub(super) value_as_string: fn(&ConfigurationParameter, &[u8]) -> StringValue,
}

impl ConfigurationParameter {
    pub fn value_as_string(&self, data: &[u8]) -> StringValue {
        (self.value_as_string)(self, data)
    }
}

macro_rules! labels {
    ( $idx:expr, $label:ident ) => {
        #[allow(dead_code)]
        pub const $label: usize = $idx;
        pub const PARAMS_COUNT: usize = $idx+1;
    };

    ( $idx:expr, $label:ident, $( $labels:ident ),+ ) => {
        #[allow(dead_code)]
        pub const $label: usize = $idx;
        labels!($idx+1, $( $labels ),+);
    };
}

macro_rules! parameter {
    ( $name:literal, $addr:literal, $len:expr, $desc:literal, $fn_str_val:expr ) => {
        ConfigurationParameter {
            name: $name,
            address: $addr,
            length: $len,
            description: $desc,
            value_as_string: $fn_str_val,
        }
    };
}

macro_rules! parameters {
    ( $( $label:ident, $name:literal, $addr:literal, $len:literal, $( $units:ident, )? $desc:literal ),+ $(,)? ) => {
        labels!(0, $( $label ),*);

        paste! {
            pub(crate) const PARAMETERS: [ConfigurationParameter; PARAMS_COUNT] = [
                $( parameter!( $name, $addr, ($len as i8 as u8), $desc, value_as_string!($len, $( [<$units:lower>] )?)) ),*
            ];
        }
    };
}

pub(crate) use {labels, parameter, parameters};
