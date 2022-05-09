use super::utils::*;
use crate::parse::*;

use nom::combinator::opt;
use nom::character::complete::char;


pub(crate) fn parse(inp: &[u8]) -> Result<(&[u8], ZdaData), nom::Err<nom::error::Error<&[u8]>>> {
    next!(inp, time = opt(parse_utc_stamp));
    next!(inp, char(','));
    next!(inp, day = num::<u8>);
    next!(inp, char(','));
    next!(inp, month = num::<u8>);
    
    next!(inp, char(','));
    next!(inp, year = num::<u16>);
    next!(inp, char(','));
    let date = day
        .and_then(|d| month.map(|m| (m, d)))
        .and_then(|dm| year.map(|y| (y, dm)))
        .map(|(year, (month, day))| GpsDate {
            year: year as u64, month, day,
        });
    
    next!(inp, hours = num::<u8>);
    next!(inp, char(','));
    next!(inp, minutes = opt(nom::combinator::map_res(
        nom::bytes::complete::take_until("*"),
        parse_num::<u8>,
    )));
    next!(inp, char('*'));
    let zone = hours
        .and_then(|h| minutes.map(|m| (m, h)))
        .map(|(hours, minutes)| Zone { hours, minutes });
    
    Ok((inp, ZdaData {
        time,
        date,
        zone,
    }))
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn t_parse() {
        let s = b"084929.282,09,05,2022,00,00*";
        assert_eq!(
            parse(s),
            Ok((
                &b""[..],
                ZdaData {
                    time: Some(GpsTime { hour: 8, minute: 49, second: 29, millisecond: 282 }),
                    date: Some(GpsDate { day: 9, month: 5, year: 2022 }),
                    zone: Some(Zone { hours: 0, minutes: 0 }),
                },
            ))
        )
    }  
}
