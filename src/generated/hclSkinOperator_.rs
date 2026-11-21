use super::class_requires::*;
use super::*;
/// # C++ Info
/// - name: `hclSkinOperator`
/// - version: `4`
/// - signature: `0xd1f2ea5e`
/// - size: `  0`(x86)/`120`(x86_64)
/// -  vtable: `true`
#[allow(non_upper_case_globals, non_snake_case)]
#[cfg_attr(feature = "json_schema", derive(schemars::JsonSchema))]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(educe::Educe)]
#[educe(Debug, Clone, Default, PartialEq)]
pub struct hclSkinOperator<'a> {
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
    pub parent: hclOperator<'a>,
    /// # C++ Info
    /// - name: `boneInfluences`(ctype: `hkArray<struct hclSkinOperatorBoneInfluence>`)
    /// - offset: `  0`(x86)/` 32`(x86_64)
    /// - type_size: `  0`(x86)/` 16`(x86_64)
    #[cfg_attr(feature = "serde_default", serde(default))]
    #[cfg_attr(feature = "serde", serde(borrow))]
    #[cfg_attr(feature = "json_schema", schemars(rename = "boneInfluences"))]
    #[cfg_attr(feature = "serde", serde(rename = "boneInfluences"))]
    pub m_boneInfluences: Vec<hclSkinOperatorBoneInfluence<'a>>,
    /// # C++ Info
    /// - name: `boneInfluenceStartPerVertex`(ctype: `hkArray<hkUint16>`)
    /// - offset: `  0`(x86)/` 48`(x86_64)
    /// - type_size: `  0`(x86)/` 16`(x86_64)
    #[cfg_attr(feature = "serde_default", serde(default))]
    #[cfg_attr(feature = "serde", serde(borrow))]
    #[cfg_attr(
        feature = "json_schema",
        schemars(rename = "boneInfluenceStartPerVertex")
    )]
    #[cfg_attr(feature = "serde", serde(rename = "boneInfluenceStartPerVertex"))]
    pub m_boneInfluenceStartPerVertex: Vec<U16<'a>>,
    /// # C++ Info
    /// - name: `boneFromSkinMeshTransforms`(ctype: `hkArray<hkMatrix4>`)
    /// - offset: `  0`(x86)/` 64`(x86_64)
    /// - type_size: `  0`(x86)/` 16`(x86_64)
    #[cfg_attr(feature = "serde_default", serde(default))]
    #[cfg_attr(feature = "json_schema", schemars(rename = "boneFromSkinMeshTransforms"))]
    #[cfg_attr(feature = "serde", serde(rename = "boneFromSkinMeshTransforms"))]
    pub m_boneFromSkinMeshTransforms: Vec<Matrix4>,
    /// # C++ Info
    /// - name: `usedBoneGroupIds`(ctype: `hkArray<hkUint16>`)
    /// - offset: `  0`(x86)/` 80`(x86_64)
    /// - type_size: `  0`(x86)/` 16`(x86_64)
    #[cfg_attr(feature = "serde_default", serde(default))]
    #[cfg_attr(feature = "serde", serde(borrow))]
    #[cfg_attr(feature = "json_schema", schemars(rename = "usedBoneGroupIds"))]
    #[cfg_attr(feature = "serde", serde(rename = "usedBoneGroupIds"))]
    pub m_usedBoneGroupIds: Vec<U16<'a>>,
    /// # C++ Info
    /// - name: `skinPositions`(ctype: `hkBool`)
    /// - offset: `  0`(x86)/` 96`(x86_64)
    /// - type_size: `  0`(x86)/`  1`(x86_64)
    #[cfg_attr(feature = "serde_default", serde(default))]
    #[cfg_attr(feature = "json_schema", schemars(rename = "skinPositions"))]
    #[cfg_attr(feature = "serde", serde(rename = "skinPositions"))]
    pub m_skinPositions: bool,
    /// # C++ Info
    /// - name: `skinNormals`(ctype: `hkBool`)
    /// - offset: `  0`(x86)/` 97`(x86_64)
    /// - type_size: `  0`(x86)/`  1`(x86_64)
    #[cfg_attr(feature = "serde_default", serde(default))]
    #[cfg_attr(feature = "json_schema", schemars(rename = "skinNormals"))]
    #[cfg_attr(feature = "serde", serde(rename = "skinNormals"))]
    pub m_skinNormals: bool,
    /// # C++ Info
    /// - name: `skinTangents`(ctype: `hkBool`)
    /// - offset: `  0`(x86)/` 98`(x86_64)
    /// - type_size: `  0`(x86)/`  1`(x86_64)
    #[cfg_attr(feature = "serde_default", serde(default))]
    #[cfg_attr(feature = "json_schema", schemars(rename = "skinTangents"))]
    #[cfg_attr(feature = "serde", serde(rename = "skinTangents"))]
    pub m_skinTangents: bool,
    /// # C++ Info
    /// - name: `skinBiTangents`(ctype: `hkBool`)
    /// - offset: `  0`(x86)/` 99`(x86_64)
    /// - type_size: `  0`(x86)/`  1`(x86_64)
    #[cfg_attr(feature = "serde_default", serde(default))]
    #[cfg_attr(feature = "json_schema", schemars(rename = "skinBiTangents"))]
    #[cfg_attr(feature = "serde", serde(rename = "skinBiTangents"))]
    pub m_skinBiTangents: bool,
    /// # C++ Info
    /// - name: `inputBufferIndex`(ctype: `hkUint32`)
    /// - offset: `  0`(x86)/`100`(x86_64)
    /// - type_size: `  0`(x86)/`  4`(x86_64)
    #[cfg_attr(feature = "serde_default", serde(default))]
    #[cfg_attr(feature = "serde", serde(borrow))]
    #[cfg_attr(feature = "json_schema", schemars(rename = "inputBufferIndex"))]
    #[cfg_attr(feature = "serde", serde(rename = "inputBufferIndex"))]
    pub m_inputBufferIndex: U32<'a>,
    /// # C++ Info
    /// - name: `outputBufferIndex`(ctype: `hkUint32`)
    /// - offset: `  0`(x86)/`104`(x86_64)
    /// - type_size: `  0`(x86)/`  4`(x86_64)
    #[cfg_attr(feature = "serde_default", serde(default))]
    #[cfg_attr(feature = "serde", serde(borrow))]
    #[cfg_attr(feature = "json_schema", schemars(rename = "outputBufferIndex"))]
    #[cfg_attr(feature = "serde", serde(rename = "outputBufferIndex"))]
    pub m_outputBufferIndex: U32<'a>,
    /// # C++ Info
    /// - name: `transformSetIndex`(ctype: `hkUint32`)
    /// - offset: `  0`(x86)/`108`(x86_64)
    /// - type_size: `  0`(x86)/`  4`(x86_64)
    #[cfg_attr(feature = "serde_default", serde(default))]
    #[cfg_attr(feature = "serde", serde(borrow))]
    #[cfg_attr(feature = "json_schema", schemars(rename = "transformSetIndex"))]
    #[cfg_attr(feature = "serde", serde(rename = "transformSetIndex"))]
    pub m_transformSetIndex: U32<'a>,
    /// # C++ Info
    /// - name: `startVertex`(ctype: `hkUint16`)
    /// - offset: `  0`(x86)/`112`(x86_64)
    /// - type_size: `  0`(x86)/`  2`(x86_64)
    #[cfg_attr(feature = "serde_default", serde(default))]
    #[cfg_attr(feature = "serde", serde(borrow))]
    #[cfg_attr(feature = "json_schema", schemars(rename = "startVertex"))]
    #[cfg_attr(feature = "serde", serde(rename = "startVertex"))]
    pub m_startVertex: U16<'a>,
    /// # C++ Info
    /// - name: `endVertex`(ctype: `hkUint16`)
    /// - offset: `  0`(x86)/`114`(x86_64)
    /// - type_size: `  0`(x86)/`  2`(x86_64)
    #[cfg_attr(feature = "serde_default", serde(default))]
    #[cfg_attr(feature = "serde", serde(borrow))]
    #[cfg_attr(feature = "json_schema", schemars(rename = "endVertex"))]
    #[cfg_attr(feature = "serde", serde(rename = "endVertex"))]
    pub m_endVertex: U16<'a>,
    /// # C++ Info
    /// - name: `partialSkinning`(ctype: `hkBool`)
    /// - offset: `  0`(x86)/`116`(x86_64)
    /// - type_size: `  0`(x86)/`  1`(x86_64)
    #[cfg_attr(feature = "serde_default", serde(default))]
    #[cfg_attr(feature = "json_schema", schemars(rename = "partialSkinning"))]
    #[cfg_attr(feature = "serde", serde(rename = "partialSkinning"))]
    pub m_partialSkinning: bool,
    /// # C++ Info
    /// - name: `dualQuaternionSkinning`(ctype: `hkBool`)
    /// - offset: `  0`(x86)/`117`(x86_64)
    /// - type_size: `  0`(x86)/`  1`(x86_64)
    #[cfg_attr(feature = "serde_default", serde(default))]
    #[cfg_attr(feature = "json_schema", schemars(rename = "dualQuaternionSkinning"))]
    #[cfg_attr(feature = "serde", serde(rename = "dualQuaternionSkinning"))]
    pub m_dualQuaternionSkinning: bool,
    /// # C++ Info
    /// - name: `boneGroupSize`(ctype: `hkUint8`)
    /// - offset: `  0`(x86)/`118`(x86_64)
    /// - type_size: `  0`(x86)/`  1`(x86_64)
    #[cfg_attr(feature = "serde_default", serde(default))]
    #[cfg_attr(feature = "serde", serde(borrow))]
    #[cfg_attr(feature = "json_schema", schemars(rename = "boneGroupSize"))]
    #[cfg_attr(feature = "serde", serde(rename = "boneGroupSize"))]
    pub m_boneGroupSize: U8<'a>,
}
const _: () = {
    use havok_serde as _serde;
    impl<'a> _serde::HavokClass for hclSkinOperator<'a> {
        #[inline]
        fn name(&self) -> &'static str {
            "hclSkinOperator"
        }
        #[inline]
        fn signature(&self) -> _serde::__private::Signature {
            _serde::__private::Signature::new(0xd1f2ea5e)
        }
        #[allow(clippy::let_and_return, clippy::vec_init_then_push)]
        fn deps_indexes(&self) -> Vec<&Pointer<'_>> {
            let mut v = Vec::new();
            v.extend(
                self
                    .m_boneInfluences
                    .iter()
                    .flat_map(|class| class.deps_indexes())
                    .collect::<Vec<&Pointer<'_>>>(),
            );
            v
        }
    }
    impl<'a> _serde::Serialize for hclSkinOperator<'a> {
        fn serialize<S>(&self, __serializer: S) -> Result<S::Ok, S::Error>
        where
            S: _serde::ser::Serializer,
        {
            let class_meta = self
                .__ptr
                .as_ref()
                .map(|name| (name, _serde::__private::Signature::new(0xd1f2ea5e)));
            let mut serializer = __serializer
                .serialize_struct("hclSkinOperator", class_meta, (0u64, 120u64))?;
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
                    "boneInfluences",
                    &self.m_boneInfluences,
                    TypeSize::Struct {
                        size_x86: 0u64,
                        size_x86_64: 2u64,
                    },
                )?;
            serializer
                .serialize_array_field(
                    "boneInfluenceStartPerVertex",
                    &self.m_boneInfluenceStartPerVertex,
                    TypeSize::NonPtr,
                )?;
            serializer
                .serialize_array_field(
                    "boneFromSkinMeshTransforms",
                    &self.m_boneFromSkinMeshTransforms,
                    TypeSize::NonPtr,
                )?;
            serializer
                .serialize_array_field(
                    "usedBoneGroupIds",
                    &self.m_usedBoneGroupIds,
                    TypeSize::NonPtr,
                )?;
            serializer.serialize_field("skinPositions", &self.m_skinPositions)?;
            serializer.serialize_field("skinNormals", &self.m_skinNormals)?;
            serializer.serialize_field("skinTangents", &self.m_skinTangents)?;
            serializer.serialize_field("skinBiTangents", &self.m_skinBiTangents)?;
            serializer.serialize_field("inputBufferIndex", &self.m_inputBufferIndex)?;
            serializer.serialize_field("outputBufferIndex", &self.m_outputBufferIndex)?;
            serializer.serialize_field("transformSetIndex", &self.m_transformSetIndex)?;
            serializer.serialize_field("startVertex", &self.m_startVertex)?;
            serializer.serialize_field("endVertex", &self.m_endVertex)?;
            serializer.serialize_field("partialSkinning", &self.m_partialSkinning)?;
            serializer
                .serialize_field(
                    "dualQuaternionSkinning",
                    &self.m_dualQuaternionSkinning,
                )?;
            serializer.serialize_field("boneGroupSize", &self.m_boneGroupSize)?;
            serializer.pad_field([0u8; 0usize].as_slice(), [0u8; 1usize].as_slice())?;
            serializer.end()
        }
    }
};
#[doc(hidden)]
#[allow(non_upper_case_globals, unused_attributes, unused_qualifications)]
const _: () = {
    use havok_serde as _serde;
    #[automatically_derived]
    impl<'de> _serde::Deserialize<'de> for hclSkinOperator<'de> {
        fn deserialize<__D>(deserializer: __D) -> core::result::Result<Self, __D::Error>
        where
            __D: _serde::Deserializer<'de>,
        {
            #[allow(non_camel_case_types)]
            enum __Field {
                m_name,
                m_boneInfluences,
                m_boneInfluenceStartPerVertex,
                m_boneFromSkinMeshTransforms,
                m_usedBoneGroupIds,
                m_skinPositions,
                m_skinNormals,
                m_skinTangents,
                m_skinBiTangents,
                m_inputBufferIndex,
                m_outputBufferIndex,
                m_transformSetIndex,
                m_startVertex,
                m_endVertex,
                m_partialSkinning,
                m_dualQuaternionSkinning,
                m_boneGroupSize,
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
                        "boneInfluences" => Ok(__Field::m_boneInfluences),
                        "boneInfluenceStartPerVertex" => {
                            Ok(__Field::m_boneInfluenceStartPerVertex)
                        }
                        "boneFromSkinMeshTransforms" => {
                            Ok(__Field::m_boneFromSkinMeshTransforms)
                        }
                        "usedBoneGroupIds" => Ok(__Field::m_usedBoneGroupIds),
                        "skinPositions" => Ok(__Field::m_skinPositions),
                        "skinNormals" => Ok(__Field::m_skinNormals),
                        "skinTangents" => Ok(__Field::m_skinTangents),
                        "skinBiTangents" => Ok(__Field::m_skinBiTangents),
                        "inputBufferIndex" => Ok(__Field::m_inputBufferIndex),
                        "outputBufferIndex" => Ok(__Field::m_outputBufferIndex),
                        "transformSetIndex" => Ok(__Field::m_transformSetIndex),
                        "startVertex" => Ok(__Field::m_startVertex),
                        "endVertex" => Ok(__Field::m_endVertex),
                        "partialSkinning" => Ok(__Field::m_partialSkinning),
                        "dualQuaternionSkinning" => Ok(__Field::m_dualQuaternionSkinning),
                        "boneGroupSize" => Ok(__Field::m_boneGroupSize),
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
            struct __hclSkinOperatorVisitor<'de> {
                marker: _serde::__private::PhantomData<hclSkinOperator<'de>>,
                lifetime: _serde::__private::PhantomData<&'de ()>,
            }
            #[allow(clippy::match_single_binding)]
            #[allow(clippy::reversed_empty_ranges)]
            #[allow(clippy::single_match)]
            impl<'de> _serde::de::Visitor<'de> for __hclSkinOperatorVisitor<'de> {
                type Value = hclSkinOperator<'de>;
                fn expecting(
                    &self,
                    __formatter: &mut core::fmt::Formatter,
                ) -> core::fmt::Result {
                    core::fmt::Formatter::write_str(
                        __formatter,
                        "struct hclSkinOperator",
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
                    let mut m_boneInfluences: _serde::__private::Option<
                        Vec<hclSkinOperatorBoneInfluence<'de>>,
                    > = _serde::__private::None;
                    let mut m_boneInfluenceStartPerVertex: _serde::__private::Option<
                        Vec<U16<'de>>,
                    > = _serde::__private::None;
                    let mut m_boneFromSkinMeshTransforms: _serde::__private::Option<
                        Vec<Matrix4>,
                    > = _serde::__private::None;
                    let mut m_usedBoneGroupIds: _serde::__private::Option<
                        Vec<U16<'de>>,
                    > = _serde::__private::None;
                    let mut m_skinPositions: _serde::__private::Option<bool> = _serde::__private::None;
                    let mut m_skinNormals: _serde::__private::Option<bool> = _serde::__private::None;
                    let mut m_skinTangents: _serde::__private::Option<bool> = _serde::__private::None;
                    let mut m_skinBiTangents: _serde::__private::Option<bool> = _serde::__private::None;
                    let mut m_inputBufferIndex: _serde::__private::Option<U32<'de>> = _serde::__private::None;
                    let mut m_outputBufferIndex: _serde::__private::Option<U32<'de>> = _serde::__private::None;
                    let mut m_transformSetIndex: _serde::__private::Option<U32<'de>> = _serde::__private::None;
                    let mut m_startVertex: _serde::__private::Option<U16<'de>> = _serde::__private::None;
                    let mut m_endVertex: _serde::__private::Option<U16<'de>> = _serde::__private::None;
                    let mut m_partialSkinning: _serde::__private::Option<bool> = _serde::__private::None;
                    let mut m_dualQuaternionSkinning: _serde::__private::Option<bool> = _serde::__private::None;
                    let mut m_boneGroupSize: _serde::__private::Option<U8<'de>> = _serde::__private::None;
                    for i in 0..16usize {
                        match i {
                            0usize => {
                                if _serde::__private::Option::is_some(&m_boneInfluences) {
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "boneInfluences",
                                        ),
                                    );
                                }
                                m_boneInfluences = _serde::__private::Some(
                                    match __A::next_value::<
                                        Vec<hclSkinOperatorBoneInfluence<'de>>,
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
                                    &m_boneInfluenceStartPerVertex,
                                ) {
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "boneInfluenceStartPerVertex",
                                        ),
                                    );
                                }
                                m_boneInfluenceStartPerVertex = _serde::__private::Some(
                                    match __A::next_value::<Vec<U16<'de>>>(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            2usize => {
                                if _serde::__private::Option::is_some(
                                    &m_boneFromSkinMeshTransforms,
                                ) {
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "boneFromSkinMeshTransforms",
                                        ),
                                    );
                                }
                                m_boneFromSkinMeshTransforms = _serde::__private::Some(
                                    match __A::next_value::<Vec<Matrix4>>(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            3usize => {
                                if _serde::__private::Option::is_some(&m_usedBoneGroupIds) {
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "usedBoneGroupIds",
                                        ),
                                    );
                                }
                                m_usedBoneGroupIds = _serde::__private::Some(
                                    match __A::next_value::<Vec<U16<'de>>>(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            4usize => {
                                if _serde::__private::Option::is_some(&m_skinPositions) {
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "skinPositions",
                                        ),
                                    );
                                }
                                m_skinPositions = _serde::__private::Some(
                                    match __A::next_value::<bool>(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            5usize => {
                                if _serde::__private::Option::is_some(&m_skinNormals) {
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "skinNormals",
                                        ),
                                    );
                                }
                                m_skinNormals = _serde::__private::Some(
                                    match __A::next_value::<bool>(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            6usize => {
                                if _serde::__private::Option::is_some(&m_skinTangents) {
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "skinTangents",
                                        ),
                                    );
                                }
                                m_skinTangents = _serde::__private::Some(
                                    match __A::next_value::<bool>(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            7usize => {
                                if _serde::__private::Option::is_some(&m_skinBiTangents) {
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "skinBiTangents",
                                        ),
                                    );
                                }
                                m_skinBiTangents = _serde::__private::Some(
                                    match __A::next_value::<bool>(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            8usize => {
                                if _serde::__private::Option::is_some(&m_inputBufferIndex) {
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "inputBufferIndex",
                                        ),
                                    );
                                }
                                m_inputBufferIndex = _serde::__private::Some(
                                    match __A::next_value::<U32<'de>>(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            9usize => {
                                if _serde::__private::Option::is_some(
                                    &m_outputBufferIndex,
                                ) {
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "outputBufferIndex",
                                        ),
                                    );
                                }
                                m_outputBufferIndex = _serde::__private::Some(
                                    match __A::next_value::<U32<'de>>(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            10usize => {
                                if _serde::__private::Option::is_some(
                                    &m_transformSetIndex,
                                ) {
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "transformSetIndex",
                                        ),
                                    );
                                }
                                m_transformSetIndex = _serde::__private::Some(
                                    match __A::next_value::<U32<'de>>(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            11usize => {
                                if _serde::__private::Option::is_some(&m_startVertex) {
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "startVertex",
                                        ),
                                    );
                                }
                                m_startVertex = _serde::__private::Some(
                                    match __A::next_value::<U16<'de>>(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            12usize => {
                                if _serde::__private::Option::is_some(&m_endVertex) {
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "endVertex",
                                        ),
                                    );
                                }
                                m_endVertex = _serde::__private::Some(
                                    match __A::next_value::<U16<'de>>(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            13usize => {
                                if _serde::__private::Option::is_some(&m_partialSkinning) {
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "partialSkinning",
                                        ),
                                    );
                                }
                                m_partialSkinning = _serde::__private::Some(
                                    match __A::next_value::<bool>(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            14usize => {
                                if _serde::__private::Option::is_some(
                                    &m_dualQuaternionSkinning,
                                ) {
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "dualQuaternionSkinning",
                                        ),
                                    );
                                }
                                m_dualQuaternionSkinning = _serde::__private::Some(
                                    match __A::next_value::<bool>(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            15usize => {
                                if _serde::__private::Option::is_some(&m_boneGroupSize) {
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "boneGroupSize",
                                        ),
                                    );
                                }
                                m_boneGroupSize = _serde::__private::Some(
                                    match __A::next_value::<U8<'de>>(&mut __map) {
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
                    __A::pad(&mut __map, 0usize, 1usize)?;
                    let m_boneInfluences = match m_boneInfluences {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "boneInfluences",
                                ),
                            );
                        }
                    };
                    let m_boneInfluenceStartPerVertex = match m_boneInfluenceStartPerVertex {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "boneInfluenceStartPerVertex",
                                ),
                            );
                        }
                    };
                    let m_boneFromSkinMeshTransforms = match m_boneFromSkinMeshTransforms {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "boneFromSkinMeshTransforms",
                                ),
                            );
                        }
                    };
                    let m_usedBoneGroupIds = match m_usedBoneGroupIds {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "usedBoneGroupIds",
                                ),
                            );
                        }
                    };
                    let m_skinPositions = match m_skinPositions {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "skinPositions",
                                ),
                            );
                        }
                    };
                    let m_skinNormals = match m_skinNormals {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "skinNormals",
                                ),
                            );
                        }
                    };
                    let m_skinTangents = match m_skinTangents {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "skinTangents",
                                ),
                            );
                        }
                    };
                    let m_skinBiTangents = match m_skinBiTangents {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "skinBiTangents",
                                ),
                            );
                        }
                    };
                    let m_inputBufferIndex = match m_inputBufferIndex {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "inputBufferIndex",
                                ),
                            );
                        }
                    };
                    let m_outputBufferIndex = match m_outputBufferIndex {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "outputBufferIndex",
                                ),
                            );
                        }
                    };
                    let m_transformSetIndex = match m_transformSetIndex {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "transformSetIndex",
                                ),
                            );
                        }
                    };
                    let m_startVertex = match m_startVertex {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "startVertex",
                                ),
                            );
                        }
                    };
                    let m_endVertex = match m_endVertex {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "endVertex",
                                ),
                            );
                        }
                    };
                    let m_partialSkinning = match m_partialSkinning {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "partialSkinning",
                                ),
                            );
                        }
                    };
                    let m_dualQuaternionSkinning = match m_dualQuaternionSkinning {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "dualQuaternionSkinning",
                                ),
                            );
                        }
                    };
                    let m_boneGroupSize = match m_boneGroupSize {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "boneGroupSize",
                                ),
                            );
                        }
                    };
                    _serde::__private::Ok(hclSkinOperator {
                        __ptr,
                        parent,
                        m_boneInfluences,
                        m_boneInfluenceStartPerVertex,
                        m_boneFromSkinMeshTransforms,
                        m_usedBoneGroupIds,
                        m_skinPositions,
                        m_skinNormals,
                        m_skinTangents,
                        m_skinBiTangents,
                        m_inputBufferIndex,
                        m_outputBufferIndex,
                        m_transformSetIndex,
                        m_startVertex,
                        m_endVertex,
                        m_partialSkinning,
                        m_dualQuaternionSkinning,
                        m_boneGroupSize,
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
                    let mut m_boneInfluences: _serde::__private::Option<
                        Vec<hclSkinOperatorBoneInfluence<'de>>,
                    > = _serde::__private::None;
                    let mut m_boneInfluenceStartPerVertex: _serde::__private::Option<
                        Vec<U16<'de>>,
                    > = _serde::__private::None;
                    let mut m_boneFromSkinMeshTransforms: _serde::__private::Option<
                        Vec<Matrix4>,
                    > = _serde::__private::None;
                    let mut m_usedBoneGroupIds: _serde::__private::Option<
                        Vec<U16<'de>>,
                    > = _serde::__private::None;
                    let mut m_skinPositions: _serde::__private::Option<bool> = _serde::__private::None;
                    let mut m_skinNormals: _serde::__private::Option<bool> = _serde::__private::None;
                    let mut m_skinTangents: _serde::__private::Option<bool> = _serde::__private::None;
                    let mut m_skinBiTangents: _serde::__private::Option<bool> = _serde::__private::None;
                    let mut m_inputBufferIndex: _serde::__private::Option<U32<'de>> = _serde::__private::None;
                    let mut m_outputBufferIndex: _serde::__private::Option<U32<'de>> = _serde::__private::None;
                    let mut m_transformSetIndex: _serde::__private::Option<U32<'de>> = _serde::__private::None;
                    let mut m_startVertex: _serde::__private::Option<U16<'de>> = _serde::__private::None;
                    let mut m_endVertex: _serde::__private::Option<U16<'de>> = _serde::__private::None;
                    let mut m_partialSkinning: _serde::__private::Option<bool> = _serde::__private::None;
                    let mut m_dualQuaternionSkinning: _serde::__private::Option<bool> = _serde::__private::None;
                    let mut m_boneGroupSize: _serde::__private::Option<U8<'de>> = _serde::__private::None;
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
                            __Field::m_boneInfluences => {
                                #[cfg(
                                    any(feature = "strict", feature = "ignore_duplicates")
                                )]
                                if _serde::__private::Option::is_some(&m_boneInfluences) {
                                    #[cfg(feature = "ignore_duplicates")]
                                    {
                                        __A::skip_value(&mut __map)?;
                                        continue;
                                    }
                                    #[cfg(feature = "strict")]
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "boneInfluences",
                                        ),
                                    );
                                }
                                m_boneInfluences = _serde::__private::Some(
                                    match __A::next_value::<
                                        Vec<hclSkinOperatorBoneInfluence<'de>>,
                                    >(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            __Field::m_boneInfluenceStartPerVertex => {
                                #[cfg(
                                    any(feature = "strict", feature = "ignore_duplicates")
                                )]
                                if _serde::__private::Option::is_some(
                                    &m_boneInfluenceStartPerVertex,
                                ) {
                                    #[cfg(feature = "ignore_duplicates")]
                                    {
                                        __A::skip_value(&mut __map)?;
                                        continue;
                                    }
                                    #[cfg(feature = "strict")]
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "boneInfluenceStartPerVertex",
                                        ),
                                    );
                                }
                                m_boneInfluenceStartPerVertex = _serde::__private::Some(
                                    match __A::next_value::<Vec<U16<'de>>>(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            __Field::m_boneFromSkinMeshTransforms => {
                                #[cfg(
                                    any(feature = "strict", feature = "ignore_duplicates")
                                )]
                                if _serde::__private::Option::is_some(
                                    &m_boneFromSkinMeshTransforms,
                                ) {
                                    #[cfg(feature = "ignore_duplicates")]
                                    {
                                        __A::skip_value(&mut __map)?;
                                        continue;
                                    }
                                    #[cfg(feature = "strict")]
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "boneFromSkinMeshTransforms",
                                        ),
                                    );
                                }
                                m_boneFromSkinMeshTransforms = _serde::__private::Some(
                                    match __A::next_value::<Vec<Matrix4>>(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            __Field::m_usedBoneGroupIds => {
                                #[cfg(
                                    any(feature = "strict", feature = "ignore_duplicates")
                                )]
                                if _serde::__private::Option::is_some(&m_usedBoneGroupIds) {
                                    #[cfg(feature = "ignore_duplicates")]
                                    {
                                        __A::skip_value(&mut __map)?;
                                        continue;
                                    }
                                    #[cfg(feature = "strict")]
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "usedBoneGroupIds",
                                        ),
                                    );
                                }
                                m_usedBoneGroupIds = _serde::__private::Some(
                                    match __A::next_value::<Vec<U16<'de>>>(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            __Field::m_skinPositions => {
                                #[cfg(
                                    any(feature = "strict", feature = "ignore_duplicates")
                                )]
                                if _serde::__private::Option::is_some(&m_skinPositions) {
                                    #[cfg(feature = "ignore_duplicates")]
                                    {
                                        __A::skip_value(&mut __map)?;
                                        continue;
                                    }
                                    #[cfg(feature = "strict")]
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "skinPositions",
                                        ),
                                    );
                                }
                                m_skinPositions = _serde::__private::Some(
                                    match __A::next_value::<bool>(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            __Field::m_skinNormals => {
                                #[cfg(
                                    any(feature = "strict", feature = "ignore_duplicates")
                                )]
                                if _serde::__private::Option::is_some(&m_skinNormals) {
                                    #[cfg(feature = "ignore_duplicates")]
                                    {
                                        __A::skip_value(&mut __map)?;
                                        continue;
                                    }
                                    #[cfg(feature = "strict")]
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "skinNormals",
                                        ),
                                    );
                                }
                                m_skinNormals = _serde::__private::Some(
                                    match __A::next_value::<bool>(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            __Field::m_skinTangents => {
                                #[cfg(
                                    any(feature = "strict", feature = "ignore_duplicates")
                                )]
                                if _serde::__private::Option::is_some(&m_skinTangents) {
                                    #[cfg(feature = "ignore_duplicates")]
                                    {
                                        __A::skip_value(&mut __map)?;
                                        continue;
                                    }
                                    #[cfg(feature = "strict")]
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "skinTangents",
                                        ),
                                    );
                                }
                                m_skinTangents = _serde::__private::Some(
                                    match __A::next_value::<bool>(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            __Field::m_skinBiTangents => {
                                #[cfg(
                                    any(feature = "strict", feature = "ignore_duplicates")
                                )]
                                if _serde::__private::Option::is_some(&m_skinBiTangents) {
                                    #[cfg(feature = "ignore_duplicates")]
                                    {
                                        __A::skip_value(&mut __map)?;
                                        continue;
                                    }
                                    #[cfg(feature = "strict")]
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "skinBiTangents",
                                        ),
                                    );
                                }
                                m_skinBiTangents = _serde::__private::Some(
                                    match __A::next_value::<bool>(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            __Field::m_inputBufferIndex => {
                                #[cfg(
                                    any(feature = "strict", feature = "ignore_duplicates")
                                )]
                                if _serde::__private::Option::is_some(&m_inputBufferIndex) {
                                    #[cfg(feature = "ignore_duplicates")]
                                    {
                                        __A::skip_value(&mut __map)?;
                                        continue;
                                    }
                                    #[cfg(feature = "strict")]
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "inputBufferIndex",
                                        ),
                                    );
                                }
                                m_inputBufferIndex = _serde::__private::Some(
                                    match __A::next_value::<U32<'de>>(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            __Field::m_outputBufferIndex => {
                                #[cfg(
                                    any(feature = "strict", feature = "ignore_duplicates")
                                )]
                                if _serde::__private::Option::is_some(
                                    &m_outputBufferIndex,
                                ) {
                                    #[cfg(feature = "ignore_duplicates")]
                                    {
                                        __A::skip_value(&mut __map)?;
                                        continue;
                                    }
                                    #[cfg(feature = "strict")]
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "outputBufferIndex",
                                        ),
                                    );
                                }
                                m_outputBufferIndex = _serde::__private::Some(
                                    match __A::next_value::<U32<'de>>(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            __Field::m_transformSetIndex => {
                                #[cfg(
                                    any(feature = "strict", feature = "ignore_duplicates")
                                )]
                                if _serde::__private::Option::is_some(
                                    &m_transformSetIndex,
                                ) {
                                    #[cfg(feature = "ignore_duplicates")]
                                    {
                                        __A::skip_value(&mut __map)?;
                                        continue;
                                    }
                                    #[cfg(feature = "strict")]
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "transformSetIndex",
                                        ),
                                    );
                                }
                                m_transformSetIndex = _serde::__private::Some(
                                    match __A::next_value::<U32<'de>>(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            __Field::m_startVertex => {
                                #[cfg(
                                    any(feature = "strict", feature = "ignore_duplicates")
                                )]
                                if _serde::__private::Option::is_some(&m_startVertex) {
                                    #[cfg(feature = "ignore_duplicates")]
                                    {
                                        __A::skip_value(&mut __map)?;
                                        continue;
                                    }
                                    #[cfg(feature = "strict")]
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "startVertex",
                                        ),
                                    );
                                }
                                m_startVertex = _serde::__private::Some(
                                    match __A::next_value::<U16<'de>>(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            __Field::m_endVertex => {
                                #[cfg(
                                    any(feature = "strict", feature = "ignore_duplicates")
                                )]
                                if _serde::__private::Option::is_some(&m_endVertex) {
                                    #[cfg(feature = "ignore_duplicates")]
                                    {
                                        __A::skip_value(&mut __map)?;
                                        continue;
                                    }
                                    #[cfg(feature = "strict")]
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "endVertex",
                                        ),
                                    );
                                }
                                m_endVertex = _serde::__private::Some(
                                    match __A::next_value::<U16<'de>>(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            __Field::m_partialSkinning => {
                                #[cfg(
                                    any(feature = "strict", feature = "ignore_duplicates")
                                )]
                                if _serde::__private::Option::is_some(&m_partialSkinning) {
                                    #[cfg(feature = "ignore_duplicates")]
                                    {
                                        __A::skip_value(&mut __map)?;
                                        continue;
                                    }
                                    #[cfg(feature = "strict")]
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "partialSkinning",
                                        ),
                                    );
                                }
                                m_partialSkinning = _serde::__private::Some(
                                    match __A::next_value::<bool>(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            __Field::m_dualQuaternionSkinning => {
                                #[cfg(
                                    any(feature = "strict", feature = "ignore_duplicates")
                                )]
                                if _serde::__private::Option::is_some(
                                    &m_dualQuaternionSkinning,
                                ) {
                                    #[cfg(feature = "ignore_duplicates")]
                                    {
                                        __A::skip_value(&mut __map)?;
                                        continue;
                                    }
                                    #[cfg(feature = "strict")]
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "dualQuaternionSkinning",
                                        ),
                                    );
                                }
                                m_dualQuaternionSkinning = _serde::__private::Some(
                                    match __A::next_value::<bool>(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            __Field::m_boneGroupSize => {
                                #[cfg(
                                    any(feature = "strict", feature = "ignore_duplicates")
                                )]
                                if _serde::__private::Option::is_some(&m_boneGroupSize) {
                                    #[cfg(feature = "ignore_duplicates")]
                                    {
                                        __A::skip_value(&mut __map)?;
                                        continue;
                                    }
                                    #[cfg(feature = "strict")]
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "boneGroupSize",
                                        ),
                                    );
                                }
                                m_boneGroupSize = _serde::__private::Some(
                                    match __A::next_value::<U8<'de>>(&mut __map) {
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
                    let m_boneInfluences = match m_boneInfluences {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            #[cfg(feature = "strict")]
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "boneInfluences",
                                ),
                            );
                            #[cfg(not(feature = "strict"))] Default::default()
                        }
                    };
                    let m_boneInfluenceStartPerVertex = match m_boneInfluenceStartPerVertex {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            #[cfg(feature = "strict")]
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "boneInfluenceStartPerVertex",
                                ),
                            );
                            #[cfg(not(feature = "strict"))] Default::default()
                        }
                    };
                    let m_boneFromSkinMeshTransforms = match m_boneFromSkinMeshTransforms {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            #[cfg(feature = "strict")]
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "boneFromSkinMeshTransforms",
                                ),
                            );
                            #[cfg(not(feature = "strict"))] Default::default()
                        }
                    };
                    let m_usedBoneGroupIds = match m_usedBoneGroupIds {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            #[cfg(feature = "strict")]
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "usedBoneGroupIds",
                                ),
                            );
                            #[cfg(not(feature = "strict"))] Default::default()
                        }
                    };
                    let m_skinPositions = match m_skinPositions {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            #[cfg(feature = "strict")]
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "skinPositions",
                                ),
                            );
                            #[cfg(not(feature = "strict"))] Default::default()
                        }
                    };
                    let m_skinNormals = match m_skinNormals {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            #[cfg(feature = "strict")]
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "skinNormals",
                                ),
                            );
                            #[cfg(not(feature = "strict"))] Default::default()
                        }
                    };
                    let m_skinTangents = match m_skinTangents {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            #[cfg(feature = "strict")]
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "skinTangents",
                                ),
                            );
                            #[cfg(not(feature = "strict"))] Default::default()
                        }
                    };
                    let m_skinBiTangents = match m_skinBiTangents {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            #[cfg(feature = "strict")]
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "skinBiTangents",
                                ),
                            );
                            #[cfg(not(feature = "strict"))] Default::default()
                        }
                    };
                    let m_inputBufferIndex = match m_inputBufferIndex {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            #[cfg(feature = "strict")]
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "inputBufferIndex",
                                ),
                            );
                            #[cfg(not(feature = "strict"))] Default::default()
                        }
                    };
                    let m_outputBufferIndex = match m_outputBufferIndex {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            #[cfg(feature = "strict")]
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "outputBufferIndex",
                                ),
                            );
                            #[cfg(not(feature = "strict"))] Default::default()
                        }
                    };
                    let m_transformSetIndex = match m_transformSetIndex {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            #[cfg(feature = "strict")]
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "transformSetIndex",
                                ),
                            );
                            #[cfg(not(feature = "strict"))] Default::default()
                        }
                    };
                    let m_startVertex = match m_startVertex {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            #[cfg(feature = "strict")]
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "startVertex",
                                ),
                            );
                            #[cfg(not(feature = "strict"))] Default::default()
                        }
                    };
                    let m_endVertex = match m_endVertex {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            #[cfg(feature = "strict")]
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "endVertex",
                                ),
                            );
                            #[cfg(not(feature = "strict"))] Default::default()
                        }
                    };
                    let m_partialSkinning = match m_partialSkinning {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            #[cfg(feature = "strict")]
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "partialSkinning",
                                ),
                            );
                            #[cfg(not(feature = "strict"))] Default::default()
                        }
                    };
                    let m_dualQuaternionSkinning = match m_dualQuaternionSkinning {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            #[cfg(feature = "strict")]
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "dualQuaternionSkinning",
                                ),
                            );
                            #[cfg(not(feature = "strict"))] Default::default()
                        }
                    };
                    let m_boneGroupSize = match m_boneGroupSize {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            #[cfg(feature = "strict")]
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "boneGroupSize",
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
                    let parent = hclOperator {
                        __ptr: __ptr.clone(),
                        parent,
                        m_name,
                        ..Default::default()
                    };
                    let __ptr = __A::class_ptr(&mut __map);
                    _serde::__private::Ok(hclSkinOperator {
                        __ptr: __ptr.clone(),
                        parent,
                        m_boneInfluences,
                        m_boneInfluenceStartPerVertex,
                        m_boneFromSkinMeshTransforms,
                        m_usedBoneGroupIds,
                        m_skinPositions,
                        m_skinNormals,
                        m_skinTangents,
                        m_skinBiTangents,
                        m_inputBufferIndex,
                        m_outputBufferIndex,
                        m_transformSetIndex,
                        m_startVertex,
                        m_endVertex,
                        m_partialSkinning,
                        m_dualQuaternionSkinning,
                        m_boneGroupSize,
                    })
                }
            }
            const FIELDS: &[&str] = &[
                "boneInfluences",
                "boneInfluenceStartPerVertex",
                "boneFromSkinMeshTransforms",
                "usedBoneGroupIds",
                "skinPositions",
                "skinNormals",
                "skinTangents",
                "skinBiTangents",
                "inputBufferIndex",
                "outputBufferIndex",
                "transformSetIndex",
                "startVertex",
                "endVertex",
                "partialSkinning",
                "dualQuaternionSkinning",
                "boneGroupSize",
            ];
            _serde::Deserializer::deserialize_struct(
                deserializer,
                "hclSkinOperator",
                FIELDS,
                __hclSkinOperatorVisitor {
                    marker: _serde::__private::PhantomData::<hclSkinOperator>,
                    lifetime: _serde::__private::PhantomData,
                },
            )
        }
    }
};
