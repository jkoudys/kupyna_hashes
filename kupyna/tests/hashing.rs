// Make sure to include the hex_literal crate in your Cargo.toml:
// [dependencies]
// hex_literal = "0.3"

use hex_literal::hex;
use kupyna::{Digest, Kupyna256, Kupyna48, Kupyna512, Kupyna384};

/// Returns the “test” message bytes – 256 bytes from 0x00 to 0xFF.
fn test_message() -> [u8; 256] {
    // This is the same as the C array: 0x00, 0x01, …, 0xFF.
    hex!(
        "000102030405060708090A0B0C0D0E0F\
         101112131415161718191A1B1C1D1E1F\
         202122232425262728292A2B2C2D2E2F\
         303132333435363738393A3B3C3D3E3F\
         404142434445464748494A4B4C4D4E4F\
         505152535455565758595A5B5C5D5E5F\
         606162636465666768696A6B6C6D6E6F\
         707172737475767778797A7B7C7D7E7F\
         808182838485868788898A8B8C8D8E8F\
         909192939495969798999A9B9C9D9E9F\
         A0A1A2A3A4A5A6A7A8A9AAABACADAEAF\
         B0B1B2B3B4B5B6B7B8B9BABBBCBDBEBF\
         C0C1C2C3C4C5C6C7C8C9CACBCCCDCECF\
         D0D1D2D3D4D5D6D7D8D9DADBDCDDDEDF\
         E0E1E2E3E4E5E6E7E8E9EAEBECEDEEEF\
         F0F1F2F3F4F5F6F7F8F9FAFBFCFDFEFF"
    )
}

/// Returns the 1-byte test vector (for 8-bit tests)
fn test_8() -> [u8; 1] {
    [0xFF]
}

/// Returns the 5-byte test vector (for the 33‑bit test). Note that in C only 33 bits are used.
/// If your API supports “bit‐oriented” updates you might feed in 33 as bit‑length;
/// here we assume that you can supply the full byte slice and the hasher internally will mask the extra bits.
fn test_33() -> [u8; 5] {
    [0x00, 0x00, 0xFF, 0x00, 0x00]
}

/// Helper: Given a full message slice, returns a new Vec<u8> that represents the first `bits` bits of the message.
/// (Any leftover bits in the final byte are masked.)
fn truncate_to_bits(msg: &[u8], bits: usize) -> Vec<u8> {
    let full_bytes = bits / 8;
    let rem = bits % 8;
    let mut out = msg[..full_bytes].to_vec();
    if rem > 0 {
        let mask = 0xFF & (!((1 << (8 - rem)) - 1));
        out.push(msg[full_bytes] & mask);
    }
    out
}

//
//=== Kupyna-256 tests ===
//

#[test]
fn kup256_n512() {
    // Use first 64 bytes (512 bits) from the test message.
    let msg = &test_message()[..64];
    let mut hasher = Kupyna256::default();
    // If your implementation supports bit-level updates, you might do:
    // hasher.update_bits(msg, 512);
    // Otherwise, assume msg length is 64 bytes = 512 bits.
    hasher.update(msg);
    let result = hasher.finalize();
    assert_eq!(
        result[..],
        hex!("08F4EE6F1BE6903B324C4E27990CB24EF69DD58DBE84813EE0A52F6631239875")[..]
    );
}

#[test]
fn kup256_n1024() {
    // Use first 128 bytes (1024 bits)
    let msg = &test_message()[..128];
    let mut hasher = Kupyna256::default();
    hasher.update(msg);
    let result = hasher.finalize();
    assert_eq!(
        result[..],
        hex!("0A9474E645A7D25E255E9E89FFF42EC7EB31349007059284F0B182E452BDA882")[..]
    );
}

#[test]
fn kup256_n2048() {
    // Use the full 256-byte message (2048 bits)
    let msg = &test_message();
    let mut hasher = Kupyna256::default();
    hasher.update(msg);
    let result = hasher.finalize();
    assert_eq!(
        result[..],
        hex!("D305A32B963D149DC765F68594505D4077024F836C1BF03806E1624CE176C08F")[..]
    );
}

#[test]
fn kup256_n8() {
    // 8 bits: one full byte (0xFF)
    let msg = &test_8();
    let mut hasher = Kupyna256::default();
    hasher.update(msg);
    let result = hasher.finalize();
    assert_eq!(
        result[..],
        hex!("EA7677CA4526555680441C117982EA14059EA6D0D7124D6ECDB3DEEC49E890F4")[..]
    );
}

