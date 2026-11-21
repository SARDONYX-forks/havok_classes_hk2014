use super::class_requires::*;
use super::*;
/// # C++ Info
/// - name: `hclTransitionConstraintSet`
/// - version: `1`
/// - signature: `0x9cfe2c7d`
/// - size: `  0`(x86)/` 72`(x86_64)
/// -  vtable: `true`
#[allow(non_upper_case_globals, non_snake_case)]
#[cfg_attr(feature = "json_schema", derive(schemars::JsonSchema))]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(educe::Educe)]
#[educe(Debug, Clone, Default, PartialEq)]
pub struct hclTransitionConstraintSet<'a> {
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
    /// Alternative to C++ class inheritance.
    #[cfg_attr(feature = "json_schema", schemars(flatten))]
    #[cfg_attr(feature = "serde", serde(flatten))]
    #[cfg_attr(feature = "serde", serde(borrow))]
    pub parent: hclConstraintSet<'a>,
    /// # C++ Info
    /// - name: `perParticleData`(ctype: `hkArray<struct hclTransitionConstraintSetPerParticle>`)
    /// - offset: `  0`(x86)/` 32`(x86_64)
    /// - type_size: `  0`(x86)/` 16`(x86_64)
    #[cfg_attr(feature = "serde_default", serde(default))]
    #[cfg_attr(feature = "serde", serde(borrow))]
    #[cfg_attr(feature = "json_schema", schemars(rename = "perParticleData"))]
    #[cfg_attr(feature = "serde", serde(rename = "perParticleData"))]
    pub m_perParticleData: Vec<hclTransitionConstraintSetPerParticle<'a>>,
    /// # C++ Info
    /// - name: `toAnimPeriod`(ctype: `hkReal`)
    /// - offset: `  0`(x86)/` 48`(x86_64)
    /// - type_size: `  0`(x86)/`  4`(x86_64)
    #[cfg_attr(feature = "serde_default", serde(default))]
    #[cfg_attr(feature = "json_schema", schemars(rename = "toAnimPeriod"))]
    #[cfg_attr(feature = "serde", serde(rename = "toAnimPeriod"))]
    pub m_toAnimPeriod: f32,
    /// # C++ Info
    /// - name: `toAnimPlusDelayPeriod`(ctype: `hkReal`)
    /// - offset: `  0`(x86)/` 52`(x86_64)
    /// - type_size: `  0`(x86)/`  4`(x86_64)
    #[cfg_attr(feature = "serde_default", serde(default))]
    #[cfg_attr(feature = "json_schema", schemars(rename = "toAnimPlusDelayPeriod"))]
    #[cfg_attr(feature = "serde", serde(rename = "toAnimPlusDelayPeriod"))]
    pub m_toAnimPlusDelayPeriod: f32,
    /// # C++ Info
    /// - name: `toSimPeriod`(ctype: `hkReal`)
    /// - offset: `  0`(x86)/` 56`(x86_64)
    /// - type_size: `  0`(x86)/`  4`(x86_64)
    #[cfg_attr(feature = "serde_default", serde(default))]
    #[cfg_attr(feature = "json_schema", schemars(rename = "toSimPeriod"))]
    #[cfg_attr(feature = "serde", serde(rename = "toSimPeriod"))]
    pub m_toSimPeriod: f32,
    /// # C++ Info
    /// - name: `toSimPlusDelayPeriod`(ctype: `hkReal`)
    /// - offset: `  0`(x86)/` 60`(x86_64)
    /// - type_size: `  0`(x86)/`  4`(x86_64)
    #[cfg_attr(feature = "serde_default", serde(default))]
    #[cfg_attr(feature = "json_schema", schemars(rename = "toSimPlusDelayPeriod"))]
    #[cfg_attr(feature = "serde", serde(rename = "toSimPlusDelayPeriod"))]
    pub m_toSimPlusDelayPeriod: f32,
    /// # C++ Info
    /// - name: `referenceMeshBufferIdx`(ctype: `hkUint32`)
    /// - offset: `  0`(x86)/` 64`(x86_64)
    /// - type_size: `  0`(x86)/`  4`(x86_64)
    #[cfg_attr(feature = "serde_default", serde(default))]
    #[cfg_attr(feature = "serde", serde(borrow))]
    #[cfg_attr(feature = "json_schema", schemars(rename = "referenceMeshBufferIdx"))]
    #[cfg_attr(feature = "serde", serde(rename = "referenceMeshBufferIdx"))]
    pub m_referenceMeshBufferIdx: U32<'a>,
}
const _: () = {
    use havok_serde as _serde;
    impl<'a> _serde::HavokClass for hclTransitionConstraintSet<'a> {
        #[inline]
        fn name(&self) -> &'static str {
            "hclTransitionConstraintSet"
        }
        #[inline]
        fn signature(&self) -> _serde::__private::Signature {
            _serde::__private::Signature::new(0x9cfe2c7d)
        }
        #[allow(clippy::let_and_return, clippy::vec_init_then_push)]
        fn deps_indexes(&self) -> Vec<&Pointer<'_>> {
            let mut v = Vec::new();
            v.extend(
                self
                    .m_perParticleData
                    .iter()
                    .flat_map(|class| class.deps_indexes())
                    .collect::<Vec<&Pointer<'_>>>(),
            );
            v
        }
    }
    impl<'a> _serde::Serialize for hclTransitionConstraintSet<'a> {
        fn serialize<S>(&self, __serializer: S) -> Result<S::Ok, S::Error>
        where
            S: _serde::ser::Serializer,
        {
            let class_meta = self
                .__ptr
                .as_ref()
                .map(|name| (name, _serde::__private::Signature::new(0x9cfe2c7d)));
            let mut serializer = __serializer
                .serialize_struct(
                    "hclTransitionConstraintSet",
                    class_meta,
                    (0u64, 72u64),
                )?;
            serializer.pad_field([0u8; 0usize].as_slice(), [0u8; 8usize].as_slice())?;
            serializer
                .skip_field(
                    "memSizeAndRefCount",
                    &self.parent.parent.m_memSizeAndRefCount,
                )?;
            serializer.pad_field([0u8; 0usize].as_slice(), [0u8; 4usize].as_slice())?;
            serializer.serialize_field("name", &self.parent.m_name)?;
            serializer.skip_field("type", &self.parent.m_type)?;
            serializer.pad_field([0u8; 0usize].as_slice(), [0u8; 4usize].as_slice())?;
            serializer
                .serialize_array_field(
                    "perParticleData",
                    &self.m_perParticleData,
                    TypeSize::Struct {
                        size_x86: 0u64,
                        size_x86_64: 16u64,
                    },
                )?;
            serializer.serialize_field("toAnimPeriod", &self.m_toAnimPeriod)?;
            serializer
                .serialize_field(
                    "toAnimPlusDelayPeriod",
                    &self.m_toAnimPlusDelayPeriod,
                )?;
            serializer.serialize_field("toSimPeriod", &self.m_toSimPeriod)?;
            serializer
                .serialize_field("toSimPlusDelayPeriod", &self.m_toSimPlusDelayPeriod)?;
            serializer
                .serialize_field(
                    "referenceMeshBufferIdx",
                    &self.m_referenceMeshBufferIdx,
                )?;
            serializer.pad_field([0u8; 0usize].as_slice(), [0u8; 4usize].as_slice())?;
            serializer.end()
        }
    }
};
#[doc(hidden)]
#[allow(non_upper_case_globals, unused_attributes, unused_qualifications)]
const _: () = {
    use havok_serde as _serde;
    #[automatically_derived]
    impl<'de> _serde::Deserialize<'de> for hclTransitionConstraintSet<'de> {
        fn deserialize<__D>(deserializer: __D) -> core::result::Result<Self, __D::Error>
        where
            __D: _serde::Deserializer<'de>,
        {
            #[allow(non_camel_case_types)]
            enum __Field {
                m_name,
                m_perParticleData,
                m_toAnimPeriod,
                m_toAnimPlusDelayPeriod,
                m_toSimPeriod,
                m_toSimPlusDelayPeriod,
                m_referenceMeshBufferIdx,
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
                        "name" => Ok(__Field::m_name),
                        "perParticleData" => Ok(__Field::m_perParticleData),
                        "toAnimPeriod" => Ok(__Field::m_toAnimPeriod),
                        "toAnimPlusDelayPeriod" => Ok(__Field::m_toAnimPlusDelayPeriod),
                        "toSimPeriod" => Ok(__Field::m_toSimPeriod),
                        "toSimPlusDelayPeriod" => Ok(__Field::m_toSimPlusDelayPeriod),
                        "referenceMeshBufferIdx" => Ok(__Field::m_referenceMeshBufferIdx),
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
            struct __hclTransitionConstraintSetVisitor<'de> {
                marker: _serde::__private::PhantomData<hclTransitionConstraintSet<'de>>,
                lifetime: _serde::__private::PhantomData<&'de ()>,
            }
            #[allow(clippy::match_single_binding)]
            #[allow(clippy::reversed_empty_ranges)]
            #[allow(clippy::single_match)]
            impl<'de> _serde::de::Visitor<'de>
            for __hclTransitionConstraintSetVisitor<'de> {
                type Value = hclTransitionConstraintSet<'de>;
                fn expecting(
                    &self,
                    __formatter: &mut core::fmt::Formatter,
                ) -> core::fmt::Result {
                    core::fmt::Formatter::write_str(
                        __formatter,
                        "struct hclTransitionConstraintSet",
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
                    let parent = __A::parent_value(&mut __map)?;
                    let mut m_perParticleData: _serde::__private::Option<
                        Vec<hclTransitionConstraintSetPerParticle<'de>>,
                    > = _serde::__private::None;
                    let mut m_toAnimPeriod: _serde::__private::Option<f32> = _serde::__private::None;
                    let mut m_toAnimPlusDelayPeriod: _serde::__private::Option<f32> = _serde::__private::None;
                    let mut m_toSimPeriod: _serde::__private::Option<f32> = _serde::__private::None;
                    let mut m_toSimPlusDelayPeriod: _serde::__private::Option<f32> = _serde::__private::None;
                    let mut m_referenceMeshBufferIdx: _serde::__private::Option<
                        U32<'de>,
                    > = _serde::__private::None;
                    for i in 0..6usize {
                        match i {
                            0usize => {
                                if _serde::__private::Option::is_some(&m_perParticleData) {
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "perParticleData",
                                        ),
                                    );
                                }
                                m_perParticleData = _serde::__private::Some(
                                    match __A::next_value::<
                                        Vec<hclTransitionConstraintSetPerParticle<'de>>,
                                    >(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            1usize => {
                                if _serde::__private::Option::is_some(&m_toAnimPeriod) {
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "toAnimPeriod",
                                        ),
                                    );
                                }
                                m_toAnimPeriod = _serde::__private::Some(
                                    match __A::next_value::<f32>(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            2usize => {
                                if _serde::__private::Option::is_some(
                                    &m_toAnimPlusDelayPeriod,
                                ) {
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "toAnimPlusDelayPeriod",
                                        ),
                                    );
                                }
                                m_toAnimPlusDelayPeriod = _serde::__private::Some(
                                    match __A::next_value::<f32>(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            3usize => {
                                if _serde::__private::Option::is_some(&m_toSimPeriod) {
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "toSimPeriod",
                                        ),
                                    );
                                }
                                m_toSimPeriod = _serde::__private::Some(
                                    match __A::next_value::<f32>(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            4usize => {
                                if _serde::__private::Option::is_some(
                                    &m_toSimPlusDelayPeriod,
                                ) {
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "toSimPlusDelayPeriod",
                                        ),
                                    );
                                }
                                m_toSimPlusDelayPeriod = _serde::__private::Some(
                                    match __A::next_value::<f32>(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            5usize => {
                                if _serde::__private::Option::is_some(
                                    &m_referenceMeshBufferIdx,
                                ) {
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "referenceMeshBufferIdx",
                                        ),
                                    );
                                }
                                m_referenceMeshBufferIdx = _serde::__private::Some(
                                    match __A::next_value::<U32<'de>>(&mut __map) {
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
                    __A::pad(&mut __map, 0usize, 4usize)?;
                    let m_perParticleData = match m_perParticleData {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "perParticleData",
                                ),
                            );
                        }
                    };
                    let m_toAnimPeriod = match m_toAnimPeriod {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "toAnimPeriod",
                                ),
                            );
                        }
                    };
                    let m_toAnimPlusDelayPeriod = match m_toAnimPlusDelayPeriod {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "toAnimPlusDelayPeriod",
                                ),
                            );
                        }
                    };
                    let m_toSimPeriod = match m_toSimPeriod {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "toSimPeriod",
                                ),
                            );
                        }
                    };
                    let m_toSimPlusDelayPeriod = match m_toSimPlusDelayPeriod {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "toSimPlusDelayPeriod",
                                ),
                            );
                        }
                    };
                    let m_referenceMeshBufferIdx = match m_referenceMeshBufferIdx {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "referenceMeshBufferIdx",
                                ),
                            );
                        }
                    };
                    _serde::__private::Ok(hclTransitionConstraintSet {
                        __ptr,
                        parent,
                        m_perParticleData,
                        m_toAnimPeriod,
                        m_toAnimPlusDelayPeriod,
                        m_toSimPeriod,
                        m_toSimPlusDelayPeriod,
                        m_referenceMeshBufferIdx,
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
                    let mut m_name: _serde::__private::Option<StringPtr<'de>> = _serde::__private::None;
                    let mut m_perParticleData: _serde::__private::Option<
                        Vec<hclTransitionConstraintSetPerParticle<'de>>,
                    > = _serde::__private::None;
                    let mut m_toAnimPeriod: _serde::__private::Option<f32> = _serde::__private::None;
                    let mut m_toAnimPlusDelayPeriod: _serde::__private::Option<f32> = _serde::__private::None;
                    let mut m_toSimPeriod: _serde::__private::Option<f32> = _serde::__private::None;
                    let mut m_toSimPlusDelayPeriod: _serde::__private::Option<f32> = _serde::__private::None;
                    let mut m_referenceMeshBufferIdx: _serde::__private::Option<
                        U32<'de>,
                    > = _serde::__private::None;
                    while let _serde::__private::Some(__key) = {
                        __A::next_key::<__Field>(&mut __map)?
                    } {
                        match __key {
                            __Field::m_name => {
                                #[cfg(
                                    any(feature = "strict", feature = "ignore_duplicates")
                                )]
                                if _serde::__private::Option::is_some(&m_name) {
                                    #[cfg(feature = "ignore_duplicates")]
                                    {
                                        __A::skip_value(&mut __map)?;
                                        continue;
                                    }
                                    #[cfg(feature = "strict")]
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field("name"),
                                    );
                                }
                                m_name = _serde::__private::Some(
                                    match __A::next_value::<StringPtr<'de>>(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            __Field::m_perParticleData => {
                                #[cfg(
                                    any(feature = "strict", feature = "ignore_duplicates")
                                )]
                                if _serde::__private::Option::is_some(&m_perParticleData) {
                                    #[cfg(feature = "ignore_duplicates")]
                                    {
                                        __A::skip_value(&mut __map)?;
                                        continue;
                                    }
                                    #[cfg(feature = "strict")]
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "perParticleData",
                                        ),
                                    );
                                }
                                m_perParticleData = _serde::__private::Some(
                                    match __A::next_value::<
                                        Vec<hclTransitionConstraintSetPerParticle<'de>>,
                                    >(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            __Field::m_toAnimPeriod => {
                                #[cfg(
                                    any(feature = "strict", feature = "ignore_duplicates")
                                )]
                                if _serde::__private::Option::is_some(&m_toAnimPeriod) {
                                    #[cfg(feature = "ignore_duplicates")]
                                    {
                                        __A::skip_value(&mut __map)?;
                                        continue;
                                    }
                                    #[cfg(feature = "strict")]
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "toAnimPeriod",
                                        ),
                                    );
                                }
                                m_toAnimPeriod = _serde::__private::Some(
                                    match __A::next_value::<f32>(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            __Field::m_toAnimPlusDelayPeriod => {
                                #[cfg(
                                    any(feature = "strict", feature = "ignore_duplicates")
                                )]
                                if _serde::__private::Option::is_some(
                                    &m_toAnimPlusDelayPeriod,
                                ) {
                                    #[cfg(feature = "ignore_duplicates")]
                                    {
                                        __A::skip_value(&mut __map)?;
                                        continue;
                                    }
                                    #[cfg(feature = "strict")]
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "toAnimPlusDelayPeriod",
                                        ),
                                    );
                                }
                                m_toAnimPlusDelayPeriod = _serde::__private::Some(
                                    match __A::next_value::<f32>(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            __Field::m_toSimPeriod => {
                                #[cfg(
                                    any(feature = "strict", feature = "ignore_duplicates")
                                )]
                                if _serde::__private::Option::is_some(&m_toSimPeriod) {
                                    #[cfg(feature = "ignore_duplicates")]
                                    {
                                        __A::skip_value(&mut __map)?;
                                        continue;
                                    }
                                    #[cfg(feature = "strict")]
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "toSimPeriod",
                                        ),
                                    );
                                }
                                m_toSimPeriod = _serde::__private::Some(
                                    match __A::next_value::<f32>(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            __Field::m_toSimPlusDelayPeriod => {
                                #[cfg(
                                    any(feature = "strict", feature = "ignore_duplicates")
                                )]
                                if _serde::__private::Option::is_some(
                                    &m_toSimPlusDelayPeriod,
                                ) {
                                    #[cfg(feature = "ignore_duplicates")]
                                    {
                                        __A::skip_value(&mut __map)?;
                                        continue;
                                    }
                                    #[cfg(feature = "strict")]
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "toSimPlusDelayPeriod",
                                        ),
                                    );
                                }
                                m_toSimPlusDelayPeriod = _serde::__private::Some(
                                    match __A::next_value::<f32>(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            __Field::m_referenceMeshBufferIdx => {
                                #[cfg(
                                    any(feature = "strict", feature = "ignore_duplicates")
                                )]
                                if _serde::__private::Option::is_some(
                                    &m_referenceMeshBufferIdx,
                                ) {
                                    #[cfg(feature = "ignore_duplicates")]
                                    {
                                        __A::skip_value(&mut __map)?;
                                        continue;
                                    }
                                    #[cfg(feature = "strict")]
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "referenceMeshBufferIdx",
                                        ),
                                    );
                                }
                                m_referenceMeshBufferIdx = _serde::__private::Some(
                                    match __A::next_value::<U32<'de>>(&mut __map) {
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
                    let m_name = match m_name {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            #[cfg(feature = "strict")]
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field("name"),
                            );
                            #[cfg(not(feature = "strict"))] Default::default()
                        }
                    };
                    let m_perParticleData = match m_perParticleData {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            #[cfg(feature = "strict")]
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "perParticleData",
                                ),
                            );
                            #[cfg(not(feature = "strict"))] Default::default()
                        }
                    };
                    let m_toAnimPeriod = match m_toAnimPeriod {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            #[cfg(feature = "strict")]
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "toAnimPeriod",
                                ),
                            );
                            #[cfg(not(feature = "strict"))] Default::default()
                        }
                    };
                    let m_toAnimPlusDelayPeriod = match m_toAnimPlusDelayPeriod {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            #[cfg(feature = "strict")]
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "toAnimPlusDelayPeriod",
                                ),
                            );
                            #[cfg(not(feature = "strict"))] Default::default()
                        }
                    };
                    let m_toSimPeriod = match m_toSimPeriod {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            #[cfg(feature = "strict")]
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "toSimPeriod",
                                ),
                            );
                            #[cfg(not(feature = "strict"))] Default::default()
                        }
                    };
                    let m_toSimPlusDelayPeriod = match m_toSimPlusDelayPeriod {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            #[cfg(feature = "strict")]
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "toSimPlusDelayPeriod",
                                ),
                            );
                            #[cfg(not(feature = "strict"))] Default::default()
                        }
                    };
                    let m_referenceMeshBufferIdx = match m_referenceMeshBufferIdx {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            #[cfg(feature = "strict")]
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "referenceMeshBufferIdx",
                                ),
                            );
                            #[cfg(not(feature = "strict"))] Default::default()
                        }
                    };
                    let __ptr = None;
                    let parent = hkBaseObject {
                        __ptr: __ptr.clone(),
                    };
                    let parent = hkReferencedObject {
                        __ptr: __ptr.clone(),
                        parent,
                        ..Default::default()
                    };
                    let parent = hclConstraintSet {
                        __ptr: __ptr.clone(),
                        parent,
                        m_name,
                        ..Default::default()
                    };
                    let __ptr = __A::class_ptr(&mut __map);
                    _serde::__private::Ok(hclTransitionConstraintSet {
                        __ptr: __ptr.clone(),
                        parent,
                        m_perParticleData,
                        m_toAnimPeriod,
                        m_toAnimPlusDelayPeriod,
                        m_toSimPeriod,
                        m_toSimPlusDelayPeriod,
                        m_referenceMeshBufferIdx,
                    })
                }
            }
            const FIELDS: &[&str] = &[
                "perParticleData",
                "toAnimPeriod",
                "toAnimPlusDelayPeriod",
                "toSimPeriod",
                "toSimPlusDelayPeriod",
                "referenceMeshBufferIdx",
            ];
            _serde::Deserializer::deserialize_struct(
                deserializer,
                "hclTransitionConstraintSet",
                FIELDS,
                __hclTransitionConstraintSetVisitor {
                    marker: _serde::__private::PhantomData::<hclTransitionConstraintSet>,
                    lifetime: _serde::__private::PhantomData,
                },
            )
        }
    }
};
