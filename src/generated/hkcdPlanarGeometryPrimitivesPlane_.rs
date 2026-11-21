use super::class_requires::*;
use super::*;
/// # C++ Info
/// - name: `hkcdPlanarGeometryPrimitivesPlane`
/// - version: `0`
/// - signature: `0xf9fd78c4`
/// - size: `  0`(x86)/` 64`(x86_64)
/// -  vtable: `false`
#[allow(non_upper_case_globals, non_snake_case)]
#[cfg_attr(feature = "json_schema", derive(schemars::JsonSchema))]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(educe::Educe)]
#[educe(Debug, Clone, Default, PartialEq)]
pub struct hkcdPlanarGeometryPrimitivesPlane<'a> {
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
    /// - name: `iEqn`(ctype: `hkInt64[4]`)
    /// - offset: `  0`(x86)/`  0`(x86_64)
    /// - type_size: `  0`(x86)/` 32`(x86_64)
    #[cfg_attr(feature = "serde_default", serde(default))]
    #[cfg_attr(feature = "serde", serde(borrow))]
    #[cfg_attr(feature = "json_schema", schemars(rename = "iEqn"))]
    #[cfg_attr(feature = "serde", serde(rename = "iEqn"))]
    pub m_iEqn: [I64<'a>; 4usize],
    /// # C++ Info
    /// - name: `dEqn`(ctype: `hkInt64[4]`)
    /// - offset: `  0`(x86)/` 32`(x86_64)
    /// - type_size: `  0`(x86)/` 32`(x86_64)
    /// - flags: `SERIALIZE_IGNORED`
    #[cfg_attr(feature = "serde", serde(default))]
    #[cfg_attr(feature = "serde", serde(borrow))]
    #[cfg_attr(feature = "json_schema", schemars(rename = "dEqn"))]
    #[cfg_attr(feature = "serde", serde(rename = "dEqn"))]
    pub m_dEqn: [I64<'a>; 4usize],
}
const _: () = {
    use havok_serde as _serde;
    impl<'a> _serde::HavokClass for hkcdPlanarGeometryPrimitivesPlane<'a> {
        #[inline]
        fn name(&self) -> &'static str {
            "hkcdPlanarGeometryPrimitivesPlane"
        }
        #[inline]
        fn signature(&self) -> _serde::__private::Signature {
            _serde::__private::Signature::new(0xf9fd78c4)
        }
        #[allow(clippy::let_and_return, clippy::vec_init_then_push)]
        fn deps_indexes(&self) -> Vec<&Pointer<'_>> {
            let mut v = Vec::new();
            v
        }
    }
    impl<'a> _serde::Serialize for hkcdPlanarGeometryPrimitivesPlane<'a> {
        fn serialize<S>(&self, __serializer: S) -> Result<S::Ok, S::Error>
        where
            S: _serde::ser::Serializer,
        {
            let class_meta = self
                .__ptr
                .as_ref()
                .map(|name| (name, _serde::__private::Signature::new(0xf9fd78c4)));
            let mut serializer = __serializer
                .serialize_struct(
                    "hkcdPlanarGeometryPrimitivesPlane",
                    class_meta,
                    (0u64, 64u64),
                )?;
            serializer
                .serialize_fixed_array_field(
                    "iEqn",
                    self.m_iEqn.as_slice(),
                    TypeSize::NonPtr,
                )?;
            serializer
                .skip_fixed_array_field(
                    "dEqn",
                    self.m_dEqn.as_slice(),
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
    impl<'de> _serde::Deserialize<'de> for hkcdPlanarGeometryPrimitivesPlane<'de> {
        fn deserialize<__D>(deserializer: __D) -> core::result::Result<Self, __D::Error>
        where
            __D: _serde::Deserializer<'de>,
        {
            #[allow(non_camel_case_types)]
            enum __Field {
                m_iEqn,
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
                        "iEqn" => Ok(__Field::m_iEqn),
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
            struct __hkcdPlanarGeometryPrimitivesPlaneVisitor<'de> {
                marker: _serde::__private::PhantomData<
                    hkcdPlanarGeometryPrimitivesPlane<'de>,
                >,
                lifetime: _serde::__private::PhantomData<&'de ()>,
            }
            #[allow(clippy::match_single_binding)]
            #[allow(clippy::reversed_empty_ranges)]
            #[allow(clippy::single_match)]
            impl<'de> _serde::de::Visitor<'de>
            for __hkcdPlanarGeometryPrimitivesPlaneVisitor<'de> {
                type Value = hkcdPlanarGeometryPrimitivesPlane<'de>;
                fn expecting(
                    &self,
                    __formatter: &mut core::fmt::Formatter,
                ) -> core::fmt::Result {
                    core::fmt::Formatter::write_str(
                        __formatter,
                        "struct hkcdPlanarGeometryPrimitivesPlane",
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
                    let mut m_iEqn: _serde::__private::Option<[I64<'de>; 4usize]> = _serde::__private::None;
                    let mut m_dEqn: _serde::__private::Option<[I64<'de>; 4usize]> = _serde::__private::None;
                    for i in 0..2usize {
                        match i {
                            0usize => {
                                if _serde::__private::Option::is_some(&m_iEqn) {
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field("iEqn"),
                                    );
                                }
                                m_iEqn = _serde::__private::Some(
                                    match __A::next_value::<[I64<'de>; 4usize]>(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            1usize => {
                                if _serde::__private::Option::is_some(&m_dEqn) {
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field("dEqn"),
                                    );
                                }
                                m_dEqn = _serde::__private::Some(
                                    match __A::next_value::<[I64<'de>; 4usize]>(&mut __map) {
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
                    let m_iEqn = match m_iEqn {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field("iEqn"),
                            );
                        }
                    };
                    let m_dEqn = match m_dEqn {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field("dEqn"),
                            );
                        }
                    };
                    _serde::__private::Ok(hkcdPlanarGeometryPrimitivesPlane {
                        __ptr,
                        m_iEqn,
                        m_dEqn,
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
                    let mut m_iEqn: _serde::__private::Option<[I64<'de>; 4usize]> = _serde::__private::None;
                    while let _serde::__private::Some(__key) = {
                        __A::next_key::<__Field>(&mut __map)?
                    } {
                        match __key {
                            __Field::m_iEqn => {
                                #[cfg(
                                    any(feature = "strict", feature = "ignore_duplicates")
                                )]
                                if _serde::__private::Option::is_some(&m_iEqn) {
                                    #[cfg(feature = "ignore_duplicates")]
                                    {
                                        __A::skip_value(&mut __map)?;
                                        continue;
                                    }
                                    #[cfg(feature = "strict")]
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field("iEqn"),
                                    );
                                }
                                m_iEqn = _serde::__private::Some(
                                    match __A::next_value::<[I64<'de>; 4usize]>(&mut __map) {
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
                    let m_iEqn = match m_iEqn {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            #[cfg(feature = "strict")]
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field("iEqn"),
                            );
                            #[cfg(not(feature = "strict"))] Default::default()
                        }
                    };
                    let __ptr = __A::class_ptr(&mut __map);
                    _serde::__private::Ok(hkcdPlanarGeometryPrimitivesPlane {
                        __ptr: __ptr.clone(),
                        m_iEqn,
                        ..Default::default()
                    })
                }
            }
            const FIELDS: &[&str] = &["iEqn", "dEqn"];
            _serde::Deserializer::deserialize_struct(
                deserializer,
                "hkcdPlanarGeometryPrimitivesPlane",
                FIELDS,
                __hkcdPlanarGeometryPrimitivesPlaneVisitor {
                    marker: _serde::__private::PhantomData::<
                        hkcdPlanarGeometryPrimitivesPlane,
                    >,
                    lifetime: _serde::__private::PhantomData,
                },
            )
        }
    }
};
