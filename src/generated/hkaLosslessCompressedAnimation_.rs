use super::class_requires::*;
use super::*;
/// # C++ Info
/// - name: `hkaLosslessCompressedAnimation`
/// - version: `0`
/// - signature: `0x278bffe8`
/// - size: `  0`(x86)/`224`(x86_64)
/// -  vtable: `true`
#[allow(non_upper_case_globals, non_snake_case)]
#[cfg_attr(feature = "json_schema", derive(schemars::JsonSchema))]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(educe::Educe)]
#[educe(Debug, Clone, Default, PartialEq)]
pub struct hkaLosslessCompressedAnimation<'a> {
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
    pub parent: hkaAnimation<'a>,
    /// # C++ Info
    /// - name: `dynamicTranslations`(ctype: `hkArray<hkReal>`)
    /// - offset: `  0`(x86)/` 56`(x86_64)
    /// - type_size: `  0`(x86)/` 16`(x86_64)
    #[cfg_attr(feature = "serde_default", serde(default))]
    #[cfg_attr(feature = "json_schema", schemars(rename = "dynamicTranslations"))]
    #[cfg_attr(feature = "serde", serde(rename = "dynamicTranslations"))]
    pub m_dynamicTranslations: Vec<f32>,
    /// # C++ Info
    /// - name: `staticTranslations`(ctype: `hkArray<hkReal>`)
    /// - offset: `  0`(x86)/` 72`(x86_64)
    /// - type_size: `  0`(x86)/` 16`(x86_64)
    #[cfg_attr(feature = "serde_default", serde(default))]
    #[cfg_attr(feature = "json_schema", schemars(rename = "staticTranslations"))]
    #[cfg_attr(feature = "serde", serde(rename = "staticTranslations"))]
    pub m_staticTranslations: Vec<f32>,
    /// # C++ Info
    /// - name: `translationTypeAndOffsets`(ctype: `hkArray<hkUint64>`)
    /// - offset: `  0`(x86)/` 88`(x86_64)
    /// - type_size: `  0`(x86)/` 16`(x86_64)
    #[cfg_attr(feature = "serde_default", serde(default))]
    #[cfg_attr(feature = "serde", serde(borrow))]
    #[cfg_attr(feature = "json_schema", schemars(rename = "translationTypeAndOffsets"))]
    #[cfg_attr(feature = "serde", serde(rename = "translationTypeAndOffsets"))]
    pub m_translationTypeAndOffsets: Vec<U64<'a>>,
    /// # C++ Info
    /// - name: `dynamicRotations`(ctype: `hkArray<hkQuaternion>`)
    /// - offset: `  0`(x86)/`104`(x86_64)
    /// - type_size: `  0`(x86)/` 16`(x86_64)
    #[cfg_attr(feature = "serde_default", serde(default))]
    #[cfg_attr(feature = "json_schema", schemars(rename = "dynamicRotations"))]
    #[cfg_attr(feature = "serde", serde(rename = "dynamicRotations"))]
    pub m_dynamicRotations: Vec<Quaternion>,
    /// # C++ Info
    /// - name: `staticRotations`(ctype: `hkArray<hkQuaternion>`)
    /// - offset: `  0`(x86)/`120`(x86_64)
    /// - type_size: `  0`(x86)/` 16`(x86_64)
    #[cfg_attr(feature = "serde_default", serde(default))]
    #[cfg_attr(feature = "json_schema", schemars(rename = "staticRotations"))]
    #[cfg_attr(feature = "serde", serde(rename = "staticRotations"))]
    pub m_staticRotations: Vec<Quaternion>,
    /// # C++ Info
    /// - name: `rotationTypeAndOffsets`(ctype: `hkArray<hkUint16>`)
    /// - offset: `  0`(x86)/`136`(x86_64)
    /// - type_size: `  0`(x86)/` 16`(x86_64)
    #[cfg_attr(feature = "serde_default", serde(default))]
    #[cfg_attr(feature = "serde", serde(borrow))]
    #[cfg_attr(feature = "json_schema", schemars(rename = "rotationTypeAndOffsets"))]
    #[cfg_attr(feature = "serde", serde(rename = "rotationTypeAndOffsets"))]
    pub m_rotationTypeAndOffsets: Vec<U16<'a>>,
    /// # C++ Info
    /// - name: `dynamicScales`(ctype: `hkArray<hkReal>`)
    /// - offset: `  0`(x86)/`152`(x86_64)
    /// - type_size: `  0`(x86)/` 16`(x86_64)
    #[cfg_attr(feature = "serde_default", serde(default))]
    #[cfg_attr(feature = "json_schema", schemars(rename = "dynamicScales"))]
    #[cfg_attr(feature = "serde", serde(rename = "dynamicScales"))]
    pub m_dynamicScales: Vec<f32>,
    /// # C++ Info
    /// - name: `staticScales`(ctype: `hkArray<hkReal>`)
    /// - offset: `  0`(x86)/`168`(x86_64)
    /// - type_size: `  0`(x86)/` 16`(x86_64)
    #[cfg_attr(feature = "serde_default", serde(default))]
    #[cfg_attr(feature = "json_schema", schemars(rename = "staticScales"))]
    #[cfg_attr(feature = "serde", serde(rename = "staticScales"))]
    pub m_staticScales: Vec<f32>,
    /// # C++ Info
    /// - name: `scaleTypeAndOffsets`(ctype: `hkArray<hkUint64>`)
    /// - offset: `  0`(x86)/`184`(x86_64)
    /// - type_size: `  0`(x86)/` 16`(x86_64)
    #[cfg_attr(feature = "serde_default", serde(default))]
    #[cfg_attr(feature = "serde", serde(borrow))]
    #[cfg_attr(feature = "json_schema", schemars(rename = "scaleTypeAndOffsets"))]
    #[cfg_attr(feature = "serde", serde(rename = "scaleTypeAndOffsets"))]
    pub m_scaleTypeAndOffsets: Vec<U64<'a>>,
    /// # C++ Info
    /// - name: `floats`(ctype: `hkArray<hkReal>`)
    /// - offset: `  0`(x86)/`200`(x86_64)
    /// - type_size: `  0`(x86)/` 16`(x86_64)
    #[cfg_attr(feature = "serde_default", serde(default))]
    #[cfg_attr(feature = "json_schema", schemars(rename = "floats"))]
    #[cfg_attr(feature = "serde", serde(rename = "floats"))]
    pub m_floats: Vec<f32>,
    /// # C++ Info
    /// - name: `numFrames`(ctype: `hkInt32`)
    /// - offset: `  0`(x86)/`216`(x86_64)
    /// - type_size: `  0`(x86)/`  4`(x86_64)
    #[cfg_attr(feature = "serde_default", serde(default))]
    #[cfg_attr(feature = "serde", serde(borrow))]
    #[cfg_attr(feature = "json_schema", schemars(rename = "numFrames"))]
    #[cfg_attr(feature = "serde", serde(rename = "numFrames"))]
    pub m_numFrames: I32<'a>,
}
const _: () = {
    use havok_serde as _serde;
    impl<'a> _serde::HavokClass for hkaLosslessCompressedAnimation<'a> {
        #[inline]
        fn name(&self) -> &'static str {
            "hkaLosslessCompressedAnimation"
        }
        #[inline]
        fn signature(&self) -> _serde::__private::Signature {
            _serde::__private::Signature::new(0x278bffe8)
        }
        #[allow(clippy::let_and_return, clippy::vec_init_then_push)]
        fn deps_indexes(&self) -> Vec<&Pointer<'_>> {
            let mut v = Vec::new();
            v.push(&self.parent.m_extractedMotion);
            v.extend(
                self
                    .parent
                    .m_annotationTracks
                    .iter()
                    .flat_map(|class| class.deps_indexes())
                    .collect::<Vec<&Pointer<'_>>>(),
            );
            v
        }
    }
    impl<'a> _serde::Serialize for hkaLosslessCompressedAnimation<'a> {
        fn serialize<S>(&self, __serializer: S) -> Result<S::Ok, S::Error>
        where
            S: _serde::ser::Serializer,
        {
            let class_meta = self
                .__ptr
                .as_ref()
                .map(|name| (name, _serde::__private::Signature::new(0x278bffe8)));
            let mut serializer = __serializer
                .serialize_struct(
                    "hkaLosslessCompressedAnimation",
                    class_meta,
                    (0u64, 224u64),
                )?;
            serializer.pad_field([0u8; 0usize].as_slice(), [0u8; 8usize].as_slice())?;
            serializer
                .skip_field(
                    "memSizeAndRefCount",
                    &self.parent.parent.m_memSizeAndRefCount,
                )?;
            serializer.pad_field([0u8; 0usize].as_slice(), [0u8; 4usize].as_slice())?;
            serializer.serialize_field("type", &self.parent.m_type)?;
            serializer.serialize_field("duration", &self.parent.m_duration)?;
            serializer
                .serialize_field(
                    "numberOfTransformTracks",
                    &self.parent.m_numberOfTransformTracks,
                )?;
            serializer
                .serialize_field(
                    "numberOfFloatTracks",
                    &self.parent.m_numberOfFloatTracks,
                )?;
            serializer
                .serialize_field("extractedMotion", &self.parent.m_extractedMotion)?;
            serializer
                .serialize_array_field(
                    "annotationTracks",
                    &self.parent.m_annotationTracks,
                    TypeSize::Struct {
                        size_x86: 0u64,
                        size_x86_64: 24u64,
                    },
                )?;
            serializer
                .serialize_array_field(
                    "dynamicTranslations",
                    &self.m_dynamicTranslations,
                    TypeSize::NonPtr,
                )?;
            serializer
                .serialize_array_field(
                    "staticTranslations",
                    &self.m_staticTranslations,
                    TypeSize::NonPtr,
                )?;
            serializer
                .serialize_array_field(
                    "translationTypeAndOffsets",
                    &self.m_translationTypeAndOffsets,
                    TypeSize::NonPtr,
                )?;
            serializer
                .serialize_array_field(
                    "dynamicRotations",
                    &self.m_dynamicRotations,
                    TypeSize::NonPtr,
                )?;
            serializer
                .serialize_array_field(
                    "staticRotations",
                    &self.m_staticRotations,
                    TypeSize::NonPtr,
                )?;
            serializer
                .serialize_array_field(
                    "rotationTypeAndOffsets",
                    &self.m_rotationTypeAndOffsets,
                    TypeSize::NonPtr,
                )?;
            serializer
                .serialize_array_field(
                    "dynamicScales",
                    &self.m_dynamicScales,
                    TypeSize::NonPtr,
                )?;
            serializer
                .serialize_array_field(
                    "staticScales",
                    &self.m_staticScales,
                    TypeSize::NonPtr,
                )?;
            serializer
                .serialize_array_field(
                    "scaleTypeAndOffsets",
                    &self.m_scaleTypeAndOffsets,
                    TypeSize::NonPtr,
                )?;
            serializer
                .serialize_array_field("floats", &self.m_floats, TypeSize::NonPtr)?;
            serializer.serialize_field("numFrames", &self.m_numFrames)?;
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
    impl<'de> _serde::Deserialize<'de> for hkaLosslessCompressedAnimation<'de> {
        fn deserialize<__D>(deserializer: __D) -> core::result::Result<Self, __D::Error>
        where
            __D: _serde::Deserializer<'de>,
        {
            #[allow(non_camel_case_types)]
            enum __Field {
                m_type,
                m_duration,
                m_numberOfTransformTracks,
                m_numberOfFloatTracks,
                m_extractedMotion,
                m_annotationTracks,
                m_dynamicTranslations,
                m_staticTranslations,
                m_translationTypeAndOffsets,
                m_dynamicRotations,
                m_staticRotations,
                m_rotationTypeAndOffsets,
                m_dynamicScales,
                m_staticScales,
                m_scaleTypeAndOffsets,
                m_floats,
                m_numFrames,
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
                        "type" => Ok(__Field::m_type),
                        "duration" => Ok(__Field::m_duration),
                        "numberOfTransformTracks" => {
                            Ok(__Field::m_numberOfTransformTracks)
                        }
                        "numberOfFloatTracks" => Ok(__Field::m_numberOfFloatTracks),
                        "extractedMotion" => Ok(__Field::m_extractedMotion),
                        "annotationTracks" => Ok(__Field::m_annotationTracks),
                        "dynamicTranslations" => Ok(__Field::m_dynamicTranslations),
                        "staticTranslations" => Ok(__Field::m_staticTranslations),
                        "translationTypeAndOffsets" => {
                            Ok(__Field::m_translationTypeAndOffsets)
                        }
                        "dynamicRotations" => Ok(__Field::m_dynamicRotations),
                        "staticRotations" => Ok(__Field::m_staticRotations),
                        "rotationTypeAndOffsets" => Ok(__Field::m_rotationTypeAndOffsets),
                        "dynamicScales" => Ok(__Field::m_dynamicScales),
                        "staticScales" => Ok(__Field::m_staticScales),
                        "scaleTypeAndOffsets" => Ok(__Field::m_scaleTypeAndOffsets),
                        "floats" => Ok(__Field::m_floats),
                        "numFrames" => Ok(__Field::m_numFrames),
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
            struct __hkaLosslessCompressedAnimationVisitor<'de> {
                marker: _serde::__private::PhantomData<
                    hkaLosslessCompressedAnimation<'de>,
                >,
                lifetime: _serde::__private::PhantomData<&'de ()>,
            }
            #[allow(clippy::match_single_binding)]
            #[allow(clippy::reversed_empty_ranges)]
            #[allow(clippy::single_match)]
            impl<'de> _serde::de::Visitor<'de>
            for __hkaLosslessCompressedAnimationVisitor<'de> {
                type Value = hkaLosslessCompressedAnimation<'de>;
                fn expecting(
                    &self,
                    __formatter: &mut core::fmt::Formatter,
                ) -> core::fmt::Result {
                    core::fmt::Formatter::write_str(
                        __formatter,
                        "struct hkaLosslessCompressedAnimation",
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
                    let mut m_dynamicTranslations: _serde::__private::Option<Vec<f32>> = _serde::__private::None;
                    let mut m_staticTranslations: _serde::__private::Option<Vec<f32>> = _serde::__private::None;
                    let mut m_translationTypeAndOffsets: _serde::__private::Option<
                        Vec<U64<'de>>,
                    > = _serde::__private::None;
                    let mut m_dynamicRotations: _serde::__private::Option<
                        Vec<Quaternion>,
                    > = _serde::__private::None;
                    let mut m_staticRotations: _serde::__private::Option<
                        Vec<Quaternion>,
                    > = _serde::__private::None;
                    let mut m_rotationTypeAndOffsets: _serde::__private::Option<
                        Vec<U16<'de>>,
                    > = _serde::__private::None;
                    let mut m_dynamicScales: _serde::__private::Option<Vec<f32>> = _serde::__private::None;
                    let mut m_staticScales: _serde::__private::Option<Vec<f32>> = _serde::__private::None;
                    let mut m_scaleTypeAndOffsets: _serde::__private::Option<
                        Vec<U64<'de>>,
                    > = _serde::__private::None;
                    let mut m_floats: _serde::__private::Option<Vec<f32>> = _serde::__private::None;
                    let mut m_numFrames: _serde::__private::Option<I32<'de>> = _serde::__private::None;
                    for i in 0..11usize {
                        match i {
                            0usize => {
                                if _serde::__private::Option::is_some(
                                    &m_dynamicTranslations,
                                ) {
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "dynamicTranslations",
                                        ),
                                    );
                                }
                                m_dynamicTranslations = _serde::__private::Some(
                                    match __A::next_value::<Vec<f32>>(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            1usize => {
                                if _serde::__private::Option::is_some(
                                    &m_staticTranslations,
                                ) {
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "staticTranslations",
                                        ),
                                    );
                                }
                                m_staticTranslations = _serde::__private::Some(
                                    match __A::next_value::<Vec<f32>>(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            2usize => {
                                if _serde::__private::Option::is_some(
                                    &m_translationTypeAndOffsets,
                                ) {
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "translationTypeAndOffsets",
                                        ),
                                    );
                                }
                                m_translationTypeAndOffsets = _serde::__private::Some(
                                    match __A::next_value::<Vec<U64<'de>>>(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            3usize => {
                                if _serde::__private::Option::is_some(&m_dynamicRotations) {
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "dynamicRotations",
                                        ),
                                    );
                                }
                                m_dynamicRotations = _serde::__private::Some(
                                    match __A::next_value::<Vec<Quaternion>>(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            4usize => {
                                if _serde::__private::Option::is_some(&m_staticRotations) {
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "staticRotations",
                                        ),
                                    );
                                }
                                m_staticRotations = _serde::__private::Some(
                                    match __A::next_value::<Vec<Quaternion>>(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            5usize => {
                                if _serde::__private::Option::is_some(
                                    &m_rotationTypeAndOffsets,
                                ) {
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "rotationTypeAndOffsets",
                                        ),
                                    );
                                }
                                m_rotationTypeAndOffsets = _serde::__private::Some(
                                    match __A::next_value::<Vec<U16<'de>>>(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            6usize => {
                                if _serde::__private::Option::is_some(&m_dynamicScales) {
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "dynamicScales",
                                        ),
                                    );
                                }
                                m_dynamicScales = _serde::__private::Some(
                                    match __A::next_value::<Vec<f32>>(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            7usize => {
                                if _serde::__private::Option::is_some(&m_staticScales) {
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "staticScales",
                                        ),
                                    );
                                }
                                m_staticScales = _serde::__private::Some(
                                    match __A::next_value::<Vec<f32>>(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            8usize => {
                                if _serde::__private::Option::is_some(
                                    &m_scaleTypeAndOffsets,
                                ) {
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "scaleTypeAndOffsets",
                                        ),
                                    );
                                }
                                m_scaleTypeAndOffsets = _serde::__private::Some(
                                    match __A::next_value::<Vec<U64<'de>>>(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            9usize => {
                                if _serde::__private::Option::is_some(&m_floats) {
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field("floats"),
                                    );
                                }
                                m_floats = _serde::__private::Some(
                                    match __A::next_value::<Vec<f32>>(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            10usize => {
                                if _serde::__private::Option::is_some(&m_numFrames) {
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "numFrames",
                                        ),
                                    );
                                }
                                m_numFrames = _serde::__private::Some(
                                    match __A::next_value::<I32<'de>>(&mut __map) {
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
                    let m_dynamicTranslations = match m_dynamicTranslations {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "dynamicTranslations",
                                ),
                            );
                        }
                    };
                    let m_staticTranslations = match m_staticTranslations {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "staticTranslations",
                                ),
                            );
                        }
                    };
                    let m_translationTypeAndOffsets = match m_translationTypeAndOffsets {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "translationTypeAndOffsets",
                                ),
                            );
                        }
                    };
                    let m_dynamicRotations = match m_dynamicRotations {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "dynamicRotations",
                                ),
                            );
                        }
                    };
                    let m_staticRotations = match m_staticRotations {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "staticRotations",
                                ),
                            );
                        }
                    };
                    let m_rotationTypeAndOffsets = match m_rotationTypeAndOffsets {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "rotationTypeAndOffsets",
                                ),
                            );
                        }
                    };
                    let m_dynamicScales = match m_dynamicScales {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "dynamicScales",
                                ),
                            );
                        }
                    };
                    let m_staticScales = match m_staticScales {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "staticScales",
                                ),
                            );
                        }
                    };
                    let m_scaleTypeAndOffsets = match m_scaleTypeAndOffsets {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "scaleTypeAndOffsets",
                                ),
                            );
                        }
                    };
                    let m_floats = match m_floats {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field("floats"),
                            );
                        }
                    };
                    let m_numFrames = match m_numFrames {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "numFrames",
                                ),
                            );
                        }
                    };
                    _serde::__private::Ok(hkaLosslessCompressedAnimation {
                        __ptr,
                        parent,
                        m_dynamicTranslations,
                        m_staticTranslations,
                        m_translationTypeAndOffsets,
                        m_dynamicRotations,
                        m_staticRotations,
                        m_rotationTypeAndOffsets,
                        m_dynamicScales,
                        m_staticScales,
                        m_scaleTypeAndOffsets,
                        m_floats,
                        m_numFrames,
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
                    let mut m_type: _serde::__private::Option<AnimationType> = _serde::__private::None;
                    let mut m_duration: _serde::__private::Option<f32> = _serde::__private::None;
                    let mut m_numberOfTransformTracks: _serde::__private::Option<
                        I32<'de>,
                    > = _serde::__private::None;
                    let mut m_numberOfFloatTracks: _serde::__private::Option<I32<'de>> = _serde::__private::None;
                    let mut m_extractedMotion: _serde::__private::Option<Pointer<'de>> = _serde::__private::None;
                    let mut m_annotationTracks: _serde::__private::Option<
                        Vec<hkaAnnotationTrack<'de>>,
                    > = _serde::__private::None;
                    let mut m_dynamicTranslations: _serde::__private::Option<Vec<f32>> = _serde::__private::None;
                    let mut m_staticTranslations: _serde::__private::Option<Vec<f32>> = _serde::__private::None;
                    let mut m_translationTypeAndOffsets: _serde::__private::Option<
                        Vec<U64<'de>>,
                    > = _serde::__private::None;
                    let mut m_dynamicRotations: _serde::__private::Option<
                        Vec<Quaternion>,
                    > = _serde::__private::None;
                    let mut m_staticRotations: _serde::__private::Option<
                        Vec<Quaternion>,
                    > = _serde::__private::None;
                    let mut m_rotationTypeAndOffsets: _serde::__private::Option<
                        Vec<U16<'de>>,
                    > = _serde::__private::None;
                    let mut m_dynamicScales: _serde::__private::Option<Vec<f32>> = _serde::__private::None;
                    let mut m_staticScales: _serde::__private::Option<Vec<f32>> = _serde::__private::None;
                    let mut m_scaleTypeAndOffsets: _serde::__private::Option<
                        Vec<U64<'de>>,
                    > = _serde::__private::None;
                    let mut m_floats: _serde::__private::Option<Vec<f32>> = _serde::__private::None;
                    let mut m_numFrames: _serde::__private::Option<I32<'de>> = _serde::__private::None;
                    while let _serde::__private::Some(__key) = {
                        __A::next_key::<__Field>(&mut __map)?
                    } {
                        match __key {
                            __Field::m_type => {
                                #[cfg(
                                    any(feature = "strict", feature = "ignore_duplicates")
                                )]
                                if _serde::__private::Option::is_some(&m_type) {
                                    #[cfg(feature = "ignore_duplicates")]
                                    {
                                        __A::skip_value(&mut __map)?;
                                        continue;
                                    }
                                    #[cfg(feature = "strict")]
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field("type"),
                                    );
                                }
                                m_type = _serde::__private::Some(
                                    match __A::next_value::<AnimationType>(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            __Field::m_duration => {
                                #[cfg(
                                    any(feature = "strict", feature = "ignore_duplicates")
                                )]
                                if _serde::__private::Option::is_some(&m_duration) {
                                    #[cfg(feature = "ignore_duplicates")]
                                    {
                                        __A::skip_value(&mut __map)?;
                                        continue;
                                    }
                                    #[cfg(feature = "strict")]
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "duration",
                                        ),
                                    );
                                }
                                m_duration = _serde::__private::Some(
                                    match __A::next_value::<f32>(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            __Field::m_numberOfTransformTracks => {
                                #[cfg(
                                    any(feature = "strict", feature = "ignore_duplicates")
                                )]
                                if _serde::__private::Option::is_some(
                                    &m_numberOfTransformTracks,
                                ) {
                                    #[cfg(feature = "ignore_duplicates")]
                                    {
                                        __A::skip_value(&mut __map)?;
                                        continue;
                                    }
                                    #[cfg(feature = "strict")]
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "numberOfTransformTracks",
                                        ),
                                    );
                                }
                                m_numberOfTransformTracks = _serde::__private::Some(
                                    match __A::next_value::<I32<'de>>(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            __Field::m_numberOfFloatTracks => {
                                #[cfg(
                                    any(feature = "strict", feature = "ignore_duplicates")
                                )]
                                if _serde::__private::Option::is_some(
                                    &m_numberOfFloatTracks,
                                ) {
                                    #[cfg(feature = "ignore_duplicates")]
                                    {
                                        __A::skip_value(&mut __map)?;
                                        continue;
                                    }
                                    #[cfg(feature = "strict")]
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "numberOfFloatTracks",
                                        ),
                                    );
                                }
                                m_numberOfFloatTracks = _serde::__private::Some(
                                    match __A::next_value::<I32<'de>>(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            __Field::m_extractedMotion => {
                                #[cfg(
                                    any(feature = "strict", feature = "ignore_duplicates")
                                )]
                                if _serde::__private::Option::is_some(&m_extractedMotion) {
                                    #[cfg(feature = "ignore_duplicates")]
                                    {
                                        __A::skip_value(&mut __map)?;
                                        continue;
                                    }
                                    #[cfg(feature = "strict")]
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "extractedMotion",
                                        ),
                                    );
                                }
                                m_extractedMotion = _serde::__private::Some(
                                    match __A::next_value::<Pointer<'de>>(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            __Field::m_annotationTracks => {
                                #[cfg(
                                    any(feature = "strict", feature = "ignore_duplicates")
                                )]
                                if _serde::__private::Option::is_some(&m_annotationTracks) {
                                    #[cfg(feature = "ignore_duplicates")]
                                    {
                                        __A::skip_value(&mut __map)?;
                                        continue;
                                    }
                                    #[cfg(feature = "strict")]
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "annotationTracks",
                                        ),
                                    );
                                }
                                m_annotationTracks = _serde::__private::Some(
                                    match __A::next_value::<
                                        Vec<hkaAnnotationTrack<'de>>,
                                    >(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            __Field::m_dynamicTranslations => {
                                #[cfg(
                                    any(feature = "strict", feature = "ignore_duplicates")
                                )]
                                if _serde::__private::Option::is_some(
                                    &m_dynamicTranslations,
                                ) {
                                    #[cfg(feature = "ignore_duplicates")]
                                    {
                                        __A::skip_value(&mut __map)?;
                                        continue;
                                    }
                                    #[cfg(feature = "strict")]
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "dynamicTranslations",
                                        ),
                                    );
                                }
                                m_dynamicTranslations = _serde::__private::Some(
                                    match __A::next_value::<Vec<f32>>(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            __Field::m_staticTranslations => {
                                #[cfg(
                                    any(feature = "strict", feature = "ignore_duplicates")
                                )]
                                if _serde::__private::Option::is_some(
                                    &m_staticTranslations,
                                ) {
                                    #[cfg(feature = "ignore_duplicates")]
                                    {
                                        __A::skip_value(&mut __map)?;
                                        continue;
                                    }
                                    #[cfg(feature = "strict")]
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "staticTranslations",
                                        ),
                                    );
                                }
                                m_staticTranslations = _serde::__private::Some(
                                    match __A::next_value::<Vec<f32>>(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            __Field::m_translationTypeAndOffsets => {
                                #[cfg(
                                    any(feature = "strict", feature = "ignore_duplicates")
                                )]
                                if _serde::__private::Option::is_some(
                                    &m_translationTypeAndOffsets,
                                ) {
                                    #[cfg(feature = "ignore_duplicates")]
                                    {
                                        __A::skip_value(&mut __map)?;
                                        continue;
                                    }
                                    #[cfg(feature = "strict")]
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "translationTypeAndOffsets",
                                        ),
                                    );
                                }
                                m_translationTypeAndOffsets = _serde::__private::Some(
                                    match __A::next_value::<Vec<U64<'de>>>(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            __Field::m_dynamicRotations => {
                                #[cfg(
                                    any(feature = "strict", feature = "ignore_duplicates")
                                )]
                                if _serde::__private::Option::is_some(&m_dynamicRotations) {
                                    #[cfg(feature = "ignore_duplicates")]
                                    {
                                        __A::skip_value(&mut __map)?;
                                        continue;
                                    }
                                    #[cfg(feature = "strict")]
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "dynamicRotations",
                                        ),
                                    );
                                }
                                m_dynamicRotations = _serde::__private::Some(
                                    match __A::next_value::<Vec<Quaternion>>(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            __Field::m_staticRotations => {
                                #[cfg(
                                    any(feature = "strict", feature = "ignore_duplicates")
                                )]
                                if _serde::__private::Option::is_some(&m_staticRotations) {
                                    #[cfg(feature = "ignore_duplicates")]
                                    {
                                        __A::skip_value(&mut __map)?;
                                        continue;
                                    }
                                    #[cfg(feature = "strict")]
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "staticRotations",
                                        ),
                                    );
                                }
                                m_staticRotations = _serde::__private::Some(
                                    match __A::next_value::<Vec<Quaternion>>(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            __Field::m_rotationTypeAndOffsets => {
                                #[cfg(
                                    any(feature = "strict", feature = "ignore_duplicates")
                                )]
                                if _serde::__private::Option::is_some(
                                    &m_rotationTypeAndOffsets,
                                ) {
                                    #[cfg(feature = "ignore_duplicates")]
                                    {
                                        __A::skip_value(&mut __map)?;
                                        continue;
                                    }
                                    #[cfg(feature = "strict")]
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "rotationTypeAndOffsets",
                                        ),
                                    );
                                }
                                m_rotationTypeAndOffsets = _serde::__private::Some(
                                    match __A::next_value::<Vec<U16<'de>>>(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            __Field::m_dynamicScales => {
                                #[cfg(
                                    any(feature = "strict", feature = "ignore_duplicates")
                                )]
                                if _serde::__private::Option::is_some(&m_dynamicScales) {
                                    #[cfg(feature = "ignore_duplicates")]
                                    {
                                        __A::skip_value(&mut __map)?;
                                        continue;
                                    }
                                    #[cfg(feature = "strict")]
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "dynamicScales",
                                        ),
                                    );
                                }
                                m_dynamicScales = _serde::__private::Some(
                                    match __A::next_value::<Vec<f32>>(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            __Field::m_staticScales => {
                                #[cfg(
                                    any(feature = "strict", feature = "ignore_duplicates")
                                )]
                                if _serde::__private::Option::is_some(&m_staticScales) {
                                    #[cfg(feature = "ignore_duplicates")]
                                    {
                                        __A::skip_value(&mut __map)?;
                                        continue;
                                    }
                                    #[cfg(feature = "strict")]
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "staticScales",
                                        ),
                                    );
                                }
                                m_staticScales = _serde::__private::Some(
                                    match __A::next_value::<Vec<f32>>(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            __Field::m_scaleTypeAndOffsets => {
                                #[cfg(
                                    any(feature = "strict", feature = "ignore_duplicates")
                                )]
                                if _serde::__private::Option::is_some(
                                    &m_scaleTypeAndOffsets,
                                ) {
                                    #[cfg(feature = "ignore_duplicates")]
                                    {
                                        __A::skip_value(&mut __map)?;
                                        continue;
                                    }
                                    #[cfg(feature = "strict")]
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "scaleTypeAndOffsets",
                                        ),
                                    );
                                }
                                m_scaleTypeAndOffsets = _serde::__private::Some(
                                    match __A::next_value::<Vec<U64<'de>>>(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            __Field::m_floats => {
                                #[cfg(
                                    any(feature = "strict", feature = "ignore_duplicates")
                                )]
                                if _serde::__private::Option::is_some(&m_floats) {
                                    #[cfg(feature = "ignore_duplicates")]
                                    {
                                        __A::skip_value(&mut __map)?;
                                        continue;
                                    }
                                    #[cfg(feature = "strict")]
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field("floats"),
                                    );
                                }
                                m_floats = _serde::__private::Some(
                                    match __A::next_value::<Vec<f32>>(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            __Field::m_numFrames => {
                                #[cfg(
                                    any(feature = "strict", feature = "ignore_duplicates")
                                )]
                                if _serde::__private::Option::is_some(&m_numFrames) {
                                    #[cfg(feature = "ignore_duplicates")]
                                    {
                                        __A::skip_value(&mut __map)?;
                                        continue;
                                    }
                                    #[cfg(feature = "strict")]
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "numFrames",
                                        ),
                                    );
                                }
                                m_numFrames = _serde::__private::Some(
                                    match __A::next_value::<I32<'de>>(&mut __map) {
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
                    let m_type = match m_type {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            #[cfg(feature = "strict")]
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field("type"),
                            );
                            #[cfg(not(feature = "strict"))] Default::default()
                        }
                    };
                    let m_duration = match m_duration {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            #[cfg(feature = "strict")]
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field("duration"),
                            );
                            #[cfg(not(feature = "strict"))] Default::default()
                        }
                    };
                    let m_numberOfTransformTracks = match m_numberOfTransformTracks {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            #[cfg(feature = "strict")]
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "numberOfTransformTracks",
                                ),
                            );
                            #[cfg(not(feature = "strict"))] Default::default()
                        }
                    };
                    let m_numberOfFloatTracks = match m_numberOfFloatTracks {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            #[cfg(feature = "strict")]
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "numberOfFloatTracks",
                                ),
                            );
                            #[cfg(not(feature = "strict"))] Default::default()
                        }
                    };
                    let m_extractedMotion = match m_extractedMotion {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            #[cfg(feature = "strict")]
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "extractedMotion",
                                ),
                            );
                            #[cfg(not(feature = "strict"))] Default::default()
                        }
                    };
                    let m_annotationTracks = match m_annotationTracks {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            #[cfg(feature = "strict")]
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "annotationTracks",
                                ),
                            );
                            #[cfg(not(feature = "strict"))] Default::default()
                        }
                    };
                    let m_dynamicTranslations = match m_dynamicTranslations {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            #[cfg(feature = "strict")]
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "dynamicTranslations",
                                ),
                            );
                            #[cfg(not(feature = "strict"))] Default::default()
                        }
                    };
                    let m_staticTranslations = match m_staticTranslations {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            #[cfg(feature = "strict")]
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "staticTranslations",
                                ),
                            );
                            #[cfg(not(feature = "strict"))] Default::default()
                        }
                    };
                    let m_translationTypeAndOffsets = match m_translationTypeAndOffsets {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            #[cfg(feature = "strict")]
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "translationTypeAndOffsets",
                                ),
                            );
                            #[cfg(not(feature = "strict"))] Default::default()
                        }
                    };
                    let m_dynamicRotations = match m_dynamicRotations {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            #[cfg(feature = "strict")]
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "dynamicRotations",
                                ),
                            );
                            #[cfg(not(feature = "strict"))] Default::default()
                        }
                    };
                    let m_staticRotations = match m_staticRotations {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            #[cfg(feature = "strict")]
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "staticRotations",
                                ),
                            );
                            #[cfg(not(feature = "strict"))] Default::default()
                        }
                    };
                    let m_rotationTypeAndOffsets = match m_rotationTypeAndOffsets {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            #[cfg(feature = "strict")]
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "rotationTypeAndOffsets",
                                ),
                            );
                            #[cfg(not(feature = "strict"))] Default::default()
                        }
                    };
                    let m_dynamicScales = match m_dynamicScales {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            #[cfg(feature = "strict")]
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "dynamicScales",
                                ),
                            );
                            #[cfg(not(feature = "strict"))] Default::default()
                        }
                    };
                    let m_staticScales = match m_staticScales {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            #[cfg(feature = "strict")]
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "staticScales",
                                ),
                            );
                            #[cfg(not(feature = "strict"))] Default::default()
                        }
                    };
                    let m_scaleTypeAndOffsets = match m_scaleTypeAndOffsets {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            #[cfg(feature = "strict")]
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "scaleTypeAndOffsets",
                                ),
                            );
                            #[cfg(not(feature = "strict"))] Default::default()
                        }
                    };
                    let m_floats = match m_floats {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            #[cfg(feature = "strict")]
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field("floats"),
                            );
                            #[cfg(not(feature = "strict"))] Default::default()
                        }
                    };
                    let m_numFrames = match m_numFrames {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            #[cfg(feature = "strict")]
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "numFrames",
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
                    let parent = hkaAnimation {
                        __ptr: __ptr.clone(),
                        parent,
                        m_type,
                        m_duration,
                        m_numberOfTransformTracks,
                        m_numberOfFloatTracks,
                        m_extractedMotion,
                        m_annotationTracks,
                    };
                    let __ptr = __A::class_ptr(&mut __map);
                    _serde::__private::Ok(hkaLosslessCompressedAnimation {
                        __ptr: __ptr.clone(),
                        parent,
                        m_dynamicTranslations,
                        m_staticTranslations,
                        m_translationTypeAndOffsets,
                        m_dynamicRotations,
                        m_staticRotations,
                        m_rotationTypeAndOffsets,
                        m_dynamicScales,
                        m_staticScales,
                        m_scaleTypeAndOffsets,
                        m_floats,
                        m_numFrames,
                    })
                }
            }
            const FIELDS: &[&str] = &[
                "dynamicTranslations",
                "staticTranslations",
                "translationTypeAndOffsets",
                "dynamicRotations",
                "staticRotations",
                "rotationTypeAndOffsets",
                "dynamicScales",
                "staticScales",
                "scaleTypeAndOffsets",
                "floats",
                "numFrames",
            ];
            _serde::Deserializer::deserialize_struct(
                deserializer,
                "hkaLosslessCompressedAnimation",
                FIELDS,
                __hkaLosslessCompressedAnimationVisitor {
                    marker: _serde::__private::PhantomData::<
                        hkaLosslessCompressedAnimation,
                    >,
                    lifetime: _serde::__private::PhantomData,
                },
            )
        }
    }
};
