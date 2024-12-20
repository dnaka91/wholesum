use std::{
    fmt::{self, Display},
    fs::File,
    io::{BufReader, Result, Write},
    ops::Add,
    path::Path,
};

use clap::ValueEnum;
use digest::{generic_array::ArrayLength, Digest, OutputSizeUser};
use strum::{Display, EnumString};

pub mod hashfile;

#[derive(Debug, Default, Clone, Copy, EnumString, ValueEnum)]
#[strum(serialize_all = "kebab-case")]
pub enum Mode {
    #[default]
    Text,
    Binary,
}

impl Display for Mode {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        use std::fmt::Write;

        f.write_char(match *self {
            Self::Text => ' ',
            Self::Binary => '*',
        })
    }
}

#[derive(Debug, Default, Clone, Copy, Display, EnumString, ValueEnum)]
#[strum(serialize_all = "kebab-case")]
pub enum Algorithm {
    #[default]
    Blake3,
    Blake2s,
    Blake2b,
    Sha3_512,
    Sha3_384,
    Sha3_256,
    Sha3_224,
    Sha2_512,
    Sha2_384,
    Sha2_256,
    Sha2_224,
    Sha1,
    Md5,
    #[strum(serialize = "fsb-512")]
    Fsb512,
    #[strum(serialize = "fsb-384")]
    Fsb384,
    #[strum(serialize = "fsb-256")]
    Fsb256,
    #[strum(serialize = "fsb-224")]
    Fsb224,
    #[strum(serialize = "fsb-160")]
    Fsb160,
    Gost94,
    #[strum(serialize = "groestl-512")]
    Groestl512,
    #[strum(serialize = "groestl-384")]
    Groestl384,
    #[strum(serialize = "groestl-256")]
    Groestl256,
    #[strum(serialize = "groestl-224")]
    Groestl224,
    Md4,
    Md2,
    Ripemd320,
    Ripemd256,
    Ripemd160,
    #[strum(serialize = "shabal-512")]
    Shabal512,
    #[strum(serialize = "shabal-384")]
    Shabal384,
    #[strum(serialize = "shabal-256")]
    Shabal256,
    #[strum(serialize = "shabal-224")]
    Shabal224,
    #[strum(serialize = "shabal-192")]
    Shabal192,
    Sm3,
    #[strum(serialize = "streebog-512")]
    Streebog512,
    #[strum(serialize = "streebog-256")]
    Streebog256,
    Tiger,
    Whirlpool,
    Ascon,
    AsconA,
    #[strum(serialize = "belt")]
    BelT,
    #[strum(serialize = "jh-512")]
    Jh512,
    #[strum(serialize = "jh-384")]
    Jh384,
    #[strum(serialize = "jh-256")]
    Jh256,
    #[strum(serialize = "jh-224")]
    Jh224,
    #[strum(serialize = "skein-1024")]
    Skein1024,
    #[strum(serialize = "skein-512")]
    Skein512,
    #[strum(serialize = "skein-256")]
    Skein256,
}

impl Algorithm {
    pub fn into_hasher(self) -> fn(&Path) -> Result<String> {
        match self {
            Self::Blake3 => hash::<blake3::Hasher>,
            Self::Blake2s => hash::<blake2::Blake2s256>,
            Self::Blake2b => hash::<blake2::Blake2b512>,
            Self::Sha3_512 => hash::<sha3::Sha3_512>,
            Self::Sha3_384 => hash::<sha3::Sha3_384>,
            Self::Sha3_256 => hash::<sha3::Sha3_256>,
            Self::Sha3_224 => hash::<sha3::Sha3_224>,
            Self::Sha2_512 => hash::<sha2::Sha512>,
            Self::Sha2_384 => hash::<sha2::Sha384>,
            Self::Sha2_256 => hash::<sha2::Sha256>,
            Self::Sha2_224 => hash::<sha2::Sha224>,
            Self::Sha1 => hash::<sha1::Sha1>,
            Self::Md5 => hash::<md5::Md5>,
            Self::Fsb512 => hash::<fsb::Fsb512>,
            Self::Fsb384 => hash::<fsb::Fsb384>,
            Self::Fsb256 => hash::<fsb::Fsb256>,
            Self::Fsb224 => hash::<fsb::Fsb224>,
            Self::Fsb160 => hash::<fsb::Fsb160>,
            Self::Gost94 => hash::<gost94::Gost94Test>,
            Self::Groestl512 => hash::<groestl::Groestl512>,
            Self::Groestl384 => hash::<groestl::Groestl384>,
            Self::Groestl256 => hash::<groestl::Groestl256>,
            Self::Groestl224 => hash::<groestl::Groestl224>,
            Self::Md4 => hash::<md4::Md4>,
            Self::Md2 => hash::<md2::Md2>,
            Self::Ripemd320 => hash::<ripemd::Ripemd320>,
            Self::Ripemd256 => hash::<ripemd::Ripemd256>,
            Self::Ripemd160 => hash::<ripemd::Ripemd160>,
            Self::Shabal512 => hash::<shabal::Shabal512>,
            Self::Shabal384 => hash::<shabal::Shabal384>,
            Self::Shabal256 => hash::<shabal::Shabal256>,
            Self::Shabal224 => hash::<shabal::Shabal224>,
            Self::Shabal192 => hash::<shabal::Shabal192>,
            Self::Sm3 => hash::<sm3::Sm3>,
            Self::Streebog512 => hash::<streebog::Streebog512>,
            Self::Streebog256 => hash::<streebog::Streebog256>,
            Self::Tiger => hash::<tiger::Tiger>,
            Self::Whirlpool => hash::<whirlpool::Whirlpool>,
            Self::Ascon => hash::<ascon_hash::AsconHash>,
            Self::AsconA => hash::<ascon_hash::AsconAHash>,
            Self::BelT => hash::<belt_hash::BeltHash>,
            Self::Jh512 => hash::<jh::Jh512>,
            Self::Jh384 => hash::<jh::Jh384>,
            Self::Jh256 => hash::<jh::Jh256>,
            Self::Jh224 => hash::<jh::Jh224>,
            Self::Skein1024 => hash::<skein::Skein1024>,
            Self::Skein512 => hash::<skein::Skein512>,
            Self::Skein256 => hash::<skein::Skein256>,
        }
    }
}

fn hash<H: SimpleHasher>(input: &Path) -> Result<String> {
    let mut reader = BufReader::new(File::open(input)?);
    let mut hasher = H::new();

    std::io::copy(&mut reader, &mut WriteWrapper(&mut hasher))?;

    Ok(hasher.finalize())
}

trait SimpleHasher {
    fn new() -> Self;
    fn update(&mut self, data: &[u8]);
    fn finalize(self) -> String;
}

impl<D> SimpleHasher for D
where
    D: Digest,
    <D as OutputSizeUser>::OutputSize: Add,
    <<D as OutputSizeUser>::OutputSize as Add>::Output: ArrayLength<u8>,
{
    fn new() -> Self {
        D::new()
    }

    fn update(&mut self, data: &[u8]) {
        D::update(self, data);
    }

    fn finalize(self) -> String {
        format!("{:x}", D::finalize(self))
    }
}

struct WriteWrapper<'a, T: SimpleHasher>(&'a mut T);

impl<T: SimpleHasher> Write for WriteWrapper<'_, T> {
    fn write(&mut self, buf: &[u8]) -> Result<usize> {
        self.0.update(buf);
        Ok(buf.len())
    }

    fn flush(&mut self) -> Result<()> {
        Ok(())
    }
}
