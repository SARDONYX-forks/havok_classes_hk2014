use super::class_requires::*;
use super::*;
/// # C++ Info
/// - name: `hclBendStiffnessConstraintSetMxBatch`
/// - version: `0`
/// - signature: `0x395d6341`
/// - size: `  0`(x86)/`768`(x86_64)
/// -  vtable: `false`
#[allow(non_upper_case_globals, non_snake_case)]
#[cfg_attr(feature = "json_schema", derive(schemars::JsonSchema))]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(educe::Educe)]
#[educe(Debug, Clone, Default, PartialEq)]
pub struct hclBendStiffnessConstraintSetMxBatch<'a> {
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
    /// - name: `weightsA`(ctype: `hkReal[16]`)
    /// - offset: `  0`(x86)/`  0`(x86_64)
    /// - type_size: `  0`(x86)/` 64`(x86_64)
    #[cfg_attr(feature = "serde_default", serde(default))]
    #[cfg_attr(feature = "json_schema", schemars(rename = "weightsA"))]
    #[cfg_attr(feature = "serde", serde(rename = "weightsA"))]
    pub m_weightsA: [f32; 16usize],
    /// # C++ Info
    /// - name: `weightsB`(ctype: `hkReal[16]`)
    /// - offset: `  0`(x86)/` 64`(x86_64)
    /// - type_size: `  0`(x86)/` 64`(x86_64)
    #[cfg_attr(feature = "serde_default", serde(default))]
    #[cfg_attr(feature = "json_schema", schemars(rename = "weightsB"))]
    #[cfg_attr(feature = "serde", serde(rename = "weightsB"))]
    pub m_weightsB: [f32; 16usize],
    /// # C++ Info
    /// - name: `weightsC`(ctype: `hkReal[16]`)
    /// - offset: `  0`(x86)/`128`(x86_64)
    /// - type_size: `  0`(x86)/` 64`(x86_64)
    #[cfg_attr(feature = "serde_default", serde(default))]
    #[cfg_attr(feature = "json_schema", schemars(rename = "weightsC"))]
    #[cfg_attr(feature = "serde", serde(rename = "weightsC"))]
    pub m_weightsC: [f32; 16usize],
    /// # C++ Info
    /// - name: `weightsD`(ctype: `hkReal[16]`)
    /// - offset: `  0`(x86)/`192`(x86_64)
    /// - type_size: `  0`(x86)/` 64`(x86_64)
    #[cfg_attr(feature = "serde_default", serde(default))]
    #[cfg_attr(feature = "json_schema", schemars(rename = "weightsD"))]
    #[cfg_attr(feature = "serde", serde(rename = "weightsD"))]
    pub m_weightsD: [f32; 16usize],
    /// # C++ Info
    /// - name: `bendStiffnesses`(ctype: `hkReal[16]`)
    /// - offset: `  0`(x86)/`256`(x86_64)
    /// - type_size: `  0`(x86)/` 64`(x86_64)
    #[cfg_attr(feature = "serde_default", serde(default))]
    #[cfg_attr(feature = "json_schema", schemars(rename = "bendStiffnesses"))]
    #[cfg_attr(feature = "serde", serde(rename = "bendStiffnesses"))]
    pub m_bendStiffnesses: [f32; 16usize],
    /// # C++ Info
    /// - name: `restCurvatures`(ctype: `hkReal[16]`)
    /// - offset: `  0`(x86)/`320`(x86_64)
    /// - type_size: `  0`(x86)/` 64`(x86_64)
    #[cfg_attr(feature = "serde_default", serde(default))]
    #[cfg_attr(feature = "json_schema", schemars(rename = "restCurvatures"))]
    #[cfg_attr(feature = "serde", serde(rename = "restCurvatures"))]
    pub m_restCurvatures: [f32; 16usize],
    /// # C++ Info
    /// - name: `invMassesA`(ctype: `hkReal[16]`)
    /// - offset: `  0`(x86)/`384`(x86_64)
    /// - type_size: `  0`(x86)/` 64`(x86_64)
    #[cfg_attr(feature = "serde_default", serde(default))]
    #[cfg_attr(feature = "json_schema", schemars(rename = "invMassesA"))]
    #[cfg_attr(feature = "serde", serde(rename = "invMassesA"))]
    pub m_invMassesA: [f32; 16usize],
    /// # C++ Info
    /// - name: `invMassesB`(ctype: `hkReal[16]`)
    /// - offset: `  0`(x86)/`448`(x86_64)
    /// - type_size: `  0`(x86)/` 64`(x86_64)
    #[cfg_attr(feature = "serde_default", serde(default))]
    #[cfg_attr(feature = "json_schema", schemars(rename = "invMassesB"))]
    #[cfg_attr(feature = "serde", serde(rename = "invMassesB"))]
    pub m_invMassesB: [f32; 16usize],
    /// # C++ Info
    /// - name: `invMassesC`(ctype: `hkReal[16]`)
    /// - offset: `  0`(x86)/`512`(x86_64)
    /// - type_size: `  0`(x86)/` 64`(x86_64)
    #[cfg_attr(feature = "serde_default", serde(default))]
    #[cfg_attr(feature = "json_schema", schemars(rename = "invMassesC"))]
    #[cfg_attr(feature = "serde", serde(rename = "invMassesC"))]
    pub m_invMassesC: [f32; 16usize],
    /// # C++ Info
    /// - name: `invMassesD`(ctype: `hkReal[16]`)
    /// - offset: `  0`(x86)/`576`(x86_64)
    /// - type_size: `  0`(x86)/` 64`(x86_64)
    #[cfg_attr(feature = "serde_default", serde(default))]
    #[cfg_attr(feature = "json_schema", schemars(rename = "invMassesD"))]
    #[cfg_attr(feature = "serde", serde(rename = "invMassesD"))]
    pub m_invMassesD: [f32; 16usize],
    /// # C++ Info
    /// - name: `particlesA`(ctype: `hkUint16[16]`)
    /// - offset: `  0`(x86)/`640`(x86_64)
    /// - type_size: `  0`(x86)/` 32`(x86_64)
    #[cfg_attr(feature = "serde_default", serde(default))]
    #[cfg_attr(feature = "serde", serde(borrow))]
    #[cfg_attr(feature = "json_schema", schemars(rename = "particlesA"))]
    #[cfg_attr(feature = "serde", serde(rename = "particlesA"))]
    pub m_particlesA: [U16<'a>; 16usize],
    /// # C++ Info
    /// - name: `particlesB`(ctype: `hkUint16[16]`)
    /// - offset: `  0`(x86)/`672`(x86_64)
    /// - type_size: `  0`(x86)/` 32`(x86_64)
    #[cfg_attr(feature = "serde_default", serde(default))]
    #[cfg_attr(feature = "serde", serde(borrow))]
    #[cfg_attr(feature = "json_schema", schemars(rename = "particlesB"))]
    #[cfg_attr(feature = "serde", serde(rename = "particlesB"))]
    pub m_particlesB: [U16<'a>; 16usize],
    /// # C++ Info
    /// - name: `particlesC`(ctype: `hkUint16[16]`)
    /// - offset: `  0`(x86)/`704`(x86_64)
    /// - type_size: `  0`(x86)/` 32`(x86_64)
    #[cfg_attr(feature = "serde_default", serde(default))]
    #[cfg_attr(feature = "serde", serde(borrow))]
    #[cfg_attr(feature = "json_schema", schemars(rename = "particlesC"))]
    #[cfg_attr(feature = "serde", serde(rename = "particlesC"))]
    pub m_particlesC: [U16<'a>; 16usize],
    /// # C++ Info
    /// - name: `particlesD`(ctype: `hkUint16[16]`)
    /// - offset: `  0`(x86)/`736`(x86_64)
    /// - type_size: `  0`(x86)/` 32`(x86_64)
    #[cfg_attr(feature = "serde_default", serde(default))]
    #[cfg_attr(feature = "serde", serde(borrow))]
    #[cfg_attr(feature = "json_schema", schemars(rename = "particlesD"))]
    #[cfg_attr(feature = "serde", serde(rename = "particlesD"))]
    pub m_particlesD: [U16<'a>; 16usize],
}
const _: () = {
    use havok_serde as _serde;
    impl<'a> _serde::HavokClass for hclBendStiffnessConstraintSetMxBatch<'a> {
        #[inline]
        fn name(&self) -> &'static str {
            "hclBendStiffnessConstraintSetMxBatch"
        }
        #[inline]
        fn signature(&self) -> _serde::__private::Signature {
            _serde::__private::Signature::new(0x395d6341)
        }
        #[allow(clippy::let_and_return, clippy::vec_init_then_push)]
        fn deps_indexes(&self) -> Vec<&Pointer<'_>> {
            let mut v = Vec::new();
            v
        }
    }
    impl<'a> _serde::Serialize for hclBendStiffnessConstraintSetMxBatch<'a> {
        fn serialize<S>(&self, __serializer: S) -> Result<S::Ok, S::Error>
        where
            S: _serde::ser::Serializer,
        {
            let class_meta = self
                .__ptr
                .as_ref()
                .map(|name| (name, _serde::__private::Signature::new(0x395d6341)));
            let mut serializer = __serializer
                .serialize_struct(
                    "hclBendStiffnessConstraintSetMxBatch",
                    class_meta,
                    (0u64, 768u64),
                )?;
            serializer
                .serialize_fixed_array_field(
                    "weightsA",
                    self.m_weightsA.as_slice(),
                    TypeSize::NonPtr,
                )?;
            serializer
                .serialize_fixed_array_field(
                    "weightsB",
                    self.m_weightsB.as_slice(),
                    TypeSize::NonPtr,
                )?;
            serializer
                .serialize_fixed_array_field(
                    "weightsC",
                    self.m_weightsC.as_slice(),
                    TypeSize::NonPtr,
                )?;
            serializer
                .serialize_fixed_array_field(
                    "weightsD",
                    self.m_weightsD.as_slice(),
                    TypeSize::NonPtr,
                )?;
            serializer
                .serialize_fixed_array_field(
                    "bendStiffnesses",
                    self.m_bendStiffnesses.as_slice(),
                    TypeSize::NonPtr,
                )?;
            serializer
                .serialize_fixed_array_field(
                    "restCurvatures",
                    self.m_restCurvatures.as_slice(),
                    TypeSize::NonPtr,
                )?;
            serializer
                .serialize_fixed_array_field(
                    "invMassesA",
                    self.m_invMassesA.as_slice(),
                    TypeSize::NonPtr,
                )?;
            serializer
                .serialize_fixed_array_field(
                    "invMassesB",
                    self.m_invMassesB.as_slice(),
                    TypeSize::NonPtr,
                )?;
            serializer
                .serialize_fixed_array_field(
                    "invMassesC",
                    self.m_invMassesC.as_slice(),
                    TypeSize::NonPtr,
                )?;
            serializer
                .serialize_fixed_array_field(
                    "invMassesD",
                    self.m_invMassesD.as_slice(),
                    TypeSize::NonPtr,
                )?;
            serializer
                .serialize_fixed_array_field(
                    "particlesA",
                    self.m_particlesA.as_slice(),
                    TypeSize::NonPtr,
                )?;
            serializer
                .serialize_fixed_array_field(
                    "particlesB",
                    self.m_particlesB.as_slice(),
                    TypeSize::NonPtr,
                )?;
            serializer
                .serialize_fixed_array_field(
                    "particlesC",
                    self.m_particlesC.as_slice(),
                    TypeSize::NonPtr,
                )?;
            serializer
                .serialize_fixed_array_field(
                    "particlesD",
                    self.m_particlesD.as_slice(),
                    TypeSize::NonPtr,
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
    impl<'de> _serde::Deserialize<'de> for hclBendStiffnessConstraintSetMxBatch<'de> {
        fn deserialize<__D>(deserializer: __D) -> core::result::Result<Self, __D::Error>
        where
            __D: _serde::Deserializer<'de>,
        {
            #[allow(non_camel_case_types)]
            enum __Field {
                m_weightsA,
                m_weightsB,
                m_weightsC,
                m_weightsD,
                m_bendStiffnesses,
                m_restCurvatures,
                m_invMassesA,
                m_invMassesB,
                m_invMassesC,
                m_invMassesD,
                m_particlesA,
                m_particlesB,
                m_particlesC,
                m_particlesD,
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
                        "weightsA" => Ok(__Field::m_weightsA),
                        "weightsB" => Ok(__Field::m_weightsB),
                        "weightsC" => Ok(__Field::m_weightsC),
                        "weightsD" => Ok(__Field::m_weightsD),
                        "bendStiffnesses" => Ok(__Field::m_bendStiffnesses),
                        "restCurvatures" => Ok(__Field::m_restCurvatures),
                        "invMassesA" => Ok(__Field::m_invMassesA),
                        "invMassesB" => Ok(__Field::m_invMassesB),
                        "invMassesC" => Ok(__Field::m_invMassesC),
                        "invMassesD" => Ok(__Field::m_invMassesD),
                        "particlesA" => Ok(__Field::m_particlesA),
                        "particlesB" => Ok(__Field::m_particlesB),
                        "particlesC" => Ok(__Field::m_particlesC),
                        "particlesD" => Ok(__Field::m_particlesD),
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
            struct __hclBendStiffnessConstraintSetMxBatchVisitor<'de> {
                marker: _serde::__private::PhantomData<
                    hclBendStiffnessConstraintSetMxBatch<'de>,
                >,
                lifetime: _serde::__private::PhantomData<&'de ()>,
            }
            #[allow(clippy::match_single_binding)]
            #[allow(clippy::reversed_empty_ranges)]
            #[allow(clippy::single_match)]
            impl<'de> _serde::de::Visitor<'de>
            for __hclBendStiffnessConstraintSetMxBatchVisitor<'de> {
                type Value = hclBendStiffnessConstraintSetMxBatch<'de>;
                fn expecting(
                    &self,
                    __formatter: &mut core::fmt::Formatter,
                ) -> core::fmt::Result {
                    core::fmt::Formatter::write_str(
                        __formatter,
                        "struct hclBendStiffnessConstraintSetMxBatch",
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
                    let mut m_weightsA: _serde::__private::Option<[f32; 16usize]> = _serde::__private::None;
                    let mut m_weightsB: _serde::__private::Option<[f32; 16usize]> = _serde::__private::None;
                    let mut m_weightsC: _serde::__private::Option<[f32; 16usize]> = _serde::__private::None;
                    let mut m_weightsD: _serde::__private::Option<[f32; 16usize]> = _serde::__private::None;
                    let mut m_bendStiffnesses: _serde::__private::Option<
                        [f32; 16usize],
                    > = _serde::__private::None;
                    let mut m_restCurvatures: _serde::__private::Option<
                        [f32; 16usize],
                    > = _serde::__private::None;
                    let mut m_invMassesA: _serde::__private::Option<[f32; 16usize]> = _serde::__private::None;
                    let mut m_invMassesB: _serde::__private::Option<[f32; 16usize]> = _serde::__private::None;
                    let mut m_invMassesC: _serde::__private::Option<[f32; 16usize]> = _serde::__private::None;
                    let mut m_invMassesD: _serde::__private::Option<[f32; 16usize]> = _serde::__private::None;
                    let mut m_particlesA: _serde::__private::Option<
                        [U16<'de>; 16usize],
                    > = _serde::__private::None;
                    let mut m_particlesB: _serde::__private::Option<
                        [U16<'de>; 16usize],
                    > = _serde::__private::None;
                    let mut m_particlesC: _serde::__private::Option<
                        [U16<'de>; 16usize],
                    > = _serde::__private::None;
                    let mut m_particlesD: _serde::__private::Option<
                        [U16<'de>; 16usize],
                    > = _serde::__private::None;
                    for i in 0..14usize {
                        match i {
                            0usize => {
                                if _serde::__private::Option::is_some(&m_weightsA) {
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "weightsA",
                                        ),
                                    );
                                }
                                m_weightsA = _serde::__private::Some(
                                    match __A::next_value::<[f32; 16usize]>(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            1usize => {
                                if _serde::__private::Option::is_some(&m_weightsB) {
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "weightsB",
                                        ),
                                    );
                                }
                                m_weightsB = _serde::__private::Some(
                                    match __A::next_value::<[f32; 16usize]>(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            2usize => {
                                if _serde::__private::Option::is_some(&m_weightsC) {
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "weightsC",
                                        ),
                                    );
                                }
                                m_weightsC = _serde::__private::Some(
                                    match __A::next_value::<[f32; 16usize]>(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            3usize => {
                                if _serde::__private::Option::is_some(&m_weightsD) {
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "weightsD",
                                        ),
                                    );
                                }
                                m_weightsD = _serde::__private::Some(
                                    match __A::next_value::<[f32; 16usize]>(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            4usize => {
                                if _serde::__private::Option::is_some(&m_bendStiffnesses) {
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "bendStiffnesses",
                                        ),
                                    );
                                }
                                m_bendStiffnesses = _serde::__private::Some(
                                    match __A::next_value::<[f32; 16usize]>(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            5usize => {
                                if _serde::__private::Option::is_some(&m_restCurvatures) {
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "restCurvatures",
                                        ),
                                    );
                                }
                                m_restCurvatures = _serde::__private::Some(
                                    match __A::next_value::<[f32; 16usize]>(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            6usize => {
                                if _serde::__private::Option::is_some(&m_invMassesA) {
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "invMassesA",
                                        ),
                                    );
                                }
                                m_invMassesA = _serde::__private::Some(
                                    match __A::next_value::<[f32; 16usize]>(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            7usize => {
                                if _serde::__private::Option::is_some(&m_invMassesB) {
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "invMassesB",
                                        ),
                                    );
                                }
                                m_invMassesB = _serde::__private::Some(
                                    match __A::next_value::<[f32; 16usize]>(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            8usize => {
                                if _serde::__private::Option::is_some(&m_invMassesC) {
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "invMassesC",
                                        ),
                                    );
                                }
                                m_invMassesC = _serde::__private::Some(
                                    match __A::next_value::<[f32; 16usize]>(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            9usize => {
                                if _serde::__private::Option::is_some(&m_invMassesD) {
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "invMassesD",
                                        ),
                                    );
                                }
                                m_invMassesD = _serde::__private::Some(
                                    match __A::next_value::<[f32; 16usize]>(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            10usize => {
                                if _serde::__private::Option::is_some(&m_particlesA) {
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "particlesA",
                                        ),
                                    );
                                }
                                m_particlesA = _serde::__private::Some(
                                    match __A::next_value::<[U16<'de>; 16usize]>(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            11usize => {
                                if _serde::__private::Option::is_some(&m_particlesB) {
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "particlesB",
                                        ),
                                    );
                                }
                                m_particlesB = _serde::__private::Some(
                                    match __A::next_value::<[U16<'de>; 16usize]>(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            12usize => {
                                if _serde::__private::Option::is_some(&m_particlesC) {
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "particlesC",
                                        ),
                                    );
                                }
                                m_particlesC = _serde::__private::Some(
                                    match __A::next_value::<[U16<'de>; 16usize]>(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            13usize => {
                                if _serde::__private::Option::is_some(&m_particlesD) {
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "particlesD",
                                        ),
                                    );
                                }
                                m_particlesD = _serde::__private::Some(
                                    match __A::next_value::<[U16<'de>; 16usize]>(&mut __map) {
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
                    let m_weightsA = match m_weightsA {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field("weightsA"),
                            );
                        }
                    };
                    let m_weightsB = match m_weightsB {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field("weightsB"),
                            );
                        }
                    };
                    let m_weightsC = match m_weightsC {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field("weightsC"),
                            );
                        }
                    };
                    let m_weightsD = match m_weightsD {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field("weightsD"),
                            );
                        }
                    };
                    let m_bendStiffnesses = match m_bendStiffnesses {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "bendStiffnesses",
                                ),
                            );
                        }
                    };
                    let m_restCurvatures = match m_restCurvatures {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "restCurvatures",
                                ),
                            );
                        }
                    };
                    let m_invMassesA = match m_invMassesA {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "invMassesA",
                                ),
                            );
                        }
                    };
                    let m_invMassesB = match m_invMassesB {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "invMassesB",
                                ),
                            );
                        }
                    };
                    let m_invMassesC = match m_invMassesC {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "invMassesC",
                                ),
                            );
                        }
                    };
                    let m_invMassesD = match m_invMassesD {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "invMassesD",
                                ),
                            );
                        }
                    };
                    let m_particlesA = match m_particlesA {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "particlesA",
                                ),
                            );
                        }
                    };
                    let m_particlesB = match m_particlesB {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "particlesB",
                                ),
                            );
                        }
                    };
                    let m_particlesC = match m_particlesC {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "particlesC",
                                ),
                            );
                        }
                    };
                    let m_particlesD = match m_particlesD {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "particlesD",
                                ),
                            );
                        }
                    };
                    _serde::__private::Ok(hclBendStiffnessConstraintSetMxBatch {
                        __ptr,
                        m_weightsA,
                        m_weightsB,
                        m_weightsC,
                        m_weightsD,
                        m_bendStiffnesses,
                        m_restCurvatures,
                        m_invMassesA,
                        m_invMassesB,
                        m_invMassesC,
                        m_invMassesD,
                        m_particlesA,
                        m_particlesB,
                        m_particlesC,
                        m_particlesD,
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
                    let mut m_weightsA: _serde::__private::Option<[f32; 16usize]> = _serde::__private::None;
                    let mut m_weightsB: _serde::__private::Option<[f32; 16usize]> = _serde::__private::None;
                    let mut m_weightsC: _serde::__private::Option<[f32; 16usize]> = _serde::__private::None;
                    let mut m_weightsD: _serde::__private::Option<[f32; 16usize]> = _serde::__private::None;
                    let mut m_bendStiffnesses: _serde::__private::Option<
                        [f32; 16usize],
                    > = _serde::__private::None;
                    let mut m_restCurvatures: _serde::__private::Option<
                        [f32; 16usize],
                    > = _serde::__private::None;
                    let mut m_invMassesA: _serde::__private::Option<[f32; 16usize]> = _serde::__private::None;
                    let mut m_invMassesB: _serde::__private::Option<[f32; 16usize]> = _serde::__private::None;
                    let mut m_invMassesC: _serde::__private::Option<[f32; 16usize]> = _serde::__private::None;
                    let mut m_invMassesD: _serde::__private::Option<[f32; 16usize]> = _serde::__private::None;
                    let mut m_particlesA: _serde::__private::Option<
                        [U16<'de>; 16usize],
                    > = _serde::__private::None;
                    let mut m_particlesB: _serde::__private::Option<
                        [U16<'de>; 16usize],
                    > = _serde::__private::None;
                    let mut m_particlesC: _serde::__private::Option<
                        [U16<'de>; 16usize],
                    > = _serde::__private::None;
                    let mut m_particlesD: _serde::__private::Option<
                        [U16<'de>; 16usize],
                    > = _serde::__private::None;
                    while let _serde::__private::Some(__key) = {
                        __A::next_key::<__Field>(&mut __map)?
                    } {
                        match __key {
                            __Field::m_weightsA => {
                                #[cfg(
                                    any(feature = "strict", feature = "ignore_duplicates")
                                )]
                                if _serde::__private::Option::is_some(&m_weightsA) {
                                    #[cfg(feature = "ignore_duplicates")]
                                    {
                                        __A::skip_value(&mut __map)?;
                                        continue;
                                    }
                                    #[cfg(feature = "strict")]
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "weightsA",
                                        ),
                                    );
                                }
                                m_weightsA = _serde::__private::Some(
                                    match __A::next_value::<[f32; 16usize]>(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            __Field::m_weightsB => {
                                #[cfg(
                                    any(feature = "strict", feature = "ignore_duplicates")
                                )]
                                if _serde::__private::Option::is_some(&m_weightsB) {
                                    #[cfg(feature = "ignore_duplicates")]
                                    {
                                        __A::skip_value(&mut __map)?;
                                        continue;
                                    }
                                    #[cfg(feature = "strict")]
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "weightsB",
                                        ),
                                    );
                                }
                                m_weightsB = _serde::__private::Some(
                                    match __A::next_value::<[f32; 16usize]>(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            __Field::m_weightsC => {
                                #[cfg(
                                    any(feature = "strict", feature = "ignore_duplicates")
                                )]
                                if _serde::__private::Option::is_some(&m_weightsC) {
                                    #[cfg(feature = "ignore_duplicates")]
                                    {
                                        __A::skip_value(&mut __map)?;
                                        continue;
                                    }
                                    #[cfg(feature = "strict")]
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "weightsC",
                                        ),
                                    );
                                }
                                m_weightsC = _serde::__private::Some(
                                    match __A::next_value::<[f32; 16usize]>(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            __Field::m_weightsD => {
                                #[cfg(
                                    any(feature = "strict", feature = "ignore_duplicates")
                                )]
                                if _serde::__private::Option::is_some(&m_weightsD) {
                                    #[cfg(feature = "ignore_duplicates")]
                                    {
                                        __A::skip_value(&mut __map)?;
                                        continue;
                                    }
                                    #[cfg(feature = "strict")]
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "weightsD",
                                        ),
                                    );
                                }
                                m_weightsD = _serde::__private::Some(
                                    match __A::next_value::<[f32; 16usize]>(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            __Field::m_bendStiffnesses => {
                                #[cfg(
                                    any(feature = "strict", feature = "ignore_duplicates")
                                )]
                                if _serde::__private::Option::is_some(&m_bendStiffnesses) {
                                    #[cfg(feature = "ignore_duplicates")]
                                    {
                                        __A::skip_value(&mut __map)?;
                                        continue;
                                    }
                                    #[cfg(feature = "strict")]
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "bendStiffnesses",
                                        ),
                                    );
                                }
                                m_bendStiffnesses = _serde::__private::Some(
                                    match __A::next_value::<[f32; 16usize]>(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            __Field::m_restCurvatures => {
                                #[cfg(
                                    any(feature = "strict", feature = "ignore_duplicates")
                                )]
                                if _serde::__private::Option::is_some(&m_restCurvatures) {
                                    #[cfg(feature = "ignore_duplicates")]
                                    {
                                        __A::skip_value(&mut __map)?;
                                        continue;
                                    }
                                    #[cfg(feature = "strict")]
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "restCurvatures",
                                        ),
                                    );
                                }
                                m_restCurvatures = _serde::__private::Some(
                                    match __A::next_value::<[f32; 16usize]>(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            __Field::m_invMassesA => {
                                #[cfg(
                                    any(feature = "strict", feature = "ignore_duplicates")
                                )]
                                if _serde::__private::Option::is_some(&m_invMassesA) {
                                    #[cfg(feature = "ignore_duplicates")]
                                    {
                                        __A::skip_value(&mut __map)?;
                                        continue;
                                    }
                                    #[cfg(feature = "strict")]
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "invMassesA",
                                        ),
                                    );
                                }
                                m_invMassesA = _serde::__private::Some(
                                    match __A::next_value::<[f32; 16usize]>(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            __Field::m_invMassesB => {
                                #[cfg(
                                    any(feature = "strict", feature = "ignore_duplicates")
                                )]
                                if _serde::__private::Option::is_some(&m_invMassesB) {
                                    #[cfg(feature = "ignore_duplicates")]
                                    {
                                        __A::skip_value(&mut __map)?;
                                        continue;
                                    }
                                    #[cfg(feature = "strict")]
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "invMassesB",
                                        ),
                                    );
                                }
                                m_invMassesB = _serde::__private::Some(
                                    match __A::next_value::<[f32; 16usize]>(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            __Field::m_invMassesC => {
                                #[cfg(
                                    any(feature = "strict", feature = "ignore_duplicates")
                                )]
                                if _serde::__private::Option::is_some(&m_invMassesC) {
                                    #[cfg(feature = "ignore_duplicates")]
                                    {
                                        __A::skip_value(&mut __map)?;
                                        continue;
                                    }
                                    #[cfg(feature = "strict")]
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "invMassesC",
                                        ),
                                    );
                                }
                                m_invMassesC = _serde::__private::Some(
                                    match __A::next_value::<[f32; 16usize]>(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            __Field::m_invMassesD => {
                                #[cfg(
                                    any(feature = "strict", feature = "ignore_duplicates")
                                )]
                                if _serde::__private::Option::is_some(&m_invMassesD) {
                                    #[cfg(feature = "ignore_duplicates")]
                                    {
                                        __A::skip_value(&mut __map)?;
                                        continue;
                                    }
                                    #[cfg(feature = "strict")]
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "invMassesD",
                                        ),
                                    );
                                }
                                m_invMassesD = _serde::__private::Some(
                                    match __A::next_value::<[f32; 16usize]>(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            __Field::m_particlesA => {
                                #[cfg(
                                    any(feature = "strict", feature = "ignore_duplicates")
                                )]
                                if _serde::__private::Option::is_some(&m_particlesA) {
                                    #[cfg(feature = "ignore_duplicates")]
                                    {
                                        __A::skip_value(&mut __map)?;
                                        continue;
                                    }
                                    #[cfg(feature = "strict")]
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "particlesA",
                                        ),
                                    );
                                }
                                m_particlesA = _serde::__private::Some(
                                    match __A::next_value::<[U16<'de>; 16usize]>(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            __Field::m_particlesB => {
                                #[cfg(
                                    any(feature = "strict", feature = "ignore_duplicates")
                                )]
                                if _serde::__private::Option::is_some(&m_particlesB) {
                                    #[cfg(feature = "ignore_duplicates")]
                                    {
                                        __A::skip_value(&mut __map)?;
                                        continue;
                                    }
                                    #[cfg(feature = "strict")]
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "particlesB",
                                        ),
                                    );
                                }
                                m_particlesB = _serde::__private::Some(
                                    match __A::next_value::<[U16<'de>; 16usize]>(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            __Field::m_particlesC => {
                                #[cfg(
                                    any(feature = "strict", feature = "ignore_duplicates")
                                )]
                                if _serde::__private::Option::is_some(&m_particlesC) {
                                    #[cfg(feature = "ignore_duplicates")]
                                    {
                                        __A::skip_value(&mut __map)?;
                                        continue;
                                    }
                                    #[cfg(feature = "strict")]
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "particlesC",
                                        ),
                                    );
                                }
                                m_particlesC = _serde::__private::Some(
                                    match __A::next_value::<[U16<'de>; 16usize]>(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            __Field::m_particlesD => {
                                #[cfg(
                                    any(feature = "strict", feature = "ignore_duplicates")
                                )]
                                if _serde::__private::Option::is_some(&m_particlesD) {
                                    #[cfg(feature = "ignore_duplicates")]
                                    {
                                        __A::skip_value(&mut __map)?;
                                        continue;
                                    }
                                    #[cfg(feature = "strict")]
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "particlesD",
                                        ),
                                    );
                                }
                                m_particlesD = _serde::__private::Some(
                                    match __A::next_value::<[U16<'de>; 16usize]>(&mut __map) {
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
                    let m_weightsA = match m_weightsA {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            #[cfg(feature = "strict")]
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field("weightsA"),
                            );
                            #[cfg(not(feature = "strict"))] Default::default()
                        }
                    };
                    let m_weightsB = match m_weightsB {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            #[cfg(feature = "strict")]
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field("weightsB"),
                            );
                            #[cfg(not(feature = "strict"))] Default::default()
                        }
                    };
                    let m_weightsC = match m_weightsC {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            #[cfg(feature = "strict")]
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field("weightsC"),
                            );
                            #[cfg(not(feature = "strict"))] Default::default()
                        }
                    };
                    let m_weightsD = match m_weightsD {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            #[cfg(feature = "strict")]
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field("weightsD"),
                            );
                            #[cfg(not(feature = "strict"))] Default::default()
                        }
                    };
                    let m_bendStiffnesses = match m_bendStiffnesses {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            #[cfg(feature = "strict")]
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "bendStiffnesses",
                                ),
                            );
                            #[cfg(not(feature = "strict"))] Default::default()
                        }
                    };
                    let m_restCurvatures = match m_restCurvatures {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            #[cfg(feature = "strict")]
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "restCurvatures",
                                ),
                            );
                            #[cfg(not(feature = "strict"))] Default::default()
                        }
                    };
                    let m_invMassesA = match m_invMassesA {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            #[cfg(feature = "strict")]
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "invMassesA",
                                ),
                            );
                            #[cfg(not(feature = "strict"))] Default::default()
                        }
                    };
                    let m_invMassesB = match m_invMassesB {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            #[cfg(feature = "strict")]
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "invMassesB",
                                ),
                            );
                            #[cfg(not(feature = "strict"))] Default::default()
                        }
                    };
                    let m_invMassesC = match m_invMassesC {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            #[cfg(feature = "strict")]
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "invMassesC",
                                ),
                            );
                            #[cfg(not(feature = "strict"))] Default::default()
                        }
                    };
                    let m_invMassesD = match m_invMassesD {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            #[cfg(feature = "strict")]
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "invMassesD",
                                ),
                            );
                            #[cfg(not(feature = "strict"))] Default::default()
                        }
                    };
                    let m_particlesA = match m_particlesA {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            #[cfg(feature = "strict")]
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "particlesA",
                                ),
                            );
                            #[cfg(not(feature = "strict"))] Default::default()
                        }
                    };
                    let m_particlesB = match m_particlesB {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            #[cfg(feature = "strict")]
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "particlesB",
                                ),
                            );
                            #[cfg(not(feature = "strict"))] Default::default()
                        }
                    };
                    let m_particlesC = match m_particlesC {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            #[cfg(feature = "strict")]
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "particlesC",
                                ),
                            );
                            #[cfg(not(feature = "strict"))] Default::default()
                        }
                    };
                    let m_particlesD = match m_particlesD {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            #[cfg(feature = "strict")]
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "particlesD",
                                ),
                            );
                            #[cfg(not(feature = "strict"))] Default::default()
                        }
                    };
                    let __ptr = __A::class_ptr(&mut __map);
                    _serde::__private::Ok(hclBendStiffnessConstraintSetMxBatch {
                        __ptr: __ptr.clone(),
                        m_weightsA,
                        m_weightsB,
                        m_weightsC,
                        m_weightsD,
                        m_bendStiffnesses,
                        m_restCurvatures,
                        m_invMassesA,
                        m_invMassesB,
                        m_invMassesC,
                        m_invMassesD,
                        m_particlesA,
                        m_particlesB,
                        m_particlesC,
                        m_particlesD,
                    })
                }
            }
            const FIELDS: &[&str] = &[
                "weightsA",
                "weightsB",
                "weightsC",
                "weightsD",
                "bendStiffnesses",
                "restCurvatures",
                "invMassesA",
                "invMassesB",
                "invMassesC",
                "invMassesD",
                "particlesA",
                "particlesB",
                "particlesC",
                "particlesD",
            ];
            _serde::Deserializer::deserialize_struct(
                deserializer,
                "hclBendStiffnessConstraintSetMxBatch",
                FIELDS,
                __hclBendStiffnessConstraintSetMxBatchVisitor {
                    marker: _serde::__private::PhantomData::<
                        hclBendStiffnessConstraintSetMxBatch,
                    >,
                    lifetime: _serde::__private::PhantomData,
                },
            )
        }
    }
};
