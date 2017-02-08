extern crate multibase;

use multibase::*;

#[test]
fn test_bases_code() {
    assert_eq!(Base2.code(), '0');
    assert_eq!(Base32hexUpper.code(), 'V');
}

#[test]
fn test_round_trip() {
    let slices: &[&[u8]] = &[
        b"helloworld",
        b"we all want decentralization",
        b"zdj7WfBb6j58iSJuAzDcSZgy2SxFhdpJ4H87uvMpfyN6hRGyH",
    ];

    for s in slices {
        assert_eq!(
            decode(encode(Base58btc, s)).unwrap(),
            (Base58btc, s.to_vec())
        );
    }

    let val = vec![1, 2, 3, 98, 255, 255, 255];
    assert_eq!(
        decode(encode(Base64url, &val)).unwrap(),
        (Base64url, val)
    )
}

#[test]
fn test_bases_from_code() {
    assert_eq!(Base::from_code('0').unwrap(), Base2);
    assert_eq!(Base::from_code('V').unwrap(), Base32hexUpper);
}

#[test]
fn test_encode() {
    let id = b"Decentralize everything!!";

    assert_eq!(encode(Base16, id),
               "f446563656e7472616c697a652065766572797468696e672121");

    assert_eq!(encode(Base16, String::from_utf8(id.to_vec()).unwrap()),
               "f446563656e7472616c697a652065766572797468696e672121");

    assert_eq!(encode(Base16, id.to_vec()),
               "f446563656e7472616c697a652065766572797468696e672121");

    assert_eq!(encode(Base58btc, id),
               "zUXE7GvtEk8XTXs1GF8HSGbVA9FCX9SEBPe");

    let id2 = b"yes mani !";

    assert_eq!(encode(Base2, id2),
               "01111001011001010111001100100000011011010110000101101110011010010010000000100\
                001");
    assert_eq!(encode(Base8, id2), "7171312714403326055632220041");
    assert_eq!(encode(Base10, id2), "9573277761329450583662625");
    assert_eq!(encode(Base16, id2), "f796573206d616e692021");
    assert_eq!(encode(Base32hex, id2), "vf5in683dc5n6i811");
    assert_eq!(encode(Base32, id2), "bpfsxgidnmfxgsibb");
    assert_eq!(encode(Base32z, id2), "hxf1zgedpcfzg1ebb");
    assert_eq!(encode(Base58flickr, id2), "Z7Pznk19XTTzBtx");
    assert_eq!(encode(Base58btc, id2), "z7paNL19xttacUY");
}

#[test]
fn test_decode() {
    let id = b"Decentralize everything!!";

    assert_eq!(decode("f446563656e7472616c697a652065766572797468696e672121").unwrap(),
               (Base16, id.to_vec()));

    assert_eq!(decode("f446563656e7472616c697a652065766572797468696e672121".to_string()).unwrap(),
               (Base16, id.to_vec()));

    assert_eq!(decode("zUXE7GvtEk8XTXs1GF8HSGbVA9FCX9SEBPe").unwrap(),
               (Base58btc, id.to_vec()));

    let id2 = b"yes mani !";

    assert_eq!(decode("011110010110010101110011001000000110110101100001011011100110100100100\
                       00000100001")
               .unwrap(),
               (Base2, id2.to_vec()));
    assert_eq!(decode("7171312714403326055632220041").unwrap(),
               (Base8, id2.to_vec()));
    assert_eq!(decode("9573277761329450583662625").unwrap(),
               (Base10, id2.to_vec()));
    assert_eq!(decode("f796573206d616e692021").unwrap(),
               (Base16, id2.to_vec()));
    assert_eq!(decode("vf5in683dc5n6i811").unwrap(),
               (Base32hex, id2.to_vec()));
    assert_eq!(decode("bpfsxgidnmfxgsibb").unwrap(),
               (Base32, id2.to_vec()));
    assert_eq!(decode("hxf1zgedpcfzg1ebb").unwrap(),
               (Base32z, id2.to_vec()));
    assert_eq!(decode("Z7Pznk19XTTzBtx").unwrap(),
               (Base58flickr, id2.to_vec()));
    assert_eq!(decode("z7paNL19xttacUY").unwrap(),
               (Base58btc, id2.to_vec()));

    // Fails
    assert_eq!(decode("Lllll"), Err(Error::UnkownBase));
    assert_eq!(decode("Ullll"), Err(Error::UnkownBase));

    assert_eq!(decode("z7pa_L19xttacUY"), Err(Error::InvalidBaseString))
}
