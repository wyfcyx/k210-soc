#![allow(unused)]

//! GPIOHS peripheral
//use k210_hal::pac;
use k210_pac as pac;

use crate::gpio::{direction, drive_mode};
use super::fpioa;
use super::gpio;
use super::utils::{set_bit,get_bit};

const GPIOHS_MAX_PINNO: u8 = 32;

// TODO embedded-hal::digital::v2::{InputPin, OutputPin}

/** Set input/output direction for a GPIOHS pin */
pub fn set_direction(pin: u8, direction: gpio::direction) {
    unsafe {
        let ptr = pac::GPIOHS::ptr();
        (*ptr)
            .output_en
            .modify(|r, w| w.bits(set_bit(r.bits(), pin, direction == gpio::direction::OUTPUT)));
        (*ptr)
            .input_en
            .modify(|r, w| w.bits(set_bit(r.bits(), pin, direction == gpio::direction::INPUT)));
    }
}

pub fn set_drive_mode(pin: u8, mode: gpio::drive_mode) {
    assert!(pin < GPIOHS_MAX_PINNO);
    let io_number = fpioa::get_io_by_function((fpioa::function::GPIOHS0 as u8 + pin).into());
    assert!(io_number >= 0);

    let p: fpioa::pull;
    let dir: direction;

    match mode {
        drive_mode::GPIO_DM_INPUT => {
            p = fpioa::pull::NONE;
            dir = direction::INPUT;
        }
        drive_mode::GPIO_DM_INPUT_PULL_DOWN => {
            p = fpioa::pull::DOWN;
            dir = direction::INPUT;
        }
        drive_mode::GPIO_DM_INPUT_PULL_UP => {
            p = fpioa::pull::UP;
            dir = direction::INPUT;
        }
        drive_mode::GPIO_DM_OUTPUT => {
            p = fpioa::pull::DOWN;
            dir = direction::OUTPUT;
        }
        _ => panic!("GPIO drive mode is not supported")
    }

    fpioa::set_io_pull(io_number as usize, p);
    set_pin_en(pin, dir == direction::OUTPUT);
}

/** Set output value for a GPIOHS pin */
pub fn set_pin(pin: u8, value: bool) {
    unsafe {
        let ptr = pac::GPIOHS::ptr();
        (*ptr)
            .output_val
            .modify(|r, w| w.bits(set_bit(r.bits(), pin, value)));
    }
}

pub fn set_pin_en(pin: u8, value: bool) {
    unsafe {
        let ptr = pac::GPIOHS::ptr();
        (*ptr)
            .output_en
            .modify(|r, w| w.bits(set_bit(r.bits(), pin, value)));
            (*ptr)
            .input_en
            .modify(|r, w| w.bits(set_bit(r.bits(), pin, !value)));
    }
}

/** Get input value for a GPIOHS pin */
pub fn get_pin(pin: u8) -> bool {
    unsafe {
        let ptr = pac::GPIOHS::ptr();
        get_bit((*ptr).input_val.read().bits(), pin)
    }
}