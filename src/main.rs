use hpke_rs::{hpke_types::*, Hpke, Mode};
use hpke_rs_libcrux::HpkeLibcrux;

fn main() -> Result<(), hpke_rs::HpkeError> {
    // X-Wing (draft-06) is currently supported by the libcrux provider.
    let mut hpke = Hpke::<HpkeLibcrux>::new(
        Mode::Base,
        KemAlgorithm::XWingDraft06,
        KdfAlgorithm::HkdfSha256,
        AeadAlgorithm::ChaCha20Poly1305,
    );

    // Generate recipient key pair.
    let (sk_r, pk_r) = hpke.generate_key_pair()?.into_keys();

    let info = b"xwing hpke info";
    let aad = b"xwing hpke aad";
    let plaintext = b"xwing hpke plaintext";

    // One-shot seal/open.
    let (enc, ciphertext) = hpke.seal(&pk_r, info, aad, plaintext, None, None, None)?;
    let decrypted = hpke.open(&enc, &sk_r, info, aad, &ciphertext, None, None, None)?;

    assert_eq!(decrypted, plaintext);
    println!("X-Wing HPKE round-trip ok");

    Ok(())
}
