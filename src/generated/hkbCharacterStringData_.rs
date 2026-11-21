use super::class_requires::*;
use super::*;
/// # C++ Info
/// - name: `hkbCharacterStringData`
/// - version: `9`
/// - signature: `0xb9d8a52`
/// - size: `  0`(x86)/`224`(x86_64)
/// -  vtable: `true`
#[allow(non_upper_case_globals, non_snake_case)]
#[cfg_attr(feature = "json_schema", derive(schemars::JsonSchema))]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(educe::Educe)]
#[educe(Debug, Clone, Default, PartialEq)]
pub struct hkbCharacterStringData<'a> {
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
    /// - name: `skinNames`(ctype: `hkArray<struct hkbCharacterStringDataFileNameMeshNamePair>`)
    /// - offset: `  0`(x86)/` 16`(x86_64)
    /// - type_size: `  0`(x86)/` 16`(x86_64)
    #[cfg_attr(feature = "serde_default", serde(default))]
    #[cfg_attr(feature = "serde", serde(borrow))]
    #[cfg_attr(feature = "json_schema", schemars(rename = "skinNames"))]
    #[cfg_attr(feature = "serde", serde(rename = "skinNames"))]
    pub m_skinNames: Vec<hkbCharacterStringDataFileNameMeshNamePair<'a>>,
    /// # C++ Info
    /// - name: `boneAttachmentNames`(ctype: `hkArray<struct hkbCharacterStringDataFileNameMeshNamePair>`)
    /// - offset: `  0`(x86)/` 32`(x86_64)
    /// - type_size: `  0`(x86)/` 16`(x86_64)
    #[cfg_attr(feature = "serde_default", serde(default))]
    #[cfg_attr(feature = "serde", serde(borrow))]
    #[cfg_attr(feature = "json_schema", schemars(rename = "boneAttachmentNames"))]
    #[cfg_attr(feature = "serde", serde(rename = "boneAttachmentNames"))]
    pub m_boneAttachmentNames: Vec<hkbCharacterStringDataFileNameMeshNamePair<'a>>,
    /// # C++ Info
    /// - name: `animationBundleNameData`(ctype: `hkArray<struct hkbAssetBundleStringData>`)
    /// - offset: `  0`(x86)/` 48`(x86_64)
    /// - type_size: `  0`(x86)/` 16`(x86_64)
    #[cfg_attr(feature = "serde_default", serde(default))]
    #[cfg_attr(feature = "serde", serde(borrow))]
    #[cfg_attr(feature = "json_schema", schemars(rename = "animationBundleNameData"))]
    #[cfg_attr(feature = "serde", serde(rename = "animationBundleNameData"))]
    pub m_animationBundleNameData: Vec<hkbAssetBundleStringData<'a>>,
    /// # C++ Info
    /// - name: `animationBundleFilenameData`(ctype: `hkArray<struct hkbAssetBundleStringData>`)
    /// - offset: `  0`(x86)/` 64`(x86_64)
    /// - type_size: `  0`(x86)/` 16`(x86_64)
    #[cfg_attr(feature = "serde_default", serde(default))]
    #[cfg_attr(feature = "serde", serde(borrow))]
    #[cfg_attr(
        feature = "json_schema",
        schemars(rename = "animationBundleFilenameData")
    )]
    #[cfg_attr(feature = "serde", serde(rename = "animationBundleFilenameData"))]
    pub m_animationBundleFilenameData: Vec<hkbAssetBundleStringData<'a>>,
    /// # C++ Info
    /// - name: `characterPropertyNames`(ctype: `hkArray<hkStringPtr>`)
    /// - offset: `  0`(x86)/` 80`(x86_64)
    /// - type_size: `  0`(x86)/` 16`(x86_64)
    #[cfg_attr(feature = "serde_default", serde(default))]
    #[cfg_attr(feature = "serde", serde(borrow))]
    #[cfg_attr(feature = "json_schema", schemars(rename = "characterPropertyNames"))]
    #[cfg_attr(feature = "serde", serde(rename = "characterPropertyNames"))]
    pub m_characterPropertyNames: Vec<StringPtr<'a>>,
    /// # C++ Info
    /// - name: `retargetingSkeletonMapperFilenames`(ctype: `hkArray<hkStringPtr>`)
    /// - offset: `  0`(x86)/` 96`(x86_64)
    /// - type_size: `  0`(x86)/` 16`(x86_64)
    #[cfg_attr(feature = "serde_default", serde(default))]
    #[cfg_attr(feature = "serde", serde(borrow))]
    #[cfg_attr(
        feature = "json_schema",
        schemars(rename = "retargetingSkeletonMapperFilenames")
    )]
    #[cfg_attr(feature = "serde", serde(rename = "retargetingSkeletonMapperFilenames"))]
    pub m_retargetingSkeletonMapperFilenames: Vec<StringPtr<'a>>,
    /// # C++ Info
    /// - name: `lodNames`(ctype: `hkArray<hkStringPtr>`)
    /// - offset: `  0`(x86)/`112`(x86_64)
    /// - type_size: `  0`(x86)/` 16`(x86_64)
    #[cfg_attr(feature = "serde_default", serde(default))]
    #[cfg_attr(feature = "serde", serde(borrow))]
    #[cfg_attr(feature = "json_schema", schemars(rename = "lodNames"))]
    #[cfg_attr(feature = "serde", serde(rename = "lodNames"))]
    pub m_lodNames: Vec<StringPtr<'a>>,
    /// # C++ Info
    /// - name: `mirroredSyncPointSubstringsA`(ctype: `hkArray<hkStringPtr>`)
    /// - offset: `  0`(x86)/`128`(x86_64)
    /// - type_size: `  0`(x86)/` 16`(x86_64)
    #[cfg_attr(feature = "serde_default", serde(default))]
    #[cfg_attr(feature = "serde", serde(borrow))]
    #[cfg_attr(
        feature = "json_schema",
        schemars(rename = "mirroredSyncPointSubstringsA")
    )]
    #[cfg_attr(feature = "serde", serde(rename = "mirroredSyncPointSubstringsA"))]
    pub m_mirroredSyncPointSubstringsA: Vec<StringPtr<'a>>,
    /// # C++ Info
    /// - name: `mirroredSyncPointSubstringsB`(ctype: `hkArray<hkStringPtr>`)
    /// - offset: `  0`(x86)/`144`(x86_64)
    /// - type_size: `  0`(x86)/` 16`(x86_64)
    #[cfg_attr(feature = "serde_default", serde(default))]
    #[cfg_attr(feature = "serde", serde(borrow))]
    #[cfg_attr(
        feature = "json_schema",
        schemars(rename = "mirroredSyncPointSubstringsB")
    )]
    #[cfg_attr(feature = "serde", serde(rename = "mirroredSyncPointSubstringsB"))]
    pub m_mirroredSyncPointSubstringsB: Vec<StringPtr<'a>>,
    /// # C++ Info
    /// - name: `name`(ctype: `hkStringPtr`)
    /// - offset: `  0`(x86)/`160`(x86_64)
    /// - type_size: `  0`(x86)/`  8`(x86_64)
    #[cfg_attr(feature = "serde_default", serde(default))]
    #[cfg_attr(feature = "serde", serde(borrow))]
    #[cfg_attr(feature = "json_schema", schemars(rename = "name"))]
    #[cfg_attr(feature = "serde", serde(rename = "name"))]
    pub m_name: StringPtr<'a>,
    /// # C++ Info
    /// - name: `rigName`(ctype: `hkStringPtr`)
    /// - offset: `  0`(x86)/`168`(x86_64)
    /// - type_size: `  0`(x86)/`  8`(x86_64)
    #[cfg_attr(feature = "serde_default", serde(default))]
    #[cfg_attr(feature = "serde", serde(borrow))]
    #[cfg_attr(feature = "json_schema", schemars(rename = "rigName"))]
    #[cfg_attr(feature = "serde", serde(rename = "rigName"))]
    pub m_rigName: StringPtr<'a>,
    /// # C++ Info
    /// - name: `ragdollName`(ctype: `hkStringPtr`)
    /// - offset: `  0`(x86)/`176`(x86_64)
    /// - type_size: `  0`(x86)/`  8`(x86_64)
    #[cfg_attr(feature = "serde_default", serde(default))]
    #[cfg_attr(feature = "serde", serde(borrow))]
    #[cfg_attr(feature = "json_schema", schemars(rename = "ragdollName"))]
    #[cfg_attr(feature = "serde", serde(rename = "ragdollName"))]
    pub m_ragdollName: StringPtr<'a>,
    /// # C++ Info
    /// - name: `behaviorFilename`(ctype: `hkStringPtr`)
    /// - offset: `  0`(x86)/`184`(x86_64)
    /// - type_size: `  0`(x86)/`  8`(x86_64)
    #[cfg_attr(feature = "serde_default", serde(default))]
    #[cfg_attr(feature = "serde", serde(borrow))]
    #[cfg_attr(feature = "json_schema", schemars(rename = "behaviorFilename"))]
    #[cfg_attr(feature = "serde", serde(rename = "behaviorFilename"))]
    pub m_behaviorFilename: StringPtr<'a>,
    /// # C++ Info
    /// - name: `luaScriptOnCharacterActivated`(ctype: `hkStringPtr`)
    /// - offset: `  0`(x86)/`192`(x86_64)
    /// - type_size: `  0`(x86)/`  8`(x86_64)
    #[cfg_attr(feature = "serde_default", serde(default))]
    #[cfg_attr(feature = "serde", serde(borrow))]
    #[cfg_attr(
        feature = "json_schema",
        schemars(rename = "luaScriptOnCharacterActivated")
    )]
    #[cfg_attr(feature = "serde", serde(rename = "luaScriptOnCharacterActivated"))]
    pub m_luaScriptOnCharacterActivated: StringPtr<'a>,
    /// # C++ Info
    /// - name: `luaScriptOnCharacterDeactivated`(ctype: `hkStringPtr`)
    /// - offset: `  0`(x86)/`200`(x86_64)
    /// - type_size: `  0`(x86)/`  8`(x86_64)
    #[cfg_attr(feature = "serde_default", serde(default))]
    #[cfg_attr(feature = "serde", serde(borrow))]
    #[cfg_attr(
        feature = "json_schema",
        schemars(rename = "luaScriptOnCharacterDeactivated")
    )]
    #[cfg_attr(feature = "serde", serde(rename = "luaScriptOnCharacterDeactivated"))]
    pub m_luaScriptOnCharacterDeactivated: StringPtr<'a>,
    /// # C++ Info
    /// - name: `luaFiles`(ctype: `hkArray<hkStringPtr>`)
    /// - offset: `  0`(x86)/`208`(x86_64)
    /// - type_size: `  0`(x86)/` 16`(x86_64)
    #[cfg_attr(feature = "serde_default", serde(default))]
    #[cfg_attr(feature = "serde", serde(borrow))]
    #[cfg_attr(feature = "json_schema", schemars(rename = "luaFiles"))]
    #[cfg_attr(feature = "serde", serde(rename = "luaFiles"))]
    pub m_luaFiles: Vec<StringPtr<'a>>,
}
const _: () = {
    use havok_serde as _serde;
    impl<'a> _serde::HavokClass for hkbCharacterStringData<'a> {
        #[inline]
        fn name(&self) -> &'static str {
            "hkbCharacterStringData"
        }
        #[inline]
        fn signature(&self) -> _serde::__private::Signature {
            _serde::__private::Signature::new(0xb9d8a52)
        }
        #[allow(clippy::let_and_return, clippy::vec_init_then_push)]
        fn deps_indexes(&self) -> Vec<&Pointer<'_>> {
            let mut v = Vec::new();
            v.extend(
                self
                    .m_skinNames
                    .iter()
                    .flat_map(|class| class.deps_indexes())
                    .collect::<Vec<&Pointer<'_>>>(),
            );
            v.extend(
                self
                    .m_boneAttachmentNames
                    .iter()
                    .flat_map(|class| class.deps_indexes())
                    .collect::<Vec<&Pointer<'_>>>(),
            );
            v.extend(
                self
                    .m_animationBundleNameData
                    .iter()
                    .flat_map(|class| class.deps_indexes())
                    .collect::<Vec<&Pointer<'_>>>(),
            );
            v.extend(
                self
                    .m_animationBundleFilenameData
                    .iter()
                    .flat_map(|class| class.deps_indexes())
                    .collect::<Vec<&Pointer<'_>>>(),
            );
            v
        }
    }
    impl<'a> _serde::Serialize for hkbCharacterStringData<'a> {
        fn serialize<S>(&self, __serializer: S) -> Result<S::Ok, S::Error>
        where
            S: _serde::ser::Serializer,
        {
            let class_meta = self
                .__ptr
                .as_ref()
                .map(|name| (name, _serde::__private::Signature::new(0xb9d8a52)));
            let mut serializer = __serializer
                .serialize_struct("hkbCharacterStringData", class_meta, (0u64, 224u64))?;
            serializer.pad_field([0u8; 0usize].as_slice(), [0u8; 8usize].as_slice())?;
            serializer
                .skip_field("memSizeAndRefCount", &self.parent.m_memSizeAndRefCount)?;
            serializer.pad_field([0u8; 0usize].as_slice(), [0u8; 4usize].as_slice())?;
            serializer
                .serialize_array_field(
                    "skinNames",
                    &self.m_skinNames,
                    TypeSize::Struct {
                        size_x86: 0u64,
                        size_x86_64: 16u64,
                    },
                )?;
            serializer
                .serialize_array_field(
                    "boneAttachmentNames",
                    &self.m_boneAttachmentNames,
                    TypeSize::Struct {
                        size_x86: 0u64,
                        size_x86_64: 16u64,
                    },
                )?;
            serializer
                .serialize_array_field(
                    "animationBundleNameData",
                    &self.m_animationBundleNameData,
                    TypeSize::Struct {
                        size_x86: 0u64,
                        size_x86_64: 24u64,
                    },
                )?;
            serializer
                .serialize_array_field(
                    "animationBundleFilenameData",
                    &self.m_animationBundleFilenameData,
                    TypeSize::Struct {
                        size_x86: 0u64,
                        size_x86_64: 24u64,
                    },
                )?;
            serializer
                .serialize_array_field(
                    "characterPropertyNames",
                    &self.m_characterPropertyNames,
                    TypeSize::String,
                )?;
            serializer
                .serialize_array_field(
                    "retargetingSkeletonMapperFilenames",
                    &self.m_retargetingSkeletonMapperFilenames,
                    TypeSize::String,
                )?;
            serializer
                .serialize_array_field("lodNames", &self.m_lodNames, TypeSize::String)?;
            serializer
                .serialize_array_field(
                    "mirroredSyncPointSubstringsA",
                    &self.m_mirroredSyncPointSubstringsA,
                    TypeSize::String,
                )?;
            serializer
                .serialize_array_field(
                    "mirroredSyncPointSubstringsB",
                    &self.m_mirroredSyncPointSubstringsB,
                    TypeSize::String,
                )?;
            serializer.serialize_field("name", &self.m_name)?;
            serializer.serialize_field("rigName", &self.m_rigName)?;
            serializer.serialize_field("ragdollName", &self.m_ragdollName)?;
            serializer.serialize_field("behaviorFilename", &self.m_behaviorFilename)?;
            serializer
                .serialize_field(
                    "luaScriptOnCharacterActivated",
                    &self.m_luaScriptOnCharacterActivated,
                )?;
            serializer
                .serialize_field(
                    "luaScriptOnCharacterDeactivated",
                    &self.m_luaScriptOnCharacterDeactivated,
                )?;
            serializer
                .serialize_array_field("luaFiles", &self.m_luaFiles, TypeSize::String)?;
            serializer.end()
        }
    }
};
#[doc(hidden)]
#[allow(non_upper_case_globals, unused_attributes, unused_qualifications)]
const _: () = {
    use havok_serde as _serde;
    #[automatically_derived]
    impl<'de> _serde::Deserialize<'de> for hkbCharacterStringData<'de> {
        fn deserialize<__D>(deserializer: __D) -> core::result::Result<Self, __D::Error>
        where
            __D: _serde::Deserializer<'de>,
        {
            #[allow(non_camel_case_types)]
            enum __Field {
                m_skinNames,
                m_boneAttachmentNames,
                m_animationBundleNameData,
                m_animationBundleFilenameData,
                m_characterPropertyNames,
                m_retargetingSkeletonMapperFilenames,
                m_lodNames,
                m_mirroredSyncPointSubstringsA,
                m_mirroredSyncPointSubstringsB,
                m_name,
                m_rigName,
                m_ragdollName,
                m_behaviorFilename,
                m_luaScriptOnCharacterActivated,
                m_luaScriptOnCharacterDeactivated,
                m_luaFiles,
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
                        "skinNames" => Ok(__Field::m_skinNames),
                        "boneAttachmentNames" => Ok(__Field::m_boneAttachmentNames),
                        "animationBundleNameData" => {
                            Ok(__Field::m_animationBundleNameData)
                        }
                        "animationBundleFilenameData" => {
                            Ok(__Field::m_animationBundleFilenameData)
                        }
                        "characterPropertyNames" => Ok(__Field::m_characterPropertyNames),
                        "retargetingSkeletonMapperFilenames" => {
                            Ok(__Field::m_retargetingSkeletonMapperFilenames)
                        }
                        "lodNames" => Ok(__Field::m_lodNames),
                        "mirroredSyncPointSubstringsA" => {
                            Ok(__Field::m_mirroredSyncPointSubstringsA)
                        }
                        "mirroredSyncPointSubstringsB" => {
                            Ok(__Field::m_mirroredSyncPointSubstringsB)
                        }
                        "name" => Ok(__Field::m_name),
                        "rigName" => Ok(__Field::m_rigName),
                        "ragdollName" => Ok(__Field::m_ragdollName),
                        "behaviorFilename" => Ok(__Field::m_behaviorFilename),
                        "luaScriptOnCharacterActivated" => {
                            Ok(__Field::m_luaScriptOnCharacterActivated)
                        }
                        "luaScriptOnCharacterDeactivated" => {
                            Ok(__Field::m_luaScriptOnCharacterDeactivated)
                        }
                        "luaFiles" => Ok(__Field::m_luaFiles),
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
            struct __hkbCharacterStringDataVisitor<'de> {
                marker: _serde::__private::PhantomData<hkbCharacterStringData<'de>>,
                lifetime: _serde::__private::PhantomData<&'de ()>,
            }
            #[allow(clippy::match_single_binding)]
            #[allow(clippy::reversed_empty_ranges)]
            #[allow(clippy::single_match)]
            impl<'de> _serde::de::Visitor<'de> for __hkbCharacterStringDataVisitor<'de> {
                type Value = hkbCharacterStringData<'de>;
                fn expecting(
                    &self,
                    __formatter: &mut core::fmt::Formatter,
                ) -> core::fmt::Result {
                    core::fmt::Formatter::write_str(
                        __formatter,
                        "struct hkbCharacterStringData",
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
                    let mut m_skinNames: _serde::__private::Option<
                        Vec<hkbCharacterStringDataFileNameMeshNamePair<'de>>,
                    > = _serde::__private::None;
                    let mut m_boneAttachmentNames: _serde::__private::Option<
                        Vec<hkbCharacterStringDataFileNameMeshNamePair<'de>>,
                    > = _serde::__private::None;
                    let mut m_animationBundleNameData: _serde::__private::Option<
                        Vec<hkbAssetBundleStringData<'de>>,
                    > = _serde::__private::None;
                    let mut m_animationBundleFilenameData: _serde::__private::Option<
                        Vec<hkbAssetBundleStringData<'de>>,
                    > = _serde::__private::None;
                    let mut m_characterPropertyNames: _serde::__private::Option<
                        Vec<StringPtr<'de>>,
                    > = _serde::__private::None;
                    let mut m_retargetingSkeletonMapperFilenames: _serde::__private::Option<
                        Vec<StringPtr<'de>>,
                    > = _serde::__private::None;
                    let mut m_lodNames: _serde::__private::Option<Vec<StringPtr<'de>>> = _serde::__private::None;
                    let mut m_mirroredSyncPointSubstringsA: _serde::__private::Option<
                        Vec<StringPtr<'de>>,
                    > = _serde::__private::None;
                    let mut m_mirroredSyncPointSubstringsB: _serde::__private::Option<
                        Vec<StringPtr<'de>>,
                    > = _serde::__private::None;
                    let mut m_name: _serde::__private::Option<StringPtr<'de>> = _serde::__private::None;
                    let mut m_rigName: _serde::__private::Option<StringPtr<'de>> = _serde::__private::None;
                    let mut m_ragdollName: _serde::__private::Option<StringPtr<'de>> = _serde::__private::None;
                    let mut m_behaviorFilename: _serde::__private::Option<
                        StringPtr<'de>,
                    > = _serde::__private::None;
                    let mut m_luaScriptOnCharacterActivated: _serde::__private::Option<
                        StringPtr<'de>,
                    > = _serde::__private::None;
                    let mut m_luaScriptOnCharacterDeactivated: _serde::__private::Option<
                        StringPtr<'de>,
                    > = _serde::__private::None;
                    let mut m_luaFiles: _serde::__private::Option<Vec<StringPtr<'de>>> = _serde::__private::None;
                    for i in 0..16usize {
                        match i {
                            0usize => {
                                if _serde::__private::Option::is_some(&m_skinNames) {
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "skinNames",
                                        ),
                                    );
                                }
                                m_skinNames = _serde::__private::Some(
                                    match __A::next_value::<
                                        Vec<hkbCharacterStringDataFileNameMeshNamePair<'de>>,
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
                                    &m_boneAttachmentNames,
                                ) {
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "boneAttachmentNames",
                                        ),
                                    );
                                }
                                m_boneAttachmentNames = _serde::__private::Some(
                                    match __A::next_value::<
                                        Vec<hkbCharacterStringDataFileNameMeshNamePair<'de>>,
                                    >(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            2usize => {
                                if _serde::__private::Option::is_some(
                                    &m_animationBundleNameData,
                                ) {
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "animationBundleNameData",
                                        ),
                                    );
                                }
                                m_animationBundleNameData = _serde::__private::Some(
                                    match __A::next_value::<
                                        Vec<hkbAssetBundleStringData<'de>>,
                                    >(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            3usize => {
                                if _serde::__private::Option::is_some(
                                    &m_animationBundleFilenameData,
                                ) {
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "animationBundleFilenameData",
                                        ),
                                    );
                                }
                                m_animationBundleFilenameData = _serde::__private::Some(
                                    match __A::next_value::<
                                        Vec<hkbAssetBundleStringData<'de>>,
                                    >(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            4usize => {
                                if _serde::__private::Option::is_some(
                                    &m_characterPropertyNames,
                                ) {
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "characterPropertyNames",
                                        ),
                                    );
                                }
                                m_characterPropertyNames = _serde::__private::Some(
                                    match __A::next_value::<Vec<StringPtr<'de>>>(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            5usize => {
                                if _serde::__private::Option::is_some(
                                    &m_retargetingSkeletonMapperFilenames,
                                ) {
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "retargetingSkeletonMapperFilenames",
                                        ),
                                    );
                                }
                                m_retargetingSkeletonMapperFilenames = _serde::__private::Some(
                                    match __A::next_value::<Vec<StringPtr<'de>>>(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            6usize => {
                                if _serde::__private::Option::is_some(&m_lodNames) {
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "lodNames",
                                        ),
                                    );
                                }
                                m_lodNames = _serde::__private::Some(
                                    match __A::next_value::<Vec<StringPtr<'de>>>(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            7usize => {
                                if _serde::__private::Option::is_some(
                                    &m_mirroredSyncPointSubstringsA,
                                ) {
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "mirroredSyncPointSubstringsA",
                                        ),
                                    );
                                }
                                m_mirroredSyncPointSubstringsA = _serde::__private::Some(
                                    match __A::next_value::<Vec<StringPtr<'de>>>(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            8usize => {
                                if _serde::__private::Option::is_some(
                                    &m_mirroredSyncPointSubstringsB,
                                ) {
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "mirroredSyncPointSubstringsB",
                                        ),
                                    );
                                }
                                m_mirroredSyncPointSubstringsB = _serde::__private::Some(
                                    match __A::next_value::<Vec<StringPtr<'de>>>(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            9usize => {
                                if _serde::__private::Option::is_some(&m_name) {
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
                            10usize => {
                                if _serde::__private::Option::is_some(&m_rigName) {
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "rigName",
                                        ),
                                    );
                                }
                                m_rigName = _serde::__private::Some(
                                    match __A::next_value::<StringPtr<'de>>(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            11usize => {
                                if _serde::__private::Option::is_some(&m_ragdollName) {
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "ragdollName",
                                        ),
                                    );
                                }
                                m_ragdollName = _serde::__private::Some(
                                    match __A::next_value::<StringPtr<'de>>(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            12usize => {
                                if _serde::__private::Option::is_some(&m_behaviorFilename) {
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "behaviorFilename",
                                        ),
                                    );
                                }
                                m_behaviorFilename = _serde::__private::Some(
                                    match __A::next_value::<StringPtr<'de>>(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            13usize => {
                                if _serde::__private::Option::is_some(
                                    &m_luaScriptOnCharacterActivated,
                                ) {
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "luaScriptOnCharacterActivated",
                                        ),
                                    );
                                }
                                m_luaScriptOnCharacterActivated = _serde::__private::Some(
                                    match __A::next_value::<StringPtr<'de>>(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            14usize => {
                                if _serde::__private::Option::is_some(
                                    &m_luaScriptOnCharacterDeactivated,
                                ) {
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "luaScriptOnCharacterDeactivated",
                                        ),
                                    );
                                }
                                m_luaScriptOnCharacterDeactivated = _serde::__private::Some(
                                    match __A::next_value::<StringPtr<'de>>(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            15usize => {
                                if _serde::__private::Option::is_some(&m_luaFiles) {
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "luaFiles",
                                        ),
                                    );
                                }
                                m_luaFiles = _serde::__private::Some(
                                    match __A::next_value::<Vec<StringPtr<'de>>>(&mut __map) {
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
                    let m_skinNames = match m_skinNames {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "skinNames",
                                ),
                            );
                        }
                    };
                    let m_boneAttachmentNames = match m_boneAttachmentNames {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "boneAttachmentNames",
                                ),
                            );
                        }
                    };
                    let m_animationBundleNameData = match m_animationBundleNameData {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "animationBundleNameData",
                                ),
                            );
                        }
                    };
                    let m_animationBundleFilenameData = match m_animationBundleFilenameData {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "animationBundleFilenameData",
                                ),
                            );
                        }
                    };
                    let m_characterPropertyNames = match m_characterPropertyNames {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "characterPropertyNames",
                                ),
                            );
                        }
                    };
                    let m_retargetingSkeletonMapperFilenames = match m_retargetingSkeletonMapperFilenames {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "retargetingSkeletonMapperFilenames",
                                ),
                            );
                        }
                    };
                    let m_lodNames = match m_lodNames {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field("lodNames"),
                            );
                        }
                    };
                    let m_mirroredSyncPointSubstringsA = match m_mirroredSyncPointSubstringsA {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "mirroredSyncPointSubstringsA",
                                ),
                            );
                        }
                    };
                    let m_mirroredSyncPointSubstringsB = match m_mirroredSyncPointSubstringsB {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "mirroredSyncPointSubstringsB",
                                ),
                            );
                        }
                    };
                    let m_name = match m_name {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field("name"),
                            );
                        }
                    };
                    let m_rigName = match m_rigName {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field("rigName"),
                            );
                        }
                    };
                    let m_ragdollName = match m_ragdollName {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "ragdollName",
                                ),
                            );
                        }
                    };
                    let m_behaviorFilename = match m_behaviorFilename {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "behaviorFilename",
                                ),
                            );
                        }
                    };
                    let m_luaScriptOnCharacterActivated = match m_luaScriptOnCharacterActivated {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "luaScriptOnCharacterActivated",
                                ),
                            );
                        }
                    };
                    let m_luaScriptOnCharacterDeactivated = match m_luaScriptOnCharacterDeactivated {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "luaScriptOnCharacterDeactivated",
                                ),
                            );
                        }
                    };
                    let m_luaFiles = match m_luaFiles {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field("luaFiles"),
                            );
                        }
                    };
                    _serde::__private::Ok(hkbCharacterStringData {
                        __ptr,
                        parent,
                        m_skinNames,
                        m_boneAttachmentNames,
                        m_animationBundleNameData,
                        m_animationBundleFilenameData,
                        m_characterPropertyNames,
                        m_retargetingSkeletonMapperFilenames,
                        m_lodNames,
                        m_mirroredSyncPointSubstringsA,
                        m_mirroredSyncPointSubstringsB,
                        m_name,
                        m_rigName,
                        m_ragdollName,
                        m_behaviorFilename,
                        m_luaScriptOnCharacterActivated,
                        m_luaScriptOnCharacterDeactivated,
                        m_luaFiles,
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
                    let mut m_skinNames: _serde::__private::Option<
                        Vec<hkbCharacterStringDataFileNameMeshNamePair<'de>>,
                    > = _serde::__private::None;
                    let mut m_boneAttachmentNames: _serde::__private::Option<
                        Vec<hkbCharacterStringDataFileNameMeshNamePair<'de>>,
                    > = _serde::__private::None;
                    let mut m_animationBundleNameData: _serde::__private::Option<
                        Vec<hkbAssetBundleStringData<'de>>,
                    > = _serde::__private::None;
                    let mut m_animationBundleFilenameData: _serde::__private::Option<
                        Vec<hkbAssetBundleStringData<'de>>,
                    > = _serde::__private::None;
                    let mut m_characterPropertyNames: _serde::__private::Option<
                        Vec<StringPtr<'de>>,
                    > = _serde::__private::None;
                    let mut m_retargetingSkeletonMapperFilenames: _serde::__private::Option<
                        Vec<StringPtr<'de>>,
                    > = _serde::__private::None;
                    let mut m_lodNames: _serde::__private::Option<Vec<StringPtr<'de>>> = _serde::__private::None;
                    let mut m_mirroredSyncPointSubstringsA: _serde::__private::Option<
                        Vec<StringPtr<'de>>,
                    > = _serde::__private::None;
                    let mut m_mirroredSyncPointSubstringsB: _serde::__private::Option<
                        Vec<StringPtr<'de>>,
                    > = _serde::__private::None;
                    let mut m_name: _serde::__private::Option<StringPtr<'de>> = _serde::__private::None;
                    let mut m_rigName: _serde::__private::Option<StringPtr<'de>> = _serde::__private::None;
                    let mut m_ragdollName: _serde::__private::Option<StringPtr<'de>> = _serde::__private::None;
                    let mut m_behaviorFilename: _serde::__private::Option<
                        StringPtr<'de>,
                    > = _serde::__private::None;
                    let mut m_luaScriptOnCharacterActivated: _serde::__private::Option<
                        StringPtr<'de>,
                    > = _serde::__private::None;
                    let mut m_luaScriptOnCharacterDeactivated: _serde::__private::Option<
                        StringPtr<'de>,
                    > = _serde::__private::None;
                    let mut m_luaFiles: _serde::__private::Option<Vec<StringPtr<'de>>> = _serde::__private::None;
                    while let _serde::__private::Some(__key) = {
                        __A::next_key::<__Field>(&mut __map)?
                    } {
                        match __key {
                            __Field::m_skinNames => {
                                #[cfg(
                                    any(feature = "strict", feature = "ignore_duplicates")
                                )]
                                if _serde::__private::Option::is_some(&m_skinNames) {
                                    #[cfg(feature = "ignore_duplicates")]
                                    {
                                        __A::skip_value(&mut __map)?;
                                        continue;
                                    }
                                    #[cfg(feature = "strict")]
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "skinNames",
                                        ),
                                    );
                                }
                                m_skinNames = _serde::__private::Some(
                                    match __A::next_value::<
                                        Vec<hkbCharacterStringDataFileNameMeshNamePair<'de>>,
                                    >(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            __Field::m_boneAttachmentNames => {
                                #[cfg(
                                    any(feature = "strict", feature = "ignore_duplicates")
                                )]
                                if _serde::__private::Option::is_some(
                                    &m_boneAttachmentNames,
                                ) {
                                    #[cfg(feature = "ignore_duplicates")]
                                    {
                                        __A::skip_value(&mut __map)?;
                                        continue;
                                    }
                                    #[cfg(feature = "strict")]
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "boneAttachmentNames",
                                        ),
                                    );
                                }
                                m_boneAttachmentNames = _serde::__private::Some(
                                    match __A::next_value::<
                                        Vec<hkbCharacterStringDataFileNameMeshNamePair<'de>>,
                                    >(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            __Field::m_animationBundleNameData => {
                                #[cfg(
                                    any(feature = "strict", feature = "ignore_duplicates")
                                )]
                                if _serde::__private::Option::is_some(
                                    &m_animationBundleNameData,
                                ) {
                                    #[cfg(feature = "ignore_duplicates")]
                                    {
                                        __A::skip_value(&mut __map)?;
                                        continue;
                                    }
                                    #[cfg(feature = "strict")]
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "animationBundleNameData",
                                        ),
                                    );
                                }
                                m_animationBundleNameData = _serde::__private::Some(
                                    match __A::next_value::<
                                        Vec<hkbAssetBundleStringData<'de>>,
                                    >(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            __Field::m_animationBundleFilenameData => {
                                #[cfg(
                                    any(feature = "strict", feature = "ignore_duplicates")
                                )]
                                if _serde::__private::Option::is_some(
                                    &m_animationBundleFilenameData,
                                ) {
                                    #[cfg(feature = "ignore_duplicates")]
                                    {
                                        __A::skip_value(&mut __map)?;
                                        continue;
                                    }
                                    #[cfg(feature = "strict")]
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "animationBundleFilenameData",
                                        ),
                                    );
                                }
                                m_animationBundleFilenameData = _serde::__private::Some(
                                    match __A::next_value::<
                                        Vec<hkbAssetBundleStringData<'de>>,
                                    >(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            __Field::m_characterPropertyNames => {
                                #[cfg(
                                    any(feature = "strict", feature = "ignore_duplicates")
                                )]
                                if _serde::__private::Option::is_some(
                                    &m_characterPropertyNames,
                                ) {
                                    #[cfg(feature = "ignore_duplicates")]
                                    {
                                        __A::skip_value(&mut __map)?;
                                        continue;
                                    }
                                    #[cfg(feature = "strict")]
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "characterPropertyNames",
                                        ),
                                    );
                                }
                                m_characterPropertyNames = _serde::__private::Some(
                                    match __A::next_value::<Vec<StringPtr<'de>>>(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            __Field::m_retargetingSkeletonMapperFilenames => {
                                #[cfg(
                                    any(feature = "strict", feature = "ignore_duplicates")
                                )]
                                if _serde::__private::Option::is_some(
                                    &m_retargetingSkeletonMapperFilenames,
                                ) {
                                    #[cfg(feature = "ignore_duplicates")]
                                    {
                                        __A::skip_value(&mut __map)?;
                                        continue;
                                    }
                                    #[cfg(feature = "strict")]
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "retargetingSkeletonMapperFilenames",
                                        ),
                                    );
                                }
                                m_retargetingSkeletonMapperFilenames = _serde::__private::Some(
                                    match __A::next_value::<Vec<StringPtr<'de>>>(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            __Field::m_lodNames => {
                                #[cfg(
                                    any(feature = "strict", feature = "ignore_duplicates")
                                )]
                                if _serde::__private::Option::is_some(&m_lodNames) {
                                    #[cfg(feature = "ignore_duplicates")]
                                    {
                                        __A::skip_value(&mut __map)?;
                                        continue;
                                    }
                                    #[cfg(feature = "strict")]
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "lodNames",
                                        ),
                                    );
                                }
                                m_lodNames = _serde::__private::Some(
                                    match __A::next_value::<Vec<StringPtr<'de>>>(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            __Field::m_mirroredSyncPointSubstringsA => {
                                #[cfg(
                                    any(feature = "strict", feature = "ignore_duplicates")
                                )]
                                if _serde::__private::Option::is_some(
                                    &m_mirroredSyncPointSubstringsA,
                                ) {
                                    #[cfg(feature = "ignore_duplicates")]
                                    {
                                        __A::skip_value(&mut __map)?;
                                        continue;
                                    }
                                    #[cfg(feature = "strict")]
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "mirroredSyncPointSubstringsA",
                                        ),
                                    );
                                }
                                m_mirroredSyncPointSubstringsA = _serde::__private::Some(
                                    match __A::next_value::<Vec<StringPtr<'de>>>(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            __Field::m_mirroredSyncPointSubstringsB => {
                                #[cfg(
                                    any(feature = "strict", feature = "ignore_duplicates")
                                )]
                                if _serde::__private::Option::is_some(
                                    &m_mirroredSyncPointSubstringsB,
                                ) {
                                    #[cfg(feature = "ignore_duplicates")]
                                    {
                                        __A::skip_value(&mut __map)?;
                                        continue;
                                    }
                                    #[cfg(feature = "strict")]
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "mirroredSyncPointSubstringsB",
                                        ),
                                    );
                                }
                                m_mirroredSyncPointSubstringsB = _serde::__private::Some(
                                    match __A::next_value::<Vec<StringPtr<'de>>>(&mut __map) {
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
                            __Field::m_rigName => {
                                #[cfg(
                                    any(feature = "strict", feature = "ignore_duplicates")
                                )]
                                if _serde::__private::Option::is_some(&m_rigName) {
                                    #[cfg(feature = "ignore_duplicates")]
                                    {
                                        __A::skip_value(&mut __map)?;
                                        continue;
                                    }
                                    #[cfg(feature = "strict")]
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "rigName",
                                        ),
                                    );
                                }
                                m_rigName = _serde::__private::Some(
                                    match __A::next_value::<StringPtr<'de>>(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            __Field::m_ragdollName => {
                                #[cfg(
                                    any(feature = "strict", feature = "ignore_duplicates")
                                )]
                                if _serde::__private::Option::is_some(&m_ragdollName) {
                                    #[cfg(feature = "ignore_duplicates")]
                                    {
                                        __A::skip_value(&mut __map)?;
                                        continue;
                                    }
                                    #[cfg(feature = "strict")]
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "ragdollName",
                                        ),
                                    );
                                }
                                m_ragdollName = _serde::__private::Some(
                                    match __A::next_value::<StringPtr<'de>>(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            __Field::m_behaviorFilename => {
                                #[cfg(
                                    any(feature = "strict", feature = "ignore_duplicates")
                                )]
                                if _serde::__private::Option::is_some(&m_behaviorFilename) {
                                    #[cfg(feature = "ignore_duplicates")]
                                    {
                                        __A::skip_value(&mut __map)?;
                                        continue;
                                    }
                                    #[cfg(feature = "strict")]
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "behaviorFilename",
                                        ),
                                    );
                                }
                                m_behaviorFilename = _serde::__private::Some(
                                    match __A::next_value::<StringPtr<'de>>(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            __Field::m_luaScriptOnCharacterActivated => {
                                #[cfg(
                                    any(feature = "strict", feature = "ignore_duplicates")
                                )]
                                if _serde::__private::Option::is_some(
                                    &m_luaScriptOnCharacterActivated,
                                ) {
                                    #[cfg(feature = "ignore_duplicates")]
                                    {
                                        __A::skip_value(&mut __map)?;
                                        continue;
                                    }
                                    #[cfg(feature = "strict")]
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "luaScriptOnCharacterActivated",
                                        ),
                                    );
                                }
                                m_luaScriptOnCharacterActivated = _serde::__private::Some(
                                    match __A::next_value::<StringPtr<'de>>(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            __Field::m_luaScriptOnCharacterDeactivated => {
                                #[cfg(
                                    any(feature = "strict", feature = "ignore_duplicates")
                                )]
                                if _serde::__private::Option::is_some(
                                    &m_luaScriptOnCharacterDeactivated,
                                ) {
                                    #[cfg(feature = "ignore_duplicates")]
                                    {
                                        __A::skip_value(&mut __map)?;
                                        continue;
                                    }
                                    #[cfg(feature = "strict")]
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "luaScriptOnCharacterDeactivated",
                                        ),
                                    );
                                }
                                m_luaScriptOnCharacterDeactivated = _serde::__private::Some(
                                    match __A::next_value::<StringPtr<'de>>(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            __Field::m_luaFiles => {
                                #[cfg(
                                    any(feature = "strict", feature = "ignore_duplicates")
                                )]
                                if _serde::__private::Option::is_some(&m_luaFiles) {
                                    #[cfg(feature = "ignore_duplicates")]
                                    {
                                        __A::skip_value(&mut __map)?;
                                        continue;
                                    }
                                    #[cfg(feature = "strict")]
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "luaFiles",
                                        ),
                                    );
                                }
                                m_luaFiles = _serde::__private::Some(
                                    match __A::next_value::<Vec<StringPtr<'de>>>(&mut __map) {
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
                    let m_skinNames = match m_skinNames {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            #[cfg(feature = "strict")]
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "skinNames",
                                ),
                            );
                            #[cfg(not(feature = "strict"))] Default::default()
                        }
                    };
                    let m_boneAttachmentNames = match m_boneAttachmentNames {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            #[cfg(feature = "strict")]
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "boneAttachmentNames",
                                ),
                            );
                            #[cfg(not(feature = "strict"))] Default::default()
                        }
                    };
                    let m_animationBundleNameData = match m_animationBundleNameData {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            #[cfg(feature = "strict")]
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "animationBundleNameData",
                                ),
                            );
                            #[cfg(not(feature = "strict"))] Default::default()
                        }
                    };
                    let m_animationBundleFilenameData = match m_animationBundleFilenameData {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            #[cfg(feature = "strict")]
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "animationBundleFilenameData",
                                ),
                            );
                            #[cfg(not(feature = "strict"))] Default::default()
                        }
                    };
                    let m_characterPropertyNames = match m_characterPropertyNames {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            #[cfg(feature = "strict")]
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "characterPropertyNames",
                                ),
                            );
                            #[cfg(not(feature = "strict"))] Default::default()
                        }
                    };
                    let m_retargetingSkeletonMapperFilenames = match m_retargetingSkeletonMapperFilenames {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            #[cfg(feature = "strict")]
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "retargetingSkeletonMapperFilenames",
                                ),
                            );
                            #[cfg(not(feature = "strict"))] Default::default()
                        }
                    };
                    let m_lodNames = match m_lodNames {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            #[cfg(feature = "strict")]
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field("lodNames"),
                            );
                            #[cfg(not(feature = "strict"))] Default::default()
                        }
                    };
                    let m_mirroredSyncPointSubstringsA = match m_mirroredSyncPointSubstringsA {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            #[cfg(feature = "strict")]
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "mirroredSyncPointSubstringsA",
                                ),
                            );
                            #[cfg(not(feature = "strict"))] Default::default()
                        }
                    };
                    let m_mirroredSyncPointSubstringsB = match m_mirroredSyncPointSubstringsB {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            #[cfg(feature = "strict")]
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "mirroredSyncPointSubstringsB",
                                ),
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
                    let m_rigName = match m_rigName {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            #[cfg(feature = "strict")]
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field("rigName"),
                            );
                            #[cfg(not(feature = "strict"))] Default::default()
                        }
                    };
                    let m_ragdollName = match m_ragdollName {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            #[cfg(feature = "strict")]
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "ragdollName",
                                ),
                            );
                            #[cfg(not(feature = "strict"))] Default::default()
                        }
                    };
                    let m_behaviorFilename = match m_behaviorFilename {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            #[cfg(feature = "strict")]
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "behaviorFilename",
                                ),
                            );
                            #[cfg(not(feature = "strict"))] Default::default()
                        }
                    };
                    let m_luaScriptOnCharacterActivated = match m_luaScriptOnCharacterActivated {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            #[cfg(feature = "strict")]
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "luaScriptOnCharacterActivated",
                                ),
                            );
                            #[cfg(not(feature = "strict"))] Default::default()
                        }
                    };
                    let m_luaScriptOnCharacterDeactivated = match m_luaScriptOnCharacterDeactivated {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            #[cfg(feature = "strict")]
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "luaScriptOnCharacterDeactivated",
                                ),
                            );
                            #[cfg(not(feature = "strict"))] Default::default()
                        }
                    };
                    let m_luaFiles = match m_luaFiles {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            #[cfg(feature = "strict")]
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field("luaFiles"),
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
                    _serde::__private::Ok(hkbCharacterStringData {
                        __ptr: __ptr.clone(),
                        parent,
                        m_skinNames,
                        m_boneAttachmentNames,
                        m_animationBundleNameData,
                        m_animationBundleFilenameData,
                        m_characterPropertyNames,
                        m_retargetingSkeletonMapperFilenames,
                        m_lodNames,
                        m_mirroredSyncPointSubstringsA,
                        m_mirroredSyncPointSubstringsB,
                        m_name,
                        m_rigName,
                        m_ragdollName,
                        m_behaviorFilename,
                        m_luaScriptOnCharacterActivated,
                        m_luaScriptOnCharacterDeactivated,
                        m_luaFiles,
                    })
                }
            }
            const FIELDS: &[&str] = &[
                "skinNames",
                "boneAttachmentNames",
                "animationBundleNameData",
                "animationBundleFilenameData",
                "characterPropertyNames",
                "retargetingSkeletonMapperFilenames",
                "lodNames",
                "mirroredSyncPointSubstringsA",
                "mirroredSyncPointSubstringsB",
                "name",
                "rigName",
                "ragdollName",
                "behaviorFilename",
                "luaScriptOnCharacterActivated",
                "luaScriptOnCharacterDeactivated",
                "luaFiles",
            ];
            _serde::Deserializer::deserialize_struct(
                deserializer,
                "hkbCharacterStringData",
                FIELDS,
                __hkbCharacterStringDataVisitor {
                    marker: _serde::__private::PhantomData::<hkbCharacterStringData>,
                    lifetime: _serde::__private::PhantomData,
                },
            )
        }
    }
};
