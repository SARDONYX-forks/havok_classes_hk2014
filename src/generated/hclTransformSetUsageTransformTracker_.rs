use super::class_requires::*;
use super::*;
/// # C++ Info
/// - name: `hclTransformSetUsageTransformTracker`
/// - version: `0`
/// - signature: `0xd9c5ea8b`
/// - size: `  0`(x86)/` 72`(x86_64)
/// -  vtable: `false`
#[allow(non_upper_case_globals, non_snake_case)]
#[cfg_attr(feature = "json_schema", derive(schemars::JsonSchema))]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(educe::Educe)]
#[educe(Debug, Clone, Default, PartialEq)]
pub struct hclTransformSetUsageTransformTracker<'a> {
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
    /// - name: `read`(ctype: `struct hkBitField`)
    /// - offset: `  0`(x86)/`  0`(x86_64)
    /// - type_size: `  0`(x86)/` 24`(x86_64)
    #[cfg_attr(feature = "serde_default", serde(default))]
    #[cfg_attr(feature = "serde", serde(borrow))]
    #[cfg_attr(feature = "json_schema", schemars(rename = "read"))]
    #[cfg_attr(feature = "serde", serde(rename = "read"))]
    pub m_read: hkBitField<'a>,
    /// # C++ Info
    /// - name: `readBeforeWrite`(ctype: `struct hkBitField`)
    /// - offset: `  0`(x86)/` 24`(x86_64)
    /// - type_size: `  0`(x86)/` 24`(x86_64)
    #[cfg_attr(feature = "serde_default", serde(default))]
    #[cfg_attr(feature = "serde", serde(borrow))]
    #[cfg_attr(feature = "json_schema", schemars(rename = "readBeforeWrite"))]
    #[cfg_attr(feature = "serde", serde(rename = "readBeforeWrite"))]
    pub m_readBeforeWrite: hkBitField<'a>,
    /// # C++ Info
    /// - name: `written`(ctype: `struct hkBitField`)
    /// - offset: `  0`(x86)/` 48`(x86_64)
    /// - type_size: `  0`(x86)/` 24`(x86_64)
    #[cfg_attr(feature = "serde_default", serde(default))]
    #[cfg_attr(feature = "serde", serde(borrow))]
    #[cfg_attr(feature = "json_schema", schemars(rename = "written"))]
    #[cfg_attr(feature = "serde", serde(rename = "written"))]
    pub m_written: hkBitField<'a>,
}
const _: () = {
    use havok_serde as _serde;
    impl<'a> _serde::HavokClass for hclTransformSetUsageTransformTracker<'a> {
        #[inline]
        fn name(&self) -> &'static str {
            "hclTransformSetUsageTransformTracker"
        }
        #[inline]
        fn signature(&self) -> _serde::__private::Signature {
            _serde::__private::Signature::new(0xd9c5ea8b)
        }
        #[allow(clippy::let_and_return, clippy::vec_init_then_push)]
        fn deps_indexes(&self) -> Vec<&Pointer<'_>> {
            let mut v = Vec::new();
            v.extend(self.m_read.deps_indexes());
            v.extend(self.m_readBeforeWrite.deps_indexes());
            v.extend(self.m_written.deps_indexes());
            v
        }
    }
    impl<'a> _serde::Serialize for hclTransformSetUsageTransformTracker<'a> {
        fn serialize<S>(&self, __serializer: S) -> Result<S::Ok, S::Error>
        where
            S: _serde::ser::Serializer,
        {
            let class_meta = self
                .__ptr
                .as_ref()
                .map(|name| (name, _serde::__private::Signature::new(0xd9c5ea8b)));
            let mut serializer = __serializer
                .serialize_struct(
                    "hclTransformSetUsageTransformTracker",
                    class_meta,
                    (0u64, 72u64),
                )?;
            serializer.serialize_field("read", &self.m_read)?;
            serializer.serialize_field("readBeforeWrite", &self.m_readBeforeWrite)?;
            serializer.serialize_field("written", &self.m_written)?;
            serializer.end()
        }
    }
};
#[doc(hidden)]
#[allow(non_upper_case_globals, unused_attributes, unused_qualifications)]
const _: () = {
    use havok_serde as _serde;
    #[automatically_derived]
    impl<'de> _serde::Deserialize<'de> for hclTransformSetUsageTransformTracker<'de> {
        fn deserialize<__D>(deserializer: __D) -> core::result::Result<Self, __D::Error>
        where
            __D: _serde::Deserializer<'de>,
        {
            #[allow(non_camel_case_types)]
            enum __Field {
                m_read,
                m_readBeforeWrite,
                m_written,
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
                        "read" => Ok(__Field::m_read),
                        "readBeforeWrite" => Ok(__Field::m_readBeforeWrite),
                        "written" => Ok(__Field::m_written),
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
            struct __hclTransformSetUsageTransformTrackerVisitor<'de> {
                marker: _serde::__private::PhantomData<
                    hclTransformSetUsageTransformTracker<'de>,
                >,
                lifetime: _serde::__private::PhantomData<&'de ()>,
            }
            #[allow(clippy::match_single_binding)]
            #[allow(clippy::reversed_empty_ranges)]
            #[allow(clippy::single_match)]
            impl<'de> _serde::de::Visitor<'de>
            for __hclTransformSetUsageTransformTrackerVisitor<'de> {
                type Value = hclTransformSetUsageTransformTracker<'de>;
                fn expecting(
                    &self,
                    __formatter: &mut core::fmt::Formatter,
                ) -> core::fmt::Result {
                    core::fmt::Formatter::write_str(
                        __formatter,
                        "struct hclTransformSetUsageTransformTracker",
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
                    let mut m_read: _serde::__private::Option<hkBitField<'de>> = _serde::__private::None;
                    let mut m_readBeforeWrite: _serde::__private::Option<
                        hkBitField<'de>,
                    > = _serde::__private::None;
                    let mut m_written: _serde::__private::Option<hkBitField<'de>> = _serde::__private::None;
                    for i in 0..3usize {
                        match i {
                            0usize => {
                                if _serde::__private::Option::is_some(&m_read) {
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field("read"),
                                    );
                                }
                                m_read = _serde::__private::Some(
                                    match __A::next_value::<hkBitField<'de>>(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            1usize => {
                                if _serde::__private::Option::is_some(&m_readBeforeWrite) {
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "readBeforeWrite",
                                        ),
                                    );
                                }
                                m_readBeforeWrite = _serde::__private::Some(
                                    match __A::next_value::<hkBitField<'de>>(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            2usize => {
                                if _serde::__private::Option::is_some(&m_written) {
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "written",
                                        ),
                                    );
                                }
                                m_written = _serde::__private::Some(
                                    match __A::next_value::<hkBitField<'de>>(&mut __map) {
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
                    let m_read = match m_read {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field("read"),
                            );
                        }
                    };
                    let m_readBeforeWrite = match m_readBeforeWrite {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "readBeforeWrite",
                                ),
                            );
                        }
                    };
                    let m_written = match m_written {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field("written"),
                            );
                        }
                    };
                    _serde::__private::Ok(hclTransformSetUsageTransformTracker {
                        __ptr,
                        m_read,
                        m_readBeforeWrite,
                        m_written,
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
                    let mut m_read: _serde::__private::Option<hkBitField<'de>> = _serde::__private::None;
                    let mut m_readBeforeWrite: _serde::__private::Option<
                        hkBitField<'de>,
                    > = _serde::__private::None;
                    let mut m_written: _serde::__private::Option<hkBitField<'de>> = _serde::__private::None;
                    while let _serde::__private::Some(__key) = {
                        __A::next_key::<__Field>(&mut __map)?
                    } {
                        match __key {
                            __Field::m_read => {
                                #[cfg(
                                    any(feature = "strict", feature = "ignore_duplicates")
                                )]
                                if _serde::__private::Option::is_some(&m_read) {
                                    #[cfg(feature = "ignore_duplicates")]
                                    {
                                        __A::skip_value(&mut __map)?;
                                        continue;
                                    }
                                    #[cfg(feature = "strict")]
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field("read"),
                                    );
                                }
                                m_read = _serde::__private::Some(
                                    match __A::next_value::<hkBitField<'de>>(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            __Field::m_readBeforeWrite => {
                                #[cfg(
                                    any(feature = "strict", feature = "ignore_duplicates")
                                )]
                                if _serde::__private::Option::is_some(&m_readBeforeWrite) {
                                    #[cfg(feature = "ignore_duplicates")]
                                    {
                                        __A::skip_value(&mut __map)?;
                                        continue;
                                    }
                                    #[cfg(feature = "strict")]
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "readBeforeWrite",
                                        ),
                                    );
                                }
                                m_readBeforeWrite = _serde::__private::Some(
                                    match __A::next_value::<hkBitField<'de>>(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            __Field::m_written => {
                                #[cfg(
                                    any(feature = "strict", feature = "ignore_duplicates")
                                )]
                                if _serde::__private::Option::is_some(&m_written) {
                                    #[cfg(feature = "ignore_duplicates")]
                                    {
                                        __A::skip_value(&mut __map)?;
                                        continue;
                                    }
                                    #[cfg(feature = "strict")]
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "written",
                                        ),
                                    );
                                }
                                m_written = _serde::__private::Some(
                                    match __A::next_value::<hkBitField<'de>>(&mut __map) {
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
                    let m_read = match m_read {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            #[cfg(feature = "strict")]
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field("read"),
                            );
                            #[cfg(not(feature = "strict"))] Default::default()
                        }
                    };
                    let m_readBeforeWrite = match m_readBeforeWrite {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            #[cfg(feature = "strict")]
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "readBeforeWrite",
                                ),
                            );
                            #[cfg(not(feature = "strict"))] Default::default()
                        }
                    };
                    let m_written = match m_written {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            #[cfg(feature = "strict")]
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field("written"),
                            );
                            #[cfg(not(feature = "strict"))] Default::default()
                        }
                    };
                    let __ptr = __A::class_ptr(&mut __map);
                    _serde::__private::Ok(hclTransformSetUsageTransformTracker {
                        __ptr: __ptr.clone(),
                        m_read,
                        m_readBeforeWrite,
                        m_written,
                    })
                }
            }
            const FIELDS: &[&str] = &["read", "readBeforeWrite", "written"];
            _serde::Deserializer::deserialize_struct(
                deserializer,
                "hclTransformSetUsageTransformTracker",
                FIELDS,
                __hclTransformSetUsageTransformTrackerVisitor {
                    marker: _serde::__private::PhantomData::<
                        hclTransformSetUsageTransformTracker,
                    >,
                    lifetime: _serde::__private::PhantomData,
                },
            )
        }
    }
};
