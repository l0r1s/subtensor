// This file is part of Substrate.

// Copyright (C) Parity Technologies (UK) Ltd.
// SPDX-License-Identifier: Apache-2.0

// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
// 	http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

use codec::{Decode, Encode, MaxEncodedLen};
use enumflags2::{BitFlags, bitflags};
use frame_support::{
    BoundedVec, CloneNoBound, PartialEqNoBound, RuntimeDebugNoBound,
    traits::{ConstU32, Get},
};
use scale_info::{
    Path, Type, TypeInfo, TypeParameter,
    build::{Fields, Variants},
    meta_type,
};
use sp_runtime::{
    RuntimeDebug,
    traits::{AppendZerosInput, Zero},
};
use sp_std::{fmt::Debug, iter::once, ops::Add, prelude::*};
use subtensor_macros::freeze_struct;

/// Either underlying data blob if it is at most 32 bytes, or a hash of it. If the data is greater
/// than 32-bytes then it will be truncated when encoding.
///
/// Can also be `None`.
#[derive(Clone, Eq, PartialEq, RuntimeDebug, MaxEncodedLen)]
pub enum Data {
    /// No data here.
    None,
    /// The data is stored directly.
    Raw(BoundedVec<u8, ConstU32<64>>),
    /// Only the Blake2 hash of the data is stored. The preimage of the hash may be retrieved
    /// through some hash-lookup service.
    BlakeTwo256([u8; 32]),
    /// Only the SHA2-256 hash of the data is stored. The preimage of the hash may be retrieved
    /// through some hash-lookup service.
    Sha256([u8; 32]),
    /// Only the Keccak-256 hash of the data is stored. The preimage of the hash may be retrieved
    /// through some hash-lookup service.
    Keccak256([u8; 32]),
    /// Only the SHA3-256 hash of the data is stored. The preimage of the hash may be retrieved
    /// through some hash-lookup service.
    ShaThree256([u8; 32]),
}

impl Data {
    pub fn is_none(&self) -> bool {
        self == &Data::None
    }
}

impl Decode for Data {
    fn decode<I: codec::Input>(input: &mut I) -> sp_std::result::Result<Self, codec::Error> {
        let b = input.read_byte()?;
        Ok(match b {
            0 => Data::None,
            n @ 1..=65 => {
                let mut r: BoundedVec<_, _> = vec![0u8; (n as usize).saturating_sub(1)]
                    .try_into()
                    .expect("bound checked in match arm condition; qed");
                input.read(&mut r[..])?;
                Data::Raw(r)
            }
            66 => Data::BlakeTwo256(<[u8; 32]>::decode(input)?),
            67 => Data::Sha256(<[u8; 32]>::decode(input)?),
            68 => Data::Keccak256(<[u8; 32]>::decode(input)?),
            69 => Data::ShaThree256(<[u8; 32]>::decode(input)?),
            _ => return Err(codec::Error::from("invalid leading byte")),
        })
    }
}

impl Encode for Data {
    fn encode(&self) -> Vec<u8> {
        match self {
            Data::None => vec![0u8; 1],
            Data::Raw(x) => {
                let l = x.len().min(64) as u8;
                let mut r = vec![l.saturating_add(1)];
                r.extend_from_slice(&x[..]);
                r
            }
            Data::BlakeTwo256(h) => once(66u8).chain(h.iter().cloned()).collect(),
            Data::Sha256(h) => once(67u8).chain(h.iter().cloned()).collect(),
            Data::Keccak256(h) => once(68u8).chain(h.iter().cloned()).collect(),
            Data::ShaThree256(h) => once(69u8).chain(h.iter().cloned()).collect(),
        }
    }
}
impl codec::EncodeLike for Data {}

/// Add a Raw variant with the given index and a fixed sized byte array
macro_rules! data_raw_variants {
    ($variants:ident, $(($index:literal, $size:literal)),* ) => {
		$variants
		$(
			.variant(concat!("Raw", stringify!($size)), |v| v
				.index($index)
				.fields(Fields::unnamed().field(|f| f.ty::<[u8; $size]>()))
			)
		)*
    }
}

