use super::utils::*;
use crate::parse::*;

use nom::combinator::opt;
use nom::character::complete::char;

pub(crate) fn parse(inp: &[u8]) -> Result<(&[u8], TxtData), nom::Err<nom::error::Error<&[u8]>>> {
    let (inp, total) = opt(nom::combinator::map_res(
        nom::bytes::complete::take_until(","),
        parse_num::<u8>,
    ))(inp)?;
    let (inp, _) = char(',')(inp)?;
    
    let (inp, sentence) = opt(nom::combinator::map_res(
        nom::bytes::complete::take_until(","),
        parse_num::<u8>,
    ))(inp)?;
    let (inp, _) = char(',')(inp)?;
    
    let (inp, id) = opt(nom::combinator::map_res(
        nom::bytes::complete::take_until(","),
        parse_num::<u8>,
    ))(inp)?;
    let (inp, _) = char(',')(inp)?;

    let (inp, text) = nom::bytes::complete::take_until("*")(inp)?;
    let (inp, _) = char('*')(inp)?;
    Ok((inp, TxtData {
        total,
        sentence,
        id,
        text,
    }))
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn t_parse() {
        let s = b"01,01,01,ANTENNA OPEN*";
        assert_eq!(
            parse(s),
            Ok((
                &b""[..],
                TxtData {
                    total: Some(1),
                    sentence: Some(1),
                    id: Some(1),
                    text: &b"ANTENNA OPEN"[..],
                },
            ))
        )
    }  
}
