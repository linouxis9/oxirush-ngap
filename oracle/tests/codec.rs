/*
   OxiRush
   Copyright 2025-2026 Valentin D'Emmanuele

   Licensed under the Apache License, Version 2.0 (the "License");
   you may not use this file except in compliance with the License.
   You may obtain a copy of the License at

   http://www.apache.org/licenses/LICENSE-2.0

   Unless required by applicable law or agreed to in writing, software
   distributed under the License is distributed on an "AS IS" BASIS,
   WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
   See the License for the specific language governing permissions and
   limitations under the License.
*/

//! The codecs that the oracle of the tests has in the place of those that rasn derives,
//! where rasn 0.29 does not write or read aligned PER as ITU-T X.691 has it: the
//! SEQUENCE, the list of a size constraint, and the strings whose length takes two
//! octets. Each is compared here with the derived codec of rasn, where the two are to
//! give the same, and with the octets of the clause where they are not.
//!
//! This file is the same in oxirush-ngap and oxirush-s1ap, as the oracle is but for
//! its bindings.

mod sequences {
    //! The same types twice, with the derived codec of rasn and with the codec of
    //! `sequence!`: each encoding rule writes the same for both, and each reads the
    //! same from the same octets.

    use oracle::rasn;

    /// The types of `derived`, with the codec of `sequence!`.
    mod direct {
        use oracle::rasn::{self, prelude::*};

        #[derive(AsnType, Debug, Clone, PartialEq, Eq)]
        #[rasn(automatic_tags)]
        #[non_exhaustive]
        pub struct Inner {
            pub flag: bool,
            #[rasn(value("0..=4095"))]
            pub number: Option<u16>,
        }
        oracle::per::sequence! { Inner extensible {
            flag: [bool] {},
            #[rasn(value("0..=4095"))]
            number: [Option<u16>] {? [u16] !},
        } }

        #[derive(AsnType, Debug, Clone, PartialEq, Eq)]
        #[rasn(automatic_tags)]
        pub struct Closed {
            #[rasn(size("3"))]
            pub octets: OctetString,
            pub inner: Option<Inner>,
        }
        oracle::per::sequence! { Closed closed {
            #[rasn(size("3"))]
            octets: [OctetString] {!},
            inner: [Option<Inner>] {? [Inner]},
        } }

        #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq)]
        #[rasn(choice, automatic_tags)]
        pub enum Either {
            Inner(Inner),
            Closed(Closed),
        }

        #[derive(AsnType, Debug, Clone, PartialEq, Eq)]
        #[rasn(automatic_tags)]
        #[non_exhaustive]
        pub struct Outer {
            #[rasn(identifier = "first-member")]
            pub first: Closed,
            #[rasn(size("0..=4"))]
            pub list: SequenceOf<Inner>,
            pub either: Option<Either>,
            #[rasn(size("1..=150"), identifier = "a-name")]
            pub name: Option<PrintableString>,
            pub last: u8,
        }
        oracle::per::sequence! { Outer extensible {
            #[rasn(identifier = "first-member")]
            first: [Closed] {},
            #[rasn(size("0..=4"))]
            list: [SequenceOf<Inner>] {!},
            either: [Option<Either>] {? [Either]},
            #[rasn(size("1..=150"), identifier = "a-name")]
            name: [Option<PrintableString>] {? [PrintableString] !},
            last: [u8] {},
        } }
    }

    /// The types of `direct`, with the derived codec of rasn.
    mod derived {
        use oracle::rasn::{self, prelude::*};

        #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq)]
        #[rasn(automatic_tags)]
        #[non_exhaustive]
        pub struct Inner {
            pub flag: bool,
            #[rasn(value("0..=4095"))]
            pub number: Option<u16>,
        }

        #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq)]
        #[rasn(automatic_tags)]
        pub struct Closed {
            #[rasn(size("3"))]
            pub octets: OctetString,
            pub inner: Option<Inner>,
        }

        #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq)]
        #[rasn(choice, automatic_tags)]
        pub enum Either {
            Inner(Inner),
            Closed(Closed),
        }

        #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq)]
        #[rasn(automatic_tags)]
        #[non_exhaustive]
        pub struct Outer {
            #[rasn(identifier = "first-member")]
            pub first: Closed,
            #[rasn(size("0..=4"))]
            pub list: SequenceOf<Inner>,
            pub either: Option<Either>,
            #[rasn(size("1..=150"), identifier = "a-name")]
            pub name: Option<PrintableString>,
            pub last: u8,
        }
    }

