extern crate nom;

const MAGIC:[u8;5] = [0x65, 0x51, 0x48, 0x54, 0x52];

pub fn extract_from_data(data: &[u8]) -> Option<&[u8]> {
    None
}
#[cfg(test)]
mod tests {
    use super::*;

    const DATA:[u8;10] = [0x65, 0x51, 0x48, 0x54, 0x52, 0x01, 0x02, 0x03, 0xff, 0x00];
    #[test]
    fn successfully_parses_first_block() {
        let expected:&[u8] = &[0x01, 0x02, 0x03];
        let output:Option<&[u8]> = extract_from_data(&DATA);
        assert_eq!(output, Some(expected));
    }
}