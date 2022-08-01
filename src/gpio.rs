//! GPIO peripheral

#![allow(non_camel_case_types)]
use k210_pac as pac;
use crate::fpioa::{function, fpioa_pull};
use super::utils::{set_bit,get_bit};
use super::fpioa;

const GPIO_MAX_PINNO: u8 = 8;

#[derive(Copy, Clone, PartialEq, Eq)]
pub enum direction {
    INPUT,
    OUTPUT,
}

pub enum drive_mode {
    GPIO_DM_INPUT,
    GPIO_DM_INPUT_PULL_DOWN,
    GPIO_DM_INPUT_PULL_UP,
    GPIO_DM_OUTPUT,
}

// TODO

pub fn set_drive_mode(pin: u8, mode: drive_mode) {
    assert!(pin < GPIO_MAX_PINNO);
    let io_number = fpioa::get_io_by_function((fpioa::function::GPIO0 as u8 + pin).into());
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
    set_direction(pin, dir);
}

fn set_direction(pin: u8, direction: direction) {
    unsafe {
        let ptr = pac::GPIO::ptr();
        (*ptr)
            .direction
            .modify(|r, w| w.bits(set_bit(r.bits(), pin, direction == direction::OUTPUT)));
        (*ptr)
            .direction
            .modify(|r, w| w.bits(set_bit(r.bits(), pin, direction == direction::INPUT)));
    }
}

pub fn set_pin(pin: u8, value: bool) {
    unsafe {
        let ptr = pac::GPIO::ptr();
        (*ptr)
            .data_output
            .modify(|r, w| w.bits(set_bit(r.bits(), pin, value)));
    }
}
