use byteorder::{LittleEndian, ReadBytesExt};
use paste::paste;
use std::io::Read;

use super::*;

#[rustfmt::skip]
parameters!(
    EEPROM_LENGTH, "EEPROM length", 0x00, 1, "Number of EEPROM data locations, must be equal to 0x64",
    EEPROM_CHECK_1, "EEPROM check 1", 0x01, 1, "EEPROM data integrity check byte, must be equal to 0x5b",
    REMAINING_TIME_ALARM, "Remaining time alarm", 0x02, 2, min, "Sets remaining time alarm",
    REMAINING_CAPACITY_ALARM, "Remaining capacity alarm", 0x04, 2, mAh, "Sets remaining capacity alarm",
    RESERVED_1, "Reserved", 0x06, 2, "Reserved for future use",
    INITIAL_CHARGING_CURRENT, "Initial charging current", 0x08, 2, mA, "Sets the initial charging current",
    CHARGING_VOLTAGE, "Charging voltage", 0x0a, 2, mV, "Sets charging voltage",
    BATTERY_STATUS, "Battery status", 0x0c, 2, "Initializes battery status",
    CYCLE_COUNT, "Cycle count", 0x0e, 2, "Initializes and stores cycle count",
    DESIGN_CAPACITY, "Design capacity", 0x10, 2, mAh, "Sets design capacity",
    DESIGN_VOLTAGE, "Design voltage", 0x12, 2, mV, "Sets design voltage",

    MANUFACTURE_DATE, "Manufacture date", 0x16, 2, date, "Programs manufacture date",
    SERIAL_NUMBER, "Serial number", 0x18, 2, "Programs serial number",
    FAST_CHARGING_CURRENT, "Fast-charging current", 0x1a, 2, mA, "Sets charging current",
    MAINTENANCE_CHARGE_CURRENT, "Maintenance-charge current", 0x1c, 2, mA, "Sets the trickle current request",
    RESERVED_2, "Reserved", 0x1e, 2, "Reserved, must be 0x0000",
    MANUFACTURER_NAME, "Manufacturer name", 0x20, 12, text, "Programs manufacturer name",
    CURRENT_OVERLOAD, "Current overload", 0x2c, 2, mA, "Sets the overload current threshold",
    BATTERY_LOW_PERCENTAGE, "Battery low %", 0x2e, 1, percent, "Sets the battery low amount",
    RESERVED_3, "Reserved", 0x2f, 1, "Reserved for future use",
    DEVICE_NAME, "Device name", 0x30, 8, text, "Programs device name",
    LION_TAPER_CURRENT, "Li-Ion taper current", 0x38, -2, mA, "Sets the upper limit of the taper current for charge termination",
    MAXIMUM_OVERCHARGE_LIMIT, "Maximum overcharge limit", 0x3a, -2, mA, "Sets the maximum amount of overcharge",
    RESERVED_4, "Reserved", 0x3c, 1, "Reserved, must be 0x00",

    DEVICE_CHEMISTRY, "Device chemistry", 0x40, 6, text, "Programs device chemistry",

    FULL_CHARGE_PERCENTAGE, "Full-charge percentage", 0x4c, -1, percent, "Sets the percent at which the battery is considered fully charged",

    SELF_DISCHARGE_RATE, "Self-discharge rate", 0x4f, 1, "Sets the battery's self-discharge rate", // func
    MANUFACTURER_DATA, "Manufacturer data", 0x50, 6, text, "Programs manufacturer data",

    RESERVED_5, "Reserved", 0x58, 2, "Reserved, should be programmed to 0",
    EDVF_CHARGING_CURRENT, "EDVF charging current", 0x5a, 2, mA, "Sets the charge current request when battery voltage is less than EDVF",
    END_OF_DISCHARGE_VOLTAGE, "End of discharge voltage 1", 0x5c, -2, mV, "Sets EDV1",
    END_OF_DISCHARGE_VOLTAGE_FINAL, "End of discharge voltage final", 0x5e, -2, mV, "Sets EDVF",
    FULL_CHARGE_CAPACITY, "Full-charge capacity", 0x60, 2, mAh, "Initializes and stores full-charge capacity",

    EEPROM_CHECK_2, "EEPROM check2", 0x64, 1, "Must be equal to 0xb5",
);

fn_value_as_string_u8!();

fn_value_as_string_u8!("percent", "%");

fn_value_as_string_i8!("percent", "%");

fn_value_as_string_u16!();

fn_value_as_string_u16!("mAh");

fn_value_as_string_u16!("mA");

fn_value_as_string_i16!("mA");

fn_value_as_string_u16!("mV");

fn_value_as_string_i16!("mV");

fn_value_as_string_u16!("min");

fn_value_as_string_u16!("date", |value| {
    let day = value & 0x001fu16;
    let month = value >> 5 & 0x000fu16;
    let year = 1980u16 + (value >> 9);
    format!("{year:4}-{month:02}-{day:02}")
});

fn_value_as_string!("text", |data: &[u8], len: u8| {
    let mut slice = data;
    let count = slice.read_u8().unwrap();
    let mut text = vec![0; (if count > len { len } else { count }).into()];
    slice.read_exact(&mut text).unwrap();

    StringValue {
        hex: String::new(),
        usr: std::str::from_utf8(&text).unwrap().to_owned(),
        units: "",
    }
});
