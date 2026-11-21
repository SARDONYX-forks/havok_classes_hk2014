use super::class_requires::*;
use super::*;
/// # C++ Info
/// - name: `hkbCustomTestGenerator`
/// - version: `4`
/// - signature: `0x4aa6a88c`
/// - size: `  0`(x86)/`1680`(x86_64)
/// -  vtable: `true`
#[allow(non_upper_case_globals, non_snake_case)]
#[cfg_attr(feature = "json_schema", derive(schemars::JsonSchema))]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(educe::Educe)]
#[educe(Debug, Clone, Default, PartialEq)]
pub struct hkbCustomTestGenerator<'a> {
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
    pub parent: hkbCustomTestGeneratorAnnotatedTypes<'a>,
    /// # C++ Info
    /// - name: `protectedHiddenMember`(ctype: `hkBool`)
    /// - offset: `  0`(x86)/`1664`(x86_64)
    /// - type_size: `  0`(x86)/`  1`(x86_64)
    #[cfg_attr(feature = "serde_default", serde(default))]
    #[cfg_attr(feature = "json_schema", schemars(rename = "protectedHiddenMember"))]
    #[cfg_attr(feature = "serde", serde(rename = "protectedHiddenMember"))]
    pub m_protectedHiddenMember: bool,
    /// # C++ Info
    /// - name: `privateHiddenMember`(ctype: `hkBool`)
    /// - offset: `  0`(x86)/`1665`(x86_64)
    /// - type_size: `  0`(x86)/`  1`(x86_64)
    #[cfg_attr(feature = "serde_default", serde(default))]
    #[cfg_attr(feature = "json_schema", schemars(rename = "privateHiddenMember"))]
    #[cfg_attr(feature = "serde", serde(rename = "privateHiddenMember"))]
    pub m_privateHiddenMember: bool,
}
const _: () = {
    use havok_serde as _serde;
    impl<'a> _serde::HavokClass for hkbCustomTestGenerator<'a> {
        #[inline]
        fn name(&self) -> &'static str {
            "hkbCustomTestGenerator"
        }
        #[inline]
        fn signature(&self) -> _serde::__private::Signature {
            _serde::__private::Signature::new(0x4aa6a88c)
        }
        #[allow(clippy::let_and_return, clippy::vec_init_then_push)]
        fn deps_indexes(&self) -> Vec<&Pointer<'_>> {
            let mut v = Vec::new();
            v.push(
                &self
                    .parent
                    .parent
                    .parent
                    .parent
                    .parent
                    .parent
                    .parent
                    .parent
                    .parent
                    .parent
                    .parent
                    .m_variableBindingSet,
            );
            v.extend(
                self
                    .parent
                    .parent
                    .parent
                    .parent
                    .parent
                    .parent
                    .parent
                    .parent
                    .parent
                    .parent
                    .parent
                    .m_cachedBindables
                    .iter(),
            );
            v.push(
                &self
                    .parent
                    .parent
                    .parent
                    .parent
                    .parent
                    .parent
                    .parent
                    .parent
                    .parent
                    .parent
                    .m_nodeInfo,
            );
            v.extend(
                self
                    .parent
                    .parent
                    .parent
                    .parent
                    .parent
                    .parent
                    .parent
                    .parent
                    .parent
                    .m_partitionInfo
                    .deps_indexes(),
            );
            v.push(
                &self
                    .parent
                    .parent
                    .parent
                    .parent
                    .parent
                    .parent
                    .parent
                    .parent
                    .parent
                    .m_syncInfo,
            );
            v.push(
                &self.parent.parent.parent.parent.parent.parent.parent.parent.m_skeleton,
            );
            v.push(&self.parent.parent.parent.parent.parent.m_complexTypeHkObjectPtr);
            v.push(&self.parent.parent.parent.parent.m_nestedTypeHkbGeneratorPtr);
            v.push(&self.parent.parent.parent.parent.m_nestedTypeHkbGeneratorRefPtr);
            v.push(&self.parent.parent.parent.parent.m_nestedTypeHkbModifierPtr);
            v.push(&self.parent.parent.parent.parent.m_nestedTypeHkbModifierRefPtr);
            v.push(&self.parent.parent.parent.parent.m_nestedTypeHkbCustomIdSelectorPtr);
            v.push(
                &self.parent.parent.parent.parent.m_nestedTypeHkbCustomIdSelectorRefPtr,
            );
            v.extend(
                self.parent.parent.parent.parent.m_nestedTypeArrayHkbGeneratorPtr.iter(),
            );
            v.extend(
                self
                    .parent
                    .parent
                    .parent
                    .parent
                    .m_nestedTypeArrayHkbGeneratorRefPtr
                    .iter(),
            );
            v.extend(
                self.parent.parent.parent.parent.m_nestedTypeArrayHkbModifierPtr.iter(),
            );
            v.extend(
                self
                    .parent
                    .parent
                    .parent
                    .parent
                    .m_nestedTypeArrayHkbModifierRefPtr
                    .iter(),
            );
            v.extend(
                self
                    .parent
                    .parent
                    .parent
                    .parent
                    .m_nestedTypeArrayHkbCustomIdSelectorPtr
                    .iter(),
            );
            v.extend(
                self
                    .parent
                    .parent
                    .parent
                    .parent
                    .m_nestedTypeArrayHkbCustomIdSelectorRefPtr
                    .iter(),
            );
            v.extend(self.parent.parent.parent.m_nestedTypeStruct.deps_indexes());
            v.extend(
                self
                    .parent
                    .parent
                    .parent
                    .m_nestedTypeArrayStruct
                    .iter()
                    .flat_map(|class| class.deps_indexes())
                    .collect::<Vec<&Pointer<'_>>>(),
            );
            v.push(&self.parent.parent.m_boneWeightArray);
            v.push(&self.parent.parent.m_boneIndexArray);
            v
        }
    }
    impl<'a> _serde::Serialize for hkbCustomTestGenerator<'a> {
        fn serialize<S>(&self, __serializer: S) -> Result<S::Ok, S::Error>
        where
            S: _serde::ser::Serializer,
        {
            let class_meta = self
                .__ptr
                .as_ref()
                .map(|name| (name, _serde::__private::Signature::new(0x4aa6a88c)));
            let mut serializer = __serializer
                .serialize_struct(
                    "hkbCustomTestGenerator",
                    class_meta,
                    (0u64, 1680u64),
                )?;
            serializer.pad_field([0u8; 0usize].as_slice(), [0u8; 8usize].as_slice())?;
            serializer
                .skip_field(
                    "memSizeAndRefCount",
                    &self
                        .parent
                        .parent
                        .parent
                        .parent
                        .parent
                        .parent
                        .parent
                        .parent
                        .parent
                        .parent
                        .parent
                        .parent
                        .m_memSizeAndRefCount,
                )?;
            serializer.pad_field([0u8; 0usize].as_slice(), [0u8; 4usize].as_slice())?;
            serializer
                .serialize_field(
                    "variableBindingSet",
                    &self
                        .parent
                        .parent
                        .parent
                        .parent
                        .parent
                        .parent
                        .parent
                        .parent
                        .parent
                        .parent
                        .parent
                        .m_variableBindingSet,
                )?;
            serializer
                .skip_array_field(
                    "cachedBindables",
                    &self
                        .parent
                        .parent
                        .parent
                        .parent
                        .parent
                        .parent
                        .parent
                        .parent
                        .parent
                        .parent
                        .parent
                        .m_cachedBindables,
                    TypeSize::NonPtr,
                )?;
            serializer
                .skip_field(
                    "areBindablesCached",
                    &self
                        .parent
                        .parent
                        .parent
                        .parent
                        .parent
                        .parent
                        .parent
                        .parent
                        .parent
                        .parent
                        .parent
                        .m_areBindablesCached,
                )?;
            serializer
                .skip_field(
                    "hasEnableChanged",
                    &self
                        .parent
                        .parent
                        .parent
                        .parent
                        .parent
                        .parent
                        .parent
                        .parent
                        .parent
                        .parent
                        .parent
                        .m_hasEnableChanged,
                )?;
            serializer.pad_field([0u8; 0usize].as_slice(), [0u8; 6usize].as_slice())?;
            serializer
                .serialize_field(
                    "userData",
                    &self
                        .parent
                        .parent
                        .parent
                        .parent
                        .parent
                        .parent
                        .parent
                        .parent
                        .parent
                        .parent
                        .m_userData,
                )?;
            serializer
                .serialize_field(
                    "name",
                    &self
                        .parent
                        .parent
                        .parent
                        .parent
                        .parent
                        .parent
                        .parent
                        .parent
                        .parent
                        .parent
                        .m_name,
                )?;
            serializer
                .skip_field(
                    "id",
                    &self
                        .parent
                        .parent
                        .parent
                        .parent
                        .parent
                        .parent
                        .parent
                        .parent
                        .parent
                        .parent
                        .m_id,
                )?;
            serializer
                .skip_field(
                    "cloneState",
                    &self
                        .parent
                        .parent
                        .parent
                        .parent
                        .parent
                        .parent
                        .parent
                        .parent
                        .parent
                        .parent
                        .m_cloneState,
                )?;
            serializer
                .skip_field(
                    "type",
                    &self
                        .parent
                        .parent
                        .parent
                        .parent
                        .parent
                        .parent
                        .parent
                        .parent
                        .parent
                        .parent
                        .m_type,
                )?;
            serializer.pad_field([0u8; 0usize].as_slice(), [0u8; 4usize].as_slice())?;
            serializer
                .skip_field(
                    "nodeInfo",
                    &self
                        .parent
                        .parent
                        .parent
                        .parent
                        .parent
                        .parent
                        .parent
                        .parent
                        .parent
                        .parent
                        .m_nodeInfo,
                )?;
            serializer
                .skip_field(
                    "partitionInfo",
                    &self
                        .parent
                        .parent
                        .parent
                        .parent
                        .parent
                        .parent
                        .parent
                        .parent
                        .parent
                        .m_partitionInfo,
                )?;
            serializer
                .skip_field(
                    "syncInfo",
                    &self
                        .parent
                        .parent
                        .parent
                        .parent
                        .parent
                        .parent
                        .parent
                        .parent
                        .parent
                        .m_syncInfo,
                )?;
            serializer
                .skip_fixed_array_field(
                    "pad",
                    self
                        .parent
                        .parent
                        .parent
                        .parent
                        .parent
                        .parent
                        .parent
                        .parent
                        .parent
                        .m_pad
                        .as_slice(),
                    TypeSize::NonPtr,
                )?;
            serializer.pad_field([0u8; 0usize].as_slice(), [0u8; 4usize].as_slice())?;
            serializer
                .skip_field(
                    "skeleton",
                    &self
                        .parent
                        .parent
                        .parent
                        .parent
                        .parent
                        .parent
                        .parent
                        .parent
                        .m_skeleton,
                )?;
            serializer
                .serialize_field(
                    "inheritedHiddenMember",
                    &self
                        .parent
                        .parent
                        .parent
                        .parent
                        .parent
                        .parent
                        .parent
                        .m_inheritedHiddenMember,
                )?;
            serializer
                .serialize_field(
                    "protectedInheritedHiddenMember",
                    &self
                        .parent
                        .parent
                        .parent
                        .parent
                        .parent
                        .parent
                        .parent
                        .m_protectedInheritedHiddenMember,
                )?;
            serializer
                .serialize_field(
                    "privateInheritedHiddenMember",
                    &self
                        .parent
                        .parent
                        .parent
                        .parent
                        .parent
                        .parent
                        .parent
                        .m_privateInheritedHiddenMember,
                )?;
            serializer.pad_field([0u8; 0usize].as_slice(), [0u8; 5usize].as_slice())?;
            serializer
                .serialize_field(
                    "simpleTypeHkInt64",
                    &self.parent.parent.parent.parent.parent.parent.m_simpleTypeHkInt64,
                )?;
            serializer
                .serialize_field(
                    "simpleTypeHkUint64",
                    &self.parent.parent.parent.parent.parent.parent.m_simpleTypeHkUint64,
                )?;
            serializer
                .serialize_field(
                    "simpleHiddenTypeCopyStart",
                    &self
                        .parent
                        .parent
                        .parent
                        .parent
                        .parent
                        .parent
                        .m_simpleHiddenTypeCopyStart,
                )?;
            serializer
                .serialize_field(
                    "simpleTypeBool",
                    &self.parent.parent.parent.parent.parent.parent.m_simpleTypeBool,
                )?;
            serializer
                .serialize_field(
                    "simpleTypeHkBool",
                    &self.parent.parent.parent.parent.parent.parent.m_simpleTypeHkBool,
                )?;
            serializer.pad_field([0u8; 0usize].as_slice(), [0u8; 5usize].as_slice())?;
            serializer
                .serialize_field(
                    "simpleTypeCString",
                    &self.parent.parent.parent.parent.parent.parent.m_simpleTypeCString,
                )?;
            serializer
                .serialize_field(
                    "simpleTypeHkStringPtr",
                    &self
                        .parent
                        .parent
                        .parent
                        .parent
                        .parent
                        .parent
                        .m_simpleTypeHkStringPtr,
                )?;
            serializer
                .serialize_field(
                    "simpleTypeHkInt8",
                    &self.parent.parent.parent.parent.parent.parent.m_simpleTypeHkInt8,
                )?;
            serializer.pad_field([0u8; 0usize].as_slice(), [0u8; 1usize].as_slice())?;
            serializer
                .serialize_field(
                    "simpleTypeHkInt16",
                    &self.parent.parent.parent.parent.parent.parent.m_simpleTypeHkInt16,
                )?;
            serializer
                .serialize_field(
                    "simpleTypeHkInt32",
                    &self.parent.parent.parent.parent.parent.parent.m_simpleTypeHkInt32,
                )?;
            serializer
                .serialize_field(
                    "simpleTypeHkUint8",
                    &self.parent.parent.parent.parent.parent.parent.m_simpleTypeHkUint8,
                )?;
            serializer.pad_field([0u8; 0usize].as_slice(), [0u8; 1usize].as_slice())?;
            serializer
                .serialize_field(
                    "simpleTypeHkUint16",
                    &self.parent.parent.parent.parent.parent.parent.m_simpleTypeHkUint16,
                )?;
            serializer
                .serialize_field(
                    "simpleTypeHkUint32",
                    &self.parent.parent.parent.parent.parent.parent.m_simpleTypeHkUint32,
                )?;
            serializer
                .serialize_field(
                    "simpleTypeHkReal",
                    &self.parent.parent.parent.parent.parent.parent.m_simpleTypeHkReal,
                )?;
            serializer
                .serialize_field(
                    "simpleTypeHkInt8Default",
                    &self
                        .parent
                        .parent
                        .parent
                        .parent
                        .parent
                        .parent
                        .m_simpleTypeHkInt8Default,
                )?;
            serializer.pad_field([0u8; 0usize].as_slice(), [0u8; 1usize].as_slice())?;
            serializer
                .serialize_field(
                    "simpleTypeHkInt16Default",
                    &self
                        .parent
                        .parent
                        .parent
                        .parent
                        .parent
                        .parent
                        .m_simpleTypeHkInt16Default,
                )?;
            serializer
                .serialize_field(
                    "simpleTypeHkInt32Default",
                    &self
                        .parent
                        .parent
                        .parent
                        .parent
                        .parent
                        .parent
                        .m_simpleTypeHkInt32Default,
                )?;
            serializer
                .serialize_field(
                    "simpleTypeHkUint8Default",
                    &self
                        .parent
                        .parent
                        .parent
                        .parent
                        .parent
                        .parent
                        .m_simpleTypeHkUint8Default,
                )?;
            serializer.pad_field([0u8; 0usize].as_slice(), [0u8; 1usize].as_slice())?;
            serializer
                .serialize_field(
                    "simpleTypeHkUint16Default",
                    &self
                        .parent
                        .parent
                        .parent
                        .parent
                        .parent
                        .parent
                        .m_simpleTypeHkUint16Default,
                )?;
            serializer
                .serialize_field(
                    "simpleTypeHkUint32Default",
                    &self
                        .parent
                        .parent
                        .parent
                        .parent
                        .parent
                        .parent
                        .m_simpleTypeHkUint32Default,
                )?;
            serializer
                .serialize_field(
                    "simpleTypeHkRealDefault",
                    &self
                        .parent
                        .parent
                        .parent
                        .parent
                        .parent
                        .parent
                        .m_simpleTypeHkRealDefault,
                )?;
            serializer
                .serialize_field(
                    "simpleTypeHkInt8Clamp",
                    &self
                        .parent
                        .parent
                        .parent
                        .parent
                        .parent
                        .parent
                        .m_simpleTypeHkInt8Clamp,
                )?;
            serializer.pad_field([0u8; 0usize].as_slice(), [0u8; 1usize].as_slice())?;
            serializer
                .serialize_field(
                    "simpleTypeHkInt16Clamp",
                    &self
                        .parent
                        .parent
                        .parent
                        .parent
                        .parent
                        .parent
                        .m_simpleTypeHkInt16Clamp,
                )?;
            serializer
                .serialize_field(
                    "simpleTypeHkInt32Clamp",
                    &self
                        .parent
                        .parent
                        .parent
                        .parent
                        .parent
                        .parent
                        .m_simpleTypeHkInt32Clamp,
                )?;
            serializer
                .serialize_field(
                    "simpleTypeHkUint8Clamp",
                    &self
                        .parent
                        .parent
                        .parent
                        .parent
                        .parent
                        .parent
                        .m_simpleTypeHkUint8Clamp,
                )?;
            serializer.pad_field([0u8; 0usize].as_slice(), [0u8; 1usize].as_slice())?;
            serializer
                .serialize_field(
                    "simpleTypeHkUint16Clamp",
                    &self
                        .parent
                        .parent
                        .parent
                        .parent
                        .parent
                        .parent
                        .m_simpleTypeHkUint16Clamp,
                )?;
            serializer
                .serialize_field(
                    "simpleTypeHkUint32Clamp",
                    &self
                        .parent
                        .parent
                        .parent
                        .parent
                        .parent
                        .parent
                        .m_simpleTypeHkUint32Clamp,
                )?;
            serializer
                .serialize_field(
                    "simpleTypeHkRealClamp",
                    &self
                        .parent
                        .parent
                        .parent
                        .parent
                        .parent
                        .parent
                        .m_simpleTypeHkRealClamp,
                )?;
            serializer
                .serialize_field(
                    "simpleHiddenTypeCopyEnd",
                    &self
                        .parent
                        .parent
                        .parent
                        .parent
                        .parent
                        .parent
                        .m_simpleHiddenTypeCopyEnd,
                )?;
            serializer.pad_field([0u8; 0usize].as_slice(), [0u8; 3usize].as_slice())?;
            serializer
                .serialize_field(
                    "complexTypeHkObjectPtr",
                    &self.parent.parent.parent.parent.parent.m_complexTypeHkObjectPtr,
                )?;
            serializer
                .serialize_field(
                    "complexHiddenTypeCopyStart",
                    &self.parent.parent.parent.parent.parent.m_complexHiddenTypeCopyStart,
                )?;
            serializer.pad_field([0u8; 0usize].as_slice(), [0u8; 7usize].as_slice())?;
            serializer
                .serialize_field(
                    "complexTypeHkQuaternion",
                    &self.parent.parent.parent.parent.parent.m_complexTypeHkQuaternion,
                )?;
            serializer
                .serialize_field(
                    "complexTypeHkVector4",
                    &self.parent.parent.parent.parent.parent.m_complexTypeHkVector4,
                )?;
            serializer
                .serialize_field(
                    "complexTypeEnumHkInt8",
                    &self.parent.parent.parent.parent.parent.m_complexTypeEnumHkInt8,
                )?;
            serializer.pad_field([0u8; 0usize].as_slice(), [0u8; 1usize].as_slice())?;
            serializer
                .serialize_field(
                    "complexTypeEnumHkInt16",
                    &self.parent.parent.parent.parent.parent.m_complexTypeEnumHkInt16,
                )?;
            serializer
                .serialize_field(
                    "complexTypeEnumHkInt32",
                    &self.parent.parent.parent.parent.parent.m_complexTypeEnumHkInt32,
                )?;
            serializer
                .serialize_field(
                    "complexTypeEnumHkUint8",
                    &self.parent.parent.parent.parent.parent.m_complexTypeEnumHkUint8,
                )?;
            serializer.pad_field([0u8; 0usize].as_slice(), [0u8; 1usize].as_slice())?;
            serializer
                .serialize_field(
                    "complexTypeEnumHkUint16",
                    &self.parent.parent.parent.parent.parent.m_complexTypeEnumHkUint16,
                )?;
            serializer
                .serialize_field(
                    "complexTypeEnumHkUint32",
                    &self.parent.parent.parent.parent.parent.m_complexTypeEnumHkUint32,
                )?;
            serializer
                .serialize_field(
                    "complexTypeEnumHkInt8InvalidCheck",
                    &self
                        .parent
                        .parent
                        .parent
                        .parent
                        .parent
                        .m_complexTypeEnumHkInt8InvalidCheck,
                )?;
            serializer.pad_field([0u8; 0usize].as_slice(), [0u8; 1usize].as_slice())?;
            serializer
                .serialize_field(
                    "complexTypeEnumHkInt16InvalidCheck",
                    &self
                        .parent
                        .parent
                        .parent
                        .parent
                        .parent
                        .m_complexTypeEnumHkInt16InvalidCheck,
                )?;
            serializer
                .serialize_field(
                    "complexTypeEnumHkInt32InvalidCheck",
                    &self
                        .parent
                        .parent
                        .parent
                        .parent
                        .parent
                        .m_complexTypeEnumHkInt32InvalidCheck,
                )?;
            serializer
                .serialize_field(
                    "complexTypeEnumHkUint8InvalidCheck",
                    &self
                        .parent
                        .parent
                        .parent
                        .parent
                        .parent
                        .m_complexTypeEnumHkUint8InvalidCheck,
                )?;
            serializer.pad_field([0u8; 0usize].as_slice(), [0u8; 1usize].as_slice())?;
            serializer
                .serialize_field(
                    "complexTypeEnumHkUint16InvalidCheck",
                    &self
                        .parent
                        .parent
                        .parent
                        .parent
                        .parent
                        .m_complexTypeEnumHkUint16InvalidCheck,
                )?;
            serializer
                .serialize_field(
                    "complexTypeEnumHkUint32InvalidCheck",
                    &self
                        .parent
                        .parent
                        .parent
                        .parent
                        .parent
                        .m_complexTypeEnumHkUint32InvalidCheck,
                )?;
            serializer
                .serialize_field(
                    "complexTypeFlagsHkInt8",
                    &self.parent.parent.parent.parent.parent.m_complexTypeFlagsHkInt8,
                )?;
            serializer.pad_field([0u8; 0usize].as_slice(), [0u8; 1usize].as_slice())?;
            serializer
                .serialize_field(
                    "complexTypeFlagsHkInt16",
                    &self.parent.parent.parent.parent.parent.m_complexTypeFlagsHkInt16,
                )?;
            serializer
                .serialize_field(
                    "complexTypeFlagsHkInt32",
                    &self.parent.parent.parent.parent.parent.m_complexTypeFlagsHkInt32,
                )?;
            serializer
                .serialize_field(
                    "complexTypeFlagsHkUint8",
                    &self.parent.parent.parent.parent.parent.m_complexTypeFlagsHkUint8,
                )?;
            serializer.pad_field([0u8; 0usize].as_slice(), [0u8; 1usize].as_slice())?;
            serializer
                .serialize_field(
                    "complexTypeFlagsHkUint16",
                    &self.parent.parent.parent.parent.parent.m_complexTypeFlagsHkUint16,
                )?;
            serializer
                .serialize_field(
                    "complexTypeFlagsHkUint32",
                    &self.parent.parent.parent.parent.parent.m_complexTypeFlagsHkUint32,
                )?;
            serializer
                .serialize_field(
                    "complexTypeFlagsHkInt8InvalidCheck",
                    &self
                        .parent
                        .parent
                        .parent
                        .parent
                        .parent
                        .m_complexTypeFlagsHkInt8InvalidCheck,
                )?;
            serializer.pad_field([0u8; 0usize].as_slice(), [0u8; 1usize].as_slice())?;
            serializer
                .serialize_field(
                    "complexTypeFlagsHkInt16InvalidCheck",
                    &self
                        .parent
                        .parent
                        .parent
                        .parent
                        .parent
                        .m_complexTypeFlagsHkInt16InvalidCheck,
                )?;
            serializer
                .serialize_field(
                    "complexTypeFlagsHkInt32InvalidCheck",
                    &self
                        .parent
                        .parent
                        .parent
                        .parent
                        .parent
                        .m_complexTypeFlagsHkInt32InvalidCheck,
                )?;
            serializer
                .serialize_field(
                    "complexTypeFlagsHkUint8InvalidCheck",
                    &self
                        .parent
                        .parent
                        .parent
                        .parent
                        .parent
                        .m_complexTypeFlagsHkUint8InvalidCheck,
                )?;
            serializer.pad_field([0u8; 0usize].as_slice(), [0u8; 1usize].as_slice())?;
            serializer
                .serialize_field(
                    "complexTypeFlagsHkUint16InvalidCheck",
                    &self
                        .parent
                        .parent
                        .parent
                        .parent
                        .parent
                        .m_complexTypeFlagsHkUint16InvalidCheck,
                )?;
            serializer
                .serialize_field(
                    "complexTypeFlagsHkUint32InvalidCheck",
                    &self
                        .parent
                        .parent
                        .parent
                        .parent
                        .parent
                        .m_complexTypeFlagsHkUint32InvalidCheck,
                )?;
            serializer
                .serialize_field(
                    "complexHiddenTypeCopyEnd",
                    &self.parent.parent.parent.parent.parent.m_complexHiddenTypeCopyEnd,
                )?;
            serializer.pad_field([0u8; 0usize].as_slice(), [0u8; 15usize].as_slice())?;
            serializer
                .serialize_field(
                    "nestedTypeHkbGeneratorPtr",
                    &self.parent.parent.parent.parent.m_nestedTypeHkbGeneratorPtr,
                )?;
            serializer
                .serialize_field(
                    "nestedTypeHkbGeneratorRefPtr",
                    &self.parent.parent.parent.parent.m_nestedTypeHkbGeneratorRefPtr,
                )?;
            serializer
                .serialize_field(
                    "nestedTypeHkbModifierPtr",
                    &self.parent.parent.parent.parent.m_nestedTypeHkbModifierPtr,
                )?;
            serializer
                .serialize_field(
                    "nestedTypeHkbModifierRefPtr",
                    &self.parent.parent.parent.parent.m_nestedTypeHkbModifierRefPtr,
                )?;
            serializer
                .serialize_field(
                    "nestedTypeHkbCustomIdSelectorPtr",
                    &self.parent.parent.parent.parent.m_nestedTypeHkbCustomIdSelectorPtr,
                )?;
            serializer
                .serialize_field(
                    "nestedTypeHkbCustomIdSelectorRefPtr",
                    &self
                        .parent
                        .parent
                        .parent
                        .parent
                        .m_nestedTypeHkbCustomIdSelectorRefPtr,
                )?;
            serializer
                .serialize_array_field(
                    "nestedTypeArrayBool",
                    &self.parent.parent.parent.parent.m_nestedTypeArrayBool,
                    TypeSize::NonPtr,
                )?;
            serializer
                .serialize_array_field(
                    "nestedTypeArrayHkBool",
                    &self.parent.parent.parent.parent.m_nestedTypeArrayHkBool,
                    TypeSize::NonPtr,
                )?;
            serializer
                .serialize_array_field(
                    "nestedTypeArrayCString",
                    &self.parent.parent.parent.parent.m_nestedTypeArrayCString,
                    TypeSize::String,
                )?;
            serializer
                .serialize_array_field(
                    "nestedTypeArrayHkStringPtr",
                    &self.parent.parent.parent.parent.m_nestedTypeArrayHkStringPtr,
                    TypeSize::String,
                )?;
            serializer
                .serialize_array_field(
                    "nestedTypeArrayHkInt8",
                    &self.parent.parent.parent.parent.m_nestedTypeArrayHkInt8,
                    TypeSize::NonPtr,
                )?;
            serializer
                .serialize_array_field(
                    "nestedTypeArrayHkInt16",
                    &self.parent.parent.parent.parent.m_nestedTypeArrayHkInt16,
                    TypeSize::NonPtr,
                )?;
            serializer
                .serialize_array_field(
                    "nestedTypeArrayHkInt32",
                    &self.parent.parent.parent.parent.m_nestedTypeArrayHkInt32,
                    TypeSize::NonPtr,
                )?;
            serializer
                .serialize_array_field(
                    "nestedTypeArrayHkUint8",
                    &self.parent.parent.parent.parent.m_nestedTypeArrayHkUint8,
                    TypeSize::NonPtr,
                )?;
            serializer
                .serialize_array_field(
                    "nestedTypeArrayHkUint16",
                    &self.parent.parent.parent.parent.m_nestedTypeArrayHkUint16,
                    TypeSize::NonPtr,
                )?;
            serializer
                .serialize_array_field(
                    "nestedTypeArrayHkUint32",
                    &self.parent.parent.parent.parent.m_nestedTypeArrayHkUint32,
                    TypeSize::NonPtr,
                )?;
            serializer
                .serialize_array_field(
                    "nestedTypeArrayHkReal",
                    &self.parent.parent.parent.parent.m_nestedTypeArrayHkReal,
                    TypeSize::NonPtr,
                )?;
            serializer
                .serialize_array_field(
                    "nestedTypeArrayHkbGeneratorPtr",
                    &self.parent.parent.parent.parent.m_nestedTypeArrayHkbGeneratorPtr,
                    TypeSize::NonPtr,
                )?;
            serializer
                .serialize_array_field(
                    "nestedTypeArrayHkbGeneratorRefPtr",
                    &self
                        .parent
                        .parent
                        .parent
                        .parent
                        .m_nestedTypeArrayHkbGeneratorRefPtr,
                    TypeSize::NonPtr,
                )?;
            serializer
                .serialize_array_field(
                    "nestedTypeArrayHkbModifierPtr",
                    &self.parent.parent.parent.parent.m_nestedTypeArrayHkbModifierPtr,
                    TypeSize::NonPtr,
                )?;
            serializer
                .serialize_array_field(
                    "nestedTypeArrayHkbModifierRefPtr",
                    &self.parent.parent.parent.parent.m_nestedTypeArrayHkbModifierRefPtr,
                    TypeSize::NonPtr,
                )?;
            serializer
                .serialize_array_field(
                    "nestedTypeArrayHkbCustomIdSelectorPtr",
                    &self
                        .parent
                        .parent
                        .parent
                        .parent
                        .m_nestedTypeArrayHkbCustomIdSelectorPtr,
                    TypeSize::NonPtr,
                )?;
            serializer
                .serialize_array_field(
                    "nestedTypeArrayHkbCustomIdSelectorRefPtr",
                    &self
                        .parent
                        .parent
                        .parent
                        .parent
                        .m_nestedTypeArrayHkbCustomIdSelectorRefPtr,
                    TypeSize::NonPtr,
                )?;
            serializer
                .serialize_field(
                    "nestedTypeStruct",
                    &self.parent.parent.parent.m_nestedTypeStruct,
                )?;
            serializer
                .serialize_array_field(
                    "nestedTypeArrayStruct",
                    &self.parent.parent.parent.m_nestedTypeArrayStruct,
                    TypeSize::Struct {
                        size_x86: 0u64,
                        size_x86_64: 704u64,
                    },
                )?;
            serializer
                .serialize_field(
                    "boneHiddenTypeCopyStart",
                    &self.parent.parent.m_boneHiddenTypeCopyStart,
                )?;
            serializer.pad_field([0u8; 0usize].as_slice(), [0u8; 1usize].as_slice())?;
            serializer
                .serialize_field("oldBoneIndex", &self.parent.parent.m_oldBoneIndex)?;
            serializer
                .serialize_field(
                    "oldBoneIndexNoVar",
                    &self.parent.parent.m_oldBoneIndexNoVar,
                )?;
            serializer.serialize_field("boneIndex", &self.parent.parent.m_boneIndex)?;
            serializer
                .serialize_field(
                    "boneIndexNoVar",
                    &self.parent.parent.m_boneIndexNoVar,
                )?;
            serializer
                .serialize_field(
                    "boneChainIndex0",
                    &self.parent.parent.m_boneChainIndex0,
                )?;
            serializer
                .serialize_field(
                    "boneChainIndex1",
                    &self.parent.parent.m_boneChainIndex1,
                )?;
            serializer
                .serialize_field(
                    "boneChainIndex2",
                    &self.parent.parent.m_boneChainIndex2,
                )?;
            serializer
                .serialize_field(
                    "boneContractIndex0",
                    &self.parent.parent.m_boneContractIndex0,
                )?;
            serializer
                .serialize_field(
                    "boneContractIndex1",
                    &self.parent.parent.m_boneContractIndex1,
                )?;
            serializer
                .serialize_field(
                    "boneContractIndex2",
                    &self.parent.parent.m_boneContractIndex2,
                )?;
            serializer
                .serialize_field(
                    "boneHiddenTypeCopyEnd",
                    &self.parent.parent.m_boneHiddenTypeCopyEnd,
                )?;
            serializer.pad_field([0u8; 0usize].as_slice(), [0u8; 1usize].as_slice())?;
            serializer
                .serialize_field(
                    "boneWeightArray",
                    &self.parent.parent.m_boneWeightArray,
                )?;
            serializer
                .serialize_field(
                    "boneIndexArray",
                    &self.parent.parent.m_boneIndexArray,
                )?;
            serializer.pad_field([0u8; 0usize].as_slice(), [0u8; 8usize].as_slice())?;
            serializer
                .serialize_field(
                    "annotatedTypeCStringFilename",
                    &self.parent.m_annotatedTypeCStringFilename,
                )?;
            serializer
                .serialize_field(
                    "annotatedTypeHkStringPtrFilename",
                    &self.parent.m_annotatedTypeHkStringPtrFilename,
                )?;
            serializer
                .serialize_field(
                    "annotatedTypeCStringScript",
                    &self.parent.m_annotatedTypeCStringScript,
                )?;
            serializer
                .serialize_field(
                    "annotatedTypeHkStringPtrScript",
                    &self.parent.m_annotatedTypeHkStringPtrScript,
                )?;
            serializer
                .serialize_field(
                    "annotatedTypeCStringBoneAttachment",
                    &self.parent.m_annotatedTypeCStringBoneAttachment,
                )?;
            serializer
                .serialize_field(
                    "annotatedTypeHkStringPtrBoneAttachment",
                    &self.parent.m_annotatedTypeHkStringPtrBoneAttachment,
                )?;
            serializer
                .serialize_field(
                    "annotatedTypeCStringLocalFrame",
                    &self.parent.m_annotatedTypeCStringLocalFrame,
                )?;
            serializer
                .serialize_field(
                    "annotatedTypeHkStringPtrLocalFrame",
                    &self.parent.m_annotatedTypeHkStringPtrLocalFrame,
                )?;
            serializer
                .serialize_field(
                    "annotatedHiddenTypeCopyStart",
                    &self.parent.m_annotatedHiddenTypeCopyStart,
                )?;
            serializer.pad_field([0u8; 0usize].as_slice(), [0u8; 3usize].as_slice())?;
            serializer
                .serialize_field(
                    "annotatedTypeHkInt32EventID",
                    &self.parent.m_annotatedTypeHkInt32EventID,
                )?;
            serializer
                .serialize_field(
                    "annotatedTypeHkInt32VariableIndex",
                    &self.parent.m_annotatedTypeHkInt32VariableIndex,
                )?;
            serializer
                .serialize_field(
                    "annotatedTypeHkInt32AttributeIndex",
                    &self.parent.m_annotatedTypeHkInt32AttributeIndex,
                )?;
            serializer
                .serialize_field(
                    "annotatedTypeHkRealTime",
                    &self.parent.m_annotatedTypeHkRealTime,
                )?;
            serializer
                .serialize_field(
                    "annotatedTypeBoolNoVar",
                    &self.parent.m_annotatedTypeBoolNoVar,
                )?;
            serializer
                .serialize_field(
                    "annotatedTypeHkBoolNoVar",
                    &self.parent.m_annotatedTypeHkBoolNoVar,
                )?;
            serializer
                .serialize_field(
                    "annotatedTypeHkInt8NoVar",
                    &self.parent.m_annotatedTypeHkInt8NoVar,
                )?;
            serializer.pad_field([0u8; 0usize].as_slice(), [0u8; 1usize].as_slice())?;
            serializer
                .serialize_field(
                    "annotatedTypeHkInt16NoVar",
                    &self.parent.m_annotatedTypeHkInt16NoVar,
                )?;
            serializer.pad_field([0u8; 0usize].as_slice(), [0u8; 2usize].as_slice())?;
            serializer
                .serialize_field(
                    "annotatedTypeHkInt32NoVar",
                    &self.parent.m_annotatedTypeHkInt32NoVar,
                )?;
            serializer
                .serialize_field(
                    "annotatedTypeHkUint8NoVar",
                    &self.parent.m_annotatedTypeHkUint8NoVar,
                )?;
            serializer.pad_field([0u8; 0usize].as_slice(), [0u8; 1usize].as_slice())?;
            serializer
                .serialize_field(
                    "annotatedTypeHkUint16NoVar",
                    &self.parent.m_annotatedTypeHkUint16NoVar,
                )?;
            serializer
                .serialize_field(
                    "annotatedTypeHkUint32NoVar",
                    &self.parent.m_annotatedTypeHkUint32NoVar,
                )?;
            serializer
                .serialize_field(
                    "annotatedTypeHkRealNoVar",
                    &self.parent.m_annotatedTypeHkRealNoVar,
                )?;
            serializer
                .serialize_field(
                    "annotatedTypeBoolOutput",
                    &self.parent.m_annotatedTypeBoolOutput,
                )?;
            serializer
                .serialize_field(
                    "annotatedTypeHkBoolOutput",
                    &self.parent.m_annotatedTypeHkBoolOutput,
                )?;
            serializer
                .serialize_field(
                    "annotatedTypeHkInt8Output",
                    &self.parent.m_annotatedTypeHkInt8Output,
                )?;
            serializer.pad_field([0u8; 0usize].as_slice(), [0u8; 1usize].as_slice())?;
            serializer
                .serialize_field(
                    "annotatedTypeHkInt16Output",
                    &self.parent.m_annotatedTypeHkInt16Output,
                )?;
            serializer.pad_field([0u8; 0usize].as_slice(), [0u8; 2usize].as_slice())?;
            serializer
                .serialize_field(
                    "annotatedTypeHkInt32Output",
                    &self.parent.m_annotatedTypeHkInt32Output,
                )?;
            serializer
                .serialize_field(
                    "annotatedTypeHkUint8Output",
                    &self.parent.m_annotatedTypeHkUint8Output,
                )?;
            serializer.pad_field([0u8; 0usize].as_slice(), [0u8; 1usize].as_slice())?;
            serializer
                .serialize_field(
                    "annotatedTypeHkUint16Output",
                    &self.parent.m_annotatedTypeHkUint16Output,
                )?;
            serializer
                .serialize_field(
                    "annotatedTypeHkUint32Output",
                    &self.parent.m_annotatedTypeHkUint32Output,
                )?;
            serializer
                .serialize_field(
                    "annotatedTypeHkRealOutput",
                    &self.parent.m_annotatedTypeHkRealOutput,
                )?;
            serializer
                .serialize_field(
                    "annotatedHiddenTypeBool",
                    &self.parent.m_annotatedHiddenTypeBool,
                )?;
            serializer
                .serialize_field(
                    "annotatedHiddenTypeHkBool",
                    &self.parent.m_annotatedHiddenTypeHkBool,
                )?;
            serializer.pad_field([0u8; 0usize].as_slice(), [0u8; 2usize].as_slice())?;
            serializer
                .serialize_field(
                    "annotatedHiddenTypeCString1",
                    &self.parent.m_annotatedHiddenTypeCString1,
                )?;
            serializer
                .serialize_field(
                    "annotatedHiddenTypeHkStringPtr1",
                    &self.parent.m_annotatedHiddenTypeHkStringPtr1,
                )?;
            serializer
                .serialize_field(
                    "annotatedHiddenTypeCString2",
                    &self.parent.m_annotatedHiddenTypeCString2,
                )?;
            serializer
                .serialize_field(
                    "annotatedHiddenTypeHkStringPtr2",
                    &self.parent.m_annotatedHiddenTypeHkStringPtr2,
                )?;
            serializer
                .serialize_field(
                    "annotatedHiddenTypeHkInt8",
                    &self.parent.m_annotatedHiddenTypeHkInt8,
                )?;
            serializer.pad_field([0u8; 0usize].as_slice(), [0u8; 1usize].as_slice())?;
            serializer
                .serialize_field(
                    "annotatedHiddenTypeHkInt16",
                    &self.parent.m_annotatedHiddenTypeHkInt16,
                )?;
            serializer
                .serialize_field(
                    "annotatedHiddenTypeHkInt32",
                    &self.parent.m_annotatedHiddenTypeHkInt32,
                )?;
            serializer
                .serialize_field(
                    "annotatedHiddenTypeHkUint8",
                    &self.parent.m_annotatedHiddenTypeHkUint8,
                )?;
            serializer.pad_field([0u8; 0usize].as_slice(), [0u8; 1usize].as_slice())?;
            serializer
                .serialize_field(
                    "annotatedHiddenTypeHkUint16",
                    &self.parent.m_annotatedHiddenTypeHkUint16,
                )?;
            serializer
                .serialize_field(
                    "annotatedHiddenTypeHkUint32",
                    &self.parent.m_annotatedHiddenTypeHkUint32,
                )?;
            serializer
                .serialize_field(
                    "annotatedHiddenTypeCopyEnd",
                    &self.parent.m_annotatedHiddenTypeCopyEnd,
                )?;
            serializer.pad_field([0u8; 0usize].as_slice(), [0u8; 7usize].as_slice())?;
            serializer
                .serialize_field(
                    "protectedHiddenMember",
                    &self.m_protectedHiddenMember,
                )?;
            serializer
                .serialize_field("privateHiddenMember", &self.m_privateHiddenMember)?;
            serializer.pad_field([0u8; 0usize].as_slice(), [0u8; 14usize].as_slice())?;
            serializer.end()
        }
    }
};
#[doc(hidden)]
#[allow(non_upper_case_globals, unused_attributes, unused_qualifications)]
const _: () = {
    use havok_serde as _serde;
    #[automatically_derived]
    impl<'de> _serde::Deserialize<'de> for hkbCustomTestGenerator<'de> {
        fn deserialize<__D>(deserializer: __D) -> core::result::Result<Self, __D::Error>
        where
            __D: _serde::Deserializer<'de>,
        {
            #[allow(non_camel_case_types)]
            enum __Field {
                m_variableBindingSet,
                m_userData,
                m_name,
                m_inheritedHiddenMember,
                m_protectedInheritedHiddenMember,
                m_privateInheritedHiddenMember,
                m_simpleTypeHkInt64,
                m_simpleTypeHkUint64,
                m_simpleHiddenTypeCopyStart,
                m_simpleTypeBool,
                m_simpleTypeHkBool,
                m_simpleTypeCString,
                m_simpleTypeHkStringPtr,
                m_simpleTypeHkInt8,
                m_simpleTypeHkInt16,
                m_simpleTypeHkInt32,
                m_simpleTypeHkUint8,
                m_simpleTypeHkUint16,
                m_simpleTypeHkUint32,
                m_simpleTypeHkReal,
                m_simpleTypeHkInt8Default,
                m_simpleTypeHkInt16Default,
                m_simpleTypeHkInt32Default,
                m_simpleTypeHkUint8Default,
                m_simpleTypeHkUint16Default,
                m_simpleTypeHkUint32Default,
                m_simpleTypeHkRealDefault,
                m_simpleTypeHkInt8Clamp,
                m_simpleTypeHkInt16Clamp,
                m_simpleTypeHkInt32Clamp,
                m_simpleTypeHkUint8Clamp,
                m_simpleTypeHkUint16Clamp,
                m_simpleTypeHkUint32Clamp,
                m_simpleTypeHkRealClamp,
                m_simpleHiddenTypeCopyEnd,
                m_complexTypeHkObjectPtr,
                m_complexHiddenTypeCopyStart,
                m_complexTypeHkQuaternion,
                m_complexTypeHkVector4,
                m_complexTypeEnumHkInt8,
                m_complexTypeEnumHkInt16,
                m_complexTypeEnumHkInt32,
                m_complexTypeEnumHkUint8,
                m_complexTypeEnumHkUint16,
                m_complexTypeEnumHkUint32,
                m_complexTypeEnumHkInt8InvalidCheck,
                m_complexTypeEnumHkInt16InvalidCheck,
                m_complexTypeEnumHkInt32InvalidCheck,
                m_complexTypeEnumHkUint8InvalidCheck,
                m_complexTypeEnumHkUint16InvalidCheck,
                m_complexTypeEnumHkUint32InvalidCheck,
                m_complexTypeFlagsHkInt8,
                m_complexTypeFlagsHkInt16,
                m_complexTypeFlagsHkInt32,
                m_complexTypeFlagsHkUint8,
                m_complexTypeFlagsHkUint16,
                m_complexTypeFlagsHkUint32,
                m_complexTypeFlagsHkInt8InvalidCheck,
                m_complexTypeFlagsHkInt16InvalidCheck,
                m_complexTypeFlagsHkInt32InvalidCheck,
                m_complexTypeFlagsHkUint8InvalidCheck,
                m_complexTypeFlagsHkUint16InvalidCheck,
                m_complexTypeFlagsHkUint32InvalidCheck,
                m_complexHiddenTypeCopyEnd,
                m_nestedTypeHkbGeneratorPtr,
                m_nestedTypeHkbGeneratorRefPtr,
                m_nestedTypeHkbModifierPtr,
                m_nestedTypeHkbModifierRefPtr,
                m_nestedTypeHkbCustomIdSelectorPtr,
                m_nestedTypeHkbCustomIdSelectorRefPtr,
                m_nestedTypeArrayBool,
                m_nestedTypeArrayHkBool,
                m_nestedTypeArrayCString,
                m_nestedTypeArrayHkStringPtr,
                m_nestedTypeArrayHkInt8,
                m_nestedTypeArrayHkInt16,
                m_nestedTypeArrayHkInt32,
                m_nestedTypeArrayHkUint8,
                m_nestedTypeArrayHkUint16,
                m_nestedTypeArrayHkUint32,
                m_nestedTypeArrayHkReal,
                m_nestedTypeArrayHkbGeneratorPtr,
                m_nestedTypeArrayHkbGeneratorRefPtr,
                m_nestedTypeArrayHkbModifierPtr,
                m_nestedTypeArrayHkbModifierRefPtr,
                m_nestedTypeArrayHkbCustomIdSelectorPtr,
                m_nestedTypeArrayHkbCustomIdSelectorRefPtr,
                m_nestedTypeStruct,
                m_nestedTypeArrayStruct,
                m_boneHiddenTypeCopyStart,
                m_oldBoneIndex,
                m_oldBoneIndexNoVar,
                m_boneIndex,
                m_boneIndexNoVar,
                m_boneChainIndex0,
                m_boneChainIndex1,
                m_boneChainIndex2,
                m_boneContractIndex0,
                m_boneContractIndex1,
                m_boneContractIndex2,
                m_boneHiddenTypeCopyEnd,
                m_boneWeightArray,
                m_boneIndexArray,
                m_annotatedTypeCStringFilename,
                m_annotatedTypeHkStringPtrFilename,
                m_annotatedTypeCStringScript,
                m_annotatedTypeHkStringPtrScript,
                m_annotatedTypeCStringBoneAttachment,
                m_annotatedTypeHkStringPtrBoneAttachment,
                m_annotatedTypeCStringLocalFrame,
                m_annotatedTypeHkStringPtrLocalFrame,
                m_annotatedHiddenTypeCopyStart,
                m_annotatedTypeHkInt32EventID,
                m_annotatedTypeHkInt32VariableIndex,
                m_annotatedTypeHkInt32AttributeIndex,
                m_annotatedTypeHkRealTime,
                m_annotatedTypeBoolNoVar,
                m_annotatedTypeHkBoolNoVar,
                m_annotatedTypeHkInt8NoVar,
                m_annotatedTypeHkInt16NoVar,
                m_annotatedTypeHkInt32NoVar,
                m_annotatedTypeHkUint8NoVar,
                m_annotatedTypeHkUint16NoVar,
                m_annotatedTypeHkUint32NoVar,
                m_annotatedTypeHkRealNoVar,
                m_annotatedTypeBoolOutput,
                m_annotatedTypeHkBoolOutput,
                m_annotatedTypeHkInt8Output,
                m_annotatedTypeHkInt16Output,
                m_annotatedTypeHkInt32Output,
                m_annotatedTypeHkUint8Output,
                m_annotatedTypeHkUint16Output,
                m_annotatedTypeHkUint32Output,
                m_annotatedTypeHkRealOutput,
                m_annotatedHiddenTypeBool,
                m_annotatedHiddenTypeHkBool,
                m_annotatedHiddenTypeCString1,
                m_annotatedHiddenTypeHkStringPtr1,
                m_annotatedHiddenTypeCString2,
                m_annotatedHiddenTypeHkStringPtr2,
                m_annotatedHiddenTypeHkInt8,
                m_annotatedHiddenTypeHkInt16,
                m_annotatedHiddenTypeHkInt32,
                m_annotatedHiddenTypeHkUint8,
                m_annotatedHiddenTypeHkUint16,
                m_annotatedHiddenTypeHkUint32,
                m_annotatedHiddenTypeCopyEnd,
                m_protectedHiddenMember,
                m_privateHiddenMember,
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
                        "inheritedHiddenMember" => Ok(__Field::m_inheritedHiddenMember),
                        "protectedInheritedHiddenMember" => {
                            Ok(__Field::m_protectedInheritedHiddenMember)
                        }
                        "privateInheritedHiddenMember" => {
                            Ok(__Field::m_privateInheritedHiddenMember)
                        }
                        "simpleTypeHkInt64" => Ok(__Field::m_simpleTypeHkInt64),
                        "simpleTypeHkUint64" => Ok(__Field::m_simpleTypeHkUint64),
                        "simpleHiddenTypeCopyStart" => {
                            Ok(__Field::m_simpleHiddenTypeCopyStart)
                        }
                        "simpleTypeBool" => Ok(__Field::m_simpleTypeBool),
                        "simpleTypeHkBool" => Ok(__Field::m_simpleTypeHkBool),
                        "simpleTypeCString" => Ok(__Field::m_simpleTypeCString),
                        "simpleTypeHkStringPtr" => Ok(__Field::m_simpleTypeHkStringPtr),
                        "simpleTypeHkInt8" => Ok(__Field::m_simpleTypeHkInt8),
                        "simpleTypeHkInt16" => Ok(__Field::m_simpleTypeHkInt16),
                        "simpleTypeHkInt32" => Ok(__Field::m_simpleTypeHkInt32),
                        "simpleTypeHkUint8" => Ok(__Field::m_simpleTypeHkUint8),
                        "simpleTypeHkUint16" => Ok(__Field::m_simpleTypeHkUint16),
                        "simpleTypeHkUint32" => Ok(__Field::m_simpleTypeHkUint32),
                        "simpleTypeHkReal" => Ok(__Field::m_simpleTypeHkReal),
                        "simpleTypeHkInt8Default" => {
                            Ok(__Field::m_simpleTypeHkInt8Default)
                        }
                        "simpleTypeHkInt16Default" => {
                            Ok(__Field::m_simpleTypeHkInt16Default)
                        }
                        "simpleTypeHkInt32Default" => {
                            Ok(__Field::m_simpleTypeHkInt32Default)
                        }
                        "simpleTypeHkUint8Default" => {
                            Ok(__Field::m_simpleTypeHkUint8Default)
                        }
                        "simpleTypeHkUint16Default" => {
                            Ok(__Field::m_simpleTypeHkUint16Default)
                        }
                        "simpleTypeHkUint32Default" => {
                            Ok(__Field::m_simpleTypeHkUint32Default)
                        }
                        "simpleTypeHkRealDefault" => {
                            Ok(__Field::m_simpleTypeHkRealDefault)
                        }
                        "simpleTypeHkInt8Clamp" => Ok(__Field::m_simpleTypeHkInt8Clamp),
                        "simpleTypeHkInt16Clamp" => Ok(__Field::m_simpleTypeHkInt16Clamp),
                        "simpleTypeHkInt32Clamp" => Ok(__Field::m_simpleTypeHkInt32Clamp),
                        "simpleTypeHkUint8Clamp" => Ok(__Field::m_simpleTypeHkUint8Clamp),
                        "simpleTypeHkUint16Clamp" => {
                            Ok(__Field::m_simpleTypeHkUint16Clamp)
                        }
                        "simpleTypeHkUint32Clamp" => {
                            Ok(__Field::m_simpleTypeHkUint32Clamp)
                        }
                        "simpleTypeHkRealClamp" => Ok(__Field::m_simpleTypeHkRealClamp),
                        "simpleHiddenTypeCopyEnd" => {
                            Ok(__Field::m_simpleHiddenTypeCopyEnd)
                        }
                        "complexTypeHkObjectPtr" => Ok(__Field::m_complexTypeHkObjectPtr),
                        "complexHiddenTypeCopyStart" => {
                            Ok(__Field::m_complexHiddenTypeCopyStart)
                        }
                        "complexTypeHkQuaternion" => {
                            Ok(__Field::m_complexTypeHkQuaternion)
                        }
                        "complexTypeHkVector4" => Ok(__Field::m_complexTypeHkVector4),
                        "complexTypeEnumHkInt8" => Ok(__Field::m_complexTypeEnumHkInt8),
                        "complexTypeEnumHkInt16" => Ok(__Field::m_complexTypeEnumHkInt16),
                        "complexTypeEnumHkInt32" => Ok(__Field::m_complexTypeEnumHkInt32),
                        "complexTypeEnumHkUint8" => Ok(__Field::m_complexTypeEnumHkUint8),
                        "complexTypeEnumHkUint16" => {
                            Ok(__Field::m_complexTypeEnumHkUint16)
                        }
                        "complexTypeEnumHkUint32" => {
                            Ok(__Field::m_complexTypeEnumHkUint32)
                        }
                        "complexTypeEnumHkInt8InvalidCheck" => {
                            Ok(__Field::m_complexTypeEnumHkInt8InvalidCheck)
                        }
                        "complexTypeEnumHkInt16InvalidCheck" => {
                            Ok(__Field::m_complexTypeEnumHkInt16InvalidCheck)
                        }
                        "complexTypeEnumHkInt32InvalidCheck" => {
                            Ok(__Field::m_complexTypeEnumHkInt32InvalidCheck)
                        }
                        "complexTypeEnumHkUint8InvalidCheck" => {
                            Ok(__Field::m_complexTypeEnumHkUint8InvalidCheck)
                        }
                        "complexTypeEnumHkUint16InvalidCheck" => {
                            Ok(__Field::m_complexTypeEnumHkUint16InvalidCheck)
                        }
                        "complexTypeEnumHkUint32InvalidCheck" => {
                            Ok(__Field::m_complexTypeEnumHkUint32InvalidCheck)
                        }
                        "complexTypeFlagsHkInt8" => Ok(__Field::m_complexTypeFlagsHkInt8),
                        "complexTypeFlagsHkInt16" => {
                            Ok(__Field::m_complexTypeFlagsHkInt16)
                        }
                        "complexTypeFlagsHkInt32" => {
                            Ok(__Field::m_complexTypeFlagsHkInt32)
                        }
                        "complexTypeFlagsHkUint8" => {
                            Ok(__Field::m_complexTypeFlagsHkUint8)
                        }
                        "complexTypeFlagsHkUint16" => {
                            Ok(__Field::m_complexTypeFlagsHkUint16)
                        }
                        "complexTypeFlagsHkUint32" => {
                            Ok(__Field::m_complexTypeFlagsHkUint32)
                        }
                        "complexTypeFlagsHkInt8InvalidCheck" => {
                            Ok(__Field::m_complexTypeFlagsHkInt8InvalidCheck)
                        }
                        "complexTypeFlagsHkInt16InvalidCheck" => {
                            Ok(__Field::m_complexTypeFlagsHkInt16InvalidCheck)
                        }
                        "complexTypeFlagsHkInt32InvalidCheck" => {
                            Ok(__Field::m_complexTypeFlagsHkInt32InvalidCheck)
                        }
                        "complexTypeFlagsHkUint8InvalidCheck" => {
                            Ok(__Field::m_complexTypeFlagsHkUint8InvalidCheck)
                        }
                        "complexTypeFlagsHkUint16InvalidCheck" => {
                            Ok(__Field::m_complexTypeFlagsHkUint16InvalidCheck)
                        }
                        "complexTypeFlagsHkUint32InvalidCheck" => {
                            Ok(__Field::m_complexTypeFlagsHkUint32InvalidCheck)
                        }
                        "complexHiddenTypeCopyEnd" => {
                            Ok(__Field::m_complexHiddenTypeCopyEnd)
                        }
                        "nestedTypeHkbGeneratorPtr" => {
                            Ok(__Field::m_nestedTypeHkbGeneratorPtr)
                        }
                        "nestedTypeHkbGeneratorRefPtr" => {
                            Ok(__Field::m_nestedTypeHkbGeneratorRefPtr)
                        }
                        "nestedTypeHkbModifierPtr" => {
                            Ok(__Field::m_nestedTypeHkbModifierPtr)
                        }
                        "nestedTypeHkbModifierRefPtr" => {
                            Ok(__Field::m_nestedTypeHkbModifierRefPtr)
                        }
                        "nestedTypeHkbCustomIdSelectorPtr" => {
                            Ok(__Field::m_nestedTypeHkbCustomIdSelectorPtr)
                        }
                        "nestedTypeHkbCustomIdSelectorRefPtr" => {
                            Ok(__Field::m_nestedTypeHkbCustomIdSelectorRefPtr)
                        }
                        "nestedTypeArrayBool" => Ok(__Field::m_nestedTypeArrayBool),
                        "nestedTypeArrayHkBool" => Ok(__Field::m_nestedTypeArrayHkBool),
                        "nestedTypeArrayCString" => Ok(__Field::m_nestedTypeArrayCString),
                        "nestedTypeArrayHkStringPtr" => {
                            Ok(__Field::m_nestedTypeArrayHkStringPtr)
                        }
                        "nestedTypeArrayHkInt8" => Ok(__Field::m_nestedTypeArrayHkInt8),
                        "nestedTypeArrayHkInt16" => Ok(__Field::m_nestedTypeArrayHkInt16),
                        "nestedTypeArrayHkInt32" => Ok(__Field::m_nestedTypeArrayHkInt32),
                        "nestedTypeArrayHkUint8" => Ok(__Field::m_nestedTypeArrayHkUint8),
                        "nestedTypeArrayHkUint16" => {
                            Ok(__Field::m_nestedTypeArrayHkUint16)
                        }
                        "nestedTypeArrayHkUint32" => {
                            Ok(__Field::m_nestedTypeArrayHkUint32)
                        }
                        "nestedTypeArrayHkReal" => Ok(__Field::m_nestedTypeArrayHkReal),
                        "nestedTypeArrayHkbGeneratorPtr" => {
                            Ok(__Field::m_nestedTypeArrayHkbGeneratorPtr)
                        }
                        "nestedTypeArrayHkbGeneratorRefPtr" => {
                            Ok(__Field::m_nestedTypeArrayHkbGeneratorRefPtr)
                        }
                        "nestedTypeArrayHkbModifierPtr" => {
                            Ok(__Field::m_nestedTypeArrayHkbModifierPtr)
                        }
                        "nestedTypeArrayHkbModifierRefPtr" => {
                            Ok(__Field::m_nestedTypeArrayHkbModifierRefPtr)
                        }
                        "nestedTypeArrayHkbCustomIdSelectorPtr" => {
                            Ok(__Field::m_nestedTypeArrayHkbCustomIdSelectorPtr)
                        }
                        "nestedTypeArrayHkbCustomIdSelectorRefPtr" => {
                            Ok(__Field::m_nestedTypeArrayHkbCustomIdSelectorRefPtr)
                        }
                        "nestedTypeStruct" => Ok(__Field::m_nestedTypeStruct),
                        "nestedTypeArrayStruct" => Ok(__Field::m_nestedTypeArrayStruct),
                        "boneHiddenTypeCopyStart" => {
                            Ok(__Field::m_boneHiddenTypeCopyStart)
                        }
                        "oldBoneIndex" => Ok(__Field::m_oldBoneIndex),
                        "oldBoneIndexNoVar" => Ok(__Field::m_oldBoneIndexNoVar),
                        "boneIndex" => Ok(__Field::m_boneIndex),
                        "boneIndexNoVar" => Ok(__Field::m_boneIndexNoVar),
                        "boneChainIndex0" => Ok(__Field::m_boneChainIndex0),
                        "boneChainIndex1" => Ok(__Field::m_boneChainIndex1),
                        "boneChainIndex2" => Ok(__Field::m_boneChainIndex2),
                        "boneContractIndex0" => Ok(__Field::m_boneContractIndex0),
                        "boneContractIndex1" => Ok(__Field::m_boneContractIndex1),
                        "boneContractIndex2" => Ok(__Field::m_boneContractIndex2),
                        "boneHiddenTypeCopyEnd" => Ok(__Field::m_boneHiddenTypeCopyEnd),
                        "boneWeightArray" => Ok(__Field::m_boneWeightArray),
                        "boneIndexArray" => Ok(__Field::m_boneIndexArray),
                        "annotatedTypeCStringFilename" => {
                            Ok(__Field::m_annotatedTypeCStringFilename)
                        }
                        "annotatedTypeHkStringPtrFilename" => {
                            Ok(__Field::m_annotatedTypeHkStringPtrFilename)
                        }
                        "annotatedTypeCStringScript" => {
                            Ok(__Field::m_annotatedTypeCStringScript)
                        }
                        "annotatedTypeHkStringPtrScript" => {
                            Ok(__Field::m_annotatedTypeHkStringPtrScript)
                        }
                        "annotatedTypeCStringBoneAttachment" => {
                            Ok(__Field::m_annotatedTypeCStringBoneAttachment)
                        }
                        "annotatedTypeHkStringPtrBoneAttachment" => {
                            Ok(__Field::m_annotatedTypeHkStringPtrBoneAttachment)
                        }
                        "annotatedTypeCStringLocalFrame" => {
                            Ok(__Field::m_annotatedTypeCStringLocalFrame)
                        }
                        "annotatedTypeHkStringPtrLocalFrame" => {
                            Ok(__Field::m_annotatedTypeHkStringPtrLocalFrame)
                        }
                        "annotatedHiddenTypeCopyStart" => {
                            Ok(__Field::m_annotatedHiddenTypeCopyStart)
                        }
                        "annotatedTypeHkInt32EventID" => {
                            Ok(__Field::m_annotatedTypeHkInt32EventID)
                        }
                        "annotatedTypeHkInt32VariableIndex" => {
                            Ok(__Field::m_annotatedTypeHkInt32VariableIndex)
                        }
                        "annotatedTypeHkInt32AttributeIndex" => {
                            Ok(__Field::m_annotatedTypeHkInt32AttributeIndex)
                        }
                        "annotatedTypeHkRealTime" => {
                            Ok(__Field::m_annotatedTypeHkRealTime)
                        }
                        "annotatedTypeBoolNoVar" => Ok(__Field::m_annotatedTypeBoolNoVar),
                        "annotatedTypeHkBoolNoVar" => {
                            Ok(__Field::m_annotatedTypeHkBoolNoVar)
                        }
                        "annotatedTypeHkInt8NoVar" => {
                            Ok(__Field::m_annotatedTypeHkInt8NoVar)
                        }
                        "annotatedTypeHkInt16NoVar" => {
                            Ok(__Field::m_annotatedTypeHkInt16NoVar)
                        }
                        "annotatedTypeHkInt32NoVar" => {
                            Ok(__Field::m_annotatedTypeHkInt32NoVar)
                        }
                        "annotatedTypeHkUint8NoVar" => {
                            Ok(__Field::m_annotatedTypeHkUint8NoVar)
                        }
                        "annotatedTypeHkUint16NoVar" => {
                            Ok(__Field::m_annotatedTypeHkUint16NoVar)
                        }
                        "annotatedTypeHkUint32NoVar" => {
                            Ok(__Field::m_annotatedTypeHkUint32NoVar)
                        }
                        "annotatedTypeHkRealNoVar" => {
                            Ok(__Field::m_annotatedTypeHkRealNoVar)
                        }
                        "annotatedTypeBoolOutput" => {
                            Ok(__Field::m_annotatedTypeBoolOutput)
                        }
                        "annotatedTypeHkBoolOutput" => {
                            Ok(__Field::m_annotatedTypeHkBoolOutput)
                        }
                        "annotatedTypeHkInt8Output" => {
                            Ok(__Field::m_annotatedTypeHkInt8Output)
                        }
                        "annotatedTypeHkInt16Output" => {
                            Ok(__Field::m_annotatedTypeHkInt16Output)
                        }
                        "annotatedTypeHkInt32Output" => {
                            Ok(__Field::m_annotatedTypeHkInt32Output)
                        }
                        "annotatedTypeHkUint8Output" => {
                            Ok(__Field::m_annotatedTypeHkUint8Output)
                        }
                        "annotatedTypeHkUint16Output" => {
                            Ok(__Field::m_annotatedTypeHkUint16Output)
                        }
                        "annotatedTypeHkUint32Output" => {
                            Ok(__Field::m_annotatedTypeHkUint32Output)
                        }
                        "annotatedTypeHkRealOutput" => {
                            Ok(__Field::m_annotatedTypeHkRealOutput)
                        }
                        "annotatedHiddenTypeBool" => {
                            Ok(__Field::m_annotatedHiddenTypeBool)
                        }
                        "annotatedHiddenTypeHkBool" => {
                            Ok(__Field::m_annotatedHiddenTypeHkBool)
                        }
                        "annotatedHiddenTypeCString1" => {
                            Ok(__Field::m_annotatedHiddenTypeCString1)
                        }
                        "annotatedHiddenTypeHkStringPtr1" => {
                            Ok(__Field::m_annotatedHiddenTypeHkStringPtr1)
                        }
                        "annotatedHiddenTypeCString2" => {
                            Ok(__Field::m_annotatedHiddenTypeCString2)
                        }
                        "annotatedHiddenTypeHkStringPtr2" => {
                            Ok(__Field::m_annotatedHiddenTypeHkStringPtr2)
                        }
                        "annotatedHiddenTypeHkInt8" => {
                            Ok(__Field::m_annotatedHiddenTypeHkInt8)
                        }
                        "annotatedHiddenTypeHkInt16" => {
                            Ok(__Field::m_annotatedHiddenTypeHkInt16)
                        }
                        "annotatedHiddenTypeHkInt32" => {
                            Ok(__Field::m_annotatedHiddenTypeHkInt32)
                        }
                        "annotatedHiddenTypeHkUint8" => {
                            Ok(__Field::m_annotatedHiddenTypeHkUint8)
                        }
                        "annotatedHiddenTypeHkUint16" => {
                            Ok(__Field::m_annotatedHiddenTypeHkUint16)
                        }
                        "annotatedHiddenTypeHkUint32" => {
                            Ok(__Field::m_annotatedHiddenTypeHkUint32)
                        }
                        "annotatedHiddenTypeCopyEnd" => {
                            Ok(__Field::m_annotatedHiddenTypeCopyEnd)
                        }
                        "protectedHiddenMember" => Ok(__Field::m_protectedHiddenMember),
                        "privateHiddenMember" => Ok(__Field::m_privateHiddenMember),
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
            struct __hkbCustomTestGeneratorVisitor<'de> {
                marker: _serde::__private::PhantomData<hkbCustomTestGenerator<'de>>,
                lifetime: _serde::__private::PhantomData<&'de ()>,
            }
            #[allow(clippy::match_single_binding)]
            #[allow(clippy::reversed_empty_ranges)]
            #[allow(clippy::single_match)]
            impl<'de> _serde::de::Visitor<'de> for __hkbCustomTestGeneratorVisitor<'de> {
                type Value = hkbCustomTestGenerator<'de>;
                fn expecting(
                    &self,
                    __formatter: &mut core::fmt::Formatter,
                ) -> core::fmt::Result {
                    core::fmt::Formatter::write_str(
                        __formatter,
                        "struct hkbCustomTestGenerator",
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
                    let mut m_protectedHiddenMember: _serde::__private::Option<bool> = _serde::__private::None;
                    let mut m_privateHiddenMember: _serde::__private::Option<bool> = _serde::__private::None;
                    for i in 0..2usize {
                        match i {
                            0usize => {
                                if _serde::__private::Option::is_some(
                                    &m_protectedHiddenMember,
                                ) {
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "protectedHiddenMember",
                                        ),
                                    );
                                }
                                m_protectedHiddenMember = _serde::__private::Some(
                                    match __A::next_value::<bool>(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            1usize => {
                                if _serde::__private::Option::is_some(
                                    &m_privateHiddenMember,
                                ) {
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "privateHiddenMember",
                                        ),
                                    );
                                }
                                m_privateHiddenMember = _serde::__private::Some(
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
                    __A::pad(&mut __map, 0usize, 14usize)?;
                    let m_protectedHiddenMember = match m_protectedHiddenMember {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "protectedHiddenMember",
                                ),
                            );
                        }
                    };
                    let m_privateHiddenMember = match m_privateHiddenMember {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "privateHiddenMember",
                                ),
                            );
                        }
                    };
                    _serde::__private::Ok(hkbCustomTestGenerator {
                        __ptr,
                        parent,
                        m_protectedHiddenMember,
                        m_privateHiddenMember,
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
                    let mut m_inheritedHiddenMember: _serde::__private::Option<bool> = _serde::__private::None;
                    let mut m_protectedInheritedHiddenMember: _serde::__private::Option<
                        bool,
                    > = _serde::__private::None;
                    let mut m_privateInheritedHiddenMember: _serde::__private::Option<
                        bool,
                    > = _serde::__private::None;
                    let mut m_simpleTypeHkInt64: _serde::__private::Option<I64<'de>> = _serde::__private::None;
                    let mut m_simpleTypeHkUint64: _serde::__private::Option<U64<'de>> = _serde::__private::None;
                    let mut m_simpleHiddenTypeCopyStart: _serde::__private::Option<
                        bool,
                    > = _serde::__private::None;
                    let mut m_simpleTypeBool: _serde::__private::Option<bool> = _serde::__private::None;
                    let mut m_simpleTypeHkBool: _serde::__private::Option<bool> = _serde::__private::None;
                    let mut m_simpleTypeCString: _serde::__private::Option<
                        CString<'de>,
                    > = _serde::__private::None;
                    let mut m_simpleTypeHkStringPtr: _serde::__private::Option<
                        StringPtr<'de>,
                    > = _serde::__private::None;
                    let mut m_simpleTypeHkInt8: _serde::__private::Option<I8<'de>> = _serde::__private::None;
                    let mut m_simpleTypeHkInt16: _serde::__private::Option<I16<'de>> = _serde::__private::None;
                    let mut m_simpleTypeHkInt32: _serde::__private::Option<I32<'de>> = _serde::__private::None;
                    let mut m_simpleTypeHkUint8: _serde::__private::Option<U8<'de>> = _serde::__private::None;
                    let mut m_simpleTypeHkUint16: _serde::__private::Option<U16<'de>> = _serde::__private::None;
                    let mut m_simpleTypeHkUint32: _serde::__private::Option<U32<'de>> = _serde::__private::None;
                    let mut m_simpleTypeHkReal: _serde::__private::Option<f32> = _serde::__private::None;
                    let mut m_simpleTypeHkInt8Default: _serde::__private::Option<
                        I8<'de>,
                    > = _serde::__private::None;
                    let mut m_simpleTypeHkInt16Default: _serde::__private::Option<
                        I16<'de>,
                    > = _serde::__private::None;
                    let mut m_simpleTypeHkInt32Default: _serde::__private::Option<
                        I32<'de>,
                    > = _serde::__private::None;
                    let mut m_simpleTypeHkUint8Default: _serde::__private::Option<
                        U8<'de>,
                    > = _serde::__private::None;
                    let mut m_simpleTypeHkUint16Default: _serde::__private::Option<
                        U16<'de>,
                    > = _serde::__private::None;
                    let mut m_simpleTypeHkUint32Default: _serde::__private::Option<
                        U32<'de>,
                    > = _serde::__private::None;
                    let mut m_simpleTypeHkRealDefault: _serde::__private::Option<f32> = _serde::__private::None;
                    let mut m_simpleTypeHkInt8Clamp: _serde::__private::Option<
                        I8<'de>,
                    > = _serde::__private::None;
                    let mut m_simpleTypeHkInt16Clamp: _serde::__private::Option<
                        I16<'de>,
                    > = _serde::__private::None;
                    let mut m_simpleTypeHkInt32Clamp: _serde::__private::Option<
                        I32<'de>,
                    > = _serde::__private::None;
                    let mut m_simpleTypeHkUint8Clamp: _serde::__private::Option<
                        U8<'de>,
                    > = _serde::__private::None;
                    let mut m_simpleTypeHkUint16Clamp: _serde::__private::Option<
                        U16<'de>,
                    > = _serde::__private::None;
                    let mut m_simpleTypeHkUint32Clamp: _serde::__private::Option<
                        U32<'de>,
                    > = _serde::__private::None;
                    let mut m_simpleTypeHkRealClamp: _serde::__private::Option<f32> = _serde::__private::None;
                    let mut m_simpleHiddenTypeCopyEnd: _serde::__private::Option<bool> = _serde::__private::None;
                    let mut m_complexTypeHkObjectPtr: _serde::__private::Option<
                        Pointer<'de>,
                    > = _serde::__private::None;
                    let mut m_complexHiddenTypeCopyStart: _serde::__private::Option<
                        bool,
                    > = _serde::__private::None;
                    let mut m_complexTypeHkQuaternion: _serde::__private::Option<
                        Quaternion,
                    > = _serde::__private::None;
                    let mut m_complexTypeHkVector4: _serde::__private::Option<Vector4> = _serde::__private::None;
                    let mut m_complexTypeEnumHkInt8: _serde::__private::Option<
                        CustomEnum,
                    > = _serde::__private::None;
                    let mut m_complexTypeEnumHkInt16: _serde::__private::Option<
                        CustomEnum,
                    > = _serde::__private::None;
                    let mut m_complexTypeEnumHkInt32: _serde::__private::Option<
                        CustomEnum,
                    > = _serde::__private::None;
                    let mut m_complexTypeEnumHkUint8: _serde::__private::Option<
                        CustomEnum,
                    > = _serde::__private::None;
                    let mut m_complexTypeEnumHkUint16: _serde::__private::Option<
                        CustomEnum,
                    > = _serde::__private::None;
                    let mut m_complexTypeEnumHkUint32: _serde::__private::Option<
                        CustomEnum,
                    > = _serde::__private::None;
                    let mut m_complexTypeEnumHkInt8InvalidCheck: _serde::__private::Option<
                        CustomEnum,
                    > = _serde::__private::None;
                    let mut m_complexTypeEnumHkInt16InvalidCheck: _serde::__private::Option<
                        CustomEnum,
                    > = _serde::__private::None;
                    let mut m_complexTypeEnumHkInt32InvalidCheck: _serde::__private::Option<
                        CustomEnum,
                    > = _serde::__private::None;
                    let mut m_complexTypeEnumHkUint8InvalidCheck: _serde::__private::Option<
                        CustomEnum,
                    > = _serde::__private::None;
                    let mut m_complexTypeEnumHkUint16InvalidCheck: _serde::__private::Option<
                        CustomEnum,
                    > = _serde::__private::None;
                    let mut m_complexTypeEnumHkUint32InvalidCheck: _serde::__private::Option<
                        CustomEnum,
                    > = _serde::__private::None;
                    let mut m_complexTypeFlagsHkInt8: _serde::__private::Option<
                        CustomFlag,
                    > = _serde::__private::None;
                    let mut m_complexTypeFlagsHkInt16: _serde::__private::Option<
                        CustomFlag,
                    > = _serde::__private::None;
                    let mut m_complexTypeFlagsHkInt32: _serde::__private::Option<
                        CustomFlag,
                    > = _serde::__private::None;
                    let mut m_complexTypeFlagsHkUint8: _serde::__private::Option<
                        CustomFlag,
                    > = _serde::__private::None;
                    let mut m_complexTypeFlagsHkUint16: _serde::__private::Option<
                        CustomFlag,
                    > = _serde::__private::None;
                    let mut m_complexTypeFlagsHkUint32: _serde::__private::Option<
                        CustomFlag,
                    > = _serde::__private::None;
                    let mut m_complexTypeFlagsHkInt8InvalidCheck: _serde::__private::Option<
                        CustomFlag,
                    > = _serde::__private::None;
                    let mut m_complexTypeFlagsHkInt16InvalidCheck: _serde::__private::Option<
                        CustomFlag,
                    > = _serde::__private::None;
                    let mut m_complexTypeFlagsHkInt32InvalidCheck: _serde::__private::Option<
                        CustomFlag,
                    > = _serde::__private::None;
                    let mut m_complexTypeFlagsHkUint8InvalidCheck: _serde::__private::Option<
                        CustomFlag,
                    > = _serde::__private::None;
                    let mut m_complexTypeFlagsHkUint16InvalidCheck: _serde::__private::Option<
                        CustomFlag,
                    > = _serde::__private::None;
                    let mut m_complexTypeFlagsHkUint32InvalidCheck: _serde::__private::Option<
                        CustomFlag,
                    > = _serde::__private::None;
                    let mut m_complexHiddenTypeCopyEnd: _serde::__private::Option<
                        bool,
                    > = _serde::__private::None;
                    let mut m_nestedTypeHkbGeneratorPtr: _serde::__private::Option<
                        Pointer<'de>,
                    > = _serde::__private::None;
                    let mut m_nestedTypeHkbGeneratorRefPtr: _serde::__private::Option<
                        Pointer<'de>,
                    > = _serde::__private::None;
                    let mut m_nestedTypeHkbModifierPtr: _serde::__private::Option<
                        Pointer<'de>,
                    > = _serde::__private::None;
                    let mut m_nestedTypeHkbModifierRefPtr: _serde::__private::Option<
                        Pointer<'de>,
                    > = _serde::__private::None;
                    let mut m_nestedTypeHkbCustomIdSelectorPtr: _serde::__private::Option<
                        Pointer<'de>,
                    > = _serde::__private::None;
                    let mut m_nestedTypeHkbCustomIdSelectorRefPtr: _serde::__private::Option<
                        Pointer<'de>,
                    > = _serde::__private::None;
                    let mut m_nestedTypeArrayBool: _serde::__private::Option<
                        Vec<bool>,
                    > = _serde::__private::None;
                    let mut m_nestedTypeArrayHkBool: _serde::__private::Option<
                        Vec<bool>,
                    > = _serde::__private::None;
                    let mut m_nestedTypeArrayCString: _serde::__private::Option<
                        Vec<CString<'de>>,
                    > = _serde::__private::None;
                    let mut m_nestedTypeArrayHkStringPtr: _serde::__private::Option<
                        Vec<StringPtr<'de>>,
                    > = _serde::__private::None;
                    let mut m_nestedTypeArrayHkInt8: _serde::__private::Option<
                        Vec<I8<'de>>,
                    > = _serde::__private::None;
                    let mut m_nestedTypeArrayHkInt16: _serde::__private::Option<
                        Vec<I16<'de>>,
                    > = _serde::__private::None;
                    let mut m_nestedTypeArrayHkInt32: _serde::__private::Option<
                        Vec<I32<'de>>,
                    > = _serde::__private::None;
                    let mut m_nestedTypeArrayHkUint8: _serde::__private::Option<
                        Vec<U8<'de>>,
                    > = _serde::__private::None;
                    let mut m_nestedTypeArrayHkUint16: _serde::__private::Option<
                        Vec<U16<'de>>,
                    > = _serde::__private::None;
                    let mut m_nestedTypeArrayHkUint32: _serde::__private::Option<
                        Vec<U32<'de>>,
                    > = _serde::__private::None;
                    let mut m_nestedTypeArrayHkReal: _serde::__private::Option<
                        Vec<f32>,
                    > = _serde::__private::None;
                    let mut m_nestedTypeArrayHkbGeneratorPtr: _serde::__private::Option<
                        Vec<Pointer<'de>>,
                    > = _serde::__private::None;
                    let mut m_nestedTypeArrayHkbGeneratorRefPtr: _serde::__private::Option<
                        Vec<Pointer<'de>>,
                    > = _serde::__private::None;
                    let mut m_nestedTypeArrayHkbModifierPtr: _serde::__private::Option<
                        Vec<Pointer<'de>>,
                    > = _serde::__private::None;
                    let mut m_nestedTypeArrayHkbModifierRefPtr: _serde::__private::Option<
                        Vec<Pointer<'de>>,
                    > = _serde::__private::None;
                    let mut m_nestedTypeArrayHkbCustomIdSelectorPtr: _serde::__private::Option<
                        Vec<Pointer<'de>>,
                    > = _serde::__private::None;
                    let mut m_nestedTypeArrayHkbCustomIdSelectorRefPtr: _serde::__private::Option<
                        Vec<Pointer<'de>>,
                    > = _serde::__private::None;
                    let mut m_nestedTypeStruct: _serde::__private::Option<
                        hkbCustomTestGeneratorNestedTypesBase<'de>,
                    > = _serde::__private::None;
                    let mut m_nestedTypeArrayStruct: _serde::__private::Option<
                        Vec<hkbCustomTestGeneratorNestedTypesBase<'de>>,
                    > = _serde::__private::None;
                    let mut m_boneHiddenTypeCopyStart: _serde::__private::Option<bool> = _serde::__private::None;
                    let mut m_oldBoneIndex: _serde::__private::Option<I16<'de>> = _serde::__private::None;
                    let mut m_oldBoneIndexNoVar: _serde::__private::Option<I16<'de>> = _serde::__private::None;
                    let mut m_boneIndex: _serde::__private::Option<I16<'de>> = _serde::__private::None;
                    let mut m_boneIndexNoVar: _serde::__private::Option<I16<'de>> = _serde::__private::None;
                    let mut m_boneChainIndex0: _serde::__private::Option<I16<'de>> = _serde::__private::None;
                    let mut m_boneChainIndex1: _serde::__private::Option<I16<'de>> = _serde::__private::None;
                    let mut m_boneChainIndex2: _serde::__private::Option<I16<'de>> = _serde::__private::None;
                    let mut m_boneContractIndex0: _serde::__private::Option<I16<'de>> = _serde::__private::None;
                    let mut m_boneContractIndex1: _serde::__private::Option<I16<'de>> = _serde::__private::None;
                    let mut m_boneContractIndex2: _serde::__private::Option<I16<'de>> = _serde::__private::None;
                    let mut m_boneHiddenTypeCopyEnd: _serde::__private::Option<bool> = _serde::__private::None;
                    let mut m_boneWeightArray: _serde::__private::Option<Pointer<'de>> = _serde::__private::None;
                    let mut m_boneIndexArray: _serde::__private::Option<Pointer<'de>> = _serde::__private::None;
                    let mut m_annotatedTypeCStringFilename: _serde::__private::Option<
                        CString<'de>,
                    > = _serde::__private::None;
                    let mut m_annotatedTypeHkStringPtrFilename: _serde::__private::Option<
                        StringPtr<'de>,
                    > = _serde::__private::None;
                    let mut m_annotatedTypeCStringScript: _serde::__private::Option<
                        CString<'de>,
                    > = _serde::__private::None;
                    let mut m_annotatedTypeHkStringPtrScript: _serde::__private::Option<
                        StringPtr<'de>,
                    > = _serde::__private::None;
                    let mut m_annotatedTypeCStringBoneAttachment: _serde::__private::Option<
                        CString<'de>,
                    > = _serde::__private::None;
                    let mut m_annotatedTypeHkStringPtrBoneAttachment: _serde::__private::Option<
                        StringPtr<'de>,
                    > = _serde::__private::None;
                    let mut m_annotatedTypeCStringLocalFrame: _serde::__private::Option<
                        CString<'de>,
                    > = _serde::__private::None;
                    let mut m_annotatedTypeHkStringPtrLocalFrame: _serde::__private::Option<
                        StringPtr<'de>,
                    > = _serde::__private::None;
                    let mut m_annotatedHiddenTypeCopyStart: _serde::__private::Option<
                        bool,
                    > = _serde::__private::None;
                    let mut m_annotatedTypeHkInt32EventID: _serde::__private::Option<
                        I32<'de>,
                    > = _serde::__private::None;
                    let mut m_annotatedTypeHkInt32VariableIndex: _serde::__private::Option<
                        I32<'de>,
                    > = _serde::__private::None;
                    let mut m_annotatedTypeHkInt32AttributeIndex: _serde::__private::Option<
                        I32<'de>,
                    > = _serde::__private::None;
                    let mut m_annotatedTypeHkRealTime: _serde::__private::Option<f32> = _serde::__private::None;
                    let mut m_annotatedTypeBoolNoVar: _serde::__private::Option<bool> = _serde::__private::None;
                    let mut m_annotatedTypeHkBoolNoVar: _serde::__private::Option<
                        bool,
                    > = _serde::__private::None;
                    let mut m_annotatedTypeHkInt8NoVar: _serde::__private::Option<
                        I8<'de>,
                    > = _serde::__private::None;
                    let mut m_annotatedTypeHkInt16NoVar: _serde::__private::Option<
                        I16<'de>,
                    > = _serde::__private::None;
                    let mut m_annotatedTypeHkInt32NoVar: _serde::__private::Option<
                        I32<'de>,
                    > = _serde::__private::None;
                    let mut m_annotatedTypeHkUint8NoVar: _serde::__private::Option<
                        U8<'de>,
                    > = _serde::__private::None;
                    let mut m_annotatedTypeHkUint16NoVar: _serde::__private::Option<
                        U16<'de>,
                    > = _serde::__private::None;
                    let mut m_annotatedTypeHkUint32NoVar: _serde::__private::Option<
                        U32<'de>,
                    > = _serde::__private::None;
                    let mut m_annotatedTypeHkRealNoVar: _serde::__private::Option<f32> = _serde::__private::None;
                    let mut m_annotatedTypeBoolOutput: _serde::__private::Option<bool> = _serde::__private::None;
                    let mut m_annotatedTypeHkBoolOutput: _serde::__private::Option<
                        bool,
                    > = _serde::__private::None;
                    let mut m_annotatedTypeHkInt8Output: _serde::__private::Option<
                        I8<'de>,
                    > = _serde::__private::None;
                    let mut m_annotatedTypeHkInt16Output: _serde::__private::Option<
                        I16<'de>,
                    > = _serde::__private::None;
                    let mut m_annotatedTypeHkInt32Output: _serde::__private::Option<
                        I32<'de>,
                    > = _serde::__private::None;
                    let mut m_annotatedTypeHkUint8Output: _serde::__private::Option<
                        U8<'de>,
                    > = _serde::__private::None;
                    let mut m_annotatedTypeHkUint16Output: _serde::__private::Option<
                        U16<'de>,
                    > = _serde::__private::None;
                    let mut m_annotatedTypeHkUint32Output: _serde::__private::Option<
                        U32<'de>,
                    > = _serde::__private::None;
                    let mut m_annotatedTypeHkRealOutput: _serde::__private::Option<
                        f32,
                    > = _serde::__private::None;
                    let mut m_annotatedHiddenTypeBool: _serde::__private::Option<bool> = _serde::__private::None;
                    let mut m_annotatedHiddenTypeHkBool: _serde::__private::Option<
                        bool,
                    > = _serde::__private::None;
                    let mut m_annotatedHiddenTypeCString1: _serde::__private::Option<
                        CString<'de>,
                    > = _serde::__private::None;
                    let mut m_annotatedHiddenTypeHkStringPtr1: _serde::__private::Option<
                        StringPtr<'de>,
                    > = _serde::__private::None;
                    let mut m_annotatedHiddenTypeCString2: _serde::__private::Option<
                        CString<'de>,
                    > = _serde::__private::None;
                    let mut m_annotatedHiddenTypeHkStringPtr2: _serde::__private::Option<
                        StringPtr<'de>,
                    > = _serde::__private::None;
                    let mut m_annotatedHiddenTypeHkInt8: _serde::__private::Option<
                        I8<'de>,
                    > = _serde::__private::None;
                    let mut m_annotatedHiddenTypeHkInt16: _serde::__private::Option<
                        I16<'de>,
                    > = _serde::__private::None;
                    let mut m_annotatedHiddenTypeHkInt32: _serde::__private::Option<
                        I32<'de>,
                    > = _serde::__private::None;
                    let mut m_annotatedHiddenTypeHkUint8: _serde::__private::Option<
                        U8<'de>,
                    > = _serde::__private::None;
                    let mut m_annotatedHiddenTypeHkUint16: _serde::__private::Option<
                        U16<'de>,
                    > = _serde::__private::None;
                    let mut m_annotatedHiddenTypeHkUint32: _serde::__private::Option<
                        U32<'de>,
                    > = _serde::__private::None;
                    let mut m_annotatedHiddenTypeCopyEnd: _serde::__private::Option<
                        bool,
                    > = _serde::__private::None;
                    let mut m_protectedHiddenMember: _serde::__private::Option<bool> = _serde::__private::None;
                    let mut m_privateHiddenMember: _serde::__private::Option<bool> = _serde::__private::None;
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
                            __Field::m_inheritedHiddenMember => {
                                #[cfg(
                                    any(feature = "strict", feature = "ignore_duplicates")
                                )]
                                if _serde::__private::Option::is_some(
                                    &m_inheritedHiddenMember,
                                ) {
                                    #[cfg(feature = "ignore_duplicates")]
                                    {
                                        __A::skip_value(&mut __map)?;
                                        continue;
                                    }
                                    #[cfg(feature = "strict")]
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "inheritedHiddenMember",
                                        ),
                                    );
                                }
                                m_inheritedHiddenMember = _serde::__private::Some(
                                    match __A::next_value::<bool>(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            __Field::m_protectedInheritedHiddenMember => {
                                #[cfg(
                                    any(feature = "strict", feature = "ignore_duplicates")
                                )]
                                if _serde::__private::Option::is_some(
                                    &m_protectedInheritedHiddenMember,
                                ) {
                                    #[cfg(feature = "ignore_duplicates")]
                                    {
                                        __A::skip_value(&mut __map)?;
                                        continue;
                                    }
                                    #[cfg(feature = "strict")]
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "protectedInheritedHiddenMember",
                                        ),
                                    );
                                }
                                m_protectedInheritedHiddenMember = _serde::__private::Some(
                                    match __A::next_value::<bool>(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            __Field::m_privateInheritedHiddenMember => {
                                #[cfg(
                                    any(feature = "strict", feature = "ignore_duplicates")
                                )]
                                if _serde::__private::Option::is_some(
                                    &m_privateInheritedHiddenMember,
                                ) {
                                    #[cfg(feature = "ignore_duplicates")]
                                    {
                                        __A::skip_value(&mut __map)?;
                                        continue;
                                    }
                                    #[cfg(feature = "strict")]
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "privateInheritedHiddenMember",
                                        ),
                                    );
                                }
                                m_privateInheritedHiddenMember = _serde::__private::Some(
                                    match __A::next_value::<bool>(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            __Field::m_simpleTypeHkInt64 => {
                                #[cfg(
                                    any(feature = "strict", feature = "ignore_duplicates")
                                )]
                                if _serde::__private::Option::is_some(
                                    &m_simpleTypeHkInt64,
                                ) {
                                    #[cfg(feature = "ignore_duplicates")]
                                    {
                                        __A::skip_value(&mut __map)?;
                                        continue;
                                    }
                                    #[cfg(feature = "strict")]
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "simpleTypeHkInt64",
                                        ),
                                    );
                                }
                                m_simpleTypeHkInt64 = _serde::__private::Some(
                                    match __A::next_value::<I64<'de>>(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            __Field::m_simpleTypeHkUint64 => {
                                #[cfg(
                                    any(feature = "strict", feature = "ignore_duplicates")
                                )]
                                if _serde::__private::Option::is_some(
                                    &m_simpleTypeHkUint64,
                                ) {
                                    #[cfg(feature = "ignore_duplicates")]
                                    {
                                        __A::skip_value(&mut __map)?;
                                        continue;
                                    }
                                    #[cfg(feature = "strict")]
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "simpleTypeHkUint64",
                                        ),
                                    );
                                }
                                m_simpleTypeHkUint64 = _serde::__private::Some(
                                    match __A::next_value::<U64<'de>>(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            __Field::m_simpleHiddenTypeCopyStart => {
                                #[cfg(
                                    any(feature = "strict", feature = "ignore_duplicates")
                                )]
                                if _serde::__private::Option::is_some(
                                    &m_simpleHiddenTypeCopyStart,
                                ) {
                                    #[cfg(feature = "ignore_duplicates")]
                                    {
                                        __A::skip_value(&mut __map)?;
                                        continue;
                                    }
                                    #[cfg(feature = "strict")]
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "simpleHiddenTypeCopyStart",
                                        ),
                                    );
                                }
                                m_simpleHiddenTypeCopyStart = _serde::__private::Some(
                                    match __A::next_value::<bool>(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            __Field::m_simpleTypeBool => {
                                #[cfg(
                                    any(feature = "strict", feature = "ignore_duplicates")
                                )]
                                if _serde::__private::Option::is_some(&m_simpleTypeBool) {
                                    #[cfg(feature = "ignore_duplicates")]
                                    {
                                        __A::skip_value(&mut __map)?;
                                        continue;
                                    }
                                    #[cfg(feature = "strict")]
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "simpleTypeBool",
                                        ),
                                    );
                                }
                                m_simpleTypeBool = _serde::__private::Some(
                                    match __A::next_value::<bool>(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            __Field::m_simpleTypeHkBool => {
                                #[cfg(
                                    any(feature = "strict", feature = "ignore_duplicates")
                                )]
                                if _serde::__private::Option::is_some(&m_simpleTypeHkBool) {
                                    #[cfg(feature = "ignore_duplicates")]
                                    {
                                        __A::skip_value(&mut __map)?;
                                        continue;
                                    }
                                    #[cfg(feature = "strict")]
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "simpleTypeHkBool",
                                        ),
                                    );
                                }
                                m_simpleTypeHkBool = _serde::__private::Some(
                                    match __A::next_value::<bool>(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            __Field::m_simpleTypeCString => {
                                #[cfg(
                                    any(feature = "strict", feature = "ignore_duplicates")
                                )]
                                if _serde::__private::Option::is_some(
                                    &m_simpleTypeCString,
                                ) {
                                    #[cfg(feature = "ignore_duplicates")]
                                    {
                                        __A::skip_value(&mut __map)?;
                                        continue;
                                    }
                                    #[cfg(feature = "strict")]
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "simpleTypeCString",
                                        ),
                                    );
                                }
                                m_simpleTypeCString = _serde::__private::Some(
                                    match __A::next_value::<CString<'de>>(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            __Field::m_simpleTypeHkStringPtr => {
                                #[cfg(
                                    any(feature = "strict", feature = "ignore_duplicates")
                                )]
                                if _serde::__private::Option::is_some(
                                    &m_simpleTypeHkStringPtr,
                                ) {
                                    #[cfg(feature = "ignore_duplicates")]
                                    {
                                        __A::skip_value(&mut __map)?;
                                        continue;
                                    }
                                    #[cfg(feature = "strict")]
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "simpleTypeHkStringPtr",
                                        ),
                                    );
                                }
                                m_simpleTypeHkStringPtr = _serde::__private::Some(
                                    match __A::next_value::<StringPtr<'de>>(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            __Field::m_simpleTypeHkInt8 => {
                                #[cfg(
                                    any(feature = "strict", feature = "ignore_duplicates")
                                )]
                                if _serde::__private::Option::is_some(&m_simpleTypeHkInt8) {
                                    #[cfg(feature = "ignore_duplicates")]
                                    {
                                        __A::skip_value(&mut __map)?;
                                        continue;
                                    }
                                    #[cfg(feature = "strict")]
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "simpleTypeHkInt8",
                                        ),
                                    );
                                }
                                m_simpleTypeHkInt8 = _serde::__private::Some(
                                    match __A::next_value::<I8<'de>>(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            __Field::m_simpleTypeHkInt16 => {
                                #[cfg(
                                    any(feature = "strict", feature = "ignore_duplicates")
                                )]
                                if _serde::__private::Option::is_some(
                                    &m_simpleTypeHkInt16,
                                ) {
                                    #[cfg(feature = "ignore_duplicates")]
                                    {
                                        __A::skip_value(&mut __map)?;
                                        continue;
                                    }
                                    #[cfg(feature = "strict")]
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "simpleTypeHkInt16",
                                        ),
                                    );
                                }
                                m_simpleTypeHkInt16 = _serde::__private::Some(
                                    match __A::next_value::<I16<'de>>(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            __Field::m_simpleTypeHkInt32 => {
                                #[cfg(
                                    any(feature = "strict", feature = "ignore_duplicates")
                                )]
                                if _serde::__private::Option::is_some(
                                    &m_simpleTypeHkInt32,
                                ) {
                                    #[cfg(feature = "ignore_duplicates")]
                                    {
                                        __A::skip_value(&mut __map)?;
                                        continue;
                                    }
                                    #[cfg(feature = "strict")]
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "simpleTypeHkInt32",
                                        ),
                                    );
                                }
                                m_simpleTypeHkInt32 = _serde::__private::Some(
                                    match __A::next_value::<I32<'de>>(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            __Field::m_simpleTypeHkUint8 => {
                                #[cfg(
                                    any(feature = "strict", feature = "ignore_duplicates")
                                )]
                                if _serde::__private::Option::is_some(
                                    &m_simpleTypeHkUint8,
                                ) {
                                    #[cfg(feature = "ignore_duplicates")]
                                    {
                                        __A::skip_value(&mut __map)?;
                                        continue;
                                    }
                                    #[cfg(feature = "strict")]
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "simpleTypeHkUint8",
                                        ),
                                    );
                                }
                                m_simpleTypeHkUint8 = _serde::__private::Some(
                                    match __A::next_value::<U8<'de>>(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            __Field::m_simpleTypeHkUint16 => {
                                #[cfg(
                                    any(feature = "strict", feature = "ignore_duplicates")
                                )]
                                if _serde::__private::Option::is_some(
                                    &m_simpleTypeHkUint16,
                                ) {
                                    #[cfg(feature = "ignore_duplicates")]
                                    {
                                        __A::skip_value(&mut __map)?;
                                        continue;
                                    }
                                    #[cfg(feature = "strict")]
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "simpleTypeHkUint16",
                                        ),
                                    );
                                }
                                m_simpleTypeHkUint16 = _serde::__private::Some(
                                    match __A::next_value::<U16<'de>>(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            __Field::m_simpleTypeHkUint32 => {
                                #[cfg(
                                    any(feature = "strict", feature = "ignore_duplicates")
                                )]
                                if _serde::__private::Option::is_some(
                                    &m_simpleTypeHkUint32,
                                ) {
                                    #[cfg(feature = "ignore_duplicates")]
                                    {
                                        __A::skip_value(&mut __map)?;
                                        continue;
                                    }
                                    #[cfg(feature = "strict")]
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "simpleTypeHkUint32",
                                        ),
                                    );
                                }
                                m_simpleTypeHkUint32 = _serde::__private::Some(
                                    match __A::next_value::<U32<'de>>(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            __Field::m_simpleTypeHkReal => {
                                #[cfg(
                                    any(feature = "strict", feature = "ignore_duplicates")
                                )]
                                if _serde::__private::Option::is_some(&m_simpleTypeHkReal) {
                                    #[cfg(feature = "ignore_duplicates")]
                                    {
                                        __A::skip_value(&mut __map)?;
                                        continue;
                                    }
                                    #[cfg(feature = "strict")]
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "simpleTypeHkReal",
                                        ),
                                    );
                                }
                                m_simpleTypeHkReal = _serde::__private::Some(
                                    match __A::next_value::<f32>(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            __Field::m_simpleTypeHkInt8Default => {
                                #[cfg(
                                    any(feature = "strict", feature = "ignore_duplicates")
                                )]
                                if _serde::__private::Option::is_some(
                                    &m_simpleTypeHkInt8Default,
                                ) {
                                    #[cfg(feature = "ignore_duplicates")]
                                    {
                                        __A::skip_value(&mut __map)?;
                                        continue;
                                    }
                                    #[cfg(feature = "strict")]
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "simpleTypeHkInt8Default",
                                        ),
                                    );
                                }
                                m_simpleTypeHkInt8Default = _serde::__private::Some(
                                    match __A::next_value::<I8<'de>>(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            __Field::m_simpleTypeHkInt16Default => {
                                #[cfg(
                                    any(feature = "strict", feature = "ignore_duplicates")
                                )]
                                if _serde::__private::Option::is_some(
                                    &m_simpleTypeHkInt16Default,
                                ) {
                                    #[cfg(feature = "ignore_duplicates")]
                                    {
                                        __A::skip_value(&mut __map)?;
                                        continue;
                                    }
                                    #[cfg(feature = "strict")]
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "simpleTypeHkInt16Default",
                                        ),
                                    );
                                }
                                m_simpleTypeHkInt16Default = _serde::__private::Some(
                                    match __A::next_value::<I16<'de>>(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            __Field::m_simpleTypeHkInt32Default => {
                                #[cfg(
                                    any(feature = "strict", feature = "ignore_duplicates")
                                )]
                                if _serde::__private::Option::is_some(
                                    &m_simpleTypeHkInt32Default,
                                ) {
                                    #[cfg(feature = "ignore_duplicates")]
                                    {
                                        __A::skip_value(&mut __map)?;
                                        continue;
                                    }
                                    #[cfg(feature = "strict")]
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "simpleTypeHkInt32Default",
                                        ),
                                    );
                                }
                                m_simpleTypeHkInt32Default = _serde::__private::Some(
                                    match __A::next_value::<I32<'de>>(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            __Field::m_simpleTypeHkUint8Default => {
                                #[cfg(
                                    any(feature = "strict", feature = "ignore_duplicates")
                                )]
                                if _serde::__private::Option::is_some(
                                    &m_simpleTypeHkUint8Default,
                                ) {
                                    #[cfg(feature = "ignore_duplicates")]
                                    {
                                        __A::skip_value(&mut __map)?;
                                        continue;
                                    }
                                    #[cfg(feature = "strict")]
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "simpleTypeHkUint8Default",
                                        ),
                                    );
                                }
                                m_simpleTypeHkUint8Default = _serde::__private::Some(
                                    match __A::next_value::<U8<'de>>(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            __Field::m_simpleTypeHkUint16Default => {
                                #[cfg(
                                    any(feature = "strict", feature = "ignore_duplicates")
                                )]
                                if _serde::__private::Option::is_some(
                                    &m_simpleTypeHkUint16Default,
                                ) {
                                    #[cfg(feature = "ignore_duplicates")]
                                    {
                                        __A::skip_value(&mut __map)?;
                                        continue;
                                    }
                                    #[cfg(feature = "strict")]
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "simpleTypeHkUint16Default",
                                        ),
                                    );
                                }
                                m_simpleTypeHkUint16Default = _serde::__private::Some(
                                    match __A::next_value::<U16<'de>>(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            __Field::m_simpleTypeHkUint32Default => {
                                #[cfg(
                                    any(feature = "strict", feature = "ignore_duplicates")
                                )]
                                if _serde::__private::Option::is_some(
                                    &m_simpleTypeHkUint32Default,
                                ) {
                                    #[cfg(feature = "ignore_duplicates")]
                                    {
                                        __A::skip_value(&mut __map)?;
                                        continue;
                                    }
                                    #[cfg(feature = "strict")]
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "simpleTypeHkUint32Default",
                                        ),
                                    );
                                }
                                m_simpleTypeHkUint32Default = _serde::__private::Some(
                                    match __A::next_value::<U32<'de>>(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            __Field::m_simpleTypeHkRealDefault => {
                                #[cfg(
                                    any(feature = "strict", feature = "ignore_duplicates")
                                )]
                                if _serde::__private::Option::is_some(
                                    &m_simpleTypeHkRealDefault,
                                ) {
                                    #[cfg(feature = "ignore_duplicates")]
                                    {
                                        __A::skip_value(&mut __map)?;
                                        continue;
                                    }
                                    #[cfg(feature = "strict")]
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "simpleTypeHkRealDefault",
                                        ),
                                    );
                                }
                                m_simpleTypeHkRealDefault = _serde::__private::Some(
                                    match __A::next_value::<f32>(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            __Field::m_simpleTypeHkInt8Clamp => {
                                #[cfg(
                                    any(feature = "strict", feature = "ignore_duplicates")
                                )]
                                if _serde::__private::Option::is_some(
                                    &m_simpleTypeHkInt8Clamp,
                                ) {
                                    #[cfg(feature = "ignore_duplicates")]
                                    {
                                        __A::skip_value(&mut __map)?;
                                        continue;
                                    }
                                    #[cfg(feature = "strict")]
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "simpleTypeHkInt8Clamp",
                                        ),
                                    );
                                }
                                m_simpleTypeHkInt8Clamp = _serde::__private::Some(
                                    match __A::next_value::<I8<'de>>(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            __Field::m_simpleTypeHkInt16Clamp => {
                                #[cfg(
                                    any(feature = "strict", feature = "ignore_duplicates")
                                )]
                                if _serde::__private::Option::is_some(
                                    &m_simpleTypeHkInt16Clamp,
                                ) {
                                    #[cfg(feature = "ignore_duplicates")]
                                    {
                                        __A::skip_value(&mut __map)?;
                                        continue;
                                    }
                                    #[cfg(feature = "strict")]
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "simpleTypeHkInt16Clamp",
                                        ),
                                    );
                                }
                                m_simpleTypeHkInt16Clamp = _serde::__private::Some(
                                    match __A::next_value::<I16<'de>>(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            __Field::m_simpleTypeHkInt32Clamp => {
                                #[cfg(
                                    any(feature = "strict", feature = "ignore_duplicates")
                                )]
                                if _serde::__private::Option::is_some(
                                    &m_simpleTypeHkInt32Clamp,
                                ) {
                                    #[cfg(feature = "ignore_duplicates")]
                                    {
                                        __A::skip_value(&mut __map)?;
                                        continue;
                                    }
                                    #[cfg(feature = "strict")]
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "simpleTypeHkInt32Clamp",
                                        ),
                                    );
                                }
                                m_simpleTypeHkInt32Clamp = _serde::__private::Some(
                                    match __A::next_value::<I32<'de>>(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            __Field::m_simpleTypeHkUint8Clamp => {
                                #[cfg(
                                    any(feature = "strict", feature = "ignore_duplicates")
                                )]
                                if _serde::__private::Option::is_some(
                                    &m_simpleTypeHkUint8Clamp,
                                ) {
                                    #[cfg(feature = "ignore_duplicates")]
                                    {
                                        __A::skip_value(&mut __map)?;
                                        continue;
                                    }
                                    #[cfg(feature = "strict")]
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "simpleTypeHkUint8Clamp",
                                        ),
                                    );
                                }
                                m_simpleTypeHkUint8Clamp = _serde::__private::Some(
                                    match __A::next_value::<U8<'de>>(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            __Field::m_simpleTypeHkUint16Clamp => {
                                #[cfg(
                                    any(feature = "strict", feature = "ignore_duplicates")
                                )]
                                if _serde::__private::Option::is_some(
                                    &m_simpleTypeHkUint16Clamp,
                                ) {
                                    #[cfg(feature = "ignore_duplicates")]
                                    {
                                        __A::skip_value(&mut __map)?;
                                        continue;
                                    }
                                    #[cfg(feature = "strict")]
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "simpleTypeHkUint16Clamp",
                                        ),
                                    );
                                }
                                m_simpleTypeHkUint16Clamp = _serde::__private::Some(
                                    match __A::next_value::<U16<'de>>(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            __Field::m_simpleTypeHkUint32Clamp => {
                                #[cfg(
                                    any(feature = "strict", feature = "ignore_duplicates")
                                )]
                                if _serde::__private::Option::is_some(
                                    &m_simpleTypeHkUint32Clamp,
                                ) {
                                    #[cfg(feature = "ignore_duplicates")]
                                    {
                                        __A::skip_value(&mut __map)?;
                                        continue;
                                    }
                                    #[cfg(feature = "strict")]
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "simpleTypeHkUint32Clamp",
                                        ),
                                    );
                                }
                                m_simpleTypeHkUint32Clamp = _serde::__private::Some(
                                    match __A::next_value::<U32<'de>>(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            __Field::m_simpleTypeHkRealClamp => {
                                #[cfg(
                                    any(feature = "strict", feature = "ignore_duplicates")
                                )]
                                if _serde::__private::Option::is_some(
                                    &m_simpleTypeHkRealClamp,
                                ) {
                                    #[cfg(feature = "ignore_duplicates")]
                                    {
                                        __A::skip_value(&mut __map)?;
                                        continue;
                                    }
                                    #[cfg(feature = "strict")]
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "simpleTypeHkRealClamp",
                                        ),
                                    );
                                }
                                m_simpleTypeHkRealClamp = _serde::__private::Some(
                                    match __A::next_value::<f32>(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            __Field::m_simpleHiddenTypeCopyEnd => {
                                #[cfg(
                                    any(feature = "strict", feature = "ignore_duplicates")
                                )]
                                if _serde::__private::Option::is_some(
                                    &m_simpleHiddenTypeCopyEnd,
                                ) {
                                    #[cfg(feature = "ignore_duplicates")]
                                    {
                                        __A::skip_value(&mut __map)?;
                                        continue;
                                    }
                                    #[cfg(feature = "strict")]
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "simpleHiddenTypeCopyEnd",
                                        ),
                                    );
                                }
                                m_simpleHiddenTypeCopyEnd = _serde::__private::Some(
                                    match __A::next_value::<bool>(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            __Field::m_complexTypeHkObjectPtr => {
                                #[cfg(
                                    any(feature = "strict", feature = "ignore_duplicates")
                                )]
                                if _serde::__private::Option::is_some(
                                    &m_complexTypeHkObjectPtr,
                                ) {
                                    #[cfg(feature = "ignore_duplicates")]
                                    {
                                        __A::skip_value(&mut __map)?;
                                        continue;
                                    }
                                    #[cfg(feature = "strict")]
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "complexTypeHkObjectPtr",
                                        ),
                                    );
                                }
                                m_complexTypeHkObjectPtr = _serde::__private::Some(
                                    match __A::next_value::<Pointer<'de>>(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            __Field::m_complexHiddenTypeCopyStart => {
                                #[cfg(
                                    any(feature = "strict", feature = "ignore_duplicates")
                                )]
                                if _serde::__private::Option::is_some(
                                    &m_complexHiddenTypeCopyStart,
                                ) {
                                    #[cfg(feature = "ignore_duplicates")]
                                    {
                                        __A::skip_value(&mut __map)?;
                                        continue;
                                    }
                                    #[cfg(feature = "strict")]
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "complexHiddenTypeCopyStart",
                                        ),
                                    );
                                }
                                m_complexHiddenTypeCopyStart = _serde::__private::Some(
                                    match __A::next_value::<bool>(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            __Field::m_complexTypeHkQuaternion => {
                                #[cfg(
                                    any(feature = "strict", feature = "ignore_duplicates")
                                )]
                                if _serde::__private::Option::is_some(
                                    &m_complexTypeHkQuaternion,
                                ) {
                                    #[cfg(feature = "ignore_duplicates")]
                                    {
                                        __A::skip_value(&mut __map)?;
                                        continue;
                                    }
                                    #[cfg(feature = "strict")]
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "complexTypeHkQuaternion",
                                        ),
                                    );
                                }
                                m_complexTypeHkQuaternion = _serde::__private::Some(
                                    match __A::next_value::<Quaternion>(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            __Field::m_complexTypeHkVector4 => {
                                #[cfg(
                                    any(feature = "strict", feature = "ignore_duplicates")
                                )]
                                if _serde::__private::Option::is_some(
                                    &m_complexTypeHkVector4,
                                ) {
                                    #[cfg(feature = "ignore_duplicates")]
                                    {
                                        __A::skip_value(&mut __map)?;
                                        continue;
                                    }
                                    #[cfg(feature = "strict")]
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "complexTypeHkVector4",
                                        ),
                                    );
                                }
                                m_complexTypeHkVector4 = _serde::__private::Some(
                                    match __A::next_value::<Vector4>(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            __Field::m_complexTypeEnumHkInt8 => {
                                #[cfg(
                                    any(feature = "strict", feature = "ignore_duplicates")
                                )]
                                if _serde::__private::Option::is_some(
                                    &m_complexTypeEnumHkInt8,
                                ) {
                                    #[cfg(feature = "ignore_duplicates")]
                                    {
                                        __A::skip_value(&mut __map)?;
                                        continue;
                                    }
                                    #[cfg(feature = "strict")]
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "complexTypeEnumHkInt8",
                                        ),
                                    );
                                }
                                m_complexTypeEnumHkInt8 = _serde::__private::Some(
                                    match __A::next_value::<CustomEnum>(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            __Field::m_complexTypeEnumHkInt16 => {
                                #[cfg(
                                    any(feature = "strict", feature = "ignore_duplicates")
                                )]
                                if _serde::__private::Option::is_some(
                                    &m_complexTypeEnumHkInt16,
                                ) {
                                    #[cfg(feature = "ignore_duplicates")]
                                    {
                                        __A::skip_value(&mut __map)?;
                                        continue;
                                    }
                                    #[cfg(feature = "strict")]
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "complexTypeEnumHkInt16",
                                        ),
                                    );
                                }
                                m_complexTypeEnumHkInt16 = _serde::__private::Some(
                                    match __A::next_value::<CustomEnum>(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            __Field::m_complexTypeEnumHkInt32 => {
                                #[cfg(
                                    any(feature = "strict", feature = "ignore_duplicates")
                                )]
                                if _serde::__private::Option::is_some(
                                    &m_complexTypeEnumHkInt32,
                                ) {
                                    #[cfg(feature = "ignore_duplicates")]
                                    {
                                        __A::skip_value(&mut __map)?;
                                        continue;
                                    }
                                    #[cfg(feature = "strict")]
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "complexTypeEnumHkInt32",
                                        ),
                                    );
                                }
                                m_complexTypeEnumHkInt32 = _serde::__private::Some(
                                    match __A::next_value::<CustomEnum>(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            __Field::m_complexTypeEnumHkUint8 => {
                                #[cfg(
                                    any(feature = "strict", feature = "ignore_duplicates")
                                )]
                                if _serde::__private::Option::is_some(
                                    &m_complexTypeEnumHkUint8,
                                ) {
                                    #[cfg(feature = "ignore_duplicates")]
                                    {
                                        __A::skip_value(&mut __map)?;
                                        continue;
                                    }
                                    #[cfg(feature = "strict")]
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "complexTypeEnumHkUint8",
                                        ),
                                    );
                                }
                                m_complexTypeEnumHkUint8 = _serde::__private::Some(
                                    match __A::next_value::<CustomEnum>(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            __Field::m_complexTypeEnumHkUint16 => {
                                #[cfg(
                                    any(feature = "strict", feature = "ignore_duplicates")
                                )]
                                if _serde::__private::Option::is_some(
                                    &m_complexTypeEnumHkUint16,
                                ) {
                                    #[cfg(feature = "ignore_duplicates")]
                                    {
                                        __A::skip_value(&mut __map)?;
                                        continue;
                                    }
                                    #[cfg(feature = "strict")]
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "complexTypeEnumHkUint16",
                                        ),
                                    );
                                }
                                m_complexTypeEnumHkUint16 = _serde::__private::Some(
                                    match __A::next_value::<CustomEnum>(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            __Field::m_complexTypeEnumHkUint32 => {
                                #[cfg(
                                    any(feature = "strict", feature = "ignore_duplicates")
                                )]
                                if _serde::__private::Option::is_some(
                                    &m_complexTypeEnumHkUint32,
                                ) {
                                    #[cfg(feature = "ignore_duplicates")]
                                    {
                                        __A::skip_value(&mut __map)?;
                                        continue;
                                    }
                                    #[cfg(feature = "strict")]
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "complexTypeEnumHkUint32",
                                        ),
                                    );
                                }
                                m_complexTypeEnumHkUint32 = _serde::__private::Some(
                                    match __A::next_value::<CustomEnum>(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            __Field::m_complexTypeEnumHkInt8InvalidCheck => {
                                #[cfg(
                                    any(feature = "strict", feature = "ignore_duplicates")
                                )]
                                if _serde::__private::Option::is_some(
                                    &m_complexTypeEnumHkInt8InvalidCheck,
                                ) {
                                    #[cfg(feature = "ignore_duplicates")]
                                    {
                                        __A::skip_value(&mut __map)?;
                                        continue;
                                    }
                                    #[cfg(feature = "strict")]
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "complexTypeEnumHkInt8InvalidCheck",
                                        ),
                                    );
                                }
                                m_complexTypeEnumHkInt8InvalidCheck = _serde::__private::Some(
                                    match __A::next_value::<CustomEnum>(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            __Field::m_complexTypeEnumHkInt16InvalidCheck => {
                                #[cfg(
                                    any(feature = "strict", feature = "ignore_duplicates")
                                )]
                                if _serde::__private::Option::is_some(
                                    &m_complexTypeEnumHkInt16InvalidCheck,
                                ) {
                                    #[cfg(feature = "ignore_duplicates")]
                                    {
                                        __A::skip_value(&mut __map)?;
                                        continue;
                                    }
                                    #[cfg(feature = "strict")]
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "complexTypeEnumHkInt16InvalidCheck",
                                        ),
                                    );
                                }
                                m_complexTypeEnumHkInt16InvalidCheck = _serde::__private::Some(
                                    match __A::next_value::<CustomEnum>(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            __Field::m_complexTypeEnumHkInt32InvalidCheck => {
                                #[cfg(
                                    any(feature = "strict", feature = "ignore_duplicates")
                                )]
                                if _serde::__private::Option::is_some(
                                    &m_complexTypeEnumHkInt32InvalidCheck,
                                ) {
                                    #[cfg(feature = "ignore_duplicates")]
                                    {
                                        __A::skip_value(&mut __map)?;
                                        continue;
                                    }
                                    #[cfg(feature = "strict")]
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "complexTypeEnumHkInt32InvalidCheck",
                                        ),
                                    );
                                }
                                m_complexTypeEnumHkInt32InvalidCheck = _serde::__private::Some(
                                    match __A::next_value::<CustomEnum>(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            __Field::m_complexTypeEnumHkUint8InvalidCheck => {
                                #[cfg(
                                    any(feature = "strict", feature = "ignore_duplicates")
                                )]
                                if _serde::__private::Option::is_some(
                                    &m_complexTypeEnumHkUint8InvalidCheck,
                                ) {
                                    #[cfg(feature = "ignore_duplicates")]
                                    {
                                        __A::skip_value(&mut __map)?;
                                        continue;
                                    }
                                    #[cfg(feature = "strict")]
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "complexTypeEnumHkUint8InvalidCheck",
                                        ),
                                    );
                                }
                                m_complexTypeEnumHkUint8InvalidCheck = _serde::__private::Some(
                                    match __A::next_value::<CustomEnum>(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            __Field::m_complexTypeEnumHkUint16InvalidCheck => {
                                #[cfg(
                                    any(feature = "strict", feature = "ignore_duplicates")
                                )]
                                if _serde::__private::Option::is_some(
                                    &m_complexTypeEnumHkUint16InvalidCheck,
                                ) {
                                    #[cfg(feature = "ignore_duplicates")]
                                    {
                                        __A::skip_value(&mut __map)?;
                                        continue;
                                    }
                                    #[cfg(feature = "strict")]
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "complexTypeEnumHkUint16InvalidCheck",
                                        ),
                                    );
                                }
                                m_complexTypeEnumHkUint16InvalidCheck = _serde::__private::Some(
                                    match __A::next_value::<CustomEnum>(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            __Field::m_complexTypeEnumHkUint32InvalidCheck => {
                                #[cfg(
                                    any(feature = "strict", feature = "ignore_duplicates")
                                )]
                                if _serde::__private::Option::is_some(
                                    &m_complexTypeEnumHkUint32InvalidCheck,
                                ) {
                                    #[cfg(feature = "ignore_duplicates")]
                                    {
                                        __A::skip_value(&mut __map)?;
                                        continue;
                                    }
                                    #[cfg(feature = "strict")]
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "complexTypeEnumHkUint32InvalidCheck",
                                        ),
                                    );
                                }
                                m_complexTypeEnumHkUint32InvalidCheck = _serde::__private::Some(
                                    match __A::next_value::<CustomEnum>(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            __Field::m_complexTypeFlagsHkInt8 => {
                                #[cfg(
                                    any(feature = "strict", feature = "ignore_duplicates")
                                )]
                                if _serde::__private::Option::is_some(
                                    &m_complexTypeFlagsHkInt8,
                                ) {
                                    #[cfg(feature = "ignore_duplicates")]
                                    {
                                        __A::skip_value(&mut __map)?;
                                        continue;
                                    }
                                    #[cfg(feature = "strict")]
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "complexTypeFlagsHkInt8",
                                        ),
                                    );
                                }
                                m_complexTypeFlagsHkInt8 = _serde::__private::Some(
                                    match __A::next_value::<CustomFlag>(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            __Field::m_complexTypeFlagsHkInt16 => {
                                #[cfg(
                                    any(feature = "strict", feature = "ignore_duplicates")
                                )]
                                if _serde::__private::Option::is_some(
                                    &m_complexTypeFlagsHkInt16,
                                ) {
                                    #[cfg(feature = "ignore_duplicates")]
                                    {
                                        __A::skip_value(&mut __map)?;
                                        continue;
                                    }
                                    #[cfg(feature = "strict")]
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "complexTypeFlagsHkInt16",
                                        ),
                                    );
                                }
                                m_complexTypeFlagsHkInt16 = _serde::__private::Some(
                                    match __A::next_value::<CustomFlag>(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            __Field::m_complexTypeFlagsHkInt32 => {
                                #[cfg(
                                    any(feature = "strict", feature = "ignore_duplicates")
                                )]
                                if _serde::__private::Option::is_some(
                                    &m_complexTypeFlagsHkInt32,
                                ) {
                                    #[cfg(feature = "ignore_duplicates")]
                                    {
                                        __A::skip_value(&mut __map)?;
                                        continue;
                                    }
                                    #[cfg(feature = "strict")]
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "complexTypeFlagsHkInt32",
                                        ),
                                    );
                                }
                                m_complexTypeFlagsHkInt32 = _serde::__private::Some(
                                    match __A::next_value::<CustomFlag>(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            __Field::m_complexTypeFlagsHkUint8 => {
                                #[cfg(
                                    any(feature = "strict", feature = "ignore_duplicates")
                                )]
                                if _serde::__private::Option::is_some(
                                    &m_complexTypeFlagsHkUint8,
                                ) {
                                    #[cfg(feature = "ignore_duplicates")]
                                    {
                                        __A::skip_value(&mut __map)?;
                                        continue;
                                    }
                                    #[cfg(feature = "strict")]
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "complexTypeFlagsHkUint8",
                                        ),
                                    );
                                }
                                m_complexTypeFlagsHkUint8 = _serde::__private::Some(
                                    match __A::next_value::<CustomFlag>(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            __Field::m_complexTypeFlagsHkUint16 => {
                                #[cfg(
                                    any(feature = "strict", feature = "ignore_duplicates")
                                )]
                                if _serde::__private::Option::is_some(
                                    &m_complexTypeFlagsHkUint16,
                                ) {
                                    #[cfg(feature = "ignore_duplicates")]
                                    {
                                        __A::skip_value(&mut __map)?;
                                        continue;
                                    }
                                    #[cfg(feature = "strict")]
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "complexTypeFlagsHkUint16",
                                        ),
                                    );
                                }
                                m_complexTypeFlagsHkUint16 = _serde::__private::Some(
                                    match __A::next_value::<CustomFlag>(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            __Field::m_complexTypeFlagsHkUint32 => {
                                #[cfg(
                                    any(feature = "strict", feature = "ignore_duplicates")
                                )]
                                if _serde::__private::Option::is_some(
                                    &m_complexTypeFlagsHkUint32,
                                ) {
                                    #[cfg(feature = "ignore_duplicates")]
                                    {
                                        __A::skip_value(&mut __map)?;
                                        continue;
                                    }
                                    #[cfg(feature = "strict")]
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "complexTypeFlagsHkUint32",
                                        ),
                                    );
                                }
                                m_complexTypeFlagsHkUint32 = _serde::__private::Some(
                                    match __A::next_value::<CustomFlag>(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            __Field::m_complexTypeFlagsHkInt8InvalidCheck => {
                                #[cfg(
                                    any(feature = "strict", feature = "ignore_duplicates")
                                )]
                                if _serde::__private::Option::is_some(
                                    &m_complexTypeFlagsHkInt8InvalidCheck,
                                ) {
                                    #[cfg(feature = "ignore_duplicates")]
                                    {
                                        __A::skip_value(&mut __map)?;
                                        continue;
                                    }
                                    #[cfg(feature = "strict")]
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "complexTypeFlagsHkInt8InvalidCheck",
                                        ),
                                    );
                                }
                                m_complexTypeFlagsHkInt8InvalidCheck = _serde::__private::Some(
                                    match __A::next_value::<CustomFlag>(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            __Field::m_complexTypeFlagsHkInt16InvalidCheck => {
                                #[cfg(
                                    any(feature = "strict", feature = "ignore_duplicates")
                                )]
                                if _serde::__private::Option::is_some(
                                    &m_complexTypeFlagsHkInt16InvalidCheck,
                                ) {
                                    #[cfg(feature = "ignore_duplicates")]
                                    {
                                        __A::skip_value(&mut __map)?;
                                        continue;
                                    }
                                    #[cfg(feature = "strict")]
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "complexTypeFlagsHkInt16InvalidCheck",
                                        ),
                                    );
                                }
                                m_complexTypeFlagsHkInt16InvalidCheck = _serde::__private::Some(
                                    match __A::next_value::<CustomFlag>(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            __Field::m_complexTypeFlagsHkInt32InvalidCheck => {
                                #[cfg(
                                    any(feature = "strict", feature = "ignore_duplicates")
                                )]
                                if _serde::__private::Option::is_some(
                                    &m_complexTypeFlagsHkInt32InvalidCheck,
                                ) {
                                    #[cfg(feature = "ignore_duplicates")]
                                    {
                                        __A::skip_value(&mut __map)?;
                                        continue;
                                    }
                                    #[cfg(feature = "strict")]
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "complexTypeFlagsHkInt32InvalidCheck",
                                        ),
                                    );
                                }
                                m_complexTypeFlagsHkInt32InvalidCheck = _serde::__private::Some(
                                    match __A::next_value::<CustomFlag>(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            __Field::m_complexTypeFlagsHkUint8InvalidCheck => {
                                #[cfg(
                                    any(feature = "strict", feature = "ignore_duplicates")
                                )]
                                if _serde::__private::Option::is_some(
                                    &m_complexTypeFlagsHkUint8InvalidCheck,
                                ) {
                                    #[cfg(feature = "ignore_duplicates")]
                                    {
                                        __A::skip_value(&mut __map)?;
                                        continue;
                                    }
                                    #[cfg(feature = "strict")]
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "complexTypeFlagsHkUint8InvalidCheck",
                                        ),
                                    );
                                }
                                m_complexTypeFlagsHkUint8InvalidCheck = _serde::__private::Some(
                                    match __A::next_value::<CustomFlag>(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            __Field::m_complexTypeFlagsHkUint16InvalidCheck => {
                                #[cfg(
                                    any(feature = "strict", feature = "ignore_duplicates")
                                )]
                                if _serde::__private::Option::is_some(
                                    &m_complexTypeFlagsHkUint16InvalidCheck,
                                ) {
                                    #[cfg(feature = "ignore_duplicates")]
                                    {
                                        __A::skip_value(&mut __map)?;
                                        continue;
                                    }
                                    #[cfg(feature = "strict")]
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "complexTypeFlagsHkUint16InvalidCheck",
                                        ),
                                    );
                                }
                                m_complexTypeFlagsHkUint16InvalidCheck = _serde::__private::Some(
                                    match __A::next_value::<CustomFlag>(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            __Field::m_complexTypeFlagsHkUint32InvalidCheck => {
                                #[cfg(
                                    any(feature = "strict", feature = "ignore_duplicates")
                                )]
                                if _serde::__private::Option::is_some(
                                    &m_complexTypeFlagsHkUint32InvalidCheck,
                                ) {
                                    #[cfg(feature = "ignore_duplicates")]
                                    {
                                        __A::skip_value(&mut __map)?;
                                        continue;
                                    }
                                    #[cfg(feature = "strict")]
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "complexTypeFlagsHkUint32InvalidCheck",
                                        ),
                                    );
                                }
                                m_complexTypeFlagsHkUint32InvalidCheck = _serde::__private::Some(
                                    match __A::next_value::<CustomFlag>(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            __Field::m_complexHiddenTypeCopyEnd => {
                                #[cfg(
                                    any(feature = "strict", feature = "ignore_duplicates")
                                )]
                                if _serde::__private::Option::is_some(
                                    &m_complexHiddenTypeCopyEnd,
                                ) {
                                    #[cfg(feature = "ignore_duplicates")]
                                    {
                                        __A::skip_value(&mut __map)?;
                                        continue;
                                    }
                                    #[cfg(feature = "strict")]
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "complexHiddenTypeCopyEnd",
                                        ),
                                    );
                                }
                                m_complexHiddenTypeCopyEnd = _serde::__private::Some(
                                    match __A::next_value::<bool>(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            __Field::m_nestedTypeHkbGeneratorPtr => {
                                #[cfg(
                                    any(feature = "strict", feature = "ignore_duplicates")
                                )]
                                if _serde::__private::Option::is_some(
                                    &m_nestedTypeHkbGeneratorPtr,
                                ) {
                                    #[cfg(feature = "ignore_duplicates")]
                                    {
                                        __A::skip_value(&mut __map)?;
                                        continue;
                                    }
                                    #[cfg(feature = "strict")]
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "nestedTypeHkbGeneratorPtr",
                                        ),
                                    );
                                }
                                m_nestedTypeHkbGeneratorPtr = _serde::__private::Some(
                                    match __A::next_value::<Pointer<'de>>(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            __Field::m_nestedTypeHkbGeneratorRefPtr => {
                                #[cfg(
                                    any(feature = "strict", feature = "ignore_duplicates")
                                )]
                                if _serde::__private::Option::is_some(
                                    &m_nestedTypeHkbGeneratorRefPtr,
                                ) {
                                    #[cfg(feature = "ignore_duplicates")]
                                    {
                                        __A::skip_value(&mut __map)?;
                                        continue;
                                    }
                                    #[cfg(feature = "strict")]
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "nestedTypeHkbGeneratorRefPtr",
                                        ),
                                    );
                                }
                                m_nestedTypeHkbGeneratorRefPtr = _serde::__private::Some(
                                    match __A::next_value::<Pointer<'de>>(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            __Field::m_nestedTypeHkbModifierPtr => {
                                #[cfg(
                                    any(feature = "strict", feature = "ignore_duplicates")
                                )]
                                if _serde::__private::Option::is_some(
                                    &m_nestedTypeHkbModifierPtr,
                                ) {
                                    #[cfg(feature = "ignore_duplicates")]
                                    {
                                        __A::skip_value(&mut __map)?;
                                        continue;
                                    }
                                    #[cfg(feature = "strict")]
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "nestedTypeHkbModifierPtr",
                                        ),
                                    );
                                }
                                m_nestedTypeHkbModifierPtr = _serde::__private::Some(
                                    match __A::next_value::<Pointer<'de>>(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            __Field::m_nestedTypeHkbModifierRefPtr => {
                                #[cfg(
                                    any(feature = "strict", feature = "ignore_duplicates")
                                )]
                                if _serde::__private::Option::is_some(
                                    &m_nestedTypeHkbModifierRefPtr,
                                ) {
                                    #[cfg(feature = "ignore_duplicates")]
                                    {
                                        __A::skip_value(&mut __map)?;
                                        continue;
                                    }
                                    #[cfg(feature = "strict")]
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "nestedTypeHkbModifierRefPtr",
                                        ),
                                    );
                                }
                                m_nestedTypeHkbModifierRefPtr = _serde::__private::Some(
                                    match __A::next_value::<Pointer<'de>>(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            __Field::m_nestedTypeHkbCustomIdSelectorPtr => {
                                #[cfg(
                                    any(feature = "strict", feature = "ignore_duplicates")
                                )]
                                if _serde::__private::Option::is_some(
                                    &m_nestedTypeHkbCustomIdSelectorPtr,
                                ) {
                                    #[cfg(feature = "ignore_duplicates")]
                                    {
                                        __A::skip_value(&mut __map)?;
                                        continue;
                                    }
                                    #[cfg(feature = "strict")]
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "nestedTypeHkbCustomIdSelectorPtr",
                                        ),
                                    );
                                }
                                m_nestedTypeHkbCustomIdSelectorPtr = _serde::__private::Some(
                                    match __A::next_value::<Pointer<'de>>(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            __Field::m_nestedTypeHkbCustomIdSelectorRefPtr => {
                                #[cfg(
                                    any(feature = "strict", feature = "ignore_duplicates")
                                )]
                                if _serde::__private::Option::is_some(
                                    &m_nestedTypeHkbCustomIdSelectorRefPtr,
                                ) {
                                    #[cfg(feature = "ignore_duplicates")]
                                    {
                                        __A::skip_value(&mut __map)?;
                                        continue;
                                    }
                                    #[cfg(feature = "strict")]
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "nestedTypeHkbCustomIdSelectorRefPtr",
                                        ),
                                    );
                                }
                                m_nestedTypeHkbCustomIdSelectorRefPtr = _serde::__private::Some(
                                    match __A::next_value::<Pointer<'de>>(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            __Field::m_nestedTypeArrayBool => {
                                #[cfg(
                                    any(feature = "strict", feature = "ignore_duplicates")
                                )]
                                if _serde::__private::Option::is_some(
                                    &m_nestedTypeArrayBool,
                                ) {
                                    #[cfg(feature = "ignore_duplicates")]
                                    {
                                        __A::skip_value(&mut __map)?;
                                        continue;
                                    }
                                    #[cfg(feature = "strict")]
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "nestedTypeArrayBool",
                                        ),
                                    );
                                }
                                m_nestedTypeArrayBool = _serde::__private::Some(
                                    match __A::next_value::<Vec<bool>>(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            __Field::m_nestedTypeArrayHkBool => {
                                #[cfg(
                                    any(feature = "strict", feature = "ignore_duplicates")
                                )]
                                if _serde::__private::Option::is_some(
                                    &m_nestedTypeArrayHkBool,
                                ) {
                                    #[cfg(feature = "ignore_duplicates")]
                                    {
                                        __A::skip_value(&mut __map)?;
                                        continue;
                                    }
                                    #[cfg(feature = "strict")]
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "nestedTypeArrayHkBool",
                                        ),
                                    );
                                }
                                m_nestedTypeArrayHkBool = _serde::__private::Some(
                                    match __A::next_value::<Vec<bool>>(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            __Field::m_nestedTypeArrayCString => {
                                #[cfg(
                                    any(feature = "strict", feature = "ignore_duplicates")
                                )]
                                if _serde::__private::Option::is_some(
                                    &m_nestedTypeArrayCString,
                                ) {
                                    #[cfg(feature = "ignore_duplicates")]
                                    {
                                        __A::skip_value(&mut __map)?;
                                        continue;
                                    }
                                    #[cfg(feature = "strict")]
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "nestedTypeArrayCString",
                                        ),
                                    );
                                }
                                m_nestedTypeArrayCString = _serde::__private::Some(
                                    match __A::next_value::<Vec<CString<'de>>>(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            __Field::m_nestedTypeArrayHkStringPtr => {
                                #[cfg(
                                    any(feature = "strict", feature = "ignore_duplicates")
                                )]
                                if _serde::__private::Option::is_some(
                                    &m_nestedTypeArrayHkStringPtr,
                                ) {
                                    #[cfg(feature = "ignore_duplicates")]
                                    {
                                        __A::skip_value(&mut __map)?;
                                        continue;
                                    }
                                    #[cfg(feature = "strict")]
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "nestedTypeArrayHkStringPtr",
                                        ),
                                    );
                                }
                                m_nestedTypeArrayHkStringPtr = _serde::__private::Some(
                                    match __A::next_value::<Vec<StringPtr<'de>>>(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            __Field::m_nestedTypeArrayHkInt8 => {
                                #[cfg(
                                    any(feature = "strict", feature = "ignore_duplicates")
                                )]
                                if _serde::__private::Option::is_some(
                                    &m_nestedTypeArrayHkInt8,
                                ) {
                                    #[cfg(feature = "ignore_duplicates")]
                                    {
                                        __A::skip_value(&mut __map)?;
                                        continue;
                                    }
                                    #[cfg(feature = "strict")]
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "nestedTypeArrayHkInt8",
                                        ),
                                    );
                                }
                                m_nestedTypeArrayHkInt8 = _serde::__private::Some(
                                    match __A::next_value::<Vec<I8<'de>>>(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            __Field::m_nestedTypeArrayHkInt16 => {
                                #[cfg(
                                    any(feature = "strict", feature = "ignore_duplicates")
                                )]
                                if _serde::__private::Option::is_some(
                                    &m_nestedTypeArrayHkInt16,
                                ) {
                                    #[cfg(feature = "ignore_duplicates")]
                                    {
                                        __A::skip_value(&mut __map)?;
                                        continue;
                                    }
                                    #[cfg(feature = "strict")]
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "nestedTypeArrayHkInt16",
                                        ),
                                    );
                                }
                                m_nestedTypeArrayHkInt16 = _serde::__private::Some(
                                    match __A::next_value::<Vec<I16<'de>>>(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            __Field::m_nestedTypeArrayHkInt32 => {
                                #[cfg(
                                    any(feature = "strict", feature = "ignore_duplicates")
                                )]
                                if _serde::__private::Option::is_some(
                                    &m_nestedTypeArrayHkInt32,
                                ) {
                                    #[cfg(feature = "ignore_duplicates")]
                                    {
                                        __A::skip_value(&mut __map)?;
                                        continue;
                                    }
                                    #[cfg(feature = "strict")]
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "nestedTypeArrayHkInt32",
                                        ),
                                    );
                                }
                                m_nestedTypeArrayHkInt32 = _serde::__private::Some(
                                    match __A::next_value::<Vec<I32<'de>>>(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            __Field::m_nestedTypeArrayHkUint8 => {
                                #[cfg(
                                    any(feature = "strict", feature = "ignore_duplicates")
                                )]
                                if _serde::__private::Option::is_some(
                                    &m_nestedTypeArrayHkUint8,
                                ) {
                                    #[cfg(feature = "ignore_duplicates")]
                                    {
                                        __A::skip_value(&mut __map)?;
                                        continue;
                                    }
                                    #[cfg(feature = "strict")]
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "nestedTypeArrayHkUint8",
                                        ),
                                    );
                                }
                                m_nestedTypeArrayHkUint8 = _serde::__private::Some(
                                    match __A::next_value::<Vec<U8<'de>>>(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            __Field::m_nestedTypeArrayHkUint16 => {
                                #[cfg(
                                    any(feature = "strict", feature = "ignore_duplicates")
                                )]
                                if _serde::__private::Option::is_some(
                                    &m_nestedTypeArrayHkUint16,
                                ) {
                                    #[cfg(feature = "ignore_duplicates")]
                                    {
                                        __A::skip_value(&mut __map)?;
                                        continue;
                                    }
                                    #[cfg(feature = "strict")]
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "nestedTypeArrayHkUint16",
                                        ),
                                    );
                                }
                                m_nestedTypeArrayHkUint16 = _serde::__private::Some(
                                    match __A::next_value::<Vec<U16<'de>>>(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            __Field::m_nestedTypeArrayHkUint32 => {
                                #[cfg(
                                    any(feature = "strict", feature = "ignore_duplicates")
                                )]
                                if _serde::__private::Option::is_some(
                                    &m_nestedTypeArrayHkUint32,
                                ) {
                                    #[cfg(feature = "ignore_duplicates")]
                                    {
                                        __A::skip_value(&mut __map)?;
                                        continue;
                                    }
                                    #[cfg(feature = "strict")]
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "nestedTypeArrayHkUint32",
                                        ),
                                    );
                                }
                                m_nestedTypeArrayHkUint32 = _serde::__private::Some(
                                    match __A::next_value::<Vec<U32<'de>>>(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            __Field::m_nestedTypeArrayHkReal => {
                                #[cfg(
                                    any(feature = "strict", feature = "ignore_duplicates")
                                )]
                                if _serde::__private::Option::is_some(
                                    &m_nestedTypeArrayHkReal,
                                ) {
                                    #[cfg(feature = "ignore_duplicates")]
                                    {
                                        __A::skip_value(&mut __map)?;
                                        continue;
                                    }
                                    #[cfg(feature = "strict")]
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "nestedTypeArrayHkReal",
                                        ),
                                    );
                                }
                                m_nestedTypeArrayHkReal = _serde::__private::Some(
                                    match __A::next_value::<Vec<f32>>(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            __Field::m_nestedTypeArrayHkbGeneratorPtr => {
                                #[cfg(
                                    any(feature = "strict", feature = "ignore_duplicates")
                                )]
                                if _serde::__private::Option::is_some(
                                    &m_nestedTypeArrayHkbGeneratorPtr,
                                ) {
                                    #[cfg(feature = "ignore_duplicates")]
                                    {
                                        __A::skip_value(&mut __map)?;
                                        continue;
                                    }
                                    #[cfg(feature = "strict")]
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "nestedTypeArrayHkbGeneratorPtr",
                                        ),
                                    );
                                }
                                m_nestedTypeArrayHkbGeneratorPtr = _serde::__private::Some(
                                    match __A::next_value::<Vec<Pointer<'de>>>(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            __Field::m_nestedTypeArrayHkbGeneratorRefPtr => {
                                #[cfg(
                                    any(feature = "strict", feature = "ignore_duplicates")
                                )]
                                if _serde::__private::Option::is_some(
                                    &m_nestedTypeArrayHkbGeneratorRefPtr,
                                ) {
                                    #[cfg(feature = "ignore_duplicates")]
                                    {
                                        __A::skip_value(&mut __map)?;
                                        continue;
                                    }
                                    #[cfg(feature = "strict")]
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "nestedTypeArrayHkbGeneratorRefPtr",
                                        ),
                                    );
                                }
                                m_nestedTypeArrayHkbGeneratorRefPtr = _serde::__private::Some(
                                    match __A::next_value::<Vec<Pointer<'de>>>(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            __Field::m_nestedTypeArrayHkbModifierPtr => {
                                #[cfg(
                                    any(feature = "strict", feature = "ignore_duplicates")
                                )]
                                if _serde::__private::Option::is_some(
                                    &m_nestedTypeArrayHkbModifierPtr,
                                ) {
                                    #[cfg(feature = "ignore_duplicates")]
                                    {
                                        __A::skip_value(&mut __map)?;
                                        continue;
                                    }
                                    #[cfg(feature = "strict")]
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "nestedTypeArrayHkbModifierPtr",
                                        ),
                                    );
                                }
                                m_nestedTypeArrayHkbModifierPtr = _serde::__private::Some(
                                    match __A::next_value::<Vec<Pointer<'de>>>(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            __Field::m_nestedTypeArrayHkbModifierRefPtr => {
                                #[cfg(
                                    any(feature = "strict", feature = "ignore_duplicates")
                                )]
                                if _serde::__private::Option::is_some(
                                    &m_nestedTypeArrayHkbModifierRefPtr,
                                ) {
                                    #[cfg(feature = "ignore_duplicates")]
                                    {
                                        __A::skip_value(&mut __map)?;
                                        continue;
                                    }
                                    #[cfg(feature = "strict")]
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "nestedTypeArrayHkbModifierRefPtr",
                                        ),
                                    );
                                }
                                m_nestedTypeArrayHkbModifierRefPtr = _serde::__private::Some(
                                    match __A::next_value::<Vec<Pointer<'de>>>(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            __Field::m_nestedTypeArrayHkbCustomIdSelectorPtr => {
                                #[cfg(
                                    any(feature = "strict", feature = "ignore_duplicates")
                                )]
                                if _serde::__private::Option::is_some(
                                    &m_nestedTypeArrayHkbCustomIdSelectorPtr,
                                ) {
                                    #[cfg(feature = "ignore_duplicates")]
                                    {
                                        __A::skip_value(&mut __map)?;
                                        continue;
                                    }
                                    #[cfg(feature = "strict")]
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "nestedTypeArrayHkbCustomIdSelectorPtr",
                                        ),
                                    );
                                }
                                m_nestedTypeArrayHkbCustomIdSelectorPtr = _serde::__private::Some(
                                    match __A::next_value::<Vec<Pointer<'de>>>(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            __Field::m_nestedTypeArrayHkbCustomIdSelectorRefPtr => {
                                #[cfg(
                                    any(feature = "strict", feature = "ignore_duplicates")
                                )]
                                if _serde::__private::Option::is_some(
                                    &m_nestedTypeArrayHkbCustomIdSelectorRefPtr,
                                ) {
                                    #[cfg(feature = "ignore_duplicates")]
                                    {
                                        __A::skip_value(&mut __map)?;
                                        continue;
                                    }
                                    #[cfg(feature = "strict")]
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "nestedTypeArrayHkbCustomIdSelectorRefPtr",
                                        ),
                                    );
                                }
                                m_nestedTypeArrayHkbCustomIdSelectorRefPtr = _serde::__private::Some(
                                    match __A::next_value::<Vec<Pointer<'de>>>(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            __Field::m_nestedTypeStruct => {
                                #[cfg(
                                    any(feature = "strict", feature = "ignore_duplicates")
                                )]
                                if _serde::__private::Option::is_some(&m_nestedTypeStruct) {
                                    #[cfg(feature = "ignore_duplicates")]
                                    {
                                        __A::skip_value(&mut __map)?;
                                        continue;
                                    }
                                    #[cfg(feature = "strict")]
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "nestedTypeStruct",
                                        ),
                                    );
                                }
                                m_nestedTypeStruct = _serde::__private::Some(
                                    match __A::next_value::<
                                        hkbCustomTestGeneratorNestedTypesBase<'de>,
                                    >(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            __Field::m_nestedTypeArrayStruct => {
                                #[cfg(
                                    any(feature = "strict", feature = "ignore_duplicates")
                                )]
                                if _serde::__private::Option::is_some(
                                    &m_nestedTypeArrayStruct,
                                ) {
                                    #[cfg(feature = "ignore_duplicates")]
                                    {
                                        __A::skip_value(&mut __map)?;
                                        continue;
                                    }
                                    #[cfg(feature = "strict")]
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "nestedTypeArrayStruct",
                                        ),
                                    );
                                }
                                m_nestedTypeArrayStruct = _serde::__private::Some(
                                    match __A::next_value::<
                                        Vec<hkbCustomTestGeneratorNestedTypesBase<'de>>,
                                    >(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            __Field::m_boneHiddenTypeCopyStart => {
                                #[cfg(
                                    any(feature = "strict", feature = "ignore_duplicates")
                                )]
                                if _serde::__private::Option::is_some(
                                    &m_boneHiddenTypeCopyStart,
                                ) {
                                    #[cfg(feature = "ignore_duplicates")]
                                    {
                                        __A::skip_value(&mut __map)?;
                                        continue;
                                    }
                                    #[cfg(feature = "strict")]
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "boneHiddenTypeCopyStart",
                                        ),
                                    );
                                }
                                m_boneHiddenTypeCopyStart = _serde::__private::Some(
                                    match __A::next_value::<bool>(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            __Field::m_oldBoneIndex => {
                                #[cfg(
                                    any(feature = "strict", feature = "ignore_duplicates")
                                )]
                                if _serde::__private::Option::is_some(&m_oldBoneIndex) {
                                    #[cfg(feature = "ignore_duplicates")]
                                    {
                                        __A::skip_value(&mut __map)?;
                                        continue;
                                    }
                                    #[cfg(feature = "strict")]
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "oldBoneIndex",
                                        ),
                                    );
                                }
                                m_oldBoneIndex = _serde::__private::Some(
                                    match __A::next_value::<I16<'de>>(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            __Field::m_oldBoneIndexNoVar => {
                                #[cfg(
                                    any(feature = "strict", feature = "ignore_duplicates")
                                )]
                                if _serde::__private::Option::is_some(
                                    &m_oldBoneIndexNoVar,
                                ) {
                                    #[cfg(feature = "ignore_duplicates")]
                                    {
                                        __A::skip_value(&mut __map)?;
                                        continue;
                                    }
                                    #[cfg(feature = "strict")]
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "oldBoneIndexNoVar",
                                        ),
                                    );
                                }
                                m_oldBoneIndexNoVar = _serde::__private::Some(
                                    match __A::next_value::<I16<'de>>(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            __Field::m_boneIndex => {
                                #[cfg(
                                    any(feature = "strict", feature = "ignore_duplicates")
                                )]
                                if _serde::__private::Option::is_some(&m_boneIndex) {
                                    #[cfg(feature = "ignore_duplicates")]
                                    {
                                        __A::skip_value(&mut __map)?;
                                        continue;
                                    }
                                    #[cfg(feature = "strict")]
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "boneIndex",
                                        ),
                                    );
                                }
                                m_boneIndex = _serde::__private::Some(
                                    match __A::next_value::<I16<'de>>(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            __Field::m_boneIndexNoVar => {
                                #[cfg(
                                    any(feature = "strict", feature = "ignore_duplicates")
                                )]
                                if _serde::__private::Option::is_some(&m_boneIndexNoVar) {
                                    #[cfg(feature = "ignore_duplicates")]
                                    {
                                        __A::skip_value(&mut __map)?;
                                        continue;
                                    }
                                    #[cfg(feature = "strict")]
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "boneIndexNoVar",
                                        ),
                                    );
                                }
                                m_boneIndexNoVar = _serde::__private::Some(
                                    match __A::next_value::<I16<'de>>(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            __Field::m_boneChainIndex0 => {
                                #[cfg(
                                    any(feature = "strict", feature = "ignore_duplicates")
                                )]
                                if _serde::__private::Option::is_some(&m_boneChainIndex0) {
                                    #[cfg(feature = "ignore_duplicates")]
                                    {
                                        __A::skip_value(&mut __map)?;
                                        continue;
                                    }
                                    #[cfg(feature = "strict")]
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "boneChainIndex0",
                                        ),
                                    );
                                }
                                m_boneChainIndex0 = _serde::__private::Some(
                                    match __A::next_value::<I16<'de>>(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            __Field::m_boneChainIndex1 => {
                                #[cfg(
                                    any(feature = "strict", feature = "ignore_duplicates")
                                )]
                                if _serde::__private::Option::is_some(&m_boneChainIndex1) {
                                    #[cfg(feature = "ignore_duplicates")]
                                    {
                                        __A::skip_value(&mut __map)?;
                                        continue;
                                    }
                                    #[cfg(feature = "strict")]
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "boneChainIndex1",
                                        ),
                                    );
                                }
                                m_boneChainIndex1 = _serde::__private::Some(
                                    match __A::next_value::<I16<'de>>(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            __Field::m_boneChainIndex2 => {
                                #[cfg(
                                    any(feature = "strict", feature = "ignore_duplicates")
                                )]
                                if _serde::__private::Option::is_some(&m_boneChainIndex2) {
                                    #[cfg(feature = "ignore_duplicates")]
                                    {
                                        __A::skip_value(&mut __map)?;
                                        continue;
                                    }
                                    #[cfg(feature = "strict")]
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "boneChainIndex2",
                                        ),
                                    );
                                }
                                m_boneChainIndex2 = _serde::__private::Some(
                                    match __A::next_value::<I16<'de>>(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            __Field::m_boneContractIndex0 => {
                                #[cfg(
                                    any(feature = "strict", feature = "ignore_duplicates")
                                )]
                                if _serde::__private::Option::is_some(
                                    &m_boneContractIndex0,
                                ) {
                                    #[cfg(feature = "ignore_duplicates")]
                                    {
                                        __A::skip_value(&mut __map)?;
                                        continue;
                                    }
                                    #[cfg(feature = "strict")]
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "boneContractIndex0",
                                        ),
                                    );
                                }
                                m_boneContractIndex0 = _serde::__private::Some(
                                    match __A::next_value::<I16<'de>>(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            __Field::m_boneContractIndex1 => {
                                #[cfg(
                                    any(feature = "strict", feature = "ignore_duplicates")
                                )]
                                if _serde::__private::Option::is_some(
                                    &m_boneContractIndex1,
                                ) {
                                    #[cfg(feature = "ignore_duplicates")]
                                    {
                                        __A::skip_value(&mut __map)?;
                                        continue;
                                    }
                                    #[cfg(feature = "strict")]
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "boneContractIndex1",
                                        ),
                                    );
                                }
                                m_boneContractIndex1 = _serde::__private::Some(
                                    match __A::next_value::<I16<'de>>(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            __Field::m_boneContractIndex2 => {
                                #[cfg(
                                    any(feature = "strict", feature = "ignore_duplicates")
                                )]
                                if _serde::__private::Option::is_some(
                                    &m_boneContractIndex2,
                                ) {
                                    #[cfg(feature = "ignore_duplicates")]
                                    {
                                        __A::skip_value(&mut __map)?;
                                        continue;
                                    }
                                    #[cfg(feature = "strict")]
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "boneContractIndex2",
                                        ),
                                    );
                                }
                                m_boneContractIndex2 = _serde::__private::Some(
                                    match __A::next_value::<I16<'de>>(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            __Field::m_boneHiddenTypeCopyEnd => {
                                #[cfg(
                                    any(feature = "strict", feature = "ignore_duplicates")
                                )]
                                if _serde::__private::Option::is_some(
                                    &m_boneHiddenTypeCopyEnd,
                                ) {
                                    #[cfg(feature = "ignore_duplicates")]
                                    {
                                        __A::skip_value(&mut __map)?;
                                        continue;
                                    }
                                    #[cfg(feature = "strict")]
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "boneHiddenTypeCopyEnd",
                                        ),
                                    );
                                }
                                m_boneHiddenTypeCopyEnd = _serde::__private::Some(
                                    match __A::next_value::<bool>(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            __Field::m_boneWeightArray => {
                                #[cfg(
                                    any(feature = "strict", feature = "ignore_duplicates")
                                )]
                                if _serde::__private::Option::is_some(&m_boneWeightArray) {
                                    #[cfg(feature = "ignore_duplicates")]
                                    {
                                        __A::skip_value(&mut __map)?;
                                        continue;
                                    }
                                    #[cfg(feature = "strict")]
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "boneWeightArray",
                                        ),
                                    );
                                }
                                m_boneWeightArray = _serde::__private::Some(
                                    match __A::next_value::<Pointer<'de>>(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            __Field::m_boneIndexArray => {
                                #[cfg(
                                    any(feature = "strict", feature = "ignore_duplicates")
                                )]
                                if _serde::__private::Option::is_some(&m_boneIndexArray) {
                                    #[cfg(feature = "ignore_duplicates")]
                                    {
                                        __A::skip_value(&mut __map)?;
                                        continue;
                                    }
                                    #[cfg(feature = "strict")]
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "boneIndexArray",
                                        ),
                                    );
                                }
                                m_boneIndexArray = _serde::__private::Some(
                                    match __A::next_value::<Pointer<'de>>(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            __Field::m_annotatedTypeCStringFilename => {
                                #[cfg(
                                    any(feature = "strict", feature = "ignore_duplicates")
                                )]
                                if _serde::__private::Option::is_some(
                                    &m_annotatedTypeCStringFilename,
                                ) {
                                    #[cfg(feature = "ignore_duplicates")]
                                    {
                                        __A::skip_value(&mut __map)?;
                                        continue;
                                    }
                                    #[cfg(feature = "strict")]
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "annotatedTypeCStringFilename",
                                        ),
                                    );
                                }
                                m_annotatedTypeCStringFilename = _serde::__private::Some(
                                    match __A::next_value::<CString<'de>>(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            __Field::m_annotatedTypeHkStringPtrFilename => {
                                #[cfg(
                                    any(feature = "strict", feature = "ignore_duplicates")
                                )]
                                if _serde::__private::Option::is_some(
                                    &m_annotatedTypeHkStringPtrFilename,
                                ) {
                                    #[cfg(feature = "ignore_duplicates")]
                                    {
                                        __A::skip_value(&mut __map)?;
                                        continue;
                                    }
                                    #[cfg(feature = "strict")]
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "annotatedTypeHkStringPtrFilename",
                                        ),
                                    );
                                }
                                m_annotatedTypeHkStringPtrFilename = _serde::__private::Some(
                                    match __A::next_value::<StringPtr<'de>>(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            __Field::m_annotatedTypeCStringScript => {
                                #[cfg(
                                    any(feature = "strict", feature = "ignore_duplicates")
                                )]
                                if _serde::__private::Option::is_some(
                                    &m_annotatedTypeCStringScript,
                                ) {
                                    #[cfg(feature = "ignore_duplicates")]
                                    {
                                        __A::skip_value(&mut __map)?;
                                        continue;
                                    }
                                    #[cfg(feature = "strict")]
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "annotatedTypeCStringScript",
                                        ),
                                    );
                                }
                                m_annotatedTypeCStringScript = _serde::__private::Some(
                                    match __A::next_value::<CString<'de>>(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            __Field::m_annotatedTypeHkStringPtrScript => {
                                #[cfg(
                                    any(feature = "strict", feature = "ignore_duplicates")
                                )]
                                if _serde::__private::Option::is_some(
                                    &m_annotatedTypeHkStringPtrScript,
                                ) {
                                    #[cfg(feature = "ignore_duplicates")]
                                    {
                                        __A::skip_value(&mut __map)?;
                                        continue;
                                    }
                                    #[cfg(feature = "strict")]
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "annotatedTypeHkStringPtrScript",
                                        ),
                                    );
                                }
                                m_annotatedTypeHkStringPtrScript = _serde::__private::Some(
                                    match __A::next_value::<StringPtr<'de>>(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            __Field::m_annotatedTypeCStringBoneAttachment => {
                                #[cfg(
                                    any(feature = "strict", feature = "ignore_duplicates")
                                )]
                                if _serde::__private::Option::is_some(
                                    &m_annotatedTypeCStringBoneAttachment,
                                ) {
                                    #[cfg(feature = "ignore_duplicates")]
                                    {
                                        __A::skip_value(&mut __map)?;
                                        continue;
                                    }
                                    #[cfg(feature = "strict")]
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "annotatedTypeCStringBoneAttachment",
                                        ),
                                    );
                                }
                                m_annotatedTypeCStringBoneAttachment = _serde::__private::Some(
                                    match __A::next_value::<CString<'de>>(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            __Field::m_annotatedTypeHkStringPtrBoneAttachment => {
                                #[cfg(
                                    any(feature = "strict", feature = "ignore_duplicates")
                                )]
                                if _serde::__private::Option::is_some(
                                    &m_annotatedTypeHkStringPtrBoneAttachment,
                                ) {
                                    #[cfg(feature = "ignore_duplicates")]
                                    {
                                        __A::skip_value(&mut __map)?;
                                        continue;
                                    }
                                    #[cfg(feature = "strict")]
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "annotatedTypeHkStringPtrBoneAttachment",
                                        ),
                                    );
                                }
                                m_annotatedTypeHkStringPtrBoneAttachment = _serde::__private::Some(
                                    match __A::next_value::<StringPtr<'de>>(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            __Field::m_annotatedTypeCStringLocalFrame => {
                                #[cfg(
                                    any(feature = "strict", feature = "ignore_duplicates")
                                )]
                                if _serde::__private::Option::is_some(
                                    &m_annotatedTypeCStringLocalFrame,
                                ) {
                                    #[cfg(feature = "ignore_duplicates")]
                                    {
                                        __A::skip_value(&mut __map)?;
                                        continue;
                                    }
                                    #[cfg(feature = "strict")]
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "annotatedTypeCStringLocalFrame",
                                        ),
                                    );
                                }
                                m_annotatedTypeCStringLocalFrame = _serde::__private::Some(
                                    match __A::next_value::<CString<'de>>(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            __Field::m_annotatedTypeHkStringPtrLocalFrame => {
                                #[cfg(
                                    any(feature = "strict", feature = "ignore_duplicates")
                                )]
                                if _serde::__private::Option::is_some(
                                    &m_annotatedTypeHkStringPtrLocalFrame,
                                ) {
                                    #[cfg(feature = "ignore_duplicates")]
                                    {
                                        __A::skip_value(&mut __map)?;
                                        continue;
                                    }
                                    #[cfg(feature = "strict")]
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "annotatedTypeHkStringPtrLocalFrame",
                                        ),
                                    );
                                }
                                m_annotatedTypeHkStringPtrLocalFrame = _serde::__private::Some(
                                    match __A::next_value::<StringPtr<'de>>(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            __Field::m_annotatedHiddenTypeCopyStart => {
                                #[cfg(
                                    any(feature = "strict", feature = "ignore_duplicates")
                                )]
                                if _serde::__private::Option::is_some(
                                    &m_annotatedHiddenTypeCopyStart,
                                ) {
                                    #[cfg(feature = "ignore_duplicates")]
                                    {
                                        __A::skip_value(&mut __map)?;
                                        continue;
                                    }
                                    #[cfg(feature = "strict")]
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "annotatedHiddenTypeCopyStart",
                                        ),
                                    );
                                }
                                m_annotatedHiddenTypeCopyStart = _serde::__private::Some(
                                    match __A::next_value::<bool>(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            __Field::m_annotatedTypeHkInt32EventID => {
                                #[cfg(
                                    any(feature = "strict", feature = "ignore_duplicates")
                                )]
                                if _serde::__private::Option::is_some(
                                    &m_annotatedTypeHkInt32EventID,
                                ) {
                                    #[cfg(feature = "ignore_duplicates")]
                                    {
                                        __A::skip_value(&mut __map)?;
                                        continue;
                                    }
                                    #[cfg(feature = "strict")]
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "annotatedTypeHkInt32EventID",
                                        ),
                                    );
                                }
                                m_annotatedTypeHkInt32EventID = _serde::__private::Some(
                                    match __A::next_value::<I32<'de>>(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            __Field::m_annotatedTypeHkInt32VariableIndex => {
                                #[cfg(
                                    any(feature = "strict", feature = "ignore_duplicates")
                                )]
                                if _serde::__private::Option::is_some(
                                    &m_annotatedTypeHkInt32VariableIndex,
                                ) {
                                    #[cfg(feature = "ignore_duplicates")]
                                    {
                                        __A::skip_value(&mut __map)?;
                                        continue;
                                    }
                                    #[cfg(feature = "strict")]
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "annotatedTypeHkInt32VariableIndex",
                                        ),
                                    );
                                }
                                m_annotatedTypeHkInt32VariableIndex = _serde::__private::Some(
                                    match __A::next_value::<I32<'de>>(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            __Field::m_annotatedTypeHkInt32AttributeIndex => {
                                #[cfg(
                                    any(feature = "strict", feature = "ignore_duplicates")
                                )]
                                if _serde::__private::Option::is_some(
                                    &m_annotatedTypeHkInt32AttributeIndex,
                                ) {
                                    #[cfg(feature = "ignore_duplicates")]
                                    {
                                        __A::skip_value(&mut __map)?;
                                        continue;
                                    }
                                    #[cfg(feature = "strict")]
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "annotatedTypeHkInt32AttributeIndex",
                                        ),
                                    );
                                }
                                m_annotatedTypeHkInt32AttributeIndex = _serde::__private::Some(
                                    match __A::next_value::<I32<'de>>(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            __Field::m_annotatedTypeHkRealTime => {
                                #[cfg(
                                    any(feature = "strict", feature = "ignore_duplicates")
                                )]
                                if _serde::__private::Option::is_some(
                                    &m_annotatedTypeHkRealTime,
                                ) {
                                    #[cfg(feature = "ignore_duplicates")]
                                    {
                                        __A::skip_value(&mut __map)?;
                                        continue;
                                    }
                                    #[cfg(feature = "strict")]
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "annotatedTypeHkRealTime",
                                        ),
                                    );
                                }
                                m_annotatedTypeHkRealTime = _serde::__private::Some(
                                    match __A::next_value::<f32>(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            __Field::m_annotatedTypeBoolNoVar => {
                                #[cfg(
                                    any(feature = "strict", feature = "ignore_duplicates")
                                )]
                                if _serde::__private::Option::is_some(
                                    &m_annotatedTypeBoolNoVar,
                                ) {
                                    #[cfg(feature = "ignore_duplicates")]
                                    {
                                        __A::skip_value(&mut __map)?;
                                        continue;
                                    }
                                    #[cfg(feature = "strict")]
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "annotatedTypeBoolNoVar",
                                        ),
                                    );
                                }
                                m_annotatedTypeBoolNoVar = _serde::__private::Some(
                                    match __A::next_value::<bool>(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            __Field::m_annotatedTypeHkBoolNoVar => {
                                #[cfg(
                                    any(feature = "strict", feature = "ignore_duplicates")
                                )]
                                if _serde::__private::Option::is_some(
                                    &m_annotatedTypeHkBoolNoVar,
                                ) {
                                    #[cfg(feature = "ignore_duplicates")]
                                    {
                                        __A::skip_value(&mut __map)?;
                                        continue;
                                    }
                                    #[cfg(feature = "strict")]
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "annotatedTypeHkBoolNoVar",
                                        ),
                                    );
                                }
                                m_annotatedTypeHkBoolNoVar = _serde::__private::Some(
                                    match __A::next_value::<bool>(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            __Field::m_annotatedTypeHkInt8NoVar => {
                                #[cfg(
                                    any(feature = "strict", feature = "ignore_duplicates")
                                )]
                                if _serde::__private::Option::is_some(
                                    &m_annotatedTypeHkInt8NoVar,
                                ) {
                                    #[cfg(feature = "ignore_duplicates")]
                                    {
                                        __A::skip_value(&mut __map)?;
                                        continue;
                                    }
                                    #[cfg(feature = "strict")]
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "annotatedTypeHkInt8NoVar",
                                        ),
                                    );
                                }
                                m_annotatedTypeHkInt8NoVar = _serde::__private::Some(
                                    match __A::next_value::<I8<'de>>(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            __Field::m_annotatedTypeHkInt16NoVar => {
                                #[cfg(
                                    any(feature = "strict", feature = "ignore_duplicates")
                                )]
                                if _serde::__private::Option::is_some(
                                    &m_annotatedTypeHkInt16NoVar,
                                ) {
                                    #[cfg(feature = "ignore_duplicates")]
                                    {
                                        __A::skip_value(&mut __map)?;
                                        continue;
                                    }
                                    #[cfg(feature = "strict")]
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "annotatedTypeHkInt16NoVar",
                                        ),
                                    );
                                }
                                m_annotatedTypeHkInt16NoVar = _serde::__private::Some(
                                    match __A::next_value::<I16<'de>>(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            __Field::m_annotatedTypeHkInt32NoVar => {
                                #[cfg(
                                    any(feature = "strict", feature = "ignore_duplicates")
                                )]
                                if _serde::__private::Option::is_some(
                                    &m_annotatedTypeHkInt32NoVar,
                                ) {
                                    #[cfg(feature = "ignore_duplicates")]
                                    {
                                        __A::skip_value(&mut __map)?;
                                        continue;
                                    }
                                    #[cfg(feature = "strict")]
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "annotatedTypeHkInt32NoVar",
                                        ),
                                    );
                                }
                                m_annotatedTypeHkInt32NoVar = _serde::__private::Some(
                                    match __A::next_value::<I32<'de>>(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            __Field::m_annotatedTypeHkUint8NoVar => {
                                #[cfg(
                                    any(feature = "strict", feature = "ignore_duplicates")
                                )]
                                if _serde::__private::Option::is_some(
                                    &m_annotatedTypeHkUint8NoVar,
                                ) {
                                    #[cfg(feature = "ignore_duplicates")]
                                    {
                                        __A::skip_value(&mut __map)?;
                                        continue;
                                    }
                                    #[cfg(feature = "strict")]
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "annotatedTypeHkUint8NoVar",
                                        ),
                                    );
                                }
                                m_annotatedTypeHkUint8NoVar = _serde::__private::Some(
                                    match __A::next_value::<U8<'de>>(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            __Field::m_annotatedTypeHkUint16NoVar => {
                                #[cfg(
                                    any(feature = "strict", feature = "ignore_duplicates")
                                )]
                                if _serde::__private::Option::is_some(
                                    &m_annotatedTypeHkUint16NoVar,
                                ) {
                                    #[cfg(feature = "ignore_duplicates")]
                                    {
                                        __A::skip_value(&mut __map)?;
                                        continue;
                                    }
                                    #[cfg(feature = "strict")]
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "annotatedTypeHkUint16NoVar",
                                        ),
                                    );
                                }
                                m_annotatedTypeHkUint16NoVar = _serde::__private::Some(
                                    match __A::next_value::<U16<'de>>(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            __Field::m_annotatedTypeHkUint32NoVar => {
                                #[cfg(
                                    any(feature = "strict", feature = "ignore_duplicates")
                                )]
                                if _serde::__private::Option::is_some(
                                    &m_annotatedTypeHkUint32NoVar,
                                ) {
                                    #[cfg(feature = "ignore_duplicates")]
                                    {
                                        __A::skip_value(&mut __map)?;
                                        continue;
                                    }
                                    #[cfg(feature = "strict")]
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "annotatedTypeHkUint32NoVar",
                                        ),
                                    );
                                }
                                m_annotatedTypeHkUint32NoVar = _serde::__private::Some(
                                    match __A::next_value::<U32<'de>>(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            __Field::m_annotatedTypeHkRealNoVar => {
                                #[cfg(
                                    any(feature = "strict", feature = "ignore_duplicates")
                                )]
                                if _serde::__private::Option::is_some(
                                    &m_annotatedTypeHkRealNoVar,
                                ) {
                                    #[cfg(feature = "ignore_duplicates")]
                                    {
                                        __A::skip_value(&mut __map)?;
                                        continue;
                                    }
                                    #[cfg(feature = "strict")]
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "annotatedTypeHkRealNoVar",
                                        ),
                                    );
                                }
                                m_annotatedTypeHkRealNoVar = _serde::__private::Some(
                                    match __A::next_value::<f32>(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            __Field::m_annotatedTypeBoolOutput => {
                                #[cfg(
                                    any(feature = "strict", feature = "ignore_duplicates")
                                )]
                                if _serde::__private::Option::is_some(
                                    &m_annotatedTypeBoolOutput,
                                ) {
                                    #[cfg(feature = "ignore_duplicates")]
                                    {
                                        __A::skip_value(&mut __map)?;
                                        continue;
                                    }
                                    #[cfg(feature = "strict")]
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "annotatedTypeBoolOutput",
                                        ),
                                    );
                                }
                                m_annotatedTypeBoolOutput = _serde::__private::Some(
                                    match __A::next_value::<bool>(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            __Field::m_annotatedTypeHkBoolOutput => {
                                #[cfg(
                                    any(feature = "strict", feature = "ignore_duplicates")
                                )]
                                if _serde::__private::Option::is_some(
                                    &m_annotatedTypeHkBoolOutput,
                                ) {
                                    #[cfg(feature = "ignore_duplicates")]
                                    {
                                        __A::skip_value(&mut __map)?;
                                        continue;
                                    }
                                    #[cfg(feature = "strict")]
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "annotatedTypeHkBoolOutput",
                                        ),
                                    );
                                }
                                m_annotatedTypeHkBoolOutput = _serde::__private::Some(
                                    match __A::next_value::<bool>(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            __Field::m_annotatedTypeHkInt8Output => {
                                #[cfg(
                                    any(feature = "strict", feature = "ignore_duplicates")
                                )]
                                if _serde::__private::Option::is_some(
                                    &m_annotatedTypeHkInt8Output,
                                ) {
                                    #[cfg(feature = "ignore_duplicates")]
                                    {
                                        __A::skip_value(&mut __map)?;
                                        continue;
                                    }
                                    #[cfg(feature = "strict")]
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "annotatedTypeHkInt8Output",
                                        ),
                                    );
                                }
                                m_annotatedTypeHkInt8Output = _serde::__private::Some(
                                    match __A::next_value::<I8<'de>>(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            __Field::m_annotatedTypeHkInt16Output => {
                                #[cfg(
                                    any(feature = "strict", feature = "ignore_duplicates")
                                )]
                                if _serde::__private::Option::is_some(
                                    &m_annotatedTypeHkInt16Output,
                                ) {
                                    #[cfg(feature = "ignore_duplicates")]
                                    {
                                        __A::skip_value(&mut __map)?;
                                        continue;
                                    }
                                    #[cfg(feature = "strict")]
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "annotatedTypeHkInt16Output",
                                        ),
                                    );
                                }
                                m_annotatedTypeHkInt16Output = _serde::__private::Some(
                                    match __A::next_value::<I16<'de>>(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            __Field::m_annotatedTypeHkInt32Output => {
                                #[cfg(
                                    any(feature = "strict", feature = "ignore_duplicates")
                                )]
                                if _serde::__private::Option::is_some(
                                    &m_annotatedTypeHkInt32Output,
                                ) {
                                    #[cfg(feature = "ignore_duplicates")]
                                    {
                                        __A::skip_value(&mut __map)?;
                                        continue;
                                    }
                                    #[cfg(feature = "strict")]
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "annotatedTypeHkInt32Output",
                                        ),
                                    );
                                }
                                m_annotatedTypeHkInt32Output = _serde::__private::Some(
                                    match __A::next_value::<I32<'de>>(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            __Field::m_annotatedTypeHkUint8Output => {
                                #[cfg(
                                    any(feature = "strict", feature = "ignore_duplicates")
                                )]
                                if _serde::__private::Option::is_some(
                                    &m_annotatedTypeHkUint8Output,
                                ) {
                                    #[cfg(feature = "ignore_duplicates")]
                                    {
                                        __A::skip_value(&mut __map)?;
                                        continue;
                                    }
                                    #[cfg(feature = "strict")]
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "annotatedTypeHkUint8Output",
                                        ),
                                    );
                                }
                                m_annotatedTypeHkUint8Output = _serde::__private::Some(
                                    match __A::next_value::<U8<'de>>(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            __Field::m_annotatedTypeHkUint16Output => {
                                #[cfg(
                                    any(feature = "strict", feature = "ignore_duplicates")
                                )]
                                if _serde::__private::Option::is_some(
                                    &m_annotatedTypeHkUint16Output,
                                ) {
                                    #[cfg(feature = "ignore_duplicates")]
                                    {
                                        __A::skip_value(&mut __map)?;
                                        continue;
                                    }
                                    #[cfg(feature = "strict")]
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "annotatedTypeHkUint16Output",
                                        ),
                                    );
                                }
                                m_annotatedTypeHkUint16Output = _serde::__private::Some(
                                    match __A::next_value::<U16<'de>>(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            __Field::m_annotatedTypeHkUint32Output => {
                                #[cfg(
                                    any(feature = "strict", feature = "ignore_duplicates")
                                )]
                                if _serde::__private::Option::is_some(
                                    &m_annotatedTypeHkUint32Output,
                                ) {
                                    #[cfg(feature = "ignore_duplicates")]
                                    {
                                        __A::skip_value(&mut __map)?;
                                        continue;
                                    }
                                    #[cfg(feature = "strict")]
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "annotatedTypeHkUint32Output",
                                        ),
                                    );
                                }
                                m_annotatedTypeHkUint32Output = _serde::__private::Some(
                                    match __A::next_value::<U32<'de>>(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            __Field::m_annotatedTypeHkRealOutput => {
                                #[cfg(
                                    any(feature = "strict", feature = "ignore_duplicates")
                                )]
                                if _serde::__private::Option::is_some(
                                    &m_annotatedTypeHkRealOutput,
                                ) {
                                    #[cfg(feature = "ignore_duplicates")]
                                    {
                                        __A::skip_value(&mut __map)?;
                                        continue;
                                    }
                                    #[cfg(feature = "strict")]
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "annotatedTypeHkRealOutput",
                                        ),
                                    );
                                }
                                m_annotatedTypeHkRealOutput = _serde::__private::Some(
                                    match __A::next_value::<f32>(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            __Field::m_annotatedHiddenTypeBool => {
                                #[cfg(
                                    any(feature = "strict", feature = "ignore_duplicates")
                                )]
                                if _serde::__private::Option::is_some(
                                    &m_annotatedHiddenTypeBool,
                                ) {
                                    #[cfg(feature = "ignore_duplicates")]
                                    {
                                        __A::skip_value(&mut __map)?;
                                        continue;
                                    }
                                    #[cfg(feature = "strict")]
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "annotatedHiddenTypeBool",
                                        ),
                                    );
                                }
                                m_annotatedHiddenTypeBool = _serde::__private::Some(
                                    match __A::next_value::<bool>(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            __Field::m_annotatedHiddenTypeHkBool => {
                                #[cfg(
                                    any(feature = "strict", feature = "ignore_duplicates")
                                )]
                                if _serde::__private::Option::is_some(
                                    &m_annotatedHiddenTypeHkBool,
                                ) {
                                    #[cfg(feature = "ignore_duplicates")]
                                    {
                                        __A::skip_value(&mut __map)?;
                                        continue;
                                    }
                                    #[cfg(feature = "strict")]
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "annotatedHiddenTypeHkBool",
                                        ),
                                    );
                                }
                                m_annotatedHiddenTypeHkBool = _serde::__private::Some(
                                    match __A::next_value::<bool>(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            __Field::m_annotatedHiddenTypeCString1 => {
                                #[cfg(
                                    any(feature = "strict", feature = "ignore_duplicates")
                                )]
                                if _serde::__private::Option::is_some(
                                    &m_annotatedHiddenTypeCString1,
                                ) {
                                    #[cfg(feature = "ignore_duplicates")]
                                    {
                                        __A::skip_value(&mut __map)?;
                                        continue;
                                    }
                                    #[cfg(feature = "strict")]
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "annotatedHiddenTypeCString1",
                                        ),
                                    );
                                }
                                m_annotatedHiddenTypeCString1 = _serde::__private::Some(
                                    match __A::next_value::<CString<'de>>(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            __Field::m_annotatedHiddenTypeHkStringPtr1 => {
                                #[cfg(
                                    any(feature = "strict", feature = "ignore_duplicates")
                                )]
                                if _serde::__private::Option::is_some(
                                    &m_annotatedHiddenTypeHkStringPtr1,
                                ) {
                                    #[cfg(feature = "ignore_duplicates")]
                                    {
                                        __A::skip_value(&mut __map)?;
                                        continue;
                                    }
                                    #[cfg(feature = "strict")]
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "annotatedHiddenTypeHkStringPtr1",
                                        ),
                                    );
                                }
                                m_annotatedHiddenTypeHkStringPtr1 = _serde::__private::Some(
                                    match __A::next_value::<StringPtr<'de>>(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            __Field::m_annotatedHiddenTypeCString2 => {
                                #[cfg(
                                    any(feature = "strict", feature = "ignore_duplicates")
                                )]
                                if _serde::__private::Option::is_some(
                                    &m_annotatedHiddenTypeCString2,
                                ) {
                                    #[cfg(feature = "ignore_duplicates")]
                                    {
                                        __A::skip_value(&mut __map)?;
                                        continue;
                                    }
                                    #[cfg(feature = "strict")]
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "annotatedHiddenTypeCString2",
                                        ),
                                    );
                                }
                                m_annotatedHiddenTypeCString2 = _serde::__private::Some(
                                    match __A::next_value::<CString<'de>>(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            __Field::m_annotatedHiddenTypeHkStringPtr2 => {
                                #[cfg(
                                    any(feature = "strict", feature = "ignore_duplicates")
                                )]
                                if _serde::__private::Option::is_some(
                                    &m_annotatedHiddenTypeHkStringPtr2,
                                ) {
                                    #[cfg(feature = "ignore_duplicates")]
                                    {
                                        __A::skip_value(&mut __map)?;
                                        continue;
                                    }
                                    #[cfg(feature = "strict")]
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "annotatedHiddenTypeHkStringPtr2",
                                        ),
                                    );
                                }
                                m_annotatedHiddenTypeHkStringPtr2 = _serde::__private::Some(
                                    match __A::next_value::<StringPtr<'de>>(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            __Field::m_annotatedHiddenTypeHkInt8 => {
                                #[cfg(
                                    any(feature = "strict", feature = "ignore_duplicates")
                                )]
                                if _serde::__private::Option::is_some(
                                    &m_annotatedHiddenTypeHkInt8,
                                ) {
                                    #[cfg(feature = "ignore_duplicates")]
                                    {
                                        __A::skip_value(&mut __map)?;
                                        continue;
                                    }
                                    #[cfg(feature = "strict")]
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "annotatedHiddenTypeHkInt8",
                                        ),
                                    );
                                }
                                m_annotatedHiddenTypeHkInt8 = _serde::__private::Some(
                                    match __A::next_value::<I8<'de>>(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            __Field::m_annotatedHiddenTypeHkInt16 => {
                                #[cfg(
                                    any(feature = "strict", feature = "ignore_duplicates")
                                )]
                                if _serde::__private::Option::is_some(
                                    &m_annotatedHiddenTypeHkInt16,
                                ) {
                                    #[cfg(feature = "ignore_duplicates")]
                                    {
                                        __A::skip_value(&mut __map)?;
                                        continue;
                                    }
                                    #[cfg(feature = "strict")]
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "annotatedHiddenTypeHkInt16",
                                        ),
                                    );
                                }
                                m_annotatedHiddenTypeHkInt16 = _serde::__private::Some(
                                    match __A::next_value::<I16<'de>>(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            __Field::m_annotatedHiddenTypeHkInt32 => {
                                #[cfg(
                                    any(feature = "strict", feature = "ignore_duplicates")
                                )]
                                if _serde::__private::Option::is_some(
                                    &m_annotatedHiddenTypeHkInt32,
                                ) {
                                    #[cfg(feature = "ignore_duplicates")]
                                    {
                                        __A::skip_value(&mut __map)?;
                                        continue;
                                    }
                                    #[cfg(feature = "strict")]
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "annotatedHiddenTypeHkInt32",
                                        ),
                                    );
                                }
                                m_annotatedHiddenTypeHkInt32 = _serde::__private::Some(
                                    match __A::next_value::<I32<'de>>(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            __Field::m_annotatedHiddenTypeHkUint8 => {
                                #[cfg(
                                    any(feature = "strict", feature = "ignore_duplicates")
                                )]
                                if _serde::__private::Option::is_some(
                                    &m_annotatedHiddenTypeHkUint8,
                                ) {
                                    #[cfg(feature = "ignore_duplicates")]
                                    {
                                        __A::skip_value(&mut __map)?;
                                        continue;
                                    }
                                    #[cfg(feature = "strict")]
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "annotatedHiddenTypeHkUint8",
                                        ),
                                    );
                                }
                                m_annotatedHiddenTypeHkUint8 = _serde::__private::Some(
                                    match __A::next_value::<U8<'de>>(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            __Field::m_annotatedHiddenTypeHkUint16 => {
                                #[cfg(
                                    any(feature = "strict", feature = "ignore_duplicates")
                                )]
                                if _serde::__private::Option::is_some(
                                    &m_annotatedHiddenTypeHkUint16,
                                ) {
                                    #[cfg(feature = "ignore_duplicates")]
                                    {
                                        __A::skip_value(&mut __map)?;
                                        continue;
                                    }
                                    #[cfg(feature = "strict")]
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "annotatedHiddenTypeHkUint16",
                                        ),
                                    );
                                }
                                m_annotatedHiddenTypeHkUint16 = _serde::__private::Some(
                                    match __A::next_value::<U16<'de>>(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            __Field::m_annotatedHiddenTypeHkUint32 => {
                                #[cfg(
                                    any(feature = "strict", feature = "ignore_duplicates")
                                )]
                                if _serde::__private::Option::is_some(
                                    &m_annotatedHiddenTypeHkUint32,
                                ) {
                                    #[cfg(feature = "ignore_duplicates")]
                                    {
                                        __A::skip_value(&mut __map)?;
                                        continue;
                                    }
                                    #[cfg(feature = "strict")]
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "annotatedHiddenTypeHkUint32",
                                        ),
                                    );
                                }
                                m_annotatedHiddenTypeHkUint32 = _serde::__private::Some(
                                    match __A::next_value::<U32<'de>>(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            __Field::m_annotatedHiddenTypeCopyEnd => {
                                #[cfg(
                                    any(feature = "strict", feature = "ignore_duplicates")
                                )]
                                if _serde::__private::Option::is_some(
                                    &m_annotatedHiddenTypeCopyEnd,
                                ) {
                                    #[cfg(feature = "ignore_duplicates")]
                                    {
                                        __A::skip_value(&mut __map)?;
                                        continue;
                                    }
                                    #[cfg(feature = "strict")]
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "annotatedHiddenTypeCopyEnd",
                                        ),
                                    );
                                }
                                m_annotatedHiddenTypeCopyEnd = _serde::__private::Some(
                                    match __A::next_value::<bool>(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            __Field::m_protectedHiddenMember => {
                                #[cfg(
                                    any(feature = "strict", feature = "ignore_duplicates")
                                )]
                                if _serde::__private::Option::is_some(
                                    &m_protectedHiddenMember,
                                ) {
                                    #[cfg(feature = "ignore_duplicates")]
                                    {
                                        __A::skip_value(&mut __map)?;
                                        continue;
                                    }
                                    #[cfg(feature = "strict")]
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "protectedHiddenMember",
                                        ),
                                    );
                                }
                                m_protectedHiddenMember = _serde::__private::Some(
                                    match __A::next_value::<bool>(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            __Field::m_privateHiddenMember => {
                                #[cfg(
                                    any(feature = "strict", feature = "ignore_duplicates")
                                )]
                                if _serde::__private::Option::is_some(
                                    &m_privateHiddenMember,
                                ) {
                                    #[cfg(feature = "ignore_duplicates")]
                                    {
                                        __A::skip_value(&mut __map)?;
                                        continue;
                                    }
                                    #[cfg(feature = "strict")]
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "privateHiddenMember",
                                        ),
                                    );
                                }
                                m_privateHiddenMember = _serde::__private::Some(
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
                    let m_inheritedHiddenMember = match m_inheritedHiddenMember {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            #[cfg(feature = "strict")]
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "inheritedHiddenMember",
                                ),
                            );
                            #[cfg(not(feature = "strict"))] Default::default()
                        }
                    };
                    let m_protectedInheritedHiddenMember = match m_protectedInheritedHiddenMember {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            #[cfg(feature = "strict")]
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "protectedInheritedHiddenMember",
                                ),
                            );
                            #[cfg(not(feature = "strict"))] Default::default()
                        }
                    };
                    let m_privateInheritedHiddenMember = match m_privateInheritedHiddenMember {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            #[cfg(feature = "strict")]
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "privateInheritedHiddenMember",
                                ),
                            );
                            #[cfg(not(feature = "strict"))] Default::default()
                        }
                    };
                    let m_simpleTypeHkInt64 = match m_simpleTypeHkInt64 {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            #[cfg(feature = "strict")]
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "simpleTypeHkInt64",
                                ),
                            );
                            #[cfg(not(feature = "strict"))] Default::default()
                        }
                    };
                    let m_simpleTypeHkUint64 = match m_simpleTypeHkUint64 {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            #[cfg(feature = "strict")]
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "simpleTypeHkUint64",
                                ),
                            );
                            #[cfg(not(feature = "strict"))] Default::default()
                        }
                    };
                    let m_simpleHiddenTypeCopyStart = match m_simpleHiddenTypeCopyStart {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            #[cfg(feature = "strict")]
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "simpleHiddenTypeCopyStart",
                                ),
                            );
                            #[cfg(not(feature = "strict"))] Default::default()
                        }
                    };
                    let m_simpleTypeBool = match m_simpleTypeBool {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            #[cfg(feature = "strict")]
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "simpleTypeBool",
                                ),
                            );
                            #[cfg(not(feature = "strict"))] Default::default()
                        }
                    };
                    let m_simpleTypeHkBool = match m_simpleTypeHkBool {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            #[cfg(feature = "strict")]
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "simpleTypeHkBool",
                                ),
                            );
                            #[cfg(not(feature = "strict"))] Default::default()
                        }
                    };
                    let m_simpleTypeCString = match m_simpleTypeCString {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            #[cfg(feature = "strict")]
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "simpleTypeCString",
                                ),
                            );
                            #[cfg(not(feature = "strict"))] Default::default()
                        }
                    };
                    let m_simpleTypeHkStringPtr = match m_simpleTypeHkStringPtr {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            #[cfg(feature = "strict")]
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "simpleTypeHkStringPtr",
                                ),
                            );
                            #[cfg(not(feature = "strict"))] Default::default()
                        }
                    };
                    let m_simpleTypeHkInt8 = match m_simpleTypeHkInt8 {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            #[cfg(feature = "strict")]
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "simpleTypeHkInt8",
                                ),
                            );
                            #[cfg(not(feature = "strict"))] Default::default()
                        }
                    };
                    let m_simpleTypeHkInt16 = match m_simpleTypeHkInt16 {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            #[cfg(feature = "strict")]
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "simpleTypeHkInt16",
                                ),
                            );
                            #[cfg(not(feature = "strict"))] Default::default()
                        }
                    };
                    let m_simpleTypeHkInt32 = match m_simpleTypeHkInt32 {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            #[cfg(feature = "strict")]
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "simpleTypeHkInt32",
                                ),
                            );
                            #[cfg(not(feature = "strict"))] Default::default()
                        }
                    };
                    let m_simpleTypeHkUint8 = match m_simpleTypeHkUint8 {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            #[cfg(feature = "strict")]
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "simpleTypeHkUint8",
                                ),
                            );
                            #[cfg(not(feature = "strict"))] Default::default()
                        }
                    };
                    let m_simpleTypeHkUint16 = match m_simpleTypeHkUint16 {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            #[cfg(feature = "strict")]
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "simpleTypeHkUint16",
                                ),
                            );
                            #[cfg(not(feature = "strict"))] Default::default()
                        }
                    };
                    let m_simpleTypeHkUint32 = match m_simpleTypeHkUint32 {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            #[cfg(feature = "strict")]
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "simpleTypeHkUint32",
                                ),
                            );
                            #[cfg(not(feature = "strict"))] Default::default()
                        }
                    };
                    let m_simpleTypeHkReal = match m_simpleTypeHkReal {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            #[cfg(feature = "strict")]
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "simpleTypeHkReal",
                                ),
                            );
                            #[cfg(not(feature = "strict"))] Default::default()
                        }
                    };
                    let m_simpleTypeHkInt8Default = match m_simpleTypeHkInt8Default {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            #[cfg(feature = "strict")]
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "simpleTypeHkInt8Default",
                                ),
                            );
                            #[cfg(not(feature = "strict"))] Default::default()
                        }
                    };
                    let m_simpleTypeHkInt16Default = match m_simpleTypeHkInt16Default {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            #[cfg(feature = "strict")]
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "simpleTypeHkInt16Default",
                                ),
                            );
                            #[cfg(not(feature = "strict"))] Default::default()
                        }
                    };
                    let m_simpleTypeHkInt32Default = match m_simpleTypeHkInt32Default {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            #[cfg(feature = "strict")]
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "simpleTypeHkInt32Default",
                                ),
                            );
                            #[cfg(not(feature = "strict"))] Default::default()
                        }
                    };
                    let m_simpleTypeHkUint8Default = match m_simpleTypeHkUint8Default {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            #[cfg(feature = "strict")]
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "simpleTypeHkUint8Default",
                                ),
                            );
                            #[cfg(not(feature = "strict"))] Default::default()
                        }
                    };
                    let m_simpleTypeHkUint16Default = match m_simpleTypeHkUint16Default {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            #[cfg(feature = "strict")]
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "simpleTypeHkUint16Default",
                                ),
                            );
                            #[cfg(not(feature = "strict"))] Default::default()
                        }
                    };
                    let m_simpleTypeHkUint32Default = match m_simpleTypeHkUint32Default {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            #[cfg(feature = "strict")]
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "simpleTypeHkUint32Default",
                                ),
                            );
                            #[cfg(not(feature = "strict"))] Default::default()
                        }
                    };
                    let m_simpleTypeHkRealDefault = match m_simpleTypeHkRealDefault {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            #[cfg(feature = "strict")]
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "simpleTypeHkRealDefault",
                                ),
                            );
                            #[cfg(not(feature = "strict"))] Default::default()
                        }
                    };
                    let m_simpleTypeHkInt8Clamp = match m_simpleTypeHkInt8Clamp {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            #[cfg(feature = "strict")]
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "simpleTypeHkInt8Clamp",
                                ),
                            );
                            #[cfg(not(feature = "strict"))] Default::default()
                        }
                    };
                    let m_simpleTypeHkInt16Clamp = match m_simpleTypeHkInt16Clamp {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            #[cfg(feature = "strict")]
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "simpleTypeHkInt16Clamp",
                                ),
                            );
                            #[cfg(not(feature = "strict"))] Default::default()
                        }
                    };
                    let m_simpleTypeHkInt32Clamp = match m_simpleTypeHkInt32Clamp {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            #[cfg(feature = "strict")]
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "simpleTypeHkInt32Clamp",
                                ),
                            );
                            #[cfg(not(feature = "strict"))] Default::default()
                        }
                    };
                    let m_simpleTypeHkUint8Clamp = match m_simpleTypeHkUint8Clamp {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            #[cfg(feature = "strict")]
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "simpleTypeHkUint8Clamp",
                                ),
                            );
                            #[cfg(not(feature = "strict"))] Default::default()
                        }
                    };
                    let m_simpleTypeHkUint16Clamp = match m_simpleTypeHkUint16Clamp {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            #[cfg(feature = "strict")]
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "simpleTypeHkUint16Clamp",
                                ),
                            );
                            #[cfg(not(feature = "strict"))] Default::default()
                        }
                    };
                    let m_simpleTypeHkUint32Clamp = match m_simpleTypeHkUint32Clamp {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            #[cfg(feature = "strict")]
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "simpleTypeHkUint32Clamp",
                                ),
                            );
                            #[cfg(not(feature = "strict"))] Default::default()
                        }
                    };
                    let m_simpleTypeHkRealClamp = match m_simpleTypeHkRealClamp {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            #[cfg(feature = "strict")]
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "simpleTypeHkRealClamp",
                                ),
                            );
                            #[cfg(not(feature = "strict"))] Default::default()
                        }
                    };
                    let m_simpleHiddenTypeCopyEnd = match m_simpleHiddenTypeCopyEnd {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            #[cfg(feature = "strict")]
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "simpleHiddenTypeCopyEnd",
                                ),
                            );
                            #[cfg(not(feature = "strict"))] Default::default()
                        }
                    };
                    let m_complexTypeHkObjectPtr = match m_complexTypeHkObjectPtr {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            #[cfg(feature = "strict")]
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "complexTypeHkObjectPtr",
                                ),
                            );
                            #[cfg(not(feature = "strict"))] Default::default()
                        }
                    };
                    let m_complexHiddenTypeCopyStart = match m_complexHiddenTypeCopyStart {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            #[cfg(feature = "strict")]
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "complexHiddenTypeCopyStart",
                                ),
                            );
                            #[cfg(not(feature = "strict"))] Default::default()
                        }
                    };
                    let m_complexTypeHkQuaternion = match m_complexTypeHkQuaternion {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            #[cfg(feature = "strict")]
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "complexTypeHkQuaternion",
                                ),
                            );
                            #[cfg(not(feature = "strict"))] Default::default()
                        }
                    };
                    let m_complexTypeHkVector4 = match m_complexTypeHkVector4 {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            #[cfg(feature = "strict")]
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "complexTypeHkVector4",
                                ),
                            );
                            #[cfg(not(feature = "strict"))] Default::default()
                        }
                    };
                    let m_complexTypeEnumHkInt8 = match m_complexTypeEnumHkInt8 {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            #[cfg(feature = "strict")]
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "complexTypeEnumHkInt8",
                                ),
                            );
                            #[cfg(not(feature = "strict"))] Default::default()
                        }
                    };
                    let m_complexTypeEnumHkInt16 = match m_complexTypeEnumHkInt16 {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            #[cfg(feature = "strict")]
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "complexTypeEnumHkInt16",
                                ),
                            );
                            #[cfg(not(feature = "strict"))] Default::default()
                        }
                    };
                    let m_complexTypeEnumHkInt32 = match m_complexTypeEnumHkInt32 {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            #[cfg(feature = "strict")]
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "complexTypeEnumHkInt32",
                                ),
                            );
                            #[cfg(not(feature = "strict"))] Default::default()
                        }
                    };
                    let m_complexTypeEnumHkUint8 = match m_complexTypeEnumHkUint8 {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            #[cfg(feature = "strict")]
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "complexTypeEnumHkUint8",
                                ),
                            );
                            #[cfg(not(feature = "strict"))] Default::default()
                        }
                    };
                    let m_complexTypeEnumHkUint16 = match m_complexTypeEnumHkUint16 {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            #[cfg(feature = "strict")]
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "complexTypeEnumHkUint16",
                                ),
                            );
                            #[cfg(not(feature = "strict"))] Default::default()
                        }
                    };
                    let m_complexTypeEnumHkUint32 = match m_complexTypeEnumHkUint32 {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            #[cfg(feature = "strict")]
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "complexTypeEnumHkUint32",
                                ),
                            );
                            #[cfg(not(feature = "strict"))] Default::default()
                        }
                    };
                    let m_complexTypeEnumHkInt8InvalidCheck = match m_complexTypeEnumHkInt8InvalidCheck {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            #[cfg(feature = "strict")]
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "complexTypeEnumHkInt8InvalidCheck",
                                ),
                            );
                            #[cfg(not(feature = "strict"))] Default::default()
                        }
                    };
                    let m_complexTypeEnumHkInt16InvalidCheck = match m_complexTypeEnumHkInt16InvalidCheck {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            #[cfg(feature = "strict")]
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "complexTypeEnumHkInt16InvalidCheck",
                                ),
                            );
                            #[cfg(not(feature = "strict"))] Default::default()
                        }
                    };
                    let m_complexTypeEnumHkInt32InvalidCheck = match m_complexTypeEnumHkInt32InvalidCheck {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            #[cfg(feature = "strict")]
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "complexTypeEnumHkInt32InvalidCheck",
                                ),
                            );
                            #[cfg(not(feature = "strict"))] Default::default()
                        }
                    };
                    let m_complexTypeEnumHkUint8InvalidCheck = match m_complexTypeEnumHkUint8InvalidCheck {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            #[cfg(feature = "strict")]
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "complexTypeEnumHkUint8InvalidCheck",
                                ),
                            );
                            #[cfg(not(feature = "strict"))] Default::default()
                        }
                    };
                    let m_complexTypeEnumHkUint16InvalidCheck = match m_complexTypeEnumHkUint16InvalidCheck {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            #[cfg(feature = "strict")]
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "complexTypeEnumHkUint16InvalidCheck",
                                ),
                            );
                            #[cfg(not(feature = "strict"))] Default::default()
                        }
                    };
                    let m_complexTypeEnumHkUint32InvalidCheck = match m_complexTypeEnumHkUint32InvalidCheck {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            #[cfg(feature = "strict")]
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "complexTypeEnumHkUint32InvalidCheck",
                                ),
                            );
                            #[cfg(not(feature = "strict"))] Default::default()
                        }
                    };
                    let m_complexTypeFlagsHkInt8 = match m_complexTypeFlagsHkInt8 {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            #[cfg(feature = "strict")]
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "complexTypeFlagsHkInt8",
                                ),
                            );
                            #[cfg(not(feature = "strict"))] Default::default()
                        }
                    };
                    let m_complexTypeFlagsHkInt16 = match m_complexTypeFlagsHkInt16 {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            #[cfg(feature = "strict")]
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "complexTypeFlagsHkInt16",
                                ),
                            );
                            #[cfg(not(feature = "strict"))] Default::default()
                        }
                    };
                    let m_complexTypeFlagsHkInt32 = match m_complexTypeFlagsHkInt32 {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            #[cfg(feature = "strict")]
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "complexTypeFlagsHkInt32",
                                ),
                            );
                            #[cfg(not(feature = "strict"))] Default::default()
                        }
                    };
                    let m_complexTypeFlagsHkUint8 = match m_complexTypeFlagsHkUint8 {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            #[cfg(feature = "strict")]
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "complexTypeFlagsHkUint8",
                                ),
                            );
                            #[cfg(not(feature = "strict"))] Default::default()
                        }
                    };
                    let m_complexTypeFlagsHkUint16 = match m_complexTypeFlagsHkUint16 {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            #[cfg(feature = "strict")]
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "complexTypeFlagsHkUint16",
                                ),
                            );
                            #[cfg(not(feature = "strict"))] Default::default()
                        }
                    };
                    let m_complexTypeFlagsHkUint32 = match m_complexTypeFlagsHkUint32 {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            #[cfg(feature = "strict")]
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "complexTypeFlagsHkUint32",
                                ),
                            );
                            #[cfg(not(feature = "strict"))] Default::default()
                        }
                    };
                    let m_complexTypeFlagsHkInt8InvalidCheck = match m_complexTypeFlagsHkInt8InvalidCheck {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            #[cfg(feature = "strict")]
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "complexTypeFlagsHkInt8InvalidCheck",
                                ),
                            );
                            #[cfg(not(feature = "strict"))] Default::default()
                        }
                    };
                    let m_complexTypeFlagsHkInt16InvalidCheck = match m_complexTypeFlagsHkInt16InvalidCheck {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            #[cfg(feature = "strict")]
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "complexTypeFlagsHkInt16InvalidCheck",
                                ),
                            );
                            #[cfg(not(feature = "strict"))] Default::default()
                        }
                    };
                    let m_complexTypeFlagsHkInt32InvalidCheck = match m_complexTypeFlagsHkInt32InvalidCheck {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            #[cfg(feature = "strict")]
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "complexTypeFlagsHkInt32InvalidCheck",
                                ),
                            );
                            #[cfg(not(feature = "strict"))] Default::default()
                        }
                    };
                    let m_complexTypeFlagsHkUint8InvalidCheck = match m_complexTypeFlagsHkUint8InvalidCheck {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            #[cfg(feature = "strict")]
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "complexTypeFlagsHkUint8InvalidCheck",
                                ),
                            );
                            #[cfg(not(feature = "strict"))] Default::default()
                        }
                    };
                    let m_complexTypeFlagsHkUint16InvalidCheck = match m_complexTypeFlagsHkUint16InvalidCheck {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            #[cfg(feature = "strict")]
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "complexTypeFlagsHkUint16InvalidCheck",
                                ),
                            );
                            #[cfg(not(feature = "strict"))] Default::default()
                        }
                    };
                    let m_complexTypeFlagsHkUint32InvalidCheck = match m_complexTypeFlagsHkUint32InvalidCheck {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            #[cfg(feature = "strict")]
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "complexTypeFlagsHkUint32InvalidCheck",
                                ),
                            );
                            #[cfg(not(feature = "strict"))] Default::default()
                        }
                    };
                    let m_complexHiddenTypeCopyEnd = match m_complexHiddenTypeCopyEnd {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            #[cfg(feature = "strict")]
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "complexHiddenTypeCopyEnd",
                                ),
                            );
                            #[cfg(not(feature = "strict"))] Default::default()
                        }
                    };
                    let m_nestedTypeHkbGeneratorPtr = match m_nestedTypeHkbGeneratorPtr {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            #[cfg(feature = "strict")]
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "nestedTypeHkbGeneratorPtr",
                                ),
                            );
                            #[cfg(not(feature = "strict"))] Default::default()
                        }
                    };
                    let m_nestedTypeHkbGeneratorRefPtr = match m_nestedTypeHkbGeneratorRefPtr {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            #[cfg(feature = "strict")]
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "nestedTypeHkbGeneratorRefPtr",
                                ),
                            );
                            #[cfg(not(feature = "strict"))] Default::default()
                        }
                    };
                    let m_nestedTypeHkbModifierPtr = match m_nestedTypeHkbModifierPtr {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            #[cfg(feature = "strict")]
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "nestedTypeHkbModifierPtr",
                                ),
                            );
                            #[cfg(not(feature = "strict"))] Default::default()
                        }
                    };
                    let m_nestedTypeHkbModifierRefPtr = match m_nestedTypeHkbModifierRefPtr {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            #[cfg(feature = "strict")]
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "nestedTypeHkbModifierRefPtr",
                                ),
                            );
                            #[cfg(not(feature = "strict"))] Default::default()
                        }
                    };
                    let m_nestedTypeHkbCustomIdSelectorPtr = match m_nestedTypeHkbCustomIdSelectorPtr {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            #[cfg(feature = "strict")]
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "nestedTypeHkbCustomIdSelectorPtr",
                                ),
                            );
                            #[cfg(not(feature = "strict"))] Default::default()
                        }
                    };
                    let m_nestedTypeHkbCustomIdSelectorRefPtr = match m_nestedTypeHkbCustomIdSelectorRefPtr {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            #[cfg(feature = "strict")]
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "nestedTypeHkbCustomIdSelectorRefPtr",
                                ),
                            );
                            #[cfg(not(feature = "strict"))] Default::default()
                        }
                    };
                    let m_nestedTypeArrayBool = match m_nestedTypeArrayBool {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            #[cfg(feature = "strict")]
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "nestedTypeArrayBool",
                                ),
                            );
                            #[cfg(not(feature = "strict"))] Default::default()
                        }
                    };
                    let m_nestedTypeArrayHkBool = match m_nestedTypeArrayHkBool {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            #[cfg(feature = "strict")]
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "nestedTypeArrayHkBool",
                                ),
                            );
                            #[cfg(not(feature = "strict"))] Default::default()
                        }
                    };
                    let m_nestedTypeArrayCString = match m_nestedTypeArrayCString {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            #[cfg(feature = "strict")]
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "nestedTypeArrayCString",
                                ),
                            );
                            #[cfg(not(feature = "strict"))] Default::default()
                        }
                    };
                    let m_nestedTypeArrayHkStringPtr = match m_nestedTypeArrayHkStringPtr {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            #[cfg(feature = "strict")]
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "nestedTypeArrayHkStringPtr",
                                ),
                            );
                            #[cfg(not(feature = "strict"))] Default::default()
                        }
                    };
                    let m_nestedTypeArrayHkInt8 = match m_nestedTypeArrayHkInt8 {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            #[cfg(feature = "strict")]
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "nestedTypeArrayHkInt8",
                                ),
                            );
                            #[cfg(not(feature = "strict"))] Default::default()
                        }
                    };
                    let m_nestedTypeArrayHkInt16 = match m_nestedTypeArrayHkInt16 {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            #[cfg(feature = "strict")]
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "nestedTypeArrayHkInt16",
                                ),
                            );
                            #[cfg(not(feature = "strict"))] Default::default()
                        }
                    };
                    let m_nestedTypeArrayHkInt32 = match m_nestedTypeArrayHkInt32 {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            #[cfg(feature = "strict")]
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "nestedTypeArrayHkInt32",
                                ),
                            );
                            #[cfg(not(feature = "strict"))] Default::default()
                        }
                    };
                    let m_nestedTypeArrayHkUint8 = match m_nestedTypeArrayHkUint8 {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            #[cfg(feature = "strict")]
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "nestedTypeArrayHkUint8",
                                ),
                            );
                            #[cfg(not(feature = "strict"))] Default::default()
                        }
                    };
                    let m_nestedTypeArrayHkUint16 = match m_nestedTypeArrayHkUint16 {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            #[cfg(feature = "strict")]
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "nestedTypeArrayHkUint16",
                                ),
                            );
                            #[cfg(not(feature = "strict"))] Default::default()
                        }
                    };
                    let m_nestedTypeArrayHkUint32 = match m_nestedTypeArrayHkUint32 {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            #[cfg(feature = "strict")]
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "nestedTypeArrayHkUint32",
                                ),
                            );
                            #[cfg(not(feature = "strict"))] Default::default()
                        }
                    };
                    let m_nestedTypeArrayHkReal = match m_nestedTypeArrayHkReal {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            #[cfg(feature = "strict")]
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "nestedTypeArrayHkReal",
                                ),
                            );
                            #[cfg(not(feature = "strict"))] Default::default()
                        }
                    };
                    let m_nestedTypeArrayHkbGeneratorPtr = match m_nestedTypeArrayHkbGeneratorPtr {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            #[cfg(feature = "strict")]
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "nestedTypeArrayHkbGeneratorPtr",
                                ),
                            );
                            #[cfg(not(feature = "strict"))] Default::default()
                        }
                    };
                    let m_nestedTypeArrayHkbGeneratorRefPtr = match m_nestedTypeArrayHkbGeneratorRefPtr {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            #[cfg(feature = "strict")]
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "nestedTypeArrayHkbGeneratorRefPtr",
                                ),
                            );
                            #[cfg(not(feature = "strict"))] Default::default()
                        }
                    };
                    let m_nestedTypeArrayHkbModifierPtr = match m_nestedTypeArrayHkbModifierPtr {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            #[cfg(feature = "strict")]
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "nestedTypeArrayHkbModifierPtr",
                                ),
                            );
                            #[cfg(not(feature = "strict"))] Default::default()
                        }
                    };
                    let m_nestedTypeArrayHkbModifierRefPtr = match m_nestedTypeArrayHkbModifierRefPtr {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            #[cfg(feature = "strict")]
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "nestedTypeArrayHkbModifierRefPtr",
                                ),
                            );
                            #[cfg(not(feature = "strict"))] Default::default()
                        }
                    };
                    let m_nestedTypeArrayHkbCustomIdSelectorPtr = match m_nestedTypeArrayHkbCustomIdSelectorPtr {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            #[cfg(feature = "strict")]
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "nestedTypeArrayHkbCustomIdSelectorPtr",
                                ),
                            );
                            #[cfg(not(feature = "strict"))] Default::default()
                        }
                    };
                    let m_nestedTypeArrayHkbCustomIdSelectorRefPtr = match m_nestedTypeArrayHkbCustomIdSelectorRefPtr {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            #[cfg(feature = "strict")]
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "nestedTypeArrayHkbCustomIdSelectorRefPtr",
                                ),
                            );
                            #[cfg(not(feature = "strict"))] Default::default()
                        }
                    };
                    let m_nestedTypeStruct = match m_nestedTypeStruct {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            #[cfg(feature = "strict")]
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "nestedTypeStruct",
                                ),
                            );
                            #[cfg(not(feature = "strict"))] Default::default()
                        }
                    };
                    let m_nestedTypeArrayStruct = match m_nestedTypeArrayStruct {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            #[cfg(feature = "strict")]
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "nestedTypeArrayStruct",
                                ),
                            );
                            #[cfg(not(feature = "strict"))] Default::default()
                        }
                    };
                    let m_boneHiddenTypeCopyStart = match m_boneHiddenTypeCopyStart {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            #[cfg(feature = "strict")]
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "boneHiddenTypeCopyStart",
                                ),
                            );
                            #[cfg(not(feature = "strict"))] Default::default()
                        }
                    };
                    let m_oldBoneIndex = match m_oldBoneIndex {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            #[cfg(feature = "strict")]
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "oldBoneIndex",
                                ),
                            );
                            #[cfg(not(feature = "strict"))] Default::default()
                        }
                    };
                    let m_oldBoneIndexNoVar = match m_oldBoneIndexNoVar {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            #[cfg(feature = "strict")]
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "oldBoneIndexNoVar",
                                ),
                            );
                            #[cfg(not(feature = "strict"))] Default::default()
                        }
                    };
                    let m_boneIndex = match m_boneIndex {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            #[cfg(feature = "strict")]
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "boneIndex",
                                ),
                            );
                            #[cfg(not(feature = "strict"))] Default::default()
                        }
                    };
                    let m_boneIndexNoVar = match m_boneIndexNoVar {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            #[cfg(feature = "strict")]
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "boneIndexNoVar",
                                ),
                            );
                            #[cfg(not(feature = "strict"))] Default::default()
                        }
                    };
                    let m_boneChainIndex0 = match m_boneChainIndex0 {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            #[cfg(feature = "strict")]
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "boneChainIndex0",
                                ),
                            );
                            #[cfg(not(feature = "strict"))] Default::default()
                        }
                    };
                    let m_boneChainIndex1 = match m_boneChainIndex1 {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            #[cfg(feature = "strict")]
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "boneChainIndex1",
                                ),
                            );
                            #[cfg(not(feature = "strict"))] Default::default()
                        }
                    };
                    let m_boneChainIndex2 = match m_boneChainIndex2 {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            #[cfg(feature = "strict")]
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "boneChainIndex2",
                                ),
                            );
                            #[cfg(not(feature = "strict"))] Default::default()
                        }
                    };
                    let m_boneContractIndex0 = match m_boneContractIndex0 {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            #[cfg(feature = "strict")]
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "boneContractIndex0",
                                ),
                            );
                            #[cfg(not(feature = "strict"))] Default::default()
                        }
                    };
                    let m_boneContractIndex1 = match m_boneContractIndex1 {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            #[cfg(feature = "strict")]
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "boneContractIndex1",
                                ),
                            );
                            #[cfg(not(feature = "strict"))] Default::default()
                        }
                    };
                    let m_boneContractIndex2 = match m_boneContractIndex2 {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            #[cfg(feature = "strict")]
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "boneContractIndex2",
                                ),
                            );
                            #[cfg(not(feature = "strict"))] Default::default()
                        }
                    };
                    let m_boneHiddenTypeCopyEnd = match m_boneHiddenTypeCopyEnd {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            #[cfg(feature = "strict")]
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "boneHiddenTypeCopyEnd",
                                ),
                            );
                            #[cfg(not(feature = "strict"))] Default::default()
                        }
                    };
                    let m_boneWeightArray = match m_boneWeightArray {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            #[cfg(feature = "strict")]
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "boneWeightArray",
                                ),
                            );
                            #[cfg(not(feature = "strict"))] Default::default()
                        }
                    };
                    let m_boneIndexArray = match m_boneIndexArray {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            #[cfg(feature = "strict")]
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "boneIndexArray",
                                ),
                            );
                            #[cfg(not(feature = "strict"))] Default::default()
                        }
                    };
                    let m_annotatedTypeCStringFilename = match m_annotatedTypeCStringFilename {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            #[cfg(feature = "strict")]
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "annotatedTypeCStringFilename",
                                ),
                            );
                            #[cfg(not(feature = "strict"))] Default::default()
                        }
                    };
                    let m_annotatedTypeHkStringPtrFilename = match m_annotatedTypeHkStringPtrFilename {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            #[cfg(feature = "strict")]
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "annotatedTypeHkStringPtrFilename",
                                ),
                            );
                            #[cfg(not(feature = "strict"))] Default::default()
                        }
                    };
                    let m_annotatedTypeCStringScript = match m_annotatedTypeCStringScript {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            #[cfg(feature = "strict")]
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "annotatedTypeCStringScript",
                                ),
                            );
                            #[cfg(not(feature = "strict"))] Default::default()
                        }
                    };
                    let m_annotatedTypeHkStringPtrScript = match m_annotatedTypeHkStringPtrScript {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            #[cfg(feature = "strict")]
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "annotatedTypeHkStringPtrScript",
                                ),
                            );
                            #[cfg(not(feature = "strict"))] Default::default()
                        }
                    };
                    let m_annotatedTypeCStringBoneAttachment = match m_annotatedTypeCStringBoneAttachment {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            #[cfg(feature = "strict")]
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "annotatedTypeCStringBoneAttachment",
                                ),
                            );
                            #[cfg(not(feature = "strict"))] Default::default()
                        }
                    };
                    let m_annotatedTypeHkStringPtrBoneAttachment = match m_annotatedTypeHkStringPtrBoneAttachment {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            #[cfg(feature = "strict")]
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "annotatedTypeHkStringPtrBoneAttachment",
                                ),
                            );
                            #[cfg(not(feature = "strict"))] Default::default()
                        }
                    };
                    let m_annotatedTypeCStringLocalFrame = match m_annotatedTypeCStringLocalFrame {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            #[cfg(feature = "strict")]
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "annotatedTypeCStringLocalFrame",
                                ),
                            );
                            #[cfg(not(feature = "strict"))] Default::default()
                        }
                    };
                    let m_annotatedTypeHkStringPtrLocalFrame = match m_annotatedTypeHkStringPtrLocalFrame {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            #[cfg(feature = "strict")]
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "annotatedTypeHkStringPtrLocalFrame",
                                ),
                            );
                            #[cfg(not(feature = "strict"))] Default::default()
                        }
                    };
                    let m_annotatedHiddenTypeCopyStart = match m_annotatedHiddenTypeCopyStart {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            #[cfg(feature = "strict")]
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "annotatedHiddenTypeCopyStart",
                                ),
                            );
                            #[cfg(not(feature = "strict"))] Default::default()
                        }
                    };
                    let m_annotatedTypeHkInt32EventID = match m_annotatedTypeHkInt32EventID {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            #[cfg(feature = "strict")]
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "annotatedTypeHkInt32EventID",
                                ),
                            );
                            #[cfg(not(feature = "strict"))] Default::default()
                        }
                    };
                    let m_annotatedTypeHkInt32VariableIndex = match m_annotatedTypeHkInt32VariableIndex {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            #[cfg(feature = "strict")]
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "annotatedTypeHkInt32VariableIndex",
                                ),
                            );
                            #[cfg(not(feature = "strict"))] Default::default()
                        }
                    };
                    let m_annotatedTypeHkInt32AttributeIndex = match m_annotatedTypeHkInt32AttributeIndex {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            #[cfg(feature = "strict")]
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "annotatedTypeHkInt32AttributeIndex",
                                ),
                            );
                            #[cfg(not(feature = "strict"))] Default::default()
                        }
                    };
                    let m_annotatedTypeHkRealTime = match m_annotatedTypeHkRealTime {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            #[cfg(feature = "strict")]
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "annotatedTypeHkRealTime",
                                ),
                            );
                            #[cfg(not(feature = "strict"))] Default::default()
                        }
                    };
                    let m_annotatedTypeBoolNoVar = match m_annotatedTypeBoolNoVar {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            #[cfg(feature = "strict")]
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "annotatedTypeBoolNoVar",
                                ),
                            );
                            #[cfg(not(feature = "strict"))] Default::default()
                        }
                    };
                    let m_annotatedTypeHkBoolNoVar = match m_annotatedTypeHkBoolNoVar {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            #[cfg(feature = "strict")]
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "annotatedTypeHkBoolNoVar",
                                ),
                            );
                            #[cfg(not(feature = "strict"))] Default::default()
                        }
                    };
                    let m_annotatedTypeHkInt8NoVar = match m_annotatedTypeHkInt8NoVar {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            #[cfg(feature = "strict")]
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "annotatedTypeHkInt8NoVar",
                                ),
                            );
                            #[cfg(not(feature = "strict"))] Default::default()
                        }
                    };
                    let m_annotatedTypeHkInt16NoVar = match m_annotatedTypeHkInt16NoVar {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            #[cfg(feature = "strict")]
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "annotatedTypeHkInt16NoVar",
                                ),
                            );
                            #[cfg(not(feature = "strict"))] Default::default()
                        }
                    };
                    let m_annotatedTypeHkInt32NoVar = match m_annotatedTypeHkInt32NoVar {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            #[cfg(feature = "strict")]
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "annotatedTypeHkInt32NoVar",
                                ),
                            );
                            #[cfg(not(feature = "strict"))] Default::default()
                        }
                    };
                    let m_annotatedTypeHkUint8NoVar = match m_annotatedTypeHkUint8NoVar {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            #[cfg(feature = "strict")]
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "annotatedTypeHkUint8NoVar",
                                ),
                            );
                            #[cfg(not(feature = "strict"))] Default::default()
                        }
                    };
                    let m_annotatedTypeHkUint16NoVar = match m_annotatedTypeHkUint16NoVar {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            #[cfg(feature = "strict")]
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "annotatedTypeHkUint16NoVar",
                                ),
                            );
                            #[cfg(not(feature = "strict"))] Default::default()
                        }
                    };
                    let m_annotatedTypeHkUint32NoVar = match m_annotatedTypeHkUint32NoVar {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            #[cfg(feature = "strict")]
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "annotatedTypeHkUint32NoVar",
                                ),
                            );
                            #[cfg(not(feature = "strict"))] Default::default()
                        }
                    };
                    let m_annotatedTypeHkRealNoVar = match m_annotatedTypeHkRealNoVar {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            #[cfg(feature = "strict")]
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "annotatedTypeHkRealNoVar",
                                ),
                            );
                            #[cfg(not(feature = "strict"))] Default::default()
                        }
                    };
                    let m_annotatedTypeBoolOutput = match m_annotatedTypeBoolOutput {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            #[cfg(feature = "strict")]
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "annotatedTypeBoolOutput",
                                ),
                            );
                            #[cfg(not(feature = "strict"))] Default::default()
                        }
                    };
                    let m_annotatedTypeHkBoolOutput = match m_annotatedTypeHkBoolOutput {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            #[cfg(feature = "strict")]
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "annotatedTypeHkBoolOutput",
                                ),
                            );
                            #[cfg(not(feature = "strict"))] Default::default()
                        }
                    };
                    let m_annotatedTypeHkInt8Output = match m_annotatedTypeHkInt8Output {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            #[cfg(feature = "strict")]
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "annotatedTypeHkInt8Output",
                                ),
                            );
                            #[cfg(not(feature = "strict"))] Default::default()
                        }
                    };
                    let m_annotatedTypeHkInt16Output = match m_annotatedTypeHkInt16Output {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            #[cfg(feature = "strict")]
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "annotatedTypeHkInt16Output",
                                ),
                            );
                            #[cfg(not(feature = "strict"))] Default::default()
                        }
                    };
                    let m_annotatedTypeHkInt32Output = match m_annotatedTypeHkInt32Output {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            #[cfg(feature = "strict")]
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "annotatedTypeHkInt32Output",
                                ),
                            );
                            #[cfg(not(feature = "strict"))] Default::default()
                        }
                    };
                    let m_annotatedTypeHkUint8Output = match m_annotatedTypeHkUint8Output {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            #[cfg(feature = "strict")]
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "annotatedTypeHkUint8Output",
                                ),
                            );
                            #[cfg(not(feature = "strict"))] Default::default()
                        }
                    };
                    let m_annotatedTypeHkUint16Output = match m_annotatedTypeHkUint16Output {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            #[cfg(feature = "strict")]
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "annotatedTypeHkUint16Output",
                                ),
                            );
                            #[cfg(not(feature = "strict"))] Default::default()
                        }
                    };
                    let m_annotatedTypeHkUint32Output = match m_annotatedTypeHkUint32Output {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            #[cfg(feature = "strict")]
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "annotatedTypeHkUint32Output",
                                ),
                            );
                            #[cfg(not(feature = "strict"))] Default::default()
                        }
                    };
                    let m_annotatedTypeHkRealOutput = match m_annotatedTypeHkRealOutput {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            #[cfg(feature = "strict")]
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "annotatedTypeHkRealOutput",
                                ),
                            );
                            #[cfg(not(feature = "strict"))] Default::default()
                        }
                    };
                    let m_annotatedHiddenTypeBool = match m_annotatedHiddenTypeBool {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            #[cfg(feature = "strict")]
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "annotatedHiddenTypeBool",
                                ),
                            );
                            #[cfg(not(feature = "strict"))] Default::default()
                        }
                    };
                    let m_annotatedHiddenTypeHkBool = match m_annotatedHiddenTypeHkBool {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            #[cfg(feature = "strict")]
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "annotatedHiddenTypeHkBool",
                                ),
                            );
                            #[cfg(not(feature = "strict"))] Default::default()
                        }
                    };
                    let m_annotatedHiddenTypeCString1 = match m_annotatedHiddenTypeCString1 {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            #[cfg(feature = "strict")]
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "annotatedHiddenTypeCString1",
                                ),
                            );
                            #[cfg(not(feature = "strict"))] Default::default()
                        }
                    };
                    let m_annotatedHiddenTypeHkStringPtr1 = match m_annotatedHiddenTypeHkStringPtr1 {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            #[cfg(feature = "strict")]
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "annotatedHiddenTypeHkStringPtr1",
                                ),
                            );
                            #[cfg(not(feature = "strict"))] Default::default()
                        }
                    };
                    let m_annotatedHiddenTypeCString2 = match m_annotatedHiddenTypeCString2 {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            #[cfg(feature = "strict")]
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "annotatedHiddenTypeCString2",
                                ),
                            );
                            #[cfg(not(feature = "strict"))] Default::default()
                        }
                    };
                    let m_annotatedHiddenTypeHkStringPtr2 = match m_annotatedHiddenTypeHkStringPtr2 {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            #[cfg(feature = "strict")]
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "annotatedHiddenTypeHkStringPtr2",
                                ),
                            );
                            #[cfg(not(feature = "strict"))] Default::default()
                        }
                    };
                    let m_annotatedHiddenTypeHkInt8 = match m_annotatedHiddenTypeHkInt8 {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            #[cfg(feature = "strict")]
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "annotatedHiddenTypeHkInt8",
                                ),
                            );
                            #[cfg(not(feature = "strict"))] Default::default()
                        }
                    };
                    let m_annotatedHiddenTypeHkInt16 = match m_annotatedHiddenTypeHkInt16 {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            #[cfg(feature = "strict")]
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "annotatedHiddenTypeHkInt16",
                                ),
                            );
                            #[cfg(not(feature = "strict"))] Default::default()
                        }
                    };
                    let m_annotatedHiddenTypeHkInt32 = match m_annotatedHiddenTypeHkInt32 {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            #[cfg(feature = "strict")]
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "annotatedHiddenTypeHkInt32",
                                ),
                            );
                            #[cfg(not(feature = "strict"))] Default::default()
                        }
                    };
                    let m_annotatedHiddenTypeHkUint8 = match m_annotatedHiddenTypeHkUint8 {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            #[cfg(feature = "strict")]
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "annotatedHiddenTypeHkUint8",
                                ),
                            );
                            #[cfg(not(feature = "strict"))] Default::default()
                        }
                    };
                    let m_annotatedHiddenTypeHkUint16 = match m_annotatedHiddenTypeHkUint16 {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            #[cfg(feature = "strict")]
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "annotatedHiddenTypeHkUint16",
                                ),
                            );
                            #[cfg(not(feature = "strict"))] Default::default()
                        }
                    };
                    let m_annotatedHiddenTypeHkUint32 = match m_annotatedHiddenTypeHkUint32 {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            #[cfg(feature = "strict")]
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "annotatedHiddenTypeHkUint32",
                                ),
                            );
                            #[cfg(not(feature = "strict"))] Default::default()
                        }
                    };
                    let m_annotatedHiddenTypeCopyEnd = match m_annotatedHiddenTypeCopyEnd {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            #[cfg(feature = "strict")]
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "annotatedHiddenTypeCopyEnd",
                                ),
                            );
                            #[cfg(not(feature = "strict"))] Default::default()
                        }
                    };
                    let m_protectedHiddenMember = match m_protectedHiddenMember {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            #[cfg(feature = "strict")]
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "protectedHiddenMember",
                                ),
                            );
                            #[cfg(not(feature = "strict"))] Default::default()
                        }
                    };
                    let m_privateHiddenMember = match m_privateHiddenMember {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            #[cfg(feature = "strict")]
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "privateHiddenMember",
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
                    let parent = hkbReferencePoseGenerator {
                        __ptr: __ptr.clone(),
                        parent,
                        ..Default::default()
                    };
                    let parent = hkbCustomTestGeneratorHiddenTypes {
                        __ptr: __ptr.clone(),
                        parent,
                        m_inheritedHiddenMember,
                        m_protectedInheritedHiddenMember,
                        m_privateInheritedHiddenMember,
                    };
                    let parent = hkbCustomTestGeneratorSimpleTypes {
                        __ptr: __ptr.clone(),
                        parent,
                        m_simpleTypeHkInt64,
                        m_simpleTypeHkUint64,
                        m_simpleHiddenTypeCopyStart,
                        m_simpleTypeBool,
                        m_simpleTypeHkBool,
                        m_simpleTypeCString,
                        m_simpleTypeHkStringPtr,
                        m_simpleTypeHkInt8,
                        m_simpleTypeHkInt16,
                        m_simpleTypeHkInt32,
                        m_simpleTypeHkUint8,
                        m_simpleTypeHkUint16,
                        m_simpleTypeHkUint32,
                        m_simpleTypeHkReal,
                        m_simpleTypeHkInt8Default,
                        m_simpleTypeHkInt16Default,
                        m_simpleTypeHkInt32Default,
                        m_simpleTypeHkUint8Default,
                        m_simpleTypeHkUint16Default,
                        m_simpleTypeHkUint32Default,
                        m_simpleTypeHkRealDefault,
                        m_simpleTypeHkInt8Clamp,
                        m_simpleTypeHkInt16Clamp,
                        m_simpleTypeHkInt32Clamp,
                        m_simpleTypeHkUint8Clamp,
                        m_simpleTypeHkUint16Clamp,
                        m_simpleTypeHkUint32Clamp,
                        m_simpleTypeHkRealClamp,
                        m_simpleHiddenTypeCopyEnd,
                    };
                    let parent = hkbCustomTestGeneratorComplexTypes {
                        __ptr: __ptr.clone(),
                        parent,
                        m_complexTypeHkObjectPtr,
                        m_complexHiddenTypeCopyStart,
                        m_complexTypeHkQuaternion,
                        m_complexTypeHkVector4,
                        m_complexTypeEnumHkInt8,
                        m_complexTypeEnumHkInt16,
                        m_complexTypeEnumHkInt32,
                        m_complexTypeEnumHkUint8,
                        m_complexTypeEnumHkUint16,
                        m_complexTypeEnumHkUint32,
                        m_complexTypeEnumHkInt8InvalidCheck,
                        m_complexTypeEnumHkInt16InvalidCheck,
                        m_complexTypeEnumHkInt32InvalidCheck,
                        m_complexTypeEnumHkUint8InvalidCheck,
                        m_complexTypeEnumHkUint16InvalidCheck,
                        m_complexTypeEnumHkUint32InvalidCheck,
                        m_complexTypeFlagsHkInt8,
                        m_complexTypeFlagsHkInt16,
                        m_complexTypeFlagsHkInt32,
                        m_complexTypeFlagsHkUint8,
                        m_complexTypeFlagsHkUint16,
                        m_complexTypeFlagsHkUint32,
                        m_complexTypeFlagsHkInt8InvalidCheck,
                        m_complexTypeFlagsHkInt16InvalidCheck,
                        m_complexTypeFlagsHkInt32InvalidCheck,
                        m_complexTypeFlagsHkUint8InvalidCheck,
                        m_complexTypeFlagsHkUint16InvalidCheck,
                        m_complexTypeFlagsHkUint32InvalidCheck,
                        m_complexHiddenTypeCopyEnd,
                    };
                    let parent = hkbCustomTestGeneratorNestedTypesBase {
                        __ptr: __ptr.clone(),
                        parent,
                        m_nestedTypeHkbGeneratorPtr,
                        m_nestedTypeHkbGeneratorRefPtr,
                        m_nestedTypeHkbModifierPtr,
                        m_nestedTypeHkbModifierRefPtr,
                        m_nestedTypeHkbCustomIdSelectorPtr,
                        m_nestedTypeHkbCustomIdSelectorRefPtr,
                        m_nestedTypeArrayBool,
                        m_nestedTypeArrayHkBool,
                        m_nestedTypeArrayCString,
                        m_nestedTypeArrayHkStringPtr,
                        m_nestedTypeArrayHkInt8,
                        m_nestedTypeArrayHkInt16,
                        m_nestedTypeArrayHkInt32,
                        m_nestedTypeArrayHkUint8,
                        m_nestedTypeArrayHkUint16,
                        m_nestedTypeArrayHkUint32,
                        m_nestedTypeArrayHkReal,
                        m_nestedTypeArrayHkbGeneratorPtr,
                        m_nestedTypeArrayHkbGeneratorRefPtr,
                        m_nestedTypeArrayHkbModifierPtr,
                        m_nestedTypeArrayHkbModifierRefPtr,
                        m_nestedTypeArrayHkbCustomIdSelectorPtr,
                        m_nestedTypeArrayHkbCustomIdSelectorRefPtr,
                    };
                    let parent = hkbCustomTestGeneratorNestedTypes {
                        __ptr: __ptr.clone(),
                        parent,
                        m_nestedTypeStruct,
                        m_nestedTypeArrayStruct,
                    };
                    let parent = hkbCustomTestGeneratorBoneTypes {
                        __ptr: __ptr.clone(),
                        parent,
                        m_boneHiddenTypeCopyStart,
                        m_oldBoneIndex,
                        m_oldBoneIndexNoVar,
                        m_boneIndex,
                        m_boneIndexNoVar,
                        m_boneChainIndex0,
                        m_boneChainIndex1,
                        m_boneChainIndex2,
                        m_boneContractIndex0,
                        m_boneContractIndex1,
                        m_boneContractIndex2,
                        m_boneHiddenTypeCopyEnd,
                        m_boneWeightArray,
                        m_boneIndexArray,
                    };
                    let parent = hkbCustomTestGeneratorAnnotatedTypes {
                        __ptr: __ptr.clone(),
                        parent,
                        m_annotatedTypeCStringFilename,
                        m_annotatedTypeHkStringPtrFilename,
                        m_annotatedTypeCStringScript,
                        m_annotatedTypeHkStringPtrScript,
                        m_annotatedTypeCStringBoneAttachment,
                        m_annotatedTypeHkStringPtrBoneAttachment,
                        m_annotatedTypeCStringLocalFrame,
                        m_annotatedTypeHkStringPtrLocalFrame,
                        m_annotatedHiddenTypeCopyStart,
                        m_annotatedTypeHkInt32EventID,
                        m_annotatedTypeHkInt32VariableIndex,
                        m_annotatedTypeHkInt32AttributeIndex,
                        m_annotatedTypeHkRealTime,
                        m_annotatedTypeBoolNoVar,
                        m_annotatedTypeHkBoolNoVar,
                        m_annotatedTypeHkInt8NoVar,
                        m_annotatedTypeHkInt16NoVar,
                        m_annotatedTypeHkInt32NoVar,
                        m_annotatedTypeHkUint8NoVar,
                        m_annotatedTypeHkUint16NoVar,
                        m_annotatedTypeHkUint32NoVar,
                        m_annotatedTypeHkRealNoVar,
                        m_annotatedTypeBoolOutput,
                        m_annotatedTypeHkBoolOutput,
                        m_annotatedTypeHkInt8Output,
                        m_annotatedTypeHkInt16Output,
                        m_annotatedTypeHkInt32Output,
                        m_annotatedTypeHkUint8Output,
                        m_annotatedTypeHkUint16Output,
                        m_annotatedTypeHkUint32Output,
                        m_annotatedTypeHkRealOutput,
                        m_annotatedHiddenTypeBool,
                        m_annotatedHiddenTypeHkBool,
                        m_annotatedHiddenTypeCString1,
                        m_annotatedHiddenTypeHkStringPtr1,
                        m_annotatedHiddenTypeCString2,
                        m_annotatedHiddenTypeHkStringPtr2,
                        m_annotatedHiddenTypeHkInt8,
                        m_annotatedHiddenTypeHkInt16,
                        m_annotatedHiddenTypeHkInt32,
                        m_annotatedHiddenTypeHkUint8,
                        m_annotatedHiddenTypeHkUint16,
                        m_annotatedHiddenTypeHkUint32,
                        m_annotatedHiddenTypeCopyEnd,
                    };
                    let __ptr = __A::class_ptr(&mut __map);
                    _serde::__private::Ok(hkbCustomTestGenerator {
                        __ptr: __ptr.clone(),
                        parent,
                        m_protectedHiddenMember,
                        m_privateHiddenMember,
                    })
                }
            }
            const FIELDS: &[&str] = &["protectedHiddenMember", "privateHiddenMember"];
            _serde::Deserializer::deserialize_struct(
                deserializer,
                "hkbCustomTestGenerator",
                FIELDS,
                __hkbCustomTestGeneratorVisitor {
                    marker: _serde::__private::PhantomData::<hkbCustomTestGenerator>,
                    lifetime: _serde::__private::PhantomData,
                },
            )
        }
    }
};
