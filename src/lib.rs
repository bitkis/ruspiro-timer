/*********************************************************************************************************************** 
 * Copyright (c) 2019 by the authors
 * 
 * Author: André Borrmann 
 * License: Apache License 2.0
 **********************************************************************************************************************/
#![doc(html_root_url = "https://docs.rs/ruspiro-timer/0.0.1")]
#![no_std]
#![feature(asm)]
//! # Timer functions
//! 
//! This crate provides simple timing functions to pause the core for a specific amount of time.
//! 
//! # Usage
//! 
//! ```
//! use rusprio_timer as timer;
//! 
//! fn demo() {
//!     timer::sleep(1000); // pause for 1 milli second
//!     timer::sleepcycles(200); // pause for 200 CPU cycles
//! }
//! 
//! ```
//! 

use ruspiro_register::define_registers;

pub type Useconds = u64;

/// Pause the current execution for the given amount of micro seconds
pub fn sleep(usec: Useconds) {
    let wait_until = now() + usec;

    while !is_due(wait_until) { };
}

/// Pause the current execution for the given amount of CPU cycles
pub fn sleepcycles(cycles: u32) {
    for _ in 0..cycles {
        unsafe { asm!("NOP") };
    }
}

// MMIO peripheral base address based on the target family provided with the custom target config file.
#[cfg(target_family="ruspiro-pi3")]
const PERIPHERAL_BASE: u32 = 0x3F00_0000;

#[cfg(not(target_family="ruspiro-pi3"))]
const PERIPHERAL_BASE: u32 = 0x2000_0000;

// Base address of timer MMIO register
const TIMER_BASE: u32 = PERIPHERAL_BASE + 0x0000_3000;

define_registers! [
    TIMERCLO: ReadOnly<u32> @ TIMER_BASE + 0x04 => [],
    TIMERCHI: ReadOnly<u32> @ TIMER_BASE + 0x08 => []
];

/// Get the current time as free running counter value of the system timer
fn now() -> Useconds {
    let t_low = TIMERCLO::Register.get() as u64;
    let t_high = TIMERCHI::Register.get() as u64;

    (t_high << 32) | t_low
}

/// Compare the given time as free running counter value with the current time.
/// Returns true if the current time is later than the time passed into this function.
fn is_due(time: Useconds) -> bool {
    if time == 0 {
        // if no valid time is given, time is always due
        true
    } else {
        // returns true if we have reached the current time (counter)
        now() >= time
    }
}