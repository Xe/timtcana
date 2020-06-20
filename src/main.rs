use anyhow::{anyhow, Result};
use gpio::{
    GpioIn, GpioOut,
    sysfs::{SysFsGpioInput, SysFsGpioOutput},
    GpioValue::{self, High, Low},
};
use log::*;
use std::{thread, time};

/// Measures of temperature and humidity
///
/// Given dht11 precision both are simple `u8`
#[derive(Debug, Clone)]
pub struct Measures {
    temperature: u8,
    humidity: u8,
}

fn read(pin: u16) -> Result<Measures> {
    debug!("opened pin {} for read", pin);
    let mut input = gpio::sysfs::SysFsGpioInput::open(pin)?;
    debug!("opened pin {} for write", pin);
    let mut output = gpio::sysfs::SysFsGpioOutput::open(pin)?;


    debug!("set pin {} low", pin);
    output.set_low()?;
    thread::sleep(time::Duration::from_millis(18));
    debug!("set pin {} high", pin);
    output.set_high()?;
    thread::sleep(time::Duration::from_micros(40));

    debug!("starting to read temperature data");
    let mut bytes = [0u8; 5];
    {
        want(&mut input, 0)?;
        want(&mut input, 1)?;
        want(&mut input, 0)?;

        for b in bytes.iter_mut() {
            debug!("reading byte");
            for _ in 0..8 {
                debug!("b: {:x}", b);
                *b <<= 1;
                want(&mut input, 1)?;
                let dur = want(&mut input, 0)?;
                if dur > 16 {
                    *b |= 1;
                }
            }
        }
    }

    let sum: u16 = bytes.iter().take(4).map(|b| *b as u16).sum();

    if bytes[4] as u16 == sum & 0x00FF {
        Ok(Measures {
            temperature: bytes[2],
            humidity: bytes[0],
        })
    } else {
        Err(anyhow::anyhow!(""))
    }
}

fn want<T: Into<GpioValue>>(p: &mut SysFsGpioInput, val: T) -> Result<u8> {
    let val = val.into();
    debug!("want pin to read {:?}", val);
    for i in 0u8..255 {
        if p.read_value()? == val {
            debug!("dur: {}", i);
            return Ok(i);
        }
        thread::sleep(time::Duration::from_micros(2));
    }

    debug!("timeout getting duration");
    Err(anyhow::anyhow!("timeout"))
}

fn main() -> Result<()> {
    pretty_env_logger::try_init()?;
    info!("reading temperature");
    let temp = read(3)?;
    println!("{:?}", temp);
    Ok(())
}