impl TypeInfo for Data {
    type Identity = Self;

    fn type_info() -> Type {
        let variants = Variants::new().variant("None", |v| v.index(0));

        // create a variant for all sizes of Raw data from 0-32
        let variants = data_raw_variants!(
            variants,
            (1, 0),
            (2, 1),
            (3, 2),
            (4, 3),
            (5, 4),
            (6, 5),
            (7, 6),
            (8, 7),
            (9, 8),
            (10, 9),
            (11, 10),
            (12, 11),
            (13, 12),
            (14, 13),
            (15, 14),
            (16, 15),
            (17, 16),
            (18, 17),
            (19, 18),
            (20, 19),
            (21, 20),
            (22, 21),
            (23, 22),
            (24, 23),
            (25, 24),
            (26, 25),
            (27, 26),
            (28, 27),
            (29, 28),
            (30, 29),
            (31, 30),
            (32, 31),
            (33, 32),
            (34, 33),
            (35, 34),
            (36, 35),
            (37, 36),
            (38, 37),
            (39, 38),
            (40, 39),
            (41, 40),
            (42, 41),
            (43, 42),
            (44, 43),
            (45, 44),
            (46, 45),
            (47, 46),
            (48, 47),
            (49, 48),
            (50, 49),
            (51, 50),
            (52, 51),
            (53, 52),
            (54, 53),
            (55, 54),
            (56, 55),
            (57, 56),
            (58, 57),
            (59, 58),
            (60, 59),
            (61, 60),
            (62, 61),
            (63, 62),
            (64, 63),
            (65, 64)
        );

        let variants = variants
            .variant("BlakeTwo256", |v| {
                v.index(66)
                    .fields(Fields::unnamed().field(|f| f.ty::<[u8; 32]>()))
            })
            .variant("Sha256", |v| {
                v.index(67)
                    .fields(Fields::unnamed().field(|f| f.ty::<[u8; 32]>()))
            })
            .variant("Keccak256", |v| {
                v.index(68)
                    .fields(Fields::unnamed().field(|f| f.ty::<[u8; 32]>()))
            })
            .variant("ShaThree256", |v| {
                v.index(69)
                    .fields(Fields::unnamed().field(|f| f.ty::<[u8; 32]>()))
            });

        Type::builder()
            .path(Path::new("Data", module_path!()))
            .variant(variants)
    }
}

impl Default for Data {
    fn default() -> Self {
        Self::None
    }
}

/// The fields that we use to identify the owner of an account with. Each corresponds to a field
/// in the `IdentityInfo` struct.
#[bitflags]
#[repr(u64)]
#[derive(Clone, Copy, PartialEq, Eq, RuntimeDebug, TypeInfo)]
pub enum IdentityField {
    Display = 0b0000000000000000000000000000000000000000000000000000000000000001,
    Legal = 0b0000000000000000000000000000000000000000000000000000000000000010,
    Web = 0b0000000000000000000000000000000000000000000000000000000000000100,
    Riot = 0b0000000000000000000000000000000000000000000000000000000000001000,
    Email = 0b0000000000000000000000000000000000000000000000000000000000010000,
    PgpFingerprint = 0b0000000000000000000000000000000000000000000000000000000000100000,
    Image = 0b0000000000000000000000000000000000000000000000000000000001000000,
    Twitter = 0b0000000000000000000000000000000000000000000000000000000010000000,
}

/// Wrapper type for `BitFlags<IdentityField>` that implements `Codec`.
#[derive(Clone, Copy, PartialEq, Default, RuntimeDebug)]
pub struct IdentityFields(pub BitFlags<IdentityField>);

impl MaxEncodedLen for IdentityFields {
    fn max_encoded_len() -> usize {
        u64::max_encoded_len()
    }
}

