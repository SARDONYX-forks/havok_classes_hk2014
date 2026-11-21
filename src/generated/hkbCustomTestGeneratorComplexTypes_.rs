use super::class_requires::*;
use super::*;
/// # C++ Info
/// - name: `hkbCustomTestGeneratorComplexTypes`
/// - version: `0`
/// - signature: `0x74966e2f`
/// - size: `  0`(x86)/`384`(x86_64)
/// -  vtable: `true`
#[allow(non_upper_case_globals, non_snake_case)]
#[cfg_attr(feature = "json_schema", derive(schemars::JsonSchema))]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(educe::Educe)]
#[educe(Debug, Clone, Default, PartialEq)]
pub struct hkbCustomTestGeneratorComplexTypes<'a> {
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
    pub parent: hkbCustomTestGeneratorSimpleTypes<'a>,
    /// # C++ Info
    /// - name: `complexTypeHkObjectPtr`(ctype: `struct hkReferencedObject*`)
    /// - offset: `  0`(x86)/`256`(x86_64)
    /// - type_size: `  0`(x86)/`  8`(x86_64)
    #[cfg_attr(feature = "serde_default", serde(default))]
    #[cfg_attr(feature = "serde", serde(borrow))]
    #[cfg_attr(feature = "json_schema", schemars(rename = "complexTypeHkObjectPtr"))]
    #[cfg_attr(feature = "serde", serde(rename = "complexTypeHkObjectPtr"))]
    pub m_complexTypeHkObjectPtr: Pointer<'a>,
    /// # C++ Info
    /// - name: `complexHiddenTypeCopyStart`(ctype: `hkBool`)
    /// - offset: `  0`(x86)/`264`(x86_64)
    /// - type_size: `  0`(x86)/`  1`(x86_64)
    #[cfg_attr(feature = "serde_default", serde(default))]
    #[cfg_attr(feature = "json_schema", schemars(rename = "complexHiddenTypeCopyStart"))]
    #[cfg_attr(feature = "serde", serde(rename = "complexHiddenTypeCopyStart"))]
    pub m_complexHiddenTypeCopyStart: bool,
    /// # C++ Info
    /// - name: `complexTypeHkQuaternion`(ctype: `hkQuaternion`)
    /// - offset: `  0`(x86)/`272`(x86_64)
    /// - type_size: `  0`(x86)/` 16`(x86_64)
    #[cfg_attr(feature = "serde_default", serde(default))]
    #[cfg_attr(feature = "json_schema", schemars(rename = "complexTypeHkQuaternion"))]
    #[cfg_attr(feature = "serde", serde(rename = "complexTypeHkQuaternion"))]
    pub m_complexTypeHkQuaternion: Quaternion,
    /// # C++ Info
    /// - name: `complexTypeHkVector4`(ctype: `hkVector4`)
    /// - offset: `  0`(x86)/`288`(x86_64)
    /// - type_size: `  0`(x86)/` 16`(x86_64)
    #[cfg_attr(feature = "serde_default", serde(default))]
    #[cfg_attr(feature = "json_schema", schemars(rename = "complexTypeHkVector4"))]
    #[cfg_attr(feature = "serde", serde(rename = "complexTypeHkVector4"))]
    pub m_complexTypeHkVector4: Vector4,
    /// # C++ Info
    /// - name: `complexTypeEnumHkInt8`(ctype: `enum CustomEnum`)
    /// - offset: `  0`(x86)/`304`(x86_64)
    /// - type_size: `  0`(x86)/`  1`(x86_64)
    #[cfg_attr(feature = "serde_default", serde(default))]
    #[cfg_attr(feature = "json_schema", schemars(rename = "complexTypeEnumHkInt8"))]
    #[cfg_attr(feature = "serde", serde(rename = "complexTypeEnumHkInt8"))]
    pub m_complexTypeEnumHkInt8: CustomEnum,
    /// # C++ Info
    /// - name: `complexTypeEnumHkInt16`(ctype: `enum CustomEnum`)
    /// - offset: `  0`(x86)/`306`(x86_64)
    /// - type_size: `  0`(x86)/`  2`(x86_64)
    #[cfg_attr(feature = "serde_default", serde(default))]
    #[cfg_attr(feature = "json_schema", schemars(rename = "complexTypeEnumHkInt16"))]
    #[cfg_attr(feature = "serde", serde(rename = "complexTypeEnumHkInt16"))]
    pub m_complexTypeEnumHkInt16: CustomEnum,
    /// # C++ Info
    /// - name: `complexTypeEnumHkInt32`(ctype: `enum CustomEnum`)
    /// - offset: `  0`(x86)/`308`(x86_64)
    /// - type_size: `  0`(x86)/`  4`(x86_64)
    #[cfg_attr(feature = "serde_default", serde(default))]
    #[cfg_attr(feature = "json_schema", schemars(rename = "complexTypeEnumHkInt32"))]
    #[cfg_attr(feature = "serde", serde(rename = "complexTypeEnumHkInt32"))]
    pub m_complexTypeEnumHkInt32: CustomEnum,
    /// # C++ Info
    /// - name: `complexTypeEnumHkUint8`(ctype: `enum CustomEnum`)
    /// - offset: `  0`(x86)/`312`(x86_64)
    /// - type_size: `  0`(x86)/`  1`(x86_64)
    #[cfg_attr(feature = "serde_default", serde(default))]
    #[cfg_attr(feature = "json_schema", schemars(rename = "complexTypeEnumHkUint8"))]
    #[cfg_attr(feature = "serde", serde(rename = "complexTypeEnumHkUint8"))]
    pub m_complexTypeEnumHkUint8: CustomEnum,
    /// # C++ Info
    /// - name: `complexTypeEnumHkUint16`(ctype: `enum CustomEnum`)
    /// - offset: `  0`(x86)/`314`(x86_64)
    /// - type_size: `  0`(x86)/`  2`(x86_64)
    #[cfg_attr(feature = "serde_default", serde(default))]
    #[cfg_attr(feature = "json_schema", schemars(rename = "complexTypeEnumHkUint16"))]
    #[cfg_attr(feature = "serde", serde(rename = "complexTypeEnumHkUint16"))]
    pub m_complexTypeEnumHkUint16: CustomEnum,
    /// # C++ Info
    /// - name: `complexTypeEnumHkUint32`(ctype: `enum CustomEnum`)
    /// - offset: `  0`(x86)/`316`(x86_64)
    /// - type_size: `  0`(x86)/`  4`(x86_64)
    #[cfg_attr(feature = "serde_default", serde(default))]
    #[cfg_attr(feature = "json_schema", schemars(rename = "complexTypeEnumHkUint32"))]
    #[cfg_attr(feature = "serde", serde(rename = "complexTypeEnumHkUint32"))]
    pub m_complexTypeEnumHkUint32: CustomEnum,
    /// # C++ Info
    /// - name: `complexTypeEnumHkInt8InvalidCheck`(ctype: `enum CustomEnum`)
    /// - offset: `  0`(x86)/`320`(x86_64)
    /// - type_size: `  0`(x86)/`  1`(x86_64)
    #[cfg_attr(feature = "serde_default", serde(default))]
    #[cfg_attr(
        feature = "json_schema",
        schemars(rename = "complexTypeEnumHkInt8InvalidCheck")
    )]
    #[cfg_attr(feature = "serde", serde(rename = "complexTypeEnumHkInt8InvalidCheck"))]
    pub m_complexTypeEnumHkInt8InvalidCheck: CustomEnum,
    /// # C++ Info
    /// - name: `complexTypeEnumHkInt16InvalidCheck`(ctype: `enum CustomEnum`)
    /// - offset: `  0`(x86)/`322`(x86_64)
    /// - type_size: `  0`(x86)/`  2`(x86_64)
    #[cfg_attr(feature = "serde_default", serde(default))]
    #[cfg_attr(
        feature = "json_schema",
        schemars(rename = "complexTypeEnumHkInt16InvalidCheck")
    )]
    #[cfg_attr(feature = "serde", serde(rename = "complexTypeEnumHkInt16InvalidCheck"))]
    pub m_complexTypeEnumHkInt16InvalidCheck: CustomEnum,
    /// # C++ Info
    /// - name: `complexTypeEnumHkInt32InvalidCheck`(ctype: `enum CustomEnum`)
    /// - offset: `  0`(x86)/`324`(x86_64)
    /// - type_size: `  0`(x86)/`  4`(x86_64)
    #[cfg_attr(feature = "serde_default", serde(default))]
    #[cfg_attr(
        feature = "json_schema",
        schemars(rename = "complexTypeEnumHkInt32InvalidCheck")
    )]
    #[cfg_attr(feature = "serde", serde(rename = "complexTypeEnumHkInt32InvalidCheck"))]
    pub m_complexTypeEnumHkInt32InvalidCheck: CustomEnum,
    /// # C++ Info
    /// - name: `complexTypeEnumHkUint8InvalidCheck`(ctype: `enum CustomEnum`)
    /// - offset: `  0`(x86)/`328`(x86_64)
    /// - type_size: `  0`(x86)/`  1`(x86_64)
    #[cfg_attr(feature = "serde_default", serde(default))]
    #[cfg_attr(
        feature = "json_schema",
        schemars(rename = "complexTypeEnumHkUint8InvalidCheck")
    )]
    #[cfg_attr(feature = "serde", serde(rename = "complexTypeEnumHkUint8InvalidCheck"))]
    pub m_complexTypeEnumHkUint8InvalidCheck: CustomEnum,
    /// # C++ Info
    /// - name: `complexTypeEnumHkUint16InvalidCheck`(ctype: `enum CustomEnum`)
    /// - offset: `  0`(x86)/`330`(x86_64)
    /// - type_size: `  0`(x86)/`  2`(x86_64)
    #[cfg_attr(feature = "serde_default", serde(default))]
    #[cfg_attr(
        feature = "json_schema",
        schemars(rename = "complexTypeEnumHkUint16InvalidCheck")
    )]
    #[cfg_attr(feature = "serde", serde(rename = "complexTypeEnumHkUint16InvalidCheck"))]
    pub m_complexTypeEnumHkUint16InvalidCheck: CustomEnum,
    /// # C++ Info
    /// - name: `complexTypeEnumHkUint32InvalidCheck`(ctype: `enum CustomEnum`)
    /// - offset: `  0`(x86)/`332`(x86_64)
    /// - type_size: `  0`(x86)/`  4`(x86_64)
    #[cfg_attr(feature = "serde_default", serde(default))]
    #[cfg_attr(
        feature = "json_schema",
        schemars(rename = "complexTypeEnumHkUint32InvalidCheck")
    )]
    #[cfg_attr(feature = "serde", serde(rename = "complexTypeEnumHkUint32InvalidCheck"))]
    pub m_complexTypeEnumHkUint32InvalidCheck: CustomEnum,
    /// # C++ Info
    /// - name: `complexTypeFlagsHkInt8`(ctype: `flags CustomFlag`)
    /// - offset: `  0`(x86)/`336`(x86_64)
    /// - type_size: `  0`(x86)/`  1`(x86_64)
    #[cfg_attr(feature = "serde_default", serde(default))]
    #[cfg_attr(feature = "json_schema", schemars(rename = "complexTypeFlagsHkInt8"))]
    #[cfg_attr(feature = "serde", serde(rename = "complexTypeFlagsHkInt8"))]
    pub m_complexTypeFlagsHkInt8: CustomFlag,
    /// # C++ Info
    /// - name: `complexTypeFlagsHkInt16`(ctype: `flags CustomFlag`)
    /// - offset: `  0`(x86)/`338`(x86_64)
    /// - type_size: `  0`(x86)/`  2`(x86_64)
    #[cfg_attr(feature = "serde_default", serde(default))]
    #[cfg_attr(feature = "json_schema", schemars(rename = "complexTypeFlagsHkInt16"))]
    #[cfg_attr(feature = "serde", serde(rename = "complexTypeFlagsHkInt16"))]
    pub m_complexTypeFlagsHkInt16: CustomFlag,
    /// # C++ Info
    /// - name: `complexTypeFlagsHkInt32`(ctype: `flags CustomFlag`)
    /// - offset: `  0`(x86)/`340`(x86_64)
    /// - type_size: `  0`(x86)/`  4`(x86_64)
    #[cfg_attr(feature = "serde_default", serde(default))]
    #[cfg_attr(feature = "json_schema", schemars(rename = "complexTypeFlagsHkInt32"))]
    #[cfg_attr(feature = "serde", serde(rename = "complexTypeFlagsHkInt32"))]
    pub m_complexTypeFlagsHkInt32: CustomFlag,
    /// # C++ Info
    /// - name: `complexTypeFlagsHkUint8`(ctype: `flags CustomFlag`)
    /// - offset: `  0`(x86)/`344`(x86_64)
    /// - type_size: `  0`(x86)/`  1`(x86_64)
    #[cfg_attr(feature = "serde_default", serde(default))]
    #[cfg_attr(feature = "json_schema", schemars(rename = "complexTypeFlagsHkUint8"))]
    #[cfg_attr(feature = "serde", serde(rename = "complexTypeFlagsHkUint8"))]
    pub m_complexTypeFlagsHkUint8: CustomFlag,
    /// # C++ Info
    /// - name: `complexTypeFlagsHkUint16`(ctype: `flags CustomFlag`)
    /// - offset: `  0`(x86)/`346`(x86_64)
    /// - type_size: `  0`(x86)/`  2`(x86_64)
    #[cfg_attr(feature = "serde_default", serde(default))]
    #[cfg_attr(feature = "json_schema", schemars(rename = "complexTypeFlagsHkUint16"))]
    #[cfg_attr(feature = "serde", serde(rename = "complexTypeFlagsHkUint16"))]
    pub m_complexTypeFlagsHkUint16: CustomFlag,
    /// # C++ Info
    /// - name: `complexTypeFlagsHkUint32`(ctype: `flags CustomFlag`)
    /// - offset: `  0`(x86)/`348`(x86_64)
    /// - type_size: `  0`(x86)/`  4`(x86_64)
    #[cfg_attr(feature = "serde_default", serde(default))]
    #[cfg_attr(feature = "json_schema", schemars(rename = "complexTypeFlagsHkUint32"))]
    #[cfg_attr(feature = "serde", serde(rename = "complexTypeFlagsHkUint32"))]
    pub m_complexTypeFlagsHkUint32: CustomFlag,
    /// # C++ Info
    /// - name: `complexTypeFlagsHkInt8InvalidCheck`(ctype: `flags CustomFlag`)
    /// - offset: `  0`(x86)/`352`(x86_64)
    /// - type_size: `  0`(x86)/`  1`(x86_64)
    #[cfg_attr(feature = "serde_default", serde(default))]
    #[cfg_attr(
        feature = "json_schema",
        schemars(rename = "complexTypeFlagsHkInt8InvalidCheck")
    )]
    #[cfg_attr(feature = "serde", serde(rename = "complexTypeFlagsHkInt8InvalidCheck"))]
    pub m_complexTypeFlagsHkInt8InvalidCheck: CustomFlag,
    /// # C++ Info
    /// - name: `complexTypeFlagsHkInt16InvalidCheck`(ctype: `flags CustomFlag`)
    /// - offset: `  0`(x86)/`354`(x86_64)
    /// - type_size: `  0`(x86)/`  2`(x86_64)
    #[cfg_attr(feature = "serde_default", serde(default))]
    #[cfg_attr(
        feature = "json_schema",
        schemars(rename = "complexTypeFlagsHkInt16InvalidCheck")
    )]
    #[cfg_attr(feature = "serde", serde(rename = "complexTypeFlagsHkInt16InvalidCheck"))]
    pub m_complexTypeFlagsHkInt16InvalidCheck: CustomFlag,
    /// # C++ Info
    /// - name: `complexTypeFlagsHkInt32InvalidCheck`(ctype: `flags CustomFlag`)
    /// - offset: `  0`(x86)/`356`(x86_64)
    /// - type_size: `  0`(x86)/`  4`(x86_64)
    #[cfg_attr(feature = "serde_default", serde(default))]
    #[cfg_attr(
        feature = "json_schema",
        schemars(rename = "complexTypeFlagsHkInt32InvalidCheck")
    )]
    #[cfg_attr(feature = "serde", serde(rename = "complexTypeFlagsHkInt32InvalidCheck"))]
    pub m_complexTypeFlagsHkInt32InvalidCheck: CustomFlag,
    /// # C++ Info
    /// - name: `complexTypeFlagsHkUint8InvalidCheck`(ctype: `flags CustomFlag`)
    /// - offset: `  0`(x86)/`360`(x86_64)
    /// - type_size: `  0`(x86)/`  1`(x86_64)
    #[cfg_attr(feature = "serde_default", serde(default))]
    #[cfg_attr(
        feature = "json_schema",
        schemars(rename = "complexTypeFlagsHkUint8InvalidCheck")
    )]
    #[cfg_attr(feature = "serde", serde(rename = "complexTypeFlagsHkUint8InvalidCheck"))]
    pub m_complexTypeFlagsHkUint8InvalidCheck: CustomFlag,
    /// # C++ Info
    /// - name: `complexTypeFlagsHkUint16InvalidCheck`(ctype: `flags CustomFlag`)
    /// - offset: `  0`(x86)/`362`(x86_64)
    /// - type_size: `  0`(x86)/`  2`(x86_64)
    #[cfg_attr(feature = "serde_default", serde(default))]
    #[cfg_attr(
        feature = "json_schema",
        schemars(rename = "complexTypeFlagsHkUint16InvalidCheck")
    )]
    #[cfg_attr(
        feature = "serde",
        serde(rename = "complexTypeFlagsHkUint16InvalidCheck")
    )]
    pub m_complexTypeFlagsHkUint16InvalidCheck: CustomFlag,
    /// # C++ Info
    /// - name: `complexTypeFlagsHkUint32InvalidCheck`(ctype: `flags CustomFlag`)
    /// - offset: `  0`(x86)/`364`(x86_64)
    /// - type_size: `  0`(x86)/`  4`(x86_64)
    #[cfg_attr(feature = "serde_default", serde(default))]
    #[cfg_attr(
        feature = "json_schema",
        schemars(rename = "complexTypeFlagsHkUint32InvalidCheck")
    )]
    #[cfg_attr(
        feature = "serde",
        serde(rename = "complexTypeFlagsHkUint32InvalidCheck")
    )]
    pub m_complexTypeFlagsHkUint32InvalidCheck: CustomFlag,
    /// # C++ Info
    /// - name: `complexHiddenTypeCopyEnd`(ctype: `hkBool`)
    /// - offset: `  0`(x86)/`368`(x86_64)
    /// - type_size: `  0`(x86)/`  1`(x86_64)
    #[cfg_attr(feature = "serde_default", serde(default))]
    #[cfg_attr(feature = "json_schema", schemars(rename = "complexHiddenTypeCopyEnd"))]
    #[cfg_attr(feature = "serde", serde(rename = "complexHiddenTypeCopyEnd"))]
    pub m_complexHiddenTypeCopyEnd: bool,
}
const _: () = {
    use havok_serde as _serde;
    impl<'a> _serde::HavokClass for hkbCustomTestGeneratorComplexTypes<'a> {
        #[inline]
        fn name(&self) -> &'static str {
            "hkbCustomTestGeneratorComplexTypes"
        }
        #[inline]
        fn signature(&self) -> _serde::__private::Signature {
            _serde::__private::Signature::new(0x74966e2f)
        }
        #[allow(clippy::let_and_return, clippy::vec_init_then_push)]
        fn deps_indexes(&self) -> Vec<&Pointer<'_>> {
            let mut v = Vec::new();
            v.push(&self.parent.parent.parent.parent.parent.parent.m_variableBindingSet);
            v.extend(
                self.parent.parent.parent.parent.parent.parent.m_cachedBindables.iter(),
            );
            v.push(&self.parent.parent.parent.parent.parent.m_nodeInfo);
            v.extend(self.parent.parent.parent.parent.m_partitionInfo.deps_indexes());
            v.push(&self.parent.parent.parent.parent.m_syncInfo);
            v.push(&self.parent.parent.parent.m_skeleton);
            v.push(&self.m_complexTypeHkObjectPtr);
            v
        }
    }
    impl<'a> _serde::Serialize for hkbCustomTestGeneratorComplexTypes<'a> {
        fn serialize<S>(&self, __serializer: S) -> Result<S::Ok, S::Error>
        where
            S: _serde::ser::Serializer,
        {
            let class_meta = self
                .__ptr
                .as_ref()
                .map(|name| (name, _serde::__private::Signature::new(0x74966e2f)));
            let mut serializer = __serializer
                .serialize_struct(
                    "hkbCustomTestGeneratorComplexTypes",
                    class_meta,
                    (0u64, 384u64),
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
                        .m_memSizeAndRefCount,
                )?;
            serializer.pad_field([0u8; 0usize].as_slice(), [0u8; 4usize].as_slice())?;
            serializer
                .serialize_field(
                    "variableBindingSet",
                    &self.parent.parent.parent.parent.parent.parent.m_variableBindingSet,
                )?;
            serializer
                .skip_array_field(
                    "cachedBindables",
                    &self.parent.parent.parent.parent.parent.parent.m_cachedBindables,
                    TypeSize::NonPtr,
                )?;
            serializer
                .skip_field(
                    "areBindablesCached",
                    &self.parent.parent.parent.parent.parent.parent.m_areBindablesCached,
                )?;
            serializer
                .skip_field(
                    "hasEnableChanged",
                    &self.parent.parent.parent.parent.parent.parent.m_hasEnableChanged,
                )?;
            serializer.pad_field([0u8; 0usize].as_slice(), [0u8; 6usize].as_slice())?;
            serializer
                .serialize_field(
                    "userData",
                    &self.parent.parent.parent.parent.parent.m_userData,
                )?;
            serializer
                .serialize_field(
                    "name",
                    &self.parent.parent.parent.parent.parent.m_name,
                )?;
            serializer.skip_field("id", &self.parent.parent.parent.parent.parent.m_id)?;
            serializer
                .skip_field(
                    "cloneState",
                    &self.parent.parent.parent.parent.parent.m_cloneState,
                )?;
            serializer
                .skip_field("type", &self.parent.parent.parent.parent.parent.m_type)?;
            serializer.pad_field([0u8; 0usize].as_slice(), [0u8; 4usize].as_slice())?;
            serializer
                .skip_field(
                    "nodeInfo",
                    &self.parent.parent.parent.parent.parent.m_nodeInfo,
                )?;
            serializer
                .skip_field(
                    "partitionInfo",
                    &self.parent.parent.parent.parent.m_partitionInfo,
                )?;
            serializer
                .skip_field("syncInfo", &self.parent.parent.parent.parent.m_syncInfo)?;
            serializer
                .skip_fixed_array_field(
                    "pad",
                    self.parent.parent.parent.parent.m_pad.as_slice(),
                    TypeSize::NonPtr,
                )?;
            serializer.pad_field([0u8; 0usize].as_slice(), [0u8; 4usize].as_slice())?;
            serializer.skip_field("skeleton", &self.parent.parent.parent.m_skeleton)?;
            serializer
                .serialize_field(
                    "inheritedHiddenMember",
                    &self.parent.parent.m_inheritedHiddenMember,
                )?;
            serializer
                .serialize_field(
                    "protectedInheritedHiddenMember",
                    &self.parent.parent.m_protectedInheritedHiddenMember,
                )?;
            serializer
                .serialize_field(
                    "privateInheritedHiddenMember",
                    &self.parent.parent.m_privateInheritedHiddenMember,
                )?;
            serializer.pad_field([0u8; 0usize].as_slice(), [0u8; 5usize].as_slice())?;
            serializer
                .serialize_field("simpleTypeHkInt64", &self.parent.m_simpleTypeHkInt64)?;
            serializer
                .serialize_field(
                    "simpleTypeHkUint64",
                    &self.parent.m_simpleTypeHkUint64,
                )?;
            serializer
                .serialize_field(
                    "simpleHiddenTypeCopyStart",
                    &self.parent.m_simpleHiddenTypeCopyStart,
                )?;
            serializer.serialize_field("simpleTypeBool", &self.parent.m_simpleTypeBool)?;
            serializer
                .serialize_field("simpleTypeHkBool", &self.parent.m_simpleTypeHkBool)?;
            serializer.pad_field([0u8; 0usize].as_slice(), [0u8; 5usize].as_slice())?;
            serializer
                .serialize_field("simpleTypeCString", &self.parent.m_simpleTypeCString)?;
            serializer
                .serialize_field(
                    "simpleTypeHkStringPtr",
                    &self.parent.m_simpleTypeHkStringPtr,
                )?;
            serializer
                .serialize_field("simpleTypeHkInt8", &self.parent.m_simpleTypeHkInt8)?;
            serializer.pad_field([0u8; 0usize].as_slice(), [0u8; 1usize].as_slice())?;
            serializer
                .serialize_field("simpleTypeHkInt16", &self.parent.m_simpleTypeHkInt16)?;
            serializer
                .serialize_field("simpleTypeHkInt32", &self.parent.m_simpleTypeHkInt32)?;
            serializer
                .serialize_field("simpleTypeHkUint8", &self.parent.m_simpleTypeHkUint8)?;
            serializer.pad_field([0u8; 0usize].as_slice(), [0u8; 1usize].as_slice())?;
            serializer
                .serialize_field(
                    "simpleTypeHkUint16",
                    &self.parent.m_simpleTypeHkUint16,
                )?;
            serializer
                .serialize_field(
                    "simpleTypeHkUint32",
                    &self.parent.m_simpleTypeHkUint32,
                )?;
            serializer
                .serialize_field("simpleTypeHkReal", &self.parent.m_simpleTypeHkReal)?;
            serializer
                .serialize_field(
                    "simpleTypeHkInt8Default",
                    &self.parent.m_simpleTypeHkInt8Default,
                )?;
            serializer.pad_field([0u8; 0usize].as_slice(), [0u8; 1usize].as_slice())?;
            serializer
                .serialize_field(
                    "simpleTypeHkInt16Default",
                    &self.parent.m_simpleTypeHkInt16Default,
                )?;
            serializer
                .serialize_field(
                    "simpleTypeHkInt32Default",
                    &self.parent.m_simpleTypeHkInt32Default,
                )?;
            serializer
                .serialize_field(
                    "simpleTypeHkUint8Default",
                    &self.parent.m_simpleTypeHkUint8Default,
                )?;
            serializer.pad_field([0u8; 0usize].as_slice(), [0u8; 1usize].as_slice())?;
            serializer
                .serialize_field(
                    "simpleTypeHkUint16Default",
                    &self.parent.m_simpleTypeHkUint16Default,
                )?;
            serializer
                .serialize_field(
                    "simpleTypeHkUint32Default",
                    &self.parent.m_simpleTypeHkUint32Default,
                )?;
            serializer
                .serialize_field(
                    "simpleTypeHkRealDefault",
                    &self.parent.m_simpleTypeHkRealDefault,
                )?;
            serializer
                .serialize_field(
                    "simpleTypeHkInt8Clamp",
                    &self.parent.m_simpleTypeHkInt8Clamp,
                )?;
            serializer.pad_field([0u8; 0usize].as_slice(), [0u8; 1usize].as_slice())?;
            serializer
                .serialize_field(
                    "simpleTypeHkInt16Clamp",
                    &self.parent.m_simpleTypeHkInt16Clamp,
                )?;
            serializer
                .serialize_field(
                    "simpleTypeHkInt32Clamp",
                    &self.parent.m_simpleTypeHkInt32Clamp,
                )?;
            serializer
                .serialize_field(
                    "simpleTypeHkUint8Clamp",
                    &self.parent.m_simpleTypeHkUint8Clamp,
                )?;
            serializer.pad_field([0u8; 0usize].as_slice(), [0u8; 1usize].as_slice())?;
            serializer
                .serialize_field(
                    "simpleTypeHkUint16Clamp",
                    &self.parent.m_simpleTypeHkUint16Clamp,
                )?;
            serializer
                .serialize_field(
                    "simpleTypeHkUint32Clamp",
                    &self.parent.m_simpleTypeHkUint32Clamp,
                )?;
            serializer
                .serialize_field(
                    "simpleTypeHkRealClamp",
                    &self.parent.m_simpleTypeHkRealClamp,
                )?;
            serializer
                .serialize_field(
                    "simpleHiddenTypeCopyEnd",
                    &self.parent.m_simpleHiddenTypeCopyEnd,
                )?;
            serializer.pad_field([0u8; 0usize].as_slice(), [0u8; 3usize].as_slice())?;
            serializer
                .serialize_field(
                    "complexTypeHkObjectPtr",
                    &self.m_complexTypeHkObjectPtr,
                )?;
            serializer
                .serialize_field(
                    "complexHiddenTypeCopyStart",
                    &self.m_complexHiddenTypeCopyStart,
                )?;
            serializer.pad_field([0u8; 0usize].as_slice(), [0u8; 7usize].as_slice())?;
            serializer
                .serialize_field(
                    "complexTypeHkQuaternion",
                    &self.m_complexTypeHkQuaternion,
                )?;
            serializer
                .serialize_field("complexTypeHkVector4", &self.m_complexTypeHkVector4)?;
            serializer
                .serialize_field(
                    "complexTypeEnumHkInt8",
                    &self.m_complexTypeEnumHkInt8,
                )?;
            serializer.pad_field([0u8; 0usize].as_slice(), [0u8; 1usize].as_slice())?;
            serializer
                .serialize_field(
                    "complexTypeEnumHkInt16",
                    &self.m_complexTypeEnumHkInt16,
                )?;
            serializer
                .serialize_field(
                    "complexTypeEnumHkInt32",
                    &self.m_complexTypeEnumHkInt32,
                )?;
            serializer
                .serialize_field(
                    "complexTypeEnumHkUint8",
                    &self.m_complexTypeEnumHkUint8,
                )?;
            serializer.pad_field([0u8; 0usize].as_slice(), [0u8; 1usize].as_slice())?;
            serializer
                .serialize_field(
                    "complexTypeEnumHkUint16",
                    &self.m_complexTypeEnumHkUint16,
                )?;
            serializer
                .serialize_field(
                    "complexTypeEnumHkUint32",
                    &self.m_complexTypeEnumHkUint32,
                )?;
            serializer
                .serialize_field(
                    "complexTypeEnumHkInt8InvalidCheck",
                    &self.m_complexTypeEnumHkInt8InvalidCheck,
                )?;
            serializer.pad_field([0u8; 0usize].as_slice(), [0u8; 1usize].as_slice())?;
            serializer
                .serialize_field(
                    "complexTypeEnumHkInt16InvalidCheck",
                    &self.m_complexTypeEnumHkInt16InvalidCheck,
                )?;
            serializer
                .serialize_field(
                    "complexTypeEnumHkInt32InvalidCheck",
                    &self.m_complexTypeEnumHkInt32InvalidCheck,
                )?;
            serializer
                .serialize_field(
                    "complexTypeEnumHkUint8InvalidCheck",
                    &self.m_complexTypeEnumHkUint8InvalidCheck,
                )?;
            serializer.pad_field([0u8; 0usize].as_slice(), [0u8; 1usize].as_slice())?;
            serializer
                .serialize_field(
                    "complexTypeEnumHkUint16InvalidCheck",
                    &self.m_complexTypeEnumHkUint16InvalidCheck,
                )?;
            serializer
                .serialize_field(
                    "complexTypeEnumHkUint32InvalidCheck",
                    &self.m_complexTypeEnumHkUint32InvalidCheck,
                )?;
            serializer
                .serialize_field(
                    "complexTypeFlagsHkInt8",
                    &self.m_complexTypeFlagsHkInt8,
                )?;
            serializer.pad_field([0u8; 0usize].as_slice(), [0u8; 1usize].as_slice())?;
            serializer
                .serialize_field(
                    "complexTypeFlagsHkInt16",
                    &self.m_complexTypeFlagsHkInt16,
                )?;
            serializer
                .serialize_field(
                    "complexTypeFlagsHkInt32",
                    &self.m_complexTypeFlagsHkInt32,
                )?;
            serializer
                .serialize_field(
                    "complexTypeFlagsHkUint8",
                    &self.m_complexTypeFlagsHkUint8,
                )?;
            serializer.pad_field([0u8; 0usize].as_slice(), [0u8; 1usize].as_slice())?;
            serializer
                .serialize_field(
                    "complexTypeFlagsHkUint16",
                    &self.m_complexTypeFlagsHkUint16,
                )?;
            serializer
                .serialize_field(
                    "complexTypeFlagsHkUint32",
                    &self.m_complexTypeFlagsHkUint32,
                )?;
            serializer
                .serialize_field(
                    "complexTypeFlagsHkInt8InvalidCheck",
                    &self.m_complexTypeFlagsHkInt8InvalidCheck,
                )?;
            serializer.pad_field([0u8; 0usize].as_slice(), [0u8; 1usize].as_slice())?;
            serializer
                .serialize_field(
                    "complexTypeFlagsHkInt16InvalidCheck",
                    &self.m_complexTypeFlagsHkInt16InvalidCheck,
                )?;
            serializer
                .serialize_field(
                    "complexTypeFlagsHkInt32InvalidCheck",
                    &self.m_complexTypeFlagsHkInt32InvalidCheck,
                )?;
            serializer
                .serialize_field(
                    "complexTypeFlagsHkUint8InvalidCheck",
                    &self.m_complexTypeFlagsHkUint8InvalidCheck,
                )?;
            serializer.pad_field([0u8; 0usize].as_slice(), [0u8; 1usize].as_slice())?;
            serializer
                .serialize_field(
                    "complexTypeFlagsHkUint16InvalidCheck",
                    &self.m_complexTypeFlagsHkUint16InvalidCheck,
                )?;
            serializer
                .serialize_field(
                    "complexTypeFlagsHkUint32InvalidCheck",
                    &self.m_complexTypeFlagsHkUint32InvalidCheck,
                )?;
            serializer
                .serialize_field(
                    "complexHiddenTypeCopyEnd",
                    &self.m_complexHiddenTypeCopyEnd,
                )?;
            serializer.pad_field([0u8; 0usize].as_slice(), [0u8; 15usize].as_slice())?;
            serializer.end()
        }
    }
};
#[doc(hidden)]
#[allow(non_upper_case_globals, unused_attributes, unused_qualifications)]
const _: () = {
    use havok_serde as _serde;
    #[automatically_derived]
    impl<'de> _serde::Deserialize<'de> for hkbCustomTestGeneratorComplexTypes<'de> {
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
            struct __hkbCustomTestGeneratorComplexTypesVisitor<'de> {
                marker: _serde::__private::PhantomData<
                    hkbCustomTestGeneratorComplexTypes<'de>,
                >,
                lifetime: _serde::__private::PhantomData<&'de ()>,
            }
            #[allow(clippy::match_single_binding)]
            #[allow(clippy::reversed_empty_ranges)]
            #[allow(clippy::single_match)]
            impl<'de> _serde::de::Visitor<'de>
            for __hkbCustomTestGeneratorComplexTypesVisitor<'de> {
                type Value = hkbCustomTestGeneratorComplexTypes<'de>;
                fn expecting(
                    &self,
                    __formatter: &mut core::fmt::Formatter,
                ) -> core::fmt::Result {
                    core::fmt::Formatter::write_str(
                        __formatter,
                        "struct hkbCustomTestGeneratorComplexTypes",
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
                    for i in 0..29usize {
                        match i {
                            0usize => {
                                if _serde::__private::Option::is_some(
                                    &m_complexTypeHkObjectPtr,
                                ) {
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
                            1usize => {
                                if _serde::__private::Option::is_some(
                                    &m_complexHiddenTypeCopyStart,
                                ) {
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
                            2usize => {
                                if _serde::__private::Option::is_some(
                                    &m_complexTypeHkQuaternion,
                                ) {
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "complexTypeHkQuaternion",
                                        ),
                                    );
                                }
                                __A::pad(&mut __map, 0usize, 7usize)?;
                                m_complexTypeHkQuaternion = _serde::__private::Some(
                                    match __A::next_value::<Quaternion>(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            3usize => {
                                if _serde::__private::Option::is_some(
                                    &m_complexTypeHkVector4,
                                ) {
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
                            4usize => {
                                if _serde::__private::Option::is_some(
                                    &m_complexTypeEnumHkInt8,
                                ) {
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
                            5usize => {
                                if _serde::__private::Option::is_some(
                                    &m_complexTypeEnumHkInt16,
                                ) {
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "complexTypeEnumHkInt16",
                                        ),
                                    );
                                }
                                __A::pad(&mut __map, 0usize, 1usize)?;
                                m_complexTypeEnumHkInt16 = _serde::__private::Some(
                                    match __A::next_value::<CustomEnum>(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            6usize => {
                                if _serde::__private::Option::is_some(
                                    &m_complexTypeEnumHkInt32,
                                ) {
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
                            7usize => {
                                if _serde::__private::Option::is_some(
                                    &m_complexTypeEnumHkUint8,
                                ) {
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
                            8usize => {
                                if _serde::__private::Option::is_some(
                                    &m_complexTypeEnumHkUint16,
                                ) {
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "complexTypeEnumHkUint16",
                                        ),
                                    );
                                }
                                __A::pad(&mut __map, 0usize, 1usize)?;
                                m_complexTypeEnumHkUint16 = _serde::__private::Some(
                                    match __A::next_value::<CustomEnum>(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            9usize => {
                                if _serde::__private::Option::is_some(
                                    &m_complexTypeEnumHkUint32,
                                ) {
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
                            10usize => {
                                if _serde::__private::Option::is_some(
                                    &m_complexTypeEnumHkInt8InvalidCheck,
                                ) {
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
                            11usize => {
                                if _serde::__private::Option::is_some(
                                    &m_complexTypeEnumHkInt16InvalidCheck,
                                ) {
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "complexTypeEnumHkInt16InvalidCheck",
                                        ),
                                    );
                                }
                                __A::pad(&mut __map, 0usize, 1usize)?;
                                m_complexTypeEnumHkInt16InvalidCheck = _serde::__private::Some(
                                    match __A::next_value::<CustomEnum>(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            12usize => {
                                if _serde::__private::Option::is_some(
                                    &m_complexTypeEnumHkInt32InvalidCheck,
                                ) {
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
                            13usize => {
                                if _serde::__private::Option::is_some(
                                    &m_complexTypeEnumHkUint8InvalidCheck,
                                ) {
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
                            14usize => {
                                if _serde::__private::Option::is_some(
                                    &m_complexTypeEnumHkUint16InvalidCheck,
                                ) {
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "complexTypeEnumHkUint16InvalidCheck",
                                        ),
                                    );
                                }
                                __A::pad(&mut __map, 0usize, 1usize)?;
                                m_complexTypeEnumHkUint16InvalidCheck = _serde::__private::Some(
                                    match __A::next_value::<CustomEnum>(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            15usize => {
                                if _serde::__private::Option::is_some(
                                    &m_complexTypeEnumHkUint32InvalidCheck,
                                ) {
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
                            16usize => {
                                if _serde::__private::Option::is_some(
                                    &m_complexTypeFlagsHkInt8,
                                ) {
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
                            17usize => {
                                if _serde::__private::Option::is_some(
                                    &m_complexTypeFlagsHkInt16,
                                ) {
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "complexTypeFlagsHkInt16",
                                        ),
                                    );
                                }
                                __A::pad(&mut __map, 0usize, 1usize)?;
                                m_complexTypeFlagsHkInt16 = _serde::__private::Some(
                                    match __A::next_value::<CustomFlag>(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            18usize => {
                                if _serde::__private::Option::is_some(
                                    &m_complexTypeFlagsHkInt32,
                                ) {
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
                            19usize => {
                                if _serde::__private::Option::is_some(
                                    &m_complexTypeFlagsHkUint8,
                                ) {
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
                            20usize => {
                                if _serde::__private::Option::is_some(
                                    &m_complexTypeFlagsHkUint16,
                                ) {
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "complexTypeFlagsHkUint16",
                                        ),
                                    );
                                }
                                __A::pad(&mut __map, 0usize, 1usize)?;
                                m_complexTypeFlagsHkUint16 = _serde::__private::Some(
                                    match __A::next_value::<CustomFlag>(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            21usize => {
                                if _serde::__private::Option::is_some(
                                    &m_complexTypeFlagsHkUint32,
                                ) {
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
                            22usize => {
                                if _serde::__private::Option::is_some(
                                    &m_complexTypeFlagsHkInt8InvalidCheck,
                                ) {
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
                            23usize => {
                                if _serde::__private::Option::is_some(
                                    &m_complexTypeFlagsHkInt16InvalidCheck,
                                ) {
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "complexTypeFlagsHkInt16InvalidCheck",
                                        ),
                                    );
                                }
                                __A::pad(&mut __map, 0usize, 1usize)?;
                                m_complexTypeFlagsHkInt16InvalidCheck = _serde::__private::Some(
                                    match __A::next_value::<CustomFlag>(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            24usize => {
                                if _serde::__private::Option::is_some(
                                    &m_complexTypeFlagsHkInt32InvalidCheck,
                                ) {
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
                            25usize => {
                                if _serde::__private::Option::is_some(
                                    &m_complexTypeFlagsHkUint8InvalidCheck,
                                ) {
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
                            26usize => {
                                if _serde::__private::Option::is_some(
                                    &m_complexTypeFlagsHkUint16InvalidCheck,
                                ) {
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "complexTypeFlagsHkUint16InvalidCheck",
                                        ),
                                    );
                                }
                                __A::pad(&mut __map, 0usize, 1usize)?;
                                m_complexTypeFlagsHkUint16InvalidCheck = _serde::__private::Some(
                                    match __A::next_value::<CustomFlag>(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            27usize => {
                                if _serde::__private::Option::is_some(
                                    &m_complexTypeFlagsHkUint32InvalidCheck,
                                ) {
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
                            28usize => {
                                if _serde::__private::Option::is_some(
                                    &m_complexHiddenTypeCopyEnd,
                                ) {
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
                            _ => {}
                        }
                    }
                    __A::pad(&mut __map, 0usize, 15usize)?;
                    let m_complexTypeHkObjectPtr = match m_complexTypeHkObjectPtr {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "complexTypeHkObjectPtr",
                                ),
                            );
                        }
                    };
                    let m_complexHiddenTypeCopyStart = match m_complexHiddenTypeCopyStart {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "complexHiddenTypeCopyStart",
                                ),
                            );
                        }
                    };
                    let m_complexTypeHkQuaternion = match m_complexTypeHkQuaternion {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "complexTypeHkQuaternion",
                                ),
                            );
                        }
                    };
                    let m_complexTypeHkVector4 = match m_complexTypeHkVector4 {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "complexTypeHkVector4",
                                ),
                            );
                        }
                    };
                    let m_complexTypeEnumHkInt8 = match m_complexTypeEnumHkInt8 {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "complexTypeEnumHkInt8",
                                ),
                            );
                        }
                    };
                    let m_complexTypeEnumHkInt16 = match m_complexTypeEnumHkInt16 {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "complexTypeEnumHkInt16",
                                ),
                            );
                        }
                    };
                    let m_complexTypeEnumHkInt32 = match m_complexTypeEnumHkInt32 {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "complexTypeEnumHkInt32",
                                ),
                            );
                        }
                    };
                    let m_complexTypeEnumHkUint8 = match m_complexTypeEnumHkUint8 {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "complexTypeEnumHkUint8",
                                ),
                            );
                        }
                    };
                    let m_complexTypeEnumHkUint16 = match m_complexTypeEnumHkUint16 {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "complexTypeEnumHkUint16",
                                ),
                            );
                        }
                    };
                    let m_complexTypeEnumHkUint32 = match m_complexTypeEnumHkUint32 {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "complexTypeEnumHkUint32",
                                ),
                            );
                        }
                    };
                    let m_complexTypeEnumHkInt8InvalidCheck = match m_complexTypeEnumHkInt8InvalidCheck {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "complexTypeEnumHkInt8InvalidCheck",
                                ),
                            );
                        }
                    };
                    let m_complexTypeEnumHkInt16InvalidCheck = match m_complexTypeEnumHkInt16InvalidCheck {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "complexTypeEnumHkInt16InvalidCheck",
                                ),
                            );
                        }
                    };
                    let m_complexTypeEnumHkInt32InvalidCheck = match m_complexTypeEnumHkInt32InvalidCheck {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "complexTypeEnumHkInt32InvalidCheck",
                                ),
                            );
                        }
                    };
                    let m_complexTypeEnumHkUint8InvalidCheck = match m_complexTypeEnumHkUint8InvalidCheck {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "complexTypeEnumHkUint8InvalidCheck",
                                ),
                            );
                        }
                    };
                    let m_complexTypeEnumHkUint16InvalidCheck = match m_complexTypeEnumHkUint16InvalidCheck {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "complexTypeEnumHkUint16InvalidCheck",
                                ),
                            );
                        }
                    };
                    let m_complexTypeEnumHkUint32InvalidCheck = match m_complexTypeEnumHkUint32InvalidCheck {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "complexTypeEnumHkUint32InvalidCheck",
                                ),
                            );
                        }
                    };
                    let m_complexTypeFlagsHkInt8 = match m_complexTypeFlagsHkInt8 {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "complexTypeFlagsHkInt8",
                                ),
                            );
                        }
                    };
                    let m_complexTypeFlagsHkInt16 = match m_complexTypeFlagsHkInt16 {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "complexTypeFlagsHkInt16",
                                ),
                            );
                        }
                    };
                    let m_complexTypeFlagsHkInt32 = match m_complexTypeFlagsHkInt32 {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "complexTypeFlagsHkInt32",
                                ),
                            );
                        }
                    };
                    let m_complexTypeFlagsHkUint8 = match m_complexTypeFlagsHkUint8 {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "complexTypeFlagsHkUint8",
                                ),
                            );
                        }
                    };
                    let m_complexTypeFlagsHkUint16 = match m_complexTypeFlagsHkUint16 {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "complexTypeFlagsHkUint16",
                                ),
                            );
                        }
                    };
                    let m_complexTypeFlagsHkUint32 = match m_complexTypeFlagsHkUint32 {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "complexTypeFlagsHkUint32",
                                ),
                            );
                        }
                    };
                    let m_complexTypeFlagsHkInt8InvalidCheck = match m_complexTypeFlagsHkInt8InvalidCheck {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "complexTypeFlagsHkInt8InvalidCheck",
                                ),
                            );
                        }
                    };
                    let m_complexTypeFlagsHkInt16InvalidCheck = match m_complexTypeFlagsHkInt16InvalidCheck {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "complexTypeFlagsHkInt16InvalidCheck",
                                ),
                            );
                        }
                    };
                    let m_complexTypeFlagsHkInt32InvalidCheck = match m_complexTypeFlagsHkInt32InvalidCheck {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "complexTypeFlagsHkInt32InvalidCheck",
                                ),
                            );
                        }
                    };
                    let m_complexTypeFlagsHkUint8InvalidCheck = match m_complexTypeFlagsHkUint8InvalidCheck {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "complexTypeFlagsHkUint8InvalidCheck",
                                ),
                            );
                        }
                    };
                    let m_complexTypeFlagsHkUint16InvalidCheck = match m_complexTypeFlagsHkUint16InvalidCheck {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "complexTypeFlagsHkUint16InvalidCheck",
                                ),
                            );
                        }
                    };
                    let m_complexTypeFlagsHkUint32InvalidCheck = match m_complexTypeFlagsHkUint32InvalidCheck {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "complexTypeFlagsHkUint32InvalidCheck",
                                ),
                            );
                        }
                    };
                    let m_complexHiddenTypeCopyEnd = match m_complexHiddenTypeCopyEnd {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "complexHiddenTypeCopyEnd",
                                ),
                            );
                        }
                    };
                    _serde::__private::Ok(hkbCustomTestGeneratorComplexTypes {
                        __ptr,
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
                    let __ptr = __A::class_ptr(&mut __map);
                    _serde::__private::Ok(hkbCustomTestGeneratorComplexTypes {
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
                    })
                }
            }
            const FIELDS: &[&str] = &[
                "complexTypeHkObjectPtr",
                "complexHiddenTypeCopyStart",
                "complexTypeHkQuaternion",
                "complexTypeHkVector4",
                "complexTypeEnumHkInt8",
                "complexTypeEnumHkInt16",
                "complexTypeEnumHkInt32",
                "complexTypeEnumHkUint8",
                "complexTypeEnumHkUint16",
                "complexTypeEnumHkUint32",
                "complexTypeEnumHkInt8InvalidCheck",
                "complexTypeEnumHkInt16InvalidCheck",
                "complexTypeEnumHkInt32InvalidCheck",
                "complexTypeEnumHkUint8InvalidCheck",
                "complexTypeEnumHkUint16InvalidCheck",
                "complexTypeEnumHkUint32InvalidCheck",
                "complexTypeFlagsHkInt8",
                "complexTypeFlagsHkInt16",
                "complexTypeFlagsHkInt32",
                "complexTypeFlagsHkUint8",
                "complexTypeFlagsHkUint16",
                "complexTypeFlagsHkUint32",
                "complexTypeFlagsHkInt8InvalidCheck",
                "complexTypeFlagsHkInt16InvalidCheck",
                "complexTypeFlagsHkInt32InvalidCheck",
                "complexTypeFlagsHkUint8InvalidCheck",
                "complexTypeFlagsHkUint16InvalidCheck",
                "complexTypeFlagsHkUint32InvalidCheck",
                "complexHiddenTypeCopyEnd",
            ];
            _serde::Deserializer::deserialize_struct(
                deserializer,
                "hkbCustomTestGeneratorComplexTypes",
                FIELDS,
                __hkbCustomTestGeneratorComplexTypesVisitor {
                    marker: _serde::__private::PhantomData::<
                        hkbCustomTestGeneratorComplexTypes,
                    >,
                    lifetime: _serde::__private::PhantomData,
                },
            )
        }
    }
};
/// # C++ Info
/// - name: `CustomEnum`(ctype: `hkEnum<CustomEnum, hkInt8>`)
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
pub enum CustomEnum {
    #[default]
    CUSTOM_ENUM_ALA = 0isize,
    CUSTOM_ENUM_DEPECHE = 1isize,
    CUSTOM_ENUM_FURIOUS = 5isize,
}
#[havok_types_derive::impl_flags_methods]
bitflags::bitflags! {
    #[doc = r" Bit flags"] #[doc = r""] #[doc = r" # C++ Info"] #[doc =
    " - name: `CustomFlag`(ctype: `hkFlags<CustomFlag, hkInt8>`)"]
    #[allow(non_upper_case_globals, non_snake_case)] #[cfg_attr(feature = "serde",
    derive(serde_with::SerializeDisplay, serde_with::DeserializeFromStr))]
    #[repr(transparent)] #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)] pub struct
    CustomFlag : i8 { #[doc = "0"] const CUSTOM_FLAG_NONE = 0i8; #[doc = "1"] const
    CUSTOM_FLAG_UNO = 1i8; #[doc = "2"] const CUSTOM_FLAG_ZWEI = 2i8; #[doc = "4"] const
    CUSTOM_FLAG_SHI_OR_YON = 4i8; #[doc = "240"] const CUSTOM_FLAG_LOTS_O_BITS = - 16i8;
    }
}
#[cfg(feature = "json_schema")]
const _: () = {
    use schemars::{SchemaGenerator, Schema, JsonSchema, json_schema};
    use std::borrow::Cow;
    impl JsonSchema for CustomFlag {
        fn schema_name() -> Cow<'static, str> {
            "CustomFlag".into()
        }
        fn schema_id() -> Cow<'static, str> {
            concat!(module_path!(), "::", "CustomFlag").into()
        }
        fn json_schema(_generate: &mut SchemaGenerator) -> Schema {
            json_schema!(
                { "description" :
                "Bitflags field. Specific flags: CUSTOM_FLAG_NONE: 0, CUSTOM_FLAG_UNO: 1, CUSTOM_FLAG_ZWEI: 2, CUSTOM_FLAG_SHI_OR_YON: 4, CUSTOM_FLAG_LOTS_O_BITS: 240. Additional unspecified bits may be set.(e.g.: BIT_FLAG|BIT_FLAG2|4)",
                "type" : "string", }
            )
        }
    }
};
const _: () = {
    use havok_serde as __serde;
    impl __serde::Serialize for CustomEnum {
        fn serialize<S>(&self, __serializer: S) -> Result<S::Ok, S::Error>
        where
            S: __serde::ser::Serializer,
        {
            let mut __serializer = __serializer.serialize_enum_flags()?;
            match self {
                Self::CUSTOM_ENUM_ALA => {
                    __serializer.serialize_field("CUSTOM_ENUM_ALA", &0u64)
                }
                Self::CUSTOM_ENUM_DEPECHE => {
                    __serializer.serialize_field("CUSTOM_ENUM_DEPECHE", &1u64)
                }
                Self::CUSTOM_ENUM_FURIOUS => {
                    __serializer.serialize_field("CUSTOM_ENUM_FURIOUS", &5u64)
                }
            }?;
            use num_traits::ToPrimitive as _;
            let num = self
                .to_i8()
                .ok_or(S::Error::custom("Failed enum CustomEnum to_i8"))?;
            __serializer.serialize_bits(&num)?;
            __serializer.end()
        }
    }
};
const _: () = {
    use havok_serde as __serde;
    impl __serde::Serialize for CustomFlag {
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
                    Self::CUSTOM_FLAG_NONE => {
                        __serializer
                            .serialize_field("CUSTOM_FLAG_NONE", &Self::CUSTOM_FLAG_NONE)
                    }
                    Self::CUSTOM_FLAG_UNO => {
                        __serializer
                            .serialize_field("CUSTOM_FLAG_UNO", &Self::CUSTOM_FLAG_UNO)
                    }
                    Self::CUSTOM_FLAG_ZWEI => {
                        __serializer
                            .serialize_field("CUSTOM_FLAG_ZWEI", &Self::CUSTOM_FLAG_ZWEI)
                    }
                    Self::CUSTOM_FLAG_SHI_OR_YON => {
                        __serializer
                            .serialize_field(
                                "CUSTOM_FLAG_SHI_OR_YON",
                                &Self::CUSTOM_FLAG_SHI_OR_YON,
                            )
                    }
                    Self::CUSTOM_FLAG_LOTS_O_BITS => {
                        __serializer
                            .serialize_field(
                                "CUSTOM_FLAG_LOTS_O_BITS",
                                &Self::CUSTOM_FLAG_LOTS_O_BITS,
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
#[doc(hidden)]
#[allow(non_upper_case_globals, unused_attributes, unused_qualifications)]
const _: () = {
    #[allow(unused_extern_crates, clippy::useless_attribute)]
    extern crate havok_serde as _serde;
    #[automatically_derived]
    impl<'de> _serde::Deserialize<'de> for CustomEnum {
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
                        I8::Number(5i8) => _serde::__private::Ok(__Field::__field2),
                        _ => {
                            _serde::__private::Err(
                                _serde::de::Error::invalid_value(
                                    _serde::de::Unexpected::Int8(__value),
                                    &"value(i8) of variant is one of 0, 1, 5",
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
                            v if v == "0"
                                || v.eq_ignore_ascii_case("CUSTOM_ENUM_ALA") => {
                                _serde::__private::Ok(__Field::__field0)
                            }
                            v if v == "1"
                                || v.eq_ignore_ascii_case("CUSTOM_ENUM_DEPECHE") => {
                                _serde::__private::Ok(__Field::__field1)
                            }
                            v if v == "5"
                                || v.eq_ignore_ascii_case("CUSTOM_ENUM_FURIOUS") => {
                                _serde::__private::Ok(__Field::__field2)
                            }
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
                marker: _serde::__private::PhantomData<CustomEnum>,
                lifetime: _serde::__private::PhantomData<&'de ()>,
            }
            impl<'de> _serde::de::Visitor<'de> for __Visitor<'de> {
                type Value = CustomEnum;
                fn expecting(
                    &self,
                    __formatter: &mut _serde::__private::Formatter,
                ) -> _serde::__private::fmt::Result {
                    _serde::__private::Formatter::write_str(
                        __formatter,
                        "enum CustomEnum",
                    )
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
                            _serde::__private::Ok(CustomEnum::CUSTOM_ENUM_ALA)
                        }
                        (__Field::__field1, __variant) => {
                            _serde::de::VariantAccess::unit_variant(__variant)?;
                            _serde::__private::Ok(CustomEnum::CUSTOM_ENUM_DEPECHE)
                        }
                        (__Field::__field2, __variant) => {
                            _serde::de::VariantAccess::unit_variant(__variant)?;
                            _serde::__private::Ok(CustomEnum::CUSTOM_ENUM_FURIOUS)
                        }
                    }
                }
            }
            #[doc(hidden)]
            const VARIANTS: &'static [&'static str] = &[
                "CUSTOM_ENUM_ALA",
                "CUSTOM_ENUM_DEPECHE",
                "CUSTOM_ENUM_FURIOUS",
            ];
            _serde::Deserializer::deserialize_enum(
                __deserializer,
                "CustomEnum",
                VARIANTS,
                __Visitor {
                    marker: _serde::__private::PhantomData::<CustomEnum>,
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
    impl<'de> _serde::Deserialize<'de> for CustomFlag {
        fn deserialize<__D>(
            __deserializer: __D,
        ) -> _serde::__private::Result<Self, __D::Error>
        where
            __D: _serde::Deserializer<'de>,
        {
            #[doc(hidden)]
            struct __Visitor<'de> {
                marker: _serde::__private::PhantomData<CustomFlag>,
                lifetime: _serde::__private::PhantomData<&'de ()>,
            }
            impl<'de> _serde::de::Visitor<'de> for __Visitor<'de> {
                type Value = CustomFlag;
                fn expecting(
                    &self,
                    __formatter: &mut _serde::__private::Formatter,
                ) -> _serde::__private::fmt::Result {
                    _serde::__private::Formatter::write_str(
                        __formatter,
                        "struct CustomFlag(flags)",
                    )
                }
                #[inline]
                fn visit_int8<__E>(
                    self,
                    __value: I8<'de>,
                ) -> _serde::__private::Result<Self::Value, __E>
                where
                    __E: _serde::de::Error,
                {
                    match __value {
                        I8::Number(__value) => {
                            Ok(CustomFlag::from_bits_retain(__value as _))
                        }
                        _ => {
                            Err(
                                _serde::de::Error::invalid_value(
                                    _serde::de::Unexpected::Int8(__value as _),
                                    &"CustomFlag(I8) Number",
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
                    match <CustomFlag as core::str::FromStr>::from_str(
                        __value.into_inner().unwrap().as_ref(),
                    ) {
                        Ok(flags) => Ok(flags),
                        Err(err) => Err(_serde::de::Error::custom(err)),
                    }
                }
            }
            _serde::Deserializer::deserialize_flags(
                __deserializer,
                _serde::de::ReadEnumSize::Int8,
                __Visitor {
                    marker: _serde::__private::PhantomData::<CustomFlag>,
                    lifetime: _serde::__private::PhantomData,
                },
            )
        }
    }
};
