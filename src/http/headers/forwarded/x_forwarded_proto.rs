use crate::http::headers::{self, Header};
use crate::http::{HeaderName, HeaderValue};
use crate::net::Protocol;

/// The X-Forwarded-Proto (XFP) header is a de-facto standard header for
/// identifying the protocol (HTTP or HTTPS) that a client used to connect to your proxy or load balancer.
///
/// Your server access logs contain the protocol used between the server and the load balancer,
/// but not the protocol used between the client and the load balancer. To determine the protocol
/// used between the client and the load balancer, the X-Forwarded-Proto request header can be used.
///
/// It is recommended to use the [`Forwarded`](super::Forwarded) header instead if you can.
///
/// More info can be found at <https://developer.mozilla.org/en-US/docs/Web/HTTP/Headers/X-Forwarded-Proto>.
///
/// # Syntax
///
/// ```text
/// X-Forwarded-Proto: <protocol>
/// ```
///
/// # Example values
///
/// * `https`
/// * `http`
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct XForwardedProto(Protocol);

impl Header for XForwardedProto {
    fn name() -> &'static HeaderName {
        &crate::http::header::X_FORWARDED_PROTO
    }

    fn decode<'i, I: Iterator<Item = &'i HeaderValue>>(
        values: &mut I,
    ) -> Result<Self, headers::Error> {
        Ok(XForwardedProto(
            values
                .next()
                .and_then(|value| value.to_str().ok().and_then(|s| s.parse().ok()))
                .ok_or_else(crate::http::headers::Error::invalid)?,
        ))
    }

    fn encode<E: Extend<HeaderValue>>(&self, values: &mut E) {
        let s = self.0.to_string();
        values.extend(Some(HeaderValue::from_str(&s).unwrap()))
    }
}

impl XForwardedProto {
    /// Get a reference to the [`Protocol`] of this [`XForwardedProto`].
    pub fn protocol(&self) -> &Protocol {
        &self.0
    }

    /// Consume this [`Header`] into the inner data ([`Protocol`]).
    pub fn into_protocol(self) -> Protocol {
        self.0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    use http::HeaderValue;

    macro_rules! test_header {
        ($name: ident, $input: expr, $expected: expr) => {
            #[test]
            fn $name() {
                assert_eq!(
                    XForwardedProto::decode(
                        &mut $input
                            .into_iter()
                            .map(|s| HeaderValue::from_bytes(s.as_bytes()).unwrap())
                            .collect::<Vec<_>>()
                            .iter()
                    )
                    .ok(),
                    $expected,
                );
            }
        };
    }

    // Tests from the Docs
    test_header!(test1, vec!["https"], Some(XForwardedProto(Protocol::Https)));
    test_header!(
        test2,
        // 2nd one gets ignored
        vec!["https", "http"],
        Some(XForwardedProto(Protocol::Https))
    );
    test_header!(test3, vec!["http"], Some(XForwardedProto(Protocol::Http)));

    #[test]
    fn test_x_forwarded_proto_symmetric_encoder() {
        for input in [Protocol::Http, Protocol::Https] {
            let input = XForwardedProto(input);
            let mut values = Vec::new();
            input.encode(&mut values);
            let output = XForwardedProto::decode(&mut values.iter()).unwrap();
            assert_eq!(input, output);
        }
    }
}
