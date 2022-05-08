use super::utils::*;
use crate::errors::NmeaSentenceError;
use crate::parse::*;

named!(pub (crate) parse_vtg<VtgData>,
    map_res!(
        do_parse!(
            bearing_true: opt!(map_res!(take_until!(","), parse_num::<f32>)) >>
            alt!(tag!(",T,") | tag!(",,")) >>
            bearing_magnetic: opt!(map_res!(take_until!(","), parse_num::<f32>)) >>
            alt!(tag!(",M,") | tag!(",,"))>>
            speed_knots: opt!(map_res!(take_until!(","), parse_num::<f32>)) >>
            alt!(tag!(",N,") | tag!(",,")) >>
            speed_kmh: opt!(map_res!(take_until!(","), parse_num::<f32>)) >>
            char!(',') >> opt!(char!('K')) >> opt!(char!(',')) >>
            mode: opt!(one_of!("ADEMSN")) >>
            char!('*') >>
            (bearing_true, bearing_magnetic, speed_knots, speed_kmh, mode)
        ),
        | sentence: (Option<f32>, Option<f32>, Option<f32>, Option<f32>, Option<char>)| -> Result<VtgData, NmeaSentenceError> {
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
                    speed_kmh: Some(0.0),
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