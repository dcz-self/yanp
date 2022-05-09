use crate::errors::NmeaSentenceError;
use crate::parse::*;
pub(crate) use nom::{map_res, named, one_of, opt, tag, take, take_until};
use nom::combinator::opt;


macro_rules! next {
    ($input:ident, $output:ident = $op:expr) => {
        let ($input, $output) = $op($input)?;
    };
    ($input:ident, $op:expr) => {
        next!($input, _x = $op)
    }
}

use nom::combinator::{map_res, rest};
use nom::character::complete::char;

pub(crate) fn parse_speed(inp: &[u8]) -> Result<(u16, u8), nom::Err<nom::error::Error<&[u8]>>> {
    next!(inp, whole = map_res(
        nom::bytes::complete::take_until("."),
        parse_num::<u16>,
    ));
    next!(inp, char('.'));
    next!(inp, hundredths = map_res(
        rest,
        parse_num::<u8>,
    ));
    let _ = inp;
    
    Ok((whole, hundredths))
}


pub(crate) fn num<T: core::str::FromStr>(inp: &[u8])
    -> Result<
        (&[u8], Option<T>),
        nom::Err<nom::error::Error<&[u8]>>
    >
{
    opt(nom::combinator::map_res(
        nom::bytes::complete::take_until(","),
        parse_num::<T>,
    ))(inp)
}

/// digits: The first digit is multiplied by 10^digits
pub(crate) fn parse_num<I: core::str::FromStr>(data: &[u8]) -> Result<I, NmeaSentenceError> {
    let raw = unsafe { core::str::from_utf8_unchecked(data) };
    str::parse::<I>(raw)
        .map_err(|_| NmeaSentenceError::GeneralParsingError)
}

/// digits: The first digit is at the place of 10^digits. Missing digits are 0s.
pub(crate) fn parse_num_fill<
    I: core::str::FromStr + core::ops::Mul<I, Output=I> + Copy
>(data: &[u8], digits: u8) -> Result<I, NmeaSentenceError> {
    let raw = unsafe { core::str::from_utf8_unchecked(data) };
    str::parse::<I>(raw)
        .map(|v| (0..(digits - raw.len() as u8)).fold(v, |a, _| a * v))
        .map_err(|_| NmeaSentenceError::GeneralParsingError)
}

macro_rules! translate_option {
    ($input:expr, $status:ident) => {
        match $input {
            Some(value) => Some($status::try_from(value)?),
            None => None,
        }
    };
}

named!(pub (crate) parse_utc_stamp<GpsTime>,
    map_res!(
        do_parse!(
            hour: map_res!(take!(2), parse_num::<u8>) >>
            minute:  map_res!(take!(2), parse_num::<u8>) >>
            second:  map_res!(take!(2), parse_num::<u8>) >>
            char!('.') >>
            millisecond: map_res!(
                take_until!(","),
                |n| parse_num_fill::<u16>(n, 3)
            ) >>
            (hour, minute, second, millisecond)
        ),
        |timestamp: (u8, u8, u8, u16)| -> Result<GpsTime, NmeaSentenceError>{
            Ok(GpsTime{
                hour: timestamp.0,
                minute: timestamp.1,
                second: timestamp.2,
                millisecond: timestamp.3,
            })
        }
    )
);

named!(pub (crate) parse_date<GpsDate>,
    map_res!(
        do_parse!(
            day: map_res!(take!(2), parse_num::<u8>) >>
            month: map_res!(take!(2), parse_num::<u8>) >>
            year: map_res!(take!(2), parse_num::<u64>) >>
            (day, month, year)
        ),
        |data: (u8, u8, u64)| -> Result<GpsDate, NmeaSentenceError>{
            Ok(GpsDate {
                day: data.0,
                month: data.1,
                year: data.2,
            })
        }
    )
);

named!(pub (crate) parse_gps_position<GpsPosition>,
    map_res!(
        do_parse!(
            lat_deg: map_res!(take!(2), parse_num::<u8>) >>
            lat_min: map_res!(take_until!(","), parse_num::<f32>) >>
            char!(',') >>
            lat_dir: one_of!("NS") >>
            char!(',') >>
            lon_deg: map_res!(take!(3), parse_num::<u8>) >>
            lon_min: map_res!(take_until!(","), parse_num::<f32>) >>
            char!(',') >>
            lon_dir: one_of!("EW") >>
            (lat_deg, lat_min, lat_dir, lon_deg, lon_min, lon_dir)
        ),
        | position: (u8, f32, char, u8, f32, char) | -> Result<GpsPosition, NmeaSentenceError>{
            Ok(GpsPosition{
                lat: (position.0 as f32) + position.1 / 60.,
                lat_dir: LatitudeDirection::try_from(position.2)?,
                lon: (position.3 as f32) + position.4 / 60.,
                lon_dir: LongitudeDirection::try_from(position.5)?,
            })
        }
    )
);

pub(crate) fn invalid_height_check<'a>(
    height: Option<&'a [u8]>,
) -> Result<Option<f32>, NmeaSentenceError> {
    Ok(match height {
        Some(val) => match val {
            b"-" | b"" => None,
            val => Some(parse_num::<f32>(val)?),
        },
        None => None,
    })
}
