use super::class_requires::*;
use super::*;
/// # C++ Info
/// - name: `hknpMotion`
/// - version: `2`
/// - signature: `0xbaa216da`
/// - size: `  0`(x86)/`128`(x86_64)
/// -  vtable: `false`
#[allow(non_upper_case_globals, non_snake_case)]
#[cfg_attr(feature = "json_schema", derive(schemars::JsonSchema))]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(educe::Educe)]
#[educe(Debug, Clone, Default, PartialEq)]
pub struct hknpMotion<'a> {
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
    /// - name: `centerOfMassAndMassFactor`(ctype: `hkVector4`)
    /// - offset: `  0`(x86)/`  0`(x86_64)
    /// - type_size: `  0`(x86)/` 16`(x86_64)
    #[cfg_attr(feature = "serde_default", serde(default))]
    #[cfg_attr(feature = "json_schema", schemars(rename = "centerOfMassAndMassFactor"))]
    #[cfg_attr(feature = "serde", serde(rename = "centerOfMassAndMassFactor"))]
    pub m_centerOfMassAndMassFactor: Vector4,
    /// # C++ Info
    /// - name: `orientation`(ctype: `hkQuaternion`)
    /// - offset: `  0`(x86)/` 16`(x86_64)
    /// - type_size: `  0`(x86)/` 16`(x86_64)
    #[cfg_attr(feature = "serde_default", serde(default))]
    #[cfg_attr(feature = "json_schema", schemars(rename = "orientation"))]
    #[cfg_attr(feature = "serde", serde(rename = "orientation"))]
    pub m_orientation: Quaternion,
    /// # C++ Info
    /// - name: `inverseInertia`(ctype: `hkHalf[4]`)
    /// - offset: `  0`(x86)/` 32`(x86_64)
    /// - type_size: `  0`(x86)/`  8`(x86_64)
    #[cfg_attr(feature = "serde_default", serde(default))]
    #[cfg_attr(feature = "json_schema", schemars(rename = "inverseInertia"))]
    #[cfg_attr(feature = "serde", serde(rename = "inverseInertia"))]
    pub m_inverseInertia: [f16; 4usize],
    /// # C++ Info
    /// - name: `firstAttachedBodyId`(ctype: `hkUint32`)
    /// - offset: `  0`(x86)/` 40`(x86_64)
    /// - type_size: `  0`(x86)/`  4`(x86_64)
    #[cfg_attr(feature = "serde_default", serde(default))]
    #[cfg_attr(feature = "serde", serde(borrow))]
    #[cfg_attr(feature = "json_schema", schemars(rename = "firstAttachedBodyId"))]
    #[cfg_attr(feature = "serde", serde(rename = "firstAttachedBodyId"))]
    pub m_firstAttachedBodyId: U32<'a>,
    /// # C++ Info
    /// - name: `solverId`(ctype: `hkUint32`)
    /// - offset: `  0`(x86)/` 44`(x86_64)
    /// - type_size: `  0`(x86)/`  4`(x86_64)
    /// - flags: `SERIALIZE_IGNORED`
    #[cfg_attr(feature = "serde", serde(default))]
    #[cfg_attr(feature = "serde", serde(borrow))]
    #[cfg_attr(feature = "json_schema", schemars(rename = "solverId"))]
    #[cfg_attr(feature = "serde", serde(rename = "solverId"))]
    pub m_solverId: U32<'a>,
    /// # C++ Info
    /// - name: `linearVelocityCage`(ctype: `hkHalf[3]`)
    /// - offset: `  0`(x86)/` 48`(x86_64)
    /// - type_size: `  0`(x86)/`  6`(x86_64)
    /// - flags: `ALIGN_16`
    #[cfg_attr(feature = "serde_default", serde(default))]
    #[cfg_attr(feature = "json_schema", schemars(rename = "linearVelocityCage"))]
    #[cfg_attr(feature = "serde", serde(rename = "linearVelocityCage"))]
    pub m_linearVelocityCage: [f16; 3usize],
    /// # C++ Info
    /// - name: `integrationFactor`(ctype: `hkHalf`)
    /// - offset: `  0`(x86)/` 54`(x86_64)
    /// - type_size: `  0`(x86)/`  2`(x86_64)
    #[cfg_attr(feature = "serde_default", serde(default))]
    #[cfg_attr(feature = "json_schema", schemars(rename = "integrationFactor"))]
    #[cfg_attr(feature = "serde", serde(rename = "integrationFactor"))]
    pub m_integrationFactor: f16,
    /// # C++ Info
    /// - name: `motionPropertiesId`(ctype: `hkUint16`)
    /// - offset: `  0`(x86)/` 56`(x86_64)
    /// - type_size: `  0`(x86)/`  2`(x86_64)
    #[cfg_attr(feature = "serde_default", serde(default))]
    #[cfg_attr(feature = "serde", serde(borrow))]
    #[cfg_attr(feature = "json_schema", schemars(rename = "motionPropertiesId"))]
    #[cfg_attr(feature = "serde", serde(rename = "motionPropertiesId"))]
    pub m_motionPropertiesId: U16<'a>,
    /// # C++ Info
    /// - name: `maxLinearAccelerationDistancePerStep`(ctype: `hkHalf`)
    /// - offset: `  0`(x86)/` 58`(x86_64)
    /// - type_size: `  0`(x86)/`  2`(x86_64)
    #[cfg_attr(feature = "serde_default", serde(default))]
    #[cfg_attr(
        feature = "json_schema",
        schemars(rename = "maxLinearAccelerationDistancePerStep")
    )]
    #[cfg_attr(
        feature = "serde",
        serde(rename = "maxLinearAccelerationDistancePerStep")
    )]
    pub m_maxLinearAccelerationDistancePerStep: f16,
    /// # C++ Info
    /// - name: `maxRotationToPreventTunneling`(ctype: `hkHalf`)
    /// - offset: `  0`(x86)/` 60`(x86_64)
    /// - type_size: `  0`(x86)/`  2`(x86_64)
    #[cfg_attr(feature = "serde_default", serde(default))]
    #[cfg_attr(
        feature = "json_schema",
        schemars(rename = "maxRotationToPreventTunneling")
    )]
    #[cfg_attr(feature = "serde", serde(rename = "maxRotationToPreventTunneling"))]
    pub m_maxRotationToPreventTunneling: f16,
    /// # C++ Info
    /// - name: `cellIndex`(ctype: `hkUint8`)
    /// - offset: `  0`(x86)/` 62`(x86_64)
    /// - type_size: `  0`(x86)/`  1`(x86_64)
    #[cfg_attr(feature = "serde_default", serde(default))]
    #[cfg_attr(feature = "serde", serde(borrow))]
    #[cfg_attr(feature = "json_schema", schemars(rename = "cellIndex"))]
    #[cfg_attr(feature = "serde", serde(rename = "cellIndex"))]
    pub m_cellIndex: U8<'a>,
    /// # C++ Info
    /// - name: `spaceSplitterWeight`(ctype: `hkUint8`)
    /// - offset: `  0`(x86)/` 63`(x86_64)
    /// - type_size: `  0`(x86)/`  1`(x86_64)
    #[cfg_attr(feature = "serde_default", serde(default))]
    #[cfg_attr(feature = "serde", serde(borrow))]
    #[cfg_attr(feature = "json_schema", schemars(rename = "spaceSplitterWeight"))]
    #[cfg_attr(feature = "serde", serde(rename = "spaceSplitterWeight"))]
    pub m_spaceSplitterWeight: U8<'a>,
    /// # C++ Info
    /// - name: `linearVelocity`(ctype: `hkVector4`)
    /// - offset: `  0`(x86)/` 64`(x86_64)
    /// - type_size: `  0`(x86)/` 16`(x86_64)
    #[cfg_attr(feature = "serde_default", serde(default))]
    #[cfg_attr(feature = "json_schema", schemars(rename = "linearVelocity"))]
    #[cfg_attr(feature = "serde", serde(rename = "linearVelocity"))]
    pub m_linearVelocity: Vector4,
    /// # C++ Info
    /// - name: `angularVelocity`(ctype: `hkVector4`)
    /// - offset: `  0`(x86)/` 80`(x86_64)
    /// - type_size: `  0`(x86)/` 16`(x86_64)
    #[cfg_attr(feature = "serde_default", serde(default))]
    #[cfg_attr(feature = "json_schema", schemars(rename = "angularVelocity"))]
    #[cfg_attr(feature = "serde", serde(rename = "angularVelocity"))]
    pub m_angularVelocity: Vector4,
    /// # C++ Info
    /// - name: `previousStepLinearVelocity`(ctype: `hkVector4`)
    /// - offset: `  0`(x86)/` 96`(x86_64)
    /// - type_size: `  0`(x86)/` 16`(x86_64)
    #[cfg_attr(feature = "serde_default", serde(default))]
    #[cfg_attr(feature = "json_schema", schemars(rename = "previousStepLinearVelocity"))]
    #[cfg_attr(feature = "serde", serde(rename = "previousStepLinearVelocity"))]
    pub m_previousStepLinearVelocity: Vector4,
    /// # C++ Info
    /// - name: `previousStepAngularVelocity`(ctype: `hkVector4`)
    /// - offset: `  0`(x86)/`112`(x86_64)
    /// - type_size: `  0`(x86)/` 16`(x86_64)
    #[cfg_attr(feature = "serde_default", serde(default))]
    #[cfg_attr(
        feature = "json_schema",
        schemars(rename = "previousStepAngularVelocity")
    )]
    #[cfg_attr(feature = "serde", serde(rename = "previousStepAngularVelocity"))]
    pub m_previousStepAngularVelocity: Vector4,
}
const _: () = {
    use havok_serde as _serde;
    impl<'a> _serde::HavokClass for hknpMotion<'a> {
        #[inline]
        fn name(&self) -> &'static str {
            "hknpMotion"
        }
        #[inline]
        fn signature(&self) -> _serde::__private::Signature {
            _serde::__private::Signature::new(0xbaa216da)
        }
        #[allow(clippy::let_and_return, clippy::vec_init_then_push)]
        fn deps_indexes(&self) -> Vec<&Pointer<'_>> {
            let mut v = Vec::new();
            v
        }
    }
    impl<'a> _serde::Serialize for hknpMotion<'a> {
        fn serialize<S>(&self, __serializer: S) -> Result<S::Ok, S::Error>
        where
            S: _serde::ser::Serializer,
        {
            let class_meta = self
                .__ptr
                .as_ref()
                .map(|name| (name, _serde::__private::Signature::new(0xbaa216da)));
            let mut serializer = __serializer
                .serialize_struct("hknpMotion", class_meta, (0u64, 128u64))?;
            serializer
                .serialize_field(
                    "centerOfMassAndMassFactor",
                    &self.m_centerOfMassAndMassFactor,
                )?;
            serializer.serialize_field("orientation", &self.m_orientation)?;
            serializer
                .serialize_fixed_array_field(
                    "inverseInertia",
                    self.m_inverseInertia.as_slice(),
                    TypeSize::NonPtr,
                )?;
            serializer
                .serialize_field("firstAttachedBodyId", &self.m_firstAttachedBodyId)?;
            serializer.skip_field("solverId", &self.m_solverId)?;
            serializer
                .serialize_fixed_array_field(
                    "linearVelocityCage",
                    self.m_linearVelocityCage.as_slice(),
                    TypeSize::NonPtr,
                )?;
            serializer.serialize_field("integrationFactor", &self.m_integrationFactor)?;
            serializer
                .serialize_field("motionPropertiesId", &self.m_motionPropertiesId)?;
            serializer
                .serialize_field(
                    "maxLinearAccelerationDistancePerStep",
                    &self.m_maxLinearAccelerationDistancePerStep,
                )?;
            serializer
                .serialize_field(
                    "maxRotationToPreventTunneling",
                    &self.m_maxRotationToPreventTunneling,
                )?;
            serializer.serialize_field("cellIndex", &self.m_cellIndex)?;
            serializer
                .serialize_field("spaceSplitterWeight", &self.m_spaceSplitterWeight)?;
            serializer.serialize_field("linearVelocity", &self.m_linearVelocity)?;
            serializer.serialize_field("angularVelocity", &self.m_angularVelocity)?;
            serializer
                .serialize_field(
                    "previousStepLinearVelocity",
                    &self.m_previousStepLinearVelocity,
                )?;
            serializer
                .serialize_field(
                    "previousStepAngularVelocity",
                    &self.m_previousStepAngularVelocity,
                )?;
            serializer.end()
        }
    }
};
#[doc(hidden)]
#[allow(non_upper_case_globals, unused_attributes, unused_qualifications)]
const _: () = {
    use havok_serde as _serde;
    #[automatically_derived]
    impl<'de> _serde::Deserialize<'de> for hknpMotion<'de> {
        fn deserialize<__D>(deserializer: __D) -> core::result::Result<Self, __D::Error>
        where
            __D: _serde::Deserializer<'de>,
        {
            #[allow(non_camel_case_types)]
            enum __Field {
                m_centerOfMassAndMassFactor,
                m_orientation,
                m_inverseInertia,
                m_firstAttachedBodyId,
                m_linearVelocityCage,
                m_integrationFactor,
                m_motionPropertiesId,
                m_maxLinearAccelerationDistancePerStep,
                m_maxRotationToPreventTunneling,
                m_cellIndex,
                m_spaceSplitterWeight,
                m_linearVelocity,
                m_angularVelocity,
                m_previousStepLinearVelocity,
                m_previousStepAngularVelocity,
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
                        "centerOfMassAndMassFactor" => {
                            Ok(__Field::m_centerOfMassAndMassFactor)
                        }
                        "orientation" => Ok(__Field::m_orientation),
                        "inverseInertia" => Ok(__Field::m_inverseInertia),
                        "firstAttachedBodyId" => Ok(__Field::m_firstAttachedBodyId),
                        "linearVelocityCage" => Ok(__Field::m_linearVelocityCage),
                        "integrationFactor" => Ok(__Field::m_integrationFactor),
                        "motionPropertiesId" => Ok(__Field::m_motionPropertiesId),
                        "maxLinearAccelerationDistancePerStep" => {
                            Ok(__Field::m_maxLinearAccelerationDistancePerStep)
                        }
                        "maxRotationToPreventTunneling" => {
                            Ok(__Field::m_maxRotationToPreventTunneling)
                        }
                        "cellIndex" => Ok(__Field::m_cellIndex),
                        "spaceSplitterWeight" => Ok(__Field::m_spaceSplitterWeight),
                        "linearVelocity" => Ok(__Field::m_linearVelocity),
                        "angularVelocity" => Ok(__Field::m_angularVelocity),
                        "previousStepLinearVelocity" => {
                            Ok(__Field::m_previousStepLinearVelocity)
                        }
                        "previousStepAngularVelocity" => {
                            Ok(__Field::m_previousStepAngularVelocity)
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
            struct __hknpMotionVisitor<'de> {
                marker: _serde::__private::PhantomData<hknpMotion<'de>>,
                lifetime: _serde::__private::PhantomData<&'de ()>,
            }
            #[allow(clippy::match_single_binding)]
            #[allow(clippy::reversed_empty_ranges)]
            #[allow(clippy::single_match)]
            impl<'de> _serde::de::Visitor<'de> for __hknpMotionVisitor<'de> {
                type Value = hknpMotion<'de>;
                fn expecting(
                    &self,
                    __formatter: &mut core::fmt::Formatter,
                ) -> core::fmt::Result {
                    core::fmt::Formatter::write_str(__formatter, "struct hknpMotion")
                }
                fn visit_struct_for_bytes<__A>(
                    self,
                    mut __map: __A,
                ) -> _serde::__private::Result<Self::Value, __A::Error>
                where
                    __A: _serde::de::MapAccess<'de>,
                {
                    let __ptr = __A::class_ptr(&mut __map);
                    let mut m_centerOfMassAndMassFactor: _serde::__private::Option<
                        Vector4,
                    > = _serde::__private::None;
                    let mut m_orientation: _serde::__private::Option<Quaternion> = _serde::__private::None;
                    let mut m_inverseInertia: _serde::__private::Option<[f16; 4usize]> = _serde::__private::None;
                    let mut m_firstAttachedBodyId: _serde::__private::Option<U32<'de>> = _serde::__private::None;
                    let mut m_solverId: _serde::__private::Option<U32<'de>> = _serde::__private::None;
                    let mut m_linearVelocityCage: _serde::__private::Option<
                        [f16; 3usize],
                    > = _serde::__private::None;
                    let mut m_integrationFactor: _serde::__private::Option<f16> = _serde::__private::None;
                    let mut m_motionPropertiesId: _serde::__private::Option<U16<'de>> = _serde::__private::None;
                    let mut m_maxLinearAccelerationDistancePerStep: _serde::__private::Option<
                        f16,
                    > = _serde::__private::None;
                    let mut m_maxRotationToPreventTunneling: _serde::__private::Option<
                        f16,
                    > = _serde::__private::None;
                    let mut m_cellIndex: _serde::__private::Option<U8<'de>> = _serde::__private::None;
                    let mut m_spaceSplitterWeight: _serde::__private::Option<U8<'de>> = _serde::__private::None;
                    let mut m_linearVelocity: _serde::__private::Option<Vector4> = _serde::__private::None;
                    let mut m_angularVelocity: _serde::__private::Option<Vector4> = _serde::__private::None;
                    let mut m_previousStepLinearVelocity: _serde::__private::Option<
                        Vector4,
                    > = _serde::__private::None;
                    let mut m_previousStepAngularVelocity: _serde::__private::Option<
                        Vector4,
                    > = _serde::__private::None;
                    for i in 0..16usize {
                        match i {
                            0usize => {
                                if _serde::__private::Option::is_some(
                                    &m_centerOfMassAndMassFactor,
                                ) {
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "centerOfMassAndMassFactor",
                                        ),
                                    );
                                }
                                m_centerOfMassAndMassFactor = _serde::__private::Some(
                                    match __A::next_value::<Vector4>(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            1usize => {
                                if _serde::__private::Option::is_some(&m_orientation) {
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "orientation",
                                        ),
                                    );
                                }
                                m_orientation = _serde::__private::Some(
                                    match __A::next_value::<Quaternion>(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            2usize => {
                                if _serde::__private::Option::is_some(&m_inverseInertia) {
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "inverseInertia",
                                        ),
                                    );
                                }
                                m_inverseInertia = _serde::__private::Some(
                                    match __A::next_value::<[f16; 4usize]>(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            3usize => {
                                if _serde::__private::Option::is_some(
                                    &m_firstAttachedBodyId,
                                ) {
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "firstAttachedBodyId",
                                        ),
                                    );
                                }
                                m_firstAttachedBodyId = _serde::__private::Some(
                                    match __A::next_value::<U32<'de>>(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            4usize => {
                                if _serde::__private::Option::is_some(&m_solverId) {
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "solverId",
                                        ),
                                    );
                                }
                                m_solverId = _serde::__private::Some(
                                    match __A::next_value::<U32<'de>>(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            5usize => {
                                if _serde::__private::Option::is_some(
                                    &m_linearVelocityCage,
                                ) {
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "linearVelocityCage",
                                        ),
                                    );
                                }
                                m_linearVelocityCage = _serde::__private::Some(
                                    match __A::next_value::<[f16; 3usize]>(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            6usize => {
                                if _serde::__private::Option::is_some(
                                    &m_integrationFactor,
                                ) {
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "integrationFactor",
                                        ),
                                    );
                                }
                                m_integrationFactor = _serde::__private::Some(
                                    match __A::next_value::<f16>(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            7usize => {
                                if _serde::__private::Option::is_some(
                                    &m_motionPropertiesId,
                                ) {
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "motionPropertiesId",
                                        ),
                                    );
                                }
                                m_motionPropertiesId = _serde::__private::Some(
                                    match __A::next_value::<U16<'de>>(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            8usize => {
                                if _serde::__private::Option::is_some(
                                    &m_maxLinearAccelerationDistancePerStep,
                                ) {
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "maxLinearAccelerationDistancePerStep",
                                        ),
                                    );
                                }
                                m_maxLinearAccelerationDistancePerStep = _serde::__private::Some(
                                    match __A::next_value::<f16>(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            9usize => {
                                if _serde::__private::Option::is_some(
                                    &m_maxRotationToPreventTunneling,
                                ) {
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "maxRotationToPreventTunneling",
                                        ),
                                    );
                                }
                                m_maxRotationToPreventTunneling = _serde::__private::Some(
                                    match __A::next_value::<f16>(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            10usize => {
                                if _serde::__private::Option::is_some(&m_cellIndex) {
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "cellIndex",
                                        ),
                                    );
                                }
                                m_cellIndex = _serde::__private::Some(
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
                                    &m_spaceSplitterWeight,
                                ) {
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "spaceSplitterWeight",
                                        ),
                                    );
                                }
                                m_spaceSplitterWeight = _serde::__private::Some(
                                    match __A::next_value::<U8<'de>>(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            12usize => {
                                if _serde::__private::Option::is_some(&m_linearVelocity) {
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "linearVelocity",
                                        ),
                                    );
                                }
                                m_linearVelocity = _serde::__private::Some(
                                    match __A::next_value::<Vector4>(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            13usize => {
                                if _serde::__private::Option::is_some(&m_angularVelocity) {
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "angularVelocity",
                                        ),
                                    );
                                }
                                m_angularVelocity = _serde::__private::Some(
                                    match __A::next_value::<Vector4>(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            14usize => {
                                if _serde::__private::Option::is_some(
                                    &m_previousStepLinearVelocity,
                                ) {
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "previousStepLinearVelocity",
                                        ),
                                    );
                                }
                                m_previousStepLinearVelocity = _serde::__private::Some(
                                    match __A::next_value::<Vector4>(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            15usize => {
                                if _serde::__private::Option::is_some(
                                    &m_previousStepAngularVelocity,
                                ) {
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "previousStepAngularVelocity",
                                        ),
                                    );
                                }
                                m_previousStepAngularVelocity = _serde::__private::Some(
                                    match __A::next_value::<Vector4>(&mut __map) {
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
                    let m_centerOfMassAndMassFactor = match m_centerOfMassAndMassFactor {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "centerOfMassAndMassFactor",
                                ),
                            );
                        }
                    };
                    let m_orientation = match m_orientation {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "orientation",
                                ),
                            );
                        }
                    };
                    let m_inverseInertia = match m_inverseInertia {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "inverseInertia",
                                ),
                            );
                        }
                    };
                    let m_firstAttachedBodyId = match m_firstAttachedBodyId {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "firstAttachedBodyId",
                                ),
                            );
                        }
                    };
                    let m_solverId = match m_solverId {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field("solverId"),
                            );
                        }
                    };
                    let m_linearVelocityCage = match m_linearVelocityCage {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "linearVelocityCage",
                                ),
                            );
                        }
                    };
                    let m_integrationFactor = match m_integrationFactor {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "integrationFactor",
                                ),
                            );
                        }
                    };
                    let m_motionPropertiesId = match m_motionPropertiesId {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "motionPropertiesId",
                                ),
                            );
                        }
                    };
                    let m_maxLinearAccelerationDistancePerStep = match m_maxLinearAccelerationDistancePerStep {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "maxLinearAccelerationDistancePerStep",
                                ),
                            );
                        }
                    };
                    let m_maxRotationToPreventTunneling = match m_maxRotationToPreventTunneling {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "maxRotationToPreventTunneling",
                                ),
                            );
                        }
                    };
                    let m_cellIndex = match m_cellIndex {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "cellIndex",
                                ),
                            );
                        }
                    };
                    let m_spaceSplitterWeight = match m_spaceSplitterWeight {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "spaceSplitterWeight",
                                ),
                            );
                        }
                    };
                    let m_linearVelocity = match m_linearVelocity {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "linearVelocity",
                                ),
                            );
                        }
                    };
                    let m_angularVelocity = match m_angularVelocity {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "angularVelocity",
                                ),
                            );
                        }
                    };
                    let m_previousStepLinearVelocity = match m_previousStepLinearVelocity {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "previousStepLinearVelocity",
                                ),
                            );
                        }
                    };
                    let m_previousStepAngularVelocity = match m_previousStepAngularVelocity {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "previousStepAngularVelocity",
                                ),
                            );
                        }
                    };
                    _serde::__private::Ok(hknpMotion {
                        __ptr,
                        m_centerOfMassAndMassFactor,
                        m_orientation,
                        m_inverseInertia,
                        m_firstAttachedBodyId,
                        m_solverId,
                        m_linearVelocityCage,
                        m_integrationFactor,
                        m_motionPropertiesId,
                        m_maxLinearAccelerationDistancePerStep,
                        m_maxRotationToPreventTunneling,
                        m_cellIndex,
                        m_spaceSplitterWeight,
                        m_linearVelocity,
                        m_angularVelocity,
                        m_previousStepLinearVelocity,
                        m_previousStepAngularVelocity,
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
                    let mut m_centerOfMassAndMassFactor: _serde::__private::Option<
                        Vector4,
                    > = _serde::__private::None;
                    let mut m_orientation: _serde::__private::Option<Quaternion> = _serde::__private::None;
                    let mut m_inverseInertia: _serde::__private::Option<[f16; 4usize]> = _serde::__private::None;
                    let mut m_firstAttachedBodyId: _serde::__private::Option<U32<'de>> = _serde::__private::None;
                    let mut m_linearVelocityCage: _serde::__private::Option<
                        [f16; 3usize],
                    > = _serde::__private::None;
                    let mut m_integrationFactor: _serde::__private::Option<f16> = _serde::__private::None;
                    let mut m_motionPropertiesId: _serde::__private::Option<U16<'de>> = _serde::__private::None;
                    let mut m_maxLinearAccelerationDistancePerStep: _serde::__private::Option<
                        f16,
                    > = _serde::__private::None;
                    let mut m_maxRotationToPreventTunneling: _serde::__private::Option<
                        f16,
                    > = _serde::__private::None;
                    let mut m_cellIndex: _serde::__private::Option<U8<'de>> = _serde::__private::None;
                    let mut m_spaceSplitterWeight: _serde::__private::Option<U8<'de>> = _serde::__private::None;
                    let mut m_linearVelocity: _serde::__private::Option<Vector4> = _serde::__private::None;
                    let mut m_angularVelocity: _serde::__private::Option<Vector4> = _serde::__private::None;
                    let mut m_previousStepLinearVelocity: _serde::__private::Option<
                        Vector4,
                    > = _serde::__private::None;
                    let mut m_previousStepAngularVelocity: _serde::__private::Option<
                        Vector4,
                    > = _serde::__private::None;
                    while let _serde::__private::Some(__key) = {
                        __A::next_key::<__Field>(&mut __map)?
                    } {
                        match __key {
                            __Field::m_centerOfMassAndMassFactor => {
                                #[cfg(
                                    any(feature = "strict", feature = "ignore_duplicates")
                                )]
                                if _serde::__private::Option::is_some(
                                    &m_centerOfMassAndMassFactor,
                                ) {
                                    #[cfg(feature = "ignore_duplicates")]
                                    {
                                        __A::skip_value(&mut __map)?;
                                        continue;
                                    }
                                    #[cfg(feature = "strict")]
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "centerOfMassAndMassFactor",
                                        ),
                                    );
                                }
                                m_centerOfMassAndMassFactor = _serde::__private::Some(
                                    match __A::next_value::<Vector4>(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            __Field::m_orientation => {
                                #[cfg(
                                    any(feature = "strict", feature = "ignore_duplicates")
                                )]
                                if _serde::__private::Option::is_some(&m_orientation) {
                                    #[cfg(feature = "ignore_duplicates")]
                                    {
                                        __A::skip_value(&mut __map)?;
                                        continue;
                                    }
                                    #[cfg(feature = "strict")]
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "orientation",
                                        ),
                                    );
                                }
                                m_orientation = _serde::__private::Some(
                                    match __A::next_value::<Quaternion>(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            __Field::m_inverseInertia => {
                                #[cfg(
                                    any(feature = "strict", feature = "ignore_duplicates")
                                )]
                                if _serde::__private::Option::is_some(&m_inverseInertia) {
                                    #[cfg(feature = "ignore_duplicates")]
                                    {
                                        __A::skip_value(&mut __map)?;
                                        continue;
                                    }
                                    #[cfg(feature = "strict")]
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "inverseInertia",
                                        ),
                                    );
                                }
                                m_inverseInertia = _serde::__private::Some(
                                    match __A::next_value::<[f16; 4usize]>(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            __Field::m_firstAttachedBodyId => {
                                #[cfg(
                                    any(feature = "strict", feature = "ignore_duplicates")
                                )]
                                if _serde::__private::Option::is_some(
                                    &m_firstAttachedBodyId,
                                ) {
                                    #[cfg(feature = "ignore_duplicates")]
                                    {
                                        __A::skip_value(&mut __map)?;
                                        continue;
                                    }
                                    #[cfg(feature = "strict")]
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "firstAttachedBodyId",
                                        ),
                                    );
                                }
                                m_firstAttachedBodyId = _serde::__private::Some(
                                    match __A::next_value::<U32<'de>>(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            __Field::m_linearVelocityCage => {
                                #[cfg(
                                    any(feature = "strict", feature = "ignore_duplicates")
                                )]
                                if _serde::__private::Option::is_some(
                                    &m_linearVelocityCage,
                                ) {
                                    #[cfg(feature = "ignore_duplicates")]
                                    {
                                        __A::skip_value(&mut __map)?;
                                        continue;
                                    }
                                    #[cfg(feature = "strict")]
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "linearVelocityCage",
                                        ),
                                    );
                                }
                                m_linearVelocityCage = _serde::__private::Some(
                                    match __A::next_value::<[f16; 3usize]>(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            __Field::m_integrationFactor => {
                                #[cfg(
                                    any(feature = "strict", feature = "ignore_duplicates")
                                )]
                                if _serde::__private::Option::is_some(
                                    &m_integrationFactor,
                                ) {
                                    #[cfg(feature = "ignore_duplicates")]
                                    {
                                        __A::skip_value(&mut __map)?;
                                        continue;
                                    }
                                    #[cfg(feature = "strict")]
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "integrationFactor",
                                        ),
                                    );
                                }
                                m_integrationFactor = _serde::__private::Some(
                                    match __A::next_value::<f16>(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            __Field::m_motionPropertiesId => {
                                #[cfg(
                                    any(feature = "strict", feature = "ignore_duplicates")
                                )]
                                if _serde::__private::Option::is_some(
                                    &m_motionPropertiesId,
                                ) {
                                    #[cfg(feature = "ignore_duplicates")]
                                    {
                                        __A::skip_value(&mut __map)?;
                                        continue;
                                    }
                                    #[cfg(feature = "strict")]
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "motionPropertiesId",
                                        ),
                                    );
                                }
                                m_motionPropertiesId = _serde::__private::Some(
                                    match __A::next_value::<U16<'de>>(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            __Field::m_maxLinearAccelerationDistancePerStep => {
                                #[cfg(
                                    any(feature = "strict", feature = "ignore_duplicates")
                                )]
                                if _serde::__private::Option::is_some(
                                    &m_maxLinearAccelerationDistancePerStep,
                                ) {
                                    #[cfg(feature = "ignore_duplicates")]
                                    {
                                        __A::skip_value(&mut __map)?;
                                        continue;
                                    }
                                    #[cfg(feature = "strict")]
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "maxLinearAccelerationDistancePerStep",
                                        ),
                                    );
                                }
                                m_maxLinearAccelerationDistancePerStep = _serde::__private::Some(
                                    match __A::next_value::<f16>(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            __Field::m_maxRotationToPreventTunneling => {
                                #[cfg(
                                    any(feature = "strict", feature = "ignore_duplicates")
                                )]
                                if _serde::__private::Option::is_some(
                                    &m_maxRotationToPreventTunneling,
                                ) {
                                    #[cfg(feature = "ignore_duplicates")]
                                    {
                                        __A::skip_value(&mut __map)?;
                                        continue;
                                    }
                                    #[cfg(feature = "strict")]
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "maxRotationToPreventTunneling",
                                        ),
                                    );
                                }
                                m_maxRotationToPreventTunneling = _serde::__private::Some(
                                    match __A::next_value::<f16>(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            __Field::m_cellIndex => {
                                #[cfg(
                                    any(feature = "strict", feature = "ignore_duplicates")
                                )]
                                if _serde::__private::Option::is_some(&m_cellIndex) {
                                    #[cfg(feature = "ignore_duplicates")]
                                    {
                                        __A::skip_value(&mut __map)?;
                                        continue;
                                    }
                                    #[cfg(feature = "strict")]
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "cellIndex",
                                        ),
                                    );
                                }
                                m_cellIndex = _serde::__private::Some(
                                    match __A::next_value::<U8<'de>>(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            __Field::m_spaceSplitterWeight => {
                                #[cfg(
                                    any(feature = "strict", feature = "ignore_duplicates")
                                )]
                                if _serde::__private::Option::is_some(
                                    &m_spaceSplitterWeight,
                                ) {
                                    #[cfg(feature = "ignore_duplicates")]
                                    {
                                        __A::skip_value(&mut __map)?;
                                        continue;
                                    }
                                    #[cfg(feature = "strict")]
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "spaceSplitterWeight",
                                        ),
                                    );
                                }
                                m_spaceSplitterWeight = _serde::__private::Some(
                                    match __A::next_value::<U8<'de>>(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            __Field::m_linearVelocity => {
                                #[cfg(
                                    any(feature = "strict", feature = "ignore_duplicates")
                                )]
                                if _serde::__private::Option::is_some(&m_linearVelocity) {
                                    #[cfg(feature = "ignore_duplicates")]
                                    {
                                        __A::skip_value(&mut __map)?;
                                        continue;
                                    }
                                    #[cfg(feature = "strict")]
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "linearVelocity",
                                        ),
                                    );
                                }
                                m_linearVelocity = _serde::__private::Some(
                                    match __A::next_value::<Vector4>(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            __Field::m_angularVelocity => {
                                #[cfg(
                                    any(feature = "strict", feature = "ignore_duplicates")
                                )]
                                if _serde::__private::Option::is_some(&m_angularVelocity) {
                                    #[cfg(feature = "ignore_duplicates")]
                                    {
                                        __A::skip_value(&mut __map)?;
                                        continue;
                                    }
                                    #[cfg(feature = "strict")]
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "angularVelocity",
                                        ),
                                    );
                                }
                                m_angularVelocity = _serde::__private::Some(
                                    match __A::next_value::<Vector4>(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            __Field::m_previousStepLinearVelocity => {
                                #[cfg(
                                    any(feature = "strict", feature = "ignore_duplicates")
                                )]
                                if _serde::__private::Option::is_some(
                                    &m_previousStepLinearVelocity,
                                ) {
                                    #[cfg(feature = "ignore_duplicates")]
                                    {
                                        __A::skip_value(&mut __map)?;
                                        continue;
                                    }
                                    #[cfg(feature = "strict")]
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "previousStepLinearVelocity",
                                        ),
                                    );
                                }
                                m_previousStepLinearVelocity = _serde::__private::Some(
                                    match __A::next_value::<Vector4>(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            __Field::m_previousStepAngularVelocity => {
                                #[cfg(
                                    any(feature = "strict", feature = "ignore_duplicates")
                                )]
                                if _serde::__private::Option::is_some(
                                    &m_previousStepAngularVelocity,
                                ) {
                                    #[cfg(feature = "ignore_duplicates")]
                                    {
                                        __A::skip_value(&mut __map)?;
                                        continue;
                                    }
                                    #[cfg(feature = "strict")]
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "previousStepAngularVelocity",
                                        ),
                                    );
                                }
                                m_previousStepAngularVelocity = _serde::__private::Some(
                                    match __A::next_value::<Vector4>(&mut __map) {
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
                    let m_centerOfMassAndMassFactor = match m_centerOfMassAndMassFactor {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            #[cfg(feature = "strict")]
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "centerOfMassAndMassFactor",
                                ),
                            );
                            #[cfg(not(feature = "strict"))] Default::default()
                        }
                    };
                    let m_orientation = match m_orientation {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            #[cfg(feature = "strict")]
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "orientation",
                                ),
                            );
                            #[cfg(not(feature = "strict"))] Default::default()
                        }
                    };
                    let m_inverseInertia = match m_inverseInertia {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            #[cfg(feature = "strict")]
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "inverseInertia",
                                ),
                            );
                            #[cfg(not(feature = "strict"))] Default::default()
                        }
                    };
                    let m_firstAttachedBodyId = match m_firstAttachedBodyId {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            #[cfg(feature = "strict")]
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "firstAttachedBodyId",
                                ),
                            );
                            #[cfg(not(feature = "strict"))] Default::default()
                        }
                    };
                    let m_linearVelocityCage = match m_linearVelocityCage {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            #[cfg(feature = "strict")]
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "linearVelocityCage",
                                ),
                            );
                            #[cfg(not(feature = "strict"))] Default::default()
                        }
                    };
                    let m_integrationFactor = match m_integrationFactor {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            #[cfg(feature = "strict")]
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "integrationFactor",
                                ),
                            );
                            #[cfg(not(feature = "strict"))] Default::default()
                        }
                    };
                    let m_motionPropertiesId = match m_motionPropertiesId {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            #[cfg(feature = "strict")]
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "motionPropertiesId",
                                ),
                            );
                            #[cfg(not(feature = "strict"))] Default::default()
                        }
                    };
                    let m_maxLinearAccelerationDistancePerStep = match m_maxLinearAccelerationDistancePerStep {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            #[cfg(feature = "strict")]
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "maxLinearAccelerationDistancePerStep",
                                ),
                            );
                            #[cfg(not(feature = "strict"))] Default::default()
                        }
                    };
                    let m_maxRotationToPreventTunneling = match m_maxRotationToPreventTunneling {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            #[cfg(feature = "strict")]
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "maxRotationToPreventTunneling",
                                ),
                            );
                            #[cfg(not(feature = "strict"))] Default::default()
                        }
                    };
                    let m_cellIndex = match m_cellIndex {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            #[cfg(feature = "strict")]
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "cellIndex",
                                ),
                            );
                            #[cfg(not(feature = "strict"))] Default::default()
                        }
                    };
                    let m_spaceSplitterWeight = match m_spaceSplitterWeight {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            #[cfg(feature = "strict")]
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "spaceSplitterWeight",
                                ),
                            );
                            #[cfg(not(feature = "strict"))] Default::default()
                        }
                    };
                    let m_linearVelocity = match m_linearVelocity {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            #[cfg(feature = "strict")]
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "linearVelocity",
                                ),
                            );
                            #[cfg(not(feature = "strict"))] Default::default()
                        }
                    };
                    let m_angularVelocity = match m_angularVelocity {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            #[cfg(feature = "strict")]
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "angularVelocity",
                                ),
                            );
                            #[cfg(not(feature = "strict"))] Default::default()
                        }
                    };
                    let m_previousStepLinearVelocity = match m_previousStepLinearVelocity {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            #[cfg(feature = "strict")]
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "previousStepLinearVelocity",
                                ),
                            );
                            #[cfg(not(feature = "strict"))] Default::default()
                        }
                    };
                    let m_previousStepAngularVelocity = match m_previousStepAngularVelocity {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            #[cfg(feature = "strict")]
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "previousStepAngularVelocity",
                                ),
                            );
                            #[cfg(not(feature = "strict"))] Default::default()
                        }
                    };
                    let __ptr = __A::class_ptr(&mut __map);
                    _serde::__private::Ok(hknpMotion {
                        __ptr: __ptr.clone(),
                        m_centerOfMassAndMassFactor,
                        m_orientation,
                        m_inverseInertia,
                        m_firstAttachedBodyId,
                        m_linearVelocityCage,
                        m_integrationFactor,
                        m_motionPropertiesId,
                        m_maxLinearAccelerationDistancePerStep,
                        m_maxRotationToPreventTunneling,
                        m_cellIndex,
                        m_spaceSplitterWeight,
                        m_linearVelocity,
                        m_angularVelocity,
                        m_previousStepLinearVelocity,
                        m_previousStepAngularVelocity,
                        ..Default::default()
                    })
                }
            }
            const FIELDS: &[&str] = &[
                "centerOfMassAndMassFactor",
                "orientation",
                "inverseInertia",
                "firstAttachedBodyId",
                "solverId",
                "linearVelocityCage",
                "integrationFactor",
                "motionPropertiesId",
                "maxLinearAccelerationDistancePerStep",
                "maxRotationToPreventTunneling",
                "cellIndex",
                "spaceSplitterWeight",
                "linearVelocity",
                "angularVelocity",
                "previousStepLinearVelocity",
                "previousStepAngularVelocity",
            ];
            _serde::Deserializer::deserialize_struct(
                deserializer,
                "hknpMotion",
                FIELDS,
                __hknpMotionVisitor {
                    marker: _serde::__private::PhantomData::<hknpMotion>,
                    lifetime: _serde::__private::PhantomData,
                },
            )
        }
    }
};
