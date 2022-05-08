use super::utils::*;
use crate::errors::NmeaSentenceError;
use crate::parse::*;

fn build_gga<'a>(
    sentence: (
        Option<GpsTime>,
        Option<GpsPosition>,
        Option<u8>,
        Option<u8>,
        Option<f32>,
        Option<f32>,
        Option<&'a [u8]>,
        Option<f32>,
        Option<u16>,
    ),
) -> Result<GgaData, NmeaSentenceError<'a>> {
    Ok(GgaData {
        time: sentence.0,
        position: sentence.1,
        quality: translate_option!(sentence.2, GpsQuality),
        sats_in_view: sentence.3,
        hdop: sentence.4,
        altitude: sentence.5,
        geoid_altitude: invalid_height_check(sentence.6)?,
        age_of_differential: sentence.7,
        differential_station_id: sentence.8,
    })
}

//use nom::count;

named!(pub (crate) parse_gga<GgaData>,
    map_res!(
        do_parse!(
            time: opt!(complete!(parse_utc_stamp)) >>
            char!(',') >>
            position: alt!(
                complete!(nom::combinator::map(parse_gps_position, Some))
                | complete!(nom::combinator::value(
                    None,
                    nom::bytes::complete::tag(b",,,")
                ))
            )
            >>
            char!(',') >>
            quality: opt!(map_res!(take_until!(","), parse_num::<u8>)) >>
            char!(',') >>
            sats_in_view: opt!(map_res!(take_until!(","), parse_num::<u8>)) >>
            char!(',') >>
            hdop: opt!(map_res!(take_until!(","), parse_num::<f32>)) >>
            char!(',') >>
            altitude: opt!(map_res!(take_until!(","), parse_num::<f32>)) >>
            tag!(",M,") >>
            geoid_altitude: opt!(take_until!(",")) >>
            tag!(",M,") >>
            age_of_differential: opt!(map_res!(take_until!(","), parse_num::<f32>)) >>
            char!(',') >>
            differential_station_id: opt!(map_res!(take_until!("*"), parse_num::<u16>)) >>
            char!('*') >>
            (time, position, quality, sats_in_view, hdop, altitude, geoid_altitude, age_of_differential, differential_station_id)
        ),
        build_gga
    )
);


#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn parse() {
        let s = b"175929.860,,,,,0,00,25.5,,M,,M,,*";
        assert_eq!(
            parse_gga(s),
            Ok((
                &b""[..],
                GgaData {
                    time: Some(GpsTime { hour: 17, minute: 59, second: 29, millisecond: 860 }),
                    position: None,
                    quality: Some(GpsQuality::FixNotAvailable),
                    hdop: Some(25.5),
                    sats_in_view: Some(0),
                    geoid_altitude: None,
                    age_of_differential: None,
                    altitude: None,
                    differential_station_id: None,
                },
            ))
        )
    }  
}