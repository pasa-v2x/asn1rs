use asn1rs::prelude::*;

#[asn(sequence)]
#[derive(Debug, Default, PartialOrd, PartialEq)]
pub struct Potato {
    #[asn(integer)]
    size: u64,
    #[asn(integer(min..max))]
    size2: u64,
    #[asn(integer(12..128), tag(APPLICATION(4)))]
    size3: u8,
    #[asn(utf8string, tag(4))]
    string: String,
}

#[test]
fn test_compiles() {
    let _p = Potato {
        size: 123,
        size2: 1234,
        size3: 234,
        string: String::from("where is the content"),
    };
}

#[test]
fn test_serialize_with_uper() {
    let p = Potato {
        size: 123,
        size2: 1234,
        size3: 128,
        string: String::from("where is the content"),
    };
    let mut uper = UperWriter::default();
    uper.write(&p).unwrap();
    assert_eq!(
        &[
            // https://asn1.io/asn1playground/
            0x01, 0x7B, 0x02, 0x04, 0xD2, 0xE8, 0x28, 0xEE, 0xD0, 0xCA, 0xE4, 0xCA, 0x40, 0xD2,
            0xE6, 0x40, 0xE8, 0xD0, 0xCA, 0x40, 0xC6, 0xDE, 0xDC, 0xE8, 0xCA, 0xDC, 0xE8
        ],
        uper.byte_content()
    );
    assert_eq!(26 * 8 + 7, uper.bit_len());
}

#[test]
fn test_deserialize_with_uper() {
    let mut uper = UperReader::from_bits(
        vec![
            // https://asn1.io/asn1playground/
            0x01, 0x7B, 0x02, 0x04, 0xD2, 0xE8, 0x28, 0xEE, 0xD0, 0xCA, 0xE4, 0xCA, 0x40, 0xD2,
            0xE6, 0x40, 0xE8, 0xD0, 0xCA, 0x40, 0xC6, 0xDE, 0xDC, 0xE8, 0xCA, 0xDC, 0xE8,
        ],
        26 * 8 + 7,
    );
    let p = uper.read::<Potato>().unwrap();
    assert_eq!(
        Potato {
            size: 123,
            size2: 1234,
            size3: 128,
            string: String::from("where is the content"),
        },
        p
    );
}

#[asn(enumerated)]
#[derive(Debug, PartialOrd, PartialEq)]
pub enum Topping {
    NotPineapple,
    EvenLessPineapple,
    NoPineappleAtAll,
}

#[test]
fn topping_test_serialize_with_uper() {
    let mut uper = UperWriter::default();
    uper.write(&Topping::NotPineapple).unwrap();
    uper.write(&Topping::EvenLessPineapple).unwrap();
    uper.write(&Topping::NoPineappleAtAll).unwrap();
    assert_eq!(&[0x00 | 0x40 >> 2 | 0x80 >> 4], uper.byte_content());
    assert_eq!(6, uper.bit_len());
}

#[test]
fn topping_test_deserialize_with_uper() {
    let mut uper = UperReader::from_bits(vec![0x00_u8 | 0x40 >> 2 | 0x80 >> 4], 6);
    assert_eq!(Topping::NotPineapple, uper.read::<Topping>().unwrap());
    assert_eq!(Topping::EvenLessPineapple, uper.read::<Topping>().unwrap());
    assert_eq!(Topping::NoPineappleAtAll, uper.read::<Topping>().unwrap());
}

#[asn(sequence)]
#[derive(Debug, PartialOrd, PartialEq)]
pub struct Pizza {
    #[asn(integer(1..4))]
    size: u8,
    #[asn(complex(Topping))]
    topping: Topping,
}

#[test]
fn pizza_test_uper_1() {
    let mut uper = UperWriter::default();
    let pizza = Pizza {
        size: 2,
        topping: Topping::NotPineapple,
    };
    uper.write(&pizza).unwrap();
    // https://asn1.io/asn1playground/
    assert_eq!(&[0x40], uper.byte_content());
    assert_eq!(4, uper.bit_len());
    let mut uper = uper.into_reader();
    assert_eq!(pizza, uper.read::<Pizza>().unwrap());
    assert_eq!(0, uper.bits_remaining());
}

