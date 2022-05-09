use super::utils::*;
use crate::errors::NmeaSentenceError;
use crate::parse::*;

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


named!(pub (crate) parse_vtg<VtgData>,
    map_res!(
        do_parse!(
            bearing_true: opt!(map_res!(take_until!(","), parse_num::<f32>)) >>
            alt!(tag!(",T,") | tag!(",,")) >>
            bearing_magnetic: opt!(map_res!(take_until!(","), parse_num::<f32>)) >>
            alt!(tag!(",M,") | tag!(",,"))>>
            speed_knots: opt!(map_res!(take_until!(","), parse_num::<f32>)) >>
            alt!(tag!(",N,") | tag!(",,")) >>
            speed_kmh: opt!(map_res!(take_until!(","), parse_speed)) >>
            char!(',') >> opt!(char!('K')) >> opt!(char!(',')) >>
            mode: opt!(one_of!("ADEMSN")) >>
            char!('*') >>
            (bearing_true, bearing_magnetic, speed_knots, speed_kmh, mode)
        ),
        | sentence: (Option<f32>, Option<f32>, Option<f32>, Option<(u16, u8)>, Option<char>)| -> Result<VtgData, NmeaSentenceError> {
            Ok(VtgData{
                bearing_true: sentence.0,
                bearing_magnetic: sentence.1,
                speed_knots: sentence.2,
                speed_kmh: sentence.3,
                mode: sentence.4,
            })
        }
    )
);


#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn parse() {
        let s = b"0.00,T,,M,0.00,N,0.00,K,A*";
        assert_eq!(
            parse_vtg(s),
            Ok((
                &b""[..],
                VtgData { bearing_magnetic: None,
                    bearing_true: Some(0.0),
                    speed_knots: Some(0.0),
                    speed_kmh: Some((0, 0)),
                    mode: Some('A'),
                },
            ))
        )
    }
    
    #[test]
    fn empty() {
        let s = b",,,,,,,,M*";
        assert_eq!(
            parse_vtg(s),
            Ok((
                &b""[..],
                VtgData { bearing_magnetic: None,
                    bearing_true: None,
                    speed_knots: None,
                    speed_kmh: None,
                    mode: Some('M'),
                },
            ))
        )
    }
}