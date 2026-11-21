use super::class_requires::*;
use super::*;
/// # C++ Info
/// - name: `hkbExtrapolatingTransitionEffect`
/// - version: `0`
/// - signature: `0x422e2752`
/// - size: `  0`(x86)/`688`(x86_64)
/// -  vtable: `true`
#[allow(non_upper_case_globals, non_snake_case)]
#[cfg_attr(feature = "json_schema", derive(schemars::JsonSchema))]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(educe::Educe)]
#[educe(Debug, Clone, Default, PartialEq)]
pub struct hkbExtrapolatingTransitionEffect<'a> {
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
    /// - name: `fromGeneratorSyncInfo`(ctype: `struct hkbGeneratorSyncInfo`)
    /// - offset: `  0`(x86)/`320`(x86_64)
    /// - type_size: `  0`(x86)/`168`(x86_64)
    /// - flags: `SERIALIZE_IGNORED`
    #[cfg_attr(feature = "serde", serde(default))]
    #[cfg_attr(feature = "serde", serde(borrow))]
    #[cfg_attr(feature = "json_schema", schemars(rename = "fromGeneratorSyncInfo"))]
    #[cfg_attr(feature = "serde", serde(rename = "fromGeneratorSyncInfo"))]
    pub m_fromGeneratorSyncInfo: hkbGeneratorSyncInfo<'a>,
    /// # C++ Info
    /// - name: `fromGeneratorPartitionInfo`(ctype: `struct hkbGeneratorPartitionInfo`)
    /// - offset: `  0`(x86)/`488`(x86_64)
    /// - type_size: `  0`(x86)/` 40`(x86_64)
    /// - flags: `SERIALIZE_IGNORED`
    #[cfg_attr(feature = "serde", serde(default))]
    #[cfg_attr(feature = "serde", serde(borrow))]
    #[cfg_attr(feature = "json_schema", schemars(rename = "fromGeneratorPartitionInfo"))]
    #[cfg_attr(feature = "serde", serde(rename = "fromGeneratorPartitionInfo"))]
    pub m_fromGeneratorPartitionInfo: hkbGeneratorPartitionInfo<'a>,
    /// # C++ Info
    /// - name: `worldFromModel`(ctype: `hkQsTransform`)
    /// - offset: `  0`(x86)/`528`(x86_64)
    /// - type_size: `  0`(x86)/` 48`(x86_64)
    /// - flags: `SERIALIZE_IGNORED`
    #[cfg_attr(feature = "serde", serde(default))]
    #[cfg_attr(feature = "json_schema", schemars(rename = "worldFromModel"))]
    #[cfg_attr(feature = "serde", serde(rename = "worldFromModel"))]
    pub m_worldFromModel: QsTransform,
    /// # C++ Info
    /// - name: `motion`(ctype: `hkQsTransform`)
    /// - offset: `  0`(x86)/`576`(x86_64)
    /// - type_size: `  0`(x86)/` 48`(x86_64)
    /// - flags: `SERIALIZE_IGNORED`
    #[cfg_attr(feature = "serde", serde(default))]
    #[cfg_attr(feature = "json_schema", schemars(rename = "motion"))]
    #[cfg_attr(feature = "serde", serde(rename = "motion"))]
    pub m_motion: QsTransform,
    /// # C++ Info
    /// - name: `pose`(ctype: `hkArray<void>`)
    /// - offset: `  0`(x86)/`624`(x86_64)
    /// - type_size: `  0`(x86)/` 16`(x86_64)
    /// - flags: `SERIALIZE_IGNORED`
    #[cfg_attr(feature = "serde", serde(default))]
    #[cfg_attr(feature = "json_schema", schemars(rename = "pose"))]
    #[cfg_attr(feature = "serde", serde(rename = "pose"))]
    pub m_pose: Vec<()>,
    /// # C++ Info
    /// - name: `additivePose`(ctype: `hkArray<void>`)
    /// - offset: `  0`(x86)/`640`(x86_64)
    /// - type_size: `  0`(x86)/` 16`(x86_64)
    /// - flags: `SERIALIZE_IGNORED`
    #[cfg_attr(feature = "serde", serde(default))]
    #[cfg_attr(feature = "json_schema", schemars(rename = "additivePose"))]
    #[cfg_attr(feature = "serde", serde(rename = "additivePose"))]
    pub m_additivePose: Vec<()>,
    /// # C++ Info
    /// - name: `boneWeights`(ctype: `hkArray<void>`)
    /// - offset: `  0`(x86)/`656`(x86_64)
    /// - type_size: `  0`(x86)/` 16`(x86_64)
    /// - flags: `SERIALIZE_IGNORED`
    #[cfg_attr(feature = "serde", serde(default))]
    #[cfg_attr(feature = "json_schema", schemars(rename = "boneWeights"))]
    #[cfg_attr(feature = "serde", serde(rename = "boneWeights"))]
    pub m_boneWeights: Vec<()>,
    /// # C++ Info
    /// - name: `toGeneratorDuration`(ctype: `hkReal`)
    /// - offset: `  0`(x86)/`672`(x86_64)
    /// - type_size: `  0`(x86)/`  4`(x86_64)
    /// - flags: `SERIALIZE_IGNORED`
    #[cfg_attr(feature = "serde", serde(default))]
    #[cfg_attr(feature = "json_schema", schemars(rename = "toGeneratorDuration"))]
    #[cfg_attr(feature = "serde", serde(rename = "toGeneratorDuration"))]
    pub m_toGeneratorDuration: f32,
    /// # C++ Info
    /// - name: `isFromGeneratorActive`(ctype: `hkBool`)
    /// - offset: `  0`(x86)/`676`(x86_64)
    /// - type_size: `  0`(x86)/`  1`(x86_64)
    /// - flags: `SERIALIZE_IGNORED`
    #[cfg_attr(feature = "serde", serde(default))]
    #[cfg_attr(feature = "json_schema", schemars(rename = "isFromGeneratorActive"))]
    #[cfg_attr(feature = "serde", serde(rename = "isFromGeneratorActive"))]
    pub m_isFromGeneratorActive: bool,
    /// # C++ Info
    /// - name: `gotPose`(ctype: `hkBool`)
    /// - offset: `  0`(x86)/`677`(x86_64)
    /// - type_size: `  0`(x86)/`  1`(x86_64)
    /// - flags: `SERIALIZE_IGNORED`
    #[cfg_attr(feature = "serde", serde(default))]
    #[cfg_attr(feature = "json_schema", schemars(rename = "gotPose"))]
    #[cfg_attr(feature = "serde", serde(rename = "gotPose"))]
    pub m_gotPose: bool,
    /// # C++ Info
    /// - name: `gotAdditivePose`(ctype: `hkBool`)
    /// - offset: `  0`(x86)/`678`(x86_64)
    /// - type_size: `  0`(x86)/`  1`(x86_64)
    /// - flags: `SERIALIZE_IGNORED`
    #[cfg_attr(feature = "serde", serde(default))]
    #[cfg_attr(feature = "json_schema", schemars(rename = "gotAdditivePose"))]
    #[cfg_attr(feature = "serde", serde(rename = "gotAdditivePose"))]
    pub m_gotAdditivePose: bool,
}
const _: () = {
    use havok_serde as _serde;
    impl<'a> _serde::HavokClass for hkbExtrapolatingTransitionEffect<'a> {
        #[inline]
        fn name(&self) -> &'static str {
            "hkbExtrapolatingTransitionEffect"
        }
        #[inline]
        fn signature(&self) -> _serde::__private::Signature {
            _serde::__private::Signature::new(0x422e2752)
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
            v.extend(self.m_fromGeneratorSyncInfo.deps_indexes());
            v.extend(self.m_fromGeneratorPartitionInfo.deps_indexes());
            v
        }
    }
    impl<'a> _serde::Serialize for hkbExtrapolatingTransitionEffect<'a> {
        fn serialize<S>(&self, __serializer: S) -> Result<S::Ok, S::Error>
        where
            S: _serde::ser::Serializer,
        {
            let class_meta = self
                .__ptr
                .as_ref()
                .map(|name| (name, _serde::__private::Signature::new(0x422e2752)));
            let mut serializer = __serializer
                .serialize_struct(
                    "hkbExtrapolatingTransitionEffect",
                    class_meta,
                    (0u64, 688u64),
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
            serializer
                .skip_field("fromGeneratorSyncInfo", &self.m_fromGeneratorSyncInfo)?;
            serializer
                .skip_field(
                    "fromGeneratorPartitionInfo",
                    &self.m_fromGeneratorPartitionInfo,
                )?;
            serializer.skip_field("worldFromModel", &self.m_worldFromModel)?;
            serializer.skip_field("motion", &self.m_motion)?;
            serializer.skip_array_field("pose", &self.m_pose, TypeSize::NonPtr)?;
            serializer
                .skip_array_field(
                    "additivePose",
                    &self.m_additivePose,
                    TypeSize::NonPtr,
                )?;
            serializer
                .skip_array_field("boneWeights", &self.m_boneWeights, TypeSize::NonPtr)?;
            serializer.skip_field("toGeneratorDuration", &self.m_toGeneratorDuration)?;
            serializer
                .skip_field("isFromGeneratorActive", &self.m_isFromGeneratorActive)?;
            serializer.skip_field("gotPose", &self.m_gotPose)?;
            serializer.skip_field("gotAdditivePose", &self.m_gotAdditivePose)?;
            serializer.pad_field([0u8; 0usize].as_slice(), [0u8; 9usize].as_slice())?;
            serializer.end()
        }
    }
};
#[doc(hidden)]
#[allow(non_upper_case_globals, unused_attributes, unused_qualifications)]
const _: () = {
    use havok_serde as _serde;
    #[automatically_derived]
    impl<'de> _serde::Deserialize<'de> for hkbExtrapolatingTransitionEffect<'de> {
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
            struct __hkbExtrapolatingTransitionEffectVisitor<'de> {
                marker: _serde::__private::PhantomData<
                    hkbExtrapolatingTransitionEffect<'de>,
                >,
                lifetime: _serde::__private::PhantomData<&'de ()>,
            }
            #[allow(clippy::match_single_binding)]
            #[allow(clippy::reversed_empty_ranges)]
            #[allow(clippy::single_match)]
            impl<'de> _serde::de::Visitor<'de>
            for __hkbExtrapolatingTransitionEffectVisitor<'de> {
                type Value = hkbExtrapolatingTransitionEffect<'de>;
                fn expecting(
                    &self,
                    __formatter: &mut core::fmt::Formatter,
                ) -> core::fmt::Result {
                    core::fmt::Formatter::write_str(
                        __formatter,
                        "struct hkbExtrapolatingTransitionEffect",
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
                    let mut m_fromGeneratorSyncInfo: _serde::__private::Option<
                        hkbGeneratorSyncInfo<'de>,
                    > = _serde::__private::None;
                    let mut m_fromGeneratorPartitionInfo: _serde::__private::Option<
                        hkbGeneratorPartitionInfo<'de>,
                    > = _serde::__private::None;
                    let mut m_worldFromModel: _serde::__private::Option<QsTransform> = _serde::__private::None;
                    let mut m_motion: _serde::__private::Option<QsTransform> = _serde::__private::None;
                    let mut m_pose: _serde::__private::Option<Vec<()>> = _serde::__private::None;
                    let mut m_additivePose: _serde::__private::Option<Vec<()>> = _serde::__private::None;
                    let mut m_boneWeights: _serde::__private::Option<Vec<()>> = _serde::__private::None;
                    let mut m_toGeneratorDuration: _serde::__private::Option<f32> = _serde::__private::None;
                    let mut m_isFromGeneratorActive: _serde::__private::Option<bool> = _serde::__private::None;
                    let mut m_gotPose: _serde::__private::Option<bool> = _serde::__private::None;
                    let mut m_gotAdditivePose: _serde::__private::Option<bool> = _serde::__private::None;
                    for i in 0..11usize {
                        match i {
                            0usize => {
                                if _serde::__private::Option::is_some(
                                    &m_fromGeneratorSyncInfo,
                                ) {
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "fromGeneratorSyncInfo",
                                        ),
                                    );
                                }
                                m_fromGeneratorSyncInfo = _serde::__private::Some(
                                    match __A::next_value::<
                                        hkbGeneratorSyncInfo<'de>,
                                    >(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            1usize => {
                                if _serde::__private::Option::is_some(
                                    &m_fromGeneratorPartitionInfo,
                                ) {
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "fromGeneratorPartitionInfo",
                                        ),
                                    );
                                }
                                m_fromGeneratorPartitionInfo = _serde::__private::Some(
                                    match __A::next_value::<
                                        hkbGeneratorPartitionInfo<'de>,
                                    >(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            2usize => {
                                if _serde::__private::Option::is_some(&m_worldFromModel) {
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "worldFromModel",
                                        ),
                                    );
                                }
                                m_worldFromModel = _serde::__private::Some(
                                    match __A::next_value::<QsTransform>(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            3usize => {
                                if _serde::__private::Option::is_some(&m_motion) {
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field("motion"),
                                    );
                                }
                                m_motion = _serde::__private::Some(
                                    match __A::next_value::<QsTransform>(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            4usize => {
                                if _serde::__private::Option::is_some(&m_pose) {
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field("pose"),
                                    );
                                }
                                m_pose = _serde::__private::Some(
                                    match __A::next_value::<Vec<()>>(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            5usize => {
                                if _serde::__private::Option::is_some(&m_additivePose) {
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "additivePose",
                                        ),
                                    );
                                }
                                m_additivePose = _serde::__private::Some(
                                    match __A::next_value::<Vec<()>>(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            6usize => {
                                if _serde::__private::Option::is_some(&m_boneWeights) {
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "boneWeights",
                                        ),
                                    );
                                }
                                m_boneWeights = _serde::__private::Some(
                                    match __A::next_value::<Vec<()>>(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            7usize => {
                                if _serde::__private::Option::is_some(
                                    &m_toGeneratorDuration,
                                ) {
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "toGeneratorDuration",
                                        ),
                                    );
                                }
                                m_toGeneratorDuration = _serde::__private::Some(
                                    match __A::next_value::<f32>(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            8usize => {
                                if _serde::__private::Option::is_some(
                                    &m_isFromGeneratorActive,
                                ) {
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "isFromGeneratorActive",
                                        ),
                                    );
                                }
                                m_isFromGeneratorActive = _serde::__private::Some(
                                    match __A::next_value::<bool>(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            9usize => {
                                if _serde::__private::Option::is_some(&m_gotPose) {
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "gotPose",
                                        ),
                                    );
                                }
                                m_gotPose = _serde::__private::Some(
                                    match __A::next_value::<bool>(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            10usize => {
                                if _serde::__private::Option::is_some(&m_gotAdditivePose) {
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "gotAdditivePose",
                                        ),
                                    );
                                }
                                m_gotAdditivePose = _serde::__private::Some(
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
                    __A::pad(&mut __map, 0usize, 9usize)?;
                    let m_fromGeneratorSyncInfo = match m_fromGeneratorSyncInfo {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "fromGeneratorSyncInfo",
                                ),
                            );
                        }
                    };
                    let m_fromGeneratorPartitionInfo = match m_fromGeneratorPartitionInfo {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "fromGeneratorPartitionInfo",
                                ),
                            );
                        }
                    };
                    let m_worldFromModel = match m_worldFromModel {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "worldFromModel",
                                ),
                            );
                        }
                    };
                    let m_motion = match m_motion {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field("motion"),
                            );
                        }
                    };
                    let m_pose = match m_pose {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field("pose"),
                            );
                        }
                    };
                    let m_additivePose = match m_additivePose {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "additivePose",
                                ),
                            );
                        }
                    };
                    let m_boneWeights = match m_boneWeights {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "boneWeights",
                                ),
                            );
                        }
                    };
                    let m_toGeneratorDuration = match m_toGeneratorDuration {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "toGeneratorDuration",
                                ),
                            );
                        }
                    };
                    let m_isFromGeneratorActive = match m_isFromGeneratorActive {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "isFromGeneratorActive",
                                ),
                            );
                        }
                    };
                    let m_gotPose = match m_gotPose {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field("gotPose"),
                            );
                        }
                    };
                    let m_gotAdditivePose = match m_gotAdditivePose {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "gotAdditivePose",
                                ),
                            );
                        }
                    };
                    _serde::__private::Ok(hkbExtrapolatingTransitionEffect {
                        __ptr,
                        parent,
                        m_fromGeneratorSyncInfo,
                        m_fromGeneratorPartitionInfo,
                        m_worldFromModel,
                        m_motion,
                        m_pose,
                        m_additivePose,
                        m_boneWeights,
                        m_toGeneratorDuration,
                        m_isFromGeneratorActive,
                        m_gotPose,
                        m_gotAdditivePose,
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
                    _serde::__private::Ok(hkbExtrapolatingTransitionEffect {
                        __ptr: __ptr.clone(),
                        parent,
                        ..Default::default()
                    })
                }
            }
            const FIELDS: &[&str] = &[
                "fromGeneratorSyncInfo",
                "fromGeneratorPartitionInfo",
                "worldFromModel",
                "motion",
                "pose",
                "additivePose",
                "boneWeights",
                "toGeneratorDuration",
                "isFromGeneratorActive",
                "gotPose",
                "gotAdditivePose",
            ];
            _serde::Deserializer::deserialize_struct(
                deserializer,
                "hkbExtrapolatingTransitionEffect",
                FIELDS,
                __hkbExtrapolatingTransitionEffectVisitor {
                    marker: _serde::__private::PhantomData::<
                        hkbExtrapolatingTransitionEffect,
                    >,
                    lifetime: _serde::__private::PhantomData,
                },
            )
        }
    }
};
