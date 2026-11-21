use super::class_requires::*;
use super::*;
/// # C++ Info
/// - name: `hkbBlendingTransitionEffect`
/// - version: `2`
/// - signature: `0x14e54c5c`
/// - size: `  0`(x86)/`320`(x86_64)
/// -  vtable: `true`
#[allow(non_upper_case_globals, non_snake_case)]
#[cfg_attr(feature = "json_schema", derive(schemars::JsonSchema))]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(educe::Educe)]
#[educe(Debug, Clone, Default, PartialEq)]
pub struct hkbBlendingTransitionEffect<'a> {
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
    pub parent: hkbTransitionEffect<'a>,
    /// # C++ Info
    /// - name: `duration`(ctype: `hkReal`)
    /// - offset: `  0`(x86)/`168`(x86_64)
    /// - type_size: `  0`(x86)/`  4`(x86_64)
    #[cfg_attr(feature = "serde_default", serde(default))]
    #[cfg_attr(feature = "json_schema", schemars(rename = "duration"))]
    #[cfg_attr(feature = "serde", serde(rename = "duration"))]
    pub m_duration: f32,
    /// # C++ Info
    /// - name: `toGeneratorStartTimeFraction`(ctype: `hkReal`)
    /// - offset: `  0`(x86)/`172`(x86_64)
    /// - type_size: `  0`(x86)/`  4`(x86_64)
    #[cfg_attr(feature = "serde_default", serde(default))]
    #[cfg_attr(
        feature = "json_schema",
        schemars(rename = "toGeneratorStartTimeFraction")
    )]
    #[cfg_attr(feature = "serde", serde(rename = "toGeneratorStartTimeFraction"))]
    pub m_toGeneratorStartTimeFraction: f32,
    /// # C++ Info
    /// - name: `flags`(ctype: `flags FlagBits`)
    /// - offset: `  0`(x86)/`176`(x86_64)
    /// - type_size: `  0`(x86)/`  2`(x86_64)
    #[cfg_attr(feature = "serde_default", serde(default))]
    #[cfg_attr(feature = "json_schema", schemars(rename = "flags"))]
    #[cfg_attr(feature = "serde", serde(rename = "flags"))]
    pub m_flags: FlagBits,
    /// # C++ Info
    /// - name: `endMode`(ctype: `enum EndMode`)
    /// - offset: `  0`(x86)/`178`(x86_64)
    /// - type_size: `  0`(x86)/`  1`(x86_64)
    #[cfg_attr(feature = "serde_default", serde(default))]
    #[cfg_attr(feature = "json_schema", schemars(rename = "endMode"))]
    #[cfg_attr(feature = "serde", serde(rename = "endMode"))]
    pub m_endMode: EndMode,
    /// # C++ Info
    /// - name: `blendCurve`(ctype: `enum BlendCurve`)
    /// - offset: `  0`(x86)/`179`(x86_64)
    /// - type_size: `  0`(x86)/`  1`(x86_64)
    #[cfg_attr(feature = "serde_default", serde(default))]
    #[cfg_attr(feature = "json_schema", schemars(rename = "blendCurve"))]
    #[cfg_attr(feature = "serde", serde(rename = "blendCurve"))]
    pub m_blendCurve: BlendCurve,
    /// # C++ Info
    /// - name: `alignmentBone`(ctype: `hkInt16`)
    /// - offset: `  0`(x86)/`180`(x86_64)
    /// - type_size: `  0`(x86)/`  2`(x86_64)
    #[cfg_attr(feature = "serde_default", serde(default))]
    #[cfg_attr(feature = "serde", serde(borrow))]
    #[cfg_attr(feature = "json_schema", schemars(rename = "alignmentBone"))]
    #[cfg_attr(feature = "serde", serde(rename = "alignmentBone"))]
    pub m_alignmentBone: I16<'a>,
    /// # C++ Info
    /// - name: `fromPos`(ctype: `hkVector4`)
    /// - offset: `  0`(x86)/`192`(x86_64)
    /// - type_size: `  0`(x86)/` 16`(x86_64)
    /// - flags: `SERIALIZE_IGNORED`
    #[cfg_attr(feature = "serde", serde(default))]
    #[cfg_attr(feature = "json_schema", schemars(rename = "fromPos"))]
    #[cfg_attr(feature = "serde", serde(rename = "fromPos"))]
    pub m_fromPos: Vector4,
    /// # C++ Info
    /// - name: `fromRot`(ctype: `hkQuaternion`)
    /// - offset: `  0`(x86)/`208`(x86_64)
    /// - type_size: `  0`(x86)/` 16`(x86_64)
    /// - flags: `SERIALIZE_IGNORED`
    #[cfg_attr(feature = "serde", serde(default))]
    #[cfg_attr(feature = "json_schema", schemars(rename = "fromRot"))]
    #[cfg_attr(feature = "serde", serde(rename = "fromRot"))]
    pub m_fromRot: Quaternion,
    /// # C++ Info
    /// - name: `toPos`(ctype: `hkVector4`)
    /// - offset: `  0`(x86)/`224`(x86_64)
    /// - type_size: `  0`(x86)/` 16`(x86_64)
    /// - flags: `SERIALIZE_IGNORED`
    #[cfg_attr(feature = "serde", serde(default))]
    #[cfg_attr(feature = "json_schema", schemars(rename = "toPos"))]
    #[cfg_attr(feature = "serde", serde(rename = "toPos"))]
    pub m_toPos: Vector4,
    /// # C++ Info
    /// - name: `toRot`(ctype: `hkQuaternion`)
    /// - offset: `  0`(x86)/`240`(x86_64)
    /// - type_size: `  0`(x86)/` 16`(x86_64)
    /// - flags: `SERIALIZE_IGNORED`
    #[cfg_attr(feature = "serde", serde(default))]
    #[cfg_attr(feature = "json_schema", schemars(rename = "toRot"))]
    #[cfg_attr(feature = "serde", serde(rename = "toRot"))]
    pub m_toRot: Quaternion,
    /// # C++ Info
    /// - name: `lastPos`(ctype: `hkVector4`)
    /// - offset: `  0`(x86)/`256`(x86_64)
    /// - type_size: `  0`(x86)/` 16`(x86_64)
    /// - flags: `SERIALIZE_IGNORED`
    #[cfg_attr(feature = "serde", serde(default))]
    #[cfg_attr(feature = "json_schema", schemars(rename = "lastPos"))]
    #[cfg_attr(feature = "serde", serde(rename = "lastPos"))]
    pub m_lastPos: Vector4,
    /// # C++ Info
    /// - name: `lastRot`(ctype: `hkQuaternion`)
    /// - offset: `  0`(x86)/`272`(x86_64)
    /// - type_size: `  0`(x86)/` 16`(x86_64)
    /// - flags: `SERIALIZE_IGNORED`
    #[cfg_attr(feature = "serde", serde(default))]
    #[cfg_attr(feature = "json_schema", schemars(rename = "lastRot"))]
    #[cfg_attr(feature = "serde", serde(rename = "lastRot"))]
    pub m_lastRot: Quaternion,
    /// # C++ Info
    /// - name: `characterPoseAtBeginningOfTransition`(ctype: `hkArray<void>`)
    /// - offset: `  0`(x86)/`288`(x86_64)
    /// - type_size: `  0`(x86)/` 16`(x86_64)
    /// - flags: `SERIALIZE_IGNORED`
    #[cfg_attr(feature = "serde", serde(default))]
    #[cfg_attr(
        feature = "json_schema",
        schemars(rename = "characterPoseAtBeginningOfTransition")
    )]
    #[cfg_attr(
        feature = "serde",
        serde(rename = "characterPoseAtBeginningOfTransition")
    )]
    pub m_characterPoseAtBeginningOfTransition: Vec<()>,
    /// # C++ Info
    /// - name: `timeRemaining`(ctype: `hkReal`)
    /// - offset: `  0`(x86)/`304`(x86_64)
    /// - type_size: `  0`(x86)/`  4`(x86_64)
    /// - flags: `SERIALIZE_IGNORED`
    #[cfg_attr(feature = "serde", serde(default))]
    #[cfg_attr(feature = "json_schema", schemars(rename = "timeRemaining"))]
    #[cfg_attr(feature = "serde", serde(rename = "timeRemaining"))]
    pub m_timeRemaining: f32,
    /// # C++ Info
    /// - name: `timeInTransition`(ctype: `hkReal`)
    /// - offset: `  0`(x86)/`308`(x86_64)
    /// - type_size: `  0`(x86)/`  4`(x86_64)
    /// - flags: `SERIALIZE_IGNORED`
    #[cfg_attr(feature = "serde", serde(default))]
    #[cfg_attr(feature = "json_schema", schemars(rename = "timeInTransition"))]
    #[cfg_attr(feature = "serde", serde(rename = "timeInTransition"))]
    pub m_timeInTransition: f32,
    /// # C++ Info
    /// - name: `toGeneratorSelfTranstitionMode`(ctype: `enum unknown`)
    /// - offset: `  0`(x86)/`312`(x86_64)
    /// - type_size: `  0`(x86)/`  1`(x86_64)
    /// - flags: `SERIALIZE_IGNORED`
    #[cfg_attr(feature = "serde", serde(default))]
    #[cfg_attr(
        feature = "json_schema",
        schemars(rename = "toGeneratorSelfTranstitionMode")
    )]
    #[cfg_attr(feature = "serde", serde(rename = "toGeneratorSelfTranstitionMode"))]
    pub m_toGeneratorSelfTranstitionMode: I8<'a>,
    /// # C++ Info
    /// - name: `initializeCharacterPose`(ctype: `hkBool`)
    /// - offset: `  0`(x86)/`313`(x86_64)
    /// - type_size: `  0`(x86)/`  1`(x86_64)
    /// - flags: `SERIALIZE_IGNORED`
    #[cfg_attr(feature = "serde", serde(default))]
    #[cfg_attr(feature = "json_schema", schemars(rename = "initializeCharacterPose"))]
    #[cfg_attr(feature = "serde", serde(rename = "initializeCharacterPose"))]
    pub m_initializeCharacterPose: bool,
    /// # C++ Info
    /// - name: `alignThisFrame`(ctype: `hkBool`)
    /// - offset: `  0`(x86)/`314`(x86_64)
    /// - type_size: `  0`(x86)/`  1`(x86_64)
    /// - flags: `SERIALIZE_IGNORED`
    #[cfg_attr(feature = "serde", serde(default))]
    #[cfg_attr(feature = "json_schema", schemars(rename = "alignThisFrame"))]
    #[cfg_attr(feature = "serde", serde(rename = "alignThisFrame"))]
    pub m_alignThisFrame: bool,
    /// # C++ Info
    /// - name: `alignmentFinished`(ctype: `hkBool`)
    /// - offset: `  0`(x86)/`315`(x86_64)
    /// - type_size: `  0`(x86)/`  1`(x86_64)
    /// - flags: `SERIALIZE_IGNORED`
    #[cfg_attr(feature = "serde", serde(default))]
    #[cfg_attr(feature = "json_schema", schemars(rename = "alignmentFinished"))]
    #[cfg_attr(feature = "serde", serde(rename = "alignmentFinished"))]
    pub m_alignmentFinished: bool,
}
const _: () = {
    use havok_serde as _serde;
    impl<'a> _serde::HavokClass for hkbBlendingTransitionEffect<'a> {
        #[inline]
        fn name(&self) -> &'static str {
            "hkbBlendingTransitionEffect"
        }
        #[inline]
        fn signature(&self) -> _serde::__private::Signature {
            _serde::__private::Signature::new(0x14e54c5c)
        }
        #[allow(clippy::let_and_return, clippy::vec_init_then_push)]
        fn deps_indexes(&self) -> Vec<&Pointer<'_>> {
            let mut v = Vec::new();
            v.push(&self.parent.parent.parent.parent.m_variableBindingSet);
            v.extend(self.parent.parent.parent.parent.m_cachedBindables.iter());
            v.push(&self.parent.parent.parent.m_nodeInfo);
            v.extend(self.parent.parent.m_partitionInfo.deps_indexes());
            v.push(&self.parent.parent.m_syncInfo);
            v.push(&self.parent.m_patchedBindingInfo);
            v.push(&self.parent.m_fromGenerator);
            v.push(&self.parent.m_toGenerator);
            v
        }
    }
    impl<'a> _serde::Serialize for hkbBlendingTransitionEffect<'a> {
        fn serialize<S>(&self, __serializer: S) -> Result<S::Ok, S::Error>
        where
            S: _serde::ser::Serializer,
        {
            let class_meta = self
                .__ptr
                .as_ref()
                .map(|name| (name, _serde::__private::Signature::new(0x14e54c5c)));
            let mut serializer = __serializer
                .serialize_struct(
                    "hkbBlendingTransitionEffect",
                    class_meta,
                    (0u64, 320u64),
                )?;
            serializer.pad_field([0u8; 0usize].as_slice(), [0u8; 8usize].as_slice())?;
            serializer
                .skip_field(
                    "memSizeAndRefCount",
                    &self.parent.parent.parent.parent.parent.m_memSizeAndRefCount,
                )?;
            serializer.pad_field([0u8; 0usize].as_slice(), [0u8; 4usize].as_slice())?;
            serializer
                .serialize_field(
                    "variableBindingSet",
                    &self.parent.parent.parent.parent.m_variableBindingSet,
                )?;
            serializer
                .skip_array_field(
                    "cachedBindables",
                    &self.parent.parent.parent.parent.m_cachedBindables,
                    TypeSize::NonPtr,
                )?;
            serializer
                .skip_field(
                    "areBindablesCached",
                    &self.parent.parent.parent.parent.m_areBindablesCached,
                )?;
            serializer
                .skip_field(
                    "hasEnableChanged",
                    &self.parent.parent.parent.parent.m_hasEnableChanged,
                )?;
            serializer.pad_field([0u8; 0usize].as_slice(), [0u8; 6usize].as_slice())?;
            serializer
                .serialize_field("userData", &self.parent.parent.parent.m_userData)?;
            serializer.serialize_field("name", &self.parent.parent.parent.m_name)?;
            serializer.skip_field("id", &self.parent.parent.parent.m_id)?;
            serializer
                .skip_field("cloneState", &self.parent.parent.parent.m_cloneState)?;
            serializer.skip_field("type", &self.parent.parent.parent.m_type)?;
            serializer.pad_field([0u8; 0usize].as_slice(), [0u8; 4usize].as_slice())?;
            serializer.skip_field("nodeInfo", &self.parent.parent.parent.m_nodeInfo)?;
            serializer.skip_field("partitionInfo", &self.parent.parent.m_partitionInfo)?;
            serializer.skip_field("syncInfo", &self.parent.parent.m_syncInfo)?;
            serializer
                .skip_fixed_array_field(
                    "pad",
                    self.parent.parent.m_pad.as_slice(),
                    TypeSize::NonPtr,
                )?;
            serializer.pad_field([0u8; 0usize].as_slice(), [0u8; 4usize].as_slice())?;
            serializer
                .serialize_field(
                    "selfTransitionMode",
                    &self.parent.m_selfTransitionMode,
                )?;
            serializer.serialize_field("eventMode", &self.parent.m_eventMode)?;
            serializer.skip_field("defaultEventMode", &self.parent.m_defaultEventMode)?;
            serializer.pad_field([0u8; 0usize].as_slice(), [0u8; 5usize].as_slice())?;
            serializer
                .skip_field("patchedBindingInfo", &self.parent.m_patchedBindingInfo)?;
            serializer.skip_field("fromGenerator", &self.parent.m_fromGenerator)?;
            serializer.skip_field("toGenerator", &self.parent.m_toGenerator)?;
            serializer.serialize_field("duration", &self.m_duration)?;
            serializer
                .serialize_field(
                    "toGeneratorStartTimeFraction",
                    &self.m_toGeneratorStartTimeFraction,
                )?;
            serializer.serialize_field("flags", &self.m_flags)?;
            serializer.serialize_field("endMode", &self.m_endMode)?;
            serializer.serialize_field("blendCurve", &self.m_blendCurve)?;
            serializer.serialize_field("alignmentBone", &self.m_alignmentBone)?;
            serializer.pad_field([0u8; 0usize].as_slice(), [0u8; 10usize].as_slice())?;
            serializer.skip_field("fromPos", &self.m_fromPos)?;
            serializer.skip_field("fromRot", &self.m_fromRot)?;
            serializer.skip_field("toPos", &self.m_toPos)?;
            serializer.skip_field("toRot", &self.m_toRot)?;
            serializer.skip_field("lastPos", &self.m_lastPos)?;
            serializer.skip_field("lastRot", &self.m_lastRot)?;
            serializer
                .skip_array_field(
                    "characterPoseAtBeginningOfTransition",
                    &self.m_characterPoseAtBeginningOfTransition,
                    TypeSize::NonPtr,
                )?;
            serializer.skip_field("timeRemaining", &self.m_timeRemaining)?;
            serializer.skip_field("timeInTransition", &self.m_timeInTransition)?;
            serializer
                .skip_field(
                    "toGeneratorSelfTranstitionMode",
                    &self.m_toGeneratorSelfTranstitionMode,
                )?;
            serializer
                .skip_field("initializeCharacterPose", &self.m_initializeCharacterPose)?;
            serializer.skip_field("alignThisFrame", &self.m_alignThisFrame)?;
            serializer.skip_field("alignmentFinished", &self.m_alignmentFinished)?;
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
    impl<'de> _serde::Deserialize<'de> for hkbBlendingTransitionEffect<'de> {
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
            struct __hkbBlendingTransitionEffectVisitor<'de> {
                marker: _serde::__private::PhantomData<hkbBlendingTransitionEffect<'de>>,
                lifetime: _serde::__private::PhantomData<&'de ()>,
            }
            #[allow(clippy::match_single_binding)]
            #[allow(clippy::reversed_empty_ranges)]
            #[allow(clippy::single_match)]
            impl<'de> _serde::de::Visitor<'de>
            for __hkbBlendingTransitionEffectVisitor<'de> {
                type Value = hkbBlendingTransitionEffect<'de>;
                fn expecting(
                    &self,
                    __formatter: &mut core::fmt::Formatter,
                ) -> core::fmt::Result {
                    core::fmt::Formatter::write_str(
                        __formatter,
                        "struct hkbBlendingTransitionEffect",
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
                    let mut m_duration: _serde::__private::Option<f32> = _serde::__private::None;
                    let mut m_toGeneratorStartTimeFraction: _serde::__private::Option<
                        f32,
                    > = _serde::__private::None;
                    let mut m_flags: _serde::__private::Option<FlagBits> = _serde::__private::None;
                    let mut m_endMode: _serde::__private::Option<EndMode> = _serde::__private::None;
                    let mut m_blendCurve: _serde::__private::Option<BlendCurve> = _serde::__private::None;
                    let mut m_alignmentBone: _serde::__private::Option<I16<'de>> = _serde::__private::None;
                    let mut m_fromPos: _serde::__private::Option<Vector4> = _serde::__private::None;
                    let mut m_fromRot: _serde::__private::Option<Quaternion> = _serde::__private::None;
                    let mut m_toPos: _serde::__private::Option<Vector4> = _serde::__private::None;
                    let mut m_toRot: _serde::__private::Option<Quaternion> = _serde::__private::None;
                    let mut m_lastPos: _serde::__private::Option<Vector4> = _serde::__private::None;
                    let mut m_lastRot: _serde::__private::Option<Quaternion> = _serde::__private::None;
                    let mut m_characterPoseAtBeginningOfTransition: _serde::__private::Option<
                        Vec<()>,
                    > = _serde::__private::None;
                    let mut m_timeRemaining: _serde::__private::Option<f32> = _serde::__private::None;
                    let mut m_timeInTransition: _serde::__private::Option<f32> = _serde::__private::None;
                    let mut m_toGeneratorSelfTranstitionMode: _serde::__private::Option<
                        I8<'de>,
                    > = _serde::__private::None;
                    let mut m_initializeCharacterPose: _serde::__private::Option<bool> = _serde::__private::None;
                    let mut m_alignThisFrame: _serde::__private::Option<bool> = _serde::__private::None;
                    let mut m_alignmentFinished: _serde::__private::Option<bool> = _serde::__private::None;
                    for i in 0..19usize {
                        match i {
                            0usize => {
                                if _serde::__private::Option::is_some(&m_duration) {
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
                            1usize => {
                                if _serde::__private::Option::is_some(
                                    &m_toGeneratorStartTimeFraction,
                                ) {
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
                            2usize => {
                                if _serde::__private::Option::is_some(&m_flags) {
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
                            3usize => {
                                if _serde::__private::Option::is_some(&m_endMode) {
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
                            4usize => {
                                if _serde::__private::Option::is_some(&m_blendCurve) {
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
                            5usize => {
                                if _serde::__private::Option::is_some(&m_alignmentBone) {
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
                            6usize => {
                                if _serde::__private::Option::is_some(&m_fromPos) {
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "fromPos",
                                        ),
                                    );
                                }
                                __A::pad(&mut __map, 0usize, 10usize)?;
                                m_fromPos = _serde::__private::Some(
                                    match __A::next_value::<Vector4>(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            7usize => {
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
                            8usize => {
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
                            9usize => {
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
                            10usize => {
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
                            11usize => {
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
                            12usize => {
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
                                    match __A::next_value::<Vec<()>>(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            13usize => {
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
                            14usize => {
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
                            15usize => {
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
                                    match __A::next_value::<I8<'de>>(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            16usize => {
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
                            17usize => {
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
                            18usize => {
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
                    let m_duration = match m_duration {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field("duration"),
                            );
                        }
                    };
                    let m_toGeneratorStartTimeFraction = match m_toGeneratorStartTimeFraction {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "toGeneratorStartTimeFraction",
                                ),
                            );
                        }
                    };
                    let m_flags = match m_flags {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field("flags"),
                            );
                        }
                    };
                    let m_endMode = match m_endMode {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field("endMode"),
                            );
                        }
                    };
                    let m_blendCurve = match m_blendCurve {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "blendCurve",
                                ),
                            );
                        }
                    };
                    let m_alignmentBone = match m_alignmentBone {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "alignmentBone",
                                ),
                            );
                        }
                    };
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
                    _serde::__private::Ok(hkbBlendingTransitionEffect {
                        __ptr,
                        parent,
                        m_duration,
                        m_toGeneratorStartTimeFraction,
                        m_flags,
                        m_endMode,
                        m_blendCurve,
                        m_alignmentBone,
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
                    let __ptr = __A::class_ptr(&mut __map);
                    _serde::__private::Ok(hkbBlendingTransitionEffect {
                        __ptr: __ptr.clone(),
                        parent,
                        m_duration,
                        m_toGeneratorStartTimeFraction,
                        m_flags,
                        m_endMode,
                        m_blendCurve,
                        m_alignmentBone,
                        ..Default::default()
                    })
                }
            }
            const FIELDS: &[&str] = &[
                "duration",
                "toGeneratorStartTimeFraction",
                "flags",
                "endMode",
                "blendCurve",
                "alignmentBone",
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
                "hkbBlendingTransitionEffect",
                FIELDS,
                __hkbBlendingTransitionEffectVisitor {
                    marker: _serde::__private::PhantomData::<
                        hkbBlendingTransitionEffect,
                    >,
                    lifetime: _serde::__private::PhantomData,
                },
            )
        }
    }
};
#[havok_types_derive::impl_flags_methods]
bitflags::bitflags! {
    #[doc = r" Bit flags"] #[doc = r""] #[doc = r" # C++ Info"] #[doc =
    " - name: `FlagBits`(ctype: `hkFlags<FlagBits, hkUint16>`)"]
    #[allow(non_upper_case_globals, non_snake_case)] #[cfg_attr(feature = "serde",
    derive(serde_with::SerializeDisplay, serde_with::DeserializeFromStr))]
    #[repr(transparent)] #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)] pub struct
    FlagBits : u16 { #[doc = "0"] const FLAG_NONE = 0u16; #[doc = "1"] const
    FLAG_IGNORE_FROM_WORLD_FROM_MODEL = 1u16; #[doc = "2"] const FLAG_SYNC = 2u16; #[doc
    = "4"] const FLAG_IGNORE_TO_WORLD_FROM_MODEL = 4u16; #[doc = "8"] const
    FLAG_IGNORE_TO_WORLD_FROM_MODEL_ROTATION = 8u16; }
}
#[cfg(feature = "json_schema")]
const _: () = {
    use schemars::{SchemaGenerator, Schema, JsonSchema, json_schema};
    use std::borrow::Cow;
    impl JsonSchema for FlagBits {
        fn schema_name() -> Cow<'static, str> {
            "FlagBits".into()
        }
        fn schema_id() -> Cow<'static, str> {
            concat!(module_path!(), "::", "FlagBits").into()
        }
        fn json_schema(_generate: &mut SchemaGenerator) -> Schema {
            json_schema!(
                { "description" :
                "Bitflags field. Specific flags: FLAG_NONE: 0, FLAG_IGNORE_FROM_WORLD_FROM_MODEL: 1, FLAG_SYNC: 2, FLAG_IGNORE_TO_WORLD_FROM_MODEL: 4, FLAG_IGNORE_TO_WORLD_FROM_MODEL_ROTATION: 8. Additional unspecified bits may be set.(e.g.: BIT_FLAG|BIT_FLAG2|4)",
                "type" : "string", }
            )
        }
    }
};
/// # C++ Info
/// - name: `EndMode`(ctype: `hkEnum<EndMode, hkInt8>`)
#[allow(non_upper_case_globals, non_snake_case)]
#[cfg_attr(feature = "json_schema", derive(schemars::JsonSchema))]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(
    Debug,
    Clone,
    Default,
    PartialEq,
    Eq,
    PartialOrd,
    Ord,
    num_derive::ToPrimitive,
    num_derive::FromPrimitive,
)]
pub enum EndMode {
    #[default]
    END_MODE_NONE = 0isize,
    END_MODE_TRANSITION_UNTIL_END_OF_FROM_GENERATOR = 1isize,
    END_MODE_CAP_DURATION_AT_END_OF_FROM_GENERATOR = 2isize,
}
const _: () = {
    use havok_serde as __serde;
    impl __serde::Serialize for FlagBits {
        fn serialize<S>(&self, __serializer: S) -> Result<S::Ok, S::Error>
        where
            S: __serde::ser::Serializer,
        {
            let mut __serializer = __serializer.serialize_enum_flags()?;
            if self.is_empty() {
                __serializer.serialize_bits(&self.bits())?;
                __serializer.serialize_empty_bit()?;
                return __serializer.end();
            }
            for flag in self.iter() {
                match flag {
                    Self::FLAG_NONE => {
                        __serializer.serialize_field("FLAG_NONE", &Self::FLAG_NONE)
                    }
                    Self::FLAG_IGNORE_FROM_WORLD_FROM_MODEL => {
                        __serializer
                            .serialize_field(
                                "FLAG_IGNORE_FROM_WORLD_FROM_MODEL",
                                &Self::FLAG_IGNORE_FROM_WORLD_FROM_MODEL,
                            )
                    }
                    Self::FLAG_SYNC => {
                        __serializer.serialize_field("FLAG_SYNC", &Self::FLAG_SYNC)
                    }
                    Self::FLAG_IGNORE_TO_WORLD_FROM_MODEL => {
                        __serializer
                            .serialize_field(
                                "FLAG_IGNORE_TO_WORLD_FROM_MODEL",
                                &Self::FLAG_IGNORE_TO_WORLD_FROM_MODEL,
                            )
                    }
                    Self::FLAG_IGNORE_TO_WORLD_FROM_MODEL_ROTATION => {
                        __serializer
                            .serialize_field(
                                "FLAG_IGNORE_TO_WORLD_FROM_MODEL_ROTATION",
                                &Self::FLAG_IGNORE_TO_WORLD_FROM_MODEL_ROTATION,
                            )
                    }
                    remain => {
                        __serializer
                            .serialize_field(&remain.bits().to_string(), &remain.bits())
                    }
                }?;
            }
            __serializer.serialize_bits(&self.bits())?;
            __serializer.end()
        }
    }
};
const _: () = {
    use havok_serde as __serde;
    impl __serde::Serialize for EndMode {
        fn serialize<S>(&self, __serializer: S) -> Result<S::Ok, S::Error>
        where
            S: __serde::ser::Serializer,
        {
            let mut __serializer = __serializer.serialize_enum_flags()?;
            match self {
                Self::END_MODE_NONE => {
                    __serializer.serialize_field("END_MODE_NONE", &0u64)
                }
                Self::END_MODE_TRANSITION_UNTIL_END_OF_FROM_GENERATOR => {
                    __serializer
                        .serialize_field(
                            "END_MODE_TRANSITION_UNTIL_END_OF_FROM_GENERATOR",
                            &1u64,
                        )
                }
                Self::END_MODE_CAP_DURATION_AT_END_OF_FROM_GENERATOR => {
                    __serializer
                        .serialize_field(
                            "END_MODE_CAP_DURATION_AT_END_OF_FROM_GENERATOR",
                            &2u64,
                        )
                }
            }?;
            use num_traits::ToPrimitive as _;
            let num = self.to_i8().ok_or(S::Error::custom("Failed enum EndMode to_i8"))?;
            __serializer.serialize_bits(&num)?;
            __serializer.end()
        }
    }
};
#[doc(hidden)]
#[allow(non_upper_case_globals, unused_attributes, unused_qualifications)]
const _: () = {
    #[allow(unused_extern_crates, clippy::useless_attribute)]
    extern crate havok_serde as _serde;
    #[automatically_derived]
    impl<'de> _serde::Deserialize<'de> for FlagBits {
        fn deserialize<__D>(
            __deserializer: __D,
        ) -> _serde::__private::Result<Self, __D::Error>
        where
            __D: _serde::Deserializer<'de>,
        {
            #[doc(hidden)]
            struct __Visitor<'de> {
                marker: _serde::__private::PhantomData<FlagBits>,
                lifetime: _serde::__private::PhantomData<&'de ()>,
            }
            impl<'de> _serde::de::Visitor<'de> for __Visitor<'de> {
                type Value = FlagBits;
                fn expecting(
                    &self,
                    __formatter: &mut _serde::__private::Formatter,
                ) -> _serde::__private::fmt::Result {
                    _serde::__private::Formatter::write_str(
                        __formatter,
                        "struct FlagBits(flags)",
                    )
                }
                #[inline]
                fn visit_uint16<__E>(
                    self,
                    __value: U16<'de>,
                ) -> _serde::__private::Result<Self::Value, __E>
                where
                    __E: _serde::de::Error,
                {
                    match __value {
                        U16::Number(__value) => {
                            Ok(FlagBits::from_bits_retain(__value as _))
                        }
                        _ => {
                            Err(
                                _serde::de::Error::invalid_value(
                                    _serde::de::Unexpected::Uint16(__value as _),
                                    &"FlagBits(U16) Number",
                                ),
                            )
                        }
                    }
                }
                fn visit_stringptr<__E>(
                    self,
                    __value: StringPtr<'de>,
                ) -> _serde::__private::Result<Self::Value, __E>
                where
                    __E: _serde::de::Error,
                {
                    match <FlagBits as core::str::FromStr>::from_str(
                        __value.into_inner().unwrap().as_ref(),
                    ) {
                        Ok(flags) => Ok(flags),
                        Err(err) => Err(_serde::de::Error::custom(err)),
                    }
                }
            }
            _serde::Deserializer::deserialize_flags(
                __deserializer,
                _serde::de::ReadEnumSize::Uint16,
                __Visitor {
                    marker: _serde::__private::PhantomData::<FlagBits>,
                    lifetime: _serde::__private::PhantomData,
                },
            )
        }
    }
};
#[doc(hidden)]
#[allow(non_upper_case_globals, unused_attributes, unused_qualifications)]
const _: () = {
    #[allow(unused_extern_crates, clippy::useless_attribute)]
    extern crate havok_serde as _serde;
    #[automatically_derived]
    impl<'de> _serde::Deserialize<'de> for EndMode {
        fn deserialize<__D>(
            __deserializer: __D,
        ) -> _serde::__private::Result<Self, __D::Error>
        where
            __D: _serde::Deserializer<'de>,
        {
            #[allow(non_camel_case_types)]
            #[doc(hidden)]
            enum __Field {
                __field0,
                __field1,
                __field2,
            }
            #[doc(hidden)]
            struct __FieldVisitor;
            impl<'de> _serde::de::Visitor<'de> for __FieldVisitor {
                type Value = __Field;
                fn expecting(
                    &self,
                    __formatter: &mut _serde::__private::Formatter,
                ) -> _serde::__private::fmt::Result {
                    _serde::__private::Formatter::write_str(
                        __formatter,
                        "variant identifier",
                    )
                }
                fn visit_int8<__E>(
                    self,
                    __value: I8<'de>,
                ) -> _serde::__private::Result<Self::Value, __E>
                where
                    __E: _serde::de::Error,
                {
                    match __value {
                        I8::Number(0i8) => _serde::__private::Ok(__Field::__field0),
                        I8::Number(1i8) => _serde::__private::Ok(__Field::__field1),
                        I8::Number(2i8) => _serde::__private::Ok(__Field::__field2),
                        _ => {
                            _serde::__private::Err(
                                _serde::de::Error::invalid_value(
                                    _serde::de::Unexpected::Int8(__value),
                                    &"value(i8) of variant is one of 0, 1, 2",
                                ),
                            )
                        }
                    }
                }
                fn visit_stringptr<__E>(
                    self,
                    __value: StringPtr<'de>,
                ) -> _serde::__private::Result<Self::Value, __E>
                where
                    __E: _serde::de::Error,
                {
                    if let Some(__value) = __value.into_inner() {
                        match __value.as_ref() {
                            v if v == "0" || v.eq_ignore_ascii_case("END_MODE_NONE") => {
                                _serde::__private::Ok(__Field::__field0)
                            }
                            v if v == "1"
                                || v
                                    .eq_ignore_ascii_case(
                                        "END_MODE_TRANSITION_UNTIL_END_OF_FROM_GENERATOR",
                                    ) => _serde::__private::Ok(__Field::__field1),
                            v if v == "2"
                                || v
                                    .eq_ignore_ascii_case(
                                        "END_MODE_CAP_DURATION_AT_END_OF_FROM_GENERATOR",
                                    ) => _serde::__private::Ok(__Field::__field2),
                            _ => {
                                _serde::__private::Err(
                                    _serde::de::Error::unknown_variant(&__value, VARIANTS),
                                )
                            }
                        }
                    } else {
                        _serde::__private::Err(
                            _serde::de::Error::unknown_variant("None", VARIANTS),
                        )
                    }
                }
            }
            impl<'de> _serde::Deserialize<'de> for __Field {
                #[inline]
                fn deserialize<__D>(
                    __deserializer: __D,
                ) -> _serde::__private::Result<Self, __D::Error>
                where
                    __D: _serde::Deserializer<'de>,
                {
                    _serde::Deserializer::deserialize_identifier(
                        __deserializer,
                        _serde::de::ReadEnumSize::Int8,
                        __FieldVisitor,
                    )
                }
            }
            #[doc(hidden)]
            struct __Visitor<'de> {
                marker: _serde::__private::PhantomData<EndMode>,
                lifetime: _serde::__private::PhantomData<&'de ()>,
            }
            impl<'de> _serde::de::Visitor<'de> for __Visitor<'de> {
                type Value = EndMode;
                fn expecting(
                    &self,
                    __formatter: &mut _serde::__private::Formatter,
                ) -> _serde::__private::fmt::Result {
                    _serde::__private::Formatter::write_str(__formatter, "enum EndMode")
                }
                fn visit_enum<__A>(
                    self,
                    __data: __A,
                ) -> _serde::__private::Result<Self::Value, __A::Error>
                where
                    __A: _serde::de::EnumAccess<'de>,
                {
                    match _serde::de::EnumAccess::variant(__data)? {
                        (__Field::__field0, __variant) => {
                            _serde::de::VariantAccess::unit_variant(__variant)?;
                            _serde::__private::Ok(EndMode::END_MODE_NONE)
                        }
                        (__Field::__field1, __variant) => {
                            _serde::de::VariantAccess::unit_variant(__variant)?;
                            _serde::__private::Ok(
                                EndMode::END_MODE_TRANSITION_UNTIL_END_OF_FROM_GENERATOR,
                            )
                        }
                        (__Field::__field2, __variant) => {
                            _serde::de::VariantAccess::unit_variant(__variant)?;
                            _serde::__private::Ok(
                                EndMode::END_MODE_CAP_DURATION_AT_END_OF_FROM_GENERATOR,
                            )
                        }
                    }
                }
            }
            #[doc(hidden)]
            const VARIANTS: &'static [&'static str] = &[
                "END_MODE_NONE",
                "END_MODE_TRANSITION_UNTIL_END_OF_FROM_GENERATOR",
                "END_MODE_CAP_DURATION_AT_END_OF_FROM_GENERATOR",
            ];
            _serde::Deserializer::deserialize_enum(
                __deserializer,
                "EndMode",
                VARIANTS,
                __Visitor {
                    marker: _serde::__private::PhantomData::<EndMode>,
                    lifetime: _serde::__private::PhantomData,
                },
            )
        }
    }
};