impl Eq for IdentityFields {}
impl Encode for IdentityFields {
    fn using_encoded<R, F: FnOnce(&[u8]) -> R>(&self, f: F) -> R {
        self.0.bits().using_encoded(f)
    }
}
impl Decode for IdentityFields {
    fn decode<I: codec::Input>(input: &mut I) -> sp_std::result::Result<Self, codec::Error> {
        let field = u64::decode(input)?;
        Ok(Self(
            <BitFlags<IdentityField>>::from_bits(field).map_err(|_| "invalid value")?,
        ))
    }
}
impl TypeInfo for IdentityFields {
    type Identity = Self;

    fn type_info() -> Type {
        Type::builder()
            .path(Path::new("BitFlags", module_path!()))
            .type_params(vec![TypeParameter::new(
                "T",
                Some(meta_type::<IdentityField>()),
            )])
            .composite(Fields::unnamed().field(|f| f.ty::<u64>().type_name("IdentityField")))
    }
}

/// Information concerning the identity of the controller of an account.
///
/// NOTE: This should be stored at the end of the storage item to facilitate the addition of extra
/// fields in a backwards compatible way through a specialized `Decode` impl.
#[freeze_struct("98e2d7fc7536226b")]
#[derive(
    CloneNoBound, Encode, Decode, Eq, MaxEncodedLen, PartialEqNoBound, RuntimeDebugNoBound, TypeInfo,
)]
#[codec(mel_bound())]
#[derive(frame_support::DefaultNoBound)]
#[scale_info(skip_type_params(FieldLimit))]
pub struct IdentityInfo<FieldLimit: Get<u32>> {
    /// Additional fields of the identity that are not catered for with the struct's explicit
    /// fields.
    pub additional: BoundedVec<(Data, Data), FieldLimit>,

    /// A reasonable display name for the controller of the account. This should be whatever it is
    /// that it is typically known as and should not be confusable with other entities, given
    /// reasonable context.
    ///
    /// Stored as UTF-8.
    pub display: Data,

    /// The full legal name in the local jurisdiction of the entity. This might be a bit
    /// long-winded.
    ///
    /// Stored as UTF-8.
    pub legal: Data,

    /// A representative website held by the controller of the account.
    ///
    /// NOTE: `https://` is automatically prepended.
    ///
    /// Stored as UTF-8.
    pub web: Data,

    /// The Riot/Matrix handle held by the controller of the account.
    ///
    /// Stored as UTF-8.
    pub riot: Data,

    /// The email address of the controller of the account.
    ///
    /// Stored as UTF-8.
    pub email: Data,

    /// The PGP/GPG public key of the controller of the account.
    pub pgp_fingerprint: Option<[u8; 20]>,

    /// A graphic image representing the controller of the account. Should be a company,
    /// organization or project logo or a headshot in the case of a human.
    pub image: Data,

    /// The Twitter identity. The leading `@` character may be elided.
    pub twitter: Data,
}

impl<FieldLimit: Get<u32>> IdentityInfo<FieldLimit> {
    pub fn fields(&self) -> IdentityFields {
        let mut res = <BitFlags<IdentityField>>::empty();
        if !self.display.is_none() {
            res.insert(IdentityField::Display);
        }
        if !self.legal.is_none() {
            res.insert(IdentityField::Legal);
        }
        if !self.web.is_none() {
            res.insert(IdentityField::Web);
        }
        if !self.riot.is_none() {
            res.insert(IdentityField::Riot);
        }
        if !self.email.is_none() {
            res.insert(IdentityField::Email);
        }
        if self.pgp_fingerprint.is_some() {
            res.insert(IdentityField::PgpFingerprint);
        }
        if !self.image.is_none() {
            res.insert(IdentityField::Image);
        }
        if !self.twitter.is_none() {
            res.insert(IdentityField::Twitter);
        }
        IdentityFields(res)
    }
}

