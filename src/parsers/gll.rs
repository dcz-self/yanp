use super::utils::*;
use crate::errors::NmeaSentenceError;
use crate::parse::*;

fn build_gll<'a>(
    sentence: (Option<GpsPosition>, Option<GpsTime>, Option<char>),
) -> Result<GllData, NmeaSentenceError<'a>> {
    Ok(GllData {
        position: sentence.0,
        time: sentence.1,
        status: translate_option!(sentence.2, GllStatus),
    })
}

named!(pub (crate) parse_gll<GllData>,
    map_res!(
        do_parse!(
            position: complete!(
                nom::branch::alt((
                    nom::combinator::complete(nom::combinator::map(parse_gps_position, Some)),
                    nom::combinator::value(
                        None,
                        nom::bytes::complete::tag(b",,,")
                    )
                ))
            ) >>
            char!(',') >>
            time: opt!(complete!(parse_utc_stamp)) >>
            char!(',') >>
            status: opt!(one_of!("AVP")) >>
            opt!(char!(',')) >>
            _mode: opt!(one_of!("ADEMSN")) >>
            char!('*') >>
            (position, time, status)
        ),
        build_gll
    )
);


#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn parse() {
        let s = b",,,,175936.860,V,M*";
        assert_eq!(
            parse_gll(s),
            Ok((
                &b""[..],
                GllData {
                    position: None,
                    time: Some(GpsTime { hour: 17, minute: 59, second: 36, millisecond: 860 }),
                    status: Some(GllStatus::DataInvalid),
                },
            ))
        )
    }  
}