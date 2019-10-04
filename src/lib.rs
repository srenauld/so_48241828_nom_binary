extern crate nom;

// Our preamble
const MAGIC:&[u8] = &[0x65, 0x51, 0x48, 0x54, 0x52];
// Our EOF byte sequence
const EOF:&[u8] = &[0xff];

// Shorthand to catch EOF
fn match_to_eof(data: &[u8]) -> nom::IResult<&[u8], &[u8]> {
    nom::bytes::complete::take_until(EOF)(data)
}

// Shorthand to catch the preamble
fn take_until_preamble(data: &[u8]) -> nom::IResult<&[u8], &[u8]> {
    nom::bytes::complete::take_until(MAGIC)(data)
}
pub fn extract_from_data(data: &[u8]) -> Option<(&[u8], &[u8])> {
    let preamble_parser = nom::sequence::preceded(
        // Ditch anything before the preamble
        take_until_preamble,
        nom::sequence::preceded(
            // Ditch the preamble
            nom::bytes::complete::tag(MAGIC),
            // And take until the EOF (0xff)
            match_to_eof
        )
    );
    // And we swap the elements because it's confusing AF
    // as a return function
    preamble_parser(data).ok().map(|r| {
        (r.1, r.0)
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn successfully_parses_first_block() {
        let data:Vec<u8> = vec![
            0x65, 0x51, 0x48, 0x54, 0x52,
            0x01, 0x02, 0x03, 0xff, 0x00
        ];
        let expected:Vec<u8> = vec![0x01, 0x02, 0x03];
        let remainder:Vec<u8> = vec![0xff, 0x00];
        let output:Option<_> = extract_from_data(&data);
        assert_eq!(output, Some((expected.as_slice(), remainder.as_slice())));
    }

        #[test]
    fn successfully_parses_subsequent_blocks() {
        let data:Vec<u8> = vec![
            0x65, 0x51, 0x48, 0x54, 0x52,
            0x01, 0x02, 0x03, 0xff, 0x00,
            0x65, 0x51, 0x48, 0x54, 0x52,
            0x01, 0x02, 0x03, 0xff, 0x00
        ];
        let mut blocks:Vec<&[u8]> = vec![];
        let mut current_state = data.as_slice();
        while let Some((block, remainder)) = extract_from_data(current_state) {
            blocks.push(block);
            current_state = remainder;
        }
        let catch = vec![0x01, 0x02, 0x03];
        assert_eq!(blocks.len(), 2);
        for i in blocks {
            assert_eq!(i, catch.as_slice());
        }
        //assert_eq!(output, Some(expected.as_slice()));
    }
}