/// Information concerning the identity of the controller of an account.
///
/// NOTE: This is stored separately primarily to facilitate the addition of extra fields in a
/// backwards compatible way through a specialized `Decode` impl.
#[freeze_struct("797b69e82710bb21")]
#[derive(
    CloneNoBound, Encode, Eq, MaxEncodedLen, PartialEqNoBound, RuntimeDebugNoBound, TypeInfo,
)]
#[codec(mel_bound())]
#[scale_info(skip_type_params(MaxAdditionalFields))]
pub struct Registration<
    Balance: Encode + Decode + MaxEncodedLen + Copy + Clone + Debug + Eq + PartialEq,
    MaxAdditionalFields: Get<u32>,
> {
    /// Amount held on deposit for this information.
    pub deposit: Balance,

    /// Information on the identity.
    pub info: IdentityInfo<MaxAdditionalFields>,
}

impl<
    Balance: Encode + Decode + MaxEncodedLen + Copy + Clone + Debug + Eq + PartialEq + Zero + Add,
    MaxAdditionalFields: Get<u32>,
> Registration<Balance, MaxAdditionalFields>
{
    pub(crate) fn total_deposit(&self) -> Balance {
        self.deposit
    }
}

impl<
    Balance: Encode + Decode + MaxEncodedLen + Copy + Clone + Debug + Eq + PartialEq,
    MaxAdditionalFields: Get<u32>,
> Decode for Registration<Balance, MaxAdditionalFields>
{
    fn decode<I: codec::Input>(input: &mut I) -> sp_std::result::Result<Self, codec::Error> {
        let (deposit, info) = Decode::decode(&mut AppendZerosInput::new(input))?;
        Ok(Self { deposit, info })
    }
}

#[cfg(test)]
#[allow(clippy::indexing_slicing, clippy::unwrap_used)]
mod tests {
    use super::*;

    #[test]
    fn manual_data_type_info() {
        let mut registry = scale_info::Registry::new();
        let type_id = registry.register_type(&scale_info::meta_type::<Data>());
        let registry: scale_info::PortableRegistry = registry.into();
        let type_info = registry.resolve(type_id.id).unwrap();

        let check_type_info = |data: &Data| {
            let variant_name = match data {
                Data::None => "None".to_string(),
                Data::BlakeTwo256(_) => "BlakeTwo256".to_string(),
                Data::Sha256(_) => "Sha256".to_string(),
                Data::Keccak256(_) => "Keccak256".to_string(),
                Data::ShaThree256(_) => "ShaThree256".to_string(),
                Data::Raw(bytes) => format!("Raw{}", bytes.len()),
            };
            if let scale_info::TypeDef::Variant(variant) = &type_info.type_def {
                let variant = variant
                    .variants
                    .iter()
                    .find(|v| v.name == variant_name)
                    .unwrap_or_else(|| panic!("Expected to find variant {}", variant_name));

                let field_arr_len = variant
                    .fields
                    .first()
                    .and_then(|f| registry.resolve(f.ty.id))
                    .map(|ty| {
                        if let scale_info::TypeDef::Array(arr) = &ty.type_def {
                            arr.len
                        } else {
                            panic!("Should be an array type")
                        }
                    })
                    .unwrap_or(0);

                let encoded = data.encode();
                assert_eq!(encoded[0], variant.index);
                assert_eq!(encoded.len() as u32 - 1, field_arr_len);
            } else {
                panic!("Should be a variant type")
            };
        };

        let mut data = vec![
            Data::None,
            Data::BlakeTwo256(Default::default()),
            Data::Sha256(Default::default()),
            Data::Keccak256(Default::default()),
            Data::ShaThree256(Default::default()),
        ];

        // A Raw instance for all possible sizes of the Raw data
        for n in 0..64 {
            data.push(Data::Raw(vec![0u8; n as usize].try_into().unwrap()))
        }

        for d in data.iter() {
            check_type_info(d);
        }
    }
}