#[test]
fn pizza_test_uper_2() {
    let mut uper = UperWriter::default();
    let pizza = Pizza {
        size: 1,
        topping: Topping::NoPineappleAtAll,
    };
    uper.write(&pizza).unwrap();
    // https://asn1.io/asn1playground/
    assert_eq!(&[0x20], uper.byte_content());
    assert_eq!(4, uper.bit_len());
    let mut uper = uper.into_reader();
    assert_eq!(pizza, uper.read::<Pizza>().unwrap());
    assert_eq!(0, uper.bits_remaining());
}

#[test]
fn pizza_test_uper_3() {
    let mut uper = UperWriter::default();
    let pizza = Pizza {
        size: 3,
        topping: Topping::EvenLessPineapple,
    };
    uper.write(&pizza).unwrap();
    // https://asn1.io/asn1playground/
    assert_eq!(&[0x90], uper.byte_content());
    assert_eq!(4, uper.bit_len());
    let mut uper = uper.into_reader();
    assert_eq!(pizza, uper.read::<Pizza>().unwrap());
    assert_eq!(0, uper.bits_remaining());
}

#[asn(choice)]
#[derive(Debug, PartialOrd, PartialEq)]
pub enum WhatToEat {
    #[asn(complex(Potato))]
    Potato(Potato),
    #[asn(complex(Pizza))]
    Pizza(Pizza),
}

#[test]
fn what_to_eat_test_uper_1() {
    let mut uper = UperWriter::default();
    let what = WhatToEat::Pizza(Pizza {
        size: 3,
        topping: Topping::EvenLessPineapple,
    });
    uper.write(&what).unwrap();
    // https://asn1.io/asn1playground/
    assert_eq!(&[0xC8], uper.byte_content());
    assert_eq!(5, uper.bit_len());
    let mut uper = uper.into_reader();
    assert_eq!(what, uper.read::<WhatToEat>().unwrap());
    assert_eq!(0, uper.bits_remaining());
}

#[test]
fn what_to_eat_test_uper_2() {
    let mut uper = UperWriter::default();
    let what = WhatToEat::Potato(Potato {
        size: 13,
        size2: 37,
        size3: 42,
        string: "such tasty potato".to_string(),
    });
    uper.write(&what).unwrap();
    // https://asn1.io/asn1playground/
    assert_eq!(
        &[
            0x00, 0x86, 0x80, 0x92, 0x9E, 0x11, 0x73, 0x75, 0x63, 0x68, 0x20, 0x74, 0x61, 0x73,
            0x74, 0x79, 0x20, 0x70, 0x6F, 0x74, 0x61, 0x74, 0x6F
        ],
        uper.byte_content()
    );
    assert_eq!(23 * 8, uper.bit_len());
    let mut uper = uper.into_reader();
    assert_eq!(what, uper.read::<WhatToEat>().unwrap());
    assert_eq!(0, uper.bits_remaining());
}

/*
BasicSchema DEFINITIONS AUTOMATIC TAGS ::=
BEGIN
  Potato ::= SEQUENCE {
    size INTEGER,
    size2 INTEGER,
    size3 INTEGER(12..128),
    string Utf8String
  }

  Topping ::= ENUMERATED
  {
    not_pineapple,
    even_less_pineapple,
    no_pineapple_at_all
  }

  Pizza ::= SEQUENCE {
    size INTEGER(1..4),
    topping Topping
  }

  WhatToEat ::= CHOICE {
    potato Potato,
    pizza Pizza
  }
END

*/

#[asn(sequence)]
#[derive(Debug, PartialOrd, PartialEq)]
pub struct AreWeBinaryYet {
    #[asn(octet_string)]
    binary: Vec<u8>,
}

