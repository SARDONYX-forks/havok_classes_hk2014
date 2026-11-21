use super::class_requires::*;
use super::*;
/// # C++ Info
/// - name: `BSReverseSpineTwistModifier`
/// - version: `2`
/// - signature: `0xfc8adeda`
/// - size: `  0`(x86)/`256`(x86_64)
/// -  vtable: `true`
#[allow(non_upper_case_globals, non_snake_case)]
#[cfg_attr(feature = "json_schema", derive(schemars::JsonSchema))]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(educe::Educe)]
#[educe(Debug, Clone, Default, PartialEq)]
pub struct BSReverseSpineTwistModifier<'a> {
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
    pub parent: hkbModifier<'a>,
    /// # C++ Info
    /// - name: `isAdditive`(ctype: `hkBool`)
    /// - offset: `  0`(x86)/` 88`(x86_64)
    /// - type_size: `  0`(x86)/`  1`(x86_64)
    #[cfg_attr(feature = "serde_default", serde(default))]
    #[cfg_attr(feature = "json_schema", schemars(rename = "isAdditive"))]
    #[cfg_attr(feature = "serde", serde(rename = "isAdditive"))]
    pub m_isAdditive: bool,
    /// # C++ Info
    /// - name: `COMIndex`(ctype: `hkInt16`)
    /// - offset: `  0`(x86)/` 90`(x86_64)
    /// - type_size: `  0`(x86)/`  2`(x86_64)
    #[cfg_attr(feature = "serde_default", serde(default))]
    #[cfg_attr(feature = "serde", serde(borrow))]
    #[cfg_attr(feature = "json_schema", schemars(rename = "COMIndex"))]
    #[cfg_attr(feature = "serde", serde(rename = "COMIndex"))]
    pub m_COMIndex: I16<'a>,
    /// # C++ Info
    /// - name: `Bone0Index`(ctype: `hkInt16`)
    /// - offset: `  0`(x86)/` 92`(x86_64)
    /// - type_size: `  0`(x86)/`  2`(x86_64)
    #[cfg_attr(feature = "serde_default", serde(default))]
    #[cfg_attr(feature = "serde", serde(borrow))]
    #[cfg_attr(feature = "json_schema", schemars(rename = "Bone0Index"))]
    #[cfg_attr(feature = "serde", serde(rename = "Bone0Index"))]
    pub m_Bone0Index: I16<'a>,
    /// # C++ Info
    /// - name: `Bone0Translate`(ctype: `hkVector4`)
    /// - offset: `  0`(x86)/` 96`(x86_64)
    /// - type_size: `  0`(x86)/` 16`(x86_64)
    #[cfg_attr(feature = "serde_default", serde(default))]
    #[cfg_attr(feature = "json_schema", schemars(rename = "Bone0Translate"))]
    #[cfg_attr(feature = "serde", serde(rename = "Bone0Translate"))]
    pub m_Bone0Translate: Vector4,
    /// # C++ Info
    /// - name: `Bone0Rotate`(ctype: `hkQuaternion`)
    /// - offset: `  0`(x86)/`112`(x86_64)
    /// - type_size: `  0`(x86)/` 16`(x86_64)
    #[cfg_attr(feature = "serde_default", serde(default))]
    #[cfg_attr(feature = "json_schema", schemars(rename = "Bone0Rotate"))]
    #[cfg_attr(feature = "serde", serde(rename = "Bone0Rotate"))]
    pub m_Bone0Rotate: Quaternion,
    /// # C++ Info
    /// - name: `Bone1Index`(ctype: `hkInt16`)
    /// - offset: `  0`(x86)/`128`(x86_64)
    /// - type_size: `  0`(x86)/`  2`(x86_64)
    #[cfg_attr(feature = "serde_default", serde(default))]
    #[cfg_attr(feature = "serde", serde(borrow))]
    #[cfg_attr(feature = "json_schema", schemars(rename = "Bone1Index"))]
    #[cfg_attr(feature = "serde", serde(rename = "Bone1Index"))]
    pub m_Bone1Index: I16<'a>,
    /// # C++ Info
    /// - name: `Bone1Translate`(ctype: `hkVector4`)
    /// - offset: `  0`(x86)/`144`(x86_64)
    /// - type_size: `  0`(x86)/` 16`(x86_64)
    #[cfg_attr(feature = "serde_default", serde(default))]
    #[cfg_attr(feature = "json_schema", schemars(rename = "Bone1Translate"))]
    #[cfg_attr(feature = "serde", serde(rename = "Bone1Translate"))]
    pub m_Bone1Translate: Vector4,
    /// # C++ Info
    /// - name: `Bone1Rotate`(ctype: `hkQuaternion`)
    /// - offset: `  0`(x86)/`160`(x86_64)
    /// - type_size: `  0`(x86)/` 16`(x86_64)
    #[cfg_attr(feature = "serde_default", serde(default))]
    #[cfg_attr(feature = "json_schema", schemars(rename = "Bone1Rotate"))]
    #[cfg_attr(feature = "serde", serde(rename = "Bone1Rotate"))]
    pub m_Bone1Rotate: Quaternion,
    /// # C++ Info
    /// - name: `Bone2Index`(ctype: `hkInt16`)
    /// - offset: `  0`(x86)/`176`(x86_64)
    /// - type_size: `  0`(x86)/`  2`(x86_64)
    #[cfg_attr(feature = "serde_default", serde(default))]
    #[cfg_attr(feature = "serde", serde(borrow))]
    #[cfg_attr(feature = "json_schema", schemars(rename = "Bone2Index"))]
    #[cfg_attr(feature = "serde", serde(rename = "Bone2Index"))]
    pub m_Bone2Index: I16<'a>,
    /// # C++ Info
    /// - name: `RunSpeed`(ctype: `hkReal`)
    /// - offset: `  0`(x86)/`180`(x86_64)
    /// - type_size: `  0`(x86)/`  4`(x86_64)
    #[cfg_attr(feature = "serde_default", serde(default))]
    #[cfg_attr(feature = "json_schema", schemars(rename = "RunSpeed"))]
    #[cfg_attr(feature = "serde", serde(rename = "RunSpeed"))]
    pub m_RunSpeed: f32,
    /// # C++ Info
    /// - name: `WalkSpeed`(ctype: `hkReal`)
    /// - offset: `  0`(x86)/`184`(x86_64)
    /// - type_size: `  0`(x86)/`  4`(x86_64)
    #[cfg_attr(feature = "serde_default", serde(default))]
    #[cfg_attr(feature = "json_schema", schemars(rename = "WalkSpeed"))]
    #[cfg_attr(feature = "serde", serde(rename = "WalkSpeed"))]
    pub m_WalkSpeed: f32,
    /// # C++ Info
    /// - name: `MaxZRotationRun`(ctype: `hkReal`)
    /// - offset: `  0`(x86)/`188`(x86_64)
    /// - type_size: `  0`(x86)/`  4`(x86_64)
    #[cfg_attr(feature = "serde_default", serde(default))]
    #[cfg_attr(feature = "json_schema", schemars(rename = "MaxZRotationRun"))]
    #[cfg_attr(feature = "serde", serde(rename = "MaxZRotationRun"))]
    pub m_MaxZRotationRun: f32,
    /// # C++ Info
    /// - name: `MaxZRotationWalk`(ctype: `hkReal`)
    /// - offset: `  0`(x86)/`192`(x86_64)
    /// - type_size: `  0`(x86)/`  4`(x86_64)
    #[cfg_attr(feature = "serde_default", serde(default))]
    #[cfg_attr(feature = "json_schema", schemars(rename = "MaxZRotationWalk"))]
    #[cfg_attr(feature = "serde", serde(rename = "MaxZRotationWalk"))]
    pub m_MaxZRotationWalk: f32,
    /// # C++ Info
    /// - name: `MaxTurnDeltaRun`(ctype: `hkReal`)
    /// - offset: `  0`(x86)/`196`(x86_64)
    /// - type_size: `  0`(x86)/`  4`(x86_64)
    #[cfg_attr(feature = "serde_default", serde(default))]
    #[cfg_attr(feature = "json_schema", schemars(rename = "MaxTurnDeltaRun"))]
    #[cfg_attr(feature = "serde", serde(rename = "MaxTurnDeltaRun"))]
    pub m_MaxTurnDeltaRun: f32,
    /// # C++ Info
    /// - name: `MaxTurnDeltaWalk`(ctype: `hkReal`)
    /// - offset: `  0`(x86)/`200`(x86_64)
    /// - type_size: `  0`(x86)/`  4`(x86_64)
    #[cfg_attr(feature = "serde_default", serde(default))]
    #[cfg_attr(feature = "json_schema", schemars(rename = "MaxTurnDeltaWalk"))]
    #[cfg_attr(feature = "serde", serde(rename = "MaxTurnDeltaWalk"))]
    pub m_MaxTurnDeltaWalk: f32,
    /// # C++ Info
    /// - name: `IN_Speed`(ctype: `hkReal`)
    /// - offset: `  0`(x86)/`204`(x86_64)
    /// - type_size: `  0`(x86)/`  4`(x86_64)
    #[cfg_attr(feature = "serde_default", serde(default))]
    #[cfg_attr(feature = "json_schema", schemars(rename = "IN_Speed"))]
    #[cfg_attr(feature = "serde", serde(rename = "IN_Speed"))]
    pub m_IN_Speed: f32,
    /// # C++ Info
    /// - name: `SpringZ`(ctype: `struct BSSpringState`)
    /// - offset: `  0`(x86)/`208`(x86_64)
    /// - type_size: `  0`(x86)/` 20`(x86_64)
    #[cfg_attr(feature = "serde_default", serde(default))]
    #[cfg_attr(feature = "json_schema", schemars(rename = "SpringZ"))]
    #[cfg_attr(feature = "serde", serde(rename = "SpringZ"))]
    pub m_SpringZ: BSSpringState<'a>,
    /// # C++ Info
    /// - name: `parentOfBone0ToCOMIndices`(ctype: `hkArray<void>`)
    /// - offset: `  0`(x86)/`232`(x86_64)
    /// - type_size: `  0`(x86)/` 16`(x86_64)
    /// - flags: `SERIALIZE_IGNORED`
    #[cfg_attr(feature = "serde", serde(default))]
    #[cfg_attr(feature = "json_schema", schemars(rename = "parentOfBone0ToCOMIndices"))]
    #[cfg_attr(feature = "serde", serde(rename = "parentOfBone0ToCOMIndices"))]
    pub m_parentOfBone0ToCOMIndices: Vec<()>,
}
const _: () = {
    use havok_serde as _serde;
    impl<'a> _serde::HavokClass for BSReverseSpineTwistModifier<'a> {
        #[inline]
        fn name(&self) -> &'static str {
            "BSReverseSpineTwistModifier"
        }
        #[inline]
        fn signature(&self) -> _serde::__private::Signature {
            _serde::__private::Signature::new(0xfc8adeda)
        }
        #[allow(clippy::let_and_return, clippy::vec_init_then_push)]
        fn deps_indexes(&self) -> Vec<&Pointer<'_>> {
            let mut v = Vec::new();
            v.push(&self.parent.parent.parent.m_variableBindingSet);
            v.extend(self.parent.parent.parent.m_cachedBindables.iter());
            v.push(&self.parent.parent.m_nodeInfo);
            v.extend(self.m_SpringZ.deps_indexes());
            v
        }
    }
    impl<'a> _serde::Serialize for BSReverseSpineTwistModifier<'a> {
        fn serialize<S>(&self, __serializer: S) -> Result<S::Ok, S::Error>
        where
            S: _serde::ser::Serializer,
        {
            let class_meta = self
                .__ptr
                .as_ref()
                .map(|name| (name, _serde::__private::Signature::new(0xfc8adeda)));
            let mut serializer = __serializer
                .serialize_struct(
                    "BSReverseSpineTwistModifier",
                    class_meta,
                    (0u64, 256u64),
                )?;
            serializer.pad_field([0u8; 0usize].as_slice(), [0u8; 8usize].as_slice())?;
            serializer
                .skip_field(
                    "memSizeAndRefCount",
                    &self.parent.parent.parent.parent.m_memSizeAndRefCount,
                )?;
            serializer.pad_field([0u8; 0usize].as_slice(), [0u8; 4usize].as_slice())?;
            serializer
                .serialize_field(
                    "variableBindingSet",
                    &self.parent.parent.parent.m_variableBindingSet,
                )?;
            serializer
                .skip_array_field(
                    "cachedBindables",
                    &self.parent.parent.parent.m_cachedBindables,
                    TypeSize::NonPtr,
                )?;
            serializer
                .skip_field(
                    "areBindablesCached",
                    &self.parent.parent.parent.m_areBindablesCached,
                )?;
            serializer
                .skip_field(
                    "hasEnableChanged",
                    &self.parent.parent.parent.m_hasEnableChanged,
                )?;
            serializer.pad_field([0u8; 0usize].as_slice(), [0u8; 6usize].as_slice())?;
            serializer.serialize_field("userData", &self.parent.parent.m_userData)?;
            serializer.serialize_field("name", &self.parent.parent.m_name)?;
            serializer.skip_field("id", &self.parent.parent.m_id)?;
            serializer.skip_field("cloneState", &self.parent.parent.m_cloneState)?;
            serializer.skip_field("type", &self.parent.parent.m_type)?;
            serializer.pad_field([0u8; 0usize].as_slice(), [0u8; 4usize].as_slice())?;
            serializer.skip_field("nodeInfo", &self.parent.parent.m_nodeInfo)?;
            serializer.serialize_field("enable", &self.parent.m_enable)?;
            serializer
                .skip_fixed_array_field(
                    "padModifier",
                    self.parent.m_padModifier.as_slice(),
                    TypeSize::NonPtr,
                )?;
            serializer.pad_field([0u8; 0usize].as_slice(), [0u8; 4usize].as_slice())?;
            serializer.serialize_field("isAdditive", &self.m_isAdditive)?;
            serializer.pad_field([0u8; 0usize].as_slice(), [0u8; 1usize].as_slice())?;
            serializer.serialize_field("COMIndex", &self.m_COMIndex)?;
            serializer.serialize_field("Bone0Index", &self.m_Bone0Index)?;
            serializer.pad_field([0u8; 0usize].as_slice(), [0u8; 2usize].as_slice())?;
            serializer.serialize_field("Bone0Translate", &self.m_Bone0Translate)?;
            serializer.serialize_field("Bone0Rotate", &self.m_Bone0Rotate)?;
            serializer.serialize_field("Bone1Index", &self.m_Bone1Index)?;
            serializer.pad_field([0u8; 0usize].as_slice(), [0u8; 14usize].as_slice())?;
            serializer.serialize_field("Bone1Translate", &self.m_Bone1Translate)?;
            serializer.serialize_field("Bone1Rotate", &self.m_Bone1Rotate)?;
            serializer.serialize_field("Bone2Index", &self.m_Bone2Index)?;
            serializer.pad_field([0u8; 0usize].as_slice(), [0u8; 2usize].as_slice())?;
            serializer.serialize_field("RunSpeed", &self.m_RunSpeed)?;
            serializer.serialize_field("WalkSpeed", &self.m_WalkSpeed)?;
            serializer.serialize_field("MaxZRotationRun", &self.m_MaxZRotationRun)?;
            serializer.serialize_field("MaxZRotationWalk", &self.m_MaxZRotationWalk)?;
            serializer.serialize_field("MaxTurnDeltaRun", &self.m_MaxTurnDeltaRun)?;
            serializer.serialize_field("MaxTurnDeltaWalk", &self.m_MaxTurnDeltaWalk)?;
            serializer.serialize_field("IN_Speed", &self.m_IN_Speed)?;
            serializer.serialize_field("SpringZ", &self.m_SpringZ)?;
            serializer.pad_field([0u8; 0usize].as_slice(), [0u8; 4usize].as_slice())?;
            serializer
                .skip_array_field(
                    "parentOfBone0ToCOMIndices",
                    &self.m_parentOfBone0ToCOMIndices,
                    TypeSize::NonPtr,
                )?;
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
    impl<'de> _serde::Deserialize<'de> for BSReverseSpineTwistModifier<'de> {
        fn deserialize<__D>(deserializer: __D) -> core::result::Result<Self, __D::Error>
        where
            __D: _serde::Deserializer<'de>,
        {
            #[allow(non_camel_case_types)]
            enum __Field {
                m_variableBindingSet,
                m_userData,
                m_name,
                m_enable,
                m_isAdditive,
                m_COMIndex,
                m_Bone0Index,
                m_Bone0Translate,
                m_Bone0Rotate,
                m_Bone1Index,
                m_Bone1Translate,
                m_Bone1Rotate,
                m_Bone2Index,
                m_RunSpeed,
                m_WalkSpeed,
                m_MaxZRotationRun,
                m_MaxZRotationWalk,
                m_MaxTurnDeltaRun,
                m_MaxTurnDeltaWalk,
                m_IN_Speed,
                m_SpringZ,
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
                        "enable" => Ok(__Field::m_enable),
                        "isAdditive" => Ok(__Field::m_isAdditive),
                        "COMIndex" => Ok(__Field::m_COMIndex),
                        "Bone0Index" => Ok(__Field::m_Bone0Index),
                        "Bone0Translate" => Ok(__Field::m_Bone0Translate),
                        "Bone0Rotate" => Ok(__Field::m_Bone0Rotate),
                        "Bone1Index" => Ok(__Field::m_Bone1Index),
                        "Bone1Translate" => Ok(__Field::m_Bone1Translate),
                        "Bone1Rotate" => Ok(__Field::m_Bone1Rotate),
                        "Bone2Index" => Ok(__Field::m_Bone2Index),
                        "RunSpeed" => Ok(__Field::m_RunSpeed),
                        "WalkSpeed" => Ok(__Field::m_WalkSpeed),
                        "MaxZRotationRun" => Ok(__Field::m_MaxZRotationRun),
                        "MaxZRotationWalk" => Ok(__Field::m_MaxZRotationWalk),
                        "MaxTurnDeltaRun" => Ok(__Field::m_MaxTurnDeltaRun),
                        "MaxTurnDeltaWalk" => Ok(__Field::m_MaxTurnDeltaWalk),
                        "IN_Speed" => Ok(__Field::m_IN_Speed),
                        "SpringZ" => Ok(__Field::m_SpringZ),
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
            struct __BSReverseSpineTwistModifierVisitor<'de> {
                marker: _serde::__private::PhantomData<BSReverseSpineTwistModifier<'de>>,
                lifetime: _serde::__private::PhantomData<&'de ()>,
            }
            #[allow(clippy::match_single_binding)]
            #[allow(clippy::reversed_empty_ranges)]
            #[allow(clippy::single_match)]
            impl<'de> _serde::de::Visitor<'de>
            for __BSReverseSpineTwistModifierVisitor<'de> {
                type Value = BSReverseSpineTwistModifier<'de>;
                fn expecting(
                    &self,
                    __formatter: &mut core::fmt::Formatter,
                ) -> core::fmt::Result {
                    core::fmt::Formatter::write_str(
                        __formatter,
                        "struct BSReverseSpineTwistModifier",
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
                    let mut m_isAdditive: _serde::__private::Option<bool> = _serde::__private::None;
                    let mut m_COMIndex: _serde::__private::Option<I16<'de>> = _serde::__private::None;
                    let mut m_Bone0Index: _serde::__private::Option<I16<'de>> = _serde::__private::None;
                    let mut m_Bone0Translate: _serde::__private::Option<Vector4> = _serde::__private::None;
                    let mut m_Bone0Rotate: _serde::__private::Option<Quaternion> = _serde::__private::None;
                    let mut m_Bone1Index: _serde::__private::Option<I16<'de>> = _serde::__private::None;
                    let mut m_Bone1Translate: _serde::__private::Option<Vector4> = _serde::__private::None;
                    let mut m_Bone1Rotate: _serde::__private::Option<Quaternion> = _serde::__private::None;
                    let mut m_Bone2Index: _serde::__private::Option<I16<'de>> = _serde::__private::None;
                    let mut m_RunSpeed: _serde::__private::Option<f32> = _serde::__private::None;
                    let mut m_WalkSpeed: _serde::__private::Option<f32> = _serde::__private::None;
                    let mut m_MaxZRotationRun: _serde::__private::Option<f32> = _serde::__private::None;
                    let mut m_MaxZRotationWalk: _serde::__private::Option<f32> = _serde::__private::None;
                    let mut m_MaxTurnDeltaRun: _serde::__private::Option<f32> = _serde::__private::None;
                    let mut m_MaxTurnDeltaWalk: _serde::__private::Option<f32> = _serde::__private::None;
                    let mut m_IN_Speed: _serde::__private::Option<f32> = _serde::__private::None;
                    let mut m_SpringZ: _serde::__private::Option<BSSpringState> = _serde::__private::None;
                    let mut m_parentOfBone0ToCOMIndices: _serde::__private::Option<
                        Vec<()>,
                    > = _serde::__private::None;
                    for i in 0..18usize {
                        match i {
                            0usize => {
                                if _serde::__private::Option::is_some(&m_isAdditive) {
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "isAdditive",
                                        ),
                                    );
                                }
                                m_isAdditive = _serde::__private::Some(
                                    match __A::next_value::<bool>(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            1usize => {
                                if _serde::__private::Option::is_some(&m_COMIndex) {
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "COMIndex",
                                        ),
                                    );
                                }
                                __A::pad(&mut __map, 0usize, 1usize)?;
                                m_COMIndex = _serde::__private::Some(
                                    match __A::next_value::<I16<'de>>(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            2usize => {
                                if _serde::__private::Option::is_some(&m_Bone0Index) {
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "Bone0Index",
                                        ),
                                    );
                                }
                                m_Bone0Index = _serde::__private::Some(
                                    match __A::next_value::<I16<'de>>(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            3usize => {
                                if _serde::__private::Option::is_some(&m_Bone0Translate) {
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "Bone0Translate",
                                        ),
                                    );
                                }
                                __A::pad(&mut __map, 0usize, 2usize)?;
                                m_Bone0Translate = _serde::__private::Some(
                                    match __A::next_value::<Vector4>(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            4usize => {
                                if _serde::__private::Option::is_some(&m_Bone0Rotate) {
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "Bone0Rotate",
                                        ),
                                    );
                                }
                                m_Bone0Rotate = _serde::__private::Some(
                                    match __A::next_value::<Quaternion>(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            5usize => {
                                if _serde::__private::Option::is_some(&m_Bone1Index) {
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "Bone1Index",
                                        ),
                                    );
                                }
                                m_Bone1Index = _serde::__private::Some(
                                    match __A::next_value::<I16<'de>>(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            6usize => {
                                if _serde::__private::Option::is_some(&m_Bone1Translate) {
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "Bone1Translate",
                                        ),
                                    );
                                }
                                __A::pad(&mut __map, 0usize, 14usize)?;
                                m_Bone1Translate = _serde::__private::Some(
                                    match __A::next_value::<Vector4>(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            7usize => {
                                if _serde::__private::Option::is_some(&m_Bone1Rotate) {
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "Bone1Rotate",
                                        ),
                                    );
                                }
                                m_Bone1Rotate = _serde::__private::Some(
                                    match __A::next_value::<Quaternion>(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            8usize => {
                                if _serde::__private::Option::is_some(&m_Bone2Index) {
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "Bone2Index",
                                        ),
                                    );
                                }
                                m_Bone2Index = _serde::__private::Some(
                                    match __A::next_value::<I16<'de>>(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            9usize => {
                                if _serde::__private::Option::is_some(&m_RunSpeed) {
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "RunSpeed",
                                        ),
                                    );
                                }
                                __A::pad(&mut __map, 0usize, 2usize)?;
                                m_RunSpeed = _serde::__private::Some(
                                    match __A::next_value::<f32>(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            10usize => {
                                if _serde::__private::Option::is_some(&m_WalkSpeed) {
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "WalkSpeed",
                                        ),
                                    );
                                }
                                m_WalkSpeed = _serde::__private::Some(
                                    match __A::next_value::<f32>(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            11usize => {
                                if _serde::__private::Option::is_some(&m_MaxZRotationRun) {
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "MaxZRotationRun",
                                        ),
                                    );
                                }
                                m_MaxZRotationRun = _serde::__private::Some(
                                    match __A::next_value::<f32>(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            12usize => {
                                if _serde::__private::Option::is_some(&m_MaxZRotationWalk) {
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "MaxZRotationWalk",
                                        ),
                                    );
                                }
                                m_MaxZRotationWalk = _serde::__private::Some(
                                    match __A::next_value::<f32>(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            13usize => {
                                if _serde::__private::Option::is_some(&m_MaxTurnDeltaRun) {
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "MaxTurnDeltaRun",
                                        ),
                                    );
                                }
                                m_MaxTurnDeltaRun = _serde::__private::Some(
                                    match __A::next_value::<f32>(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            14usize => {
                                if _serde::__private::Option::is_some(&m_MaxTurnDeltaWalk) {
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "MaxTurnDeltaWalk",
                                        ),
                                    );
                                }
                                m_MaxTurnDeltaWalk = _serde::__private::Some(
                                    match __A::next_value::<f32>(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            15usize => {
                                if _serde::__private::Option::is_some(&m_IN_Speed) {
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "IN_Speed",
                                        ),
                                    );
                                }
                                m_IN_Speed = _serde::__private::Some(
                                    match __A::next_value::<f32>(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            16usize => {
                                if _serde::__private::Option::is_some(&m_SpringZ) {
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "SpringZ",
                                        ),
                                    );
                                }
                                m_SpringZ = _serde::__private::Some(
                                    match __A::next_value::<BSSpringState>(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            17usize => {
                                if _serde::__private::Option::is_some(
                                    &m_parentOfBone0ToCOMIndices,
                                ) {
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "parentOfBone0ToCOMIndices",
                                        ),
                                    );
                                }
                                __A::pad(&mut __map, 0usize, 4usize)?;
                                m_parentOfBone0ToCOMIndices = _serde::__private::Some(
                                    match __A::next_value::<Vec<()>>(&mut __map) {
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
                    let m_isAdditive = match m_isAdditive {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "isAdditive",
                                ),
                            );
                        }
                    };
                    let m_COMIndex = match m_COMIndex {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field("COMIndex"),
                            );
                        }
                    };
                    let m_Bone0Index = match m_Bone0Index {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "Bone0Index",
                                ),
                            );
                        }
                    };
                    let m_Bone0Translate = match m_Bone0Translate {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "Bone0Translate",
                                ),
                            );
                        }
                    };
                    let m_Bone0Rotate = match m_Bone0Rotate {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "Bone0Rotate",
                                ),
                            );
                        }
                    };
                    let m_Bone1Index = match m_Bone1Index {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "Bone1Index",
                                ),
                            );
                        }
                    };
                    let m_Bone1Translate = match m_Bone1Translate {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "Bone1Translate",
                                ),
                            );
                        }
                    };
                    let m_Bone1Rotate = match m_Bone1Rotate {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "Bone1Rotate",
                                ),
                            );
                        }
                    };
                    let m_Bone2Index = match m_Bone2Index {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "Bone2Index",
                                ),
                            );
                        }
                    };
                    let m_RunSpeed = match m_RunSpeed {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field("RunSpeed"),
                            );
                        }
                    };
                    let m_WalkSpeed = match m_WalkSpeed {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "WalkSpeed",
                                ),
                            );
                        }
                    };
                    let m_MaxZRotationRun = match m_MaxZRotationRun {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "MaxZRotationRun",
                                ),
                            );
                        }
                    };
                    let m_MaxZRotationWalk = match m_MaxZRotationWalk {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "MaxZRotationWalk",
                                ),
                            );
                        }
                    };
                    let m_MaxTurnDeltaRun = match m_MaxTurnDeltaRun {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "MaxTurnDeltaRun",
                                ),
                            );
                        }
                    };
                    let m_MaxTurnDeltaWalk = match m_MaxTurnDeltaWalk {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "MaxTurnDeltaWalk",
                                ),
                            );
                        }
                    };
                    let m_IN_Speed = match m_IN_Speed {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field("IN_Speed"),
                            );
                        }
                    };
                    let m_SpringZ = match m_SpringZ {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field("SpringZ"),
                            );
                        }
                    };
                    let m_parentOfBone0ToCOMIndices = match m_parentOfBone0ToCOMIndices {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "parentOfBone0ToCOMIndices",
                                ),
                            );
                        }
                    };
                    _serde::__private::Ok(BSReverseSpineTwistModifier {
                        __ptr,
                        parent,
                        m_isAdditive,
                        m_COMIndex,
                        m_Bone0Index,
                        m_Bone0Translate,
                        m_Bone0Rotate,
                        m_Bone1Index,
                        m_Bone1Translate,
                        m_Bone1Rotate,
                        m_Bone2Index,
                        m_RunSpeed,
                        m_WalkSpeed,
                        m_MaxZRotationRun,
                        m_MaxZRotationWalk,
                        m_MaxTurnDeltaRun,
                        m_MaxTurnDeltaWalk,
                        m_IN_Speed,
                        m_SpringZ,
                        m_parentOfBone0ToCOMIndices,
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
                    let mut m_enable: _serde::__private::Option<bool> = _serde::__private::None;
                    let mut m_isAdditive: _serde::__private::Option<bool> = _serde::__private::None;
                    let mut m_COMIndex: _serde::__private::Option<I16<'de>> = _serde::__private::None;
                    let mut m_Bone0Index: _serde::__private::Option<I16<'de>> = _serde::__private::None;
                    let mut m_Bone0Translate: _serde::__private::Option<Vector4> = _serde::__private::None;
                    let mut m_Bone0Rotate: _serde::__private::Option<Quaternion> = _serde::__private::None;
                    let mut m_Bone1Index: _serde::__private::Option<I16<'de>> = _serde::__private::None;
                    let mut m_Bone1Translate: _serde::__private::Option<Vector4> = _serde::__private::None;
                    let mut m_Bone1Rotate: _serde::__private::Option<Quaternion> = _serde::__private::None;
                    let mut m_Bone2Index: _serde::__private::Option<I16<'de>> = _serde::__private::None;
                    let mut m_RunSpeed: _serde::__private::Option<f32> = _serde::__private::None;
                    let mut m_WalkSpeed: _serde::__private::Option<f32> = _serde::__private::None;
                    let mut m_MaxZRotationRun: _serde::__private::Option<f32> = _serde::__private::None;
                    let mut m_MaxZRotationWalk: _serde::__private::Option<f32> = _serde::__private::None;
                    let mut m_MaxTurnDeltaRun: _serde::__private::Option<f32> = _serde::__private::None;
                    let mut m_MaxTurnDeltaWalk: _serde::__private::Option<f32> = _serde::__private::None;
                    let mut m_IN_Speed: _serde::__private::Option<f32> = _serde::__private::None;
                    let mut m_SpringZ: _serde::__private::Option<BSSpringState> = _serde::__private::None;
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
                            __Field::m_enable => {
                                #[cfg(
                                    any(feature = "strict", feature = "ignore_duplicates")
                                )]
                                if _serde::__private::Option::is_some(&m_enable) {
                                    #[cfg(feature = "ignore_duplicates")]
                                    {
                                        __A::skip_value(&mut __map)?;
                                        continue;
                                    }
                                    #[cfg(feature = "strict")]
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field("enable"),
                                    );
                                }
                                m_enable = _serde::__private::Some(
                                    match __A::next_value::<bool>(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            __Field::m_isAdditive => {
                                #[cfg(
                                    any(feature = "strict", feature = "ignore_duplicates")
                                )]
                                if _serde::__private::Option::is_some(&m_isAdditive) {
                                    #[cfg(feature = "ignore_duplicates")]
                                    {
                                        __A::skip_value(&mut __map)?;
                                        continue;
                                    }
                                    #[cfg(feature = "strict")]
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "isAdditive",
                                        ),
                                    );
                                }
                                m_isAdditive = _serde::__private::Some(
                                    match __A::next_value::<bool>(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            __Field::m_COMIndex => {
                                #[cfg(
                                    any(feature = "strict", feature = "ignore_duplicates")
                                )]
                                if _serde::__private::Option::is_some(&m_COMIndex) {
                                    #[cfg(feature = "ignore_duplicates")]
                                    {
                                        __A::skip_value(&mut __map)?;
                                        continue;
                                    }
                                    #[cfg(feature = "strict")]
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "COMIndex",
                                        ),
                                    );
                                }
                                m_COMIndex = _serde::__private::Some(
                                    match __A::next_value::<I16<'de>>(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            __Field::m_Bone0Index => {
                                #[cfg(
                                    any(feature = "strict", feature = "ignore_duplicates")
                                )]
                                if _serde::__private::Option::is_some(&m_Bone0Index) {
                                    #[cfg(feature = "ignore_duplicates")]
                                    {
                                        __A::skip_value(&mut __map)?;
                                        continue;
                                    }
                                    #[cfg(feature = "strict")]
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "Bone0Index",
                                        ),
                                    );
                                }
                                m_Bone0Index = _serde::__private::Some(
                                    match __A::next_value::<I16<'de>>(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            __Field::m_Bone0Translate => {
                                #[cfg(
                                    any(feature = "strict", feature = "ignore_duplicates")
                                )]
                                if _serde::__private::Option::is_some(&m_Bone0Translate) {
                                    #[cfg(feature = "ignore_duplicates")]
                                    {
                                        __A::skip_value(&mut __map)?;
                                        continue;
                                    }
                                    #[cfg(feature = "strict")]
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "Bone0Translate",
                                        ),
                                    );
                                }
                                m_Bone0Translate = _serde::__private::Some(
                                    match __A::next_value::<Vector4>(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            __Field::m_Bone0Rotate => {
                                #[cfg(
                                    any(feature = "strict", feature = "ignore_duplicates")
                                )]
                                if _serde::__private::Option::is_some(&m_Bone0Rotate) {
                                    #[cfg(feature = "ignore_duplicates")]
                                    {
                                        __A::skip_value(&mut __map)?;
                                        continue;
                                    }
                                    #[cfg(feature = "strict")]
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "Bone0Rotate",
                                        ),
                                    );
                                }
                                m_Bone0Rotate = _serde::__private::Some(
                                    match __A::next_value::<Quaternion>(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            __Field::m_Bone1Index => {
                                #[cfg(
                                    any(feature = "strict", feature = "ignore_duplicates")
                                )]
                                if _serde::__private::Option::is_some(&m_Bone1Index) {
                                    #[cfg(feature = "ignore_duplicates")]
                                    {
                                        __A::skip_value(&mut __map)?;
                                        continue;
                                    }
                                    #[cfg(feature = "strict")]
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "Bone1Index",
                                        ),
                                    );
                                }
                                m_Bone1Index = _serde::__private::Some(
                                    match __A::next_value::<I16<'de>>(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            __Field::m_Bone1Translate => {
                                #[cfg(
                                    any(feature = "strict", feature = "ignore_duplicates")
                                )]
                                if _serde::__private::Option::is_some(&m_Bone1Translate) {
                                    #[cfg(feature = "ignore_duplicates")]
                                    {
                                        __A::skip_value(&mut __map)?;
                                        continue;
                                    }
                                    #[cfg(feature = "strict")]
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "Bone1Translate",
                                        ),
                                    );
                                }
                                m_Bone1Translate = _serde::__private::Some(
                                    match __A::next_value::<Vector4>(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            __Field::m_Bone1Rotate => {
                                #[cfg(
                                    any(feature = "strict", feature = "ignore_duplicates")
                                )]
                                if _serde::__private::Option::is_some(&m_Bone1Rotate) {
                                    #[cfg(feature = "ignore_duplicates")]
                                    {
                                        __A::skip_value(&mut __map)?;
                                        continue;
                                    }
                                    #[cfg(feature = "strict")]
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "Bone1Rotate",
                                        ),
                                    );
                                }
                                m_Bone1Rotate = _serde::__private::Some(
                                    match __A::next_value::<Quaternion>(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            __Field::m_Bone2Index => {
                                #[cfg(
                                    any(feature = "strict", feature = "ignore_duplicates")
                                )]
                                if _serde::__private::Option::is_some(&m_Bone2Index) {
                                    #[cfg(feature = "ignore_duplicates")]
                                    {
                                        __A::skip_value(&mut __map)?;
                                        continue;
                                    }
                                    #[cfg(feature = "strict")]
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "Bone2Index",
                                        ),
                                    );
                                }
                                m_Bone2Index = _serde::__private::Some(
                                    match __A::next_value::<I16<'de>>(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            __Field::m_RunSpeed => {
                                #[cfg(
                                    any(feature = "strict", feature = "ignore_duplicates")
                                )]
                                if _serde::__private::Option::is_some(&m_RunSpeed) {
                                    #[cfg(feature = "ignore_duplicates")]
                                    {
                                        __A::skip_value(&mut __map)?;
                                        continue;
                                    }
                                    #[cfg(feature = "strict")]
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "RunSpeed",
                                        ),
                                    );
                                }
                                m_RunSpeed = _serde::__private::Some(
                                    match __A::next_value::<f32>(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            __Field::m_WalkSpeed => {
                                #[cfg(
                                    any(feature = "strict", feature = "ignore_duplicates")
                                )]
                                if _serde::__private::Option::is_some(&m_WalkSpeed) {
                                    #[cfg(feature = "ignore_duplicates")]
                                    {
                                        __A::skip_value(&mut __map)?;
                                        continue;
                                    }
                                    #[cfg(feature = "strict")]
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "WalkSpeed",
                                        ),
                                    );
                                }
                                m_WalkSpeed = _serde::__private::Some(
                                    match __A::next_value::<f32>(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            __Field::m_MaxZRotationRun => {
                                #[cfg(
                                    any(feature = "strict", feature = "ignore_duplicates")
                                )]
                                if _serde::__private::Option::is_some(&m_MaxZRotationRun) {
                                    #[cfg(feature = "ignore_duplicates")]
                                    {
                                        __A::skip_value(&mut __map)?;
                                        continue;
                                    }
                                    #[cfg(feature = "strict")]
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "MaxZRotationRun",
                                        ),
                                    );
                                }
                                m_MaxZRotationRun = _serde::__private::Some(
                                    match __A::next_value::<f32>(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            __Field::m_MaxZRotationWalk => {
                                #[cfg(
                                    any(feature = "strict", feature = "ignore_duplicates")
                                )]
                                if _serde::__private::Option::is_some(&m_MaxZRotationWalk) {
                                    #[cfg(feature = "ignore_duplicates")]
                                    {
                                        __A::skip_value(&mut __map)?;
                                        continue;
                                    }
                                    #[cfg(feature = "strict")]
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "MaxZRotationWalk",
                                        ),
                                    );
                                }
                                m_MaxZRotationWalk = _serde::__private::Some(
                                    match __A::next_value::<f32>(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            __Field::m_MaxTurnDeltaRun => {
                                #[cfg(
                                    any(feature = "strict", feature = "ignore_duplicates")
                                )]
                                if _serde::__private::Option::is_some(&m_MaxTurnDeltaRun) {
                                    #[cfg(feature = "ignore_duplicates")]
                                    {
                                        __A::skip_value(&mut __map)?;
                                        continue;
                                    }
                                    #[cfg(feature = "strict")]
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "MaxTurnDeltaRun",
                                        ),
                                    );
                                }
                                m_MaxTurnDeltaRun = _serde::__private::Some(
                                    match __A::next_value::<f32>(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            __Field::m_MaxTurnDeltaWalk => {
                                #[cfg(
                                    any(feature = "strict", feature = "ignore_duplicates")
                                )]
                                if _serde::__private::Option::is_some(&m_MaxTurnDeltaWalk) {
                                    #[cfg(feature = "ignore_duplicates")]
                                    {
                                        __A::skip_value(&mut __map)?;
                                        continue;
                                    }
                                    #[cfg(feature = "strict")]
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "MaxTurnDeltaWalk",
                                        ),
                                    );
                                }
                                m_MaxTurnDeltaWalk = _serde::__private::Some(
                                    match __A::next_value::<f32>(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            __Field::m_IN_Speed => {
                                #[cfg(
                                    any(feature = "strict", feature = "ignore_duplicates")
                                )]
                                if _serde::__private::Option::is_some(&m_IN_Speed) {
                                    #[cfg(feature = "ignore_duplicates")]
                                    {
                                        __A::skip_value(&mut __map)?;
                                        continue;
                                    }
                                    #[cfg(feature = "strict")]
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "IN_Speed",
                                        ),
                                    );
                                }
                                m_IN_Speed = _serde::__private::Some(
                                    match __A::next_value::<f32>(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            __Field::m_SpringZ => {
                                #[cfg(
                                    any(feature = "strict", feature = "ignore_duplicates")
                                )]
                                if _serde::__private::Option::is_some(&m_SpringZ) {
                                    #[cfg(feature = "ignore_duplicates")]
                                    {
                                        __A::skip_value(&mut __map)?;
                                        continue;
                                    }
                                    #[cfg(feature = "strict")]
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "SpringZ",
                                        ),
                                    );
                                }
                                m_SpringZ = _serde::__private::Some(
                                    match __A::next_value::<BSSpringState>(&mut __map) {
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
                    let m_enable = match m_enable {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            #[cfg(feature = "strict")]
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field("enable"),
                            );
                            #[cfg(not(feature = "strict"))] Default::default()
                        }
                    };
                    let m_isAdditive = match m_isAdditive {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            #[cfg(feature = "strict")]
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "isAdditive",
                                ),
                            );
                            #[cfg(not(feature = "strict"))] Default::default()
                        }
                    };
                    let m_COMIndex = match m_COMIndex {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            #[cfg(feature = "strict")]
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field("COMIndex"),
                            );
                            #[cfg(not(feature = "strict"))] Default::default()
                        }
                    };
                    let m_Bone0Index = match m_Bone0Index {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            #[cfg(feature = "strict")]
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "Bone0Index",
                                ),
                            );
                            #[cfg(not(feature = "strict"))] Default::default()
                        }
                    };
                    let m_Bone0Translate = match m_Bone0Translate {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            #[cfg(feature = "strict")]
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "Bone0Translate",
                                ),
                            );
                            #[cfg(not(feature = "strict"))] Default::default()
                        }
                    };
                    let m_Bone0Rotate = match m_Bone0Rotate {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            #[cfg(feature = "strict")]
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "Bone0Rotate",
                                ),
                            );
                            #[cfg(not(feature = "strict"))] Default::default()
                        }
                    };
                    let m_Bone1Index = match m_Bone1Index {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            #[cfg(feature = "strict")]
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "Bone1Index",
                                ),
                            );
                            #[cfg(not(feature = "strict"))] Default::default()
                        }
                    };
                    let m_Bone1Translate = match m_Bone1Translate {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            #[cfg(feature = "strict")]
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "Bone1Translate",
                                ),
                            );
                            #[cfg(not(feature = "strict"))] Default::default()
                        }
                    };
                    let m_Bone1Rotate = match m_Bone1Rotate {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            #[cfg(feature = "strict")]
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "Bone1Rotate",
                                ),
                            );
                            #[cfg(not(feature = "strict"))] Default::default()
                        }
                    };
                    let m_Bone2Index = match m_Bone2Index {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            #[cfg(feature = "strict")]
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "Bone2Index",
                                ),
                            );
                            #[cfg(not(feature = "strict"))] Default::default()
                        }
                    };
                    let m_RunSpeed = match m_RunSpeed {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            #[cfg(feature = "strict")]
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field("RunSpeed"),
                            );
                            #[cfg(not(feature = "strict"))] Default::default()
                        }
                    };
                    let m_WalkSpeed = match m_WalkSpeed {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            #[cfg(feature = "strict")]
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "WalkSpeed",
                                ),
                            );
                            #[cfg(not(feature = "strict"))] Default::default()
                        }
                    };
                    let m_MaxZRotationRun = match m_MaxZRotationRun {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            #[cfg(feature = "strict")]
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "MaxZRotationRun",
                                ),
                            );
                            #[cfg(not(feature = "strict"))] Default::default()
                        }
                    };
                    let m_MaxZRotationWalk = match m_MaxZRotationWalk {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            #[cfg(feature = "strict")]
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "MaxZRotationWalk",
                                ),
                            );
                            #[cfg(not(feature = "strict"))] Default::default()
                        }
                    };
                    let m_MaxTurnDeltaRun = match m_MaxTurnDeltaRun {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            #[cfg(feature = "strict")]
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "MaxTurnDeltaRun",
                                ),
                            );
                            #[cfg(not(feature = "strict"))] Default::default()
                        }
                    };
                    let m_MaxTurnDeltaWalk = match m_MaxTurnDeltaWalk {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            #[cfg(feature = "strict")]
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "MaxTurnDeltaWalk",
                                ),
                            );
                            #[cfg(not(feature = "strict"))] Default::default()
                        }
                    };
                    let m_IN_Speed = match m_IN_Speed {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            #[cfg(feature = "strict")]
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field("IN_Speed"),
                            );
                            #[cfg(not(feature = "strict"))] Default::default()
                        }
                    };
                    let m_SpringZ = match m_SpringZ {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            #[cfg(feature = "strict")]
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field("SpringZ"),
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
                    let parent = hkbModifier {
                        __ptr: __ptr.clone(),
                        parent,
                        m_enable,
                        ..Default::default()
                    };
                    let __ptr = __A::class_ptr(&mut __map);
                    _serde::__private::Ok(BSReverseSpineTwistModifier {
                        __ptr: __ptr.clone(),
                        parent,
                        m_isAdditive,
                        m_COMIndex,
                        m_Bone0Index,
                        m_Bone0Translate,
                        m_Bone0Rotate,
                        m_Bone1Index,
                        m_Bone1Translate,
                        m_Bone1Rotate,
                        m_Bone2Index,
                        m_RunSpeed,
                        m_WalkSpeed,
                        m_MaxZRotationRun,
                        m_MaxZRotationWalk,
                        m_MaxTurnDeltaRun,
                        m_MaxTurnDeltaWalk,
                        m_IN_Speed,
                        m_SpringZ,
                        ..Default::default()
                    })
                }
            }
            const FIELDS: &[&str] = &[
                "isAdditive",
                "COMIndex",
                "Bone0Index",
                "Bone0Translate",
                "Bone0Rotate",
                "Bone1Index",
                "Bone1Translate",
                "Bone1Rotate",
                "Bone2Index",
                "RunSpeed",
                "WalkSpeed",
                "MaxZRotationRun",
                "MaxZRotationWalk",
                "MaxTurnDeltaRun",
                "MaxTurnDeltaWalk",
                "IN_Speed",
                "SpringZ",
                "parentOfBone0ToCOMIndices",
            ];
            _serde::Deserializer::deserialize_struct(
                deserializer,
                "BSReverseSpineTwistModifier",
                FIELDS,
                __BSReverseSpineTwistModifierVisitor {
                    marker: _serde::__private::PhantomData::<
                        BSReverseSpineTwistModifier,
                    >,
                    lifetime: _serde::__private::PhantomData,
                },
            )
        }
    }
};
