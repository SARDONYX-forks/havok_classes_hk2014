use super::class_requires::*;
use super::*;
/// # C++ Info
/// - name: `hkbHoldFromBlendingTransitionEffect`
/// - version: `0`
/// - signature: `0xc0052dee`
/// - size: `  0`(x86)/`400`(x86_64)
/// -  vtable: `true`
#[allow(non_upper_case_globals, non_snake_case)]
#[cfg_attr(feature = "json_schema", derive(schemars::JsonSchema))]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(educe::Educe)]
#[educe(Debug, Clone, Default, PartialEq)]
pub struct hkbHoldFromBlendingTransitionEffect<'a> {
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
    pub parent: hkbBlendingTransitionEffect<'a>,
    /// # C++ Info
    /// - name: `heldFromPose`(ctype: `void*`)
    /// - offset: `  0`(x86)/`320`(x86_64)
    /// - type_size: `  0`(x86)/`  8`(x86_64)
    /// - flags: `SERIALIZE_IGNORED`
    #[cfg_attr(feature = "serde", serde(default))]
    #[cfg_attr(feature = "serde", serde(borrow))]
    #[cfg_attr(feature = "json_schema", schemars(rename = "heldFromPose"))]
    #[cfg_attr(feature = "serde", serde(rename = "heldFromPose"))]
    pub m_heldFromPose: Pointer<'a>,
    /// # C++ Info
    /// - name: `heldFromPoseSize`(ctype: `hkInt32`)
    /// - offset: `  0`(x86)/`328`(x86_64)
    /// - type_size: `  0`(x86)/`  4`(x86_64)
    /// - flags: `SERIALIZE_IGNORED`
    #[cfg_attr(feature = "serde", serde(default))]
    #[cfg_attr(feature = "serde", serde(borrow))]
    #[cfg_attr(feature = "json_schema", schemars(rename = "heldFromPoseSize"))]
    #[cfg_attr(feature = "serde", serde(rename = "heldFromPoseSize"))]
    pub m_heldFromPoseSize: I32<'a>,
    /// # C++ Info
    /// - name: `heldWorldFromModel`(ctype: `hkQsTransform`)
    /// - offset: `  0`(x86)/`336`(x86_64)
    /// - type_size: `  0`(x86)/` 48`(x86_64)
    /// - flags: `SERIALIZE_IGNORED`
    #[cfg_attr(feature = "serde", serde(default))]
    #[cfg_attr(feature = "json_schema", schemars(rename = "heldWorldFromModel"))]
    #[cfg_attr(feature = "serde", serde(rename = "heldWorldFromModel"))]
    pub m_heldWorldFromModel: QsTransform,
    /// # C++ Info
    /// - name: `heldFromSkeleton`(ctype: `void*`)
    /// - offset: `  0`(x86)/`384`(x86_64)
    /// - type_size: `  0`(x86)/`  8`(x86_64)
    /// - flags: `SERIALIZE_IGNORED`
    #[cfg_attr(feature = "serde", serde(default))]
    #[cfg_attr(feature = "serde", serde(borrow))]
    #[cfg_attr(feature = "json_schema", schemars(rename = "heldFromSkeleton"))]
    #[cfg_attr(feature = "serde", serde(rename = "heldFromSkeleton"))]
    pub m_heldFromSkeleton: Pointer<'a>,
}
const _: () = {
    use havok_serde as _serde;
    impl<'a> _serde::HavokClass for hkbHoldFromBlendingTransitionEffect<'a> {
        #[inline]
        fn name(&self) -> &'static str {
            "hkbHoldFromBlendingTransitionEffect"
        }
        #[inline]
        fn signature(&self) -> _serde::__private::Signature {
            _serde::__private::Signature::new(0xc0052dee)
        }
        #[allow(clippy::let_and_return, clippy::vec_init_then_push)]
        fn deps_indexes(&self) -> Vec<&Pointer<'_>> {
            let mut v = Vec::new();
            v.push(&self.parent.parent.parent.parent.parent.m_variableBindingSet);
            v.extend(self.parent.parent.parent.parent.parent.m_cachedBindables.iter());
            v.push(&self.parent.parent.parent.parent.m_nodeInfo);
            v.extend(self.parent.parent.parent.m_partitionInfo.deps_indexes());
            v.push(&self.parent.parent.parent.m_syncInfo);
            v.push(&self.parent.parent.m_patchedBindingInfo);
            v.push(&self.parent.parent.m_fromGenerator);
            v.push(&self.parent.parent.m_toGenerator);
            v.push(&self.m_heldFromPose);
            v.push(&self.m_heldFromSkeleton);
            v
        }
    }
    impl<'a> _serde::Serialize for hkbHoldFromBlendingTransitionEffect<'a> {
        fn serialize<S>(&self, __serializer: S) -> Result<S::Ok, S::Error>
        where
            S: _serde::ser::Serializer,
        {
            let class_meta = self
                .__ptr
                .as_ref()
                .map(|name| (name, _serde::__private::Signature::new(0xc0052dee)));
            let mut serializer = __serializer
                .serialize_struct(
                    "hkbHoldFromBlendingTransitionEffect",
                    class_meta,
                    (0u64, 400u64),
                )?;
            serializer.pad_field([0u8; 0usize].as_slice(), [0u8; 8usize].as_slice())?;
            serializer
                .skip_field(
                    "memSizeAndRefCount",
                    &self.parent.parent.parent.parent.parent.parent.m_memSizeAndRefCount,
                )?;
            serializer.pad_field([0u8; 0usize].as_slice(), [0u8; 4usize].as_slice())?;
            serializer
                .serialize_field(
                    "variableBindingSet",
                    &self.parent.parent.parent.parent.parent.m_variableBindingSet,
                )?;
            serializer
                .skip_array_field(
                    "cachedBindables",
                    &self.parent.parent.parent.parent.parent.m_cachedBindables,
                    TypeSize::NonPtr,
                )?;
            serializer
                .skip_field(
                    "areBindablesCached",
                    &self.parent.parent.parent.parent.parent.m_areBindablesCached,
                )?;
            serializer
                .skip_field(
                    "hasEnableChanged",
                    &self.parent.parent.parent.parent.parent.m_hasEnableChanged,
                )?;
            serializer.pad_field([0u8; 0usize].as_slice(), [0u8; 6usize].as_slice())?;
            serializer
                .serialize_field(
                    "userData",
                    &self.parent.parent.parent.parent.m_userData,
                )?;
            serializer
                .serialize_field("name", &self.parent.parent.parent.parent.m_name)?;
            serializer.skip_field("id", &self.parent.parent.parent.parent.m_id)?;
            serializer
                .skip_field(
                    "cloneState",
                    &self.parent.parent.parent.parent.m_cloneState,
                )?;
            serializer.skip_field("type", &self.parent.parent.parent.parent.m_type)?;
            serializer.pad_field([0u8; 0usize].as_slice(), [0u8; 4usize].as_slice())?;
            serializer
                .skip_field("nodeInfo", &self.parent.parent.parent.parent.m_nodeInfo)?;
            serializer
                .skip_field(
                    "partitionInfo",
                    &self.parent.parent.parent.m_partitionInfo,
                )?;
            serializer.skip_field("syncInfo", &self.parent.parent.parent.m_syncInfo)?;
            serializer
                .skip_fixed_array_field(
                    "pad",
                    self.parent.parent.parent.m_pad.as_slice(),
                    TypeSize::NonPtr,
                )?;
            serializer.pad_field([0u8; 0usize].as_slice(), [0u8; 4usize].as_slice())?;
            serializer
                .serialize_field(
                    "selfTransitionMode",
                    &self.parent.parent.m_selfTransitionMode,
                )?;
            serializer.serialize_field("eventMode", &self.parent.parent.m_eventMode)?;
            serializer
                .skip_field("defaultEventMode", &self.parent.parent.m_defaultEventMode)?;
            serializer.pad_field([0u8; 0usize].as_slice(), [0u8; 5usize].as_slice())?;
            serializer
                .skip_field(
                    "patchedBindingInfo",
                    &self.parent.parent.m_patchedBindingInfo,
                )?;
            serializer.skip_field("fromGenerator", &self.parent.parent.m_fromGenerator)?;
            serializer.skip_field("toGenerator", &self.parent.parent.m_toGenerator)?;
            serializer.serialize_field("duration", &self.parent.m_duration)?;
            serializer
                .serialize_field(
                    "toGeneratorStartTimeFraction",
                    &self.parent.m_toGeneratorStartTimeFraction,
                )?;
            serializer.serialize_field("flags", &self.parent.m_flags)?;
            serializer.serialize_field("endMode", &self.parent.m_endMode)?;
            serializer.serialize_field("blendCurve", &self.parent.m_blendCurve)?;
            serializer.serialize_field("alignmentBone", &self.parent.m_alignmentBone)?;
            serializer.pad_field([0u8; 0usize].as_slice(), [0u8; 10usize].as_slice())?;
            serializer.skip_field("fromPos", &self.parent.m_fromPos)?;
            serializer.skip_field("fromRot", &self.parent.m_fromRot)?;
            serializer.skip_field("toPos", &self.parent.m_toPos)?;
            serializer.skip_field("toRot", &self.parent.m_toRot)?;
            serializer.skip_field("lastPos", &self.parent.m_lastPos)?;
            serializer.skip_field("lastRot", &self.parent.m_lastRot)?;
            serializer
                .skip_array_field(
                    "characterPoseAtBeginningOfTransition",
                    &self.parent.m_characterPoseAtBeginningOfTransition,
                    TypeSize::NonPtr,
                )?;
            serializer.skip_field("timeRemaining", &self.parent.m_timeRemaining)?;
            serializer.skip_field("timeInTransition", &self.parent.m_timeInTransition)?;
            serializer
                .skip_field(
                    "toGeneratorSelfTranstitionMode",
                    &self.parent.m_toGeneratorSelfTranstitionMode,
                )?;
            serializer
                .skip_field(
                    "initializeCharacterPose",
                    &self.parent.m_initializeCharacterPose,
                )?;
            serializer.skip_field("alignThisFrame", &self.parent.m_alignThisFrame)?;
            serializer
                .skip_field("alignmentFinished", &self.parent.m_alignmentFinished)?;
            serializer.pad_field([0u8; 0usize].as_slice(), [0u8; 4usize].as_slice())?;
            serializer.skip_field("heldFromPose", &self.m_heldFromPose)?;
            serializer.skip_field("heldFromPoseSize", &self.m_heldFromPoseSize)?;
            serializer.pad_field([0u8; 0usize].as_slice(), [0u8; 4usize].as_slice())?;
            serializer.skip_field("heldWorldFromModel", &self.m_heldWorldFromModel)?;
            serializer.skip_field("heldFromSkeleton", &self.m_heldFromSkeleton)?;
            serializer.pad_field([0u8; 0usize].as_slice(), [0u8; 8usize].as_slice())?;
            serializer.end()
        }
    }
};
#[doc(hidden)]
#[allow(non_upper_case_globals, unused_attributes, unused_qualifications)]
const _: () = {
    use havok_serde as _serde;
    #[automatically_derived]
    impl<'de> _serde::Deserialize<'de> for hkbHoldFromBlendingTransitionEffect<'de> {
        fn deserialize<__D>(deserializer: __D) -> core::result::Result<Self, __D::Error>
        where
            __D: _serde::Deserializer<'de>,
        {
            #[allow(non_camel_case_types)]
            enum __Field {
                m_variableBindingSet,
                m_userData,
                m_name,
                m_selfTransitionMode,
                m_eventMode,
                m_duration,
                m_toGeneratorStartTimeFraction,
                m_flags,
                m_endMode,
                m_blendCurve,
                m_alignmentBone,
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
                        "variableBindingSet" => Ok(__Field::m_variableBindingSet),
                        "userData" => Ok(__Field::m_userData),
                        "name" => Ok(__Field::m_name),
                        "selfTransitionMode" => Ok(__Field::m_selfTransitionMode),
                        "eventMode" => Ok(__Field::m_eventMode),
                        "duration" => Ok(__Field::m_duration),
                        "toGeneratorStartTimeFraction" => {
                            Ok(__Field::m_toGeneratorStartTimeFraction)
                        }
                        "flags" => Ok(__Field::m_flags),
                        "endMode" => Ok(__Field::m_endMode),
                        "blendCurve" => Ok(__Field::m_blendCurve),
                        "alignmentBone" => Ok(__Field::m_alignmentBone),
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
            struct __hkbHoldFromBlendingTransitionEffectVisitor<'de> {
                marker: _serde::__private::PhantomData<
                    hkbHoldFromBlendingTransitionEffect<'de>,
                >,
                lifetime: _serde::__private::PhantomData<&'de ()>,
            }
            #[allow(clippy::match_single_binding)]
            #[allow(clippy::reversed_empty_ranges)]
            #[allow(clippy::single_match)]
            impl<'de> _serde::de::Visitor<'de>
            for __hkbHoldFromBlendingTransitionEffectVisitor<'de> {
                type Value = hkbHoldFromBlendingTransitionEffect<'de>;
                fn expecting(
                    &self,
                    __formatter: &mut core::fmt::Formatter,
                ) -> core::fmt::Result {
                    core::fmt::Formatter::write_str(
                        __formatter,
                        "struct hkbHoldFromBlendingTransitionEffect",
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
                    let mut m_heldFromPose: _serde::__private::Option<Pointer<'de>> = _serde::__private::None;
                    let mut m_heldFromPoseSize: _serde::__private::Option<I32<'de>> = _serde::__private::None;
                    let mut m_heldWorldFromModel: _serde::__private::Option<
                        QsTransform,
                    > = _serde::__private::None;
                    let mut m_heldFromSkeleton: _serde::__private::Option<
                        Pointer<'de>,
                    > = _serde::__private::None;
                    for i in 0..4usize {
                        match i {
                            0usize => {
                                if _serde::__private::Option::is_some(&m_heldFromPose) {
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "heldFromPose",
                                        ),
                                    );
                                }
                                m_heldFromPose = _serde::__private::Some(
                                    match __A::next_value::<Pointer<'de>>(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            1usize => {
                                if _serde::__private::Option::is_some(&m_heldFromPoseSize) {
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "heldFromPoseSize",
                                        ),
                                    );
                                }
                                m_heldFromPoseSize = _serde::__private::Some(
                                    match __A::next_value::<I32<'de>>(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            2usize => {
                                if _serde::__private::Option::is_some(
                                    &m_heldWorldFromModel,
                                ) {
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "heldWorldFromModel",
                                        ),
                                    );
                                }
                                __A::pad(&mut __map, 0usize, 4usize)?;
                                m_heldWorldFromModel = _serde::__private::Some(
                                    match __A::next_value::<QsTransform>(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            3usize => {
                                if _serde::__private::Option::is_some(&m_heldFromSkeleton) {
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "heldFromSkeleton",
                                        ),
                                    );
                                }
                                m_heldFromSkeleton = _serde::__private::Some(
                                    match __A::next_value::<Pointer<'de>>(&mut __map) {
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
                    __A::pad(&mut __map, 0usize, 8usize)?;
                    let m_heldFromPose = match m_heldFromPose {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "heldFromPose",
                                ),
                            );
                        }
                    };
                    let m_heldFromPoseSize = match m_heldFromPoseSize {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "heldFromPoseSize",
                                ),
                            );
                        }
                    };
                    let m_heldWorldFromModel = match m_heldWorldFromModel {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "heldWorldFromModel",
                                ),
                            );
                        }
                    };
                    let m_heldFromSkeleton = match m_heldFromSkeleton {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "heldFromSkeleton",
                                ),
                            );
                        }
                    };
                    _serde::__private::Ok(hkbHoldFromBlendingTransitionEffect {
                        __ptr,
                        parent,
                        m_heldFromPose,
                        m_heldFromPoseSize,
                        m_heldWorldFromModel,
                        m_heldFromSkeleton,
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
                    let mut m_variableBindingSet: _serde::__private::Option<
                        Pointer<'de>,
                    > = _serde::__private::None;
                    let mut m_userData: _serde::__private::Option<Ulong> = _serde::__private::None;
                    let mut m_name: _serde::__private::Option<StringPtr<'de>> = _serde::__private::None;
                    let mut m_selfTransitionMode: _serde::__private::Option<
                        SelfTransitionMode,
                    > = _serde::__private::None;
                    let mut m_eventMode: _serde::__private::Option<EventMode> = _serde::__private::None;
                    let mut m_duration: _serde::__private::Option<f32> = _serde::__private::None;
                    let mut m_toGeneratorStartTimeFraction: _serde::__private::Option<
                        f32,
                    > = _serde::__private::None;
                    let mut m_flags: _serde::__private::Option<FlagBits> = _serde::__private::None;
                    let mut m_endMode: _serde::__private::Option<EndMode> = _serde::__private::None;
                    let mut m_blendCurve: _serde::__private::Option<BlendCurve> = _serde::__private::None;
                    let mut m_alignmentBone: _serde::__private::Option<I16<'de>> = _serde::__private::None;
                    while let _serde::__private::Some(__key) = {
                        __A::next_key::<__Field>(&mut __map)?
                    } {
                        match __key {
                            __Field::m_variableBindingSet => {
                                #[cfg(
                                    any(feature = "strict", feature = "ignore_duplicates")
                                )]
                                if _serde::__private::Option::is_some(
                                    &m_variableBindingSet,
                                ) {
                                    #[cfg(feature = "ignore_duplicates")]
                                    {
                                        __A::skip_value(&mut __map)?;
                                        continue;
                                    }
                                    #[cfg(feature = "strict")]
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "variableBindingSet",
                                        ),
                                    );
                                }
                                m_variableBindingSet = _serde::__private::Some(
                                    match __A::next_value::<Pointer<'de>>(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            __Field::m_userData => {
                                #[cfg(
                                    any(feature = "strict", feature = "ignore_duplicates")
                                )]
                                if _serde::__private::Option::is_some(&m_userData) {
                                    #[cfg(feature = "ignore_duplicates")]
                                    {
                                        __A::skip_value(&mut __map)?;
                                        continue;
                                    }
                                    #[cfg(feature = "strict")]
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "userData",
                                        ),
                                    );
                                }
                                m_userData = _serde::__private::Some(
                                    match __A::next_value::<Ulong>(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
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
                            __Field::m_selfTransitionMode => {
                                #[cfg(
                                    any(feature = "strict", feature = "ignore_duplicates")
                                )]
                                if _serde::__private::Option::is_some(
                                    &m_selfTransitionMode,
                                ) {
                                    #[cfg(feature = "ignore_duplicates")]
                                    {
                                        __A::skip_value(&mut __map)?;
                                        continue;
                                    }
                                    #[cfg(feature = "strict")]
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "selfTransitionMode",
                                        ),
                                    );
                                }
                                m_selfTransitionMode = _serde::__private::Some(
                                    match __A::next_value::<SelfTransitionMode>(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            __Field::m_eventMode => {
                                #[cfg(
                                    any(feature = "strict", feature = "ignore_duplicates")
                                )]
                                if _serde::__private::Option::is_some(&m_eventMode) {
                                    #[cfg(feature = "ignore_duplicates")]
                                    {
                                        __A::skip_value(&mut __map)?;
                                        continue;
                                    }
                                    #[cfg(feature = "strict")]
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "eventMode",
                                        ),
                                    );
                                }
                                m_eventMode = _serde::__private::Some(
                                    match __A::next_value::<EventMode>(&mut __map) {
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
                            __Field::m_toGeneratorStartTimeFraction => {
                                #[cfg(
                                    any(feature = "strict", feature = "ignore_duplicates")
                                )]
                                if _serde::__private::Option::is_some(
                                    &m_toGeneratorStartTimeFraction,
                                ) {
                                    #[cfg(feature = "ignore_duplicates")]
                                    {
                                        __A::skip_value(&mut __map)?;
                                        continue;
                                    }
                                    #[cfg(feature = "strict")]
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "toGeneratorStartTimeFraction",
                                        ),
                                    );
                                }
                                m_toGeneratorStartTimeFraction = _serde::__private::Some(
                                    match __A::next_value::<f32>(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            __Field::m_flags => {
                                #[cfg(
                                    any(feature = "strict", feature = "ignore_duplicates")
                                )]
                                if _serde::__private::Option::is_some(&m_flags) {
                                    #[cfg(feature = "ignore_duplicates")]
                                    {
                                        __A::skip_value(&mut __map)?;
                                        continue;
                                    }
                                    #[cfg(feature = "strict")]
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field("flags"),
                                    );
                                }
                                m_flags = _serde::__private::Some(
                                    match __A::next_value::<FlagBits>(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            __Field::m_endMode => {
                                #[cfg(
                                    any(feature = "strict", feature = "ignore_duplicates")
                                )]
                                if _serde::__private::Option::is_some(&m_endMode) {
                                    #[cfg(feature = "ignore_duplicates")]
                                    {
                                        __A::skip_value(&mut __map)?;
                                        continue;
                                    }
                                    #[cfg(feature = "strict")]
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "endMode",
                                        ),
                                    );
                                }
                                m_endMode = _serde::__private::Some(
                                    match __A::next_value::<EndMode>(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            __Field::m_blendCurve => {
                                #[cfg(
                                    any(feature = "strict", feature = "ignore_duplicates")
                                )]
                                if _serde::__private::Option::is_some(&m_blendCurve) {
                                    #[cfg(feature = "ignore_duplicates")]
                                    {
                                        __A::skip_value(&mut __map)?;
                                        continue;
                                    }
                                    #[cfg(feature = "strict")]
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "blendCurve",
                                        ),
                                    );
                                }
                                m_blendCurve = _serde::__private::Some(
                                    match __A::next_value::<BlendCurve>(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            __Field::m_alignmentBone => {
                                #[cfg(
                                    any(feature = "strict", feature = "ignore_duplicates")
                                )]
                                if _serde::__private::Option::is_some(&m_alignmentBone) {
                                    #[cfg(feature = "ignore_duplicates")]
                                    {
                                        __A::skip_value(&mut __map)?;
                                        continue;
                                    }
                                    #[cfg(feature = "strict")]
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "alignmentBone",
                                        ),
                                    );
                                }
                                m_alignmentBone = _serde::__private::Some(
                                    match __A::next_value::<I16<'de>>(&mut __map) {
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
                    let m_variableBindingSet = match m_variableBindingSet {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            #[cfg(feature = "strict")]
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "variableBindingSet",
                                ),
                            );
                            #[cfg(not(feature = "strict"))] Default::default()
                        }
                    };
                    let m_userData = match m_userData {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            #[cfg(feature = "strict")]
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field("userData"),
                            );
                            #[cfg(not(feature = "strict"))] Default::default()
                        }
                    };
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
                    let m_selfTransitionMode = match m_selfTransitionMode {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            #[cfg(feature = "strict")]
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "selfTransitionMode",
                                ),
                            );
                            #[cfg(not(feature = "strict"))] Default::default()
                        }
                    };
                    let m_eventMode = match m_eventMode {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            #[cfg(feature = "strict")]
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "eventMode",
                                ),
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
                    let m_toGeneratorStartTimeFraction = match m_toGeneratorStartTimeFraction {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            #[cfg(feature = "strict")]
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "toGeneratorStartTimeFraction",
                                ),
                            );
                            #[cfg(not(feature = "strict"))] Default::default()
                        }
                    };
                    let m_flags = match m_flags {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            #[cfg(feature = "strict")]
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field("flags"),
                            );
                            #[cfg(not(feature = "strict"))] Default::default()
                        }
                    };
                    let m_endMode = match m_endMode {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            #[cfg(feature = "strict")]
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field("endMode"),
                            );
                            #[cfg(not(feature = "strict"))] Default::default()
                        }
                    };
                    let m_blendCurve = match m_blendCurve {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            #[cfg(feature = "strict")]
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "blendCurve",
                                ),
                            );
                            #[cfg(not(feature = "strict"))] Default::default()
                        }
                    };
                    let m_alignmentBone = match m_alignmentBone {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            #[cfg(feature = "strict")]
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "alignmentBone",
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
                    let parent = hkbBindable {
                        __ptr: __ptr.clone(),
                        parent,
                        m_variableBindingSet,
                        ..Default::default()
                    };
                    let parent = hkbNode {
                        __ptr: __ptr.clone(),
                        parent,
                        m_userData,
                        m_name,
                        ..Default::default()
                    };
                    let parent = hkbGenerator {
                        __ptr: __ptr.clone(),
                        parent,
                        ..Default::default()
                    };
                    let parent = hkbTransitionEffect {
                        __ptr: __ptr.clone(),
                        parent,
                        m_selfTransitionMode,
                        m_eventMode,
                        ..Default::default()
                    };
                    let parent = hkbBlendingTransitionEffect {
                        __ptr: __ptr.clone(),
                        parent,
                        m_duration,
                        m_toGeneratorStartTimeFraction,
                        m_flags,
                        m_endMode,
                        m_blendCurve,
                        m_alignmentBone,
                        ..Default::default()
                    };
                    let __ptr = __A::class_ptr(&mut __map);
                    _serde::__private::Ok(hkbHoldFromBlendingTransitionEffect {
                        __ptr: __ptr.clone(),
                        parent,
                        ..Default::default()
                    })
                }
            }
            const FIELDS: &[&str] = &[
                "heldFromPose",
                "heldFromPoseSize",
                "heldWorldFromModel",
                "heldFromSkeleton",
            ];
            _serde::Deserializer::deserialize_struct(
                deserializer,
                "hkbHoldFromBlendingTransitionEffect",
                FIELDS,
                __hkbHoldFromBlendingTransitionEffectVisitor {
                    marker: _serde::__private::PhantomData::<
                        hkbHoldFromBlendingTransitionEffect,
                    >,
                    lifetime: _serde::__private::PhantomData,
                },
            )
        }
    }
};
