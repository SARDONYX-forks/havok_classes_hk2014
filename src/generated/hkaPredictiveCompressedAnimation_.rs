use super::class_requires::*;
use super::*;
/// # C++ Info
/// - name: `hkaPredictiveCompressedAnimation`
/// - version: `1`
/// - signature: `0xca3194b2`
/// - size: `  0`(x86)/`192`(x86_64)
/// -  vtable: `true`
#[allow(non_upper_case_globals, non_snake_case)]
#[cfg_attr(feature = "json_schema", derive(schemars::JsonSchema))]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(educe::Educe)]
#[educe(Debug, Clone, Default, PartialEq)]
pub struct hkaPredictiveCompressedAnimation<'a> {
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
    /// - name: `compressedData`(ctype: `hkArray<hkUint8>`)
    /// - offset: `  0`(x86)/` 56`(x86_64)
    /// - type_size: `  0`(x86)/` 16`(x86_64)
    #[cfg_attr(feature = "serde_default", serde(default))]
    #[cfg_attr(feature = "serde", serde(borrow))]
    #[cfg_attr(feature = "json_schema", schemars(rename = "compressedData"))]
    #[cfg_attr(feature = "serde", serde(rename = "compressedData"))]
    pub m_compressedData: Vec<U8<'a>>,
    /// # C++ Info
    /// - name: `intData`(ctype: `hkArray<hkUint16>`)
    /// - offset: `  0`(x86)/` 72`(x86_64)
    /// - type_size: `  0`(x86)/` 16`(x86_64)
    #[cfg_attr(feature = "serde_default", serde(default))]
    #[cfg_attr(feature = "serde", serde(borrow))]
    #[cfg_attr(feature = "json_schema", schemars(rename = "intData"))]
    #[cfg_attr(feature = "serde", serde(rename = "intData"))]
    pub m_intData: Vec<U16<'a>>,
    /// # C++ Info
    /// - name: `intArrayOffsets`(ctype: `hkInt32[9]`)
    /// - offset: `  0`(x86)/` 88`(x86_64)
    /// - type_size: `  0`(x86)/` 36`(x86_64)
    #[cfg_attr(feature = "serde_default", serde(default))]
    #[cfg_attr(feature = "serde", serde(borrow))]
    #[cfg_attr(feature = "json_schema", schemars(rename = "intArrayOffsets"))]
    #[cfg_attr(feature = "serde", serde(rename = "intArrayOffsets"))]
    pub m_intArrayOffsets: [I32<'a>; 9usize],
    /// # C++ Info
    /// - name: `floatData`(ctype: `hkArray<hkReal>`)
    /// - offset: `  0`(x86)/`128`(x86_64)
    /// - type_size: `  0`(x86)/` 16`(x86_64)
    #[cfg_attr(feature = "serde_default", serde(default))]
    #[cfg_attr(feature = "json_schema", schemars(rename = "floatData"))]
    #[cfg_attr(feature = "serde", serde(rename = "floatData"))]
    pub m_floatData: Vec<f32>,
    /// # C++ Info
    /// - name: `floatArrayOffsets`(ctype: `hkInt32[3]`)
    /// - offset: `  0`(x86)/`144`(x86_64)
    /// - type_size: `  0`(x86)/` 12`(x86_64)
    #[cfg_attr(feature = "serde_default", serde(default))]
    #[cfg_attr(feature = "serde", serde(borrow))]
    #[cfg_attr(feature = "json_schema", schemars(rename = "floatArrayOffsets"))]
    #[cfg_attr(feature = "serde", serde(rename = "floatArrayOffsets"))]
    pub m_floatArrayOffsets: [I32<'a>; 3usize],
    /// # C++ Info
    /// - name: `numBones`(ctype: `hkInt32`)
    /// - offset: `  0`(x86)/`156`(x86_64)
    /// - type_size: `  0`(x86)/`  4`(x86_64)
    #[cfg_attr(feature = "serde_default", serde(default))]
    #[cfg_attr(feature = "serde", serde(borrow))]
    #[cfg_attr(feature = "json_schema", schemars(rename = "numBones"))]
    #[cfg_attr(feature = "serde", serde(rename = "numBones"))]
    pub m_numBones: I32<'a>,
    /// # C++ Info
    /// - name: `numFloatSlots`(ctype: `hkInt32`)
    /// - offset: `  0`(x86)/`160`(x86_64)
    /// - type_size: `  0`(x86)/`  4`(x86_64)
    #[cfg_attr(feature = "serde_default", serde(default))]
    #[cfg_attr(feature = "serde", serde(borrow))]
    #[cfg_attr(feature = "json_schema", schemars(rename = "numFloatSlots"))]
    #[cfg_attr(feature = "serde", serde(rename = "numFloatSlots"))]
    pub m_numFloatSlots: I32<'a>,
    /// # C++ Info
    /// - name: `numFrames`(ctype: `hkInt32`)
    /// - offset: `  0`(x86)/`164`(x86_64)
    /// - type_size: `  0`(x86)/`  4`(x86_64)
    #[cfg_attr(feature = "serde_default", serde(default))]
    #[cfg_attr(feature = "serde", serde(borrow))]
    #[cfg_attr(feature = "json_schema", schemars(rename = "numFrames"))]
    #[cfg_attr(feature = "serde", serde(rename = "numFrames"))]
    pub m_numFrames: I32<'a>,
    /// # C++ Info
    /// - name: `firstFloatBlockScaleAndOffsetIndex`(ctype: `hkInt32`)
    /// - offset: `  0`(x86)/`168`(x86_64)
    /// - type_size: `  0`(x86)/`  4`(x86_64)
    #[cfg_attr(feature = "serde_default", serde(default))]
    #[cfg_attr(feature = "serde", serde(borrow))]
    #[cfg_attr(
        feature = "json_schema",
        schemars(rename = "firstFloatBlockScaleAndOffsetIndex")
    )]
    #[cfg_attr(feature = "serde", serde(rename = "firstFloatBlockScaleAndOffsetIndex"))]
    pub m_firstFloatBlockScaleAndOffsetIndex: I32<'a>,
    /// # C++ Info
    /// - name: `skeleton`(ctype: `void*`)
    /// - offset: `  0`(x86)/`176`(x86_64)
    /// - type_size: `  0`(x86)/`  8`(x86_64)
    /// - flags: `SERIALIZE_IGNORED`
    #[cfg_attr(feature = "serde", serde(default))]
    #[cfg_attr(feature = "serde", serde(borrow))]
    #[cfg_attr(feature = "json_schema", schemars(rename = "skeleton"))]
    #[cfg_attr(feature = "serde", serde(rename = "skeleton"))]
    pub m_skeleton: Pointer<'a>,
    /// # C++ Info
    /// - name: `maxCompressedBytesPerFrame`(ctype: `hkInt32`)
    /// - offset: `  0`(x86)/`184`(x86_64)
    /// - type_size: `  0`(x86)/`  4`(x86_64)
    /// - flags: `SERIALIZE_IGNORED`
    #[cfg_attr(feature = "serde", serde(default))]
    #[cfg_attr(feature = "serde", serde(borrow))]
    #[cfg_attr(feature = "json_schema", schemars(rename = "maxCompressedBytesPerFrame"))]
    #[cfg_attr(feature = "serde", serde(rename = "maxCompressedBytesPerFrame"))]
    pub m_maxCompressedBytesPerFrame: I32<'a>,
}
const _: () = {
    use havok_serde as _serde;
    impl<'a> _serde::HavokClass for hkaPredictiveCompressedAnimation<'a> {
        #[inline]
        fn name(&self) -> &'static str {
            "hkaPredictiveCompressedAnimation"
        }
        #[inline]
        fn signature(&self) -> _serde::__private::Signature {
            _serde::__private::Signature::new(0xca3194b2)
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
            v.push(&self.m_skeleton);
            v
        }
    }
    impl<'a> _serde::Serialize for hkaPredictiveCompressedAnimation<'a> {
        fn serialize<S>(&self, __serializer: S) -> Result<S::Ok, S::Error>
        where
            S: _serde::ser::Serializer,
        {
            let class_meta = self
                .__ptr
                .as_ref()
                .map(|name| (name, _serde::__private::Signature::new(0xca3194b2)));
            let mut serializer = __serializer
                .serialize_struct(
                    "hkaPredictiveCompressedAnimation",
                    class_meta,
                    (0u64, 192u64),
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
                    "compressedData",
                    &self.m_compressedData,
                    TypeSize::NonPtr,
                )?;
            serializer
                .serialize_array_field("intData", &self.m_intData, TypeSize::NonPtr)?;
            serializer
                .serialize_fixed_array_field(
                    "intArrayOffsets",
                    self.m_intArrayOffsets.as_slice(),
                    TypeSize::NonPtr,
                )?;
            serializer.pad_field([0u8; 0usize].as_slice(), [0u8; 4usize].as_slice())?;
            serializer
                .serialize_array_field(
                    "floatData",
                    &self.m_floatData,
                    TypeSize::NonPtr,
                )?;
            serializer
                .serialize_fixed_array_field(
                    "floatArrayOffsets",
                    self.m_floatArrayOffsets.as_slice(),
                    TypeSize::NonPtr,
                )?;
            serializer.serialize_field("numBones", &self.m_numBones)?;
            serializer.serialize_field("numFloatSlots", &self.m_numFloatSlots)?;
            serializer.serialize_field("numFrames", &self.m_numFrames)?;
            serializer
                .serialize_field(
                    "firstFloatBlockScaleAndOffsetIndex",
                    &self.m_firstFloatBlockScaleAndOffsetIndex,
                )?;
            serializer.pad_field([0u8; 0usize].as_slice(), [0u8; 4usize].as_slice())?;
            serializer.skip_field("skeleton", &self.m_skeleton)?;
            serializer
                .skip_field(
                    "maxCompressedBytesPerFrame",
                    &self.m_maxCompressedBytesPerFrame,
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
    impl<'de> _serde::Deserialize<'de> for hkaPredictiveCompressedAnimation<'de> {
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
                m_compressedData,
                m_intData,
                m_intArrayOffsets,
                m_floatData,
                m_floatArrayOffsets,
                m_numBones,
                m_numFloatSlots,
                m_numFrames,
                m_firstFloatBlockScaleAndOffsetIndex,
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
                        "compressedData" => Ok(__Field::m_compressedData),
                        "intData" => Ok(__Field::m_intData),
                        "intArrayOffsets" => Ok(__Field::m_intArrayOffsets),
                        "floatData" => Ok(__Field::m_floatData),
                        "floatArrayOffsets" => Ok(__Field::m_floatArrayOffsets),
                        "numBones" => Ok(__Field::m_numBones),
                        "numFloatSlots" => Ok(__Field::m_numFloatSlots),
                        "numFrames" => Ok(__Field::m_numFrames),
                        "firstFloatBlockScaleAndOffsetIndex" => {
                            Ok(__Field::m_firstFloatBlockScaleAndOffsetIndex)
                        }
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
            struct __hkaPredictiveCompressedAnimationVisitor<'de> {
                marker: _serde::__private::PhantomData<
                    hkaPredictiveCompressedAnimation<'de>,
                >,
                lifetime: _serde::__private::PhantomData<&'de ()>,
            }
            #[allow(clippy::match_single_binding)]
            #[allow(clippy::reversed_empty_ranges)]
            #[allow(clippy::single_match)]
            impl<'de> _serde::de::Visitor<'de>
            for __hkaPredictiveCompressedAnimationVisitor<'de> {
                type Value = hkaPredictiveCompressedAnimation<'de>;
                fn expecting(
                    &self,
                    __formatter: &mut core::fmt::Formatter,
                ) -> core::fmt::Result {
                    core::fmt::Formatter::write_str(
                        __formatter,
                        "struct hkaPredictiveCompressedAnimation",
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
                    let mut m_compressedData: _serde::__private::Option<Vec<U8<'de>>> = _serde::__private::None;
                    let mut m_intData: _serde::__private::Option<Vec<U16<'de>>> = _serde::__private::None;
                    let mut m_intArrayOffsets: _serde::__private::Option<
                        [I32<'de>; 9usize],
                    > = _serde::__private::None;
                    let mut m_floatData: _serde::__private::Option<Vec<f32>> = _serde::__private::None;
                    let mut m_floatArrayOffsets: _serde::__private::Option<
                        [I32<'de>; 3usize],
                    > = _serde::__private::None;
                    let mut m_numBones: _serde::__private::Option<I32<'de>> = _serde::__private::None;
                    let mut m_numFloatSlots: _serde::__private::Option<I32<'de>> = _serde::__private::None;
                    let mut m_numFrames: _serde::__private::Option<I32<'de>> = _serde::__private::None;
                    let mut m_firstFloatBlockScaleAndOffsetIndex: _serde::__private::Option<
                        I32<'de>,
                    > = _serde::__private::None;
                    let mut m_skeleton: _serde::__private::Option<Pointer<'de>> = _serde::__private::None;
                    let mut m_maxCompressedBytesPerFrame: _serde::__private::Option<
                        I32<'de>,
                    > = _serde::__private::None;
                    for i in 0..11usize {
                        match i {
                            0usize => {
                                if _serde::__private::Option::is_some(&m_compressedData) {
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "compressedData",
                                        ),
                                    );
                                }
                                m_compressedData = _serde::__private::Some(
                                    match __A::next_value::<Vec<U8<'de>>>(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            1usize => {
                                if _serde::__private::Option::is_some(&m_intData) {
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "intData",
                                        ),
                                    );
                                }
                                m_intData = _serde::__private::Some(
                                    match __A::next_value::<Vec<U16<'de>>>(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            2usize => {
                                if _serde::__private::Option::is_some(&m_intArrayOffsets) {
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "intArrayOffsets",
                                        ),
                                    );
                                }
                                m_intArrayOffsets = _serde::__private::Some(
                                    match __A::next_value::<[I32<'de>; 9usize]>(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            3usize => {
                                if _serde::__private::Option::is_some(&m_floatData) {
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "floatData",
                                        ),
                                    );
                                }
                                __A::pad(&mut __map, 0usize, 4usize)?;
                                m_floatData = _serde::__private::Some(
                                    match __A::next_value::<Vec<f32>>(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            4usize => {
                                if _serde::__private::Option::is_some(
                                    &m_floatArrayOffsets,
                                ) {
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "floatArrayOffsets",
                                        ),
                                    );
                                }
                                m_floatArrayOffsets = _serde::__private::Some(
                                    match __A::next_value::<[I32<'de>; 3usize]>(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            5usize => {
                                if _serde::__private::Option::is_some(&m_numBones) {
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "numBones",
                                        ),
                                    );
                                }
                                m_numBones = _serde::__private::Some(
                                    match __A::next_value::<I32<'de>>(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            6usize => {
                                if _serde::__private::Option::is_some(&m_numFloatSlots) {
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "numFloatSlots",
                                        ),
                                    );
                                }
                                m_numFloatSlots = _serde::__private::Some(
                                    match __A::next_value::<I32<'de>>(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            7usize => {
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
                            8usize => {
                                if _serde::__private::Option::is_some(
                                    &m_firstFloatBlockScaleAndOffsetIndex,
                                ) {
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "firstFloatBlockScaleAndOffsetIndex",
                                        ),
                                    );
                                }
                                m_firstFloatBlockScaleAndOffsetIndex = _serde::__private::Some(
                                    match __A::next_value::<I32<'de>>(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            9usize => {
                                if _serde::__private::Option::is_some(&m_skeleton) {
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "skeleton",
                                        ),
                                    );
                                }
                                __A::pad(&mut __map, 0usize, 4usize)?;
                                m_skeleton = _serde::__private::Some(
                                    match __A::next_value::<Pointer<'de>>(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            10usize => {
                                if _serde::__private::Option::is_some(
                                    &m_maxCompressedBytesPerFrame,
                                ) {
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "maxCompressedBytesPerFrame",
                                        ),
                                    );
                                }
                                m_maxCompressedBytesPerFrame = _serde::__private::Some(
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
                    let m_compressedData = match m_compressedData {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "compressedData",
                                ),
                            );
                        }
                    };
                    let m_intData = match m_intData {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field("intData"),
                            );
                        }
                    };
                    let m_intArrayOffsets = match m_intArrayOffsets {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "intArrayOffsets",
                                ),
                            );
                        }
                    };
                    let m_floatData = match m_floatData {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "floatData",
                                ),
                            );
                        }
                    };
                    let m_floatArrayOffsets = match m_floatArrayOffsets {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "floatArrayOffsets",
                                ),
                            );
                        }
                    };
                    let m_numBones = match m_numBones {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field("numBones"),
                            );
                        }
                    };
                    let m_numFloatSlots = match m_numFloatSlots {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "numFloatSlots",
                                ),
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
                    let m_firstFloatBlockScaleAndOffsetIndex = match m_firstFloatBlockScaleAndOffsetIndex {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "firstFloatBlockScaleAndOffsetIndex",
                                ),
                            );
                        }
                    };
                    let m_skeleton = match m_skeleton {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field("skeleton"),
                            );
                        }
                    };
                    let m_maxCompressedBytesPerFrame = match m_maxCompressedBytesPerFrame {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "maxCompressedBytesPerFrame",
                                ),
                            );
                        }
                    };
                    _serde::__private::Ok(hkaPredictiveCompressedAnimation {
                        __ptr,
                        parent,
                        m_compressedData,
                        m_intData,
                        m_intArrayOffsets,
                        m_floatData,
                        m_floatArrayOffsets,
                        m_numBones,
                        m_numFloatSlots,
                        m_numFrames,
                        m_firstFloatBlockScaleAndOffsetIndex,
                        m_skeleton,
                        m_maxCompressedBytesPerFrame,
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
                    let mut m_compressedData: _serde::__private::Option<Vec<U8<'de>>> = _serde::__private::None;
                    let mut m_intData: _serde::__private::Option<Vec<U16<'de>>> = _serde::__private::None;
                    let mut m_intArrayOffsets: _serde::__private::Option<
                        [I32<'de>; 9usize],
                    > = _serde::__private::None;
                    let mut m_floatData: _serde::__private::Option<Vec<f32>> = _serde::__private::None;
                    let mut m_floatArrayOffsets: _serde::__private::Option<
                        [I32<'de>; 3usize],
                    > = _serde::__private::None;
                    let mut m_numBones: _serde::__private::Option<I32<'de>> = _serde::__private::None;
                    let mut m_numFloatSlots: _serde::__private::Option<I32<'de>> = _serde::__private::None;
                    let mut m_numFrames: _serde::__private::Option<I32<'de>> = _serde::__private::None;
                    let mut m_firstFloatBlockScaleAndOffsetIndex: _serde::__private::Option<
                        I32<'de>,
                    > = _serde::__private::None;
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
                            __Field::m_compressedData => {
                                #[cfg(
                                    any(feature = "strict", feature = "ignore_duplicates")
                                )]
                                if _serde::__private::Option::is_some(&m_compressedData) {
                                    #[cfg(feature = "ignore_duplicates")]
                                    {
                                        __A::skip_value(&mut __map)?;
                                        continue;
                                    }
                                    #[cfg(feature = "strict")]
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "compressedData",
                                        ),
                                    );
                                }
                                m_compressedData = _serde::__private::Some(
                                    match __A::next_value::<Vec<U8<'de>>>(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            __Field::m_intData => {
                                #[cfg(
                                    any(feature = "strict", feature = "ignore_duplicates")
                                )]
                                if _serde::__private::Option::is_some(&m_intData) {
                                    #[cfg(feature = "ignore_duplicates")]
                                    {
                                        __A::skip_value(&mut __map)?;
                                        continue;
                                    }
                                    #[cfg(feature = "strict")]
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "intData",
                                        ),
                                    );
                                }
                                m_intData = _serde::__private::Some(
                                    match __A::next_value::<Vec<U16<'de>>>(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            __Field::m_intArrayOffsets => {
                                #[cfg(
                                    any(feature = "strict", feature = "ignore_duplicates")
                                )]
                                if _serde::__private::Option::is_some(&m_intArrayOffsets) {
                                    #[cfg(feature = "ignore_duplicates")]
                                    {
                                        __A::skip_value(&mut __map)?;
                                        continue;
                                    }
                                    #[cfg(feature = "strict")]
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "intArrayOffsets",
                                        ),
                                    );
                                }
                                m_intArrayOffsets = _serde::__private::Some(
                                    match __A::next_value::<[I32<'de>; 9usize]>(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            __Field::m_floatData => {
                                #[cfg(
                                    any(feature = "strict", feature = "ignore_duplicates")
                                )]
                                if _serde::__private::Option::is_some(&m_floatData) {
                                    #[cfg(feature = "ignore_duplicates")]
                                    {
                                        __A::skip_value(&mut __map)?;
                                        continue;
                                    }
                                    #[cfg(feature = "strict")]
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "floatData",
                                        ),
                                    );
                                }
                                m_floatData = _serde::__private::Some(
                                    match __A::next_value::<Vec<f32>>(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            __Field::m_floatArrayOffsets => {
                                #[cfg(
                                    any(feature = "strict", feature = "ignore_duplicates")
                                )]
                                if _serde::__private::Option::is_some(
                                    &m_floatArrayOffsets,
                                ) {
                                    #[cfg(feature = "ignore_duplicates")]
                                    {
                                        __A::skip_value(&mut __map)?;
                                        continue;
                                    }
                                    #[cfg(feature = "strict")]
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "floatArrayOffsets",
                                        ),
                                    );
                                }
                                m_floatArrayOffsets = _serde::__private::Some(
                                    match __A::next_value::<[I32<'de>; 3usize]>(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            __Field::m_numBones => {
                                #[cfg(
                                    any(feature = "strict", feature = "ignore_duplicates")
                                )]
                                if _serde::__private::Option::is_some(&m_numBones) {
                                    #[cfg(feature = "ignore_duplicates")]
                                    {
                                        __A::skip_value(&mut __map)?;
                                        continue;
                                    }
                                    #[cfg(feature = "strict")]
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "numBones",
                                        ),
                                    );
                                }
                                m_numBones = _serde::__private::Some(
                                    match __A::next_value::<I32<'de>>(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            __Field::m_numFloatSlots => {
                                #[cfg(
                                    any(feature = "strict", feature = "ignore_duplicates")
                                )]
                                if _serde::__private::Option::is_some(&m_numFloatSlots) {
                                    #[cfg(feature = "ignore_duplicates")]
                                    {
                                        __A::skip_value(&mut __map)?;
                                        continue;
                                    }
                                    #[cfg(feature = "strict")]
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "numFloatSlots",
                                        ),
                                    );
                                }
                                m_numFloatSlots = _serde::__private::Some(
                                    match __A::next_value::<I32<'de>>(&mut __map) {
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
                            __Field::m_firstFloatBlockScaleAndOffsetIndex => {
                                #[cfg(
                                    any(feature = "strict", feature = "ignore_duplicates")
                                )]
                                if _serde::__private::Option::is_some(
                                    &m_firstFloatBlockScaleAndOffsetIndex,
                                ) {
                                    #[cfg(feature = "ignore_duplicates")]
                                    {
                                        __A::skip_value(&mut __map)?;
                                        continue;
                                    }
                                    #[cfg(feature = "strict")]
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "firstFloatBlockScaleAndOffsetIndex",
                                        ),
                                    );
                                }
                                m_firstFloatBlockScaleAndOffsetIndex = _serde::__private::Some(
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
                    let m_compressedData = match m_compressedData {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            #[cfg(feature = "strict")]
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "compressedData",
                                ),
                            );
                            #[cfg(not(feature = "strict"))] Default::default()
                        }
                    };
                    let m_intData = match m_intData {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            #[cfg(feature = "strict")]
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field("intData"),
                            );
                            #[cfg(not(feature = "strict"))] Default::default()
                        }
                    };
                    let m_intArrayOffsets = match m_intArrayOffsets {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            #[cfg(feature = "strict")]
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "intArrayOffsets",
                                ),
                            );
                            #[cfg(not(feature = "strict"))] Default::default()
                        }
                    };
                    let m_floatData = match m_floatData {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            #[cfg(feature = "strict")]
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "floatData",
                                ),
                            );
                            #[cfg(not(feature = "strict"))] Default::default()
                        }
                    };
                    let m_floatArrayOffsets = match m_floatArrayOffsets {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            #[cfg(feature = "strict")]
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "floatArrayOffsets",
                                ),
                            );
                            #[cfg(not(feature = "strict"))] Default::default()
                        }
                    };
                    let m_numBones = match m_numBones {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            #[cfg(feature = "strict")]
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field("numBones"),
                            );
                            #[cfg(not(feature = "strict"))] Default::default()
                        }
                    };
                    let m_numFloatSlots = match m_numFloatSlots {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            #[cfg(feature = "strict")]
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "numFloatSlots",
                                ),
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
                    let m_firstFloatBlockScaleAndOffsetIndex = match m_firstFloatBlockScaleAndOffsetIndex {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            #[cfg(feature = "strict")]
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "firstFloatBlockScaleAndOffsetIndex",
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
                    _serde::__private::Ok(hkaPredictiveCompressedAnimation {
                        __ptr: __ptr.clone(),
                        parent,
                        m_compressedData,
                        m_intData,
                        m_intArrayOffsets,
                        m_floatData,
                        m_floatArrayOffsets,
                        m_numBones,
                        m_numFloatSlots,
                        m_numFrames,
                        m_firstFloatBlockScaleAndOffsetIndex,
                        ..Default::default()
                    })
                }
            }
            const FIELDS: &[&str] = &[
                "compressedData",
                "intData",
                "intArrayOffsets",
                "floatData",
                "floatArrayOffsets",
                "numBones",
                "numFloatSlots",
                "numFrames",
                "firstFloatBlockScaleAndOffsetIndex",
                "skeleton",
                "maxCompressedBytesPerFrame",
            ];
            _serde::Deserializer::deserialize_struct(
                deserializer,
                "hkaPredictiveCompressedAnimation",
                FIELDS,
                __hkaPredictiveCompressedAnimationVisitor {
                    marker: _serde::__private::PhantomData::<
                        hkaPredictiveCompressedAnimation,
                    >,
                    lifetime: _serde::__private::PhantomData,
                },
            )
        }
    }
};