#[test]
fn kup256_n760() {
    // 760 bits: that’s 95 full bytes exactly.
    let msg = &test_message()[..95];
    let mut hasher = Kupyna256::default();
    hasher.update(msg);
    let result = hasher.finalize();
    assert_eq!(
        result[..],
        hex!("1075C8B0CB910F116BDA5FA1F19C29CF8ECC75CAFF7208BA2994B68FC56E8D16")[..]
    );
}

#[test]
fn kup256_empty() {
    // 0 bits – the empty string.
    let msg: &[u8] = b"";
    let mut hasher = Kupyna256::default();
    hasher.update(msg);
    let result = hasher.finalize();
    assert_eq!(
        result[..],
        hex!("CD5101D1CCDF0D1D1F4ADA56E888CD724CA1A0838A3521E7131D4FB78D0F5EB6")[..]
    );
}

#[test]
fn kup256_n510() {
    // 510 bits: that is 63 full bytes and 6 bits of the 64th byte.
    let full_msg = test_message();
    let msg = truncate_to_bits(&full_msg, 510);
    let mut hasher = Kupyna256::default();
    // If supported, use a bit-level update:
    // hasher.update_bits(&msg, 510);
    // Otherwise, assume you have pre-masked msg.
    hasher.update(&msg);
    let result = hasher.finalize();
    assert_eq!(
        result[..],
        hex!("875C0023DAA0C077809FDD6A9672B49E03903BFF98EBE48740AE998C7BE3851E")[..]
    );
}

#[test]
fn kup256_n655() {
    // 655 bits: 81 full bytes and 7 bits from the next byte.
    let full_msg = test_message();
    let msg = truncate_to_bits(&full_msg, 655);
    let mut hasher = Kupyna256::default();
    // Use bit-level update if available.
    hasher.update(&msg);
    let result = hasher.finalize();
    assert_eq!(
        result[..],
        hex!("4237D7DE1A00C4CC8037EDE9C54BA60D1C705CD1495DE19E5245BF3509DB59CE")[..]
    );
}

//
//=== Kupyna-48 test ===
//

#[test]
fn kup48_n512() {
    // For Kupyna-48, we use the first 64 bytes (512 bits) as input.
    let msg = &test_message()[..64];
    let mut hasher = Kupyna48::default();
    hasher.update(msg);
    let result = hasher.finalize();
    assert_eq!(
        result[..],
        hex!("2F6631239875")[..]
    );
}

//
//=== Kupyna-512 tests ===
//

#[test]
fn kup512_n512() {
    // 512 bits of input: first 64 bytes.
    let msg = &test_message()[..64];
    let mut hasher = Kupyna512::default();
    hasher.update(msg);
    let result = hasher.finalize();
    assert_eq!(
        result[..],
        hex!(
            "3813E2109118CDFB5A6D5E72F7208DCC"
            "C80A2DFB3AFDFB02F46992B5EDBE536B"
            "3560DD1D7E29C6F53978AF58B444E37B"
            "A685C0DD910533BA5D78EFFFC13DE62A"
        )[..]
    );
}

#[test]
fn kup512_n1024() {
    // 1024 bits of input: first 128 bytes.
    let msg = &test_message()[..128];
    let mut hasher = Kupyna512::default();
    hasher.update(msg);
    let result = hasher.finalize();
    assert_eq!(
        result[..],
        hex!(
            "76ED1AC28B1D0143013FFA87213B4090\
             B356441263C13E03FA060A8CADA32B97\
             9635657F256B15D5FCA4A174DE029F0B\
             1B4387C878FCC1C00E8705D783FD7FFE"
        )
    );
}

#[test]
fn kup512_n2048() {
    // 2048 bits: full 256-byte message.
    let msg = &test_message();
    let mut hasher = Kupyna512::default();
    hasher.update(msg);
    let result = hasher.finalize();
    assert_eq!(
        result[..],
        hex!(
            "0DD03D7350C409CB3C29C25893A0724F\
             6B133FA8B9EB90A64D1A8FA93B565566\
             11EB187D715A956B107E3BFC76482298\
             133A9CE8CBC0BD5E1436A5B197284F7E"
        )
    );
}

