use super::class_requires::*;
use super::*;
/// # C++ Info
/// - name: `hclBendStiffnessConstraintSetMxSingle`
/// - version: `0`
/// - signature: `0xc563fd3f`
/// - size: `  0`(x86)/` 48`(x86_64)
/// -  vtable: `false`
#[allow(non_upper_case_globals, non_snake_case)]
#[cfg_attr(feature = "json_schema", derive(schemars::JsonSchema))]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(educe::Educe)]
#[educe(Debug, Clone, Default, PartialEq)]
pub struct hclBendStiffnessConstraintSetMxSingle<'a> {
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
    /// - name: `weightA`(ctype: `hkReal`)
    /// - offset: `  0`(x86)/`  0`(x86_64)
    /// - type_size: `  0`(x86)/`  4`(x86_64)
    #[cfg_attr(feature = "serde_default", serde(default))]
    #[cfg_attr(feature = "json_schema", schemars(rename = "weightA"))]
    #[cfg_attr(feature = "serde", serde(rename = "weightA"))]
    pub m_weightA: f32,
    /// # C++ Info
    /// - name: `weightB`(ctype: `hkReal`)
    /// - offset: `  0`(x86)/`  4`(x86_64)
    /// - type_size: `  0`(x86)/`  4`(x86_64)
    #[cfg_attr(feature = "serde_default", serde(default))]
    #[cfg_attr(feature = "json_schema", schemars(rename = "weightB"))]
    #[cfg_attr(feature = "serde", serde(rename = "weightB"))]
    pub m_weightB: f32,
    /// # C++ Info
    /// - name: `weightC`(ctype: `hkReal`)
    /// - offset: `  0`(x86)/`  8`(x86_64)
    /// - type_size: `  0`(x86)/`  4`(x86_64)
    #[cfg_attr(feature = "serde_default", serde(default))]
    #[cfg_attr(feature = "json_schema", schemars(rename = "weightC"))]
    #[cfg_attr(feature = "serde", serde(rename = "weightC"))]
    pub m_weightC: f32,
    /// # C++ Info
    /// - name: `weightD`(ctype: `hkReal`)
    /// - offset: `  0`(x86)/` 12`(x86_64)
    /// - type_size: `  0`(x86)/`  4`(x86_64)
    #[cfg_attr(feature = "serde_default", serde(default))]
    #[cfg_attr(feature = "json_schema", schemars(rename = "weightD"))]
    #[cfg_attr(feature = "serde", serde(rename = "weightD"))]
    pub m_weightD: f32,
    /// # C++ Info
    /// - name: `bendStiffness`(ctype: `hkReal`)
    /// - offset: `  0`(x86)/` 16`(x86_64)
    /// - type_size: `  0`(x86)/`  4`(x86_64)
    #[cfg_attr(feature = "serde_default", serde(default))]
    #[cfg_attr(feature = "json_schema", schemars(rename = "bendStiffness"))]
    #[cfg_attr(feature = "serde", serde(rename = "bendStiffness"))]
    pub m_bendStiffness: f32,
    /// # C++ Info
    /// - name: `restCurvature`(ctype: `hkReal`)
    /// - offset: `  0`(x86)/` 20`(x86_64)
    /// - type_size: `  0`(x86)/`  4`(x86_64)
    #[cfg_attr(feature = "serde_default", serde(default))]
    #[cfg_attr(feature = "json_schema", schemars(rename = "restCurvature"))]
    #[cfg_attr(feature = "serde", serde(rename = "restCurvature"))]
    pub m_restCurvature: f32,
    /// # C++ Info
    /// - name: `invMassA`(ctype: `hkReal`)
    /// - offset: `  0`(x86)/` 24`(x86_64)
    /// - type_size: `  0`(x86)/`  4`(x86_64)
    #[cfg_attr(feature = "serde_default", serde(default))]
    #[cfg_attr(feature = "json_schema", schemars(rename = "invMassA"))]
    #[cfg_attr(feature = "serde", serde(rename = "invMassA"))]
    pub m_invMassA: f32,
    /// # C++ Info
    /// - name: `invMassB`(ctype: `hkReal`)
    /// - offset: `  0`(x86)/` 28`(x86_64)
    /// - type_size: `  0`(x86)/`  4`(x86_64)
    #[cfg_attr(feature = "serde_default", serde(default))]
    #[cfg_attr(feature = "json_schema", schemars(rename = "invMassB"))]
    #[cfg_attr(feature = "serde", serde(rename = "invMassB"))]
    pub m_invMassB: f32,
    /// # C++ Info
    /// - name: `invMassC`(ctype: `hkReal`)
    /// - offset: `  0`(x86)/` 32`(x86_64)
    /// - type_size: `  0`(x86)/`  4`(x86_64)
    #[cfg_attr(feature = "serde_default", serde(default))]
    #[cfg_attr(feature = "json_schema", schemars(rename = "invMassC"))]
    #[cfg_attr(feature = "serde", serde(rename = "invMassC"))]
    pub m_invMassC: f32,
    /// # C++ Info
    /// - name: `invMassD`(ctype: `hkReal`)
    /// - offset: `  0`(x86)/` 36`(x86_64)
    /// - type_size: `  0`(x86)/`  4`(x86_64)
    #[cfg_attr(feature = "serde_default", serde(default))]
    #[cfg_attr(feature = "json_schema", schemars(rename = "invMassD"))]
    #[cfg_attr(feature = "serde", serde(rename = "invMassD"))]
    pub m_invMassD: f32,
    /// # C++ Info
    /// - name: `particleA`(ctype: `hkUint16`)
    /// - offset: `  0`(x86)/` 40`(x86_64)
    /// - type_size: `  0`(x86)/`  2`(x86_64)
    #[cfg_attr(feature = "serde_default", serde(default))]
    #[cfg_attr(feature = "serde", serde(borrow))]
    #[cfg_attr(feature = "json_schema", schemars(rename = "particleA"))]
    #[cfg_attr(feature = "serde", serde(rename = "particleA"))]
    pub m_particleA: U16<'a>,
    /// # C++ Info
    /// - name: `particleB`(ctype: `hkUint16`)
    /// - offset: `  0`(x86)/` 42`(x86_64)
    /// - type_size: `  0`(x86)/`  2`(x86_64)
    #[cfg_attr(feature = "serde_default", serde(default))]
    #[cfg_attr(feature = "serde", serde(borrow))]
    #[cfg_attr(feature = "json_schema", schemars(rename = "particleB"))]
    #[cfg_attr(feature = "serde", serde(rename = "particleB"))]
    pub m_particleB: U16<'a>,
    /// # C++ Info
    /// - name: `particleC`(ctype: `hkUint16`)
    /// - offset: `  0`(x86)/` 44`(x86_64)
    /// - type_size: `  0`(x86)/`  2`(x86_64)
    #[cfg_attr(feature = "serde_default", serde(default))]
    #[cfg_attr(feature = "serde", serde(borrow))]
    #[cfg_attr(feature = "json_schema", schemars(rename = "particleC"))]
    #[cfg_attr(feature = "serde", serde(rename = "particleC"))]
    pub m_particleC: U16<'a>,
    /// # C++ Info
    /// - name: `particleD`(ctype: `hkUint16`)
    /// - offset: `  0`(x86)/` 46`(x86_64)
    /// - type_size: `  0`(x86)/`  2`(x86_64)
    #[cfg_attr(feature = "serde_default", serde(default))]
    #[cfg_attr(feature = "serde", serde(borrow))]
    #[cfg_attr(feature = "json_schema", schemars(rename = "particleD"))]
    #[cfg_attr(feature = "serde", serde(rename = "particleD"))]
    pub m_particleD: U16<'a>,
}
const _: () = {
    use havok_serde as _serde;
    impl<'a> _serde::HavokClass for hclBendStiffnessConstraintSetMxSingle<'a> {
        #[inline]
        fn name(&self) -> &'static str {
            "hclBendStiffnessConstraintSetMxSingle"
        }
        #[inline]
        fn signature(&self) -> _serde::__private::Signature {
            _serde::__private::Signature::new(0xc563fd3f)
        }
        #[allow(clippy::let_and_return, clippy::vec_init_then_push)]
        fn deps_indexes(&self) -> Vec<&Pointer<'_>> {
            let mut v = Vec::new();
            v
        }
    }
    impl<'a> _serde::Serialize for hclBendStiffnessConstraintSetMxSingle<'a> {
        fn serialize<S>(&self, __serializer: S) -> Result<S::Ok, S::Error>
        where
            S: _serde::ser::Serializer,
        {
            let class_meta = self
                .__ptr
                .as_ref()
                .map(|name| (name, _serde::__private::Signature::new(0xc563fd3f)));
            let mut serializer = __serializer
                .serialize_struct(
                    "hclBendStiffnessConstraintSetMxSingle",
                    class_meta,
                    (0u64, 48u64),
                )?;
            serializer.serialize_field("weightA", &self.m_weightA)?;
            serializer.serialize_field("weightB", &self.m_weightB)?;
            serializer.serialize_field("weightC", &self.m_weightC)?;
            serializer.serialize_field("weightD", &self.m_weightD)?;
            serializer.serialize_field("bendStiffness", &self.m_bendStiffness)?;
            serializer.serialize_field("restCurvature", &self.m_restCurvature)?;
            serializer.serialize_field("invMassA", &self.m_invMassA)?;
            serializer.serialize_field("invMassB", &self.m_invMassB)?;
            serializer.serialize_field("invMassC", &self.m_invMassC)?;
            serializer.serialize_field("invMassD", &self.m_invMassD)?;
            serializer.serialize_field("particleA", &self.m_particleA)?;
            serializer.serialize_field("particleB", &self.m_particleB)?;
            serializer.serialize_field("particleC", &self.m_particleC)?;
            serializer.serialize_field("particleD", &self.m_particleD)?;
            serializer.end()
        }
    }
};
#[doc(hidden)]
#[allow(non_upper_case_globals, unused_attributes, unused_qualifications)]
const _: () = {
    use havok_serde as _serde;
    #[automatically_derived]
    impl<'de> _serde::Deserialize<'de> for hclBendStiffnessConstraintSetMxSingle<'de> {
        fn deserialize<__D>(deserializer: __D) -> core::result::Result<Self, __D::Error>
        where
            __D: _serde::Deserializer<'de>,
        {
            #[allow(non_camel_case_types)]
            enum __Field {
                m_weightA,
                m_weightB,
                m_weightC,
                m_weightD,
                m_bendStiffness,
                m_restCurvature,
                m_invMassA,
                m_invMassB,
                m_invMassC,
                m_invMassD,
                m_particleA,
                m_particleB,
                m_particleC,
                m_particleD,
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
                        "weightA" => Ok(__Field::m_weightA),
                        "weightB" => Ok(__Field::m_weightB),
                        "weightC" => Ok(__Field::m_weightC),
                        "weightD" => Ok(__Field::m_weightD),
                        "bendStiffness" => Ok(__Field::m_bendStiffness),
                        "restCurvature" => Ok(__Field::m_restCurvature),
                        "invMassA" => Ok(__Field::m_invMassA),
                        "invMassB" => Ok(__Field::m_invMassB),
                        "invMassC" => Ok(__Field::m_invMassC),
                        "invMassD" => Ok(__Field::m_invMassD),
                        "particleA" => Ok(__Field::m_particleA),
                        "particleB" => Ok(__Field::m_particleB),
                        "particleC" => Ok(__Field::m_particleC),
                        "particleD" => Ok(__Field::m_particleD),
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
            struct __hclBendStiffnessConstraintSetMxSingleVisitor<'de> {
                marker: _serde::__private::PhantomData<
                    hclBendStiffnessConstraintSetMxSingle<'de>,
                >,
                lifetime: _serde::__private::PhantomData<&'de ()>,
            }
            #[allow(clippy::match_single_binding)]
            #[allow(clippy::reversed_empty_ranges)]
            #[allow(clippy::single_match)]
            impl<'de> _serde::de::Visitor<'de>
            for __hclBendStiffnessConstraintSetMxSingleVisitor<'de> {
                type Value = hclBendStiffnessConstraintSetMxSingle<'de>;
                fn expecting(
                    &self,
                    __formatter: &mut core::fmt::Formatter,
                ) -> core::fmt::Result {
                    core::fmt::Formatter::write_str(
                        __formatter,
                        "struct hclBendStiffnessConstraintSetMxSingle",
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
                    let mut m_weightA: _serde::__private::Option<f32> = _serde::__private::None;
                    let mut m_weightB: _serde::__private::Option<f32> = _serde::__private::None;
                    let mut m_weightC: _serde::__private::Option<f32> = _serde::__private::None;
                    let mut m_weightD: _serde::__private::Option<f32> = _serde::__private::None;
                    let mut m_bendStiffness: _serde::__private::Option<f32> = _serde::__private::None;
                    let mut m_restCurvature: _serde::__private::Option<f32> = _serde::__private::None;
                    let mut m_invMassA: _serde::__private::Option<f32> = _serde::__private::None;
                    let mut m_invMassB: _serde::__private::Option<f32> = _serde::__private::None;
                    let mut m_invMassC: _serde::__private::Option<f32> = _serde::__private::None;
                    let mut m_invMassD: _serde::__private::Option<f32> = _serde::__private::None;
                    let mut m_particleA: _serde::__private::Option<U16<'de>> = _serde::__private::None;
                    let mut m_particleB: _serde::__private::Option<U16<'de>> = _serde::__private::None;
                    let mut m_particleC: _serde::__private::Option<U16<'de>> = _serde::__private::None;
                    let mut m_particleD: _serde::__private::Option<U16<'de>> = _serde::__private::None;
                    for i in 0..14usize {
                        match i {
                            0usize => {
                                if _serde::__private::Option::is_some(&m_weightA) {
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "weightA",
                                        ),
                                    );
                                }
                                m_weightA = _serde::__private::Some(
                                    match __A::next_value::<f32>(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            1usize => {
                                if _serde::__private::Option::is_some(&m_weightB) {
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "weightB",
                                        ),
                                    );
                                }
                                m_weightB = _serde::__private::Some(
                                    match __A::next_value::<f32>(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            2usize => {
                                if _serde::__private::Option::is_some(&m_weightC) {
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "weightC",
                                        ),
                                    );
                                }
                                m_weightC = _serde::__private::Some(
                                    match __A::next_value::<f32>(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            3usize => {
                                if _serde::__private::Option::is_some(&m_weightD) {
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "weightD",
                                        ),
                                    );
                                }
                                m_weightD = _serde::__private::Some(
                                    match __A::next_value::<f32>(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            4usize => {
                                if _serde::__private::Option::is_some(&m_bendStiffness) {
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "bendStiffness",
                                        ),
                                    );
                                }
                                m_bendStiffness = _serde::__private::Some(
                                    match __A::next_value::<f32>(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            5usize => {
                                if _serde::__private::Option::is_some(&m_restCurvature) {
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "restCurvature",
                                        ),
                                    );
                                }
                                m_restCurvature = _serde::__private::Some(
                                    match __A::next_value::<f32>(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            6usize => {
                                if _serde::__private::Option::is_some(&m_invMassA) {
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "invMassA",
                                        ),
                                    );
                                }
                                m_invMassA = _serde::__private::Some(
                                    match __A::next_value::<f32>(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            7usize => {
                                if _serde::__private::Option::is_some(&m_invMassB) {
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "invMassB",
                                        ),
                                    );
                                }
                                m_invMassB = _serde::__private::Some(
                                    match __A::next_value::<f32>(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            8usize => {
                                if _serde::__private::Option::is_some(&m_invMassC) {
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "invMassC",
                                        ),
                                    );
                                }
                                m_invMassC = _serde::__private::Some(
                                    match __A::next_value::<f32>(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            9usize => {
                                if _serde::__private::Option::is_some(&m_invMassD) {
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "invMassD",
                                        ),
                                    );
                                }
                                m_invMassD = _serde::__private::Some(
                                    match __A::next_value::<f32>(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            10usize => {
                                if _serde::__private::Option::is_some(&m_particleA) {
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "particleA",
                                        ),
                                    );
                                }
                                m_particleA = _serde::__private::Some(
                                    match __A::next_value::<U16<'de>>(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            11usize => {
                                if _serde::__private::Option::is_some(&m_particleB) {
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "particleB",
                                        ),
                                    );
                                }
                                m_particleB = _serde::__private::Some(
                                    match __A::next_value::<U16<'de>>(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            12usize => {
                                if _serde::__private::Option::is_some(&m_particleC) {
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "particleC",
                                        ),
                                    );
                                }
                                m_particleC = _serde::__private::Some(
                                    match __A::next_value::<U16<'de>>(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            13usize => {
                                if _serde::__private::Option::is_some(&m_particleD) {
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "particleD",
                                        ),
                                    );
                                }
                                m_particleD = _serde::__private::Some(
                                    match __A::next_value::<U16<'de>>(&mut __map) {
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
                    let m_weightA = match m_weightA {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field("weightA"),
                            );
                        }
                    };
                    let m_weightB = match m_weightB {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field("weightB"),
                            );
                        }
                    };
                    let m_weightC = match m_weightC {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field("weightC"),
                            );
                        }
                    };
                    let m_weightD = match m_weightD {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field("weightD"),
                            );
                        }
                    };
                    let m_bendStiffness = match m_bendStiffness {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "bendStiffness",
                                ),
                            );
                        }
                    };
                    let m_restCurvature = match m_restCurvature {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "restCurvature",
                                ),
                            );
                        }
                    };
                    let m_invMassA = match m_invMassA {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field("invMassA"),
                            );
                        }
                    };
                    let m_invMassB = match m_invMassB {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field("invMassB"),
                            );
                        }
                    };
                    let m_invMassC = match m_invMassC {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field("invMassC"),
                            );
                        }
                    };
                    let m_invMassD = match m_invMassD {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field("invMassD"),
                            );
                        }
                    };
                    let m_particleA = match m_particleA {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "particleA",
                                ),
                            );
                        }
                    };
                    let m_particleB = match m_particleB {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "particleB",
                                ),
                            );
                        }
                    };
                    let m_particleC = match m_particleC {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "particleC",
                                ),
                            );
                        }
                    };
                    let m_particleD = match m_particleD {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "particleD",
                                ),
                            );
                        }
                    };
                    _serde::__private::Ok(hclBendStiffnessConstraintSetMxSingle {
                        __ptr,
                        m_weightA,
                        m_weightB,
                        m_weightC,
                        m_weightD,
                        m_bendStiffness,
                        m_restCurvature,
                        m_invMassA,
                        m_invMassB,
                        m_invMassC,
                        m_invMassD,
                        m_particleA,
                        m_particleB,
                        m_particleC,
                        m_particleD,
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
                    let mut m_weightA: _serde::__private::Option<f32> = _serde::__private::None;
                    let mut m_weightB: _serde::__private::Option<f32> = _serde::__private::None;
                    let mut m_weightC: _serde::__private::Option<f32> = _serde::__private::None;
                    let mut m_weightD: _serde::__private::Option<f32> = _serde::__private::None;
                    let mut m_bendStiffness: _serde::__private::Option<f32> = _serde::__private::None;
                    let mut m_restCurvature: _serde::__private::Option<f32> = _serde::__private::None;
                    let mut m_invMassA: _serde::__private::Option<f32> = _serde::__private::None;
                    let mut m_invMassB: _serde::__private::Option<f32> = _serde::__private::None;
                    let mut m_invMassC: _serde::__private::Option<f32> = _serde::__private::None;
                    let mut m_invMassD: _serde::__private::Option<f32> = _serde::__private::None;
                    let mut m_particleA: _serde::__private::Option<U16<'de>> = _serde::__private::None;
                    let mut m_particleB: _serde::__private::Option<U16<'de>> = _serde::__private::None;
                    let mut m_particleC: _serde::__private::Option<U16<'de>> = _serde::__private::None;
                    let mut m_particleD: _serde::__private::Option<U16<'de>> = _serde::__private::None;
                    while let _serde::__private::Some(__key) = {
                        __A::next_key::<__Field>(&mut __map)?
                    } {
                        match __key {
                            __Field::m_weightA => {
                                #[cfg(
                                    any(feature = "strict", feature = "ignore_duplicates")
                                )]
                                if _serde::__private::Option::is_some(&m_weightA) {
                                    #[cfg(feature = "ignore_duplicates")]
                                    {
                                        __A::skip_value(&mut __map)?;
                                        continue;
                                    }
                                    #[cfg(feature = "strict")]
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "weightA",
                                        ),
                                    );
                                }
                                m_weightA = _serde::__private::Some(
                                    match __A::next_value::<f32>(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            __Field::m_weightB => {
                                #[cfg(
                                    any(feature = "strict", feature = "ignore_duplicates")
                                )]
                                if _serde::__private::Option::is_some(&m_weightB) {
                                    #[cfg(feature = "ignore_duplicates")]
                                    {
                                        __A::skip_value(&mut __map)?;
                                        continue;
                                    }
                                    #[cfg(feature = "strict")]
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "weightB",
                                        ),
                                    );
                                }
                                m_weightB = _serde::__private::Some(
                                    match __A::next_value::<f32>(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            __Field::m_weightC => {
                                #[cfg(
                                    any(feature = "strict", feature = "ignore_duplicates")
                                )]
                                if _serde::__private::Option::is_some(&m_weightC) {
                                    #[cfg(feature = "ignore_duplicates")]
                                    {
                                        __A::skip_value(&mut __map)?;
                                        continue;
                                    }
                                    #[cfg(feature = "strict")]
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "weightC",
                                        ),
                                    );
                                }
                                m_weightC = _serde::__private::Some(
                                    match __A::next_value::<f32>(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            __Field::m_weightD => {
                                #[cfg(
                                    any(feature = "strict", feature = "ignore_duplicates")
                                )]
                                if _serde::__private::Option::is_some(&m_weightD) {
                                    #[cfg(feature = "ignore_duplicates")]
                                    {
                                        __A::skip_value(&mut __map)?;
                                        continue;
                                    }
                                    #[cfg(feature = "strict")]
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "weightD",
                                        ),
                                    );
                                }
                                m_weightD = _serde::__private::Some(
                                    match __A::next_value::<f32>(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            __Field::m_bendStiffness => {
                                #[cfg(
                                    any(feature = "strict", feature = "ignore_duplicates")
                                )]
                                if _serde::__private::Option::is_some(&m_bendStiffness) {
                                    #[cfg(feature = "ignore_duplicates")]
                                    {
                                        __A::skip_value(&mut __map)?;
                                        continue;
                                    }
                                    #[cfg(feature = "strict")]
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "bendStiffness",
                                        ),
                                    );
                                }
                                m_bendStiffness = _serde::__private::Some(
                                    match __A::next_value::<f32>(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            __Field::m_restCurvature => {
                                #[cfg(
                                    any(feature = "strict", feature = "ignore_duplicates")
                                )]
                                if _serde::__private::Option::is_some(&m_restCurvature) {
                                    #[cfg(feature = "ignore_duplicates")]
                                    {
                                        __A::skip_value(&mut __map)?;
                                        continue;
                                    }
                                    #[cfg(feature = "strict")]
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "restCurvature",
                                        ),
                                    );
                                }
                                m_restCurvature = _serde::__private::Some(
                                    match __A::next_value::<f32>(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            __Field::m_invMassA => {
                                #[cfg(
                                    any(feature = "strict", feature = "ignore_duplicates")
                                )]
                                if _serde::__private::Option::is_some(&m_invMassA) {
                                    #[cfg(feature = "ignore_duplicates")]
                                    {
                                        __A::skip_value(&mut __map)?;
                                        continue;
                                    }
                                    #[cfg(feature = "strict")]
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "invMassA",
                                        ),
                                    );
                                }
                                m_invMassA = _serde::__private::Some(
                                    match __A::next_value::<f32>(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            __Field::m_invMassB => {
                                #[cfg(
                                    any(feature = "strict", feature = "ignore_duplicates")
                                )]
                                if _serde::__private::Option::is_some(&m_invMassB) {
                                    #[cfg(feature = "ignore_duplicates")]
                                    {
                                        __A::skip_value(&mut __map)?;
                                        continue;
                                    }
                                    #[cfg(feature = "strict")]
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "invMassB",
                                        ),
                                    );
                                }
                                m_invMassB = _serde::__private::Some(
                                    match __A::next_value::<f32>(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            __Field::m_invMassC => {
                                #[cfg(
                                    any(feature = "strict", feature = "ignore_duplicates")
                                )]
                                if _serde::__private::Option::is_some(&m_invMassC) {
                                    #[cfg(feature = "ignore_duplicates")]
                                    {
                                        __A::skip_value(&mut __map)?;
                                        continue;
                                    }
                                    #[cfg(feature = "strict")]
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "invMassC",
                                        ),
                                    );
                                }
                                m_invMassC = _serde::__private::Some(
                                    match __A::next_value::<f32>(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            __Field::m_invMassD => {
                                #[cfg(
                                    any(feature = "strict", feature = "ignore_duplicates")
                                )]
                                if _serde::__private::Option::is_some(&m_invMassD) {
                                    #[cfg(feature = "ignore_duplicates")]
                                    {
                                        __A::skip_value(&mut __map)?;
                                        continue;
                                    }
                                    #[cfg(feature = "strict")]
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "invMassD",
                                        ),
                                    );
                                }
                                m_invMassD = _serde::__private::Some(
                                    match __A::next_value::<f32>(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            __Field::m_particleA => {
                                #[cfg(
                                    any(feature = "strict", feature = "ignore_duplicates")
                                )]
                                if _serde::__private::Option::is_some(&m_particleA) {
                                    #[cfg(feature = "ignore_duplicates")]
                                    {
                                        __A::skip_value(&mut __map)?;
                                        continue;
                                    }
                                    #[cfg(feature = "strict")]
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "particleA",
                                        ),
                                    );
                                }
                                m_particleA = _serde::__private::Some(
                                    match __A::next_value::<U16<'de>>(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            __Field::m_particleB => {
                                #[cfg(
                                    any(feature = "strict", feature = "ignore_duplicates")
                                )]
                                if _serde::__private::Option::is_some(&m_particleB) {
                                    #[cfg(feature = "ignore_duplicates")]
                                    {
                                        __A::skip_value(&mut __map)?;
                                        continue;
                                    }
                                    #[cfg(feature = "strict")]
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "particleB",
                                        ),
                                    );
                                }
                                m_particleB = _serde::__private::Some(
                                    match __A::next_value::<U16<'de>>(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            __Field::m_particleC => {
                                #[cfg(
                                    any(feature = "strict", feature = "ignore_duplicates")
                                )]
                                if _serde::__private::Option::is_some(&m_particleC) {
                                    #[cfg(feature = "ignore_duplicates")]
                                    {
                                        __A::skip_value(&mut __map)?;
                                        continue;
                                    }
                                    #[cfg(feature = "strict")]
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "particleC",
                                        ),
                                    );
                                }
                                m_particleC = _serde::__private::Some(
                                    match __A::next_value::<U16<'de>>(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            __Field::m_particleD => {
                                #[cfg(
                                    any(feature = "strict", feature = "ignore_duplicates")
                                )]
                                if _serde::__private::Option::is_some(&m_particleD) {
                                    #[cfg(feature = "ignore_duplicates")]
                                    {
                                        __A::skip_value(&mut __map)?;
                                        continue;
                                    }
                                    #[cfg(feature = "strict")]
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "particleD",
                                        ),
                                    );
                                }
                                m_particleD = _serde::__private::Some(
                                    match __A::next_value::<U16<'de>>(&mut __map) {
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
                    let m_weightA = match m_weightA {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            #[cfg(feature = "strict")]
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field("weightA"),
                            );
                            #[cfg(not(feature = "strict"))] Default::default()
                        }
                    };
                    let m_weightB = match m_weightB {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            #[cfg(feature = "strict")]
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field("weightB"),
                            );
                            #[cfg(not(feature = "strict"))] Default::default()
                        }
                    };
                    let m_weightC = match m_weightC {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            #[cfg(feature = "strict")]
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field("weightC"),
                            );
                            #[cfg(not(feature = "strict"))] Default::default()
                        }
                    };
                    let m_weightD = match m_weightD {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            #[cfg(feature = "strict")]
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field("weightD"),
                            );
                            #[cfg(not(feature = "strict"))] Default::default()
                        }
                    };
                    let m_bendStiffness = match m_bendStiffness {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            #[cfg(feature = "strict")]
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "bendStiffness",
                                ),
                            );
                            #[cfg(not(feature = "strict"))] Default::default()
                        }
                    };
                    let m_restCurvature = match m_restCurvature {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            #[cfg(feature = "strict")]
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "restCurvature",
                                ),
                            );
                            #[cfg(not(feature = "strict"))] Default::default()
                        }
                    };
                    let m_invMassA = match m_invMassA {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            #[cfg(feature = "strict")]
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field("invMassA"),
                            );
                            #[cfg(not(feature = "strict"))] Default::default()
                        }
                    };
                    let m_invMassB = match m_invMassB {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            #[cfg(feature = "strict")]
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field("invMassB"),
                            );
                            #[cfg(not(feature = "strict"))] Default::default()
                        }
                    };
                    let m_invMassC = match m_invMassC {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            #[cfg(feature = "strict")]
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field("invMassC"),
                            );
                            #[cfg(not(feature = "strict"))] Default::default()
                        }
                    };
                    let m_invMassD = match m_invMassD {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            #[cfg(feature = "strict")]
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field("invMassD"),
                            );
                            #[cfg(not(feature = "strict"))] Default::default()
                        }
                    };
                    let m_particleA = match m_particleA {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            #[cfg(feature = "strict")]
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "particleA",
                                ),
                            );
                            #[cfg(not(feature = "strict"))] Default::default()
                        }
                    };
                    let m_particleB = match m_particleB {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            #[cfg(feature = "strict")]
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "particleB",
                                ),
                            );
                            #[cfg(not(feature = "strict"))] Default::default()
                        }
                    };
                    let m_particleC = match m_particleC {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            #[cfg(feature = "strict")]
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "particleC",
                                ),
                            );
                            #[cfg(not(feature = "strict"))] Default::default()
                        }
                    };
                    let m_particleD = match m_particleD {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            #[cfg(feature = "strict")]
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "particleD",
                                ),
                            );
                            #[cfg(not(feature = "strict"))] Default::default()
                        }
                    };
                    let __ptr = __A::class_ptr(&mut __map);
                    _serde::__private::Ok(hclBendStiffnessConstraintSetMxSingle {
                        __ptr: __ptr.clone(),
                        m_weightA,
                        m_weightB,
                        m_weightC,
                        m_weightD,
                        m_bendStiffness,
                        m_restCurvature,
                        m_invMassA,
                        m_invMassB,
                        m_invMassC,
                        m_invMassD,
                        m_particleA,
                        m_particleB,
                        m_particleC,
                        m_particleD,
                    })
                }
            }
            const FIELDS: &[&str] = &[
                "weightA",
                "weightB",
                "weightC",
                "weightD",
                "bendStiffness",
                "restCurvature",
                "invMassA",
                "invMassB",
                "invMassC",
                "invMassD",
                "particleA",
                "particleB",
                "particleC",
                "particleD",
            ];
            _serde::Deserializer::deserialize_struct(
                deserializer,
                "hclBendStiffnessConstraintSetMxSingle",
                FIELDS,
                __hclBendStiffnessConstraintSetMxSingleVisitor {
                    marker: _serde::__private::PhantomData::<
                        hclBendStiffnessConstraintSetMxSingle,
                    >,
                    lifetime: _serde::__private::PhantomData,
                },
            )
        }
    }
};
