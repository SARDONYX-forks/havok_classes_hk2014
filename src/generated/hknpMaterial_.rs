use super::class_requires::*;
use super::*;
/// # C++ Info
/// - name: `hknpMaterial`
/// - version: `1`
/// - signature: `0xb7c5f24e`
/// - size: `  0`(x86)/` 80`(x86_64)
/// -  vtable: `false`
#[allow(non_upper_case_globals, non_snake_case)]
#[cfg_attr(feature = "json_schema", derive(schemars::JsonSchema))]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(educe::Educe)]
#[educe(Debug, Clone, Default, PartialEq)]
pub struct hknpMaterial<'a> {
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
    /// # C++ Info
    /// - name: `name`(ctype: `hkStringPtr`)
    /// - offset: `  0`(x86)/`  0`(x86_64)
    /// - type_size: `  0`(x86)/`  8`(x86_64)
    /// - flags: `ALIGN_16`
    #[cfg_attr(feature = "serde_default", serde(default))]
    #[cfg_attr(feature = "serde", serde(borrow))]
    #[cfg_attr(feature = "json_schema", schemars(rename = "name"))]
    #[cfg_attr(feature = "serde", serde(rename = "name"))]
    pub m_name: StringPtr<'a>,
    /// # C++ Info
    /// - name: `isExclusive`(ctype: `hkUint32`)
    /// - offset: `  0`(x86)/`  8`(x86_64)
    /// - type_size: `  0`(x86)/`  4`(x86_64)
    #[cfg_attr(feature = "serde_default", serde(default))]
    #[cfg_attr(feature = "serde", serde(borrow))]
    #[cfg_attr(feature = "json_schema", schemars(rename = "isExclusive"))]
    #[cfg_attr(feature = "serde", serde(rename = "isExclusive"))]
    pub m_isExclusive: U32<'a>,
    /// # C++ Info
    /// - name: `flags`(ctype: `hkInt32`)
    /// - offset: `  0`(x86)/` 12`(x86_64)
    /// - type_size: `  0`(x86)/`  4`(x86_64)
    #[cfg_attr(feature = "serde_default", serde(default))]
    #[cfg_attr(feature = "serde", serde(borrow))]
    #[cfg_attr(feature = "json_schema", schemars(rename = "flags"))]
    #[cfg_attr(feature = "serde", serde(rename = "flags"))]
    pub m_flags: I32<'a>,
    /// # C++ Info
    /// - name: `triggerType`(ctype: `enum TriggerType`)
    /// - offset: `  0`(x86)/` 16`(x86_64)
    /// - type_size: `  0`(x86)/`  1`(x86_64)
    #[cfg_attr(feature = "serde_default", serde(default))]
    #[cfg_attr(feature = "json_schema", schemars(rename = "triggerType"))]
    #[cfg_attr(feature = "serde", serde(rename = "triggerType"))]
    pub m_triggerType: TriggerType,
    /// # C++ Info
    /// - name: `triggerManifoldTolerance`(ctype: `struct hkUFloat8`)
    /// - offset: `  0`(x86)/` 17`(x86_64)
    /// - type_size: `  0`(x86)/`  1`(x86_64)
    #[cfg_attr(feature = "serde_default", serde(default))]
    #[cfg_attr(feature = "serde", serde(borrow))]
    #[cfg_attr(feature = "json_schema", schemars(rename = "triggerManifoldTolerance"))]
    #[cfg_attr(feature = "serde", serde(rename = "triggerManifoldTolerance"))]
    pub m_triggerManifoldTolerance: hkUFloat8<'a>,
    /// # C++ Info
    /// - name: `dynamicFriction`(ctype: `hkHalf`)
    /// - offset: `  0`(x86)/` 18`(x86_64)
    /// - type_size: `  0`(x86)/`  2`(x86_64)
    #[cfg_attr(feature = "serde_default", serde(default))]
    #[cfg_attr(feature = "json_schema", schemars(rename = "dynamicFriction"))]
    #[cfg_attr(feature = "serde", serde(rename = "dynamicFriction"))]
    pub m_dynamicFriction: f16,
    /// # C++ Info
    /// - name: `staticFriction`(ctype: `hkHalf`)
    /// - offset: `  0`(x86)/` 20`(x86_64)
    /// - type_size: `  0`(x86)/`  2`(x86_64)
    #[cfg_attr(feature = "serde_default", serde(default))]
    #[cfg_attr(feature = "json_schema", schemars(rename = "staticFriction"))]
    #[cfg_attr(feature = "serde", serde(rename = "staticFriction"))]
    pub m_staticFriction: f16,
    /// # C++ Info
    /// - name: `restitution`(ctype: `hkHalf`)
    /// - offset: `  0`(x86)/` 22`(x86_64)
    /// - type_size: `  0`(x86)/`  2`(x86_64)
    #[cfg_attr(feature = "serde_default", serde(default))]
    #[cfg_attr(feature = "json_schema", schemars(rename = "restitution"))]
    #[cfg_attr(feature = "serde", serde(rename = "restitution"))]
    pub m_restitution: f16,
    /// # C++ Info
    /// - name: `frictionCombinePolicy`(ctype: `enum CombinePolicy`)
    /// - offset: `  0`(x86)/` 24`(x86_64)
    /// - type_size: `  0`(x86)/`  1`(x86_64)
    #[cfg_attr(feature = "serde_default", serde(default))]
    #[cfg_attr(feature = "json_schema", schemars(rename = "frictionCombinePolicy"))]
    #[cfg_attr(feature = "serde", serde(rename = "frictionCombinePolicy"))]
    pub m_frictionCombinePolicy: CombinePolicy,
    /// # C++ Info
    /// - name: `restitutionCombinePolicy`(ctype: `enum CombinePolicy`)
    /// - offset: `  0`(x86)/` 25`(x86_64)
    /// - type_size: `  0`(x86)/`  1`(x86_64)
    #[cfg_attr(feature = "serde_default", serde(default))]
    #[cfg_attr(feature = "json_schema", schemars(rename = "restitutionCombinePolicy"))]
    #[cfg_attr(feature = "serde", serde(rename = "restitutionCombinePolicy"))]
    pub m_restitutionCombinePolicy: CombinePolicy,
    /// # C++ Info
    /// - name: `weldingTolerance`(ctype: `hkHalf`)
    /// - offset: `  0`(x86)/` 26`(x86_64)
    /// - type_size: `  0`(x86)/`  2`(x86_64)
    #[cfg_attr(feature = "serde_default", serde(default))]
    #[cfg_attr(feature = "json_schema", schemars(rename = "weldingTolerance"))]
    #[cfg_attr(feature = "serde", serde(rename = "weldingTolerance"))]
    pub m_weldingTolerance: f16,
    /// # C++ Info
    /// - name: `maxContactImpulse`(ctype: `hkReal`)
    /// - offset: `  0`(x86)/` 28`(x86_64)
    /// - type_size: `  0`(x86)/`  4`(x86_64)
    #[cfg_attr(feature = "serde_default", serde(default))]
    #[cfg_attr(feature = "json_schema", schemars(rename = "maxContactImpulse"))]
    #[cfg_attr(feature = "serde", serde(rename = "maxContactImpulse"))]
    pub m_maxContactImpulse: f32,
    /// # C++ Info
    /// - name: `fractionOfClippedImpulseToApply`(ctype: `hkReal`)
    /// - offset: `  0`(x86)/` 32`(x86_64)
    /// - type_size: `  0`(x86)/`  4`(x86_64)
    #[cfg_attr(feature = "serde_default", serde(default))]
    #[cfg_attr(
        feature = "json_schema",
        schemars(rename = "fractionOfClippedImpulseToApply")
    )]
    #[cfg_attr(feature = "serde", serde(rename = "fractionOfClippedImpulseToApply"))]
    pub m_fractionOfClippedImpulseToApply: f32,
    /// # C++ Info
    /// - name: `massChangerCategory`(ctype: `enum MassChangerCategory`)
    /// - offset: `  0`(x86)/` 36`(x86_64)
    /// - type_size: `  0`(x86)/`  1`(x86_64)
    #[cfg_attr(feature = "serde_default", serde(default))]
    #[cfg_attr(feature = "json_schema", schemars(rename = "massChangerCategory"))]
    #[cfg_attr(feature = "serde", serde(rename = "massChangerCategory"))]
    pub m_massChangerCategory: MassChangerCategory,
    /// # C++ Info
    /// - name: `massChangerHeavyObjectFactor`(ctype: `hkHalf`)
    /// - offset: `  0`(x86)/` 38`(x86_64)
    /// - type_size: `  0`(x86)/`  2`(x86_64)
    #[cfg_attr(feature = "serde_default", serde(default))]
    #[cfg_attr(
        feature = "json_schema",
        schemars(rename = "massChangerHeavyObjectFactor")
    )]
    #[cfg_attr(feature = "serde", serde(rename = "massChangerHeavyObjectFactor"))]
    pub m_massChangerHeavyObjectFactor: f16,
    /// # C++ Info
    /// - name: `softContactForceFactor`(ctype: `hkHalf`)
    /// - offset: `  0`(x86)/` 40`(x86_64)
    /// - type_size: `  0`(x86)/`  2`(x86_64)
    #[cfg_attr(feature = "serde_default", serde(default))]
    #[cfg_attr(feature = "json_schema", schemars(rename = "softContactForceFactor"))]
    #[cfg_attr(feature = "serde", serde(rename = "softContactForceFactor"))]
    pub m_softContactForceFactor: f16,
    /// # C++ Info
    /// - name: `softContactDampFactor`(ctype: `hkHalf`)
    /// - offset: `  0`(x86)/` 42`(x86_64)
    /// - type_size: `  0`(x86)/`  2`(x86_64)
    #[cfg_attr(feature = "serde_default", serde(default))]
    #[cfg_attr(feature = "json_schema", schemars(rename = "softContactDampFactor"))]
    #[cfg_attr(feature = "serde", serde(rename = "softContactDampFactor"))]
    pub m_softContactDampFactor: f16,
    /// # C++ Info
    /// - name: `softContactSeperationVelocity`(ctype: `struct hkUFloat8`)
    /// - offset: `  0`(x86)/` 44`(x86_64)
    /// - type_size: `  0`(x86)/`  1`(x86_64)
    #[cfg_attr(feature = "serde_default", serde(default))]
    #[cfg_attr(feature = "serde", serde(borrow))]
    #[cfg_attr(
        feature = "json_schema",
        schemars(rename = "softContactSeperationVelocity")
    )]
    #[cfg_attr(feature = "serde", serde(rename = "softContactSeperationVelocity"))]
    pub m_softContactSeperationVelocity: hkUFloat8<'a>,
    /// # C++ Info
    /// - name: `surfaceVelocity`(ctype: `struct hknpSurfaceVelocity*`)
    /// - offset: `  0`(x86)/` 48`(x86_64)
    /// - type_size: `  0`(x86)/`  8`(x86_64)
    #[cfg_attr(feature = "serde_default", serde(default))]
    #[cfg_attr(feature = "serde", serde(borrow))]
    #[cfg_attr(feature = "json_schema", schemars(rename = "surfaceVelocity"))]
    #[cfg_attr(feature = "serde", serde(rename = "surfaceVelocity"))]
    pub m_surfaceVelocity: Pointer<'a>,
    /// # C++ Info
    /// - name: `disablingCollisionsBetweenCvxCvxDynamicObjectsDistance`(ctype: `hkHalf`)
    /// - offset: `  0`(x86)/` 56`(x86_64)
    /// - type_size: `  0`(x86)/`  2`(x86_64)
    #[cfg_attr(feature = "serde_default", serde(default))]
    #[cfg_attr(
        feature = "json_schema",
        schemars(rename = "disablingCollisionsBetweenCvxCvxDynamicObjectsDistance")
    )]
    #[cfg_attr(
        feature = "serde",
        serde(rename = "disablingCollisionsBetweenCvxCvxDynamicObjectsDistance")
    )]
    pub m_disablingCollisionsBetweenCvxCvxDynamicObjectsDistance: f16,
    /// # C++ Info
    /// - name: `userData`(ctype: `hkUint64`)
    /// - offset: `  0`(x86)/` 64`(x86_64)
    /// - type_size: `  0`(x86)/`  8`(x86_64)
    /// - flags: `ALIGN_8`
    #[cfg_attr(feature = "serde_default", serde(default))]
    #[cfg_attr(feature = "serde", serde(borrow))]
    #[cfg_attr(feature = "json_schema", schemars(rename = "userData"))]
    #[cfg_attr(feature = "serde", serde(rename = "userData"))]
    pub m_userData: U64<'a>,
    /// # C++ Info
    /// - name: `isShared`(ctype: `hkBool`)
    /// - offset: `  0`(x86)/` 72`(x86_64)
    /// - type_size: `  0`(x86)/`  1`(x86_64)
    #[cfg_attr(feature = "serde_default", serde(default))]
    #[cfg_attr(feature = "json_schema", schemars(rename = "isShared"))]
    #[cfg_attr(feature = "serde", serde(rename = "isShared"))]
    pub m_isShared: bool,
}
const _: () = {
    use havok_serde as _serde;
    impl<'a> _serde::HavokClass for hknpMaterial<'a> {
        #[inline]
        fn name(&self) -> &'static str {
            "hknpMaterial"
        }
        #[inline]
        fn signature(&self) -> _serde::__private::Signature {
            _serde::__private::Signature::new(0xb7c5f24e)
        }
        #[allow(clippy::let_and_return, clippy::vec_init_then_push)]
        fn deps_indexes(&self) -> Vec<&Pointer<'_>> {
            let mut v = Vec::new();
            v.extend(self.m_triggerManifoldTolerance.deps_indexes());
            v.extend(self.m_softContactSeperationVelocity.deps_indexes());
            v.push(&self.m_surfaceVelocity);
            v
        }
    }
    impl<'a> _serde::Serialize for hknpMaterial<'a> {
        fn serialize<S>(&self, __serializer: S) -> Result<S::Ok, S::Error>
        where
            S: _serde::ser::Serializer,
        {
            let class_meta = self
                .__ptr
                .as_ref()
                .map(|name| (name, _serde::__private::Signature::new(0xb7c5f24e)));
            let mut serializer = __serializer
                .serialize_struct("hknpMaterial", class_meta, (0u64, 80u64))?;
            serializer.serialize_field("name", &self.m_name)?;
            serializer.serialize_field("isExclusive", &self.m_isExclusive)?;
            serializer.serialize_field("flags", &self.m_flags)?;
            serializer.serialize_field("triggerType", &self.m_triggerType)?;
            serializer
                .serialize_field(
                    "triggerManifoldTolerance",
                    &self.m_triggerManifoldTolerance,
                )?;
            serializer.serialize_field("dynamicFriction", &self.m_dynamicFriction)?;
            serializer.serialize_field("staticFriction", &self.m_staticFriction)?;
            serializer.serialize_field("restitution", &self.m_restitution)?;
            serializer
                .serialize_field(
                    "frictionCombinePolicy",
                    &self.m_frictionCombinePolicy,
                )?;
            serializer
                .serialize_field(
                    "restitutionCombinePolicy",
                    &self.m_restitutionCombinePolicy,
                )?;
            serializer.serialize_field("weldingTolerance", &self.m_weldingTolerance)?;
            serializer.serialize_field("maxContactImpulse", &self.m_maxContactImpulse)?;
            serializer
                .serialize_field(
                    "fractionOfClippedImpulseToApply",
                    &self.m_fractionOfClippedImpulseToApply,
                )?;
            serializer
                .serialize_field("massChangerCategory", &self.m_massChangerCategory)?;
            serializer.pad_field([0u8; 0usize].as_slice(), [0u8; 1usize].as_slice())?;
            serializer
                .serialize_field(
                    "massChangerHeavyObjectFactor",
                    &self.m_massChangerHeavyObjectFactor,
                )?;
            serializer
                .serialize_field(
                    "softContactForceFactor",
                    &self.m_softContactForceFactor,
                )?;
            serializer
                .serialize_field(
                    "softContactDampFactor",
                    &self.m_softContactDampFactor,
                )?;
            serializer
                .serialize_field(
                    "softContactSeperationVelocity",
                    &self.m_softContactSeperationVelocity,
                )?;
            serializer.pad_field([0u8; 0usize].as_slice(), [0u8; 3usize].as_slice())?;
            serializer.serialize_field("surfaceVelocity", &self.m_surfaceVelocity)?;
            serializer
                .serialize_field(
                    "disablingCollisionsBetweenCvxCvxDynamicObjectsDistance",
                    &self.m_disablingCollisionsBetweenCvxCvxDynamicObjectsDistance,
                )?;
            serializer.pad_field([0u8; 0usize].as_slice(), [0u8; 6usize].as_slice())?;
            serializer.serialize_field("userData", &self.m_userData)?;
            serializer.serialize_field("isShared", &self.m_isShared)?;
            serializer.pad_field([0u8; 0usize].as_slice(), [0u8; 7usize].as_slice())?;
            serializer.end()
        }
    }
};
#[doc(hidden)]
#[allow(non_upper_case_globals, unused_attributes, unused_qualifications)]
const _: () = {
    use havok_serde as _serde;
    #[automatically_derived]
    impl<'de> _serde::Deserialize<'de> for hknpMaterial<'de> {
        fn deserialize<__D>(deserializer: __D) -> core::result::Result<Self, __D::Error>
        where
            __D: _serde::Deserializer<'de>,
        {
            #[allow(non_camel_case_types)]
            enum __Field {
                m_name,
                m_isExclusive,
                m_flags,
                m_triggerType,
                m_triggerManifoldTolerance,
                m_dynamicFriction,
                m_staticFriction,
                m_restitution,
                m_frictionCombinePolicy,
                m_restitutionCombinePolicy,
                m_weldingTolerance,
                m_maxContactImpulse,
                m_fractionOfClippedImpulseToApply,
                m_massChangerCategory,
                m_massChangerHeavyObjectFactor,
                m_softContactForceFactor,
                m_softContactDampFactor,
                m_softContactSeperationVelocity,
                m_surfaceVelocity,
                m_disablingCollisionsBetweenCvxCvxDynamicObjectsDistance,
                m_userData,
                m_isShared,
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
                        "isExclusive" => Ok(__Field::m_isExclusive),
                        "flags" => Ok(__Field::m_flags),
                        "triggerType" => Ok(__Field::m_triggerType),
                        "triggerManifoldTolerance" => {
                            Ok(__Field::m_triggerManifoldTolerance)
                        }
                        "dynamicFriction" => Ok(__Field::m_dynamicFriction),
                        "staticFriction" => Ok(__Field::m_staticFriction),
                        "restitution" => Ok(__Field::m_restitution),
                        "frictionCombinePolicy" => Ok(__Field::m_frictionCombinePolicy),
                        "restitutionCombinePolicy" => {
                            Ok(__Field::m_restitutionCombinePolicy)
                        }
                        "weldingTolerance" => Ok(__Field::m_weldingTolerance),
                        "maxContactImpulse" => Ok(__Field::m_maxContactImpulse),
                        "fractionOfClippedImpulseToApply" => {
                            Ok(__Field::m_fractionOfClippedImpulseToApply)
                        }
                        "massChangerCategory" => Ok(__Field::m_massChangerCategory),
                        "massChangerHeavyObjectFactor" => {
                            Ok(__Field::m_massChangerHeavyObjectFactor)
                        }
                        "softContactForceFactor" => Ok(__Field::m_softContactForceFactor),
                        "softContactDampFactor" => Ok(__Field::m_softContactDampFactor),
                        "softContactSeperationVelocity" => {
                            Ok(__Field::m_softContactSeperationVelocity)
                        }
                        "surfaceVelocity" => Ok(__Field::m_surfaceVelocity),
                        "disablingCollisionsBetweenCvxCvxDynamicObjectsDistance" => {
                            Ok(
                                __Field::m_disablingCollisionsBetweenCvxCvxDynamicObjectsDistance,
                            )
                        }
                        "userData" => Ok(__Field::m_userData),
                        "isShared" => Ok(__Field::m_isShared),
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
            struct __hknpMaterialVisitor<'de> {
                marker: _serde::__private::PhantomData<hknpMaterial<'de>>,
                lifetime: _serde::__private::PhantomData<&'de ()>,
            }
            #[allow(clippy::match_single_binding)]
            #[allow(clippy::reversed_empty_ranges)]
            #[allow(clippy::single_match)]
            impl<'de> _serde::de::Visitor<'de> for __hknpMaterialVisitor<'de> {
                type Value = hknpMaterial<'de>;
                fn expecting(
                    &self,
                    __formatter: &mut core::fmt::Formatter,
                ) -> core::fmt::Result {
                    core::fmt::Formatter::write_str(__formatter, "struct hknpMaterial")
                }
                fn visit_struct_for_bytes<__A>(
                    self,
                    mut __map: __A,
                ) -> _serde::__private::Result<Self::Value, __A::Error>
                where
                    __A: _serde::de::MapAccess<'de>,
                {
                    let __ptr = __A::class_ptr(&mut __map);
                    let mut m_name: _serde::__private::Option<StringPtr<'de>> = _serde::__private::None;
                    let mut m_isExclusive: _serde::__private::Option<U32<'de>> = _serde::__private::None;
                    let mut m_flags: _serde::__private::Option<I32<'de>> = _serde::__private::None;
                    let mut m_triggerType: _serde::__private::Option<TriggerType> = _serde::__private::None;
                    let mut m_triggerManifoldTolerance: _serde::__private::Option<
                        hkUFloat8<'de>,
                    > = _serde::__private::None;
                    let mut m_dynamicFriction: _serde::__private::Option<f16> = _serde::__private::None;
                    let mut m_staticFriction: _serde::__private::Option<f16> = _serde::__private::None;
                    let mut m_restitution: _serde::__private::Option<f16> = _serde::__private::None;
                    let mut m_frictionCombinePolicy: _serde::__private::Option<
                        CombinePolicy,
                    > = _serde::__private::None;
                    let mut m_restitutionCombinePolicy: _serde::__private::Option<
                        CombinePolicy,
                    > = _serde::__private::None;
                    let mut m_weldingTolerance: _serde::__private::Option<f16> = _serde::__private::None;
                    let mut m_maxContactImpulse: _serde::__private::Option<f32> = _serde::__private::None;
                    let mut m_fractionOfClippedImpulseToApply: _serde::__private::Option<
                        f32,
                    > = _serde::__private::None;
                    let mut m_massChangerCategory: _serde::__private::Option<
                        MassChangerCategory,
                    > = _serde::__private::None;
                    let mut m_massChangerHeavyObjectFactor: _serde::__private::Option<
                        f16,
                    > = _serde::__private::None;
                    let mut m_softContactForceFactor: _serde::__private::Option<f16> = _serde::__private::None;
                    let mut m_softContactDampFactor: _serde::__private::Option<f16> = _serde::__private::None;
                    let mut m_softContactSeperationVelocity: _serde::__private::Option<
                        hkUFloat8<'de>,
                    > = _serde::__private::None;
                    let mut m_surfaceVelocity: _serde::__private::Option<Pointer<'de>> = _serde::__private::None;
                    let mut m_disablingCollisionsBetweenCvxCvxDynamicObjectsDistance: _serde::__private::Option<
                        f16,
                    > = _serde::__private::None;
                    let mut m_userData: _serde::__private::Option<U64<'de>> = _serde::__private::None;
                    let mut m_isShared: _serde::__private::Option<bool> = _serde::__private::None;
                    for i in 0..22usize {
                        match i {
                            0usize => {
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
                            1usize => {
                                if _serde::__private::Option::is_some(&m_isExclusive) {
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "isExclusive",
                                        ),
                                    );
                                }
                                m_isExclusive = _serde::__private::Some(
                                    match __A::next_value::<U32<'de>>(&mut __map) {
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
                                    match __A::next_value::<I32<'de>>(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            3usize => {
                                if _serde::__private::Option::is_some(&m_triggerType) {
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "triggerType",
                                        ),
                                    );
                                }
                                m_triggerType = _serde::__private::Some(
                                    match __A::next_value::<TriggerType>(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            4usize => {
                                if _serde::__private::Option::is_some(
                                    &m_triggerManifoldTolerance,
                                ) {
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "triggerManifoldTolerance",
                                        ),
                                    );
                                }
                                m_triggerManifoldTolerance = _serde::__private::Some(
                                    match __A::next_value::<hkUFloat8<'de>>(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            5usize => {
                                if _serde::__private::Option::is_some(&m_dynamicFriction) {
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "dynamicFriction",
                                        ),
                                    );
                                }
                                m_dynamicFriction = _serde::__private::Some(
                                    match __A::next_value::<f16>(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            6usize => {
                                if _serde::__private::Option::is_some(&m_staticFriction) {
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "staticFriction",
                                        ),
                                    );
                                }
                                m_staticFriction = _serde::__private::Some(
                                    match __A::next_value::<f16>(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            7usize => {
                                if _serde::__private::Option::is_some(&m_restitution) {
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "restitution",
                                        ),
                                    );
                                }
                                m_restitution = _serde::__private::Some(
                                    match __A::next_value::<f16>(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            8usize => {
                                if _serde::__private::Option::is_some(
                                    &m_frictionCombinePolicy,
                                ) {
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "frictionCombinePolicy",
                                        ),
                                    );
                                }
                                m_frictionCombinePolicy = _serde::__private::Some(
                                    match __A::next_value::<CombinePolicy>(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            9usize => {
                                if _serde::__private::Option::is_some(
                                    &m_restitutionCombinePolicy,
                                ) {
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "restitutionCombinePolicy",
                                        ),
                                    );
                                }
                                m_restitutionCombinePolicy = _serde::__private::Some(
                                    match __A::next_value::<CombinePolicy>(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            10usize => {
                                if _serde::__private::Option::is_some(&m_weldingTolerance) {
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "weldingTolerance",
                                        ),
                                    );
                                }
                                m_weldingTolerance = _serde::__private::Some(
                                    match __A::next_value::<f16>(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            11usize => {
                                if _serde::__private::Option::is_some(
                                    &m_maxContactImpulse,
                                ) {
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "maxContactImpulse",
                                        ),
                                    );
                                }
                                m_maxContactImpulse = _serde::__private::Some(
                                    match __A::next_value::<f32>(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            12usize => {
                                if _serde::__private::Option::is_some(
                                    &m_fractionOfClippedImpulseToApply,
                                ) {
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "fractionOfClippedImpulseToApply",
                                        ),
                                    );
                                }
                                m_fractionOfClippedImpulseToApply = _serde::__private::Some(
                                    match __A::next_value::<f32>(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            13usize => {
                                if _serde::__private::Option::is_some(
                                    &m_massChangerCategory,
                                ) {
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "massChangerCategory",
                                        ),
                                    );
                                }
                                m_massChangerCategory = _serde::__private::Some(
                                    match __A::next_value::<MassChangerCategory>(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            14usize => {
                                if _serde::__private::Option::is_some(
                                    &m_massChangerHeavyObjectFactor,
                                ) {
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "massChangerHeavyObjectFactor",
                                        ),
                                    );
                                }
                                __A::pad(&mut __map, 0usize, 1usize)?;
                                m_massChangerHeavyObjectFactor = _serde::__private::Some(
                                    match __A::next_value::<f16>(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            15usize => {
                                if _serde::__private::Option::is_some(
                                    &m_softContactForceFactor,
                                ) {
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "softContactForceFactor",
                                        ),
                                    );
                                }
                                m_softContactForceFactor = _serde::__private::Some(
                                    match __A::next_value::<f16>(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            16usize => {
                                if _serde::__private::Option::is_some(
                                    &m_softContactDampFactor,
                                ) {
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "softContactDampFactor",
                                        ),
                                    );
                                }
                                m_softContactDampFactor = _serde::__private::Some(
                                    match __A::next_value::<f16>(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            17usize => {
                                if _serde::__private::Option::is_some(
                                    &m_softContactSeperationVelocity,
                                ) {
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "softContactSeperationVelocity",
                                        ),
                                    );
                                }
                                m_softContactSeperationVelocity = _serde::__private::Some(
                                    match __A::next_value::<hkUFloat8<'de>>(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            18usize => {
                                if _serde::__private::Option::is_some(&m_surfaceVelocity) {
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "surfaceVelocity",
                                        ),
                                    );
                                }
                                __A::pad(&mut __map, 0usize, 3usize)?;
                                m_surfaceVelocity = _serde::__private::Some(
                                    match __A::next_value::<Pointer<'de>>(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            19usize => {
                                if _serde::__private::Option::is_some(
                                    &m_disablingCollisionsBetweenCvxCvxDynamicObjectsDistance,
                                ) {
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "disablingCollisionsBetweenCvxCvxDynamicObjectsDistance",
                                        ),
                                    );
                                }
                                m_disablingCollisionsBetweenCvxCvxDynamicObjectsDistance = _serde::__private::Some(
                                    match __A::next_value::<f16>(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            20usize => {
                                if _serde::__private::Option::is_some(&m_userData) {
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "userData",
                                        ),
                                    );
                                }
                                __A::pad(&mut __map, 0usize, 6usize)?;
                                m_userData = _serde::__private::Some(
                                    match __A::next_value::<U64<'de>>(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            21usize => {
                                if _serde::__private::Option::is_some(&m_isShared) {
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "isShared",
                                        ),
                                    );
                                }
                                m_isShared = _serde::__private::Some(
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
                    __A::pad(&mut __map, 0usize, 7usize)?;
                    let m_name = match m_name {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field("name"),
                            );
                        }
                    };
                    let m_isExclusive = match m_isExclusive {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "isExclusive",
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
                    let m_triggerType = match m_triggerType {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "triggerType",
                                ),
                            );
                        }
                    };
                    let m_triggerManifoldTolerance = match m_triggerManifoldTolerance {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "triggerManifoldTolerance",
                                ),
                            );
                        }
                    };
                    let m_dynamicFriction = match m_dynamicFriction {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "dynamicFriction",
                                ),
                            );
                        }
                    };
                    let m_staticFriction = match m_staticFriction {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "staticFriction",
                                ),
                            );
                        }
                    };
                    let m_restitution = match m_restitution {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "restitution",
                                ),
                            );
                        }
                    };
                    let m_frictionCombinePolicy = match m_frictionCombinePolicy {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "frictionCombinePolicy",
                                ),
                            );
                        }
                    };
                    let m_restitutionCombinePolicy = match m_restitutionCombinePolicy {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "restitutionCombinePolicy",
                                ),
                            );
                        }
                    };
                    let m_weldingTolerance = match m_weldingTolerance {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "weldingTolerance",
                                ),
                            );
                        }
                    };
                    let m_maxContactImpulse = match m_maxContactImpulse {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "maxContactImpulse",
                                ),
                            );
                        }
                    };
                    let m_fractionOfClippedImpulseToApply = match m_fractionOfClippedImpulseToApply {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "fractionOfClippedImpulseToApply",
                                ),
                            );
                        }
                    };
                    let m_massChangerCategory = match m_massChangerCategory {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "massChangerCategory",
                                ),
                            );
                        }
                    };
                    let m_massChangerHeavyObjectFactor = match m_massChangerHeavyObjectFactor {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "massChangerHeavyObjectFactor",
                                ),
                            );
                        }
                    };
                    let m_softContactForceFactor = match m_softContactForceFactor {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "softContactForceFactor",
                                ),
                            );
                        }
                    };
                    let m_softContactDampFactor = match m_softContactDampFactor {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "softContactDampFactor",
                                ),
                            );
                        }
                    };
                    let m_softContactSeperationVelocity = match m_softContactSeperationVelocity {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "softContactSeperationVelocity",
                                ),
                            );
                        }
                    };
                    let m_surfaceVelocity = match m_surfaceVelocity {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "surfaceVelocity",
                                ),
                            );
                        }
                    };
                    let m_disablingCollisionsBetweenCvxCvxDynamicObjectsDistance = match m_disablingCollisionsBetweenCvxCvxDynamicObjectsDistance {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "disablingCollisionsBetweenCvxCvxDynamicObjectsDistance",
                                ),
                            );
                        }
                    };
                    let m_userData = match m_userData {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field("userData"),
                            );
                        }
                    };
                    let m_isShared = match m_isShared {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field("isShared"),
                            );
                        }
                    };
                    _serde::__private::Ok(hknpMaterial {
                        __ptr,
                        m_name,
                        m_isExclusive,
                        m_flags,
                        m_triggerType,
                        m_triggerManifoldTolerance,
                        m_dynamicFriction,
                        m_staticFriction,
                        m_restitution,
                        m_frictionCombinePolicy,
                        m_restitutionCombinePolicy,
                        m_weldingTolerance,
                        m_maxContactImpulse,
                        m_fractionOfClippedImpulseToApply,
                        m_massChangerCategory,
                        m_massChangerHeavyObjectFactor,
                        m_softContactForceFactor,
                        m_softContactDampFactor,
                        m_softContactSeperationVelocity,
                        m_surfaceVelocity,
                        m_disablingCollisionsBetweenCvxCvxDynamicObjectsDistance,
                        m_userData,
                        m_isShared,
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
                    let mut m_isExclusive: _serde::__private::Option<U32<'de>> = _serde::__private::None;
                    let mut m_flags: _serde::__private::Option<I32<'de>> = _serde::__private::None;
                    let mut m_triggerType: _serde::__private::Option<TriggerType> = _serde::__private::None;
                    let mut m_triggerManifoldTolerance: _serde::__private::Option<
                        hkUFloat8<'de>,
                    > = _serde::__private::None;
                    let mut m_dynamicFriction: _serde::__private::Option<f16> = _serde::__private::None;
                    let mut m_staticFriction: _serde::__private::Option<f16> = _serde::__private::None;
                    let mut m_restitution: _serde::__private::Option<f16> = _serde::__private::None;
                    let mut m_frictionCombinePolicy: _serde::__private::Option<
                        CombinePolicy,
                    > = _serde::__private::None;
                    let mut m_restitutionCombinePolicy: _serde::__private::Option<
                        CombinePolicy,
                    > = _serde::__private::None;
                    let mut m_weldingTolerance: _serde::__private::Option<f16> = _serde::__private::None;
                    let mut m_maxContactImpulse: _serde::__private::Option<f32> = _serde::__private::None;
                    let mut m_fractionOfClippedImpulseToApply: _serde::__private::Option<
                        f32,
                    > = _serde::__private::None;
                    let mut m_massChangerCategory: _serde::__private::Option<
                        MassChangerCategory,
                    > = _serde::__private::None;
                    let mut m_massChangerHeavyObjectFactor: _serde::__private::Option<
                        f16,
                    > = _serde::__private::None;
                    let mut m_softContactForceFactor: _serde::__private::Option<f16> = _serde::__private::None;
                    let mut m_softContactDampFactor: _serde::__private::Option<f16> = _serde::__private::None;
                    let mut m_softContactSeperationVelocity: _serde::__private::Option<
                        hkUFloat8<'de>,
                    > = _serde::__private::None;
                    let mut m_surfaceVelocity: _serde::__private::Option<Pointer<'de>> = _serde::__private::None;
                    let mut m_disablingCollisionsBetweenCvxCvxDynamicObjectsDistance: _serde::__private::Option<
                        f16,
                    > = _serde::__private::None;
                    let mut m_userData: _serde::__private::Option<U64<'de>> = _serde::__private::None;
                    let mut m_isShared: _serde::__private::Option<bool> = _serde::__private::None;
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
                            __Field::m_isExclusive => {
                                #[cfg(
                                    any(feature = "strict", feature = "ignore_duplicates")
                                )]
                                if _serde::__private::Option::is_some(&m_isExclusive) {
                                    #[cfg(feature = "ignore_duplicates")]
                                    {
                                        __A::skip_value(&mut __map)?;
                                        continue;
                                    }
                                    #[cfg(feature = "strict")]
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "isExclusive",
                                        ),
                                    );
                                }
                                m_isExclusive = _serde::__private::Some(
                                    match __A::next_value::<U32<'de>>(&mut __map) {
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
                                    match __A::next_value::<I32<'de>>(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            __Field::m_triggerType => {
                                #[cfg(
                                    any(feature = "strict", feature = "ignore_duplicates")
                                )]
                                if _serde::__private::Option::is_some(&m_triggerType) {
                                    #[cfg(feature = "ignore_duplicates")]
                                    {
                                        __A::skip_value(&mut __map)?;
                                        continue;
                                    }
                                    #[cfg(feature = "strict")]
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "triggerType",
                                        ),
                                    );
                                }
                                m_triggerType = _serde::__private::Some(
                                    match __A::next_value::<TriggerType>(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            __Field::m_triggerManifoldTolerance => {
                                #[cfg(
                                    any(feature = "strict", feature = "ignore_duplicates")
                                )]
                                if _serde::__private::Option::is_some(
                                    &m_triggerManifoldTolerance,
                                ) {
                                    #[cfg(feature = "ignore_duplicates")]
                                    {
                                        __A::skip_value(&mut __map)?;
                                        continue;
                                    }
                                    #[cfg(feature = "strict")]
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "triggerManifoldTolerance",
                                        ),
                                    );
                                }
                                m_triggerManifoldTolerance = _serde::__private::Some(
                                    match __A::next_value::<hkUFloat8<'de>>(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            __Field::m_dynamicFriction => {
                                #[cfg(
                                    any(feature = "strict", feature = "ignore_duplicates")
                                )]
                                if _serde::__private::Option::is_some(&m_dynamicFriction) {
                                    #[cfg(feature = "ignore_duplicates")]
                                    {
                                        __A::skip_value(&mut __map)?;
                                        continue;
                                    }
                                    #[cfg(feature = "strict")]
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "dynamicFriction",
                                        ),
                                    );
                                }
                                m_dynamicFriction = _serde::__private::Some(
                                    match __A::next_value::<f16>(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            __Field::m_staticFriction => {
                                #[cfg(
                                    any(feature = "strict", feature = "ignore_duplicates")
                                )]
                                if _serde::__private::Option::is_some(&m_staticFriction) {
                                    #[cfg(feature = "ignore_duplicates")]
                                    {
                                        __A::skip_value(&mut __map)?;
                                        continue;
                                    }
                                    #[cfg(feature = "strict")]
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "staticFriction",
                                        ),
                                    );
                                }
                                m_staticFriction = _serde::__private::Some(
                                    match __A::next_value::<f16>(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            __Field::m_restitution => {
                                #[cfg(
                                    any(feature = "strict", feature = "ignore_duplicates")
                                )]
                                if _serde::__private::Option::is_some(&m_restitution) {
                                    #[cfg(feature = "ignore_duplicates")]
                                    {
                                        __A::skip_value(&mut __map)?;
                                        continue;
                                    }
                                    #[cfg(feature = "strict")]
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "restitution",
                                        ),
                                    );
                                }
                                m_restitution = _serde::__private::Some(
                                    match __A::next_value::<f16>(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            __Field::m_frictionCombinePolicy => {
                                #[cfg(
                                    any(feature = "strict", feature = "ignore_duplicates")
                                )]
                                if _serde::__private::Option::is_some(
                                    &m_frictionCombinePolicy,
                                ) {
                                    #[cfg(feature = "ignore_duplicates")]
                                    {
                                        __A::skip_value(&mut __map)?;
                                        continue;
                                    }
                                    #[cfg(feature = "strict")]
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "frictionCombinePolicy",
                                        ),
                                    );
                                }
                                m_frictionCombinePolicy = _serde::__private::Some(
                                    match __A::next_value::<CombinePolicy>(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            __Field::m_restitutionCombinePolicy => {
                                #[cfg(
                                    any(feature = "strict", feature = "ignore_duplicates")
                                )]
                                if _serde::__private::Option::is_some(
                                    &m_restitutionCombinePolicy,
                                ) {
                                    #[cfg(feature = "ignore_duplicates")]
                                    {
                                        __A::skip_value(&mut __map)?;
                                        continue;
                                    }
                                    #[cfg(feature = "strict")]
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "restitutionCombinePolicy",
                                        ),
                                    );
                                }
                                m_restitutionCombinePolicy = _serde::__private::Some(
                                    match __A::next_value::<CombinePolicy>(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            __Field::m_weldingTolerance => {
                                #[cfg(
                                    any(feature = "strict", feature = "ignore_duplicates")
                                )]
                                if _serde::__private::Option::is_some(&m_weldingTolerance) {
                                    #[cfg(feature = "ignore_duplicates")]
                                    {
                                        __A::skip_value(&mut __map)?;
                                        continue;
                                    }
                                    #[cfg(feature = "strict")]
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "weldingTolerance",
                                        ),
                                    );
                                }
                                m_weldingTolerance = _serde::__private::Some(
                                    match __A::next_value::<f16>(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            __Field::m_maxContactImpulse => {
                                #[cfg(
                                    any(feature = "strict", feature = "ignore_duplicates")
                                )]
                                if _serde::__private::Option::is_some(
                                    &m_maxContactImpulse,
                                ) {
                                    #[cfg(feature = "ignore_duplicates")]
                                    {
                                        __A::skip_value(&mut __map)?;
                                        continue;
                                    }
                                    #[cfg(feature = "strict")]
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "maxContactImpulse",
                                        ),
                                    );
                                }
                                m_maxContactImpulse = _serde::__private::Some(
                                    match __A::next_value::<f32>(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            __Field::m_fractionOfClippedImpulseToApply => {
                                #[cfg(
                                    any(feature = "strict", feature = "ignore_duplicates")
                                )]
                                if _serde::__private::Option::is_some(
                                    &m_fractionOfClippedImpulseToApply,
                                ) {
                                    #[cfg(feature = "ignore_duplicates")]
                                    {
                                        __A::skip_value(&mut __map)?;
                                        continue;
                                    }
                                    #[cfg(feature = "strict")]
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "fractionOfClippedImpulseToApply",
                                        ),
                                    );
                                }
                                m_fractionOfClippedImpulseToApply = _serde::__private::Some(
                                    match __A::next_value::<f32>(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            __Field::m_massChangerCategory => {
                                #[cfg(
                                    any(feature = "strict", feature = "ignore_duplicates")
                                )]
                                if _serde::__private::Option::is_some(
                                    &m_massChangerCategory,
                                ) {
                                    #[cfg(feature = "ignore_duplicates")]
                                    {
                                        __A::skip_value(&mut __map)?;
                                        continue;
                                    }
                                    #[cfg(feature = "strict")]
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "massChangerCategory",
                                        ),
                                    );
                                }
                                m_massChangerCategory = _serde::__private::Some(
                                    match __A::next_value::<MassChangerCategory>(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            __Field::m_massChangerHeavyObjectFactor => {
                                #[cfg(
                                    any(feature = "strict", feature = "ignore_duplicates")
                                )]
                                if _serde::__private::Option::is_some(
                                    &m_massChangerHeavyObjectFactor,
                                ) {
                                    #[cfg(feature = "ignore_duplicates")]
                                    {
                                        __A::skip_value(&mut __map)?;
                                        continue;
                                    }
                                    #[cfg(feature = "strict")]
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "massChangerHeavyObjectFactor",
                                        ),
                                    );
                                }
                                m_massChangerHeavyObjectFactor = _serde::__private::Some(
                                    match __A::next_value::<f16>(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            __Field::m_softContactForceFactor => {
                                #[cfg(
                                    any(feature = "strict", feature = "ignore_duplicates")
                                )]
                                if _serde::__private::Option::is_some(
                                    &m_softContactForceFactor,
                                ) {
                                    #[cfg(feature = "ignore_duplicates")]
                                    {
                                        __A::skip_value(&mut __map)?;
                                        continue;
                                    }
                                    #[cfg(feature = "strict")]
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "softContactForceFactor",
                                        ),
                                    );
                                }
                                m_softContactForceFactor = _serde::__private::Some(
                                    match __A::next_value::<f16>(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            __Field::m_softContactDampFactor => {
                                #[cfg(
                                    any(feature = "strict", feature = "ignore_duplicates")
                                )]
                                if _serde::__private::Option::is_some(
                                    &m_softContactDampFactor,
                                ) {
                                    #[cfg(feature = "ignore_duplicates")]
                                    {
                                        __A::skip_value(&mut __map)?;
                                        continue;
                                    }
                                    #[cfg(feature = "strict")]
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "softContactDampFactor",
                                        ),
                                    );
                                }
                                m_softContactDampFactor = _serde::__private::Some(
                                    match __A::next_value::<f16>(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            __Field::m_softContactSeperationVelocity => {
                                #[cfg(
                                    any(feature = "strict", feature = "ignore_duplicates")
                                )]
                                if _serde::__private::Option::is_some(
                                    &m_softContactSeperationVelocity,
                                ) {
                                    #[cfg(feature = "ignore_duplicates")]
                                    {
                                        __A::skip_value(&mut __map)?;
                                        continue;
                                    }
                                    #[cfg(feature = "strict")]
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "softContactSeperationVelocity",
                                        ),
                                    );
                                }
                                m_softContactSeperationVelocity = _serde::__private::Some(
                                    match __A::next_value::<hkUFloat8<'de>>(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            __Field::m_surfaceVelocity => {
                                #[cfg(
                                    any(feature = "strict", feature = "ignore_duplicates")
                                )]
                                if _serde::__private::Option::is_some(&m_surfaceVelocity) {
                                    #[cfg(feature = "ignore_duplicates")]
                                    {
                                        __A::skip_value(&mut __map)?;
                                        continue;
                                    }
                                    #[cfg(feature = "strict")]
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "surfaceVelocity",
                                        ),
                                    );
                                }
                                m_surfaceVelocity = _serde::__private::Some(
                                    match __A::next_value::<Pointer<'de>>(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            __Field::m_disablingCollisionsBetweenCvxCvxDynamicObjectsDistance => {
                                #[cfg(
                                    any(feature = "strict", feature = "ignore_duplicates")
                                )]
                                if _serde::__private::Option::is_some(
                                    &m_disablingCollisionsBetweenCvxCvxDynamicObjectsDistance,
                                ) {
                                    #[cfg(feature = "ignore_duplicates")]
                                    {
                                        __A::skip_value(&mut __map)?;
                                        continue;
                                    }
                                    #[cfg(feature = "strict")]
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "disablingCollisionsBetweenCvxCvxDynamicObjectsDistance",
                                        ),
                                    );
                                }
                                m_disablingCollisionsBetweenCvxCvxDynamicObjectsDistance = _serde::__private::Some(
                                    match __A::next_value::<f16>(&mut __map) {
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
                                    match __A::next_value::<U64<'de>>(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            __Field::m_isShared => {
                                #[cfg(
                                    any(feature = "strict", feature = "ignore_duplicates")
                                )]
                                if _serde::__private::Option::is_some(&m_isShared) {
                                    #[cfg(feature = "ignore_duplicates")]
                                    {
                                        __A::skip_value(&mut __map)?;
                                        continue;
                                    }
                                    #[cfg(feature = "strict")]
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "isShared",
                                        ),
                                    );
                                }
                                m_isShared = _serde::__private::Some(
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
                    let m_isExclusive = match m_isExclusive {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            #[cfg(feature = "strict")]
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "isExclusive",
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
                    let m_triggerType = match m_triggerType {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            #[cfg(feature = "strict")]
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "triggerType",
                                ),
                            );
                            #[cfg(not(feature = "strict"))] Default::default()
                        }
                    };
                    let m_triggerManifoldTolerance = match m_triggerManifoldTolerance {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            #[cfg(feature = "strict")]
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "triggerManifoldTolerance",
                                ),
                            );
                            #[cfg(not(feature = "strict"))] Default::default()
                        }
                    };
                    let m_dynamicFriction = match m_dynamicFriction {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            #[cfg(feature = "strict")]
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "dynamicFriction",
                                ),
                            );
                            #[cfg(not(feature = "strict"))] Default::default()
                        }
                    };
                    let m_staticFriction = match m_staticFriction {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            #[cfg(feature = "strict")]
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "staticFriction",
                                ),
                            );
                            #[cfg(not(feature = "strict"))] Default::default()
                        }
                    };
                    let m_restitution = match m_restitution {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            #[cfg(feature = "strict")]
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "restitution",
                                ),
                            );
                            #[cfg(not(feature = "strict"))] Default::default()
                        }
                    };
                    let m_frictionCombinePolicy = match m_frictionCombinePolicy {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            #[cfg(feature = "strict")]
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "frictionCombinePolicy",
                                ),
                            );
                            #[cfg(not(feature = "strict"))] Default::default()
                        }
                    };
                    let m_restitutionCombinePolicy = match m_restitutionCombinePolicy {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            #[cfg(feature = "strict")]
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "restitutionCombinePolicy",
                                ),
                            );
                            #[cfg(not(feature = "strict"))] Default::default()
                        }
                    };
                    let m_weldingTolerance = match m_weldingTolerance {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            #[cfg(feature = "strict")]
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "weldingTolerance",
                                ),
                            );
                            #[cfg(not(feature = "strict"))] Default::default()
                        }
                    };
                    let m_maxContactImpulse = match m_maxContactImpulse {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            #[cfg(feature = "strict")]
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "maxContactImpulse",
                                ),
                            );
                            #[cfg(not(feature = "strict"))] Default::default()
                        }
                    };
                    let m_fractionOfClippedImpulseToApply = match m_fractionOfClippedImpulseToApply {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            #[cfg(feature = "strict")]
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "fractionOfClippedImpulseToApply",
                                ),
                            );
                            #[cfg(not(feature = "strict"))] Default::default()
                        }
                    };
                    let m_massChangerCategory = match m_massChangerCategory {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            #[cfg(feature = "strict")]
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "massChangerCategory",
                                ),
                            );
                            #[cfg(not(feature = "strict"))] Default::default()
                        }
                    };
                    let m_massChangerHeavyObjectFactor = match m_massChangerHeavyObjectFactor {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            #[cfg(feature = "strict")]
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "massChangerHeavyObjectFactor",
                                ),
                            );
                            #[cfg(not(feature = "strict"))] Default::default()
                        }
                    };
                    let m_softContactForceFactor = match m_softContactForceFactor {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            #[cfg(feature = "strict")]
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "softContactForceFactor",
                                ),
                            );
                            #[cfg(not(feature = "strict"))] Default::default()
                        }
                    };
                    let m_softContactDampFactor = match m_softContactDampFactor {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            #[cfg(feature = "strict")]
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "softContactDampFactor",
                                ),
                            );
                            #[cfg(not(feature = "strict"))] Default::default()
                        }
                    };
                    let m_softContactSeperationVelocity = match m_softContactSeperationVelocity {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            #[cfg(feature = "strict")]
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "softContactSeperationVelocity",
                                ),
                            );
                            #[cfg(not(feature = "strict"))] Default::default()
                        }
                    };
                    let m_surfaceVelocity = match m_surfaceVelocity {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            #[cfg(feature = "strict")]
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "surfaceVelocity",
                                ),
                            );
                            #[cfg(not(feature = "strict"))] Default::default()
                        }
                    };
                    let m_disablingCollisionsBetweenCvxCvxDynamicObjectsDistance = match m_disablingCollisionsBetweenCvxCvxDynamicObjectsDistance {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            #[cfg(feature = "strict")]
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "disablingCollisionsBetweenCvxCvxDynamicObjectsDistance",
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
                    let m_isShared = match m_isShared {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            #[cfg(feature = "strict")]
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field("isShared"),
                            );
                            #[cfg(not(feature = "strict"))] Default::default()
                        }
                    };
                    let __ptr = __A::class_ptr(&mut __map);
                    _serde::__private::Ok(hknpMaterial {
                        __ptr: __ptr.clone(),
                        m_name,
                        m_isExclusive,
                        m_flags,
                        m_triggerType,
                        m_triggerManifoldTolerance,
                        m_dynamicFriction,
                        m_staticFriction,
                        m_restitution,
                        m_frictionCombinePolicy,
                        m_restitutionCombinePolicy,
                        m_weldingTolerance,
                        m_maxContactImpulse,
                        m_fractionOfClippedImpulseToApply,
                        m_massChangerCategory,
                        m_massChangerHeavyObjectFactor,
                        m_softContactForceFactor,
                        m_softContactDampFactor,
                        m_softContactSeperationVelocity,
                        m_surfaceVelocity,
                        m_disablingCollisionsBetweenCvxCvxDynamicObjectsDistance,
                        m_userData,
                        m_isShared,
                    })
                }
            }
            const FIELDS: &[&str] = &[
                "name",
                "isExclusive",
                "flags",
                "triggerType",
                "triggerManifoldTolerance",
                "dynamicFriction",
                "staticFriction",
                "restitution",
                "frictionCombinePolicy",
                "restitutionCombinePolicy",
                "weldingTolerance",
                "maxContactImpulse",
                "fractionOfClippedImpulseToApply",
                "massChangerCategory",
                "massChangerHeavyObjectFactor",
                "softContactForceFactor",
                "softContactDampFactor",
                "softContactSeperationVelocity",
                "surfaceVelocity",
                "disablingCollisionsBetweenCvxCvxDynamicObjectsDistance",
                "userData",
                "isShared",
            ];
            _serde::Deserializer::deserialize_struct(
                deserializer,
                "hknpMaterial",
                FIELDS,
                __hknpMaterialVisitor {
                    marker: _serde::__private::PhantomData::<hknpMaterial>,
                    lifetime: _serde::__private::PhantomData,
                },
            )
        }
    }
};
/// # C++ Info
/// - name: `TriggerType`(ctype: `hkEnum<TriggerType, hkUint8>`)
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
pub enum TriggerType {
    #[default]
    TRIGGER_TYPE_NONE = 0isize,
    TRIGGER_TYPE_BROAD_PHASE = 1isize,
    TRIGGER_TYPE_NARROW_PHASE = 2isize,
    TRIGGER_TYPE_CONTACT_SOLVER = 3isize,
}
/// # C++ Info
/// - name: `CombinePolicy`(ctype: `hkEnum<CombinePolicy, hkUint8>`)
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
pub enum CombinePolicy {
    #[default]
    COMBINE_AVG = 0isize,
    COMBINE_MIN = 1isize,
    COMBINE_MAX = 2isize,
}
/// # C++ Info
/// - name: `MassChangerCategory`(ctype: `hkEnum<MassChangerCategory, hkUint8>`)
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
pub enum MassChangerCategory {
    #[default]
    MASS_CHANGER_IGNORE = 0isize,
    MASS_CHANGER_DEBRIS = 1isize,
    MASS_CHANGER_HEAVY = 2isize,
}
const _: () = {
    use havok_serde as __serde;
    impl __serde::Serialize for TriggerType {
        fn serialize<S>(&self, __serializer: S) -> Result<S::Ok, S::Error>
        where
            S: __serde::ser::Serializer,
        {
            let mut __serializer = __serializer.serialize_enum_flags()?;
            match self {
                Self::TRIGGER_TYPE_NONE => {
                    __serializer.serialize_field("TRIGGER_TYPE_NONE", &0u64)
                }
                Self::TRIGGER_TYPE_BROAD_PHASE => {
                    __serializer.serialize_field("TRIGGER_TYPE_BROAD_PHASE", &1u64)
                }
                Self::TRIGGER_TYPE_NARROW_PHASE => {
                    __serializer.serialize_field("TRIGGER_TYPE_NARROW_PHASE", &2u64)
                }
                Self::TRIGGER_TYPE_CONTACT_SOLVER => {
                    __serializer.serialize_field("TRIGGER_TYPE_CONTACT_SOLVER", &3u64)
                }
            }?;
            use num_traits::ToPrimitive as _;
            let num = self
                .to_u8()
                .ok_or(S::Error::custom("Failed enum TriggerType to_u8"))?;
            __serializer.serialize_bits(&num)?;
            __serializer.end()
        }
    }
};
const _: () = {
    use havok_serde as __serde;
    impl __serde::Serialize for CombinePolicy {
        fn serialize<S>(&self, __serializer: S) -> Result<S::Ok, S::Error>
        where
            S: __serde::ser::Serializer,
        {
            let mut __serializer = __serializer.serialize_enum_flags()?;
            match self {
                Self::COMBINE_AVG => __serializer.serialize_field("COMBINE_AVG", &0u64),
                Self::COMBINE_MIN => __serializer.serialize_field("COMBINE_MIN", &1u64),
                Self::COMBINE_MAX => __serializer.serialize_field("COMBINE_MAX", &2u64),
            }?;
            use num_traits::ToPrimitive as _;
            let num = self
                .to_u8()
                .ok_or(S::Error::custom("Failed enum CombinePolicy to_u8"))?;
            __serializer.serialize_bits(&num)?;
            __serializer.end()
        }
    }
};
const _: () = {
    use havok_serde as __serde;
    impl __serde::Serialize for MassChangerCategory {
        fn serialize<S>(&self, __serializer: S) -> Result<S::Ok, S::Error>
        where
            S: __serde::ser::Serializer,
        {
            let mut __serializer = __serializer.serialize_enum_flags()?;
            match self {
                Self::MASS_CHANGER_IGNORE => {
                    __serializer.serialize_field("MASS_CHANGER_IGNORE", &0u64)
                }
                Self::MASS_CHANGER_DEBRIS => {
                    __serializer.serialize_field("MASS_CHANGER_DEBRIS", &1u64)
                }
                Self::MASS_CHANGER_HEAVY => {
                    __serializer.serialize_field("MASS_CHANGER_HEAVY", &2u64)
                }
            }?;
            use num_traits::ToPrimitive as _;
            let num = self
                .to_u8()
                .ok_or(S::Error::custom("Failed enum MassChangerCategory to_u8"))?;
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
    impl<'de> _serde::Deserialize<'de> for TriggerType {
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
                __field3,
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
                fn visit_uint8<__E>(
                    self,
                    __value: U8<'de>,
                ) -> _serde::__private::Result<Self::Value, __E>
                where
                    __E: _serde::de::Error,
                {
                    match __value {
                        U8::Number(0u8) => _serde::__private::Ok(__Field::__field0),
                        U8::Number(1u8) => _serde::__private::Ok(__Field::__field1),
                        U8::Number(2u8) => _serde::__private::Ok(__Field::__field2),
                        U8::Number(3u8) => _serde::__private::Ok(__Field::__field3),
                        _ => {
                            _serde::__private::Err(
                                _serde::de::Error::invalid_value(
                                    _serde::de::Unexpected::Uint8(__value),
                                    &"value(u8) of variant is one of 0, 1, 2, 3",
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
                                || v.eq_ignore_ascii_case("TRIGGER_TYPE_NONE") => {
                                _serde::__private::Ok(__Field::__field0)
                            }
                            v if v == "1"
                                || v.eq_ignore_ascii_case("TRIGGER_TYPE_BROAD_PHASE") => {
                                _serde::__private::Ok(__Field::__field1)
                            }
                            v if v == "2"
                                || v.eq_ignore_ascii_case("TRIGGER_TYPE_NARROW_PHASE") => {
                                _serde::__private::Ok(__Field::__field2)
                            }
                            v if v == "3"
                                || v.eq_ignore_ascii_case("TRIGGER_TYPE_CONTACT_SOLVER") => {
                                _serde::__private::Ok(__Field::__field3)
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
                        _serde::de::ReadEnumSize::Uint8,
                        __FieldVisitor,
                    )
                }
            }
            #[doc(hidden)]
            struct __Visitor<'de> {
                marker: _serde::__private::PhantomData<TriggerType>,
                lifetime: _serde::__private::PhantomData<&'de ()>,
            }
            impl<'de> _serde::de::Visitor<'de> for __Visitor<'de> {
                type Value = TriggerType;
                fn expecting(
                    &self,
                    __formatter: &mut _serde::__private::Formatter,
                ) -> _serde::__private::fmt::Result {
                    _serde::__private::Formatter::write_str(
                        __formatter,
                        "enum TriggerType",
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
                            _serde::__private::Ok(TriggerType::TRIGGER_TYPE_NONE)
                        }
                        (__Field::__field1, __variant) => {
                            _serde::de::VariantAccess::unit_variant(__variant)?;
                            _serde::__private::Ok(TriggerType::TRIGGER_TYPE_BROAD_PHASE)
                        }
                        (__Field::__field2, __variant) => {
                            _serde::de::VariantAccess::unit_variant(__variant)?;
                            _serde::__private::Ok(TriggerType::TRIGGER_TYPE_NARROW_PHASE)
                        }
                        (__Field::__field3, __variant) => {
                            _serde::de::VariantAccess::unit_variant(__variant)?;
                            _serde::__private::Ok(
                                TriggerType::TRIGGER_TYPE_CONTACT_SOLVER,
                            )
                        }
                    }
                }
            }
            #[doc(hidden)]
            const VARIANTS: &'static [&'static str] = &[
                "TRIGGER_TYPE_NONE",
                "TRIGGER_TYPE_BROAD_PHASE",
                "TRIGGER_TYPE_NARROW_PHASE",
                "TRIGGER_TYPE_CONTACT_SOLVER",
            ];
            _serde::Deserializer::deserialize_enum(
                __deserializer,
                "TriggerType",
                VARIANTS,
                __Visitor {
                    marker: _serde::__private::PhantomData::<TriggerType>,
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
    impl<'de> _serde::Deserialize<'de> for CombinePolicy {
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
                fn visit_uint8<__E>(
                    self,
                    __value: U8<'de>,
                ) -> _serde::__private::Result<Self::Value, __E>
                where
                    __E: _serde::de::Error,
                {
                    match __value {
                        U8::Number(0u8) => _serde::__private::Ok(__Field::__field0),
                        U8::Number(1u8) => _serde::__private::Ok(__Field::__field1),
                        U8::Number(2u8) => _serde::__private::Ok(__Field::__field2),
                        _ => {
                            _serde::__private::Err(
                                _serde::de::Error::invalid_value(
                                    _serde::de::Unexpected::Uint8(__value),
                                    &"value(u8) of variant is one of 0, 1, 2",
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
                            v if v == "0" || v.eq_ignore_ascii_case("COMBINE_AVG") => {
                                _serde::__private::Ok(__Field::__field0)
                            }
                            v if v == "1" || v.eq_ignore_ascii_case("COMBINE_MIN") => {
                                _serde::__private::Ok(__Field::__field1)
                            }
                            v if v == "2" || v.eq_ignore_ascii_case("COMBINE_MAX") => {
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
                        _serde::de::ReadEnumSize::Uint8,
                        __FieldVisitor,
                    )
                }
            }
            #[doc(hidden)]
            struct __Visitor<'de> {
                marker: _serde::__private::PhantomData<CombinePolicy>,
                lifetime: _serde::__private::PhantomData<&'de ()>,
            }
            impl<'de> _serde::de::Visitor<'de> for __Visitor<'de> {
                type Value = CombinePolicy;
                fn expecting(
                    &self,
                    __formatter: &mut _serde::__private::Formatter,
                ) -> _serde::__private::fmt::Result {
                    _serde::__private::Formatter::write_str(
                        __formatter,
                        "enum CombinePolicy",
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
                            _serde::__private::Ok(CombinePolicy::COMBINE_AVG)
                        }
                        (__Field::__field1, __variant) => {
                            _serde::de::VariantAccess::unit_variant(__variant)?;
                            _serde::__private::Ok(CombinePolicy::COMBINE_MIN)
                        }
                        (__Field::__field2, __variant) => {
                            _serde::de::VariantAccess::unit_variant(__variant)?;
                            _serde::__private::Ok(CombinePolicy::COMBINE_MAX)
                        }
                    }
                }
            }
            #[doc(hidden)]
            const VARIANTS: &'static [&'static str] = &[
                "COMBINE_AVG",
                "COMBINE_MIN",
                "COMBINE_MAX",
            ];
            _serde::Deserializer::deserialize_enum(
                __deserializer,
                "CombinePolicy",
                VARIANTS,
                __Visitor {
                    marker: _serde::__private::PhantomData::<CombinePolicy>,
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
    impl<'de> _serde::Deserialize<'de> for MassChangerCategory {
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
                fn visit_uint8<__E>(
                    self,
                    __value: U8<'de>,
                ) -> _serde::__private::Result<Self::Value, __E>
                where
                    __E: _serde::de::Error,
                {
                    match __value {
                        U8::Number(0u8) => _serde::__private::Ok(__Field::__field0),
                        U8::Number(1u8) => _serde::__private::Ok(__Field::__field1),
                        U8::Number(2u8) => _serde::__private::Ok(__Field::__field2),
                        _ => {
                            _serde::__private::Err(
                                _serde::de::Error::invalid_value(
                                    _serde::de::Unexpected::Uint8(__value),
                                    &"value(u8) of variant is one of 0, 1, 2",
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
                                || v.eq_ignore_ascii_case("MASS_CHANGER_IGNORE") => {
                                _serde::__private::Ok(__Field::__field0)
                            }
                            v if v == "1"
                                || v.eq_ignore_ascii_case("MASS_CHANGER_DEBRIS") => {
                                _serde::__private::Ok(__Field::__field1)
                            }
                            v if v == "2"
                                || v.eq_ignore_ascii_case("MASS_CHANGER_HEAVY") => {
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
                        _serde::de::ReadEnumSize::Uint8,
                        __FieldVisitor,
                    )
                }
            }
            #[doc(hidden)]
            struct __Visitor<'de> {
                marker: _serde::__private::PhantomData<MassChangerCategory>,
                lifetime: _serde::__private::PhantomData<&'de ()>,
            }
            impl<'de> _serde::de::Visitor<'de> for __Visitor<'de> {
                type Value = MassChangerCategory;
                fn expecting(
                    &self,
                    __formatter: &mut _serde::__private::Formatter,
                ) -> _serde::__private::fmt::Result {
                    _serde::__private::Formatter::write_str(
                        __formatter,
                        "enum MassChangerCategory",
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
                            _serde::__private::Ok(
                                MassChangerCategory::MASS_CHANGER_IGNORE,
                            )
                        }
                        (__Field::__field1, __variant) => {
                            _serde::de::VariantAccess::unit_variant(__variant)?;
                            _serde::__private::Ok(
                                MassChangerCategory::MASS_CHANGER_DEBRIS,
                            )
                        }
                        (__Field::__field2, __variant) => {
                            _serde::de::VariantAccess::unit_variant(__variant)?;
                            _serde::__private::Ok(
                                MassChangerCategory::MASS_CHANGER_HEAVY,
                            )
                        }
                    }
                }
            }
            #[doc(hidden)]
            const VARIANTS: &'static [&'static str] = &[
                "MASS_CHANGER_IGNORE",
                "MASS_CHANGER_DEBRIS",
                "MASS_CHANGER_HEAVY",
            ];
            _serde::Deserializer::deserialize_enum(
                __deserializer,
                "MassChangerCategory",
                VARIANTS,
                __Visitor {
                    marker: _serde::__private::PhantomData::<MassChangerCategory>,
                    lifetime: _serde::__private::PhantomData,
                },
            )
        }
    }
};
