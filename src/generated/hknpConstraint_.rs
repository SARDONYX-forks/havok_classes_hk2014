use super::class_requires::*;
use super::*;
/// # C++ Info
/// - name: `hknpConstraint`
/// - version: `0`
/// - signature: `0x68e3b294`
/// - size: `  0`(x86)/` 56`(x86_64)
/// -  vtable: `false`
#[allow(non_upper_case_globals, non_snake_case)]
#[cfg_attr(feature = "json_schema", derive(schemars::JsonSchema))]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(educe::Educe)]
#[educe(Debug, Clone, Default, PartialEq)]
pub struct hknpConstraint<'a> {
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
    /// - name: `bodyIdA`(ctype: `hkUint32`)
    /// - offset: `  0`(x86)/`  0`(x86_64)
    /// - type_size: `  0`(x86)/`  4`(x86_64)
    #[cfg_attr(feature = "serde_default", serde(default))]
    #[cfg_attr(feature = "serde", serde(borrow))]
    #[cfg_attr(feature = "json_schema", schemars(rename = "bodyIdA"))]
    #[cfg_attr(feature = "serde", serde(rename = "bodyIdA"))]
    pub m_bodyIdA: U32<'a>,
    /// # C++ Info
    /// - name: `bodyIdB`(ctype: `hkUint32`)
    /// - offset: `  0`(x86)/`  4`(x86_64)
    /// - type_size: `  0`(x86)/`  4`(x86_64)
    #[cfg_attr(feature = "serde_default", serde(default))]
    #[cfg_attr(feature = "serde", serde(borrow))]
    #[cfg_attr(feature = "json_schema", schemars(rename = "bodyIdB"))]
    #[cfg_attr(feature = "serde", serde(rename = "bodyIdB"))]
    pub m_bodyIdB: U32<'a>,
    /// # C++ Info
    /// - name: `data`(ctype: `struct hkpConstraintData*`)
    /// - offset: `  0`(x86)/`  8`(x86_64)
    /// - type_size: `  0`(x86)/`  8`(x86_64)
    #[cfg_attr(feature = "serde_default", serde(default))]
    #[cfg_attr(feature = "serde", serde(borrow))]
    #[cfg_attr(feature = "json_schema", schemars(rename = "data"))]
    #[cfg_attr(feature = "serde", serde(rename = "data"))]
    pub m_data: Pointer<'a>,
    /// # C++ Info
    /// - name: `id`(ctype: `hkUint32`)
    /// - offset: `  0`(x86)/` 16`(x86_64)
    /// - type_size: `  0`(x86)/`  4`(x86_64)
    #[cfg_attr(feature = "serde_default", serde(default))]
    #[cfg_attr(feature = "serde", serde(borrow))]
    #[cfg_attr(feature = "json_schema", schemars(rename = "id"))]
    #[cfg_attr(feature = "serde", serde(rename = "id"))]
    pub m_id: U32<'a>,
    /// # C++ Info
    /// - name: `immediateId`(ctype: `hkUint16`)
    /// - offset: `  0`(x86)/` 20`(x86_64)
    /// - type_size: `  0`(x86)/`  2`(x86_64)
    /// - flags: `SERIALIZE_IGNORED`
    #[cfg_attr(feature = "serde", serde(default))]
    #[cfg_attr(feature = "serde", serde(borrow))]
    #[cfg_attr(feature = "json_schema", schemars(rename = "immediateId"))]
    #[cfg_attr(feature = "serde", serde(rename = "immediateId"))]
    pub m_immediateId: U16<'a>,
    /// # C++ Info
    /// - name: `flags`(ctype: `flags FlagsEnum`)
    /// - offset: `  0`(x86)/` 22`(x86_64)
    /// - type_size: `  0`(x86)/`  1`(x86_64)
    #[cfg_attr(feature = "serde_default", serde(default))]
    #[cfg_attr(feature = "json_schema", schemars(rename = "flags"))]
    #[cfg_attr(feature = "serde", serde(rename = "flags"))]
    pub m_flags: FlagsEnum,
    /// # C++ Info
    /// - name: `type`(ctype: `enum ConstraintType`)
    /// - offset: `  0`(x86)/` 23`(x86_64)
    /// - type_size: `  0`(x86)/`  1`(x86_64)
    #[cfg_attr(feature = "serde_default", serde(default))]
    #[cfg_attr(feature = "json_schema", schemars(rename = "type"))]
    #[cfg_attr(feature = "serde", serde(rename = "type"))]
    pub m_type: ConstraintType,
    /// # C++ Info
    /// - name: `atoms`(ctype: `void*`)
    /// - offset: `  0`(x86)/` 24`(x86_64)
    /// - type_size: `  0`(x86)/`  8`(x86_64)
    /// - flags: `SERIALIZE_IGNORED`
    #[cfg_attr(feature = "serde", serde(default))]
    #[cfg_attr(feature = "serde", serde(borrow))]
    #[cfg_attr(feature = "json_schema", schemars(rename = "atoms"))]
    #[cfg_attr(feature = "serde", serde(rename = "atoms"))]
    pub m_atoms: Pointer<'a>,
    /// # C++ Info
    /// - name: `sizeOfAtoms`(ctype: `hkUint16`)
    /// - offset: `  0`(x86)/` 32`(x86_64)
    /// - type_size: `  0`(x86)/`  2`(x86_64)
    #[cfg_attr(feature = "serde_default", serde(default))]
    #[cfg_attr(feature = "serde", serde(borrow))]
    #[cfg_attr(feature = "json_schema", schemars(rename = "sizeOfAtoms"))]
    #[cfg_attr(feature = "serde", serde(rename = "sizeOfAtoms"))]
    pub m_sizeOfAtoms: U16<'a>,
    /// # C++ Info
    /// - name: `sizeOfSchemas`(ctype: `hkUint16`)
    /// - offset: `  0`(x86)/` 34`(x86_64)
    /// - type_size: `  0`(x86)/`  2`(x86_64)
    #[cfg_attr(feature = "serde_default", serde(default))]
    #[cfg_attr(feature = "serde", serde(borrow))]
    #[cfg_attr(feature = "json_schema", schemars(rename = "sizeOfSchemas"))]
    #[cfg_attr(feature = "serde", serde(rename = "sizeOfSchemas"))]
    pub m_sizeOfSchemas: U16<'a>,
    /// # C++ Info
    /// - name: `numSolverResults`(ctype: `hkUint8`)
    /// - offset: `  0`(x86)/` 36`(x86_64)
    /// - type_size: `  0`(x86)/`  1`(x86_64)
    #[cfg_attr(feature = "serde_default", serde(default))]
    #[cfg_attr(feature = "serde", serde(borrow))]
    #[cfg_attr(feature = "json_schema", schemars(rename = "numSolverResults"))]
    #[cfg_attr(feature = "serde", serde(rename = "numSolverResults"))]
    pub m_numSolverResults: U8<'a>,
    /// # C++ Info
    /// - name: `numSolverElemTemps`(ctype: `hkUint8`)
    /// - offset: `  0`(x86)/` 37`(x86_64)
    /// - type_size: `  0`(x86)/`  1`(x86_64)
    #[cfg_attr(feature = "serde_default", serde(default))]
    #[cfg_attr(feature = "serde", serde(borrow))]
    #[cfg_attr(feature = "json_schema", schemars(rename = "numSolverElemTemps"))]
    #[cfg_attr(feature = "serde", serde(rename = "numSolverElemTemps"))]
    pub m_numSolverElemTemps: U8<'a>,
    /// # C++ Info
    /// - name: `runtimeSize`(ctype: `hkUint16`)
    /// - offset: `  0`(x86)/` 38`(x86_64)
    /// - type_size: `  0`(x86)/`  2`(x86_64)
    #[cfg_attr(feature = "serde_default", serde(default))]
    #[cfg_attr(feature = "serde", serde(borrow))]
    #[cfg_attr(feature = "json_schema", schemars(rename = "runtimeSize"))]
    #[cfg_attr(feature = "serde", serde(rename = "runtimeSize"))]
    pub m_runtimeSize: U16<'a>,
    /// # C++ Info
    /// - name: `runtime`(ctype: `void*`)
    /// - offset: `  0`(x86)/` 40`(x86_64)
    /// - type_size: `  0`(x86)/`  8`(x86_64)
    /// - flags: `SERIALIZE_IGNORED`
    #[cfg_attr(feature = "serde", serde(default))]
    #[cfg_attr(feature = "serde", serde(borrow))]
    #[cfg_attr(feature = "json_schema", schemars(rename = "runtime"))]
    #[cfg_attr(feature = "serde", serde(rename = "runtime"))]
    pub m_runtime: Pointer<'a>,
    /// # C++ Info
    /// - name: `userData`(ctype: `hkUint64`)
    /// - offset: `  0`(x86)/` 48`(x86_64)
    /// - type_size: `  0`(x86)/`  8`(x86_64)
    #[cfg_attr(feature = "serde_default", serde(default))]
    #[cfg_attr(feature = "serde", serde(borrow))]
    #[cfg_attr(feature = "json_schema", schemars(rename = "userData"))]
    #[cfg_attr(feature = "serde", serde(rename = "userData"))]
    pub m_userData: U64<'a>,
}
const _: () = {
    use havok_serde as _serde;
    impl<'a> _serde::HavokClass for hknpConstraint<'a> {
        #[inline]
        fn name(&self) -> &'static str {
            "hknpConstraint"
        }
        #[inline]
        fn signature(&self) -> _serde::__private::Signature {
            _serde::__private::Signature::new(0x68e3b294)
        }
        #[allow(clippy::let_and_return, clippy::vec_init_then_push)]
        fn deps_indexes(&self) -> Vec<&Pointer<'_>> {
            let mut v = Vec::new();
            v.push(&self.m_data);
            v.push(&self.m_atoms);
            v.push(&self.m_runtime);
            v
        }
    }
    impl<'a> _serde::Serialize for hknpConstraint<'a> {
        fn serialize<S>(&self, __serializer: S) -> Result<S::Ok, S::Error>
        where
            S: _serde::ser::Serializer,
        {
            let class_meta = self
                .__ptr
                .as_ref()
                .map(|name| (name, _serde::__private::Signature::new(0x68e3b294)));
            let mut serializer = __serializer
                .serialize_struct("hknpConstraint", class_meta, (0u64, 56u64))?;
            serializer.serialize_field("bodyIdA", &self.m_bodyIdA)?;
            serializer.serialize_field("bodyIdB", &self.m_bodyIdB)?;
            serializer.serialize_field("data", &self.m_data)?;
            serializer.serialize_field("id", &self.m_id)?;
            serializer.skip_field("immediateId", &self.m_immediateId)?;
            serializer.serialize_field("flags", &self.m_flags)?;
            serializer.serialize_field("type", &self.m_type)?;
            serializer.skip_field("atoms", &self.m_atoms)?;
            serializer.serialize_field("sizeOfAtoms", &self.m_sizeOfAtoms)?;
            serializer.serialize_field("sizeOfSchemas", &self.m_sizeOfSchemas)?;
            serializer.serialize_field("numSolverResults", &self.m_numSolverResults)?;
            serializer
                .serialize_field("numSolverElemTemps", &self.m_numSolverElemTemps)?;
            serializer.serialize_field("runtimeSize", &self.m_runtimeSize)?;
            serializer.skip_field("runtime", &self.m_runtime)?;
            serializer.serialize_field("userData", &self.m_userData)?;
            serializer.end()
        }
    }
};
#[doc(hidden)]
#[allow(non_upper_case_globals, unused_attributes, unused_qualifications)]
const _: () = {
    use havok_serde as _serde;
    #[automatically_derived]
    impl<'de> _serde::Deserialize<'de> for hknpConstraint<'de> {
        fn deserialize<__D>(deserializer: __D) -> core::result::Result<Self, __D::Error>
        where
            __D: _serde::Deserializer<'de>,
        {
            #[allow(non_camel_case_types)]
            enum __Field {
                m_bodyIdA,
                m_bodyIdB,
                m_data,
                m_id,
                m_flags,
                m_type,
                m_sizeOfAtoms,
                m_sizeOfSchemas,
                m_numSolverResults,
                m_numSolverElemTemps,
                m_runtimeSize,
                m_userData,
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
                        "bodyIdA" => Ok(__Field::m_bodyIdA),
                        "bodyIdB" => Ok(__Field::m_bodyIdB),
                        "data" => Ok(__Field::m_data),
                        "id" => Ok(__Field::m_id),
                        "flags" => Ok(__Field::m_flags),
                        "type" => Ok(__Field::m_type),
                        "sizeOfAtoms" => Ok(__Field::m_sizeOfAtoms),
                        "sizeOfSchemas" => Ok(__Field::m_sizeOfSchemas),
                        "numSolverResults" => Ok(__Field::m_numSolverResults),
                        "numSolverElemTemps" => Ok(__Field::m_numSolverElemTemps),
                        "runtimeSize" => Ok(__Field::m_runtimeSize),
                        "userData" => Ok(__Field::m_userData),
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
            struct __hknpConstraintVisitor<'de> {
                marker: _serde::__private::PhantomData<hknpConstraint<'de>>,
                lifetime: _serde::__private::PhantomData<&'de ()>,
            }
            #[allow(clippy::match_single_binding)]
            #[allow(clippy::reversed_empty_ranges)]
            #[allow(clippy::single_match)]
            impl<'de> _serde::de::Visitor<'de> for __hknpConstraintVisitor<'de> {
                type Value = hknpConstraint<'de>;
                fn expecting(
                    &self,
                    __formatter: &mut core::fmt::Formatter,
                ) -> core::fmt::Result {
                    core::fmt::Formatter::write_str(__formatter, "struct hknpConstraint")
                }
                fn visit_struct_for_bytes<__A>(
                    self,
                    mut __map: __A,
                ) -> _serde::__private::Result<Self::Value, __A::Error>
                where
                    __A: _serde::de::MapAccess<'de>,
                {
                    let __ptr = __A::class_ptr(&mut __map);
                    let mut m_bodyIdA: _serde::__private::Option<U32<'de>> = _serde::__private::None;
                    let mut m_bodyIdB: _serde::__private::Option<U32<'de>> = _serde::__private::None;
                    let mut m_data: _serde::__private::Option<Pointer<'de>> = _serde::__private::None;
                    let mut m_id: _serde::__private::Option<U32<'de>> = _serde::__private::None;
                    let mut m_immediateId: _serde::__private::Option<U16<'de>> = _serde::__private::None;
                    let mut m_flags: _serde::__private::Option<FlagsEnum> = _serde::__private::None;
                    let mut m_type: _serde::__private::Option<ConstraintType> = _serde::__private::None;
                    let mut m_atoms: _serde::__private::Option<Pointer<'de>> = _serde::__private::None;
                    let mut m_sizeOfAtoms: _serde::__private::Option<U16<'de>> = _serde::__private::None;
                    let mut m_sizeOfSchemas: _serde::__private::Option<U16<'de>> = _serde::__private::None;
                    let mut m_numSolverResults: _serde::__private::Option<U8<'de>> = _serde::__private::None;
                    let mut m_numSolverElemTemps: _serde::__private::Option<U8<'de>> = _serde::__private::None;
                    let mut m_runtimeSize: _serde::__private::Option<U16<'de>> = _serde::__private::None;
                    let mut m_runtime: _serde::__private::Option<Pointer<'de>> = _serde::__private::None;
                    let mut m_userData: _serde::__private::Option<U64<'de>> = _serde::__private::None;
                    for i in 0..15usize {
                        match i {
                            0usize => {
                                if _serde::__private::Option::is_some(&m_bodyIdA) {
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "bodyIdA",
                                        ),
                                    );
                                }
                                m_bodyIdA = _serde::__private::Some(
                                    match __A::next_value::<U32<'de>>(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            1usize => {
                                if _serde::__private::Option::is_some(&m_bodyIdB) {
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "bodyIdB",
                                        ),
                                    );
                                }
                                m_bodyIdB = _serde::__private::Some(
                                    match __A::next_value::<U32<'de>>(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            2usize => {
                                if _serde::__private::Option::is_some(&m_data) {
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field("data"),
                                    );
                                }
                                m_data = _serde::__private::Some(
                                    match __A::next_value::<Pointer<'de>>(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            3usize => {
                                if _serde::__private::Option::is_some(&m_id) {
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field("id"),
                                    );
                                }
                                m_id = _serde::__private::Some(
                                    match __A::next_value::<U32<'de>>(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            4usize => {
                                if _serde::__private::Option::is_some(&m_immediateId) {
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "immediateId",
                                        ),
                                    );
                                }
                                m_immediateId = _serde::__private::Some(
                                    match __A::next_value::<U16<'de>>(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            5usize => {
                                if _serde::__private::Option::is_some(&m_flags) {
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field("flags"),
                                    );
                                }
                                m_flags = _serde::__private::Some(
                                    match __A::next_value::<FlagsEnum>(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            6usize => {
                                if _serde::__private::Option::is_some(&m_type) {
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field("type"),
                                    );
                                }
                                m_type = _serde::__private::Some(
                                    match __A::next_value::<ConstraintType>(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            7usize => {
                                if _serde::__private::Option::is_some(&m_atoms) {
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field("atoms"),
                                    );
                                }
                                m_atoms = _serde::__private::Some(
                                    match __A::next_value::<Pointer<'de>>(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            8usize => {
                                if _serde::__private::Option::is_some(&m_sizeOfAtoms) {
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "sizeOfAtoms",
                                        ),
                                    );
                                }
                                m_sizeOfAtoms = _serde::__private::Some(
                                    match __A::next_value::<U16<'de>>(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            9usize => {
                                if _serde::__private::Option::is_some(&m_sizeOfSchemas) {
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "sizeOfSchemas",
                                        ),
                                    );
                                }
                                m_sizeOfSchemas = _serde::__private::Some(
                                    match __A::next_value::<U16<'de>>(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            10usize => {
                                if _serde::__private::Option::is_some(&m_numSolverResults) {
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "numSolverResults",
                                        ),
                                    );
                                }
                                m_numSolverResults = _serde::__private::Some(
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
                                    &m_numSolverElemTemps,
                                ) {
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "numSolverElemTemps",
                                        ),
                                    );
                                }
                                m_numSolverElemTemps = _serde::__private::Some(
                                    match __A::next_value::<U8<'de>>(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            12usize => {
                                if _serde::__private::Option::is_some(&m_runtimeSize) {
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "runtimeSize",
                                        ),
                                    );
                                }
                                m_runtimeSize = _serde::__private::Some(
                                    match __A::next_value::<U16<'de>>(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            13usize => {
                                if _serde::__private::Option::is_some(&m_runtime) {
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "runtime",
                                        ),
                                    );
                                }
                                m_runtime = _serde::__private::Some(
                                    match __A::next_value::<Pointer<'de>>(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            14usize => {
                                if _serde::__private::Option::is_some(&m_userData) {
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
                            _ => {}
                        }
                    }
                    let m_bodyIdA = match m_bodyIdA {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field("bodyIdA"),
                            );
                        }
                    };
                    let m_bodyIdB = match m_bodyIdB {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field("bodyIdB"),
                            );
                        }
                    };
                    let m_data = match m_data {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field("data"),
                            );
                        }
                    };
                    let m_id = match m_id {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field("id"),
                            );
                        }
                    };
                    let m_immediateId = match m_immediateId {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "immediateId",
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
                    let m_type = match m_type {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field("type"),
                            );
                        }
                    };
                    let m_atoms = match m_atoms {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field("atoms"),
                            );
                        }
                    };
                    let m_sizeOfAtoms = match m_sizeOfAtoms {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "sizeOfAtoms",
                                ),
                            );
                        }
                    };
                    let m_sizeOfSchemas = match m_sizeOfSchemas {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "sizeOfSchemas",
                                ),
                            );
                        }
                    };
                    let m_numSolverResults = match m_numSolverResults {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "numSolverResults",
                                ),
                            );
                        }
                    };
                    let m_numSolverElemTemps = match m_numSolverElemTemps {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "numSolverElemTemps",
                                ),
                            );
                        }
                    };
                    let m_runtimeSize = match m_runtimeSize {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "runtimeSize",
                                ),
                            );
                        }
                    };
                    let m_runtime = match m_runtime {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field("runtime"),
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
                    _serde::__private::Ok(hknpConstraint {
                        __ptr,
                        m_bodyIdA,
                        m_bodyIdB,
                        m_data,
                        m_id,
                        m_immediateId,
                        m_flags,
                        m_type,
                        m_atoms,
                        m_sizeOfAtoms,
                        m_sizeOfSchemas,
                        m_numSolverResults,
                        m_numSolverElemTemps,
                        m_runtimeSize,
                        m_runtime,
                        m_userData,
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
                    let mut m_bodyIdA: _serde::__private::Option<U32<'de>> = _serde::__private::None;
                    let mut m_bodyIdB: _serde::__private::Option<U32<'de>> = _serde::__private::None;
                    let mut m_data: _serde::__private::Option<Pointer<'de>> = _serde::__private::None;
                    let mut m_id: _serde::__private::Option<U32<'de>> = _serde::__private::None;
                    let mut m_flags: _serde::__private::Option<FlagsEnum> = _serde::__private::None;
                    let mut m_type: _serde::__private::Option<ConstraintType> = _serde::__private::None;
                    let mut m_sizeOfAtoms: _serde::__private::Option<U16<'de>> = _serde::__private::None;
                    let mut m_sizeOfSchemas: _serde::__private::Option<U16<'de>> = _serde::__private::None;
                    let mut m_numSolverResults: _serde::__private::Option<U8<'de>> = _serde::__private::None;
                    let mut m_numSolverElemTemps: _serde::__private::Option<U8<'de>> = _serde::__private::None;
                    let mut m_runtimeSize: _serde::__private::Option<U16<'de>> = _serde::__private::None;
                    let mut m_userData: _serde::__private::Option<U64<'de>> = _serde::__private::None;
                    while let _serde::__private::Some(__key) = {
                        __A::next_key::<__Field>(&mut __map)?
                    } {
                        match __key {
                            __Field::m_bodyIdA => {
                                #[cfg(
                                    any(feature = "strict", feature = "ignore_duplicates")
                                )]
                                if _serde::__private::Option::is_some(&m_bodyIdA) {
                                    #[cfg(feature = "ignore_duplicates")]
                                    {
                                        __A::skip_value(&mut __map)?;
                                        continue;
                                    }
                                    #[cfg(feature = "strict")]
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "bodyIdA",
                                        ),
                                    );
                                }
                                m_bodyIdA = _serde::__private::Some(
                                    match __A::next_value::<U32<'de>>(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            __Field::m_bodyIdB => {
                                #[cfg(
                                    any(feature = "strict", feature = "ignore_duplicates")
                                )]
                                if _serde::__private::Option::is_some(&m_bodyIdB) {
                                    #[cfg(feature = "ignore_duplicates")]
                                    {
                                        __A::skip_value(&mut __map)?;
                                        continue;
                                    }
                                    #[cfg(feature = "strict")]
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "bodyIdB",
                                        ),
                                    );
                                }
                                m_bodyIdB = _serde::__private::Some(
                                    match __A::next_value::<U32<'de>>(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            __Field::m_data => {
                                #[cfg(
                                    any(feature = "strict", feature = "ignore_duplicates")
                                )]
                                if _serde::__private::Option::is_some(&m_data) {
                                    #[cfg(feature = "ignore_duplicates")]
                                    {
                                        __A::skip_value(&mut __map)?;
                                        continue;
                                    }
                                    #[cfg(feature = "strict")]
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field("data"),
                                    );
                                }
                                m_data = _serde::__private::Some(
                                    match __A::next_value::<Pointer<'de>>(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            __Field::m_id => {
                                #[cfg(
                                    any(feature = "strict", feature = "ignore_duplicates")
                                )]
                                if _serde::__private::Option::is_some(&m_id) {
                                    #[cfg(feature = "ignore_duplicates")]
                                    {
                                        __A::skip_value(&mut __map)?;
                                        continue;
                                    }
                                    #[cfg(feature = "strict")]
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field("id"),
                                    );
                                }
                                m_id = _serde::__private::Some(
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
                                    match __A::next_value::<FlagsEnum>(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            __Field::m_type => {
                                #[cfg(
                                    any(feature = "strict", feature = "ignore_duplicates")
                                )]
                                if _serde::__private::Option::is_some(&m_type) {
                                    #[cfg(feature = "ignore_duplicates")]
                                    {
                                        __A::skip_value(&mut __map)?;
                                        continue;
                                    }
                                    #[cfg(feature = "strict")]
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field("type"),
                                    );
                                }
                                m_type = _serde::__private::Some(
                                    match __A::next_value::<ConstraintType>(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            __Field::m_sizeOfAtoms => {
                                #[cfg(
                                    any(feature = "strict", feature = "ignore_duplicates")
                                )]
                                if _serde::__private::Option::is_some(&m_sizeOfAtoms) {
                                    #[cfg(feature = "ignore_duplicates")]
                                    {
                                        __A::skip_value(&mut __map)?;
                                        continue;
                                    }
                                    #[cfg(feature = "strict")]
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "sizeOfAtoms",
                                        ),
                                    );
                                }
                                m_sizeOfAtoms = _serde::__private::Some(
                                    match __A::next_value::<U16<'de>>(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            __Field::m_sizeOfSchemas => {
                                #[cfg(
                                    any(feature = "strict", feature = "ignore_duplicates")
                                )]
                                if _serde::__private::Option::is_some(&m_sizeOfSchemas) {
                                    #[cfg(feature = "ignore_duplicates")]
                                    {
                                        __A::skip_value(&mut __map)?;
                                        continue;
                                    }
                                    #[cfg(feature = "strict")]
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "sizeOfSchemas",
                                        ),
                                    );
                                }
                                m_sizeOfSchemas = _serde::__private::Some(
                                    match __A::next_value::<U16<'de>>(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            __Field::m_numSolverResults => {
                                #[cfg(
                                    any(feature = "strict", feature = "ignore_duplicates")
                                )]
                                if _serde::__private::Option::is_some(&m_numSolverResults) {
                                    #[cfg(feature = "ignore_duplicates")]
                                    {
                                        __A::skip_value(&mut __map)?;
                                        continue;
                                    }
                                    #[cfg(feature = "strict")]
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "numSolverResults",
                                        ),
                                    );
                                }
                                m_numSolverResults = _serde::__private::Some(
                                    match __A::next_value::<U8<'de>>(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            __Field::m_numSolverElemTemps => {
                                #[cfg(
                                    any(feature = "strict", feature = "ignore_duplicates")
                                )]
                                if _serde::__private::Option::is_some(
                                    &m_numSolverElemTemps,
                                ) {
                                    #[cfg(feature = "ignore_duplicates")]
                                    {
                                        __A::skip_value(&mut __map)?;
                                        continue;
                                    }
                                    #[cfg(feature = "strict")]
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "numSolverElemTemps",
                                        ),
                                    );
                                }
                                m_numSolverElemTemps = _serde::__private::Some(
                                    match __A::next_value::<U8<'de>>(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            __Field::m_runtimeSize => {
                                #[cfg(
                                    any(feature = "strict", feature = "ignore_duplicates")
                                )]
                                if _serde::__private::Option::is_some(&m_runtimeSize) {
                                    #[cfg(feature = "ignore_duplicates")]
                                    {
                                        __A::skip_value(&mut __map)?;
                                        continue;
                                    }
                                    #[cfg(feature = "strict")]
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "runtimeSize",
                                        ),
                                    );
                                }
                                m_runtimeSize = _serde::__private::Some(
                                    match __A::next_value::<U16<'de>>(&mut __map) {
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
                            _ => __A::skip_value(&mut __map)?,
                        }
                    }
                    let m_bodyIdA = match m_bodyIdA {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            #[cfg(feature = "strict")]
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field("bodyIdA"),
                            );
                            #[cfg(not(feature = "strict"))] Default::default()
                        }
                    };
                    let m_bodyIdB = match m_bodyIdB {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            #[cfg(feature = "strict")]
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field("bodyIdB"),
                            );
                            #[cfg(not(feature = "strict"))] Default::default()
                        }
                    };
                    let m_data = match m_data {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            #[cfg(feature = "strict")]
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field("data"),
                            );
                            #[cfg(not(feature = "strict"))] Default::default()
                        }
                    };
                    let m_id = match m_id {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            #[cfg(feature = "strict")]
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field("id"),
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
                    let m_type = match m_type {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            #[cfg(feature = "strict")]
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field("type"),
                            );
                            #[cfg(not(feature = "strict"))] Default::default()
                        }
                    };
                    let m_sizeOfAtoms = match m_sizeOfAtoms {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            #[cfg(feature = "strict")]
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "sizeOfAtoms",
                                ),
                            );
                            #[cfg(not(feature = "strict"))] Default::default()
                        }
                    };
                    let m_sizeOfSchemas = match m_sizeOfSchemas {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            #[cfg(feature = "strict")]
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "sizeOfSchemas",
                                ),
                            );
                            #[cfg(not(feature = "strict"))] Default::default()
                        }
                    };
                    let m_numSolverResults = match m_numSolverResults {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            #[cfg(feature = "strict")]
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "numSolverResults",
                                ),
                            );
                            #[cfg(not(feature = "strict"))] Default::default()
                        }
                    };
                    let m_numSolverElemTemps = match m_numSolverElemTemps {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            #[cfg(feature = "strict")]
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "numSolverElemTemps",
                                ),
                            );
                            #[cfg(not(feature = "strict"))] Default::default()
                        }
                    };
                    let m_runtimeSize = match m_runtimeSize {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            #[cfg(feature = "strict")]
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "runtimeSize",
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
                    let __ptr = __A::class_ptr(&mut __map);
                    _serde::__private::Ok(hknpConstraint {
                        __ptr: __ptr.clone(),
                        m_bodyIdA,
                        m_bodyIdB,
                        m_data,
                        m_id,
                        m_flags,
                        m_type,
                        m_sizeOfAtoms,
                        m_sizeOfSchemas,
                        m_numSolverResults,
                        m_numSolverElemTemps,
                        m_runtimeSize,
                        m_userData,
                        ..Default::default()
                    })
                }
            }
            const FIELDS: &[&str] = &[
                "bodyIdA",
                "bodyIdB",
                "data",
                "id",
                "immediateId",
                "flags",
                "type",
                "atoms",
                "sizeOfAtoms",
                "sizeOfSchemas",
                "numSolverResults",
                "numSolverElemTemps",
                "runtimeSize",
                "runtime",
                "userData",
            ];
            _serde::Deserializer::deserialize_struct(
                deserializer,
                "hknpConstraint",
                FIELDS,
                __hknpConstraintVisitor {
                    marker: _serde::__private::PhantomData::<hknpConstraint>,
                    lifetime: _serde::__private::PhantomData,
                },
            )
        }
    }
};
#[havok_types_derive::impl_flags_methods]
bitflags::bitflags! {
    #[doc = r" Bit flags"] #[doc = r""] #[doc = r" # C++ Info"] #[doc =
    " - name: `FlagsEnum`(ctype: `hkFlags<FlagsEnum, hkUint8>`)"]
    #[allow(non_upper_case_globals, non_snake_case)] #[cfg_attr(feature = "serde",
    derive(serde_with::SerializeDisplay, serde_with::DeserializeFromStr))]
    #[repr(transparent)] #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)] pub struct
    FlagsEnum : u8 { #[doc = "0"] const NO_FLAGS = 0u8; #[doc = "1"] const IS_EXPORTABLE
    = 1u8; #[doc = "2"] const IS_IMMEDIATE = 2u8; #[doc = "4"] const IS_DISABLED = 4u8;
    #[doc = "8"] const IS_DESTRUCTION_INTERNAL = 8u8; #[doc = "16"] const
    AUTO_REMOVE_ON_DESTRUCTION_RESET = 16u8; #[doc = "32"] const
    AUTO_REMOVE_ON_DESTRUCTION = 32u8; #[doc = "64"] const RAISE_CONSTRAINT_FORCE_EVENTS
    = 64u8; #[doc = "128"] const RAISE_CONSTRAINT_FORCE_EXCEEDED_EVENTS = 128u8; }
}
#[cfg(feature = "json_schema")]
const _: () = {
    use schemars::{SchemaGenerator, Schema, JsonSchema, json_schema};
    use std::borrow::Cow;
    impl JsonSchema for FlagsEnum {
        fn schema_name() -> Cow<'static, str> {
            "FlagsEnum".into()
        }
        fn schema_id() -> Cow<'static, str> {
            concat!(module_path!(), "::", "FlagsEnum").into()
        }
        fn json_schema(_generate: &mut SchemaGenerator) -> Schema {
            json_schema!(
                { "description" :
                "Bitflags field. Specific flags: NO_FLAGS: 0, IS_EXPORTABLE: 1, IS_IMMEDIATE: 2, IS_DISABLED: 4, IS_DESTRUCTION_INTERNAL: 8, AUTO_REMOVE_ON_DESTRUCTION_RESET: 16, AUTO_REMOVE_ON_DESTRUCTION: 32, RAISE_CONSTRAINT_FORCE_EVENTS: 64, RAISE_CONSTRAINT_FORCE_EXCEEDED_EVENTS: 128. Additional unspecified bits may be set.(e.g.: BIT_FLAG|BIT_FLAG2|4)",
                "type" : "string", }
            )
        }
    }
};
const _: () = {
    use havok_serde as __serde;
    impl __serde::Serialize for FlagsEnum {
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
                    Self::NO_FLAGS => {
                        __serializer.serialize_field("NO_FLAGS", &Self::NO_FLAGS)
                    }
                    Self::IS_EXPORTABLE => {
                        __serializer
                            .serialize_field("IS_EXPORTABLE", &Self::IS_EXPORTABLE)
                    }
                    Self::IS_IMMEDIATE => {
                        __serializer.serialize_field("IS_IMMEDIATE", &Self::IS_IMMEDIATE)
                    }
                    Self::IS_DISABLED => {
                        __serializer.serialize_field("IS_DISABLED", &Self::IS_DISABLED)
                    }
                    Self::IS_DESTRUCTION_INTERNAL => {
                        __serializer
                            .serialize_field(
                                "IS_DESTRUCTION_INTERNAL",
                                &Self::IS_DESTRUCTION_INTERNAL,
                            )
                    }
                    Self::AUTO_REMOVE_ON_DESTRUCTION_RESET => {
                        __serializer
                            .serialize_field(
                                "AUTO_REMOVE_ON_DESTRUCTION_RESET",
                                &Self::AUTO_REMOVE_ON_DESTRUCTION_RESET,
                            )
                    }
                    Self::AUTO_REMOVE_ON_DESTRUCTION => {
                        __serializer
                            .serialize_field(
                                "AUTO_REMOVE_ON_DESTRUCTION",
                                &Self::AUTO_REMOVE_ON_DESTRUCTION,
                            )
                    }
                    Self::RAISE_CONSTRAINT_FORCE_EVENTS => {
                        __serializer
                            .serialize_field(
                                "RAISE_CONSTRAINT_FORCE_EVENTS",
                                &Self::RAISE_CONSTRAINT_FORCE_EVENTS,
                            )
                    }
                    Self::RAISE_CONSTRAINT_FORCE_EXCEEDED_EVENTS => {
                        __serializer
                            .serialize_field(
                                "RAISE_CONSTRAINT_FORCE_EXCEEDED_EVENTS",
                                &Self::RAISE_CONSTRAINT_FORCE_EXCEEDED_EVENTS,
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
    impl<'de> _serde::Deserialize<'de> for FlagsEnum {
        fn deserialize<__D>(
            __deserializer: __D,
        ) -> _serde::__private::Result<Self, __D::Error>
        where
            __D: _serde::Deserializer<'de>,
        {
            #[doc(hidden)]
            struct __Visitor<'de> {
                marker: _serde::__private::PhantomData<FlagsEnum>,
                lifetime: _serde::__private::PhantomData<&'de ()>,
            }
            impl<'de> _serde::de::Visitor<'de> for __Visitor<'de> {
                type Value = FlagsEnum;
                fn expecting(
                    &self,
                    __formatter: &mut _serde::__private::Formatter,
                ) -> _serde::__private::fmt::Result {
                    _serde::__private::Formatter::write_str(
                        __formatter,
                        "struct FlagsEnum(flags)",
                    )
                }
                #[inline]
                fn visit_uint8<__E>(
                    self,
                    __value: U8<'de>,
                ) -> _serde::__private::Result<Self::Value, __E>
                where
                    __E: _serde::de::Error,
                {
                    match __value {
                        U8::Number(__value) => {
                            Ok(FlagsEnum::from_bits_retain(__value as _))
                        }
                        _ => {
                            Err(
                                _serde::de::Error::invalid_value(
                                    _serde::de::Unexpected::Uint8(__value as _),
                                    &"FlagsEnum(U8) Number",
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
                    match <FlagsEnum as core::str::FromStr>::from_str(
                        __value.into_inner().unwrap().as_ref(),
                    ) {
                        Ok(flags) => Ok(flags),
                        Err(err) => Err(_serde::de::Error::custom(err)),
                    }
                }
            }
            _serde::Deserializer::deserialize_flags(
                __deserializer,
                _serde::de::ReadEnumSize::Uint8,
                __Visitor {
                    marker: _serde::__private::PhantomData::<FlagsEnum>,
                    lifetime: _serde::__private::PhantomData,
                },
            )
        }
    }
};
