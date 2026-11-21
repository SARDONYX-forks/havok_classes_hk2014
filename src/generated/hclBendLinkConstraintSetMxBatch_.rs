use super::class_requires::*;
use super::*;
/// # C++ Info
/// - name: `hclBendLinkConstraintSetMxBatch`
/// - version: `0`
/// - signature: `0x96e40e87`
/// - size: `  0`(x86)/`448`(x86_64)
/// -  vtable: `false`
#[allow(non_upper_case_globals, non_snake_case)]
#[cfg_attr(feature = "json_schema", derive(schemars::JsonSchema))]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(educe::Educe)]
#[educe(Debug, Clone, Default, PartialEq)]
pub struct hclBendLinkConstraintSetMxBatch<'a> {
    /// # Unique index for this class
    /// - Represents a pointer on XML (`<hkobject name="#0001"></hkobject>`)
    /// - [`Option::None`] => This class is `class in field`.(`<hkobject></hkobject>`)
    ///
    /// # Note
    /// Not present in the binary & Not exist actual C++ field.
    #[cfg_attr(
        feature = "serde",
        serde(skip_serializing_if = "Option::is_none", default)
    )]
    #[cfg_attr(feature = "serde", serde(borrow))]
    pub __ptr: Option<Pointer<'a>>,
    /// # C++ Info
    /// - name: `bendMinLengths`(ctype: `hkReal[16]`)
    /// - offset: `  0`(x86)/`  0`(x86_64)
    /// - type_size: `  0`(x86)/` 64`(x86_64)
    #[cfg_attr(feature = "serde_default", serde(default))]
    #[cfg_attr(feature = "json_schema", schemars(rename = "bendMinLengths"))]
    #[cfg_attr(feature = "serde", serde(rename = "bendMinLengths"))]
    pub m_bendMinLengths: [f32; 16usize],
    /// # C++ Info
    /// - name: `stretchMaxLengths`(ctype: `hkReal[16]`)
    /// - offset: `  0`(x86)/` 64`(x86_64)
    /// - type_size: `  0`(x86)/` 64`(x86_64)
    #[cfg_attr(feature = "serde_default", serde(default))]
    #[cfg_attr(feature = "json_schema", schemars(rename = "stretchMaxLengths"))]
    #[cfg_attr(feature = "serde", serde(rename = "stretchMaxLengths"))]
    pub m_stretchMaxLengths: [f32; 16usize],
    /// # C++ Info
    /// - name: `stretchStiffnesses`(ctype: `hkReal[16]`)
    /// - offset: `  0`(x86)/`128`(x86_64)
    /// - type_size: `  0`(x86)/` 64`(x86_64)
    #[cfg_attr(feature = "serde_default", serde(default))]
    #[cfg_attr(feature = "json_schema", schemars(rename = "stretchStiffnesses"))]
    #[cfg_attr(feature = "serde", serde(rename = "stretchStiffnesses"))]
    pub m_stretchStiffnesses: [f32; 16usize],
    /// # C++ Info
    /// - name: `bendStiffnesses`(ctype: `hkReal[16]`)
    /// - offset: `  0`(x86)/`192`(x86_64)
    /// - type_size: `  0`(x86)/` 64`(x86_64)
    #[cfg_attr(feature = "serde_default", serde(default))]
    #[cfg_attr(feature = "json_schema", schemars(rename = "bendStiffnesses"))]
    #[cfg_attr(feature = "serde", serde(rename = "bendStiffnesses"))]
    pub m_bendStiffnesses: [f32; 16usize],
    /// # C++ Info
    /// - name: `invMassesA`(ctype: `hkReal[16]`)
    /// - offset: `  0`(x86)/`256`(x86_64)
    /// - type_size: `  0`(x86)/` 64`(x86_64)
    #[cfg_attr(feature = "serde_default", serde(default))]
    #[cfg_attr(feature = "json_schema", schemars(rename = "invMassesA"))]
    #[cfg_attr(feature = "serde", serde(rename = "invMassesA"))]
    pub m_invMassesA: [f32; 16usize],
    /// # C++ Info
    /// - name: `invMassesB`(ctype: `hkReal[16]`)
    /// - offset: `  0`(x86)/`320`(x86_64)
    /// - type_size: `  0`(x86)/` 64`(x86_64)
    #[cfg_attr(feature = "serde_default", serde(default))]
    #[cfg_attr(feature = "json_schema", schemars(rename = "invMassesB"))]
    #[cfg_attr(feature = "serde", serde(rename = "invMassesB"))]
    pub m_invMassesB: [f32; 16usize],
    /// # C++ Info
    /// - name: `particlesA`(ctype: `hkUint16[16]`)
    /// - offset: `  0`(x86)/`384`(x86_64)
    /// - type_size: `  0`(x86)/` 32`(x86_64)
    #[cfg_attr(feature = "serde_default", serde(default))]
    #[cfg_attr(feature = "serde", serde(borrow))]
    #[cfg_attr(feature = "json_schema", schemars(rename = "particlesA"))]
    #[cfg_attr(feature = "serde", serde(rename = "particlesA"))]
    pub m_particlesA: [U16<'a>; 16usize],
    /// # C++ Info
    /// - name: `particlesB`(ctype: `hkUint16[16]`)
    /// - offset: `  0`(x86)/`416`(x86_64)
    /// - type_size: `  0`(x86)/` 32`(x86_64)
    #[cfg_attr(feature = "serde_default", serde(default))]
    #[cfg_attr(feature = "serde", serde(borrow))]
    #[cfg_attr(feature = "json_schema", schemars(rename = "particlesB"))]
    #[cfg_attr(feature = "serde", serde(rename = "particlesB"))]
    pub m_particlesB: [U16<'a>; 16usize],
}
const _: () = {
    use havok_serde as _serde;
    impl<'a> _serde::HavokClass for hclBendLinkConstraintSetMxBatch<'a> {
        #[inline]
        fn name(&self) -> &'static str {
            "hclBendLinkConstraintSetMxBatch"
        }
        #[inline]
        fn signature(&self) -> _serde::__private::Signature {
            _serde::__private::Signature::new(0x96e40e87)
        }
        #[allow(clippy::let_and_return, clippy::vec_init_then_push)]
        fn deps_indexes(&self) -> Vec<&Pointer<'_>> {
            let mut v = Vec::new();
            v
        }
    }
    impl<'a> _serde::Serialize for hclBendLinkConstraintSetMxBatch<'a> {
        fn serialize<S>(&self, __serializer: S) -> Result<S::Ok, S::Error>
        where
            S: _serde::ser::Serializer,
        {
            let class_meta = self
                .__ptr
                .as_ref()
                .map(|name| (name, _serde::__private::Signature::new(0x96e40e87)));
            let mut serializer = __serializer
                .serialize_struct(
                    "hclBendLinkConstraintSetMxBatch",
                    class_meta,
                    (0u64, 448u64),
                )?;
            serializer
                .serialize_fixed_array_field(
                    "bendMinLengths",
                    self.m_bendMinLengths.as_slice(),
                    TypeSize::NonPtr,
                )?;
            serializer
                .serialize_fixed_array_field(
                    "stretchMaxLengths",
                    self.m_stretchMaxLengths.as_slice(),
                    TypeSize::NonPtr,
                )?;
            serializer
                .serialize_fixed_array_field(
                    "stretchStiffnesses",
                    self.m_stretchStiffnesses.as_slice(),
                    TypeSize::NonPtr,
                )?;
            serializer
                .serialize_fixed_array_field(
                    "bendStiffnesses",
                    self.m_bendStiffnesses.as_slice(),
                    TypeSize::NonPtr,
                )?;
            serializer
                .serialize_fixed_array_field(
                    "invMassesA",
                    self.m_invMassesA.as_slice(),
                    TypeSize::NonPtr,
                )?;
            serializer
                .serialize_fixed_array_field(
                    "invMassesB",
                    self.m_invMassesB.as_slice(),
                    TypeSize::NonPtr,
                )?;
            serializer
                .serialize_fixed_array_field(
                    "particlesA",
                    self.m_particlesA.as_slice(),
                    TypeSize::NonPtr,
                )?;
            serializer
                .serialize_fixed_array_field(
                    "particlesB",
                    self.m_particlesB.as_slice(),
                    TypeSize::NonPtr,
                )?;
            serializer.end()
        }
    }
};
#[doc(hidden)]
#[allow(non_upper_case_globals, unused_attributes, unused_qualifications)]
const _: () = {
    use havok_serde as _serde;
    #[automatically_derived]
    impl<'de> _serde::Deserialize<'de> for hclBendLinkConstraintSetMxBatch<'de> {
        fn deserialize<__D>(deserializer: __D) -> core::result::Result<Self, __D::Error>
        where
            __D: _serde::Deserializer<'de>,
        {
            #[allow(non_camel_case_types)]
            enum __Field {
                m_bendMinLengths,
                m_stretchMaxLengths,
                m_stretchStiffnesses,
                m_bendStiffnesses,
                m_invMassesA,
                m_invMassesB,
                m_particlesA,
                m_particlesB,
                __ignore,
            }
            struct __FieldVisitor;
            impl<'de> _serde::de::Visitor<'de> for __FieldVisitor {
                type Value = __Field;
                fn expecting(
                    &self,
                    __formatter: &mut core::fmt::Formatter,
                ) -> core::fmt::Result {
                    core::fmt::Formatter::write_str(__formatter, "field identifier")
                }
                /// Intended for use in XML.
                #[allow(clippy::match_single_binding)]
                #[allow(clippy::reversed_empty_ranges)]
                #[allow(clippy::single_match)]
                fn visit_key<__E>(
                    self,
                    __value: &str,
                ) -> core::result::Result<Self::Value, __E>
                where
                    __E: _serde::de::Error,
                {
                    match __value {
                        "bendMinLengths" => Ok(__Field::m_bendMinLengths),
                        "stretchMaxLengths" => Ok(__Field::m_stretchMaxLengths),
                        "stretchStiffnesses" => Ok(__Field::m_stretchStiffnesses),
                        "bendStiffnesses" => Ok(__Field::m_bendStiffnesses),
                        "invMassesA" => Ok(__Field::m_invMassesA),
                        "invMassesB" => Ok(__Field::m_invMassesB),
                        "particlesA" => Ok(__Field::m_particlesA),
                        "particlesB" => Ok(__Field::m_particlesB),
                        _ => Ok(__Field::__ignore),
                    }
                }
            }
            impl<'de> _serde::Deserialize<'de> for __Field {
                #[inline]
                fn deserialize<__D>(
                    __deserializer: __D,
                ) -> core::result::Result<Self, __D::Error>
                where
                    __D: _serde::Deserializer<'de>,
                {
                    _serde::Deserializer::deserialize_key(__deserializer, __FieldVisitor)
                }
            }
            struct __hclBendLinkConstraintSetMxBatchVisitor<'de> {
                marker: _serde::__private::PhantomData<
                    hclBendLinkConstraintSetMxBatch<'de>,
                >,
                lifetime: _serde::__private::PhantomData<&'de ()>,
            }
            #[allow(clippy::match_single_binding)]
            #[allow(clippy::reversed_empty_ranges)]
            #[allow(clippy::single_match)]
            impl<'de> _serde::de::Visitor<'de>
            for __hclBendLinkConstraintSetMxBatchVisitor<'de> {
                type Value = hclBendLinkConstraintSetMxBatch<'de>;
                fn expecting(
                    &self,
                    __formatter: &mut core::fmt::Formatter,
                ) -> core::fmt::Result {
                    core::fmt::Formatter::write_str(
                        __formatter,
                        "struct hclBendLinkConstraintSetMxBatch",
                    )
                }
                fn visit_struct_for_bytes<__A>(
                    self,
                    mut __map: __A,
                ) -> _serde::__private::Result<Self::Value, __A::Error>
                where
                    __A: _serde::de::MapAccess<'de>,
                {
                    let __ptr = __A::class_ptr(&mut __map);
                    let mut m_bendMinLengths: _serde::__private::Option<
                        [f32; 16usize],
                    > = _serde::__private::None;
                    let mut m_stretchMaxLengths: _serde::__private::Option<
                        [f32; 16usize],
                    > = _serde::__private::None;
                    let mut m_stretchStiffnesses: _serde::__private::Option<
                        [f32; 16usize],
                    > = _serde::__private::None;
                    let mut m_bendStiffnesses: _serde::__private::Option<
                        [f32; 16usize],
                    > = _serde::__private::None;
                    let mut m_invMassesA: _serde::__private::Option<[f32; 16usize]> = _serde::__private::None;
                    let mut m_invMassesB: _serde::__private::Option<[f32; 16usize]> = _serde::__private::None;
                    let mut m_particlesA: _serde::__private::Option<
                        [U16<'de>; 16usize],
                    > = _serde::__private::None;
                    let mut m_particlesB: _serde::__private::Option<
                        [U16<'de>; 16usize],
                    > = _serde::__private::None;
                    for i in 0..8usize {
                        match i {
                            0usize => {
                                if _serde::__private::Option::is_some(&m_bendMinLengths) {
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "bendMinLengths",
                                        ),
                                    );
                                }
                                m_bendMinLengths = _serde::__private::Some(
                                    match __A::next_value::<[f32; 16usize]>(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            1usize => {
                                if _serde::__private::Option::is_some(
                                    &m_stretchMaxLengths,
                                ) {
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "stretchMaxLengths",
                                        ),
                                    );
                                }
                                m_stretchMaxLengths = _serde::__private::Some(
                                    match __A::next_value::<[f32; 16usize]>(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            2usize => {
                                if _serde::__private::Option::is_some(
                                    &m_stretchStiffnesses,
                                ) {
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "stretchStiffnesses",
                                        ),
                                    );
                                }
                                m_stretchStiffnesses = _serde::__private::Some(
                                    match __A::next_value::<[f32; 16usize]>(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            3usize => {
                                if _serde::__private::Option::is_some(&m_bendStiffnesses) {
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "bendStiffnesses",
                                        ),
                                    );
                                }
                                m_bendStiffnesses = _serde::__private::Some(
                                    match __A::next_value::<[f32; 16usize]>(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            4usize => {
                                if _serde::__private::Option::is_some(&m_invMassesA) {
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "invMassesA",
                                        ),
                                    );
                                }
                                m_invMassesA = _serde::__private::Some(
                                    match __A::next_value::<[f32; 16usize]>(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            5usize => {
                                if _serde::__private::Option::is_some(&m_invMassesB) {
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "invMassesB",
                                        ),
                                    );
                                }
                                m_invMassesB = _serde::__private::Some(
                                    match __A::next_value::<[f32; 16usize]>(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            6usize => {
                                if _serde::__private::Option::is_some(&m_particlesA) {
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "particlesA",
                                        ),
                                    );
                                }
                                m_particlesA = _serde::__private::Some(
                                    match __A::next_value::<[U16<'de>; 16usize]>(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            7usize => {
                                if _serde::__private::Option::is_some(&m_particlesB) {
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "particlesB",
                                        ),
                                    );
                                }
                                m_particlesB = _serde::__private::Some(
                                    match __A::next_value::<[U16<'de>; 16usize]>(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            _ => {}
                        }
                    }
                    let m_bendMinLengths = match m_bendMinLengths {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "bendMinLengths",
                                ),
                            );
                        }
                    };
                    let m_stretchMaxLengths = match m_stretchMaxLengths {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "stretchMaxLengths",
                                ),
                            );
                        }
                    };
                    let m_stretchStiffnesses = match m_stretchStiffnesses {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "stretchStiffnesses",
                                ),
                            );
                        }
                    };
                    let m_bendStiffnesses = match m_bendStiffnesses {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "bendStiffnesses",
                                ),
                            );
                        }
                    };
                    let m_invMassesA = match m_invMassesA {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "invMassesA",
                                ),
                            );
                        }
                    };
                    let m_invMassesB = match m_invMassesB {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "invMassesB",
                                ),
                            );
                        }
                    };
                    let m_particlesA = match m_particlesA {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "particlesA",
                                ),
                            );
                        }
                    };
                    let m_particlesB = match m_particlesB {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "particlesB",
                                ),
                            );
                        }
                    };
                    _serde::__private::Ok(hclBendLinkConstraintSetMxBatch {
                        __ptr,
                        m_bendMinLengths,
                        m_stretchMaxLengths,
                        m_stretchStiffnesses,
                        m_bendStiffnesses,
                        m_invMassesA,
                        m_invMassesB,
                        m_particlesA,
                        m_particlesB,
                    })
                }
                #[allow(clippy::manual_unwrap_or_default)]
                fn visit_struct<__A>(
                    self,
                    mut __map: __A,
                ) -> _serde::__private::Result<Self::Value, __A::Error>
                where
                    __A: _serde::de::MapAccess<'de>,
                {
                    let mut m_bendMinLengths: _serde::__private::Option<
                        [f32; 16usize],
                    > = _serde::__private::None;
                    let mut m_stretchMaxLengths: _serde::__private::Option<
                        [f32; 16usize],
                    > = _serde::__private::None;
                    let mut m_stretchStiffnesses: _serde::__private::Option<
                        [f32; 16usize],
                    > = _serde::__private::None;
                    let mut m_bendStiffnesses: _serde::__private::Option<
                        [f32; 16usize],
                    > = _serde::__private::None;
                    let mut m_invMassesA: _serde::__private::Option<[f32; 16usize]> = _serde::__private::None;
                    let mut m_invMassesB: _serde::__private::Option<[f32; 16usize]> = _serde::__private::None;
                    let mut m_particlesA: _serde::__private::Option<
                        [U16<'de>; 16usize],
                    > = _serde::__private::None;
                    let mut m_particlesB: _serde::__private::Option<
                        [U16<'de>; 16usize],
                    > = _serde::__private::None;
                    while let _serde::__private::Some(__key) = {
                        __A::next_key::<__Field>(&mut __map)?
                    } {
                        match __key {
                            __Field::m_bendMinLengths => {
                                #[cfg(
                                    any(feature = "strict", feature = "ignore_duplicates")
                                )]
                                if _serde::__private::Option::is_some(&m_bendMinLengths) {
                                    #[cfg(feature = "ignore_duplicates")]
                                    {
                                        __A::skip_value(&mut __map)?;
                                        continue;
                                    }
                                    #[cfg(feature = "strict")]
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "bendMinLengths",
                                        ),
                                    );
                                }
                                m_bendMinLengths = _serde::__private::Some(
                                    match __A::next_value::<[f32; 16usize]>(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            __Field::m_stretchMaxLengths => {
                                #[cfg(
                                    any(feature = "strict", feature = "ignore_duplicates")
                                )]
                                if _serde::__private::Option::is_some(
                                    &m_stretchMaxLengths,
                                ) {
                                    #[cfg(feature = "ignore_duplicates")]
                                    {
                                        __A::skip_value(&mut __map)?;
                                        continue;
                                    }
                                    #[cfg(feature = "strict")]
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "stretchMaxLengths",
                                        ),
                                    );
                                }
                                m_stretchMaxLengths = _serde::__private::Some(
                                    match __A::next_value::<[f32; 16usize]>(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            __Field::m_stretchStiffnesses => {
                                #[cfg(
                                    any(feature = "strict", feature = "ignore_duplicates")
                                )]
                                if _serde::__private::Option::is_some(
                                    &m_stretchStiffnesses,
                                ) {
                                    #[cfg(feature = "ignore_duplicates")]
                                    {
                                        __A::skip_value(&mut __map)?;
                                        continue;
                                    }
                                    #[cfg(feature = "strict")]
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "stretchStiffnesses",
                                        ),
                                    );
                                }
                                m_stretchStiffnesses = _serde::__private::Some(
                                    match __A::next_value::<[f32; 16usize]>(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            __Field::m_bendStiffnesses => {
                                #[cfg(
                                    any(feature = "strict", feature = "ignore_duplicates")
                                )]
                                if _serde::__private::Option::is_some(&m_bendStiffnesses) {
                                    #[cfg(feature = "ignore_duplicates")]
                                    {
                                        __A::skip_value(&mut __map)?;
                                        continue;
                                    }
                                    #[cfg(feature = "strict")]
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "bendStiffnesses",
                                        ),
                                    );
                                }
                                m_bendStiffnesses = _serde::__private::Some(
                                    match __A::next_value::<[f32; 16usize]>(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            __Field::m_invMassesA => {
                                #[cfg(
                                    any(feature = "strict", feature = "ignore_duplicates")
                                )]
                                if _serde::__private::Option::is_some(&m_invMassesA) {
                                    #[cfg(feature = "ignore_duplicates")]
                                    {
                                        __A::skip_value(&mut __map)?;
                                        continue;
                                    }
                                    #[cfg(feature = "strict")]
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "invMassesA",
                                        ),
                                    );
                                }
                                m_invMassesA = _serde::__private::Some(
                                    match __A::next_value::<[f32; 16usize]>(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            __Field::m_invMassesB => {
                                #[cfg(
                                    any(feature = "strict", feature = "ignore_duplicates")
                                )]
                                if _serde::__private::Option::is_some(&m_invMassesB) {
                                    #[cfg(feature = "ignore_duplicates")]
                                    {
                                        __A::skip_value(&mut __map)?;
                                        continue;
                                    }
                                    #[cfg(feature = "strict")]
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "invMassesB",
                                        ),
                                    );
                                }
                                m_invMassesB = _serde::__private::Some(
                                    match __A::next_value::<[f32; 16usize]>(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            __Field::m_particlesA => {
                                #[cfg(
                                    any(feature = "strict", feature = "ignore_duplicates")
                                )]
                                if _serde::__private::Option::is_some(&m_particlesA) {
                                    #[cfg(feature = "ignore_duplicates")]
                                    {
                                        __A::skip_value(&mut __map)?;
                                        continue;
                                    }
                                    #[cfg(feature = "strict")]
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "particlesA",
                                        ),
                                    );
                                }
                                m_particlesA = _serde::__private::Some(
                                    match __A::next_value::<[U16<'de>; 16usize]>(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            __Field::m_particlesB => {
                                #[cfg(
                                    any(feature = "strict", feature = "ignore_duplicates")
                                )]
                                if _serde::__private::Option::is_some(&m_particlesB) {
                                    #[cfg(feature = "ignore_duplicates")]
                                    {
                                        __A::skip_value(&mut __map)?;
                                        continue;
                                    }
                                    #[cfg(feature = "strict")]
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "particlesB",
                                        ),
                                    );
                                }
                                m_particlesB = _serde::__private::Some(
                                    match __A::next_value::<[U16<'de>; 16usize]>(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            _ => __A::skip_value(&mut __map)?,
                        }
                    }
                    let m_bendMinLengths = match m_bendMinLengths {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            #[cfg(feature = "strict")]
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "bendMinLengths",
                                ),
                            );
                            #[cfg(not(feature = "strict"))] Default::default()
                        }
                    };
                    let m_stretchMaxLengths = match m_stretchMaxLengths {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            #[cfg(feature = "strict")]
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "stretchMaxLengths",
                                ),
                            );
                            #[cfg(not(feature = "strict"))] Default::default()
                        }
                    };
                    let m_stretchStiffnesses = match m_stretchStiffnesses {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            #[cfg(feature = "strict")]
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "stretchStiffnesses",
                                ),
                            );
                            #[cfg(not(feature = "strict"))] Default::default()
                        }
                    };
                    let m_bendStiffnesses = match m_bendStiffnesses {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            #[cfg(feature = "strict")]
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "bendStiffnesses",
                                ),
                            );
                            #[cfg(not(feature = "strict"))] Default::default()
                        }
                    };
                    let m_invMassesA = match m_invMassesA {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            #[cfg(feature = "strict")]
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "invMassesA",
                                ),
                            );
                            #[cfg(not(feature = "strict"))] Default::default()
                        }
                    };
                    let m_invMassesB = match m_invMassesB {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            #[cfg(feature = "strict")]
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "invMassesB",
                                ),
                            );
                            #[cfg(not(feature = "strict"))] Default::default()
                        }
                    };
                    let m_particlesA = match m_particlesA {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            #[cfg(feature = "strict")]
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "particlesA",
                                ),
                            );
                            #[cfg(not(feature = "strict"))] Default::default()
                        }
                    };
                    let m_particlesB = match m_particlesB {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            #[cfg(feature = "strict")]
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "particlesB",
                                ),
                            );
                            #[cfg(not(feature = "strict"))] Default::default()
                        }
                    };
                    let __ptr = __A::class_ptr(&mut __map);
                    _serde::__private::Ok(hclBendLinkConstraintSetMxBatch {
                        __ptr: __ptr.clone(),
                        m_bendMinLengths,
                        m_stretchMaxLengths,
                        m_stretchStiffnesses,
                        m_bendStiffnesses,
                        m_invMassesA,
                        m_invMassesB,
                        m_particlesA,
                        m_particlesB,
                    })
                }
            }
            const FIELDS: &[&str] = &[
                "bendMinLengths",
                "stretchMaxLengths",
                "stretchStiffnesses",
                "bendStiffnesses",
                "invMassesA",
                "invMassesB",
                "particlesA",
                "particlesB",
            ];
            _serde::Deserializer::deserialize_struct(
                deserializer,
                "hclBendLinkConstraintSetMxBatch",
                FIELDS,
                __hclBendLinkConstraintSetMxBatchVisitor {
                    marker: _serde::__private::PhantomData::<
                        hclBendLinkConstraintSetMxBatch,
                    >,
                    lifetime: _serde::__private::PhantomData,
                },
            )
        }
    }
};
