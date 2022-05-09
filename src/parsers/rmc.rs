use super::utils::*;
use crate::errors::NmeaSentenceError;
use crate::parse::*;

fn build_rmc<'a>(
    sentence: (
        Option<GpsTime>,
        Option<char>,
        Option<GpsPosition>,
        Option<(u16, u8)>,
        Option<f32>,
        Option<GpsDate>,
        Option<f32>,
        Option<char>,
    ),
) -> Result<RmcData, NmeaSentenceError<'a>> {
    Ok(RmcData {
        time: sentence.0,
        status: translate_option!(sentence.1, RmStatus),
        position: sentence.2,
        speed_knots: sentence.3,
        heading: sentence.4,
        date: sentence.5,
        magnetic_variation: sentence.6,
        magnetic_direction: translate_option!(sentence.7, LongitudeDirection),
    })
}

named!(pub (crate) parse_rmc<RmcData>,
    map_res!(
        do_parse!(
            time: opt!(complete!(parse_utc_stamp)) >>
            char!(',') >>
            status: opt!(one_of!("AVP")) >>
            char!(',') >>
            position: opt!(complete!(parse_gps_position)) >>
            char!(',') >>
            speed_knots: opt!(map_res!(take_until!(","), parse_speed)) >>
            char!(',') >>
            heading: opt!(map_res!(take_until!(","), parse_num::<f32>)) >>
            char!(',') >>
            date: opt!(complete!(parse_date)) >>
            char!(',') >>
            magnetic_variation: opt!(map_res!(take_until!(","), parse_num::<f32>)) >>
            char!(',') >>
            magnetic_direction: opt!(one_of!("EW")) >>
            // AT6558 returns two etra fields, see tests
            take_until!("*") >>
            char!('*') >>
            (time, status, position, speed_knots, heading, date, magnetic_variation, magnetic_direction)
        ),
        build_rmc
    )
);

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn parse() {
        let s = b"163428.000,A,0053.9,N,00002.33,E,0.58,0.00,080522,,,E,V*";
        assert_eq!(
            parse_rmc(s),
            Ok((
                &b""[..],
                RmcData { time: Some(GpsTime { hour: 16, minute: 34,
                second: 28, millisecond: 0 }), status: Some(RmStatus::Active), position: Some(GpsPosition { lat: 0.8983334, lat_dir: LatitudeDirection::North, lon: 0.03883333, lon_dir: LongitudeDirection::East }), speed_knots: Some((0, 58)),
                heading: Some(0.0), date: Some(GpsDate { day: 8, month: 5, year: 22 }), magnetic_variation: None, magnetic_direction: None },
            ))
        )
        
    }
}