#[test]
fn are_we_binary_yet_uper() {
    let mut uper = UperWriter::default();
    let are_we = AreWeBinaryYet {
        binary: vec![0x13, 0x37],
    };
    uper.write(&are_we).unwrap();
    // https://asn1.io/asn1playground/
    assert_eq!(&[02, 0x13, 0x37], uper.byte_content());
    assert_eq!(3 * 8, uper.bit_len());
    let mut uper = uper.into_reader();
    assert_eq!(are_we, uper.read::<AreWeBinaryYet>().unwrap());
    assert_eq!(0, uper.bits_remaining());
}

#[asn(sequence)]
#[derive(Debug, PartialOrd, PartialEq)]
pub struct Optional {
    #[asn(option(integer))]
    value: Option<u64>,
}

#[test]
fn test_optional_uper() {
    let mut uper = UperWriter::default();
    let v = Optional { value: Some(1337) };
    uper.write(&v).unwrap();
    // https://asn1.io/asn1playground/
    assert_eq!(&[0x81, 0x02, 0x9C, 0x80], uper.byte_content());
    assert_eq!(3 * 8 + 1, uper.bit_len());
    let mut uper = uper.into_reader();
    assert_eq!(v, uper.read::<Optional>().unwrap());
    assert_eq!(0, uper.bits_remaining());
}

#[asn(sequence)]
#[derive(Debug, PartialOrd, PartialEq)]
pub struct CrazyList {
    #[asn(sequence_of(option(option(sequence_of(integer)))))]
    values: Vec<Option<Option<Vec<u64>>>>,
}

#[test]
fn test_crazy_list_println() {
    let mut writer = PrintlnWriter::default();
    let list = CrazyList {
        values: vec![Some(Some(vec![13])), Some(Some(vec![37])), Some(None), None],
    };
    // Prints something like
    //
    // Writing sequence CrazyList
    //  Writing sequence-of (MIN..MAX)
    //   Writing OPTIONAL
    //    Some
    //     Writing OPTIONAL
    //      Some
    //       Writing sequence-of (MIN..MAX)
    //        WRITING Integer 13
    //   Writing OPTIONAL
    //    Some
    //     Writing OPTIONAL
    //      Some
    //       Writing sequence-of (MIN..MAX)
    //        WRITING Integer 37
    //   Writing OPTIONAL
    //    Some
    //     Writing OPTIONAL
    //      None
    //   Writing OPTIONAL
    //    None
    list.write(&mut writer).unwrap();
}

#[test]
fn test_crazy_list_uper() {
    let mut uper = UperWriter::default();
    let list = CrazyList {
        values: vec![Some(Some(vec![13])), Some(Some(vec![37])), Some(None), None],
    };
    uper.write(&list).unwrap();
    assert_eq!(
        &[
            // from analytic, I hate myself for it and I am sorry to everyone that needs to adjust this
            //           ...well... probably myself in the future... so self.await ... hehe ...
            // -- 0
            0x04, // 4 elements in the list
            // -- 1
            0b11 << 6 // first element: Some, Some
                | 0x01 >> 2, // length of inner list, part 1
            // -- 2
            0x01 << 6 // length of inner list, part2
                | 0x01 >> 2, // length of integer, part 1
            // -- 3
            0x01 << 6 // length of integer, part 2
                | (13 >> 2), // value of integer, part 1
            // -- 4
            13 << 6 // value of integer, part 2, end of element
                | 0b11 << 4 // second element: Some, Some
                | 0x01 >> 4, // length of inner list, part 1
            // -- 5
            0x01 << 4 // length of inner list, part 2
                | 0x01 >> 4, // length of integer, part 1
            // -- 6
            0x01 << 4 // length of integer, part 2
                | 37 >> 4, // value of integer, part 1
            // -- 7
            37 << 4 // value of integer, part 2, end of element
                | 0b10 << 2 // third element: Some, None
                | 0b0 << 1 // fourth element: None
        ],
        uper.byte_content()
    );
    assert_eq!(7 * 8 + 7, uper.bit_len());
    let mut uper = uper.into_reader();
    assert_eq!(list, uper.read::<CrazyList>().unwrap());
    assert_eq!(0, uper.bits_remaining());
}