    /// Values of the types of a module: members that are there and that are not, at
    /// each depth, and a number and a size outside their constraints.
    macro_rules! samples {
        ($types:ident) => {{
            use oracle::rasn::prelude::*;
            use $types::{Closed, Either, Inner, Outer};
            let inner = |flag, number| Inner { flag, number };
            let closed = |octets: [u8; 3], inner| Closed {
                octets: OctetString::from(octets.to_vec()),
                inner,
            };
            let name = |name: &str| PrintableString::try_from(name).unwrap();
            vec![
                Outer {
                    first: closed([1, 2, 3], None),
                    list: vec![],
                    either: None,
                    name: None,
                    last: 0,
                },
                Outer {
                    first: closed([0xff, 0, 0x80], Some(inner(true, Some(4095)))),
                    list: vec![
                        inner(false, None),
                        inner(true, Some(0)),
                        inner(true, Some(7)),
                    ],
                    either: Some(Either::Inner(inner(true, Some(1)))),
                    name: Some(name("a name of a few letters")),
                    last: 255,
                },
                Outer {
                    first: closed([0, 0, 0], Some(inner(false, None))),
                    list: vec![inner(true, None); 4],
                    either: Some(Either::Closed(closed(
                        [9, 8, 7],
                        Some(inner(false, Some(256))),
                    ))),
                    name: Some(name(&"x".repeat(150))),
                    last: 1,
                },
                // A number outside its constraint, and more entries than a list has.
                Outer {
                    first: closed([1, 2, 3], Some(inner(true, Some(4096)))),
                    list: vec![],
                    either: None,
                    name: None,
                    last: 0,
                },
                Outer {
                    first: closed([1, 2, 3], None),
                    list: vec![inner(true, None); 5],
                    either: None,
                    name: None,
                    last: 0,
                },
            ]
        }};
    }

    #[test]
    fn a_sequence_is_written_as_the_derived_codec_writes_it() {
        let (direct, derived) = (samples!(direct), samples!(derived));
        let mut written = 0;
        for (direct, derived) in direct.iter().zip(&derived) {
            macro_rules! same {
                ($($codec:ident),*) => {$(
                    let ours = rasn::$codec::encode(direct).map_err(|error| error.to_string());
                    let theirs = rasn::$codec::encode(derived).map_err(|error| error.to_string());
                    assert_eq!(ours, theirs, "{} of {derived:?}", stringify!($codec));
                    written += usize::from(ours.is_ok());
                )*};
            }
            same!(aper, uper, ber, der, cer, oer, coer, jer, xer);
        }
        // Three of the values encode, and two are outside their constraints.
        assert!(written >= 3 * 9, "{written} encodings");
    }

    #[test]
    fn a_sequence_is_read_as_the_derived_codec_reads_it() {
        let values = samples!(derived);
        let mut read = 0;
        for value in &values {
            macro_rules! same {
                ($($codec:ident),*) => {$(
                    if let Ok(octets) = rasn::$codec::encode(value) {
                        // The octets, and each of their truncations.
                        for length in 0..=octets.len() {
                            let octets = &octets[..length];
                            let ours = rasn::$codec::decode::<direct::Outer>(octets);
                            let theirs = rasn::$codec::decode::<derived::Outer>(octets);
                            assert_eq!(
                                ours.as_ref().map(|value| format!("{value:?}")).ok(),
                                theirs.as_ref().map(|value| format!("{value:?}")).ok(),
                                "{} of {octets:02x?}",
                                stringify!($codec)
                            );
                            read += usize::from(ours.is_ok());
                        }
                    }
                )*};
            }
            same!(aper, uper, ber, der, cer, oer, coer, jer, xer);
        }
        assert!(read >= 3 * 9, "{read} values read");
    }

    /// A list of 1 to 65536 elements, with the codec of `sequence_of!`.
    #[derive(rasn::AsnType, Debug, PartialEq)]
    #[rasn(delegate)]
    struct Flags(Vec<bool>);
    oracle::per::sequence_of! { Flags, 1, 65536, unconstrained }

