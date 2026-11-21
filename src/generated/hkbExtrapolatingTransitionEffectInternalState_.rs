use super::class_requires::*;
use super::*;
/// # C++ Info
/// - name: `hkbExtrapolatingTransitionEffectInternalState`
/// - version: `1`
/// - signature: `0x62b05fff`
/// - size: `  0`(x86)/`384`(x86_64)
/// -  vtable: `true`
#[allow(non_upper_case_globals, non_snake_case)]
#[cfg_attr(feature = "json_schema", derive(schemars::JsonSchema))]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(educe::Educe)]
#[educe(Debug, Clone, Default, PartialEq)]
pub struct hkbExtrapolatingTransitionEffectInternalState<'a> {
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
    /// - name: `fromGeneratorSyncInfo`(ctype: `struct hkbGeneratorSyncInfo`)
    /// - offset: `  0`(x86)/` 16`(x86_64)
    /// - type_size: `  0`(x86)/`168`(x86_64)
    #[cfg_attr(feature = "serde_default", serde(default))]
    #[cfg_attr(feature = "serde", serde(borrow))]
    #[cfg_attr(feature = "json_schema", schemars(rename = "fromGeneratorSyncInfo"))]
    #[cfg_attr(feature = "serde", serde(rename = "fromGeneratorSyncInfo"))]
    pub m_fromGeneratorSyncInfo: hkbGeneratorSyncInfo<'a>,
    /// # C++ Info
    /// - name: `fromGeneratorPartitionInfo`(ctype: `struct hkbGeneratorPartitionInfo`)
    /// - offset: `  0`(x86)/`184`(x86_64)
    /// - type_size: `  0`(x86)/` 40`(x86_64)
    #[cfg_attr(feature = "serde_default", serde(default))]
    #[cfg_attr(feature = "serde", serde(borrow))]
    #[cfg_attr(feature = "json_schema", schemars(rename = "fromGeneratorPartitionInfo"))]
    #[cfg_attr(feature = "serde", serde(rename = "fromGeneratorPartitionInfo"))]
    pub m_fromGeneratorPartitionInfo: hkbGeneratorPartitionInfo<'a>,
    /// # C++ Info
    /// - name: `worldFromModel`(ctype: `hkQsTransform`)
    /// - offset: `  0`(x86)/`224`(x86_64)
    /// - type_size: `  0`(x86)/` 48`(x86_64)
    #[cfg_attr(feature = "serde_default", serde(default))]
    #[cfg_attr(feature = "json_schema", schemars(rename = "worldFromModel"))]
    #[cfg_attr(feature = "serde", serde(rename = "worldFromModel"))]
    pub m_worldFromModel: QsTransform,
    /// # C++ Info
    /// - name: `motion`(ctype: `hkQsTransform`)
    /// - offset: `  0`(x86)/`272`(x86_64)
    /// - type_size: `  0`(x86)/` 48`(x86_64)
    #[cfg_attr(feature = "serde_default", serde(default))]
    #[cfg_attr(feature = "json_schema", schemars(rename = "motion"))]
    #[cfg_attr(feature = "serde", serde(rename = "motion"))]
    pub m_motion: QsTransform,
    /// # C++ Info
    /// - name: `pose`(ctype: `hkArray<hkQsTransform>`)
    /// - offset: `  0`(x86)/`320`(x86_64)
    /// - type_size: `  0`(x86)/` 16`(x86_64)
    #[cfg_attr(feature = "serde_default", serde(default))]
    #[cfg_attr(feature = "json_schema", schemars(rename = "pose"))]
    #[cfg_attr(feature = "serde", serde(rename = "pose"))]
    pub m_pose: Vec<QsTransform>,
    /// # C++ Info
    /// - name: `additivePose`(ctype: `hkArray<hkQsTransform>`)
    /// - offset: `  0`(x86)/`336`(x86_64)
    /// - type_size: `  0`(x86)/` 16`(x86_64)
    #[cfg_attr(feature = "serde_default", serde(default))]
    #[cfg_attr(feature = "json_schema", schemars(rename = "additivePose"))]
    #[cfg_attr(feature = "serde", serde(rename = "additivePose"))]
    pub m_additivePose: Vec<QsTransform>,
    /// # C++ Info
    /// - name: `boneWeights`(ctype: `hkArray<hkReal>`)
    /// - offset: `  0`(x86)/`352`(x86_64)
    /// - type_size: `  0`(x86)/` 16`(x86_64)
    #[cfg_attr(feature = "serde_default", serde(default))]
    #[cfg_attr(feature = "json_schema", schemars(rename = "boneWeights"))]
    #[cfg_attr(feature = "serde", serde(rename = "boneWeights"))]
    pub m_boneWeights: Vec<f32>,
    /// # C++ Info
    /// - name: `toGeneratorDuration`(ctype: `hkReal`)
    /// - offset: `  0`(x86)/`368`(x86_64)
    /// - type_size: `  0`(x86)/`  4`(x86_64)
    #[cfg_attr(feature = "serde_default", serde(default))]
    #[cfg_attr(feature = "json_schema", schemars(rename = "toGeneratorDuration"))]
    #[cfg_attr(feature = "serde", serde(rename = "toGeneratorDuration"))]
    pub m_toGeneratorDuration: f32,
    /// # C++ Info
    /// - name: `isFromGeneratorActive`(ctype: `hkBool`)
    /// - offset: `  0`(x86)/`372`(x86_64)
    /// - type_size: `  0`(x86)/`  1`(x86_64)
    #[cfg_attr(feature = "serde_default", serde(default))]
    #[cfg_attr(feature = "json_schema", schemars(rename = "isFromGeneratorActive"))]
    #[cfg_attr(feature = "serde", serde(rename = "isFromGeneratorActive"))]
    pub m_isFromGeneratorActive: bool,
    /// # C++ Info
    /// - name: `gotPose`(ctype: `hkBool`)
    /// - offset: `  0`(x86)/`373`(x86_64)
    /// - type_size: `  0`(x86)/`  1`(x86_64)
    #[cfg_attr(feature = "serde_default", serde(default))]
    #[cfg_attr(feature = "json_schema", schemars(rename = "gotPose"))]
    #[cfg_attr(feature = "serde", serde(rename = "gotPose"))]
    pub m_gotPose: bool,
    /// # C++ Info
    /// - name: `gotAdditivePose`(ctype: `hkBool`)
    /// - offset: `  0`(x86)/`374`(x86_64)
    /// - type_size: `  0`(x86)/`  1`(x86_64)
    #[cfg_attr(feature = "serde_default", serde(default))]
    #[cfg_attr(feature = "json_schema", schemars(rename = "gotAdditivePose"))]
    #[cfg_attr(feature = "serde", serde(rename = "gotAdditivePose"))]
    pub m_gotAdditivePose: bool,
}
const _: () = {
    use havok_serde as _serde;
    impl<'a> _serde::HavokClass for hkbExtrapolatingTransitionEffectInternalState<'a> {
        #[inline]
        fn name(&self) -> &'static str {
            "hkbExtrapolatingTransitionEffectInternalState"
        }
        #[inline]
        fn signature(&self) -> _serde::__private::Signature {
            _serde::__private::Signature::new(0x62b05fff)
        }
        #[allow(clippy::let_and_return, clippy::vec_init_then_push)]
        fn deps_indexes(&self) -> Vec<&Pointer<'_>> {
            let mut v = Vec::new();
            v.extend(self.m_fromGeneratorSyncInfo.deps_indexes());
            v.extend(self.m_fromGeneratorPartitionInfo.deps_indexes());
            v
        }
    }
    impl<'a> _serde::Serialize for hkbExtrapolatingTransitionEffectInternalState<'a> {
        fn serialize<S>(&self, __serializer: S) -> Result<S::Ok, S::Error>
        where
            S: _serde::ser::Serializer,
        {
            let class_meta = self
                .__ptr
                .as_ref()
                .map(|name| (name, _serde::__private::Signature::new(0x62b05fff)));
            let mut serializer = __serializer
                .serialize_struct(
                    "hkbExtrapolatingTransitionEffectInternalState",
                    class_meta,
                    (0u64, 384u64),
                )?;
            serializer.pad_field([0u8; 0usize].as_slice(), [0u8; 8usize].as_slice())?;
            serializer
                .skip_field("memSizeAndRefCount", &self.parent.m_memSizeAndRefCount)?;
            serializer.pad_field([0u8; 0usize].as_slice(), [0u8; 4usize].as_slice())?;
            serializer
                .serialize_field(
                    "fromGeneratorSyncInfo",
                    &self.m_fromGeneratorSyncInfo,
                )?;
            serializer
                .serialize_field(
                    "fromGeneratorPartitionInfo",
                    &self.m_fromGeneratorPartitionInfo,
                )?;
            serializer.serialize_field("worldFromModel", &self.m_worldFromModel)?;
            serializer.serialize_field("motion", &self.m_motion)?;
            serializer.serialize_array_field("pose", &self.m_pose, TypeSize::NonPtr)?;
            serializer
                .serialize_array_field(
                    "additivePose",
                    &self.m_additivePose,
                    TypeSize::NonPtr,
                )?;
            serializer
                .serialize_array_field(
                    "boneWeights",
                    &self.m_boneWeights,
                    TypeSize::NonPtr,
                )?;
            serializer
                .serialize_field("toGeneratorDuration", &self.m_toGeneratorDuration)?;
            serializer
                .serialize_field(
                    "isFromGeneratorActive",
                    &self.m_isFromGeneratorActive,
                )?;
            serializer.serialize_field("gotPose", &self.m_gotPose)?;
            serializer.serialize_field("gotAdditivePose", &self.m_gotAdditivePose)?;
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
    impl<'de> _serde::Deserialize<'de>
    for hkbExtrapolatingTransitionEffectInternalState<'de> {
        fn deserialize<__D>(deserializer: __D) -> core::result::Result<Self, __D::Error>
        where
            __D: _serde::Deserializer<'de>,
        {
            #[allow(non_camel_case_types)]
            enum __Field {
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
                        "fromGeneratorSyncInfo" => Ok(__Field::m_fromGeneratorSyncInfo),
                        "fromGeneratorPartitionInfo" => {
                            Ok(__Field::m_fromGeneratorPartitionInfo)
                        }
                        "worldFromModel" => Ok(__Field::m_worldFromModel),
                        "motion" => Ok(__Field::m_motion),
                        "pose" => Ok(__Field::m_pose),
                        "additivePose" => Ok(__Field::m_additivePose),
                        "boneWeights" => Ok(__Field::m_boneWeights),
                        "toGeneratorDuration" => Ok(__Field::m_toGeneratorDuration),
                        "isFromGeneratorActive" => Ok(__Field::m_isFromGeneratorActive),
                        "gotPose" => Ok(__Field::m_gotPose),
                        "gotAdditivePose" => Ok(__Field::m_gotAdditivePose),
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
            struct __hkbExtrapolatingTransitionEffectInternalStateVisitor<'de> {
                marker: _serde::__private::PhantomData<
                    hkbExtrapolatingTransitionEffectInternalState<'de>,
                >,
                lifetime: _serde::__private::PhantomData<&'de ()>,
            }
            #[allow(clippy::match_single_binding)]
            #[allow(clippy::reversed_empty_ranges)]
            #[allow(clippy::single_match)]
            impl<'de> _serde::de::Visitor<'de>
            for __hkbExtrapolatingTransitionEffectInternalStateVisitor<'de> {
                type Value = hkbExtrapolatingTransitionEffectInternalState<'de>;
                fn expecting(
                    &self,
                    __formatter: &mut core::fmt::Formatter,
                ) -> core::fmt::Result {
                    core::fmt::Formatter::write_str(
                        __formatter,
                        "struct hkbExtrapolatingTransitionEffectInternalState",
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
                    let mut m_pose: _serde::__private::Option<Vec<QsTransform>> = _serde::__private::None;
                    let mut m_additivePose: _serde::__private::Option<
                        Vec<QsTransform>,
                    > = _serde::__private::None;
                    let mut m_boneWeights: _serde::__private::Option<Vec<f32>> = _serde::__private::None;
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
                                    match __A::next_value::<Vec<QsTransform>>(&mut __map) {
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
                                    match __A::next_value::<Vec<QsTransform>>(&mut __map) {
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
                                    match __A::next_value::<Vec<f32>>(&mut __map) {
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
                    _serde::__private::Ok(hkbExtrapolatingTransitionEffectInternalState {
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
                    let mut m_fromGeneratorSyncInfo: _serde::__private::Option<
                        hkbGeneratorSyncInfo<'de>,
                    > = _serde::__private::None;
                    let mut m_fromGeneratorPartitionInfo: _serde::__private::Option<
                        hkbGeneratorPartitionInfo<'de>,
                    > = _serde::__private::None;
                    let mut m_worldFromModel: _serde::__private::Option<QsTransform> = _serde::__private::None;
                    let mut m_motion: _serde::__private::Option<QsTransform> = _serde::__private::None;
                    let mut m_pose: _serde::__private::Option<Vec<QsTransform>> = _serde::__private::None;
                    let mut m_additivePose: _serde::__private::Option<
                        Vec<QsTransform>,
                    > = _serde::__private::None;
                    let mut m_boneWeights: _serde::__private::Option<Vec<f32>> = _serde::__private::None;
                    let mut m_toGeneratorDuration: _serde::__private::Option<f32> = _serde::__private::None;
                    let mut m_isFromGeneratorActive: _serde::__private::Option<bool> = _serde::__private::None;
                    let mut m_gotPose: _serde::__private::Option<bool> = _serde::__private::None;
                    let mut m_gotAdditivePose: _serde::__private::Option<bool> = _serde::__private::None;
                    while let _serde::__private::Some(__key) = {
                        __A::next_key::<__Field>(&mut __map)?
                    } {
                        match __key {
                            __Field::m_fromGeneratorSyncInfo => {
                                #[cfg(
                                    any(feature = "strict", feature = "ignore_duplicates")
                                )]
                                if _serde::__private::Option::is_some(
                                    &m_fromGeneratorSyncInfo,
                                ) {
                                    #[cfg(feature = "ignore_duplicates")]
                                    {
                                        __A::skip_value(&mut __map)?;
                                        continue;
                                    }
                                    #[cfg(feature = "strict")]
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
                            __Field::m_fromGeneratorPartitionInfo => {
                                #[cfg(
                                    any(feature = "strict", feature = "ignore_duplicates")
                                )]
                                if _serde::__private::Option::is_some(
                                    &m_fromGeneratorPartitionInfo,
                                ) {
                                    #[cfg(feature = "ignore_duplicates")]
                                    {
                                        __A::skip_value(&mut __map)?;
                                        continue;
                                    }
                                    #[cfg(feature = "strict")]
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
                            __Field::m_worldFromModel => {
                                #[cfg(
                                    any(feature = "strict", feature = "ignore_duplicates")
                                )]
                                if _serde::__private::Option::is_some(&m_worldFromModel) {
                                    #[cfg(feature = "ignore_duplicates")]
                                    {
                                        __A::skip_value(&mut __map)?;
                                        continue;
                                    }
                                    #[cfg(feature = "strict")]
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
                            __Field::m_motion => {
                                #[cfg(
                                    any(feature = "strict", feature = "ignore_duplicates")
                                )]
                                if _serde::__private::Option::is_some(&m_motion) {
                                    #[cfg(feature = "ignore_duplicates")]
                                    {
                                        __A::skip_value(&mut __map)?;
                                        continue;
                                    }
                                    #[cfg(feature = "strict")]
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
                            __Field::m_pose => {
                                #[cfg(
                                    any(feature = "strict", feature = "ignore_duplicates")
                                )]
                                if _serde::__private::Option::is_some(&m_pose) {
                                    #[cfg(feature = "ignore_duplicates")]
                                    {
                                        __A::skip_value(&mut __map)?;
                                        continue;
                                    }
                                    #[cfg(feature = "strict")]
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field("pose"),
                                    );
                                }
                                m_pose = _serde::__private::Some(
                                    match __A::next_value::<Vec<QsTransform>>(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            __Field::m_additivePose => {
                                #[cfg(
                                    any(feature = "strict", feature = "ignore_duplicates")
                                )]
                                if _serde::__private::Option::is_some(&m_additivePose) {
                                    #[cfg(feature = "ignore_duplicates")]
                                    {
                                        __A::skip_value(&mut __map)?;
                                        continue;
                                    }
                                    #[cfg(feature = "strict")]
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "additivePose",
                                        ),
                                    );
                                }
                                m_additivePose = _serde::__private::Some(
                                    match __A::next_value::<Vec<QsTransform>>(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            __Field::m_boneWeights => {
                                #[cfg(
                                    any(feature = "strict", feature = "ignore_duplicates")
                                )]
                                if _serde::__private::Option::is_some(&m_boneWeights) {
                                    #[cfg(feature = "ignore_duplicates")]
                                    {
                                        __A::skip_value(&mut __map)?;
                                        continue;
                                    }
                                    #[cfg(feature = "strict")]
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "boneWeights",
                                        ),
                                    );
                                }
                                m_boneWeights = _serde::__private::Some(
                                    match __A::next_value::<Vec<f32>>(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            __Field::m_toGeneratorDuration => {
                                #[cfg(
                                    any(feature = "strict", feature = "ignore_duplicates")
                                )]
                                if _serde::__private::Option::is_some(
                                    &m_toGeneratorDuration,
                                ) {
                                    #[cfg(feature = "ignore_duplicates")]
                                    {
                                        __A::skip_value(&mut __map)?;
                                        continue;
                                    }
                                    #[cfg(feature = "strict")]
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
                            __Field::m_isFromGeneratorActive => {
                                #[cfg(
                                    any(feature = "strict", feature = "ignore_duplicates")
                                )]
                                if _serde::__private::Option::is_some(
                                    &m_isFromGeneratorActive,
                                ) {
                                    #[cfg(feature = "ignore_duplicates")]
                                    {
                                        __A::skip_value(&mut __map)?;
                                        continue;
                                    }
                                    #[cfg(feature = "strict")]
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
                            __Field::m_gotPose => {
                                #[cfg(
                                    any(feature = "strict", feature = "ignore_duplicates")
                                )]
                                if _serde::__private::Option::is_some(&m_gotPose) {
                                    #[cfg(feature = "ignore_duplicates")]
                                    {
                                        __A::skip_value(&mut __map)?;
                                        continue;
                                    }
                                    #[cfg(feature = "strict")]
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
                            __Field::m_gotAdditivePose => {
                                #[cfg(
                                    any(feature = "strict", feature = "ignore_duplicates")
                                )]
                                if _serde::__private::Option::is_some(&m_gotAdditivePose) {
                                    #[cfg(feature = "ignore_duplicates")]
                                    {
                                        __A::skip_value(&mut __map)?;
                                        continue;
                                    }
                                    #[cfg(feature = "strict")]
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
                            _ => __A::skip_value(&mut __map)?,
                        }
                    }
                    let m_fromGeneratorSyncInfo = match m_fromGeneratorSyncInfo {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            #[cfg(feature = "strict")]
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "fromGeneratorSyncInfo",
                                ),
                            );
                            #[cfg(not(feature = "strict"))] Default::default()
                        }
                    };
                    let m_fromGeneratorPartitionInfo = match m_fromGeneratorPartitionInfo {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            #[cfg(feature = "strict")]
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "fromGeneratorPartitionInfo",
                                ),
                            );
                            #[cfg(not(feature = "strict"))] Default::default()
                        }
                    };
                    let m_worldFromModel = match m_worldFromModel {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            #[cfg(feature = "strict")]
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "worldFromModel",
                                ),
                            );
                            #[cfg(not(feature = "strict"))] Default::default()
                        }
                    };
                    let m_motion = match m_motion {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            #[cfg(feature = "strict")]
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field("motion"),
                            );
                            #[cfg(not(feature = "strict"))] Default::default()
                        }
                    };
                    let m_pose = match m_pose {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            #[cfg(feature = "strict")]
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field("pose"),
                            );
                            #[cfg(not(feature = "strict"))] Default::default()
                        }
                    };
                    let m_additivePose = match m_additivePose {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            #[cfg(feature = "strict")]
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "additivePose",
                                ),
                            );
                            #[cfg(not(feature = "strict"))] Default::default()
                        }
                    };
                    let m_boneWeights = match m_boneWeights {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            #[cfg(feature = "strict")]
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "boneWeights",
                                ),
                            );
                            #[cfg(not(feature = "strict"))] Default::default()
                        }
                    };
                    let m_toGeneratorDuration = match m_toGeneratorDuration {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            #[cfg(feature = "strict")]
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "toGeneratorDuration",
                                ),
                            );
                            #[cfg(not(feature = "strict"))] Default::default()
                        }
                    };
                    let m_isFromGeneratorActive = match m_isFromGeneratorActive {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            #[cfg(feature = "strict")]
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "isFromGeneratorActive",
                                ),
                            );
                            #[cfg(not(feature = "strict"))] Default::default()
                        }
                    };
                    let m_gotPose = match m_gotPose {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            #[cfg(feature = "strict")]
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field("gotPose"),
                            );
                            #[cfg(not(feature = "strict"))] Default::default()
                        }
                    };
                    let m_gotAdditivePose = match m_gotAdditivePose {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            #[cfg(feature = "strict")]
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "gotAdditivePose",
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
                    _serde::__private::Ok(hkbExtrapolatingTransitionEffectInternalState {
                        __ptr: __ptr.clone(),
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
                "hkbExtrapolatingTransitionEffectInternalState",
                FIELDS,
                __hkbExtrapolatingTransitionEffectInternalStateVisitor {
                    marker: _serde::__private::PhantomData::<
                        hkbExtrapolatingTransitionEffectInternalState,
                    >,
                    lifetime: _serde::__private::PhantomData,
                },
            )
        }
    }
};