#[test]
fn kup512_n8() {
    // 8-bit input: one byte 0xFF.
    let msg = &test_8();
    let mut hasher = Kupyna512::default();
    hasher.update(msg);
    let result = hasher.finalize();
    assert_eq!(
        result[..],
        hex!(
            "871B18CF754B72740307A97B449ABEB3\
             2B64444CC0D5A4D65830AE5456837A72\
             D8458F12C8F06C98C616ABE11897F862\
             63B5CB77C420FB375374BEC52B6D0292"
        )
    );
}

#[test]
fn kup512_n1536() {
    // 1536 bits: first 192 bytes.
    let msg = &test_message()[..192];
    let mut hasher = Kupyna512::default();
    hasher.update(msg);
    let result = hasher.finalize();
    assert_eq!(
        result[..],
        hex!(
            "B189BFE987F682F5F167F0D7FA565330\
             E126B6E592B1C55D44299064EF95B1A5\
             7F3C2D0ECF17869D1D199EBBD02E8857\
             FB8ADD67A8C31F56CD82C016CF743121"
        )
    );
}

#[test]
fn kup512_empty() {
    // 0-bit input: empty.
    let msg: &[u8] = b"";
    let mut hasher = Kupyna512::default();
    hasher.update(msg);
    let result = hasher.finalize();
    assert_eq!(
        result[..],
        hex!(
            "656B2F4CD71462388B64A37043EA55DB\
             E445D452AECD46C3298343314EF04019\
             BCF A3F04265A9857F91BE91FCE197096\
             187CEDA78C9C1C021C294A0689198538"
        )
    );
}

#[test]
fn kup512_n510() {
    // 510 bits: 63 full bytes + 6 bits of the next byte.
    let full_msg = test_message();
    let msg = truncate_to_bits(&full_msg, 510);
    let mut hasher = Kupyna512::default();
    hasher.update(&msg);
    let result = hasher.finalize();
    assert_eq!(
        result[..],
        hex!(
            "2F3BBAC98E8771D6E3B8AA30153ABC4D\
             0C2985E91DA1B5568FD1BDD705CCAB7E\
             E8D95D2FC98BFA5322A241E09C896B58\
             284C83F2488CF943E4B3DE43E05F0DEA"
        )
    );
}

#[test]
fn kup512_n655() {
    // 655 bits: 81 full bytes + 7 bits from the next byte.
    let full_msg = test_message();
    let msg = truncate_to_bits(&full_msg, 655);
    let mut hasher = Kupyna512::default();
    hasher.update(&msg);
    let result = hasher.finalize();
    assert_eq!(
        result[..],
        hex!(
            "01B7BDA1DBA77D7379F53C2A498A390D\
             E5E688A12BC75FEE9E010CB6FEBED3B9\
             C7023931C74A7B55168A15047D5E2CB7\
             8A8B5CA2F75E05E80CA398030E02C7AA"
        )
    );
}

//
//=== Kupyna-384 tests ===
//

#[test]
fn kup384_n760() {
    // 760 bits: first 95 bytes.
    let msg = &test_message()[..95];
    let mut hasher = Kupyna384::default();
    hasher.update(msg);
    let result = hasher.finalize();
    assert_eq!(
        result[..],
        hex!(
            "D9021692D84E5175735654846BA751E6\
             D0ED0FAC36DFBC0841287DCB0B5584C7\
             5016C3DECC2A6E47C50B2F3811E351B8"
        )
    );
}

#[test]
fn kup384_n33() {
    // 33 bits: test_33 message (5 bytes, with only 33 bits used)
    let msg = test_33();
    // If your API supports specifying 33 bits rather than 40 bits,
    // you may want to call an update_bits(msg, 33) method.
    // Here we assume the input is pre-masked as needed.
    let mut hasher = Kupyna384::default();
    hasher.update(&msg);
    let result = hasher.finalize();
    assert_eq!(
        result[..],
        hex!(
            "B0331847CB0F28E0A7ECCBDF72386F49\
             2B8A07BD6AE6B4AF8C279F1C1E8D771C\
             D033917FCDFD22EB20A0C4F663C3611D"
        )
    );
}

#[test]
fn kup384_n1() {
    // 1 bit: using test_8 but only the first bit.
    let msg = truncate_to_bits(&test_8(), 1);
    let mut hasher = Kupyna384::default();
    hasher.update(&msg);
    let result = hasher.finalize();
    assert_eq!(
        result[..],
        hex!(
            "801BA7ACEFFF771FC331690512D432EF\
             031829EDF1705B487D90B8A333C29868\
             F586B377BE9C92F08D63F79277C82221"
        )
    );
}
