//! Run this file with `cargo test --test 05_srl_validator`.

// TODO: Implement a SRL (Simple Resource Locator) validator.
// A SRL consists of two parts, an optional protocol (string) and an address (string).
// The format of the SRL looks like this: `[<protocol>://]<address>`
// The protocol and the address have to contain only lowercase English characters.
// Protocol must not be empty if :// is present in the SRL.
// Address must not be empty.
//
// As an example, these are valid SRLs:
// - `http://foo`
// - `bar://baz`
// - `foobar`
//
// And these are invalid SRLs:
// - `http://foo1` (invalid character in address)
// - `asd://bar://` (invalid character in address)
// - `://baz` (empty protocol)
// - `01://baz` (invalid character in protocol)
//
// Create a struct `SRL` in a module named `srl`. Expose functions for parsing a SRL and getting
// its individual parts, but do not allow modifying the fields of `SRL` outside its module.
// Do not use regular expressions, SRLs can be easily parsed with a big of parsing logic.
//
// Hint: Put `#[derive(Debug, Eq, PartialEq)]` on top of `SRL` and `SRLValidationError`,
// so that asserts in tests work.

mod srl {
    #[derive(Debug, Eq, PartialEq)]
    pub struct Srl {
        protocol: Option<String>,
        address: String,
    }

    #[derive(Debug, Eq, PartialEq)]
    pub enum SRLValidationError {
        EmptyProtocol,
        EmptyAddress,
        InvalidCharacterInProtocol(char),
        InvalidCharacterInAddress(char),
    }

    impl Srl {
        pub fn new(srl: &str) -> Result<Self, SRLValidationError> {
            if srl.is_empty() {
                return Err(SRLValidationError::EmptyAddress);
            }

            let Some((protocol, address)) = srl.split_once("://") else {
                if srl.chars().any(|c| !c.is_ascii_lowercase()) {
                    return Err(SRLValidationError::InvalidCharacterInAddress(
                        srl.chars().find(|c| !c.is_ascii_lowercase()).unwrap(),
                    ));
                }
                return Ok(Self {
                    protocol: None,
                    address: srl.to_string(),
                });
            };

            if protocol.is_empty() {
                return Err(SRLValidationError::EmptyProtocol);
            }

            if protocol.chars().any(|c| !c.is_ascii_lowercase()) {
                return Err(SRLValidationError::InvalidCharacterInProtocol(
                    protocol.chars().find(|c| !c.is_ascii_lowercase()).unwrap(),
                ));
            }

            if address.chars().any(|c| !c.is_ascii_lowercase()) {
                return Err(SRLValidationError::InvalidCharacterInAddress(
                    address.chars().find(|c| !c.is_ascii_lowercase()).unwrap(),
                ));
            }

            Ok(Self {
                protocol: Some(protocol.to_string()),
                address: address.to_string(),
            })
        }

        pub fn get_protocol(&self) -> Option<&str> {
            self.protocol.as_deref()
        }

        pub fn get_address(&self) -> &str {
            &self.address
        }
    }
}

/// Below you can find a set of unit tests.
#[cfg(test)]
mod tests {
    use super::srl::{SRLValidationError, Srl};

    #[test]
    fn empty_address() {
        assert_eq!(Srl::new(""), Err(SRLValidationError::EmptyAddress));
    }

    #[test]
    fn only_separator() {
        assert_eq!(Srl::new("://"), Err(SRLValidationError::EmptyProtocol));
    }

    #[test]
    fn empty_protocol() {
        assert_eq!(Srl::new("://foo"), Err(SRLValidationError::EmptyProtocol));
    }

    #[test]
    fn multiple_protocols() {
        assert_eq!(
            Srl::new("ab://bc://foo"),
            Err(SRLValidationError::InvalidCharacterInAddress(':'))
        );
    }

    #[test]
    fn invalid_protocol() {
        assert_eq!(
            Srl::new("bAc://foo"),
            Err(SRLValidationError::InvalidCharacterInProtocol('A'))
        );
        assert_eq!(
            Srl::new("a02://foo"),
            Err(SRLValidationError::InvalidCharacterInProtocol('0'))
        );
    }

    #[test]
    fn invalid_address_with_protocol() {
        assert_eq!(
            Srl::new("abc://fo1o"),
            Err(SRLValidationError::InvalidCharacterInAddress('1'))
        );
        assert_eq!(
            Srl::new("bar://fooBZcX"),
            Err(SRLValidationError::InvalidCharacterInAddress('B'))
        );
    }

    #[test]
    fn invalid_address_without_protocol() {
        assert_eq!(
            Srl::new("fo1o"),
            Err(SRLValidationError::InvalidCharacterInAddress('1'))
        );
        assert_eq!(
            Srl::new("fooBAc"),
            Err(SRLValidationError::InvalidCharacterInAddress('B'))
        );
    }

    #[test]
    fn invalid_protocol_and_address() {
        assert_eq!(
            Srl::new("bZcA://fo2o"),
            Err(SRLValidationError::InvalidCharacterInProtocol('Z'))
        );
        assert_eq!(
            Srl::new("a20://barBAZ"),
            Err(SRLValidationError::InvalidCharacterInProtocol('2'))
        );
    }

    #[test]
    fn invalid_char_emoji() {
        assert_eq!(
            Srl::new("asd://fo🙃o"),
            Err(SRLValidationError::InvalidCharacterInAddress('🙃'))
        );
    }

    #[test]
    fn no_protocol() {
        let srl = Srl::new("foobar").unwrap();
        assert_eq!(srl.get_protocol(), None);
        assert_eq!(srl.get_address(), "foobar");
    }

    #[test]
    fn protocol_and_scheme() {
        let srl = Srl::new("bar://foobar").unwrap();
        assert_eq!(srl.get_protocol(), Some("bar"));
        assert_eq!(srl.get_address(), "foobar");
    }
}