#[asn(transparent)]
#[derive(Debug, PartialOrd, PartialEq)]
pub struct FlatList(#[asn(sequence_of(integer))] Vec<u64>);

#[test]
fn test_flat_list_println() {
    // Writing sequence FlatList
    //  Writing sequence-of (MIN..MAX)
    //   WRITING Integer 13
    //   WRITING Integer 37
    //   WRITING Integer 42
    PrintlnWriter::default()
        .write(&FlatList(vec![13, 37, 42]))
        .unwrap();
}

#[test]
fn test_flat_list_uper() {
    let mut uper = UperWriter::default();
    let v = FlatList(vec![13, 37, 42]);
    uper.write(&v).unwrap();
    // https://asn1.io/asn1playground/
    assert_eq!(
        &[0x03, 0x01, 0x0D, 0x01, 0x25, 0x01, 0x2A],
        uper.byte_content()
    );
    assert_eq!(7 * 8, uper.bit_len());
    let mut uper = uper.into_reader();
    assert_eq!(v, uper.read::<FlatList>().unwrap());
    assert_eq!(0, uper.bits_remaining());
}

#[asn(transparent)]
#[derive(Debug, PartialOrd, PartialEq)]
pub struct Important(#[asn(option(integer))] Option<u64>);

#[test]
fn test_transparent_important_println() {
    // Writing sequence FlatList
    //  Writing sequence-of (MIN..MAX)
    //   WRITING Integer 13
    //   WRITING Integer 37
    //   WRITING Integer 42
    PrintlnWriter::default()
        .write(&Important(Some(42)))
        .unwrap();
}

#[test]
fn test_transparent_important_uper_some() {
    let mut uper = UperWriter::default();
    let v = Important(Some(42));
    uper.write(&v).unwrap();
    // invalid according to https://asn1.io/asn1playground/
    // but who cares... :P
    assert_eq!(
        &[
            // --- 0
            0b1 << 7 // Some
                | 0x01 >> 1, // length of the integer, part 1
            // --- 1
            0x01 << 7 // length of the integer, part 2
                | 42 >> 1, // value of the  integer, part 1
            // --- 2
            42 << 7 // value of the integer, part 2
        ],
        uper.byte_content()
    );

    assert_eq!(2 * 8 + 1, uper.bit_len());
    let mut uper = uper.into_reader();
    assert_eq!(v, uper.read::<Important>().unwrap());
    assert_eq!(0, uper.bits_remaining());
}

#[test]
fn test_transparent_important_uper_none() {
    let mut uper = UperWriter::default();
    let v = Important(None);
    uper.write(&v).unwrap();
    // invalid according to https://asn1.io/asn1playground/
    // but who cares... :P
    assert_eq!(&[0b0 << 7], uper.byte_content());

    assert_eq!(1, uper.bit_len());
    let mut uper = uper.into_reader();
    assert_eq!(v, uper.read::<Important>().unwrap());
    assert_eq!(0, uper.bits_remaining());
}

#[asn(sequence)]
#[derive(Debug, Default, PartialOrd, PartialEq)]
pub struct BoolContainer {
    #[asn(boolean)]
    bool1: bool,
    #[asn(boolean)]
    bool2: bool,
    #[asn(boolean)]
    bool3: bool,
}

#[test]
fn test_bool_container_uper() {
    let mut uper = UperWriter::default();
    let v = BoolContainer {
        bool1: false,
        bool2: true,
        bool3: true,
    };
    uper.write(&v).unwrap();
    assert_eq!(&[0b011_0_0000], uper.byte_content());
    assert_eq!(3, uper.bit_len());

    let mut uper = uper.into_reader();
    assert_eq!(v, uper.read::<BoolContainer>().unwrap());
    assert_eq!(0, uper.bits_remaining());
}

#[asn(transparent)]
#[derive(Debug, Default, PartialOrd, PartialEq)]
pub struct NegativeRangeMin(#[asn(integer(- 12..12))] i8);
