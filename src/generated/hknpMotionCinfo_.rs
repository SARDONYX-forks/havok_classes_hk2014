use super::class_requires::*;
use super::*;
/// # C++ Info
/// - name: `hknpMotionCinfo`
/// - version: `1`
/// - signature: `0x693d1b16`
/// - size: `  0`(x86)/`112`(x86_64)
/// -  vtable: `false`
#[allow(non_upper_case_globals, non_snake_case)]
#[cfg_attr(feature = "json_schema", derive(schemars::JsonSchema))]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(educe::Educe)]
#[educe(Debug, Clone, Default, PartialEq)]
pub struct hknpMotionCinfo<'a> {
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
    /// - name: `motionPropertiesId`(ctype: `hkUint16`)
    /// - offset: `  0`(x86)/`  0`(x86_64)
    /// - type_size: `  0`(x86)/`  2`(x86_64)
    #[cfg_attr(feature = "serde_default", serde(default))]
    #[cfg_attr(feature = "serde", serde(borrow))]
    #[cfg_attr(feature = "json_schema", schemars(rename = "motionPropertiesId"))]
    #[cfg_attr(feature = "serde", serde(rename = "motionPropertiesId"))]
    pub m_motionPropertiesId: U16<'a>,
    /// # C++ Info
    /// - name: `enableDeactivation`(ctype: `hkBool`)
    /// - offset: `  0`(x86)/`  2`(x86_64)
    /// - type_size: `  0`(x86)/`  1`(x86_64)
    #[cfg_attr(feature = "serde_default", serde(default))]
    #[cfg_attr(feature = "json_schema", schemars(rename = "enableDeactivation"))]
    #[cfg_attr(feature = "serde", serde(rename = "enableDeactivation"))]
    pub m_enableDeactivation: bool,
    /// # C++ Info
    /// - name: `inverseMass`(ctype: `hkReal`)
    /// - offset: `  0`(x86)/`  4`(x86_64)
    /// - type_size: `  0`(x86)/`  4`(x86_64)
    #[cfg_attr(feature = "serde_default", serde(default))]
    #[cfg_attr(feature = "json_schema", schemars(rename = "inverseMass"))]
    #[cfg_attr(feature = "serde", serde(rename = "inverseMass"))]
    pub m_inverseMass: f32,
    /// # C++ Info
    /// - name: `massFactor`(ctype: `hkReal`)
    /// - offset: `  0`(x86)/`  8`(x86_64)
    /// - type_size: `  0`(x86)/`  4`(x86_64)
    #[cfg_attr(feature = "serde_default", serde(default))]
    #[cfg_attr(feature = "json_schema", schemars(rename = "massFactor"))]
    #[cfg_attr(feature = "serde", serde(rename = "massFactor"))]
    pub m_massFactor: f32,
    /// # C++ Info
    /// - name: `maxLinearAccelerationDistancePerStep`(ctype: `hkReal`)
    /// - offset: `  0`(x86)/` 12`(x86_64)
    /// - type_size: `  0`(x86)/`  4`(x86_64)
    #[cfg_attr(feature = "serde_default", serde(default))]
    #[cfg_attr(
        feature = "json_schema",
        schemars(rename = "maxLinearAccelerationDistancePerStep")
    )]
    #[cfg_attr(
        feature = "serde",
        serde(rename = "maxLinearAccelerationDistancePerStep")
    )]
    pub m_maxLinearAccelerationDistancePerStep: f32,
    /// # C++ Info
    /// - name: `maxRotationToPreventTunneling`(ctype: `hkReal`)
    /// - offset: `  0`(x86)/` 16`(x86_64)
    /// - type_size: `  0`(x86)/`  4`(x86_64)
    #[cfg_attr(feature = "serde_default", serde(default))]
    #[cfg_attr(
        feature = "json_schema",
        schemars(rename = "maxRotationToPreventTunneling")
    )]
    #[cfg_attr(feature = "serde", serde(rename = "maxRotationToPreventTunneling"))]
    pub m_maxRotationToPreventTunneling: f32,
    /// # C++ Info
    /// - name: `inverseInertiaLocal`(ctype: `hkVector4`)
    /// - offset: `  0`(x86)/` 32`(x86_64)
    /// - type_size: `  0`(x86)/` 16`(x86_64)
    #[cfg_attr(feature = "serde_default", serde(default))]
    #[cfg_attr(feature = "json_schema", schemars(rename = "inverseInertiaLocal"))]
    #[cfg_attr(feature = "serde", serde(rename = "inverseInertiaLocal"))]
    pub m_inverseInertiaLocal: Vector4,
    /// # C++ Info
    /// - name: `centerOfMassWorld`(ctype: `hkVector4`)
    /// - offset: `  0`(x86)/` 48`(x86_64)
    /// - type_size: `  0`(x86)/` 16`(x86_64)
    #[cfg_attr(feature = "serde_default", serde(default))]
    #[cfg_attr(feature = "json_schema", schemars(rename = "centerOfMassWorld"))]
    #[cfg_attr(feature = "serde", serde(rename = "centerOfMassWorld"))]
    pub m_centerOfMassWorld: Vector4,
    /// # C++ Info
    /// - name: `orientation`(ctype: `hkQuaternion`)
    /// - offset: `  0`(x86)/` 64`(x86_64)
    /// - type_size: `  0`(x86)/` 16`(x86_64)
    #[cfg_attr(feature = "serde_default", serde(default))]
    #[cfg_attr(feature = "json_schema", schemars(rename = "orientation"))]
    #[cfg_attr(feature = "serde", serde(rename = "orientation"))]
    pub m_orientation: Quaternion,
    /// # C++ Info
    /// - name: `linearVelocity`(ctype: `hkVector4`)
    /// - offset: `  0`(x86)/` 80`(x86_64)
    /// - type_size: `  0`(x86)/` 16`(x86_64)
    #[cfg_attr(feature = "serde_default", serde(default))]
    #[cfg_attr(feature = "json_schema", schemars(rename = "linearVelocity"))]
    #[cfg_attr(feature = "serde", serde(rename = "linearVelocity"))]
    pub m_linearVelocity: Vector4,
    /// # C++ Info
    /// - name: `angularVelocity`(ctype: `hkVector4`)
    /// - offset: `  0`(x86)/` 96`(x86_64)
    /// - type_size: `  0`(x86)/` 16`(x86_64)
    #[cfg_attr(feature = "serde_default", serde(default))]
    #[cfg_attr(feature = "json_schema", schemars(rename = "angularVelocity"))]
    #[cfg_attr(feature = "serde", serde(rename = "angularVelocity"))]
    pub m_angularVelocity: Vector4,
}
const _: () = {
    use havok_serde as _serde;
    impl<'a> _serde::HavokClass for hknpMotionCinfo<'a> {
        #[inline]
        fn name(&self) -> &'static str {
            "hknpMotionCinfo"
        }
        #[inline]
        fn signature(&self) -> _serde::__private::Signature {
            _serde::__private::Signature::new(0x693d1b16)
        }
        #[allow(clippy::let_and_return, clippy::vec_init_then_push)]
        fn deps_indexes(&self) -> Vec<&Pointer<'_>> {
            let mut v = Vec::new();
            v
        }
    }
    impl<'a> _serde::Serialize for hknpMotionCinfo<'a> {
        fn serialize<S>(&self, __serializer: S) -> Result<S::Ok, S::Error>
        where
            S: _serde::ser::Serializer,
        {
            let class_meta = self
                .__ptr
                .as_ref()
                .map(|name| (name, _serde::__private::Signature::new(0x693d1b16)));
            let mut serializer = __serializer
                .serialize_struct("hknpMotionCinfo", class_meta, (0u64, 112u64))?;
            serializer
                .serialize_field("motionPropertiesId", &self.m_motionPropertiesId)?;
            serializer
                .serialize_field("enableDeactivation", &self.m_enableDeactivation)?;
            serializer.pad_field([0u8; 0usize].as_slice(), [0u8; 1usize].as_slice())?;
            serializer.serialize_field("inverseMass", &self.m_inverseMass)?;
            serializer.serialize_field("massFactor", &self.m_massFactor)?;
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
            serializer.pad_field([0u8; 0usize].as_slice(), [0u8; 12usize].as_slice())?;
            serializer
                .serialize_field("inverseInertiaLocal", &self.m_inverseInertiaLocal)?;
            serializer.serialize_field("centerOfMassWorld", &self.m_centerOfMassWorld)?;
            serializer.serialize_field("orientation", &self.m_orientation)?;
            serializer.serialize_field("linearVelocity", &self.m_linearVelocity)?;
            serializer.serialize_field("angularVelocity", &self.m_angularVelocity)?;
            serializer.end()
        }
    }
};
#[doc(hidden)]
#[allow(non_upper_case_globals, unused_attributes, unused_qualifications)]
const _: () = {
    use havok_serde as _serde;
    #[automatically_derived]
    impl<'de> _serde::Deserialize<'de> for hknpMotionCinfo<'de> {
        fn deserialize<__D>(deserializer: __D) -> core::result::Result<Self, __D::Error>
        where
            __D: _serde::Deserializer<'de>,
        {
            #[allow(non_camel_case_types)]
            enum __Field {
                m_motionPropertiesId,
                m_enableDeactivation,
                m_inverseMass,
                m_massFactor,
                m_maxLinearAccelerationDistancePerStep,
                m_maxRotationToPreventTunneling,
                m_inverseInertiaLocal,
                m_centerOfMassWorld,
                m_orientation,
                m_linearVelocity,
                m_angularVelocity,
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
                        "motionPropertiesId" => Ok(__Field::m_motionPropertiesId),
                        "enableDeactivation" => Ok(__Field::m_enableDeactivation),
                        "inverseMass" => Ok(__Field::m_inverseMass),
                        "massFactor" => Ok(__Field::m_massFactor),
                        "maxLinearAccelerationDistancePerStep" => {
                            Ok(__Field::m_maxLinearAccelerationDistancePerStep)
                        }
                        "maxRotationToPreventTunneling" => {
                            Ok(__Field::m_maxRotationToPreventTunneling)
                        }
                        "inverseInertiaLocal" => Ok(__Field::m_inverseInertiaLocal),
                        "centerOfMassWorld" => Ok(__Field::m_centerOfMassWorld),
                        "orientation" => Ok(__Field::m_orientation),
                        "linearVelocity" => Ok(__Field::m_linearVelocity),
                        "angularVelocity" => Ok(__Field::m_angularVelocity),
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
            struct __hknpMotionCinfoVisitor<'de> {
                marker: _serde::__private::PhantomData<hknpMotionCinfo<'de>>,
                lifetime: _serde::__private::PhantomData<&'de ()>,
            }
            #[allow(clippy::match_single_binding)]
            #[allow(clippy::reversed_empty_ranges)]
            #[allow(clippy::single_match)]
            impl<'de> _serde::de::Visitor<'de> for __hknpMotionCinfoVisitor<'de> {
                type Value = hknpMotionCinfo<'de>;
                fn expecting(
                    &self,
                    __formatter: &mut core::fmt::Formatter,
                ) -> core::fmt::Result {
                    core::fmt::Formatter::write_str(
                        __formatter,
                        "struct hknpMotionCinfo",
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
                    let mut m_motionPropertiesId: _serde::__private::Option<U16<'de>> = _serde::__private::None;
                    let mut m_enableDeactivation: _serde::__private::Option<bool> = _serde::__private::None;
                    let mut m_inverseMass: _serde::__private::Option<f32> = _serde::__private::None;
                    let mut m_massFactor: _serde::__private::Option<f32> = _serde::__private::None;
                    let mut m_maxLinearAccelerationDistancePerStep: _serde::__private::Option<
                        f32,
                    > = _serde::__private::None;
                    let mut m_maxRotationToPreventTunneling: _serde::__private::Option<
                        f32,
                    > = _serde::__private::None;
                    let mut m_inverseInertiaLocal: _serde::__private::Option<Vector4> = _serde::__private::None;
                    let mut m_centerOfMassWorld: _serde::__private::Option<Vector4> = _serde::__private::None;
                    let mut m_orientation: _serde::__private::Option<Quaternion> = _serde::__private::None;
                    let mut m_linearVelocity: _serde::__private::Option<Vector4> = _serde::__private::None;
                    let mut m_angularVelocity: _serde::__private::Option<Vector4> = _serde::__private::None;
                    for i in 0..11usize {
                        match i {
                            0usize => {
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
                            1usize => {
                                if _serde::__private::Option::is_some(
                                    &m_enableDeactivation,
                                ) {
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "enableDeactivation",
                                        ),
                                    );
                                }
                                m_enableDeactivation = _serde::__private::Some(
                                    match __A::next_value::<bool>(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            2usize => {
                                if _serde::__private::Option::is_some(&m_inverseMass) {
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "inverseMass",
                                        ),
                                    );
                                }
                                __A::pad(&mut __map, 0usize, 1usize)?;
                                m_inverseMass = _serde::__private::Some(
                                    match __A::next_value::<f32>(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            3usize => {
                                if _serde::__private::Option::is_some(&m_massFactor) {
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "massFactor",
                                        ),
                                    );
                                }
                                m_massFactor = _serde::__private::Some(
                                    match __A::next_value::<f32>(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            4usize => {
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
                                    match __A::next_value::<f32>(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            5usize => {
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
                                    match __A::next_value::<f32>(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            6usize => {
                                if _serde::__private::Option::is_some(
                                    &m_inverseInertiaLocal,
                                ) {
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "inverseInertiaLocal",
                                        ),
                                    );
                                }
                                __A::pad(&mut __map, 0usize, 12usize)?;
                                m_inverseInertiaLocal = _serde::__private::Some(
                                    match __A::next_value::<Vector4>(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            7usize => {
                                if _serde::__private::Option::is_some(
                                    &m_centerOfMassWorld,
                                ) {
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "centerOfMassWorld",
                                        ),
                                    );
                                }
                                m_centerOfMassWorld = _serde::__private::Some(
                                    match __A::next_value::<Vector4>(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            8usize => {
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
                            9usize => {
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
                            10usize => {
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
                            _ => {}
                        }
                    }
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
                    let m_enableDeactivation = match m_enableDeactivation {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "enableDeactivation",
                                ),
                            );
                        }
                    };
                    let m_inverseMass = match m_inverseMass {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "inverseMass",
                                ),
                            );
                        }
                    };
                    let m_massFactor = match m_massFactor {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "massFactor",
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
                    let m_inverseInertiaLocal = match m_inverseInertiaLocal {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "inverseInertiaLocal",
                                ),
                            );
                        }
                    };
                    let m_centerOfMassWorld = match m_centerOfMassWorld {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "centerOfMassWorld",
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
                    _serde::__private::Ok(hknpMotionCinfo {
                        __ptr,
                        m_motionPropertiesId,
                        m_enableDeactivation,
                        m_inverseMass,
                        m_massFactor,
                        m_maxLinearAccelerationDistancePerStep,
                        m_maxRotationToPreventTunneling,
                        m_inverseInertiaLocal,
                        m_centerOfMassWorld,
                        m_orientation,
                        m_linearVelocity,
                        m_angularVelocity,
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
                    let mut m_motionPropertiesId: _serde::__private::Option<U16<'de>> = _serde::__private::None;
                    let mut m_enableDeactivation: _serde::__private::Option<bool> = _serde::__private::None;
                    let mut m_inverseMass: _serde::__private::Option<f32> = _serde::__private::None;
                    let mut m_massFactor: _serde::__private::Option<f32> = _serde::__private::None;
                    let mut m_maxLinearAccelerationDistancePerStep: _serde::__private::Option<
                        f32,
                    > = _serde::__private::None;
                    let mut m_maxRotationToPreventTunneling: _serde::__private::Option<
                        f32,
                    > = _serde::__private::None;
                    let mut m_inverseInertiaLocal: _serde::__private::Option<Vector4> = _serde::__private::None;
                    let mut m_centerOfMassWorld: _serde::__private::Option<Vector4> = _serde::__private::None;
                    let mut m_orientation: _serde::__private::Option<Quaternion> = _serde::__private::None;
                    let mut m_linearVelocity: _serde::__private::Option<Vector4> = _serde::__private::None;
                    let mut m_angularVelocity: _serde::__private::Option<Vector4> = _serde::__private::None;
                    while let _serde::__private::Some(__key) = {
                        __A::next_key::<__Field>(&mut __map)?
                    } {
                        match __key {
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
                            __Field::m_enableDeactivation => {
                                #[cfg(
                                    any(feature = "strict", feature = "ignore_duplicates")
                                )]
                                if _serde::__private::Option::is_some(
                                    &m_enableDeactivation,
                                ) {
                                    #[cfg(feature = "ignore_duplicates")]
                                    {
                                        __A::skip_value(&mut __map)?;
                                        continue;
                                    }
                                    #[cfg(feature = "strict")]
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "enableDeactivation",
                                        ),
                                    );
                                }
                                m_enableDeactivation = _serde::__private::Some(
                                    match __A::next_value::<bool>(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            __Field::m_inverseMass => {
                                #[cfg(
                                    any(feature = "strict", feature = "ignore_duplicates")
                                )]
                                if _serde::__private::Option::is_some(&m_inverseMass) {
                                    #[cfg(feature = "ignore_duplicates")]
                                    {
                                        __A::skip_value(&mut __map)?;
                                        continue;
                                    }
                                    #[cfg(feature = "strict")]
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "inverseMass",
                                        ),
                                    );
                                }
                                m_inverseMass = _serde::__private::Some(
                                    match __A::next_value::<f32>(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            __Field::m_massFactor => {
                                #[cfg(
                                    any(feature = "strict", feature = "ignore_duplicates")
                                )]
                                if _serde::__private::Option::is_some(&m_massFactor) {
                                    #[cfg(feature = "ignore_duplicates")]
                                    {
                                        __A::skip_value(&mut __map)?;
                                        continue;
                                    }
                                    #[cfg(feature = "strict")]
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "massFactor",
                                        ),
                                    );
                                }
                                m_massFactor = _serde::__private::Some(
                                    match __A::next_value::<f32>(&mut __map) {
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
                                    match __A::next_value::<f32>(&mut __map) {
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
                                    match __A::next_value::<f32>(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            __Field::m_inverseInertiaLocal => {
                                #[cfg(
                                    any(feature = "strict", feature = "ignore_duplicates")
                                )]
                                if _serde::__private::Option::is_some(
                                    &m_inverseInertiaLocal,
                                ) {
                                    #[cfg(feature = "ignore_duplicates")]
                                    {
                                        __A::skip_value(&mut __map)?;
                                        continue;
                                    }
                                    #[cfg(feature = "strict")]
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "inverseInertiaLocal",
                                        ),
                                    );
                                }
                                m_inverseInertiaLocal = _serde::__private::Some(
                                    match __A::next_value::<Vector4>(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            __Field::m_centerOfMassWorld => {
                                #[cfg(
                                    any(feature = "strict", feature = "ignore_duplicates")
                                )]
                                if _serde::__private::Option::is_some(
                                    &m_centerOfMassWorld,
                                ) {
                                    #[cfg(feature = "ignore_duplicates")]
                                    {
                                        __A::skip_value(&mut __map)?;
                                        continue;
                                    }
                                    #[cfg(feature = "strict")]
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "centerOfMassWorld",
                                        ),
                                    );
                                }
                                m_centerOfMassWorld = _serde::__private::Some(
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
                            _ => __A::skip_value(&mut __map)?,
                        }
                    }
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
                    let m_enableDeactivation = match m_enableDeactivation {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            #[cfg(feature = "strict")]
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "enableDeactivation",
                                ),
                            );
                            #[cfg(not(feature = "strict"))] Default::default()
                        }
                    };
                    let m_inverseMass = match m_inverseMass {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            #[cfg(feature = "strict")]
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "inverseMass",
                                ),
                            );
                            #[cfg(not(feature = "strict"))] Default::default()
                        }
                    };
                    let m_massFactor = match m_massFactor {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            #[cfg(feature = "strict")]
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "massFactor",
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
                    let m_inverseInertiaLocal = match m_inverseInertiaLocal {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            #[cfg(feature = "strict")]
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "inverseInertiaLocal",
                                ),
                            );
                            #[cfg(not(feature = "strict"))] Default::default()
                        }
                    };
                    let m_centerOfMassWorld = match m_centerOfMassWorld {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            #[cfg(feature = "strict")]
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "centerOfMassWorld",
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
                    let __ptr = __A::class_ptr(&mut __map);
                    _serde::__private::Ok(hknpMotionCinfo {
                        __ptr: __ptr.clone(),
                        m_motionPropertiesId,
                        m_enableDeactivation,
                        m_inverseMass,
                        m_massFactor,
                        m_maxLinearAccelerationDistancePerStep,
                        m_maxRotationToPreventTunneling,
                        m_inverseInertiaLocal,
                        m_centerOfMassWorld,
                        m_orientation,
                        m_linearVelocity,
                        m_angularVelocity,
                    })
                }
            }
            const FIELDS: &[&str] = &[
                "motionPropertiesId",
                "enableDeactivation",
                "inverseMass",
                "massFactor",
                "maxLinearAccelerationDistancePerStep",
                "maxRotationToPreventTunneling",
                "inverseInertiaLocal",
                "centerOfMassWorld",
                "orientation",
                "linearVelocity",
                "angularVelocity",
            ];
            _serde::Deserializer::deserialize_struct(
                deserializer,
                "hknpMotionCinfo",
                FIELDS,
                __hknpMotionCinfoVisitor {
                    marker: _serde::__private::PhantomData::<hknpMotionCinfo>,
                    lifetime: _serde::__private::PhantomData,
                },
            )
        }
    }
};
