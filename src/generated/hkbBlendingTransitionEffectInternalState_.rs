use super::class_requires::*;
use super::*;
/// # C++ Info
/// - name: `hkbBlendingTransitionEffectInternalState`
/// - version: `2`
/// - signature: `0x7b3a67c3`
/// - size: `  0`(x86)/`144`(x86_64)
/// -  vtable: `true`
#[allow(non_upper_case_globals, non_snake_case)]
#[cfg_attr(feature = "json_schema", derive(schemars::JsonSchema))]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(educe::Educe)]
#[educe(Debug, Clone, Default, PartialEq)]
pub struct hkbBlendingTransitionEffectInternalState<'a> {
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
    pub parent: hkReferencedObject<'a>,
    /// # C++ Info
    /// - name: `fromPos`(ctype: `hkVector4`)
    /// - offset: `  0`(x86)/` 16`(x86_64)
    /// - type_size: `  0`(x86)/` 16`(x86_64)
    #[cfg_attr(feature = "serde_default", serde(default))]
    #[cfg_attr(feature = "json_schema", schemars(rename = "fromPos"))]
    #[cfg_attr(feature = "serde", serde(rename = "fromPos"))]
    pub m_fromPos: Vector4,
    /// # C++ Info
    /// - name: `fromRot`(ctype: `hkQuaternion`)
    /// - offset: `  0`(x86)/` 32`(x86_64)
    /// - type_size: `  0`(x86)/` 16`(x86_64)
    #[cfg_attr(feature = "serde_default", serde(default))]
    #[cfg_attr(feature = "json_schema", schemars(rename = "fromRot"))]
    #[cfg_attr(feature = "serde", serde(rename = "fromRot"))]
    pub m_fromRot: Quaternion,
    /// # C++ Info
    /// - name: `toPos`(ctype: `hkVector4`)
    /// - offset: `  0`(x86)/` 48`(x86_64)
    /// - type_size: `  0`(x86)/` 16`(x86_64)
    #[cfg_attr(feature = "serde_default", serde(default))]
    #[cfg_attr(feature = "json_schema", schemars(rename = "toPos"))]
    #[cfg_attr(feature = "serde", serde(rename = "toPos"))]
    pub m_toPos: Vector4,
    /// # C++ Info
    /// - name: `toRot`(ctype: `hkQuaternion`)
    /// - offset: `  0`(x86)/` 64`(x86_64)
    /// - type_size: `  0`(x86)/` 16`(x86_64)
    #[cfg_attr(feature = "serde_default", serde(default))]
    #[cfg_attr(feature = "json_schema", schemars(rename = "toRot"))]
    #[cfg_attr(feature = "serde", serde(rename = "toRot"))]
    pub m_toRot: Quaternion,
    /// # C++ Info
    /// - name: `lastPos`(ctype: `hkVector4`)
    /// - offset: `  0`(x86)/` 80`(x86_64)
    /// - type_size: `  0`(x86)/` 16`(x86_64)
    #[cfg_attr(feature = "serde_default", serde(default))]
    #[cfg_attr(feature = "json_schema", schemars(rename = "lastPos"))]
    #[cfg_attr(feature = "serde", serde(rename = "lastPos"))]
    pub m_lastPos: Vector4,
    /// # C++ Info
    /// - name: `lastRot`(ctype: `hkQuaternion`)
    /// - offset: `  0`(x86)/` 96`(x86_64)
    /// - type_size: `  0`(x86)/` 16`(x86_64)
    #[cfg_attr(feature = "serde_default", serde(default))]
    #[cfg_attr(feature = "json_schema", schemars(rename = "lastRot"))]
    #[cfg_attr(feature = "serde", serde(rename = "lastRot"))]
    pub m_lastRot: Quaternion,
    /// # C++ Info
    /// - name: `characterPoseAtBeginningOfTransition`(ctype: `hkArray<hkQsTransform>`)
    /// - offset: `  0`(x86)/`112`(x86_64)
    /// - type_size: `  0`(x86)/` 16`(x86_64)
    #[cfg_attr(feature = "serde_default", serde(default))]
    #[cfg_attr(
        feature = "json_schema",
        schemars(rename = "characterPoseAtBeginningOfTransition")
    )]
    #[cfg_attr(
        feature = "serde",
        serde(rename = "characterPoseAtBeginningOfTransition")
    )]
    pub m_characterPoseAtBeginningOfTransition: Vec<QsTransform>,
    /// # C++ Info
    /// - name: `timeRemaining`(ctype: `hkReal`)
    /// - offset: `  0`(x86)/`128`(x86_64)
    /// - type_size: `  0`(x86)/`  4`(x86_64)
    #[cfg_attr(feature = "serde_default", serde(default))]
    #[cfg_attr(feature = "json_schema", schemars(rename = "timeRemaining"))]
    #[cfg_attr(feature = "serde", serde(rename = "timeRemaining"))]
    pub m_timeRemaining: f32,
    /// # C++ Info
    /// - name: `timeInTransition`(ctype: `hkReal`)
    /// - offset: `  0`(x86)/`132`(x86_64)
    /// - type_size: `  0`(x86)/`  4`(x86_64)
    #[cfg_attr(feature = "serde_default", serde(default))]
    #[cfg_attr(feature = "json_schema", schemars(rename = "timeInTransition"))]
    #[cfg_attr(feature = "serde", serde(rename = "timeInTransition"))]
    pub m_timeInTransition: f32,
    /// # C++ Info
    /// - name: `toGeneratorSelfTranstitionMode`(ctype: `enum SelfTransitionMode`)
    /// - offset: `  0`(x86)/`136`(x86_64)
    /// - type_size: `  0`(x86)/`  1`(x86_64)
    #[cfg_attr(feature = "serde_default", serde(default))]
    #[cfg_attr(
        feature = "json_schema",
        schemars(rename = "toGeneratorSelfTranstitionMode")
    )]
    #[cfg_attr(feature = "serde", serde(rename = "toGeneratorSelfTranstitionMode"))]
    pub m_toGeneratorSelfTranstitionMode: SelfTransitionMode,
    /// # C++ Info
    /// - name: `initializeCharacterPose`(ctype: `hkBool`)
    /// - offset: `  0`(x86)/`137`(x86_64)
    /// - type_size: `  0`(x86)/`  1`(x86_64)
    #[cfg_attr(feature = "serde_default", serde(default))]
    #[cfg_attr(feature = "json_schema", schemars(rename = "initializeCharacterPose"))]
    #[cfg_attr(feature = "serde", serde(rename = "initializeCharacterPose"))]
    pub m_initializeCharacterPose: bool,
    /// # C++ Info
    /// - name: `alignThisFrame`(ctype: `hkBool`)
    /// - offset: `  0`(x86)/`138`(x86_64)
    /// - type_size: `  0`(x86)/`  1`(x86_64)
    #[cfg_attr(feature = "serde_default", serde(default))]
    #[cfg_attr(feature = "json_schema", schemars(rename = "alignThisFrame"))]
    #[cfg_attr(feature = "serde", serde(rename = "alignThisFrame"))]
    pub m_alignThisFrame: bool,
    /// # C++ Info
    /// - name: `alignmentFinished`(ctype: `hkBool`)
    /// - offset: `  0`(x86)/`139`(x86_64)
    /// - type_size: `  0`(x86)/`  1`(x86_64)
    #[cfg_attr(feature = "serde_default", serde(default))]
    #[cfg_attr(feature = "json_schema", schemars(rename = "alignmentFinished"))]
    #[cfg_attr(feature = "serde", serde(rename = "alignmentFinished"))]
    pub m_alignmentFinished: bool,
}
const _: () = {
    use havok_serde as _serde;
    impl<'a> _serde::HavokClass for hkbBlendingTransitionEffectInternalState<'a> {
        #[inline]
        fn name(&self) -> &'static str {
            "hkbBlendingTransitionEffectInternalState"
        }
        #[inline]
        fn signature(&self) -> _serde::__private::Signature {
            _serde::__private::Signature::new(0x7b3a67c3)
        }
        #[allow(clippy::let_and_return, clippy::vec_init_then_push)]
        fn deps_indexes(&self) -> Vec<&Pointer<'_>> {
            let mut v = Vec::new();
            v
        }
    }
    impl<'a> _serde::Serialize for hkbBlendingTransitionEffectInternalState<'a> {
        fn serialize<S>(&self, __serializer: S) -> Result<S::Ok, S::Error>
        where
            S: _serde::ser::Serializer,
        {
            let class_meta = self
                .__ptr
                .as_ref()
                .map(|name| (name, _serde::__private::Signature::new(0x7b3a67c3)));
            let mut serializer = __serializer
                .serialize_struct(
                    "hkbBlendingTransitionEffectInternalState",
                    class_meta,
                    (0u64, 144u64),
                )?;
            serializer.pad_field([0u8; 0usize].as_slice(), [0u8; 8usize].as_slice())?;
            serializer
                .skip_field("memSizeAndRefCount", &self.parent.m_memSizeAndRefCount)?;
            serializer.pad_field([0u8; 0usize].as_slice(), [0u8; 4usize].as_slice())?;
            serializer.serialize_field("fromPos", &self.m_fromPos)?;
            serializer.serialize_field("fromRot", &self.m_fromRot)?;
            serializer.serialize_field("toPos", &self.m_toPos)?;
            serializer.serialize_field("toRot", &self.m_toRot)?;
            serializer.serialize_field("lastPos", &self.m_lastPos)?;
            serializer.serialize_field("lastRot", &self.m_lastRot)?;
            serializer
                .serialize_array_field(
                    "characterPoseAtBeginningOfTransition",
                    &self.m_characterPoseAtBeginningOfTransition,
                    TypeSize::NonPtr,
                )?;
            serializer.serialize_field("timeRemaining", &self.m_timeRemaining)?;
            serializer.serialize_field("timeInTransition", &self.m_timeInTransition)?;
            serializer
                .serialize_field(
                    "toGeneratorSelfTranstitionMode",
                    &self.m_toGeneratorSelfTranstitionMode,
                )?;
            serializer
                .serialize_field(
                    "initializeCharacterPose",
                    &self.m_initializeCharacterPose,
                )?;
            serializer.serialize_field("alignThisFrame", &self.m_alignThisFrame)?;
            serializer.serialize_field("alignmentFinished", &self.m_alignmentFinished)?;
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
    impl<'de> _serde::Deserialize<'de>
    for hkbBlendingTransitionEffectInternalState<'de> {
        fn deserialize<__D>(deserializer: __D) -> core::result::Result<Self, __D::Error>
        where
            __D: _serde::Deserializer<'de>,
        {
            #[allow(non_camel_case_types)]
            enum __Field {
                m_fromPos,
                m_fromRot,
                m_toPos,
                m_toRot,
                m_lastPos,
                m_lastRot,
                m_characterPoseAtBeginningOfTransition,
                m_timeRemaining,
                m_timeInTransition,
                m_toGeneratorSelfTranstitionMode,
                m_initializeCharacterPose,
                m_alignThisFrame,
                m_alignmentFinished,
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
                        "fromPos" => Ok(__Field::m_fromPos),
                        "fromRot" => Ok(__Field::m_fromRot),
                        "toPos" => Ok(__Field::m_toPos),
                        "toRot" => Ok(__Field::m_toRot),
                        "lastPos" => Ok(__Field::m_lastPos),
                        "lastRot" => Ok(__Field::m_lastRot),
                        "characterPoseAtBeginningOfTransition" => {
                            Ok(__Field::m_characterPoseAtBeginningOfTransition)
                        }
                        "timeRemaining" => Ok(__Field::m_timeRemaining),
                        "timeInTransition" => Ok(__Field::m_timeInTransition),
                        "toGeneratorSelfTranstitionMode" => {
                            Ok(__Field::m_toGeneratorSelfTranstitionMode)
                        }
                        "initializeCharacterPose" => {
                            Ok(__Field::m_initializeCharacterPose)
                        }
                        "alignThisFrame" => Ok(__Field::m_alignThisFrame),
                        "alignmentFinished" => Ok(__Field::m_alignmentFinished),
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
            struct __hkbBlendingTransitionEffectInternalStateVisitor<'de> {
                marker: _serde::__private::PhantomData<
                    hkbBlendingTransitionEffectInternalState<'de>,
                >,
                lifetime: _serde::__private::PhantomData<&'de ()>,
            }
            #[allow(clippy::match_single_binding)]
            #[allow(clippy::reversed_empty_ranges)]
            #[allow(clippy::single_match)]
            impl<'de> _serde::de::Visitor<'de>
            for __hkbBlendingTransitionEffectInternalStateVisitor<'de> {
                type Value = hkbBlendingTransitionEffectInternalState<'de>;
                fn expecting(
                    &self,
                    __formatter: &mut core::fmt::Formatter,
                ) -> core::fmt::Result {
                    core::fmt::Formatter::write_str(
                        __formatter,
                        "struct hkbBlendingTransitionEffectInternalState",
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
                    let mut m_fromPos: _serde::__private::Option<Vector4> = _serde::__private::None;
                    let mut m_fromRot: _serde::__private::Option<Quaternion> = _serde::__private::None;
                    let mut m_toPos: _serde::__private::Option<Vector4> = _serde::__private::None;
                    let mut m_toRot: _serde::__private::Option<Quaternion> = _serde::__private::None;
                    let mut m_lastPos: _serde::__private::Option<Vector4> = _serde::__private::None;
                    let mut m_lastRot: _serde::__private::Option<Quaternion> = _serde::__private::None;
                    let mut m_characterPoseAtBeginningOfTransition: _serde::__private::Option<
                        Vec<QsTransform>,
                    > = _serde::__private::None;
                    let mut m_timeRemaining: _serde::__private::Option<f32> = _serde::__private::None;
                    let mut m_timeInTransition: _serde::__private::Option<f32> = _serde::__private::None;
                    let mut m_toGeneratorSelfTranstitionMode: _serde::__private::Option<
                        SelfTransitionMode,
                    > = _serde::__private::None;
                    let mut m_initializeCharacterPose: _serde::__private::Option<bool> = _serde::__private::None;
                    let mut m_alignThisFrame: _serde::__private::Option<bool> = _serde::__private::None;
                    let mut m_alignmentFinished: _serde::__private::Option<bool> = _serde::__private::None;
                    for i in 0..13usize {
                        match i {
                            0usize => {
                                if _serde::__private::Option::is_some(&m_fromPos) {
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "fromPos",
                                        ),
                                    );
                                }
                                m_fromPos = _serde::__private::Some(
                                    match __A::next_value::<Vector4>(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            1usize => {
                                if _serde::__private::Option::is_some(&m_fromRot) {
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "fromRot",
                                        ),
                                    );
                                }
                                m_fromRot = _serde::__private::Some(
                                    match __A::next_value::<Quaternion>(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            2usize => {
                                if _serde::__private::Option::is_some(&m_toPos) {
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field("toPos"),
                                    );
                                }
                                m_toPos = _serde::__private::Some(
                                    match __A::next_value::<Vector4>(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            3usize => {
                                if _serde::__private::Option::is_some(&m_toRot) {
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field("toRot"),
                                    );
                                }
                                m_toRot = _serde::__private::Some(
                                    match __A::next_value::<Quaternion>(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            4usize => {
                                if _serde::__private::Option::is_some(&m_lastPos) {
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "lastPos",
                                        ),
                                    );
                                }
                                m_lastPos = _serde::__private::Some(
                                    match __A::next_value::<Vector4>(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            5usize => {
                                if _serde::__private::Option::is_some(&m_lastRot) {
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "lastRot",
                                        ),
                                    );
                                }
                                m_lastRot = _serde::__private::Some(
                                    match __A::next_value::<Quaternion>(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            6usize => {
                                if _serde::__private::Option::is_some(
                                    &m_characterPoseAtBeginningOfTransition,
                                ) {
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "characterPoseAtBeginningOfTransition",
                                        ),
                                    );
                                }
                                m_characterPoseAtBeginningOfTransition = _serde::__private::Some(
                                    match __A::next_value::<Vec<QsTransform>>(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            7usize => {
                                if _serde::__private::Option::is_some(&m_timeRemaining) {
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "timeRemaining",
                                        ),
                                    );
                                }
                                m_timeRemaining = _serde::__private::Some(
                                    match __A::next_value::<f32>(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            8usize => {
                                if _serde::__private::Option::is_some(&m_timeInTransition) {
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "timeInTransition",
                                        ),
                                    );
                                }
                                m_timeInTransition = _serde::__private::Some(
                                    match __A::next_value::<f32>(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            9usize => {
                                if _serde::__private::Option::is_some(
                                    &m_toGeneratorSelfTranstitionMode,
                                ) {
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "toGeneratorSelfTranstitionMode",
                                        ),
                                    );
                                }
                                m_toGeneratorSelfTranstitionMode = _serde::__private::Some(
                                    match __A::next_value::<SelfTransitionMode>(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            10usize => {
                                if _serde::__private::Option::is_some(
                                    &m_initializeCharacterPose,
                                ) {
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "initializeCharacterPose",
                                        ),
                                    );
                                }
                                m_initializeCharacterPose = _serde::__private::Some(
                                    match __A::next_value::<bool>(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            11usize => {
                                if _serde::__private::Option::is_some(&m_alignThisFrame) {
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "alignThisFrame",
                                        ),
                                    );
                                }
                                m_alignThisFrame = _serde::__private::Some(
                                    match __A::next_value::<bool>(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            12usize => {
                                if _serde::__private::Option::is_some(
                                    &m_alignmentFinished,
                                ) {
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "alignmentFinished",
                                        ),
                                    );
                                }
                                m_alignmentFinished = _serde::__private::Some(
                                    match __A::next_value::<bool>(&mut __map) {
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
                    let m_fromPos = match m_fromPos {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field("fromPos"),
                            );
                        }
                    };
                    let m_fromRot = match m_fromRot {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field("fromRot"),
                            );
                        }
                    };
                    let m_toPos = match m_toPos {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field("toPos"),
                            );
                        }
                    };
                    let m_toRot = match m_toRot {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field("toRot"),
                            );
                        }
                    };
                    let m_lastPos = match m_lastPos {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field("lastPos"),
                            );
                        }
                    };
                    let m_lastRot = match m_lastRot {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field("lastRot"),
                            );
                        }
                    };
                    let m_characterPoseAtBeginningOfTransition = match m_characterPoseAtBeginningOfTransition {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "characterPoseAtBeginningOfTransition",
                                ),
                            );
                        }
                    };
                    let m_timeRemaining = match m_timeRemaining {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "timeRemaining",
                                ),
                            );
                        }
                    };
                    let m_timeInTransition = match m_timeInTransition {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "timeInTransition",
                                ),
                            );
                        }
                    };
                    let m_toGeneratorSelfTranstitionMode = match m_toGeneratorSelfTranstitionMode {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "toGeneratorSelfTranstitionMode",
                                ),
                            );
                        }
                    };
                    let m_initializeCharacterPose = match m_initializeCharacterPose {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "initializeCharacterPose",
                                ),
                            );
                        }
                    };
                    let m_alignThisFrame = match m_alignThisFrame {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "alignThisFrame",
                                ),
                            );
                        }
                    };
                    let m_alignmentFinished = match m_alignmentFinished {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "alignmentFinished",
                                ),
                            );
                        }
                    };
                    _serde::__private::Ok(hkbBlendingTransitionEffectInternalState {
                        __ptr,
                        parent,
                        m_fromPos,
                        m_fromRot,
                        m_toPos,
                        m_toRot,
                        m_lastPos,
                        m_lastRot,
                        m_characterPoseAtBeginningOfTransition,
                        m_timeRemaining,
                        m_timeInTransition,
                        m_toGeneratorSelfTranstitionMode,
                        m_initializeCharacterPose,
                        m_alignThisFrame,
                        m_alignmentFinished,
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
                    let mut m_fromPos: _serde::__private::Option<Vector4> = _serde::__private::None;
                    let mut m_fromRot: _serde::__private::Option<Quaternion> = _serde::__private::None;
                    let mut m_toPos: _serde::__private::Option<Vector4> = _serde::__private::None;
                    let mut m_toRot: _serde::__private::Option<Quaternion> = _serde::__private::None;
                    let mut m_lastPos: _serde::__private::Option<Vector4> = _serde::__private::None;
                    let mut m_lastRot: _serde::__private::Option<Quaternion> = _serde::__private::None;
                    let mut m_characterPoseAtBeginningOfTransition: _serde::__private::Option<
                        Vec<QsTransform>,
                    > = _serde::__private::None;
                    let mut m_timeRemaining: _serde::__private::Option<f32> = _serde::__private::None;
                    let mut m_timeInTransition: _serde::__private::Option<f32> = _serde::__private::None;
                    let mut m_toGeneratorSelfTranstitionMode: _serde::__private::Option<
                        SelfTransitionMode,
                    > = _serde::__private::None;
                    let mut m_initializeCharacterPose: _serde::__private::Option<bool> = _serde::__private::None;
                    let mut m_alignThisFrame: _serde::__private::Option<bool> = _serde::__private::None;
                    let mut m_alignmentFinished: _serde::__private::Option<bool> = _serde::__private::None;
                    while let _serde::__private::Some(__key) = {
                        __A::next_key::<__Field>(&mut __map)?
                    } {
                        match __key {
                            __Field::m_fromPos => {
                                #[cfg(
                                    any(feature = "strict", feature = "ignore_duplicates")
                                )]
                                if _serde::__private::Option::is_some(&m_fromPos) {
                                    #[cfg(feature = "ignore_duplicates")]
                                    {
                                        __A::skip_value(&mut __map)?;
                                        continue;
                                    }
                                    #[cfg(feature = "strict")]
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "fromPos",
                                        ),
                                    );
                                }
                                m_fromPos = _serde::__private::Some(
                                    match __A::next_value::<Vector4>(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            __Field::m_fromRot => {
                                #[cfg(
                                    any(feature = "strict", feature = "ignore_duplicates")
                                )]
                                if _serde::__private::Option::is_some(&m_fromRot) {
                                    #[cfg(feature = "ignore_duplicates")]
                                    {
                                        __A::skip_value(&mut __map)?;
                                        continue;
                                    }
                                    #[cfg(feature = "strict")]
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "fromRot",
                                        ),
                                    );
                                }
                                m_fromRot = _serde::__private::Some(
                                    match __A::next_value::<Quaternion>(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            __Field::m_toPos => {
                                #[cfg(
                                    any(feature = "strict", feature = "ignore_duplicates")
                                )]
                                if _serde::__private::Option::is_some(&m_toPos) {
                                    #[cfg(feature = "ignore_duplicates")]
                                    {
                                        __A::skip_value(&mut __map)?;
                                        continue;
                                    }
                                    #[cfg(feature = "strict")]
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field("toPos"),
                                    );
                                }
                                m_toPos = _serde::__private::Some(
                                    match __A::next_value::<Vector4>(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            __Field::m_toRot => {
                                #[cfg(
                                    any(feature = "strict", feature = "ignore_duplicates")
                                )]
                                if _serde::__private::Option::is_some(&m_toRot) {
                                    #[cfg(feature = "ignore_duplicates")]
                                    {
                                        __A::skip_value(&mut __map)?;
                                        continue;
                                    }
                                    #[cfg(feature = "strict")]
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field("toRot"),
                                    );
                                }
                                m_toRot = _serde::__private::Some(
                                    match __A::next_value::<Quaternion>(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            __Field::m_lastPos => {
                                #[cfg(
                                    any(feature = "strict", feature = "ignore_duplicates")
                                )]
                                if _serde::__private::Option::is_some(&m_lastPos) {
                                    #[cfg(feature = "ignore_duplicates")]
                                    {
                                        __A::skip_value(&mut __map)?;
                                        continue;
                                    }
                                    #[cfg(feature = "strict")]
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "lastPos",
                                        ),
                                    );
                                }
                                m_lastPos = _serde::__private::Some(
                                    match __A::next_value::<Vector4>(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            __Field::m_lastRot => {
                                #[cfg(
                                    any(feature = "strict", feature = "ignore_duplicates")
                                )]
                                if _serde::__private::Option::is_some(&m_lastRot) {
                                    #[cfg(feature = "ignore_duplicates")]
                                    {
                                        __A::skip_value(&mut __map)?;
                                        continue;
                                    }
                                    #[cfg(feature = "strict")]
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "lastRot",
                                        ),
                                    );
                                }
                                m_lastRot = _serde::__private::Some(
                                    match __A::next_value::<Quaternion>(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            __Field::m_characterPoseAtBeginningOfTransition => {
                                #[cfg(
                                    any(feature = "strict", feature = "ignore_duplicates")
                                )]
                                if _serde::__private::Option::is_some(
                                    &m_characterPoseAtBeginningOfTransition,
                                ) {
                                    #[cfg(feature = "ignore_duplicates")]
                                    {
                                        __A::skip_value(&mut __map)?;
                                        continue;
                                    }
                                    #[cfg(feature = "strict")]
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "characterPoseAtBeginningOfTransition",
                                        ),
                                    );
                                }
                                m_characterPoseAtBeginningOfTransition = _serde::__private::Some(
                                    match __A::next_value::<Vec<QsTransform>>(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            __Field::m_timeRemaining => {
                                #[cfg(
                                    any(feature = "strict", feature = "ignore_duplicates")
                                )]
                                if _serde::__private::Option::is_some(&m_timeRemaining) {
                                    #[cfg(feature = "ignore_duplicates")]
                                    {
                                        __A::skip_value(&mut __map)?;
                                        continue;
                                    }
                                    #[cfg(feature = "strict")]
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "timeRemaining",
                                        ),
                                    );
                                }
                                m_timeRemaining = _serde::__private::Some(
                                    match __A::next_value::<f32>(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            __Field::m_timeInTransition => {
                                #[cfg(
                                    any(feature = "strict", feature = "ignore_duplicates")
                                )]
                                if _serde::__private::Option::is_some(&m_timeInTransition) {
                                    #[cfg(feature = "ignore_duplicates")]
                                    {
                                        __A::skip_value(&mut __map)?;
                                        continue;
                                    }
                                    #[cfg(feature = "strict")]
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "timeInTransition",
                                        ),
                                    );
                                }
                                m_timeInTransition = _serde::__private::Some(
                                    match __A::next_value::<f32>(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            __Field::m_toGeneratorSelfTranstitionMode => {
                                #[cfg(
                                    any(feature = "strict", feature = "ignore_duplicates")
                                )]
                                if _serde::__private::Option::is_some(
                                    &m_toGeneratorSelfTranstitionMode,
                                ) {
                                    #[cfg(feature = "ignore_duplicates")]
                                    {
                                        __A::skip_value(&mut __map)?;
                                        continue;
                                    }
                                    #[cfg(feature = "strict")]
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "toGeneratorSelfTranstitionMode",
                                        ),
                                    );
                                }
                                m_toGeneratorSelfTranstitionMode = _serde::__private::Some(
                                    match __A::next_value::<SelfTransitionMode>(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            __Field::m_initializeCharacterPose => {
                                #[cfg(
                                    any(feature = "strict", feature = "ignore_duplicates")
                                )]
                                if _serde::__private::Option::is_some(
                                    &m_initializeCharacterPose,
                                ) {
                                    #[cfg(feature = "ignore_duplicates")]
                                    {
                                        __A::skip_value(&mut __map)?;
                                        continue;
                                    }
                                    #[cfg(feature = "strict")]
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "initializeCharacterPose",
                                        ),
                                    );
                                }
                                m_initializeCharacterPose = _serde::__private::Some(
                                    match __A::next_value::<bool>(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            __Field::m_alignThisFrame => {
                                #[cfg(
                                    any(feature = "strict", feature = "ignore_duplicates")
                                )]
                                if _serde::__private::Option::is_some(&m_alignThisFrame) {
                                    #[cfg(feature = "ignore_duplicates")]
                                    {
                                        __A::skip_value(&mut __map)?;
                                        continue;
                                    }
                                    #[cfg(feature = "strict")]
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "alignThisFrame",
                                        ),
                                    );
                                }
                                m_alignThisFrame = _serde::__private::Some(
                                    match __A::next_value::<bool>(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            __Field::m_alignmentFinished => {
                                #[cfg(
                                    any(feature = "strict", feature = "ignore_duplicates")
                                )]
                                if _serde::__private::Option::is_some(
                                    &m_alignmentFinished,
                                ) {
                                    #[cfg(feature = "ignore_duplicates")]
                                    {
                                        __A::skip_value(&mut __map)?;
                                        continue;
                                    }
                                    #[cfg(feature = "strict")]
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "alignmentFinished",
                                        ),
                                    );
                                }
                                m_alignmentFinished = _serde::__private::Some(
                                    match __A::next_value::<bool>(&mut __map) {
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
                    let m_fromPos = match m_fromPos {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            #[cfg(feature = "strict")]
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field("fromPos"),
                            );
                            #[cfg(not(feature = "strict"))] Default::default()
                        }
                    };
                    let m_fromRot = match m_fromRot {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            #[cfg(feature = "strict")]
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field("fromRot"),
                            );
                            #[cfg(not(feature = "strict"))] Default::default()
                        }
                    };
                    let m_toPos = match m_toPos {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            #[cfg(feature = "strict")]
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field("toPos"),
                            );
                            #[cfg(not(feature = "strict"))] Default::default()
                        }
                    };
                    let m_toRot = match m_toRot {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            #[cfg(feature = "strict")]
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field("toRot"),
                            );
                            #[cfg(not(feature = "strict"))] Default::default()
                        }
                    };
                    let m_lastPos = match m_lastPos {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            #[cfg(feature = "strict")]
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field("lastPos"),
                            );
                            #[cfg(not(feature = "strict"))] Default::default()
                        }
                    };
                    let m_lastRot = match m_lastRot {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            #[cfg(feature = "strict")]
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field("lastRot"),
                            );
                            #[cfg(not(feature = "strict"))] Default::default()
                        }
                    };
                    let m_characterPoseAtBeginningOfTransition = match m_characterPoseAtBeginningOfTransition {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            #[cfg(feature = "strict")]
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "characterPoseAtBeginningOfTransition",
                                ),
                            );
                            #[cfg(not(feature = "strict"))] Default::default()
                        }
                    };
                    let m_timeRemaining = match m_timeRemaining {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            #[cfg(feature = "strict")]
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "timeRemaining",
                                ),
                            );
                            #[cfg(not(feature = "strict"))] Default::default()
                        }
                    };
                    let m_timeInTransition = match m_timeInTransition {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            #[cfg(feature = "strict")]
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "timeInTransition",
                                ),
                            );
                            #[cfg(not(feature = "strict"))] Default::default()
                        }
                    };
                    let m_toGeneratorSelfTranstitionMode = match m_toGeneratorSelfTranstitionMode {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            #[cfg(feature = "strict")]
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "toGeneratorSelfTranstitionMode",
                                ),
                            );
                            #[cfg(not(feature = "strict"))] Default::default()
                        }
                    };
                    let m_initializeCharacterPose = match m_initializeCharacterPose {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            #[cfg(feature = "strict")]
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "initializeCharacterPose",
                                ),
                            );
                            #[cfg(not(feature = "strict"))] Default::default()
                        }
                    };
                    let m_alignThisFrame = match m_alignThisFrame {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            #[cfg(feature = "strict")]
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "alignThisFrame",
                                ),
                            );
                            #[cfg(not(feature = "strict"))] Default::default()
                        }
                    };
                    let m_alignmentFinished = match m_alignmentFinished {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            #[cfg(feature = "strict")]
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "alignmentFinished",
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
                    let __ptr = __A::class_ptr(&mut __map);
                    _serde::__private::Ok(hkbBlendingTransitionEffectInternalState {
                        __ptr: __ptr.clone(),
                        parent,
                        m_fromPos,
                        m_fromRot,
                        m_toPos,
                        m_toRot,
                        m_lastPos,
                        m_lastRot,
                        m_characterPoseAtBeginningOfTransition,
                        m_timeRemaining,
                        m_timeInTransition,
                        m_toGeneratorSelfTranstitionMode,
                        m_initializeCharacterPose,
                        m_alignThisFrame,
                        m_alignmentFinished,
                    })
                }
            }
            const FIELDS: &[&str] = &[
                "fromPos",
                "fromRot",
                "toPos",
                "toRot",
                "lastPos",
                "lastRot",
                "characterPoseAtBeginningOfTransition",
                "timeRemaining",
                "timeInTransition",
                "toGeneratorSelfTranstitionMode",
                "initializeCharacterPose",
                "alignThisFrame",
                "alignmentFinished",
            ];
            _serde::Deserializer::deserialize_struct(
                deserializer,
                "hkbBlendingTransitionEffectInternalState",
                FIELDS,
                __hkbBlendingTransitionEffectInternalStateVisitor {
                    marker: _serde::__private::PhantomData::<
                        hkbBlendingTransitionEffectInternalState,
                    >,
                    lifetime: _serde::__private::PhantomData,
                },
            )
        }
    }
};
