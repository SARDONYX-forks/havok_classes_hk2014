use super::class_requires::*;
use super::*;
/// # C++ Info
/// - name: `hkbCustomTestGeneratorSimpleTypes`
/// - version: `0`
/// - signature: `0x7b280d91`
/// - size: `  0`(x86)/`256`(x86_64)
/// -  vtable: `true`
#[allow(non_upper_case_globals, non_snake_case)]
#[cfg_attr(feature = "json_schema", derive(schemars::JsonSchema))]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(educe::Educe)]
#[educe(Debug, Clone, Default, PartialEq)]
pub struct hkbCustomTestGeneratorSimpleTypes<'a> {
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
    pub parent: hkbCustomTestGeneratorHiddenTypes<'a>,
    /// # C++ Info
    /// - name: `simpleTypeHkInt64`(ctype: `hkInt64`)
    /// - offset: `  0`(x86)/`152`(x86_64)
    /// - type_size: `  0`(x86)/`  8`(x86_64)
    /// - flags: `ALIGN_8`
    #[cfg_attr(feature = "serde_default", serde(default))]
    #[cfg_attr(feature = "serde", serde(borrow))]
    #[cfg_attr(feature = "json_schema", schemars(rename = "simpleTypeHkInt64"))]
    #[cfg_attr(feature = "serde", serde(rename = "simpleTypeHkInt64"))]
    pub m_simpleTypeHkInt64: I64<'a>,
    /// # C++ Info
    /// - name: `simpleTypeHkUint64`(ctype: `hkUint64`)
    /// - offset: `  0`(x86)/`160`(x86_64)
    /// - type_size: `  0`(x86)/`  8`(x86_64)
    #[cfg_attr(feature = "serde_default", serde(default))]
    #[cfg_attr(feature = "serde", serde(borrow))]
    #[cfg_attr(feature = "json_schema", schemars(rename = "simpleTypeHkUint64"))]
    #[cfg_attr(feature = "serde", serde(rename = "simpleTypeHkUint64"))]
    pub m_simpleTypeHkUint64: U64<'a>,
    /// # C++ Info
    /// - name: `simpleHiddenTypeCopyStart`(ctype: `hkBool`)
    /// - offset: `  0`(x86)/`168`(x86_64)
    /// - type_size: `  0`(x86)/`  1`(x86_64)
    #[cfg_attr(feature = "serde_default", serde(default))]
    #[cfg_attr(feature = "json_schema", schemars(rename = "simpleHiddenTypeCopyStart"))]
    #[cfg_attr(feature = "serde", serde(rename = "simpleHiddenTypeCopyStart"))]
    pub m_simpleHiddenTypeCopyStart: bool,
    /// # C++ Info
    /// - name: `simpleTypeBool`(ctype: `hkBool`)
    /// - offset: `  0`(x86)/`169`(x86_64)
    /// - type_size: `  0`(x86)/`  1`(x86_64)
    #[cfg_attr(feature = "serde_default", serde(default))]
    #[cfg_attr(feature = "json_schema", schemars(rename = "simpleTypeBool"))]
    #[cfg_attr(feature = "serde", serde(rename = "simpleTypeBool"))]
    pub m_simpleTypeBool: bool,
    /// # C++ Info
    /// - name: `simpleTypeHkBool`(ctype: `hkBool`)
    /// - offset: `  0`(x86)/`170`(x86_64)
    /// - type_size: `  0`(x86)/`  1`(x86_64)
    #[cfg_attr(feature = "serde_default", serde(default))]
    #[cfg_attr(feature = "json_schema", schemars(rename = "simpleTypeHkBool"))]
    #[cfg_attr(feature = "serde", serde(rename = "simpleTypeHkBool"))]
    pub m_simpleTypeHkBool: bool,
    /// # C++ Info
    /// - name: `simpleTypeCString`(ctype: `char*`)
    /// - offset: `  0`(x86)/`176`(x86_64)
    /// - type_size: `  0`(x86)/`  8`(x86_64)
    #[cfg_attr(feature = "serde_default", serde(default))]
    #[cfg_attr(feature = "serde", serde(borrow))]
    #[cfg_attr(feature = "json_schema", schemars(rename = "simpleTypeCString"))]
    #[cfg_attr(feature = "serde", serde(rename = "simpleTypeCString"))]
    pub m_simpleTypeCString: CString<'a>,
    /// # C++ Info
    /// - name: `simpleTypeHkStringPtr`(ctype: `hkStringPtr`)
    /// - offset: `  0`(x86)/`184`(x86_64)
    /// - type_size: `  0`(x86)/`  8`(x86_64)
    #[cfg_attr(feature = "serde_default", serde(default))]
    #[cfg_attr(feature = "serde", serde(borrow))]
    #[cfg_attr(feature = "json_schema", schemars(rename = "simpleTypeHkStringPtr"))]
    #[cfg_attr(feature = "serde", serde(rename = "simpleTypeHkStringPtr"))]
    pub m_simpleTypeHkStringPtr: StringPtr<'a>,
    /// # C++ Info
    /// - name: `simpleTypeHkInt8`(ctype: `hkInt8`)
    /// - offset: `  0`(x86)/`192`(x86_64)
    /// - type_size: `  0`(x86)/`  1`(x86_64)
    #[cfg_attr(feature = "serde_default", serde(default))]
    #[cfg_attr(feature = "serde", serde(borrow))]
    #[cfg_attr(feature = "json_schema", schemars(rename = "simpleTypeHkInt8"))]
    #[cfg_attr(feature = "serde", serde(rename = "simpleTypeHkInt8"))]
    pub m_simpleTypeHkInt8: I8<'a>,
    /// # C++ Info
    /// - name: `simpleTypeHkInt16`(ctype: `hkInt16`)
    /// - offset: `  0`(x86)/`194`(x86_64)
    /// - type_size: `  0`(x86)/`  2`(x86_64)
    #[cfg_attr(feature = "serde_default", serde(default))]
    #[cfg_attr(feature = "serde", serde(borrow))]
    #[cfg_attr(feature = "json_schema", schemars(rename = "simpleTypeHkInt16"))]
    #[cfg_attr(feature = "serde", serde(rename = "simpleTypeHkInt16"))]
    pub m_simpleTypeHkInt16: I16<'a>,
    /// # C++ Info
    /// - name: `simpleTypeHkInt32`(ctype: `hkInt32`)
    /// - offset: `  0`(x86)/`196`(x86_64)
    /// - type_size: `  0`(x86)/`  4`(x86_64)
    #[cfg_attr(feature = "serde_default", serde(default))]
    #[cfg_attr(feature = "serde", serde(borrow))]
    #[cfg_attr(feature = "json_schema", schemars(rename = "simpleTypeHkInt32"))]
    #[cfg_attr(feature = "serde", serde(rename = "simpleTypeHkInt32"))]
    pub m_simpleTypeHkInt32: I32<'a>,
    /// # C++ Info
    /// - name: `simpleTypeHkUint8`(ctype: `hkUint8`)
    /// - offset: `  0`(x86)/`200`(x86_64)
    /// - type_size: `  0`(x86)/`  1`(x86_64)
    #[cfg_attr(feature = "serde_default", serde(default))]
    #[cfg_attr(feature = "serde", serde(borrow))]
    #[cfg_attr(feature = "json_schema", schemars(rename = "simpleTypeHkUint8"))]
    #[cfg_attr(feature = "serde", serde(rename = "simpleTypeHkUint8"))]
    pub m_simpleTypeHkUint8: U8<'a>,
    /// # C++ Info
    /// - name: `simpleTypeHkUint16`(ctype: `hkUint16`)
    /// - offset: `  0`(x86)/`202`(x86_64)
    /// - type_size: `  0`(x86)/`  2`(x86_64)
    #[cfg_attr(feature = "serde_default", serde(default))]
    #[cfg_attr(feature = "serde", serde(borrow))]
    #[cfg_attr(feature = "json_schema", schemars(rename = "simpleTypeHkUint16"))]
    #[cfg_attr(feature = "serde", serde(rename = "simpleTypeHkUint16"))]
    pub m_simpleTypeHkUint16: U16<'a>,
    /// # C++ Info
    /// - name: `simpleTypeHkUint32`(ctype: `hkUint32`)
    /// - offset: `  0`(x86)/`204`(x86_64)
    /// - type_size: `  0`(x86)/`  4`(x86_64)
    #[cfg_attr(feature = "serde_default", serde(default))]
    #[cfg_attr(feature = "serde", serde(borrow))]
    #[cfg_attr(feature = "json_schema", schemars(rename = "simpleTypeHkUint32"))]
    #[cfg_attr(feature = "serde", serde(rename = "simpleTypeHkUint32"))]
    pub m_simpleTypeHkUint32: U32<'a>,
    /// # C++ Info
    /// - name: `simpleTypeHkReal`(ctype: `hkReal`)
    /// - offset: `  0`(x86)/`208`(x86_64)
    /// - type_size: `  0`(x86)/`  4`(x86_64)
    #[cfg_attr(feature = "serde_default", serde(default))]
    #[cfg_attr(feature = "json_schema", schemars(rename = "simpleTypeHkReal"))]
    #[cfg_attr(feature = "serde", serde(rename = "simpleTypeHkReal"))]
    pub m_simpleTypeHkReal: f32,
    /// # C++ Info
    /// - name: `simpleTypeHkInt8Default`(ctype: `hkInt8`)
    /// - offset: `  0`(x86)/`212`(x86_64)
    /// - type_size: `  0`(x86)/`  1`(x86_64)
    #[cfg_attr(feature = "serde_default", serde(default))]
    #[cfg_attr(feature = "serde", serde(borrow))]
    #[cfg_attr(feature = "json_schema", schemars(rename = "simpleTypeHkInt8Default"))]
    #[cfg_attr(feature = "serde", serde(rename = "simpleTypeHkInt8Default"))]
    pub m_simpleTypeHkInt8Default: I8<'a>,
    /// # C++ Info
    /// - name: `simpleTypeHkInt16Default`(ctype: `hkInt16`)
    /// - offset: `  0`(x86)/`214`(x86_64)
    /// - type_size: `  0`(x86)/`  2`(x86_64)
    #[cfg_attr(feature = "serde_default", serde(default))]
    #[cfg_attr(feature = "serde", serde(borrow))]
    #[cfg_attr(feature = "json_schema", schemars(rename = "simpleTypeHkInt16Default"))]
    #[cfg_attr(feature = "serde", serde(rename = "simpleTypeHkInt16Default"))]
    pub m_simpleTypeHkInt16Default: I16<'a>,
    /// # C++ Info
    /// - name: `simpleTypeHkInt32Default`(ctype: `hkInt32`)
    /// - offset: `  0`(x86)/`216`(x86_64)
    /// - type_size: `  0`(x86)/`  4`(x86_64)
    #[cfg_attr(feature = "serde_default", serde(default))]
    #[cfg_attr(feature = "serde", serde(borrow))]
    #[cfg_attr(feature = "json_schema", schemars(rename = "simpleTypeHkInt32Default"))]
    #[cfg_attr(feature = "serde", serde(rename = "simpleTypeHkInt32Default"))]
    pub m_simpleTypeHkInt32Default: I32<'a>,
    /// # C++ Info
    /// - name: `simpleTypeHkUint8Default`(ctype: `hkUint8`)
    /// - offset: `  0`(x86)/`220`(x86_64)
    /// - type_size: `  0`(x86)/`  1`(x86_64)
    #[cfg_attr(feature = "serde_default", serde(default))]
    #[cfg_attr(feature = "serde", serde(borrow))]
    #[cfg_attr(feature = "json_schema", schemars(rename = "simpleTypeHkUint8Default"))]
    #[cfg_attr(feature = "serde", serde(rename = "simpleTypeHkUint8Default"))]
    pub m_simpleTypeHkUint8Default: U8<'a>,
    /// # C++ Info
    /// - name: `simpleTypeHkUint16Default`(ctype: `hkUint16`)
    /// - offset: `  0`(x86)/`222`(x86_64)
    /// - type_size: `  0`(x86)/`  2`(x86_64)
    #[cfg_attr(feature = "serde_default", serde(default))]
    #[cfg_attr(feature = "serde", serde(borrow))]
    #[cfg_attr(feature = "json_schema", schemars(rename = "simpleTypeHkUint16Default"))]
    #[cfg_attr(feature = "serde", serde(rename = "simpleTypeHkUint16Default"))]
    pub m_simpleTypeHkUint16Default: U16<'a>,
    /// # C++ Info
    /// - name: `simpleTypeHkUint32Default`(ctype: `hkUint32`)
    /// - offset: `  0`(x86)/`224`(x86_64)
    /// - type_size: `  0`(x86)/`  4`(x86_64)
    #[cfg_attr(feature = "serde_default", serde(default))]
    #[cfg_attr(feature = "serde", serde(borrow))]
    #[cfg_attr(feature = "json_schema", schemars(rename = "simpleTypeHkUint32Default"))]
    #[cfg_attr(feature = "serde", serde(rename = "simpleTypeHkUint32Default"))]
    pub m_simpleTypeHkUint32Default: U32<'a>,
    /// # C++ Info
    /// - name: `simpleTypeHkRealDefault`(ctype: `hkReal`)
    /// - offset: `  0`(x86)/`228`(x86_64)
    /// - type_size: `  0`(x86)/`  4`(x86_64)
    #[cfg_attr(feature = "serde_default", serde(default))]
    #[cfg_attr(feature = "json_schema", schemars(rename = "simpleTypeHkRealDefault"))]
    #[cfg_attr(feature = "serde", serde(rename = "simpleTypeHkRealDefault"))]
    pub m_simpleTypeHkRealDefault: f32,
    /// # C++ Info
    /// - name: `simpleTypeHkInt8Clamp`(ctype: `hkInt8`)
    /// - offset: `  0`(x86)/`232`(x86_64)
    /// - type_size: `  0`(x86)/`  1`(x86_64)
    #[cfg_attr(feature = "serde_default", serde(default))]
    #[cfg_attr(feature = "serde", serde(borrow))]
    #[cfg_attr(feature = "json_schema", schemars(rename = "simpleTypeHkInt8Clamp"))]
    #[cfg_attr(feature = "serde", serde(rename = "simpleTypeHkInt8Clamp"))]
    pub m_simpleTypeHkInt8Clamp: I8<'a>,
    /// # C++ Info
    /// - name: `simpleTypeHkInt16Clamp`(ctype: `hkInt16`)
    /// - offset: `  0`(x86)/`234`(x86_64)
    /// - type_size: `  0`(x86)/`  2`(x86_64)
    #[cfg_attr(feature = "serde_default", serde(default))]
    #[cfg_attr(feature = "serde", serde(borrow))]
    #[cfg_attr(feature = "json_schema", schemars(rename = "simpleTypeHkInt16Clamp"))]
    #[cfg_attr(feature = "serde", serde(rename = "simpleTypeHkInt16Clamp"))]
    pub m_simpleTypeHkInt16Clamp: I16<'a>,
    /// # C++ Info
    /// - name: `simpleTypeHkInt32Clamp`(ctype: `hkInt32`)
    /// - offset: `  0`(x86)/`236`(x86_64)
    /// - type_size: `  0`(x86)/`  4`(x86_64)
    #[cfg_attr(feature = "serde_default", serde(default))]
    #[cfg_attr(feature = "serde", serde(borrow))]
    #[cfg_attr(feature = "json_schema", schemars(rename = "simpleTypeHkInt32Clamp"))]
    #[cfg_attr(feature = "serde", serde(rename = "simpleTypeHkInt32Clamp"))]
    pub m_simpleTypeHkInt32Clamp: I32<'a>,
    /// # C++ Info
    /// - name: `simpleTypeHkUint8Clamp`(ctype: `hkUint8`)
    /// - offset: `  0`(x86)/`240`(x86_64)
    /// - type_size: `  0`(x86)/`  1`(x86_64)
    #[cfg_attr(feature = "serde_default", serde(default))]
    #[cfg_attr(feature = "serde", serde(borrow))]
    #[cfg_attr(feature = "json_schema", schemars(rename = "simpleTypeHkUint8Clamp"))]
    #[cfg_attr(feature = "serde", serde(rename = "simpleTypeHkUint8Clamp"))]
    pub m_simpleTypeHkUint8Clamp: U8<'a>,
    /// # C++ Info
    /// - name: `simpleTypeHkUint16Clamp`(ctype: `hkUint16`)
    /// - offset: `  0`(x86)/`242`(x86_64)
    /// - type_size: `  0`(x86)/`  2`(x86_64)
    #[cfg_attr(feature = "serde_default", serde(default))]
    #[cfg_attr(feature = "serde", serde(borrow))]
    #[cfg_attr(feature = "json_schema", schemars(rename = "simpleTypeHkUint16Clamp"))]
    #[cfg_attr(feature = "serde", serde(rename = "simpleTypeHkUint16Clamp"))]
    pub m_simpleTypeHkUint16Clamp: U16<'a>,
    /// # C++ Info
    /// - name: `simpleTypeHkUint32Clamp`(ctype: `hkUint32`)
    /// - offset: `  0`(x86)/`244`(x86_64)
    /// - type_size: `  0`(x86)/`  4`(x86_64)
    #[cfg_attr(feature = "serde_default", serde(default))]
    #[cfg_attr(feature = "serde", serde(borrow))]
    #[cfg_attr(feature = "json_schema", schemars(rename = "simpleTypeHkUint32Clamp"))]
    #[cfg_attr(feature = "serde", serde(rename = "simpleTypeHkUint32Clamp"))]
    pub m_simpleTypeHkUint32Clamp: U32<'a>,
    /// # C++ Info
    /// - name: `simpleTypeHkRealClamp`(ctype: `hkReal`)
    /// - offset: `  0`(x86)/`248`(x86_64)
    /// - type_size: `  0`(x86)/`  4`(x86_64)
    #[cfg_attr(feature = "serde_default", serde(default))]
    #[cfg_attr(feature = "json_schema", schemars(rename = "simpleTypeHkRealClamp"))]
    #[cfg_attr(feature = "serde", serde(rename = "simpleTypeHkRealClamp"))]
    pub m_simpleTypeHkRealClamp: f32,
    /// # C++ Info
    /// - name: `simpleHiddenTypeCopyEnd`(ctype: `hkBool`)
    /// - offset: `  0`(x86)/`252`(x86_64)
    /// - type_size: `  0`(x86)/`  1`(x86_64)
    #[cfg_attr(feature = "serde_default", serde(default))]
    #[cfg_attr(feature = "json_schema", schemars(rename = "simpleHiddenTypeCopyEnd"))]
    #[cfg_attr(feature = "serde", serde(rename = "simpleHiddenTypeCopyEnd"))]
    pub m_simpleHiddenTypeCopyEnd: bool,
}
const _: () = {
    use havok_serde as _serde;
    impl<'a> _serde::HavokClass for hkbCustomTestGeneratorSimpleTypes<'a> {
        #[inline]
        fn name(&self) -> &'static str {
            "hkbCustomTestGeneratorSimpleTypes"
        }
        #[inline]
        fn signature(&self) -> _serde::__private::Signature {
            _serde::__private::Signature::new(0x7b280d91)
        }
        #[allow(clippy::let_and_return, clippy::vec_init_then_push)]
        fn deps_indexes(&self) -> Vec<&Pointer<'_>> {
            let mut v = Vec::new();
            v.push(&self.parent.parent.parent.parent.parent.m_variableBindingSet);
            v.extend(self.parent.parent.parent.parent.parent.m_cachedBindables.iter());
            v.push(&self.parent.parent.parent.parent.m_nodeInfo);
            v.extend(self.parent.parent.parent.m_partitionInfo.deps_indexes());
            v.push(&self.parent.parent.parent.m_syncInfo);
            v.push(&self.parent.parent.m_skeleton);
            v
        }
    }
    impl<'a> _serde::Serialize for hkbCustomTestGeneratorSimpleTypes<'a> {
        fn serialize<S>(&self, __serializer: S) -> Result<S::Ok, S::Error>
        where
            S: _serde::ser::Serializer,
        {
            let class_meta = self
                .__ptr
                .as_ref()
                .map(|name| (name, _serde::__private::Signature::new(0x7b280d91)));
            let mut serializer = __serializer
                .serialize_struct(
                    "hkbCustomTestGeneratorSimpleTypes",
                    class_meta,
                    (0u64, 256u64),
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
            serializer.skip_field("skeleton", &self.parent.parent.m_skeleton)?;
            serializer
                .serialize_field(
                    "inheritedHiddenMember",
                    &self.parent.m_inheritedHiddenMember,
                )?;
            serializer
                .serialize_field(
                    "protectedInheritedHiddenMember",
                    &self.parent.m_protectedInheritedHiddenMember,
                )?;
            serializer
                .serialize_field(
                    "privateInheritedHiddenMember",
                    &self.parent.m_privateInheritedHiddenMember,
                )?;
            serializer.pad_field([0u8; 0usize].as_slice(), [0u8; 5usize].as_slice())?;
            serializer.serialize_field("simpleTypeHkInt64", &self.m_simpleTypeHkInt64)?;
            serializer
                .serialize_field("simpleTypeHkUint64", &self.m_simpleTypeHkUint64)?;
            serializer
                .serialize_field(
                    "simpleHiddenTypeCopyStart",
                    &self.m_simpleHiddenTypeCopyStart,
                )?;
            serializer.serialize_field("simpleTypeBool", &self.m_simpleTypeBool)?;
            serializer.serialize_field("simpleTypeHkBool", &self.m_simpleTypeHkBool)?;
            serializer.pad_field([0u8; 0usize].as_slice(), [0u8; 5usize].as_slice())?;
            serializer.serialize_field("simpleTypeCString", &self.m_simpleTypeCString)?;
            serializer
                .serialize_field(
                    "simpleTypeHkStringPtr",
                    &self.m_simpleTypeHkStringPtr,
                )?;
            serializer.serialize_field("simpleTypeHkInt8", &self.m_simpleTypeHkInt8)?;
            serializer.pad_field([0u8; 0usize].as_slice(), [0u8; 1usize].as_slice())?;
            serializer.serialize_field("simpleTypeHkInt16", &self.m_simpleTypeHkInt16)?;
            serializer.serialize_field("simpleTypeHkInt32", &self.m_simpleTypeHkInt32)?;
            serializer.serialize_field("simpleTypeHkUint8", &self.m_simpleTypeHkUint8)?;
            serializer.pad_field([0u8; 0usize].as_slice(), [0u8; 1usize].as_slice())?;
            serializer
                .serialize_field("simpleTypeHkUint16", &self.m_simpleTypeHkUint16)?;
            serializer
                .serialize_field("simpleTypeHkUint32", &self.m_simpleTypeHkUint32)?;
            serializer.serialize_field("simpleTypeHkReal", &self.m_simpleTypeHkReal)?;
            serializer
                .serialize_field(
                    "simpleTypeHkInt8Default",
                    &self.m_simpleTypeHkInt8Default,
                )?;
            serializer.pad_field([0u8; 0usize].as_slice(), [0u8; 1usize].as_slice())?;
            serializer
                .serialize_field(
                    "simpleTypeHkInt16Default",
                    &self.m_simpleTypeHkInt16Default,
                )?;
            serializer
                .serialize_field(
                    "simpleTypeHkInt32Default",
                    &self.m_simpleTypeHkInt32Default,
                )?;
            serializer
                .serialize_field(
                    "simpleTypeHkUint8Default",
                    &self.m_simpleTypeHkUint8Default,
                )?;
            serializer.pad_field([0u8; 0usize].as_slice(), [0u8; 1usize].as_slice())?;
            serializer
                .serialize_field(
                    "simpleTypeHkUint16Default",
                    &self.m_simpleTypeHkUint16Default,
                )?;
            serializer
                .serialize_field(
                    "simpleTypeHkUint32Default",
                    &self.m_simpleTypeHkUint32Default,
                )?;
            serializer
                .serialize_field(
                    "simpleTypeHkRealDefault",
                    &self.m_simpleTypeHkRealDefault,
                )?;
            serializer
                .serialize_field(
                    "simpleTypeHkInt8Clamp",
                    &self.m_simpleTypeHkInt8Clamp,
                )?;
            serializer.pad_field([0u8; 0usize].as_slice(), [0u8; 1usize].as_slice())?;
            serializer
                .serialize_field(
                    "simpleTypeHkInt16Clamp",
                    &self.m_simpleTypeHkInt16Clamp,
                )?;
            serializer
                .serialize_field(
                    "simpleTypeHkInt32Clamp",
                    &self.m_simpleTypeHkInt32Clamp,
                )?;
            serializer
                .serialize_field(
                    "simpleTypeHkUint8Clamp",
                    &self.m_simpleTypeHkUint8Clamp,
                )?;
            serializer.pad_field([0u8; 0usize].as_slice(), [0u8; 1usize].as_slice())?;
            serializer
                .serialize_field(
                    "simpleTypeHkUint16Clamp",
                    &self.m_simpleTypeHkUint16Clamp,
                )?;
            serializer
                .serialize_field(
                    "simpleTypeHkUint32Clamp",
                    &self.m_simpleTypeHkUint32Clamp,
                )?;
            serializer
                .serialize_field(
                    "simpleTypeHkRealClamp",
                    &self.m_simpleTypeHkRealClamp,
                )?;
            serializer
                .serialize_field(
                    "simpleHiddenTypeCopyEnd",
                    &self.m_simpleHiddenTypeCopyEnd,
                )?;
            serializer.pad_field([0u8; 0usize].as_slice(), [0u8; 3usize].as_slice())?;
            serializer.end()
        }
    }
};
#[doc(hidden)]
#[allow(non_upper_case_globals, unused_attributes, unused_qualifications)]
const _: () = {
    use havok_serde as _serde;
    #[automatically_derived]
    impl<'de> _serde::Deserialize<'de> for hkbCustomTestGeneratorSimpleTypes<'de> {
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
            struct __hkbCustomTestGeneratorSimpleTypesVisitor<'de> {
                marker: _serde::__private::PhantomData<
                    hkbCustomTestGeneratorSimpleTypes<'de>,
                >,
                lifetime: _serde::__private::PhantomData<&'de ()>,
            }
            #[allow(clippy::match_single_binding)]
            #[allow(clippy::reversed_empty_ranges)]
            #[allow(clippy::single_match)]
            impl<'de> _serde::de::Visitor<'de>
            for __hkbCustomTestGeneratorSimpleTypesVisitor<'de> {
                type Value = hkbCustomTestGeneratorSimpleTypes<'de>;
                fn expecting(
                    &self,
                    __formatter: &mut core::fmt::Formatter,
                ) -> core::fmt::Result {
                    core::fmt::Formatter::write_str(
                        __formatter,
                        "struct hkbCustomTestGeneratorSimpleTypes",
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
                    for i in 0..29usize {
                        match i {
                            0usize => {
                                if _serde::__private::Option::is_some(
                                    &m_simpleTypeHkInt64,
                                ) {
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
                            1usize => {
                                if _serde::__private::Option::is_some(
                                    &m_simpleTypeHkUint64,
                                ) {
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
                            2usize => {
                                if _serde::__private::Option::is_some(
                                    &m_simpleHiddenTypeCopyStart,
                                ) {
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
                            3usize => {
                                if _serde::__private::Option::is_some(&m_simpleTypeBool) {
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
                            4usize => {
                                if _serde::__private::Option::is_some(&m_simpleTypeHkBool) {
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
                            5usize => {
                                if _serde::__private::Option::is_some(
                                    &m_simpleTypeCString,
                                ) {
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "simpleTypeCString",
                                        ),
                                    );
                                }
                                __A::pad(&mut __map, 0usize, 5usize)?;
                                m_simpleTypeCString = _serde::__private::Some(
                                    match __A::next_value::<CString<'de>>(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            6usize => {
                                if _serde::__private::Option::is_some(
                                    &m_simpleTypeHkStringPtr,
                                ) {
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
                            7usize => {
                                if _serde::__private::Option::is_some(&m_simpleTypeHkInt8) {
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
                            8usize => {
                                if _serde::__private::Option::is_some(
                                    &m_simpleTypeHkInt16,
                                ) {
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "simpleTypeHkInt16",
                                        ),
                                    );
                                }
                                __A::pad(&mut __map, 0usize, 1usize)?;
                                m_simpleTypeHkInt16 = _serde::__private::Some(
                                    match __A::next_value::<I16<'de>>(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            9usize => {
                                if _serde::__private::Option::is_some(
                                    &m_simpleTypeHkInt32,
                                ) {
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
                            10usize => {
                                if _serde::__private::Option::is_some(
                                    &m_simpleTypeHkUint8,
                                ) {
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
                            11usize => {
                                if _serde::__private::Option::is_some(
                                    &m_simpleTypeHkUint16,
                                ) {
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "simpleTypeHkUint16",
                                        ),
                                    );
                                }
                                __A::pad(&mut __map, 0usize, 1usize)?;
                                m_simpleTypeHkUint16 = _serde::__private::Some(
                                    match __A::next_value::<U16<'de>>(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            12usize => {
                                if _serde::__private::Option::is_some(
                                    &m_simpleTypeHkUint32,
                                ) {
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
                            13usize => {
                                if _serde::__private::Option::is_some(&m_simpleTypeHkReal) {
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
                            14usize => {
                                if _serde::__private::Option::is_some(
                                    &m_simpleTypeHkInt8Default,
                                ) {
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
                            15usize => {
                                if _serde::__private::Option::is_some(
                                    &m_simpleTypeHkInt16Default,
                                ) {
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "simpleTypeHkInt16Default",
                                        ),
                                    );
                                }
                                __A::pad(&mut __map, 0usize, 1usize)?;
                                m_simpleTypeHkInt16Default = _serde::__private::Some(
                                    match __A::next_value::<I16<'de>>(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            16usize => {
                                if _serde::__private::Option::is_some(
                                    &m_simpleTypeHkInt32Default,
                                ) {
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
                            17usize => {
                                if _serde::__private::Option::is_some(
                                    &m_simpleTypeHkUint8Default,
                                ) {
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
                            18usize => {
                                if _serde::__private::Option::is_some(
                                    &m_simpleTypeHkUint16Default,
                                ) {
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "simpleTypeHkUint16Default",
                                        ),
                                    );
                                }
                                __A::pad(&mut __map, 0usize, 1usize)?;
                                m_simpleTypeHkUint16Default = _serde::__private::Some(
                                    match __A::next_value::<U16<'de>>(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            19usize => {
                                if _serde::__private::Option::is_some(
                                    &m_simpleTypeHkUint32Default,
                                ) {
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
                            20usize => {
                                if _serde::__private::Option::is_some(
                                    &m_simpleTypeHkRealDefault,
                                ) {
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
                            21usize => {
                                if _serde::__private::Option::is_some(
                                    &m_simpleTypeHkInt8Clamp,
                                ) {
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
                            22usize => {
                                if _serde::__private::Option::is_some(
                                    &m_simpleTypeHkInt16Clamp,
                                ) {
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "simpleTypeHkInt16Clamp",
                                        ),
                                    );
                                }
                                __A::pad(&mut __map, 0usize, 1usize)?;
                                m_simpleTypeHkInt16Clamp = _serde::__private::Some(
                                    match __A::next_value::<I16<'de>>(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            23usize => {
                                if _serde::__private::Option::is_some(
                                    &m_simpleTypeHkInt32Clamp,
                                ) {
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
                            24usize => {
                                if _serde::__private::Option::is_some(
                                    &m_simpleTypeHkUint8Clamp,
                                ) {
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
                            25usize => {
                                if _serde::__private::Option::is_some(
                                    &m_simpleTypeHkUint16Clamp,
                                ) {
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "simpleTypeHkUint16Clamp",
                                        ),
                                    );
                                }
                                __A::pad(&mut __map, 0usize, 1usize)?;
                                m_simpleTypeHkUint16Clamp = _serde::__private::Some(
                                    match __A::next_value::<U16<'de>>(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            26usize => {
                                if _serde::__private::Option::is_some(
                                    &m_simpleTypeHkUint32Clamp,
                                ) {
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
                            27usize => {
                                if _serde::__private::Option::is_some(
                                    &m_simpleTypeHkRealClamp,
                                ) {
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
                            28usize => {
                                if _serde::__private::Option::is_some(
                                    &m_simpleHiddenTypeCopyEnd,
                                ) {
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
                            _ => {}
                        }
                    }
                    __A::pad(&mut __map, 0usize, 3usize)?;
                    let m_simpleTypeHkInt64 = match m_simpleTypeHkInt64 {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "simpleTypeHkInt64",
                                ),
                            );
                        }
                    };
                    let m_simpleTypeHkUint64 = match m_simpleTypeHkUint64 {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "simpleTypeHkUint64",
                                ),
                            );
                        }
                    };
                    let m_simpleHiddenTypeCopyStart = match m_simpleHiddenTypeCopyStart {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "simpleHiddenTypeCopyStart",
                                ),
                            );
                        }
                    };
                    let m_simpleTypeBool = match m_simpleTypeBool {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "simpleTypeBool",
                                ),
                            );
                        }
                    };
                    let m_simpleTypeHkBool = match m_simpleTypeHkBool {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "simpleTypeHkBool",
                                ),
                            );
                        }
                    };
                    let m_simpleTypeCString = match m_simpleTypeCString {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "simpleTypeCString",
                                ),
                            );
                        }
                    };
                    let m_simpleTypeHkStringPtr = match m_simpleTypeHkStringPtr {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "simpleTypeHkStringPtr",
                                ),
                            );
                        }
                    };
                    let m_simpleTypeHkInt8 = match m_simpleTypeHkInt8 {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "simpleTypeHkInt8",
                                ),
                            );
                        }
                    };
                    let m_simpleTypeHkInt16 = match m_simpleTypeHkInt16 {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "simpleTypeHkInt16",
                                ),
                            );
                        }
                    };
                    let m_simpleTypeHkInt32 = match m_simpleTypeHkInt32 {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "simpleTypeHkInt32",
                                ),
                            );
                        }
                    };
                    let m_simpleTypeHkUint8 = match m_simpleTypeHkUint8 {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "simpleTypeHkUint8",
                                ),
                            );
                        }
                    };
                    let m_simpleTypeHkUint16 = match m_simpleTypeHkUint16 {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "simpleTypeHkUint16",
                                ),
                            );
                        }
                    };
                    let m_simpleTypeHkUint32 = match m_simpleTypeHkUint32 {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "simpleTypeHkUint32",
                                ),
                            );
                        }
                    };
                    let m_simpleTypeHkReal = match m_simpleTypeHkReal {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "simpleTypeHkReal",
                                ),
                            );
                        }
                    };
                    let m_simpleTypeHkInt8Default = match m_simpleTypeHkInt8Default {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "simpleTypeHkInt8Default",
                                ),
                            );
                        }
                    };
                    let m_simpleTypeHkInt16Default = match m_simpleTypeHkInt16Default {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "simpleTypeHkInt16Default",
                                ),
                            );
                        }
                    };
                    let m_simpleTypeHkInt32Default = match m_simpleTypeHkInt32Default {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "simpleTypeHkInt32Default",
                                ),
                            );
                        }
                    };
                    let m_simpleTypeHkUint8Default = match m_simpleTypeHkUint8Default {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "simpleTypeHkUint8Default",
                                ),
                            );
                        }
                    };
                    let m_simpleTypeHkUint16Default = match m_simpleTypeHkUint16Default {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "simpleTypeHkUint16Default",
                                ),
                            );
                        }
                    };
                    let m_simpleTypeHkUint32Default = match m_simpleTypeHkUint32Default {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "simpleTypeHkUint32Default",
                                ),
                            );
                        }
                    };
                    let m_simpleTypeHkRealDefault = match m_simpleTypeHkRealDefault {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "simpleTypeHkRealDefault",
                                ),
                            );
                        }
                    };
                    let m_simpleTypeHkInt8Clamp = match m_simpleTypeHkInt8Clamp {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "simpleTypeHkInt8Clamp",
                                ),
                            );
                        }
                    };
                    let m_simpleTypeHkInt16Clamp = match m_simpleTypeHkInt16Clamp {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "simpleTypeHkInt16Clamp",
                                ),
                            );
                        }
                    };
                    let m_simpleTypeHkInt32Clamp = match m_simpleTypeHkInt32Clamp {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "simpleTypeHkInt32Clamp",
                                ),
                            );
                        }
                    };
                    let m_simpleTypeHkUint8Clamp = match m_simpleTypeHkUint8Clamp {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "simpleTypeHkUint8Clamp",
                                ),
                            );
                        }
                    };
                    let m_simpleTypeHkUint16Clamp = match m_simpleTypeHkUint16Clamp {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "simpleTypeHkUint16Clamp",
                                ),
                            );
                        }
                    };
                    let m_simpleTypeHkUint32Clamp = match m_simpleTypeHkUint32Clamp {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "simpleTypeHkUint32Clamp",
                                ),
                            );
                        }
                    };
                    let m_simpleTypeHkRealClamp = match m_simpleTypeHkRealClamp {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "simpleTypeHkRealClamp",
                                ),
                            );
                        }
                    };
                    let m_simpleHiddenTypeCopyEnd = match m_simpleHiddenTypeCopyEnd {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "simpleHiddenTypeCopyEnd",
                                ),
                            );
                        }
                    };
                    _serde::__private::Ok(hkbCustomTestGeneratorSimpleTypes {
                        __ptr,
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
                    let __ptr = __A::class_ptr(&mut __map);
                    _serde::__private::Ok(hkbCustomTestGeneratorSimpleTypes {
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
                    })
                }
            }
            const FIELDS: &[&str] = &[
                "simpleTypeHkInt64",
                "simpleTypeHkUint64",
                "simpleHiddenTypeCopyStart",
                "simpleTypeBool",
                "simpleTypeHkBool",
                "simpleTypeCString",
                "simpleTypeHkStringPtr",
                "simpleTypeHkInt8",
                "simpleTypeHkInt16",
                "simpleTypeHkInt32",
                "simpleTypeHkUint8",
                "simpleTypeHkUint16",
                "simpleTypeHkUint32",
                "simpleTypeHkReal",
                "simpleTypeHkInt8Default",
                "simpleTypeHkInt16Default",
                "simpleTypeHkInt32Default",
                "simpleTypeHkUint8Default",
                "simpleTypeHkUint16Default",
                "simpleTypeHkUint32Default",
                "simpleTypeHkRealDefault",
                "simpleTypeHkInt8Clamp",
                "simpleTypeHkInt16Clamp",
                "simpleTypeHkInt32Clamp",
                "simpleTypeHkUint8Clamp",
                "simpleTypeHkUint16Clamp",
                "simpleTypeHkUint32Clamp",
                "simpleTypeHkRealClamp",
                "simpleHiddenTypeCopyEnd",
            ];
            _serde::Deserializer::deserialize_struct(
                deserializer,
                "hkbCustomTestGeneratorSimpleTypes",
                FIELDS,
                __hkbCustomTestGeneratorSimpleTypesVisitor {
                    marker: _serde::__private::PhantomData::<
                        hkbCustomTestGeneratorSimpleTypes,
                    >,
                    lifetime: _serde::__private::PhantomData,
                },
            )
        }
    }
};
