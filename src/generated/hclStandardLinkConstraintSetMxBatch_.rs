use super::class_requires::*;
use super::*;
/// # C++ Info
/// - name: `hclStandardLinkConstraintSetMxBatch`
/// - version: `0`
/// - signature: `0x7c3c5dce`
/// - size: `  0`(x86)/`256`(x86_64)
/// -  vtable: `false`
#[allow(non_upper_case_globals, non_snake_case)]
#[cfg_attr(feature = "json_schema", derive(schemars::JsonSchema))]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(educe::Educe)]
#[educe(Debug, Clone, Default, PartialEq)]
pub struct hclStandardLinkConstraintSetMxBatch<'a> {
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
    /// - name: `restLengths`(ctype: `hkReal[16]`)
    /// - offset: `  0`(x86)/`  0`(x86_64)
    /// - type_size: `  0`(x86)/` 64`(x86_64)
    #[cfg_attr(feature = "serde_default", serde(default))]
    #[cfg_attr(feature = "json_schema", schemars(rename = "restLengths"))]
    #[cfg_attr(feature = "serde", serde(rename = "restLengths"))]
    pub m_restLengths: [f32; 16usize],
    /// # C++ Info
    /// - name: `stiffnessesA`(ctype: `hkReal[16]`)
    /// - offset: `  0`(x86)/` 64`(x86_64)
    /// - type_size: `  0`(x86)/` 64`(x86_64)
    #[cfg_attr(feature = "serde_default", serde(default))]
    #[cfg_attr(feature = "json_schema", schemars(rename = "stiffnessesA"))]
    #[cfg_attr(feature = "serde", serde(rename = "stiffnessesA"))]
    pub m_stiffnessesA: [f32; 16usize],
    /// # C++ Info
    /// - name: `stiffnessesB`(ctype: `hkReal[16]`)
    /// - offset: `  0`(x86)/`128`(x86_64)
    /// - type_size: `  0`(x86)/` 64`(x86_64)
    #[cfg_attr(feature = "serde_default", serde(default))]
    #[cfg_attr(feature = "json_schema", schemars(rename = "stiffnessesB"))]
    #[cfg_attr(feature = "serde", serde(rename = "stiffnessesB"))]
    pub m_stiffnessesB: [f32; 16usize],
    /// # C++ Info
    /// - name: `particlesA`(ctype: `hkUint16[16]`)
    /// - offset: `  0`(x86)/`192`(x86_64)
    /// - type_size: `  0`(x86)/` 32`(x86_64)
    #[cfg_attr(feature = "serde_default", serde(default))]
    #[cfg_attr(feature = "serde", serde(borrow))]
    #[cfg_attr(feature = "json_schema", schemars(rename = "particlesA"))]
    #[cfg_attr(feature = "serde", serde(rename = "particlesA"))]
    pub m_particlesA: [U16<'a>; 16usize],
    /// # C++ Info
    /// - name: `particlesB`(ctype: `hkUint16[16]`)
    /// - offset: `  0`(x86)/`224`(x86_64)
    /// - type_size: `  0`(x86)/` 32`(x86_64)
    #[cfg_attr(feature = "serde_default", serde(default))]
    #[cfg_attr(feature = "serde", serde(borrow))]
    #[cfg_attr(feature = "json_schema", schemars(rename = "particlesB"))]
    #[cfg_attr(feature = "serde", serde(rename = "particlesB"))]
    pub m_particlesB: [U16<'a>; 16usize],
}
const _: () = {
    use havok_serde as _serde;
    impl<'a> _serde::HavokClass for hclStandardLinkConstraintSetMxBatch<'a> {
        #[inline]
        fn name(&self) -> &'static str {
            "hclStandardLinkConstraintSetMxBatch"
        }
        #[inline]
        fn signature(&self) -> _serde::__private::Signature {
            _serde::__private::Signature::new(0x7c3c5dce)
        }
        #[allow(clippy::let_and_return, clippy::vec_init_then_push)]
        fn deps_indexes(&self) -> Vec<&Pointer<'_>> {
            let mut v = Vec::new();
            v
        }
    }
    impl<'a> _serde::Serialize for hclStandardLinkConstraintSetMxBatch<'a> {
        fn serialize<S>(&self, __serializer: S) -> Result<S::Ok, S::Error>
        where
            S: _serde::ser::Serializer,
        {
            let class_meta = self
                .__ptr
                .as_ref()
                .map(|name| (name, _serde::__private::Signature::new(0x7c3c5dce)));
            let mut serializer = __serializer
                .serialize_struct(
                    "hclStandardLinkConstraintSetMxBatch",
                    class_meta,
                    (0u64, 256u64),
                )?;
            serializer
                .serialize_fixed_array_field(
                    "restLengths",
                    self.m_restLengths.as_slice(),
                    TypeSize::NonPtr,
                )?;
            serializer
                .serialize_fixed_array_field(
                    "stiffnessesA",
                    self.m_stiffnessesA.as_slice(),
                    TypeSize::NonPtr,
                )?;
            serializer
                .serialize_fixed_array_field(
                    "stiffnessesB",
                    self.m_stiffnessesB.as_slice(),
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
    impl<'de> _serde::Deserialize<'de> for hclStandardLinkConstraintSetMxBatch<'de> {
        fn deserialize<__D>(deserializer: __D) -> core::result::Result<Self, __D::Error>
        where
            __D: _serde::Deserializer<'de>,
        {
            #[allow(non_camel_case_types)]
            enum __Field {
                m_restLengths,
                m_stiffnessesA,
                m_stiffnessesB,
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
                        "restLengths" => Ok(__Field::m_restLengths),
                        "stiffnessesA" => Ok(__Field::m_stiffnessesA),
                        "stiffnessesB" => Ok(__Field::m_stiffnessesB),
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
            struct __hclStandardLinkConstraintSetMxBatchVisitor<'de> {
                marker: _serde::__private::PhantomData<
                    hclStandardLinkConstraintSetMxBatch<'de>,
                >,
                lifetime: _serde::__private::PhantomData<&'de ()>,
            }
            #[allow(clippy::match_single_binding)]
            #[allow(clippy::reversed_empty_ranges)]
            #[allow(clippy::single_match)]
            impl<'de> _serde::de::Visitor<'de>
            for __hclStandardLinkConstraintSetMxBatchVisitor<'de> {
                type Value = hclStandardLinkConstraintSetMxBatch<'de>;
                fn expecting(
                    &self,
                    __formatter: &mut core::fmt::Formatter,
                ) -> core::fmt::Result {
                    core::fmt::Formatter::write_str(
                        __formatter,
                        "struct hclStandardLinkConstraintSetMxBatch",
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
                    let mut m_restLengths: _serde::__private::Option<[f32; 16usize]> = _serde::__private::None;
                    let mut m_stiffnessesA: _serde::__private::Option<[f32; 16usize]> = _serde::__private::None;
                    let mut m_stiffnessesB: _serde::__private::Option<[f32; 16usize]> = _serde::__private::None;
                    let mut m_particlesA: _serde::__private::Option<
                        [U16<'de>; 16usize],
                    > = _serde::__private::None;
                    let mut m_particlesB: _serde::__private::Option<
                        [U16<'de>; 16usize],
                    > = _serde::__private::None;
                    for i in 0..5usize {
                        match i {
                            0usize => {
                                if _serde::__private::Option::is_some(&m_restLengths) {
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "restLengths",
                                        ),
                                    );
                                }
                                m_restLengths = _serde::__private::Some(
                                    match __A::next_value::<[f32; 16usize]>(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            1usize => {
                                if _serde::__private::Option::is_some(&m_stiffnessesA) {
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "stiffnessesA",
                                        ),
                                    );
                                }
                                m_stiffnessesA = _serde::__private::Some(
                                    match __A::next_value::<[f32; 16usize]>(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            2usize => {
                                if _serde::__private::Option::is_some(&m_stiffnessesB) {
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "stiffnessesB",
                                        ),
                                    );
                                }
                                m_stiffnessesB = _serde::__private::Some(
                                    match __A::next_value::<[f32; 16usize]>(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            3usize => {
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
                            4usize => {
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
                    let m_restLengths = match m_restLengths {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "restLengths",
                                ),
                            );
                        }
                    };
                    let m_stiffnessesA = match m_stiffnessesA {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "stiffnessesA",
                                ),
                            );
                        }
                    };
                    let m_stiffnessesB = match m_stiffnessesB {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "stiffnessesB",
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
                    _serde::__private::Ok(hclStandardLinkConstraintSetMxBatch {
                        __ptr,
                        m_restLengths,
                        m_stiffnessesA,
                        m_stiffnessesB,
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
                    let mut m_restLengths: _serde::__private::Option<[f32; 16usize]> = _serde::__private::None;
                    let mut m_stiffnessesA: _serde::__private::Option<[f32; 16usize]> = _serde::__private::None;
                    let mut m_stiffnessesB: _serde::__private::Option<[f32; 16usize]> = _serde::__private::None;
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
                            __Field::m_restLengths => {
                                #[cfg(
                                    any(feature = "strict", feature = "ignore_duplicates")
                                )]
                                if _serde::__private::Option::is_some(&m_restLengths) {
                                    #[cfg(feature = "ignore_duplicates")]
                                    {
                                        __A::skip_value(&mut __map)?;
                                        continue;
                                    }
                                    #[cfg(feature = "strict")]
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "restLengths",
                                        ),
                                    );
                                }
                                m_restLengths = _serde::__private::Some(
                                    match __A::next_value::<[f32; 16usize]>(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            __Field::m_stiffnessesA => {
                                #[cfg(
                                    any(feature = "strict", feature = "ignore_duplicates")
                                )]
                                if _serde::__private::Option::is_some(&m_stiffnessesA) {
                                    #[cfg(feature = "ignore_duplicates")]
                                    {
                                        __A::skip_value(&mut __map)?;
                                        continue;
                                    }
                                    #[cfg(feature = "strict")]
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "stiffnessesA",
                                        ),
                                    );
                                }
                                m_stiffnessesA = _serde::__private::Some(
                                    match __A::next_value::<[f32; 16usize]>(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            __Field::m_stiffnessesB => {
                                #[cfg(
                                    any(feature = "strict", feature = "ignore_duplicates")
                                )]
                                if _serde::__private::Option::is_some(&m_stiffnessesB) {
                                    #[cfg(feature = "ignore_duplicates")]
                                    {
                                        __A::skip_value(&mut __map)?;
                                        continue;
                                    }
                                    #[cfg(feature = "strict")]
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "stiffnessesB",
                                        ),
                                    );
                                }
                                m_stiffnessesB = _serde::__private::Some(
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
                    let m_restLengths = match m_restLengths {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            #[cfg(feature = "strict")]
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "restLengths",
                                ),
                            );
                            #[cfg(not(feature = "strict"))] Default::default()
                        }
                    };
                    let m_stiffnessesA = match m_stiffnessesA {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            #[cfg(feature = "strict")]
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "stiffnessesA",
                                ),
                            );
                            #[cfg(not(feature = "strict"))] Default::default()
                        }
                    };
                    let m_stiffnessesB = match m_stiffnessesB {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            #[cfg(feature = "strict")]
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "stiffnessesB",
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
                    _serde::__private::Ok(hclStandardLinkConstraintSetMxBatch {
                        __ptr: __ptr.clone(),
                        m_restLengths,
                        m_stiffnessesA,
                        m_stiffnessesB,
                        m_particlesA,
                        m_particlesB,
                    })
                }
            }
            const FIELDS: &[&str] = &[
                "restLengths",
                "stiffnessesA",
                "stiffnessesB",
                "particlesA",
                "particlesB",
            ];
            _serde::Deserializer::deserialize_struct(
                deserializer,
                "hclStandardLinkConstraintSetMxBatch",
                FIELDS,
                __hclStandardLinkConstraintSetMxBatchVisitor {
                    marker: _serde::__private::PhantomData::<
                        hclStandardLinkConstraintSetMxBatch,
                    >,
                    lifetime: _serde::__private::PhantomData,
                },
            )
        }
    }
};