    #[test]
    fn a_list_in_fragments_is_read_as_the_sequence_codec_of_rasn_reads_it() {
        // A count of each form: what is written for it, then the same cut at each of
        // its counts and with another octet in the place of each.
        let mut read = 0;
        for count in [1, 127, 128, 16383, 16384, 16385, 49153, 65536] {
            let flags: Vec<bool> = (0..count).map(|at| at % 3 == 0).collect();
            let written = rasn::aper::encode(&flags).unwrap();
            assert_eq!(rasn::aper::encode(&Flags(flags)).unwrap(), written);
            // The first count, and the count after a fragment of 16384 flags or more.
            let fragment = usize::from(written[0] & 0x3f) * 2048;
            let after = (written[0] >= 0xc0).then_some(1 + fragment);
            let counts = [0, 1].into_iter().chain(after).chain([written.len() - 1]);
            let mut inputs = vec![written.clone()];
            for at in counts {
                inputs.push(written[..at].to_vec());
                for other in [0x00, 0x7f, 0x80, 0xc0, 0xc1, 0xc5] {
                    let mut changed = written.clone();
                    changed[at] = other;
                    inputs.push(changed);
                }
            }
            for octets in inputs {
                let ours = rasn::aper::decode::<Flags>(&octets).ok();
                let theirs = rasn::aper::decode::<Vec<bool>>(&octets).ok();
                // The size of the list is checked here alone.
                let theirs = theirs.filter(|flags| (1..=65536).contains(&flags.len()));
                assert_eq!(ours.map(|flags| flags.0), theirs, "{} octets", octets.len());
                read += usize::from(theirs.is_some());
            }
        }
        assert!(read > 50, "{read} lists read");
    }

    #[test]
    fn the_additions_of_a_later_release_are_skipped_in_both_variants_of_per() {
        // `Inner` with its extension bit set, no number and its flag, then one addition:
        // a count of one in seven bits, the bit of its presence, and an open type of one
        // octet, which starts an octet in the aligned variant.
        let aligned = [0b1010_0000, 0b0010_0000, 0x01, 0xaa];
        let read = rasn::aper::decode::<direct::Inner>(&aligned).unwrap();
        assert_eq!(
            read,
            direct::Inner {
                flag: true,
                number: None
            }
        );
        let unaligned = [0b1010_0000, 0b0010_0000, 0b0011_0101, 0b0100_0000];
        let read = rasn::uper::decode::<direct::Inner>(&unaligned).unwrap();
        assert_eq!(
            read,
            direct::Inner {
                flag: true,
                number: None
            }
        );
    }
}

mod sized_strings {
    use oracle::rasn::{self, prelude::*};
    use oracle::sized::{SizedBitString, SizedOctetString};

    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq)]
    struct Octets {
        flag: bool,
        value: SizedOctetString<1, 1000>,
    }

    #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq)]
    struct Bits {
        flag: bool,
        value: SizedBitString<1, 1024, true>,
    }

    fn bits(bytes: &[u8], length: usize) -> BitString {
        let mut bits = BitString::from_slice(bytes);
        bits.truncate(length);
        bits
    }

    /// The two-octet length and the octets both start on an octet boundary
    /// (X.691 (02/2021) §11.5.7.3, §17.8), as pycrate and asn1tools encode it.
    #[test]
    fn octets_have_an_aligned_two_octet_length() {
        let value = Octets {
            flag: true,
            value: vec![0x11, 0x22, 0x33].into(),
        };
        let wire = rasn::aper::encode(&value).unwrap();
        assert_eq!(wire, [0x80, 0x00, 0x02, 0x11, 0x22, 0x33]);
        assert_eq!(rasn::aper::decode::<Octets>(&wire).unwrap(), value);
    }

    /// The extension bit, the aligned two-octet length, then the bits
    /// (X.691 (02/2021) §16.6, §16.11).
    #[test]
    fn bits_have_an_aligned_two_octet_length() {
        for (value, wire) in [
            (bits(&[0xa0], 3), vec![0x80, 0x00, 0x02, 0xa0]),
            (
                bits(&[0xab, 0xcd, 0xe0], 20),
                vec![0x80, 0x00, 0x13, 0xab, 0xcd, 0xe0],
            ),
        ] {
            let value = Bits {
                flag: true,
                value: value.into(),
            };
            assert_eq!(rasn::aper::encode(&value).unwrap(), wire);
            assert_eq!(rasn::aper::decode::<Bits>(&wire).unwrap(), value);
        }
    }

    /// A length outside the extensible root is unconstrained (§16.6).
    #[test]
    fn bits_outside_the_root_have_an_unconstrained_length() {
        let value = Bits {
            flag: true,
            value: BitString::repeat(true, 1025).into(),
        };
        let wire = rasn::aper::encode(&value).unwrap();
        assert_eq!(&wire[..3], [0xc0, 0x84, 0x01]);
        assert_eq!(rasn::aper::decode::<Bits>(&wire).unwrap(), value);
    }
}
