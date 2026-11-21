use super::class_requires::*;
use super::*;
/// # C++ Info
/// - name: `BSAssignVariablesModifier`
/// - version: `1`
/// - signature: `0x64a6ca08`
/// - size: `  0`(x86)/`280`(x86_64)
/// -  vtable: `true`
#[allow(non_upper_case_globals, non_snake_case)]
#[cfg_attr(feature = "json_schema", derive(schemars::JsonSchema))]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(educe::Educe)]
#[educe(Debug, Clone, Default, PartialEq)]
pub struct BSAssignVariablesModifier<'a> {
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
    /// - name: `floatVariable1`(ctype: `hkReal`)
    /// - offset: `  0`(x86)/` 88`(x86_64)
    /// - type_size: `  0`(x86)/`  4`(x86_64)
    #[cfg_attr(feature = "serde_default", serde(default))]
    #[cfg_attr(feature = "json_schema", schemars(rename = "floatVariable1"))]
    #[cfg_attr(feature = "serde", serde(rename = "floatVariable1"))]
    pub m_floatVariable1: f32,
    /// # C++ Info
    /// - name: `floatValue1`(ctype: `hkReal`)
    /// - offset: `  0`(x86)/` 92`(x86_64)
    /// - type_size: `  0`(x86)/`  4`(x86_64)
    #[cfg_attr(feature = "serde_default", serde(default))]
    #[cfg_attr(feature = "json_schema", schemars(rename = "floatValue1"))]
    #[cfg_attr(feature = "serde", serde(rename = "floatValue1"))]
    pub m_floatValue1: f32,
    /// # C++ Info
    /// - name: `floatVariable2`(ctype: `hkReal`)
    /// - offset: `  0`(x86)/` 96`(x86_64)
    /// - type_size: `  0`(x86)/`  4`(x86_64)
    #[cfg_attr(feature = "serde_default", serde(default))]
    #[cfg_attr(feature = "json_schema", schemars(rename = "floatVariable2"))]
    #[cfg_attr(feature = "serde", serde(rename = "floatVariable2"))]
    pub m_floatVariable2: f32,
    /// # C++ Info
    /// - name: `floatValue2`(ctype: `hkReal`)
    /// - offset: `  0`(x86)/`100`(x86_64)
    /// - type_size: `  0`(x86)/`  4`(x86_64)
    #[cfg_attr(feature = "serde_default", serde(default))]
    #[cfg_attr(feature = "json_schema", schemars(rename = "floatValue2"))]
    #[cfg_attr(feature = "serde", serde(rename = "floatValue2"))]
    pub m_floatValue2: f32,
    /// # C++ Info
    /// - name: `floatVariable3`(ctype: `hkReal`)
    /// - offset: `  0`(x86)/`104`(x86_64)
    /// - type_size: `  0`(x86)/`  4`(x86_64)
    #[cfg_attr(feature = "serde_default", serde(default))]
    #[cfg_attr(feature = "json_schema", schemars(rename = "floatVariable3"))]
    #[cfg_attr(feature = "serde", serde(rename = "floatVariable3"))]
    pub m_floatVariable3: f32,
    /// # C++ Info
    /// - name: `floatValue3`(ctype: `hkReal`)
    /// - offset: `  0`(x86)/`108`(x86_64)
    /// - type_size: `  0`(x86)/`  4`(x86_64)
    #[cfg_attr(feature = "serde_default", serde(default))]
    #[cfg_attr(feature = "json_schema", schemars(rename = "floatValue3"))]
    #[cfg_attr(feature = "serde", serde(rename = "floatValue3"))]
    pub m_floatValue3: f32,
    /// # C++ Info
    /// - name: `floatVariable4`(ctype: `hkReal`)
    /// - offset: `  0`(x86)/`112`(x86_64)
    /// - type_size: `  0`(x86)/`  4`(x86_64)
    #[cfg_attr(feature = "serde_default", serde(default))]
    #[cfg_attr(feature = "json_schema", schemars(rename = "floatVariable4"))]
    #[cfg_attr(feature = "serde", serde(rename = "floatVariable4"))]
    pub m_floatVariable4: f32,
    /// # C++ Info
    /// - name: `floatValue4`(ctype: `hkReal`)
    /// - offset: `  0`(x86)/`116`(x86_64)
    /// - type_size: `  0`(x86)/`  4`(x86_64)
    #[cfg_attr(feature = "serde_default", serde(default))]
    #[cfg_attr(feature = "json_schema", schemars(rename = "floatValue4"))]
    #[cfg_attr(feature = "serde", serde(rename = "floatValue4"))]
    pub m_floatValue4: f32,
    /// # C++ Info
    /// - name: `floatVariable5`(ctype: `hkReal`)
    /// - offset: `  0`(x86)/`120`(x86_64)
    /// - type_size: `  0`(x86)/`  4`(x86_64)
    #[cfg_attr(feature = "serde_default", serde(default))]
    #[cfg_attr(feature = "json_schema", schemars(rename = "floatVariable5"))]
    #[cfg_attr(feature = "serde", serde(rename = "floatVariable5"))]
    pub m_floatVariable5: f32,
    /// # C++ Info
    /// - name: `floatValue5`(ctype: `hkReal`)
    /// - offset: `  0`(x86)/`124`(x86_64)
    /// - type_size: `  0`(x86)/`  4`(x86_64)
    #[cfg_attr(feature = "serde_default", serde(default))]
    #[cfg_attr(feature = "json_schema", schemars(rename = "floatValue5"))]
    #[cfg_attr(feature = "serde", serde(rename = "floatValue5"))]
    pub m_floatValue5: f32,
    /// # C++ Info
    /// - name: `floatVariable6`(ctype: `hkReal`)
    /// - offset: `  0`(x86)/`128`(x86_64)
    /// - type_size: `  0`(x86)/`  4`(x86_64)
    #[cfg_attr(feature = "serde_default", serde(default))]
    #[cfg_attr(feature = "json_schema", schemars(rename = "floatVariable6"))]
    #[cfg_attr(feature = "serde", serde(rename = "floatVariable6"))]
    pub m_floatVariable6: f32,
    /// # C++ Info
    /// - name: `floatValue6`(ctype: `hkReal`)
    /// - offset: `  0`(x86)/`132`(x86_64)
    /// - type_size: `  0`(x86)/`  4`(x86_64)
    #[cfg_attr(feature = "serde_default", serde(default))]
    #[cfg_attr(feature = "json_schema", schemars(rename = "floatValue6"))]
    #[cfg_attr(feature = "serde", serde(rename = "floatValue6"))]
    pub m_floatValue6: f32,
    /// # C++ Info
    /// - name: `floatVariable7`(ctype: `hkReal`)
    /// - offset: `  0`(x86)/`136`(x86_64)
    /// - type_size: `  0`(x86)/`  4`(x86_64)
    #[cfg_attr(feature = "serde_default", serde(default))]
    #[cfg_attr(feature = "json_schema", schemars(rename = "floatVariable7"))]
    #[cfg_attr(feature = "serde", serde(rename = "floatVariable7"))]
    pub m_floatVariable7: f32,
    /// # C++ Info
    /// - name: `floatValue7`(ctype: `hkReal`)
    /// - offset: `  0`(x86)/`140`(x86_64)
    /// - type_size: `  0`(x86)/`  4`(x86_64)
    #[cfg_attr(feature = "serde_default", serde(default))]
    #[cfg_attr(feature = "json_schema", schemars(rename = "floatValue7"))]
    #[cfg_attr(feature = "serde", serde(rename = "floatValue7"))]
    pub m_floatValue7: f32,
    /// # C++ Info
    /// - name: `floatVariable8`(ctype: `hkReal`)
    /// - offset: `  0`(x86)/`144`(x86_64)
    /// - type_size: `  0`(x86)/`  4`(x86_64)
    #[cfg_attr(feature = "serde_default", serde(default))]
    #[cfg_attr(feature = "json_schema", schemars(rename = "floatVariable8"))]
    #[cfg_attr(feature = "serde", serde(rename = "floatVariable8"))]
    pub m_floatVariable8: f32,
    /// # C++ Info
    /// - name: `floatValue8`(ctype: `hkReal`)
    /// - offset: `  0`(x86)/`148`(x86_64)
    /// - type_size: `  0`(x86)/`  4`(x86_64)
    #[cfg_attr(feature = "serde_default", serde(default))]
    #[cfg_attr(feature = "json_schema", schemars(rename = "floatValue8"))]
    #[cfg_attr(feature = "serde", serde(rename = "floatValue8"))]
    pub m_floatValue8: f32,
    /// # C++ Info
    /// - name: `floatVariable9`(ctype: `hkReal`)
    /// - offset: `  0`(x86)/`152`(x86_64)
    /// - type_size: `  0`(x86)/`  4`(x86_64)
    #[cfg_attr(feature = "serde_default", serde(default))]
    #[cfg_attr(feature = "json_schema", schemars(rename = "floatVariable9"))]
    #[cfg_attr(feature = "serde", serde(rename = "floatVariable9"))]
    pub m_floatVariable9: f32,
    /// # C++ Info
    /// - name: `floatValue9`(ctype: `hkReal`)
    /// - offset: `  0`(x86)/`156`(x86_64)
    /// - type_size: `  0`(x86)/`  4`(x86_64)
    #[cfg_attr(feature = "serde_default", serde(default))]
    #[cfg_attr(feature = "json_schema", schemars(rename = "floatValue9"))]
    #[cfg_attr(feature = "serde", serde(rename = "floatValue9"))]
    pub m_floatValue9: f32,
    /// # C++ Info
    /// - name: `floatVariable10`(ctype: `hkReal`)
    /// - offset: `  0`(x86)/`160`(x86_64)
    /// - type_size: `  0`(x86)/`  4`(x86_64)
    #[cfg_attr(feature = "serde_default", serde(default))]
    #[cfg_attr(feature = "json_schema", schemars(rename = "floatVariable10"))]
    #[cfg_attr(feature = "serde", serde(rename = "floatVariable10"))]
    pub m_floatVariable10: f32,
    /// # C++ Info
    /// - name: `floatValue10`(ctype: `hkReal`)
    /// - offset: `  0`(x86)/`164`(x86_64)
    /// - type_size: `  0`(x86)/`  4`(x86_64)
    #[cfg_attr(feature = "serde_default", serde(default))]
    #[cfg_attr(feature = "json_schema", schemars(rename = "floatValue10"))]
    #[cfg_attr(feature = "serde", serde(rename = "floatValue10"))]
    pub m_floatValue10: f32,
    /// # C++ Info
    /// - name: `floatVariable11`(ctype: `hkReal`)
    /// - offset: `  0`(x86)/`168`(x86_64)
    /// - type_size: `  0`(x86)/`  4`(x86_64)
    #[cfg_attr(feature = "serde_default", serde(default))]
    #[cfg_attr(feature = "json_schema", schemars(rename = "floatVariable11"))]
    #[cfg_attr(feature = "serde", serde(rename = "floatVariable11"))]
    pub m_floatVariable11: f32,
    /// # C++ Info
    /// - name: `floatValue11`(ctype: `hkReal`)
    /// - offset: `  0`(x86)/`172`(x86_64)
    /// - type_size: `  0`(x86)/`  4`(x86_64)
    #[cfg_attr(feature = "serde_default", serde(default))]
    #[cfg_attr(feature = "json_schema", schemars(rename = "floatValue11"))]
    #[cfg_attr(feature = "serde", serde(rename = "floatValue11"))]
    pub m_floatValue11: f32,
    /// # C++ Info
    /// - name: `floatVariable12`(ctype: `hkReal`)
    /// - offset: `  0`(x86)/`176`(x86_64)
    /// - type_size: `  0`(x86)/`  4`(x86_64)
    #[cfg_attr(feature = "serde_default", serde(default))]
    #[cfg_attr(feature = "json_schema", schemars(rename = "floatVariable12"))]
    #[cfg_attr(feature = "serde", serde(rename = "floatVariable12"))]
    pub m_floatVariable12: f32,
    /// # C++ Info
    /// - name: `floatValue12`(ctype: `hkReal`)
    /// - offset: `  0`(x86)/`180`(x86_64)
    /// - type_size: `  0`(x86)/`  4`(x86_64)
    #[cfg_attr(feature = "serde_default", serde(default))]
    #[cfg_attr(feature = "json_schema", schemars(rename = "floatValue12"))]
    #[cfg_attr(feature = "serde", serde(rename = "floatValue12"))]
    pub m_floatValue12: f32,
    /// # C++ Info
    /// - name: `floatVariable13`(ctype: `hkReal`)
    /// - offset: `  0`(x86)/`184`(x86_64)
    /// - type_size: `  0`(x86)/`  4`(x86_64)
    #[cfg_attr(feature = "serde_default", serde(default))]
    #[cfg_attr(feature = "json_schema", schemars(rename = "floatVariable13"))]
    #[cfg_attr(feature = "serde", serde(rename = "floatVariable13"))]
    pub m_floatVariable13: f32,
    /// # C++ Info
    /// - name: `floatValue13`(ctype: `hkReal`)
    /// - offset: `  0`(x86)/`188`(x86_64)
    /// - type_size: `  0`(x86)/`  4`(x86_64)
    #[cfg_attr(feature = "serde_default", serde(default))]
    #[cfg_attr(feature = "json_schema", schemars(rename = "floatValue13"))]
    #[cfg_attr(feature = "serde", serde(rename = "floatValue13"))]
    pub m_floatValue13: f32,
    /// # C++ Info
    /// - name: `floatVariable14`(ctype: `hkReal`)
    /// - offset: `  0`(x86)/`192`(x86_64)
    /// - type_size: `  0`(x86)/`  4`(x86_64)
    #[cfg_attr(feature = "serde_default", serde(default))]
    #[cfg_attr(feature = "json_schema", schemars(rename = "floatVariable14"))]
    #[cfg_attr(feature = "serde", serde(rename = "floatVariable14"))]
    pub m_floatVariable14: f32,
    /// # C++ Info
    /// - name: `floatValue14`(ctype: `hkReal`)
    /// - offset: `  0`(x86)/`196`(x86_64)
    /// - type_size: `  0`(x86)/`  4`(x86_64)
    #[cfg_attr(feature = "serde_default", serde(default))]
    #[cfg_attr(feature = "json_schema", schemars(rename = "floatValue14"))]
    #[cfg_attr(feature = "serde", serde(rename = "floatValue14"))]
    pub m_floatValue14: f32,
    /// # C++ Info
    /// - name: `floatVariable15`(ctype: `hkReal`)
    /// - offset: `  0`(x86)/`200`(x86_64)
    /// - type_size: `  0`(x86)/`  4`(x86_64)
    #[cfg_attr(feature = "serde_default", serde(default))]
    #[cfg_attr(feature = "json_schema", schemars(rename = "floatVariable15"))]
    #[cfg_attr(feature = "serde", serde(rename = "floatVariable15"))]
    pub m_floatVariable15: f32,
    /// # C++ Info
    /// - name: `floatValue15`(ctype: `hkReal`)
    /// - offset: `  0`(x86)/`204`(x86_64)
    /// - type_size: `  0`(x86)/`  4`(x86_64)
    #[cfg_attr(feature = "serde_default", serde(default))]
    #[cfg_attr(feature = "json_schema", schemars(rename = "floatValue15"))]
    #[cfg_attr(feature = "serde", serde(rename = "floatValue15"))]
    pub m_floatValue15: f32,
    /// # C++ Info
    /// - name: `floatVariable16`(ctype: `hkReal`)
    /// - offset: `  0`(x86)/`208`(x86_64)
    /// - type_size: `  0`(x86)/`  4`(x86_64)
    #[cfg_attr(feature = "serde_default", serde(default))]
    #[cfg_attr(feature = "json_schema", schemars(rename = "floatVariable16"))]
    #[cfg_attr(feature = "serde", serde(rename = "floatVariable16"))]
    pub m_floatVariable16: f32,
    /// # C++ Info
    /// - name: `floatValue16`(ctype: `hkReal`)
    /// - offset: `  0`(x86)/`212`(x86_64)
    /// - type_size: `  0`(x86)/`  4`(x86_64)
    #[cfg_attr(feature = "serde_default", serde(default))]
    #[cfg_attr(feature = "json_schema", schemars(rename = "floatValue16"))]
    #[cfg_attr(feature = "serde", serde(rename = "floatValue16"))]
    pub m_floatValue16: f32,
    /// # C++ Info
    /// - name: `floatVariable17`(ctype: `hkReal`)
    /// - offset: `  0`(x86)/`216`(x86_64)
    /// - type_size: `  0`(x86)/`  4`(x86_64)
    #[cfg_attr(feature = "serde_default", serde(default))]
    #[cfg_attr(feature = "json_schema", schemars(rename = "floatVariable17"))]
    #[cfg_attr(feature = "serde", serde(rename = "floatVariable17"))]
    pub m_floatVariable17: f32,
    /// # C++ Info
    /// - name: `floatValue17`(ctype: `hkReal`)
    /// - offset: `  0`(x86)/`220`(x86_64)
    /// - type_size: `  0`(x86)/`  4`(x86_64)
    #[cfg_attr(feature = "serde_default", serde(default))]
    #[cfg_attr(feature = "json_schema", schemars(rename = "floatValue17"))]
    #[cfg_attr(feature = "serde", serde(rename = "floatValue17"))]
    pub m_floatValue17: f32,
    /// # C++ Info
    /// - name: `floatVariable18`(ctype: `hkReal`)
    /// - offset: `  0`(x86)/`224`(x86_64)
    /// - type_size: `  0`(x86)/`  4`(x86_64)
    #[cfg_attr(feature = "serde_default", serde(default))]
    #[cfg_attr(feature = "json_schema", schemars(rename = "floatVariable18"))]
    #[cfg_attr(feature = "serde", serde(rename = "floatVariable18"))]
    pub m_floatVariable18: f32,
    /// # C++ Info
    /// - name: `floatValue18`(ctype: `hkReal`)
    /// - offset: `  0`(x86)/`228`(x86_64)
    /// - type_size: `  0`(x86)/`  4`(x86_64)
    #[cfg_attr(feature = "serde_default", serde(default))]
    #[cfg_attr(feature = "json_schema", schemars(rename = "floatValue18"))]
    #[cfg_attr(feature = "serde", serde(rename = "floatValue18"))]
    pub m_floatValue18: f32,
    /// # C++ Info
    /// - name: `floatVariable19`(ctype: `hkReal`)
    /// - offset: `  0`(x86)/`232`(x86_64)
    /// - type_size: `  0`(x86)/`  4`(x86_64)
    #[cfg_attr(feature = "serde_default", serde(default))]
    #[cfg_attr(feature = "json_schema", schemars(rename = "floatVariable19"))]
    #[cfg_attr(feature = "serde", serde(rename = "floatVariable19"))]
    pub m_floatVariable19: f32,
    /// # C++ Info
    /// - name: `floatValue19`(ctype: `hkReal`)
    /// - offset: `  0`(x86)/`236`(x86_64)
    /// - type_size: `  0`(x86)/`  4`(x86_64)
    #[cfg_attr(feature = "serde_default", serde(default))]
    #[cfg_attr(feature = "json_schema", schemars(rename = "floatValue19"))]
    #[cfg_attr(feature = "serde", serde(rename = "floatValue19"))]
    pub m_floatValue19: f32,
    /// # C++ Info
    /// - name: `floatVariable20`(ctype: `hkReal`)
    /// - offset: `  0`(x86)/`240`(x86_64)
    /// - type_size: `  0`(x86)/`  4`(x86_64)
    #[cfg_attr(feature = "serde_default", serde(default))]
    #[cfg_attr(feature = "json_schema", schemars(rename = "floatVariable20"))]
    #[cfg_attr(feature = "serde", serde(rename = "floatVariable20"))]
    pub m_floatVariable20: f32,
    /// # C++ Info
    /// - name: `floatValue20`(ctype: `hkReal`)
    /// - offset: `  0`(x86)/`244`(x86_64)
    /// - type_size: `  0`(x86)/`  4`(x86_64)
    #[cfg_attr(feature = "serde_default", serde(default))]
    #[cfg_attr(feature = "json_schema", schemars(rename = "floatValue20"))]
    #[cfg_attr(feature = "serde", serde(rename = "floatValue20"))]
    pub m_floatValue20: f32,
    /// # C++ Info
    /// - name: `intVariable1`(ctype: `hkInt32`)
    /// - offset: `  0`(x86)/`248`(x86_64)
    /// - type_size: `  0`(x86)/`  4`(x86_64)
    #[cfg_attr(feature = "serde_default", serde(default))]
    #[cfg_attr(feature = "serde", serde(borrow))]
    #[cfg_attr(feature = "json_schema", schemars(rename = "intVariable1"))]
    #[cfg_attr(feature = "serde", serde(rename = "intVariable1"))]
    pub m_intVariable1: I32<'a>,
    /// # C++ Info
    /// - name: `intValue1`(ctype: `hkInt32`)
    /// - offset: `  0`(x86)/`252`(x86_64)
    /// - type_size: `  0`(x86)/`  4`(x86_64)
    #[cfg_attr(feature = "serde_default", serde(default))]
    #[cfg_attr(feature = "serde", serde(borrow))]
    #[cfg_attr(feature = "json_schema", schemars(rename = "intValue1"))]
    #[cfg_attr(feature = "serde", serde(rename = "intValue1"))]
    pub m_intValue1: I32<'a>,
    /// # C++ Info
    /// - name: `intVariable2`(ctype: `hkInt32`)
    /// - offset: `  0`(x86)/`256`(x86_64)
    /// - type_size: `  0`(x86)/`  4`(x86_64)
    #[cfg_attr(feature = "serde_default", serde(default))]
    #[cfg_attr(feature = "serde", serde(borrow))]
    #[cfg_attr(feature = "json_schema", schemars(rename = "intVariable2"))]
    #[cfg_attr(feature = "serde", serde(rename = "intVariable2"))]
    pub m_intVariable2: I32<'a>,
    /// # C++ Info
    /// - name: `intValue2`(ctype: `hkInt32`)
    /// - offset: `  0`(x86)/`260`(x86_64)
    /// - type_size: `  0`(x86)/`  4`(x86_64)
    #[cfg_attr(feature = "serde_default", serde(default))]
    #[cfg_attr(feature = "serde", serde(borrow))]
    #[cfg_attr(feature = "json_schema", schemars(rename = "intValue2"))]
    #[cfg_attr(feature = "serde", serde(rename = "intValue2"))]
    pub m_intValue2: I32<'a>,
    /// # C++ Info
    /// - name: `intVariable3`(ctype: `hkInt32`)
    /// - offset: `  0`(x86)/`264`(x86_64)
    /// - type_size: `  0`(x86)/`  4`(x86_64)
    #[cfg_attr(feature = "serde_default", serde(default))]
    #[cfg_attr(feature = "serde", serde(borrow))]
    #[cfg_attr(feature = "json_schema", schemars(rename = "intVariable3"))]
    #[cfg_attr(feature = "serde", serde(rename = "intVariable3"))]
    pub m_intVariable3: I32<'a>,
    /// # C++ Info
    /// - name: `intValue3`(ctype: `hkInt32`)
    /// - offset: `  0`(x86)/`268`(x86_64)
    /// - type_size: `  0`(x86)/`  4`(x86_64)
    #[cfg_attr(feature = "serde_default", serde(default))]
    #[cfg_attr(feature = "serde", serde(borrow))]
    #[cfg_attr(feature = "json_schema", schemars(rename = "intValue3"))]
    #[cfg_attr(feature = "serde", serde(rename = "intValue3"))]
    pub m_intValue3: I32<'a>,
    /// # C++ Info
    /// - name: `intVariable4`(ctype: `hkInt32`)
    /// - offset: `  0`(x86)/`272`(x86_64)
    /// - type_size: `  0`(x86)/`  4`(x86_64)
    #[cfg_attr(feature = "serde_default", serde(default))]
    #[cfg_attr(feature = "serde", serde(borrow))]
    #[cfg_attr(feature = "json_schema", schemars(rename = "intVariable4"))]
    #[cfg_attr(feature = "serde", serde(rename = "intVariable4"))]
    pub m_intVariable4: I32<'a>,
    /// # C++ Info
    /// - name: `intValue4`(ctype: `hkInt32`)
    /// - offset: `  0`(x86)/`276`(x86_64)
    /// - type_size: `  0`(x86)/`  4`(x86_64)
    #[cfg_attr(feature = "serde_default", serde(default))]
    #[cfg_attr(feature = "serde", serde(borrow))]
    #[cfg_attr(feature = "json_schema", schemars(rename = "intValue4"))]
    #[cfg_attr(feature = "serde", serde(rename = "intValue4"))]
    pub m_intValue4: I32<'a>,
}
const _: () = {
    use havok_serde as _serde;
    impl<'a> _serde::HavokClass for BSAssignVariablesModifier<'a> {
        #[inline]
        fn name(&self) -> &'static str {
            "BSAssignVariablesModifier"
        }
        #[inline]
        fn signature(&self) -> _serde::__private::Signature {
            _serde::__private::Signature::new(0x64a6ca08)
        }
        #[allow(clippy::let_and_return, clippy::vec_init_then_push)]
        fn deps_indexes(&self) -> Vec<&Pointer<'_>> {
            let mut v = Vec::new();
            v.push(&self.parent.parent.parent.m_variableBindingSet);
            v.extend(self.parent.parent.parent.m_cachedBindables.iter());
            v.push(&self.parent.parent.m_nodeInfo);
            v
        }
    }
    impl<'a> _serde::Serialize for BSAssignVariablesModifier<'a> {
        fn serialize<S>(&self, __serializer: S) -> Result<S::Ok, S::Error>
        where
            S: _serde::ser::Serializer,
        {
            let class_meta = self
                .__ptr
                .as_ref()
                .map(|name| (name, _serde::__private::Signature::new(0x64a6ca08)));
            let mut serializer = __serializer
                .serialize_struct(
                    "BSAssignVariablesModifier",
                    class_meta,
                    (0u64, 280u64),
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
            serializer.serialize_field("floatVariable1", &self.m_floatVariable1)?;
            serializer.serialize_field("floatValue1", &self.m_floatValue1)?;
            serializer.serialize_field("floatVariable2", &self.m_floatVariable2)?;
            serializer.serialize_field("floatValue2", &self.m_floatValue2)?;
            serializer.serialize_field("floatVariable3", &self.m_floatVariable3)?;
            serializer.serialize_field("floatValue3", &self.m_floatValue3)?;
            serializer.serialize_field("floatVariable4", &self.m_floatVariable4)?;
            serializer.serialize_field("floatValue4", &self.m_floatValue4)?;
            serializer.serialize_field("floatVariable5", &self.m_floatVariable5)?;
            serializer.serialize_field("floatValue5", &self.m_floatValue5)?;
            serializer.serialize_field("floatVariable6", &self.m_floatVariable6)?;
            serializer.serialize_field("floatValue6", &self.m_floatValue6)?;
            serializer.serialize_field("floatVariable7", &self.m_floatVariable7)?;
            serializer.serialize_field("floatValue7", &self.m_floatValue7)?;
            serializer.serialize_field("floatVariable8", &self.m_floatVariable8)?;
            serializer.serialize_field("floatValue8", &self.m_floatValue8)?;
            serializer.serialize_field("floatVariable9", &self.m_floatVariable9)?;
            serializer.serialize_field("floatValue9", &self.m_floatValue9)?;
            serializer.serialize_field("floatVariable10", &self.m_floatVariable10)?;
            serializer.serialize_field("floatValue10", &self.m_floatValue10)?;
            serializer.serialize_field("floatVariable11", &self.m_floatVariable11)?;
            serializer.serialize_field("floatValue11", &self.m_floatValue11)?;
            serializer.serialize_field("floatVariable12", &self.m_floatVariable12)?;
            serializer.serialize_field("floatValue12", &self.m_floatValue12)?;
            serializer.serialize_field("floatVariable13", &self.m_floatVariable13)?;
            serializer.serialize_field("floatValue13", &self.m_floatValue13)?;
            serializer.serialize_field("floatVariable14", &self.m_floatVariable14)?;
            serializer.serialize_field("floatValue14", &self.m_floatValue14)?;
            serializer.serialize_field("floatVariable15", &self.m_floatVariable15)?;
            serializer.serialize_field("floatValue15", &self.m_floatValue15)?;
            serializer.serialize_field("floatVariable16", &self.m_floatVariable16)?;
            serializer.serialize_field("floatValue16", &self.m_floatValue16)?;
            serializer.serialize_field("floatVariable17", &self.m_floatVariable17)?;
            serializer.serialize_field("floatValue17", &self.m_floatValue17)?;
            serializer.serialize_field("floatVariable18", &self.m_floatVariable18)?;
            serializer.serialize_field("floatValue18", &self.m_floatValue18)?;
            serializer.serialize_field("floatVariable19", &self.m_floatVariable19)?;
            serializer.serialize_field("floatValue19", &self.m_floatValue19)?;
            serializer.serialize_field("floatVariable20", &self.m_floatVariable20)?;
            serializer.serialize_field("floatValue20", &self.m_floatValue20)?;
            serializer.serialize_field("intVariable1", &self.m_intVariable1)?;
            serializer.serialize_field("intValue1", &self.m_intValue1)?;
            serializer.serialize_field("intVariable2", &self.m_intVariable2)?;
            serializer.serialize_field("intValue2", &self.m_intValue2)?;
            serializer.serialize_field("intVariable3", &self.m_intVariable3)?;
            serializer.serialize_field("intValue3", &self.m_intValue3)?;
            serializer.serialize_field("intVariable4", &self.m_intVariable4)?;
            serializer.serialize_field("intValue4", &self.m_intValue4)?;
            serializer.end()
        }
    }
};
#[doc(hidden)]
#[allow(non_upper_case_globals, unused_attributes, unused_qualifications)]
const _: () = {
    use havok_serde as _serde;
    #[automatically_derived]
    impl<'de> _serde::Deserialize<'de> for BSAssignVariablesModifier<'de> {
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
                m_floatVariable1,
                m_floatValue1,
                m_floatVariable2,
                m_floatValue2,
                m_floatVariable3,
                m_floatValue3,
                m_floatVariable4,
                m_floatValue4,
                m_floatVariable5,
                m_floatValue5,
                m_floatVariable6,
                m_floatValue6,
                m_floatVariable7,
                m_floatValue7,
                m_floatVariable8,
                m_floatValue8,
                m_floatVariable9,
                m_floatValue9,
                m_floatVariable10,
                m_floatValue10,
                m_floatVariable11,
                m_floatValue11,
                m_floatVariable12,
                m_floatValue12,
                m_floatVariable13,
                m_floatValue13,
                m_floatVariable14,
                m_floatValue14,
                m_floatVariable15,
                m_floatValue15,
                m_floatVariable16,
                m_floatValue16,
                m_floatVariable17,
                m_floatValue17,
                m_floatVariable18,
                m_floatValue18,
                m_floatVariable19,
                m_floatValue19,
                m_floatVariable20,
                m_floatValue20,
                m_intVariable1,
                m_intValue1,
                m_intVariable2,
                m_intValue2,
                m_intVariable3,
                m_intValue3,
                m_intVariable4,
                m_intValue4,
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
                        "floatVariable1" => Ok(__Field::m_floatVariable1),
                        "floatValue1" => Ok(__Field::m_floatValue1),
                        "floatVariable2" => Ok(__Field::m_floatVariable2),
                        "floatValue2" => Ok(__Field::m_floatValue2),
                        "floatVariable3" => Ok(__Field::m_floatVariable3),
                        "floatValue3" => Ok(__Field::m_floatValue3),
                        "floatVariable4" => Ok(__Field::m_floatVariable4),
                        "floatValue4" => Ok(__Field::m_floatValue4),
                        "floatVariable5" => Ok(__Field::m_floatVariable5),
                        "floatValue5" => Ok(__Field::m_floatValue5),
                        "floatVariable6" => Ok(__Field::m_floatVariable6),
                        "floatValue6" => Ok(__Field::m_floatValue6),
                        "floatVariable7" => Ok(__Field::m_floatVariable7),
                        "floatValue7" => Ok(__Field::m_floatValue7),
                        "floatVariable8" => Ok(__Field::m_floatVariable8),
                        "floatValue8" => Ok(__Field::m_floatValue8),
                        "floatVariable9" => Ok(__Field::m_floatVariable9),
                        "floatValue9" => Ok(__Field::m_floatValue9),
                        "floatVariable10" => Ok(__Field::m_floatVariable10),
                        "floatValue10" => Ok(__Field::m_floatValue10),
                        "floatVariable11" => Ok(__Field::m_floatVariable11),
                        "floatValue11" => Ok(__Field::m_floatValue11),
                        "floatVariable12" => Ok(__Field::m_floatVariable12),
                        "floatValue12" => Ok(__Field::m_floatValue12),
                        "floatVariable13" => Ok(__Field::m_floatVariable13),
                        "floatValue13" => Ok(__Field::m_floatValue13),
                        "floatVariable14" => Ok(__Field::m_floatVariable14),
                        "floatValue14" => Ok(__Field::m_floatValue14),
                        "floatVariable15" => Ok(__Field::m_floatVariable15),
                        "floatValue15" => Ok(__Field::m_floatValue15),
                        "floatVariable16" => Ok(__Field::m_floatVariable16),
                        "floatValue16" => Ok(__Field::m_floatValue16),
                        "floatVariable17" => Ok(__Field::m_floatVariable17),
                        "floatValue17" => Ok(__Field::m_floatValue17),
                        "floatVariable18" => Ok(__Field::m_floatVariable18),
                        "floatValue18" => Ok(__Field::m_floatValue18),
                        "floatVariable19" => Ok(__Field::m_floatVariable19),
                        "floatValue19" => Ok(__Field::m_floatValue19),
                        "floatVariable20" => Ok(__Field::m_floatVariable20),
                        "floatValue20" => Ok(__Field::m_floatValue20),
                        "intVariable1" => Ok(__Field::m_intVariable1),
                        "intValue1" => Ok(__Field::m_intValue1),
                        "intVariable2" => Ok(__Field::m_intVariable2),
                        "intValue2" => Ok(__Field::m_intValue2),
                        "intVariable3" => Ok(__Field::m_intVariable3),
                        "intValue3" => Ok(__Field::m_intValue3),
                        "intVariable4" => Ok(__Field::m_intVariable4),
                        "intValue4" => Ok(__Field::m_intValue4),
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
            struct __BSAssignVariablesModifierVisitor<'de> {
                marker: _serde::__private::PhantomData<BSAssignVariablesModifier<'de>>,
                lifetime: _serde::__private::PhantomData<&'de ()>,
            }
            #[allow(clippy::match_single_binding)]
            #[allow(clippy::reversed_empty_ranges)]
            #[allow(clippy::single_match)]
            impl<'de> _serde::de::Visitor<'de>
            for __BSAssignVariablesModifierVisitor<'de> {
                type Value = BSAssignVariablesModifier<'de>;
                fn expecting(
                    &self,
                    __formatter: &mut core::fmt::Formatter,
                ) -> core::fmt::Result {
                    core::fmt::Formatter::write_str(
                        __formatter,
                        "struct BSAssignVariablesModifier",
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
                    let mut m_floatVariable1: _serde::__private::Option<f32> = _serde::__private::None;
                    let mut m_floatValue1: _serde::__private::Option<f32> = _serde::__private::None;
                    let mut m_floatVariable2: _serde::__private::Option<f32> = _serde::__private::None;
                    let mut m_floatValue2: _serde::__private::Option<f32> = _serde::__private::None;
                    let mut m_floatVariable3: _serde::__private::Option<f32> = _serde::__private::None;
                    let mut m_floatValue3: _serde::__private::Option<f32> = _serde::__private::None;
                    let mut m_floatVariable4: _serde::__private::Option<f32> = _serde::__private::None;
                    let mut m_floatValue4: _serde::__private::Option<f32> = _serde::__private::None;
                    let mut m_floatVariable5: _serde::__private::Option<f32> = _serde::__private::None;
                    let mut m_floatValue5: _serde::__private::Option<f32> = _serde::__private::None;
                    let mut m_floatVariable6: _serde::__private::Option<f32> = _serde::__private::None;
                    let mut m_floatValue6: _serde::__private::Option<f32> = _serde::__private::None;
                    let mut m_floatVariable7: _serde::__private::Option<f32> = _serde::__private::None;
                    let mut m_floatValue7: _serde::__private::Option<f32> = _serde::__private::None;
                    let mut m_floatVariable8: _serde::__private::Option<f32> = _serde::__private::None;
                    let mut m_floatValue8: _serde::__private::Option<f32> = _serde::__private::None;
                    let mut m_floatVariable9: _serde::__private::Option<f32> = _serde::__private::None;
                    let mut m_floatValue9: _serde::__private::Option<f32> = _serde::__private::None;
                    let mut m_floatVariable10: _serde::__private::Option<f32> = _serde::__private::None;
                    let mut m_floatValue10: _serde::__private::Option<f32> = _serde::__private::None;
                    let mut m_floatVariable11: _serde::__private::Option<f32> = _serde::__private::None;
                    let mut m_floatValue11: _serde::__private::Option<f32> = _serde::__private::None;
                    let mut m_floatVariable12: _serde::__private::Option<f32> = _serde::__private::None;
                    let mut m_floatValue12: _serde::__private::Option<f32> = _serde::__private::None;
                    let mut m_floatVariable13: _serde::__private::Option<f32> = _serde::__private::None;
                    let mut m_floatValue13: _serde::__private::Option<f32> = _serde::__private::None;
                    let mut m_floatVariable14: _serde::__private::Option<f32> = _serde::__private::None;
                    let mut m_floatValue14: _serde::__private::Option<f32> = _serde::__private::None;
                    let mut m_floatVariable15: _serde::__private::Option<f32> = _serde::__private::None;
                    let mut m_floatValue15: _serde::__private::Option<f32> = _serde::__private::None;
                    let mut m_floatVariable16: _serde::__private::Option<f32> = _serde::__private::None;
                    let mut m_floatValue16: _serde::__private::Option<f32> = _serde::__private::None;
                    let mut m_floatVariable17: _serde::__private::Option<f32> = _serde::__private::None;
                    let mut m_floatValue17: _serde::__private::Option<f32> = _serde::__private::None;
                    let mut m_floatVariable18: _serde::__private::Option<f32> = _serde::__private::None;
                    let mut m_floatValue18: _serde::__private::Option<f32> = _serde::__private::None;
                    let mut m_floatVariable19: _serde::__private::Option<f32> = _serde::__private::None;
                    let mut m_floatValue19: _serde::__private::Option<f32> = _serde::__private::None;
                    let mut m_floatVariable20: _serde::__private::Option<f32> = _serde::__private::None;
                    let mut m_floatValue20: _serde::__private::Option<f32> = _serde::__private::None;
                    let mut m_intVariable1: _serde::__private::Option<I32<'de>> = _serde::__private::None;
                    let mut m_intValue1: _serde::__private::Option<I32<'de>> = _serde::__private::None;
                    let mut m_intVariable2: _serde::__private::Option<I32<'de>> = _serde::__private::None;
                    let mut m_intValue2: _serde::__private::Option<I32<'de>> = _serde::__private::None;
                    let mut m_intVariable3: _serde::__private::Option<I32<'de>> = _serde::__private::None;
                    let mut m_intValue3: _serde::__private::Option<I32<'de>> = _serde::__private::None;
                    let mut m_intVariable4: _serde::__private::Option<I32<'de>> = _serde::__private::None;
                    let mut m_intValue4: _serde::__private::Option<I32<'de>> = _serde::__private::None;
                    for i in 0..48usize {
                        match i {
                            0usize => {
                                if _serde::__private::Option::is_some(&m_floatVariable1) {
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "floatVariable1",
                                        ),
                                    );
                                }
                                m_floatVariable1 = _serde::__private::Some(
                                    match __A::next_value::<f32>(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            1usize => {
                                if _serde::__private::Option::is_some(&m_floatValue1) {
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "floatValue1",
                                        ),
                                    );
                                }
                                m_floatValue1 = _serde::__private::Some(
                                    match __A::next_value::<f32>(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            2usize => {
                                if _serde::__private::Option::is_some(&m_floatVariable2) {
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "floatVariable2",
                                        ),
                                    );
                                }
                                m_floatVariable2 = _serde::__private::Some(
                                    match __A::next_value::<f32>(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            3usize => {
                                if _serde::__private::Option::is_some(&m_floatValue2) {
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "floatValue2",
                                        ),
                                    );
                                }
                                m_floatValue2 = _serde::__private::Some(
                                    match __A::next_value::<f32>(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            4usize => {
                                if _serde::__private::Option::is_some(&m_floatVariable3) {
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "floatVariable3",
                                        ),
                                    );
                                }
                                m_floatVariable3 = _serde::__private::Some(
                                    match __A::next_value::<f32>(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            5usize => {
                                if _serde::__private::Option::is_some(&m_floatValue3) {
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "floatValue3",
                                        ),
                                    );
                                }
                                m_floatValue3 = _serde::__private::Some(
                                    match __A::next_value::<f32>(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            6usize => {
                                if _serde::__private::Option::is_some(&m_floatVariable4) {
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "floatVariable4",
                                        ),
                                    );
                                }
                                m_floatVariable4 = _serde::__private::Some(
                                    match __A::next_value::<f32>(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            7usize => {
                                if _serde::__private::Option::is_some(&m_floatValue4) {
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "floatValue4",
                                        ),
                                    );
                                }
                                m_floatValue4 = _serde::__private::Some(
                                    match __A::next_value::<f32>(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            8usize => {
                                if _serde::__private::Option::is_some(&m_floatVariable5) {
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "floatVariable5",
                                        ),
                                    );
                                }
                                m_floatVariable5 = _serde::__private::Some(
                                    match __A::next_value::<f32>(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            9usize => {
                                if _serde::__private::Option::is_some(&m_floatValue5) {
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "floatValue5",
                                        ),
                                    );
                                }
                                m_floatValue5 = _serde::__private::Some(
                                    match __A::next_value::<f32>(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            10usize => {
                                if _serde::__private::Option::is_some(&m_floatVariable6) {
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "floatVariable6",
                                        ),
                                    );
                                }
                                m_floatVariable6 = _serde::__private::Some(
                                    match __A::next_value::<f32>(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            11usize => {
                                if _serde::__private::Option::is_some(&m_floatValue6) {
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "floatValue6",
                                        ),
                                    );
                                }
                                m_floatValue6 = _serde::__private::Some(
                                    match __A::next_value::<f32>(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            12usize => {
                                if _serde::__private::Option::is_some(&m_floatVariable7) {
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "floatVariable7",
                                        ),
                                    );
                                }
                                m_floatVariable7 = _serde::__private::Some(
                                    match __A::next_value::<f32>(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            13usize => {
                                if _serde::__private::Option::is_some(&m_floatValue7) {
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "floatValue7",
                                        ),
                                    );
                                }
                                m_floatValue7 = _serde::__private::Some(
                                    match __A::next_value::<f32>(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            14usize => {
                                if _serde::__private::Option::is_some(&m_floatVariable8) {
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "floatVariable8",
                                        ),
                                    );
                                }
                                m_floatVariable8 = _serde::__private::Some(
                                    match __A::next_value::<f32>(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            15usize => {
                                if _serde::__private::Option::is_some(&m_floatValue8) {
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "floatValue8",
                                        ),
                                    );
                                }
                                m_floatValue8 = _serde::__private::Some(
                                    match __A::next_value::<f32>(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            16usize => {
                                if _serde::__private::Option::is_some(&m_floatVariable9) {
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "floatVariable9",
                                        ),
                                    );
                                }
                                m_floatVariable9 = _serde::__private::Some(
                                    match __A::next_value::<f32>(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            17usize => {
                                if _serde::__private::Option::is_some(&m_floatValue9) {
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "floatValue9",
                                        ),
                                    );
                                }
                                m_floatValue9 = _serde::__private::Some(
                                    match __A::next_value::<f32>(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            18usize => {
                                if _serde::__private::Option::is_some(&m_floatVariable10) {
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "floatVariable10",
                                        ),
                                    );
                                }
                                m_floatVariable10 = _serde::__private::Some(
                                    match __A::next_value::<f32>(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            19usize => {
                                if _serde::__private::Option::is_some(&m_floatValue10) {
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "floatValue10",
                                        ),
                                    );
                                }
                                m_floatValue10 = _serde::__private::Some(
                                    match __A::next_value::<f32>(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            20usize => {
                                if _serde::__private::Option::is_some(&m_floatVariable11) {
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "floatVariable11",
                                        ),
                                    );
                                }
                                m_floatVariable11 = _serde::__private::Some(
                                    match __A::next_value::<f32>(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            21usize => {
                                if _serde::__private::Option::is_some(&m_floatValue11) {
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "floatValue11",
                                        ),
                                    );
                                }
                                m_floatValue11 = _serde::__private::Some(
                                    match __A::next_value::<f32>(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            22usize => {
                                if _serde::__private::Option::is_some(&m_floatVariable12) {
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "floatVariable12",
                                        ),
                                    );
                                }
                                m_floatVariable12 = _serde::__private::Some(
                                    match __A::next_value::<f32>(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            23usize => {
                                if _serde::__private::Option::is_some(&m_floatValue12) {
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "floatValue12",
                                        ),
                                    );
                                }
                                m_floatValue12 = _serde::__private::Some(
                                    match __A::next_value::<f32>(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            24usize => {
                                if _serde::__private::Option::is_some(&m_floatVariable13) {
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "floatVariable13",
                                        ),
                                    );
                                }
                                m_floatVariable13 = _serde::__private::Some(
                                    match __A::next_value::<f32>(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            25usize => {
                                if _serde::__private::Option::is_some(&m_floatValue13) {
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "floatValue13",
                                        ),
                                    );
                                }
                                m_floatValue13 = _serde::__private::Some(
                                    match __A::next_value::<f32>(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            26usize => {
                                if _serde::__private::Option::is_some(&m_floatVariable14) {
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "floatVariable14",
                                        ),
                                    );
                                }
                                m_floatVariable14 = _serde::__private::Some(
                                    match __A::next_value::<f32>(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            27usize => {
                                if _serde::__private::Option::is_some(&m_floatValue14) {
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "floatValue14",
                                        ),
                                    );
                                }
                                m_floatValue14 = _serde::__private::Some(
                                    match __A::next_value::<f32>(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            28usize => {
                                if _serde::__private::Option::is_some(&m_floatVariable15) {
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "floatVariable15",
                                        ),
                                    );
                                }
                                m_floatVariable15 = _serde::__private::Some(
                                    match __A::next_value::<f32>(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            29usize => {
                                if _serde::__private::Option::is_some(&m_floatValue15) {
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "floatValue15",
                                        ),
                                    );
                                }
                                m_floatValue15 = _serde::__private::Some(
                                    match __A::next_value::<f32>(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            30usize => {
                                if _serde::__private::Option::is_some(&m_floatVariable16) {
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "floatVariable16",
                                        ),
                                    );
                                }
                                m_floatVariable16 = _serde::__private::Some(
                                    match __A::next_value::<f32>(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            31usize => {
                                if _serde::__private::Option::is_some(&m_floatValue16) {
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "floatValue16",
                                        ),
                                    );
                                }
                                m_floatValue16 = _serde::__private::Some(
                                    match __A::next_value::<f32>(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            32usize => {
                                if _serde::__private::Option::is_some(&m_floatVariable17) {
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "floatVariable17",
                                        ),
                                    );
                                }
                                m_floatVariable17 = _serde::__private::Some(
                                    match __A::next_value::<f32>(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            33usize => {
                                if _serde::__private::Option::is_some(&m_floatValue17) {
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "floatValue17",
                                        ),
                                    );
                                }
                                m_floatValue17 = _serde::__private::Some(
                                    match __A::next_value::<f32>(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            34usize => {
                                if _serde::__private::Option::is_some(&m_floatVariable18) {
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "floatVariable18",
                                        ),
                                    );
                                }
                                m_floatVariable18 = _serde::__private::Some(
                                    match __A::next_value::<f32>(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            35usize => {
                                if _serde::__private::Option::is_some(&m_floatValue18) {
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "floatValue18",
                                        ),
                                    );
                                }
                                m_floatValue18 = _serde::__private::Some(
                                    match __A::next_value::<f32>(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            36usize => {
                                if _serde::__private::Option::is_some(&m_floatVariable19) {
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "floatVariable19",
                                        ),
                                    );
                                }
                                m_floatVariable19 = _serde::__private::Some(
                                    match __A::next_value::<f32>(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            37usize => {
                                if _serde::__private::Option::is_some(&m_floatValue19) {
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "floatValue19",
                                        ),
                                    );
                                }
                                m_floatValue19 = _serde::__private::Some(
                                    match __A::next_value::<f32>(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            38usize => {
                                if _serde::__private::Option::is_some(&m_floatVariable20) {
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "floatVariable20",
                                        ),
                                    );
                                }
                                m_floatVariable20 = _serde::__private::Some(
                                    match __A::next_value::<f32>(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            39usize => {
                                if _serde::__private::Option::is_some(&m_floatValue20) {
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "floatValue20",
                                        ),
                                    );
                                }
                                m_floatValue20 = _serde::__private::Some(
                                    match __A::next_value::<f32>(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            40usize => {
                                if _serde::__private::Option::is_some(&m_intVariable1) {
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "intVariable1",
                                        ),
                                    );
                                }
                                m_intVariable1 = _serde::__private::Some(
                                    match __A::next_value::<I32<'de>>(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            41usize => {
                                if _serde::__private::Option::is_some(&m_intValue1) {
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "intValue1",
                                        ),
                                    );
                                }
                                m_intValue1 = _serde::__private::Some(
                                    match __A::next_value::<I32<'de>>(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            42usize => {
                                if _serde::__private::Option::is_some(&m_intVariable2) {
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "intVariable2",
                                        ),
                                    );
                                }
                                m_intVariable2 = _serde::__private::Some(
                                    match __A::next_value::<I32<'de>>(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            43usize => {
                                if _serde::__private::Option::is_some(&m_intValue2) {
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "intValue2",
                                        ),
                                    );
                                }
                                m_intValue2 = _serde::__private::Some(
                                    match __A::next_value::<I32<'de>>(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            44usize => {
                                if _serde::__private::Option::is_some(&m_intVariable3) {
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "intVariable3",
                                        ),
                                    );
                                }
                                m_intVariable3 = _serde::__private::Some(
                                    match __A::next_value::<I32<'de>>(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            45usize => {
                                if _serde::__private::Option::is_some(&m_intValue3) {
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "intValue3",
                                        ),
                                    );
                                }
                                m_intValue3 = _serde::__private::Some(
                                    match __A::next_value::<I32<'de>>(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            46usize => {
                                if _serde::__private::Option::is_some(&m_intVariable4) {
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "intVariable4",
                                        ),
                                    );
                                }
                                m_intVariable4 = _serde::__private::Some(
                                    match __A::next_value::<I32<'de>>(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            47usize => {
                                if _serde::__private::Option::is_some(&m_intValue4) {
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "intValue4",
                                        ),
                                    );
                                }
                                m_intValue4 = _serde::__private::Some(
                                    match __A::next_value::<I32<'de>>(&mut __map) {
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
                    let m_floatVariable1 = match m_floatVariable1 {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "floatVariable1",
                                ),
                            );
                        }
                    };
                    let m_floatValue1 = match m_floatValue1 {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "floatValue1",
                                ),
                            );
                        }
                    };
                    let m_floatVariable2 = match m_floatVariable2 {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "floatVariable2",
                                ),
                            );
                        }
                    };
                    let m_floatValue2 = match m_floatValue2 {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "floatValue2",
                                ),
                            );
                        }
                    };
                    let m_floatVariable3 = match m_floatVariable3 {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "floatVariable3",
                                ),
                            );
                        }
                    };
                    let m_floatValue3 = match m_floatValue3 {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "floatValue3",
                                ),
                            );
                        }
                    };
                    let m_floatVariable4 = match m_floatVariable4 {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "floatVariable4",
                                ),
                            );
                        }
                    };
                    let m_floatValue4 = match m_floatValue4 {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "floatValue4",
                                ),
                            );
                        }
                    };
                    let m_floatVariable5 = match m_floatVariable5 {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "floatVariable5",
                                ),
                            );
                        }
                    };
                    let m_floatValue5 = match m_floatValue5 {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "floatValue5",
                                ),
                            );
                        }
                    };
                    let m_floatVariable6 = match m_floatVariable6 {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "floatVariable6",
                                ),
                            );
                        }
                    };
                    let m_floatValue6 = match m_floatValue6 {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "floatValue6",
                                ),
                            );
                        }
                    };
                    let m_floatVariable7 = match m_floatVariable7 {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "floatVariable7",
                                ),
                            );
                        }
                    };
                    let m_floatValue7 = match m_floatValue7 {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "floatValue7",
                                ),
                            );
                        }
                    };
                    let m_floatVariable8 = match m_floatVariable8 {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "floatVariable8",
                                ),
                            );
                        }
                    };
                    let m_floatValue8 = match m_floatValue8 {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "floatValue8",
                                ),
                            );
                        }
                    };
                    let m_floatVariable9 = match m_floatVariable9 {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "floatVariable9",
                                ),
                            );
                        }
                    };
                    let m_floatValue9 = match m_floatValue9 {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "floatValue9",
                                ),
                            );
                        }
                    };
                    let m_floatVariable10 = match m_floatVariable10 {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "floatVariable10",
                                ),
                            );
                        }
                    };
                    let m_floatValue10 = match m_floatValue10 {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "floatValue10",
                                ),
                            );
                        }
                    };
                    let m_floatVariable11 = match m_floatVariable11 {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "floatVariable11",
                                ),
                            );
                        }
                    };
                    let m_floatValue11 = match m_floatValue11 {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "floatValue11",
                                ),
                            );
                        }
                    };
                    let m_floatVariable12 = match m_floatVariable12 {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "floatVariable12",
                                ),
                            );
                        }
                    };
                    let m_floatValue12 = match m_floatValue12 {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "floatValue12",
                                ),
                            );
                        }
                    };
                    let m_floatVariable13 = match m_floatVariable13 {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "floatVariable13",
                                ),
                            );
                        }
                    };
                    let m_floatValue13 = match m_floatValue13 {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "floatValue13",
                                ),
                            );
                        }
                    };
                    let m_floatVariable14 = match m_floatVariable14 {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "floatVariable14",
                                ),
                            );
                        }
                    };
                    let m_floatValue14 = match m_floatValue14 {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "floatValue14",
                                ),
                            );
                        }
                    };
                    let m_floatVariable15 = match m_floatVariable15 {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "floatVariable15",
                                ),
                            );
                        }
                    };
                    let m_floatValue15 = match m_floatValue15 {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "floatValue15",
                                ),
                            );
                        }
                    };
                    let m_floatVariable16 = match m_floatVariable16 {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "floatVariable16",
                                ),
                            );
                        }
                    };
                    let m_floatValue16 = match m_floatValue16 {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "floatValue16",
                                ),
                            );
                        }
                    };
                    let m_floatVariable17 = match m_floatVariable17 {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "floatVariable17",
                                ),
                            );
                        }
                    };
                    let m_floatValue17 = match m_floatValue17 {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "floatValue17",
                                ),
                            );
                        }
                    };
                    let m_floatVariable18 = match m_floatVariable18 {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "floatVariable18",
                                ),
                            );
                        }
                    };
                    let m_floatValue18 = match m_floatValue18 {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "floatValue18",
                                ),
                            );
                        }
                    };
                    let m_floatVariable19 = match m_floatVariable19 {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "floatVariable19",
                                ),
                            );
                        }
                    };
                    let m_floatValue19 = match m_floatValue19 {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "floatValue19",
                                ),
                            );
                        }
                    };
                    let m_floatVariable20 = match m_floatVariable20 {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "floatVariable20",
                                ),
                            );
                        }
                    };
                    let m_floatValue20 = match m_floatValue20 {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "floatValue20",
                                ),
                            );
                        }
                    };
                    let m_intVariable1 = match m_intVariable1 {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "intVariable1",
                                ),
                            );
                        }
                    };
                    let m_intValue1 = match m_intValue1 {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "intValue1",
                                ),
                            );
                        }
                    };
                    let m_intVariable2 = match m_intVariable2 {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "intVariable2",
                                ),
                            );
                        }
                    };
                    let m_intValue2 = match m_intValue2 {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "intValue2",
                                ),
                            );
                        }
                    };
                    let m_intVariable3 = match m_intVariable3 {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "intVariable3",
                                ),
                            );
                        }
                    };
                    let m_intValue3 = match m_intValue3 {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "intValue3",
                                ),
                            );
                        }
                    };
                    let m_intVariable4 = match m_intVariable4 {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "intVariable4",
                                ),
                            );
                        }
                    };
                    let m_intValue4 = match m_intValue4 {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "intValue4",
                                ),
                            );
                        }
                    };
                    _serde::__private::Ok(BSAssignVariablesModifier {
                        __ptr,
                        parent,
                        m_floatVariable1,
                        m_floatValue1,
                        m_floatVariable2,
                        m_floatValue2,
                        m_floatVariable3,
                        m_floatValue3,
                        m_floatVariable4,
                        m_floatValue4,
                        m_floatVariable5,
                        m_floatValue5,
                        m_floatVariable6,
                        m_floatValue6,
                        m_floatVariable7,
                        m_floatValue7,
                        m_floatVariable8,
                        m_floatValue8,
                        m_floatVariable9,
                        m_floatValue9,
                        m_floatVariable10,
                        m_floatValue10,
                        m_floatVariable11,
                        m_floatValue11,
                        m_floatVariable12,
                        m_floatValue12,
                        m_floatVariable13,
                        m_floatValue13,
                        m_floatVariable14,
                        m_floatValue14,
                        m_floatVariable15,
                        m_floatValue15,
                        m_floatVariable16,
                        m_floatValue16,
                        m_floatVariable17,
                        m_floatValue17,
                        m_floatVariable18,
                        m_floatValue18,
                        m_floatVariable19,
                        m_floatValue19,
                        m_floatVariable20,
                        m_floatValue20,
                        m_intVariable1,
                        m_intValue1,
                        m_intVariable2,
                        m_intValue2,
                        m_intVariable3,
                        m_intValue3,
                        m_intVariable4,
                        m_intValue4,
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
                    let mut m_floatVariable1: _serde::__private::Option<f32> = _serde::__private::None;
                    let mut m_floatValue1: _serde::__private::Option<f32> = _serde::__private::None;
                    let mut m_floatVariable2: _serde::__private::Option<f32> = _serde::__private::None;
                    let mut m_floatValue2: _serde::__private::Option<f32> = _serde::__private::None;
                    let mut m_floatVariable3: _serde::__private::Option<f32> = _serde::__private::None;
                    let mut m_floatValue3: _serde::__private::Option<f32> = _serde::__private::None;
                    let mut m_floatVariable4: _serde::__private::Option<f32> = _serde::__private::None;
                    let mut m_floatValue4: _serde::__private::Option<f32> = _serde::__private::None;
                    let mut m_floatVariable5: _serde::__private::Option<f32> = _serde::__private::None;
                    let mut m_floatValue5: _serde::__private::Option<f32> = _serde::__private::None;
                    let mut m_floatVariable6: _serde::__private::Option<f32> = _serde::__private::None;
                    let mut m_floatValue6: _serde::__private::Option<f32> = _serde::__private::None;
                    let mut m_floatVariable7: _serde::__private::Option<f32> = _serde::__private::None;
                    let mut m_floatValue7: _serde::__private::Option<f32> = _serde::__private::None;
                    let mut m_floatVariable8: _serde::__private::Option<f32> = _serde::__private::None;
                    let mut m_floatValue8: _serde::__private::Option<f32> = _serde::__private::None;
                    let mut m_floatVariable9: _serde::__private::Option<f32> = _serde::__private::None;
                    let mut m_floatValue9: _serde::__private::Option<f32> = _serde::__private::None;
                    let mut m_floatVariable10: _serde::__private::Option<f32> = _serde::__private::None;
                    let mut m_floatValue10: _serde::__private::Option<f32> = _serde::__private::None;
                    let mut m_floatVariable11: _serde::__private::Option<f32> = _serde::__private::None;
                    let mut m_floatValue11: _serde::__private::Option<f32> = _serde::__private::None;
                    let mut m_floatVariable12: _serde::__private::Option<f32> = _serde::__private::None;
                    let mut m_floatValue12: _serde::__private::Option<f32> = _serde::__private::None;
                    let mut m_floatVariable13: _serde::__private::Option<f32> = _serde::__private::None;
                    let mut m_floatValue13: _serde::__private::Option<f32> = _serde::__private::None;
                    let mut m_floatVariable14: _serde::__private::Option<f32> = _serde::__private::None;
                    let mut m_floatValue14: _serde::__private::Option<f32> = _serde::__private::None;
                    let mut m_floatVariable15: _serde::__private::Option<f32> = _serde::__private::None;
                    let mut m_floatValue15: _serde::__private::Option<f32> = _serde::__private::None;
                    let mut m_floatVariable16: _serde::__private::Option<f32> = _serde::__private::None;
                    let mut m_floatValue16: _serde::__private::Option<f32> = _serde::__private::None;
                    let mut m_floatVariable17: _serde::__private::Option<f32> = _serde::__private::None;
                    let mut m_floatValue17: _serde::__private::Option<f32> = _serde::__private::None;
                    let mut m_floatVariable18: _serde::__private::Option<f32> = _serde::__private::None;
                    let mut m_floatValue18: _serde::__private::Option<f32> = _serde::__private::None;
                    let mut m_floatVariable19: _serde::__private::Option<f32> = _serde::__private::None;
                    let mut m_floatValue19: _serde::__private::Option<f32> = _serde::__private::None;
                    let mut m_floatVariable20: _serde::__private::Option<f32> = _serde::__private::None;
                    let mut m_floatValue20: _serde::__private::Option<f32> = _serde::__private::None;
                    let mut m_intVariable1: _serde::__private::Option<I32<'de>> = _serde::__private::None;
                    let mut m_intValue1: _serde::__private::Option<I32<'de>> = _serde::__private::None;
                    let mut m_intVariable2: _serde::__private::Option<I32<'de>> = _serde::__private::None;
                    let mut m_intValue2: _serde::__private::Option<I32<'de>> = _serde::__private::None;
                    let mut m_intVariable3: _serde::__private::Option<I32<'de>> = _serde::__private::None;
                    let mut m_intValue3: _serde::__private::Option<I32<'de>> = _serde::__private::None;
                    let mut m_intVariable4: _serde::__private::Option<I32<'de>> = _serde::__private::None;
                    let mut m_intValue4: _serde::__private::Option<I32<'de>> = _serde::__private::None;
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
                            __Field::m_floatVariable1 => {
                                #[cfg(
                                    any(feature = "strict", feature = "ignore_duplicates")
                                )]
                                if _serde::__private::Option::is_some(&m_floatVariable1) {
                                    #[cfg(feature = "ignore_duplicates")]
                                    {
                                        __A::skip_value(&mut __map)?;
                                        continue;
                                    }
                                    #[cfg(feature = "strict")]
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "floatVariable1",
                                        ),
                                    );
                                }
                                m_floatVariable1 = _serde::__private::Some(
                                    match __A::next_value::<f32>(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            __Field::m_floatValue1 => {
                                #[cfg(
                                    any(feature = "strict", feature = "ignore_duplicates")
                                )]
                                if _serde::__private::Option::is_some(&m_floatValue1) {
                                    #[cfg(feature = "ignore_duplicates")]
                                    {
                                        __A::skip_value(&mut __map)?;
                                        continue;
                                    }
                                    #[cfg(feature = "strict")]
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "floatValue1",
                                        ),
                                    );
                                }
                                m_floatValue1 = _serde::__private::Some(
                                    match __A::next_value::<f32>(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            __Field::m_floatVariable2 => {
                                #[cfg(
                                    any(feature = "strict", feature = "ignore_duplicates")
                                )]
                                if _serde::__private::Option::is_some(&m_floatVariable2) {
                                    #[cfg(feature = "ignore_duplicates")]
                                    {
                                        __A::skip_value(&mut __map)?;
                                        continue;
                                    }
                                    #[cfg(feature = "strict")]
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "floatVariable2",
                                        ),
                                    );
                                }
                                m_floatVariable2 = _serde::__private::Some(
                                    match __A::next_value::<f32>(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            __Field::m_floatValue2 => {
                                #[cfg(
                                    any(feature = "strict", feature = "ignore_duplicates")
                                )]
                                if _serde::__private::Option::is_some(&m_floatValue2) {
                                    #[cfg(feature = "ignore_duplicates")]
                                    {
                                        __A::skip_value(&mut __map)?;
                                        continue;
                                    }
                                    #[cfg(feature = "strict")]
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "floatValue2",
                                        ),
                                    );
                                }
                                m_floatValue2 = _serde::__private::Some(
                                    match __A::next_value::<f32>(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            __Field::m_floatVariable3 => {
                                #[cfg(
                                    any(feature = "strict", feature = "ignore_duplicates")
                                )]
                                if _serde::__private::Option::is_some(&m_floatVariable3) {
                                    #[cfg(feature = "ignore_duplicates")]
                                    {
                                        __A::skip_value(&mut __map)?;
                                        continue;
                                    }
                                    #[cfg(feature = "strict")]
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "floatVariable3",
                                        ),
                                    );
                                }
                                m_floatVariable3 = _serde::__private::Some(
                                    match __A::next_value::<f32>(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            __Field::m_floatValue3 => {
                                #[cfg(
                                    any(feature = "strict", feature = "ignore_duplicates")
                                )]
                                if _serde::__private::Option::is_some(&m_floatValue3) {
                                    #[cfg(feature = "ignore_duplicates")]
                                    {
                                        __A::skip_value(&mut __map)?;
                                        continue;
                                    }
                                    #[cfg(feature = "strict")]
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "floatValue3",
                                        ),
                                    );
                                }
                                m_floatValue3 = _serde::__private::Some(
                                    match __A::next_value::<f32>(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            __Field::m_floatVariable4 => {
                                #[cfg(
                                    any(feature = "strict", feature = "ignore_duplicates")
                                )]
                                if _serde::__private::Option::is_some(&m_floatVariable4) {
                                    #[cfg(feature = "ignore_duplicates")]
                                    {
                                        __A::skip_value(&mut __map)?;
                                        continue;
                                    }
                                    #[cfg(feature = "strict")]
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "floatVariable4",
                                        ),
                                    );
                                }
                                m_floatVariable4 = _serde::__private::Some(
                                    match __A::next_value::<f32>(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            __Field::m_floatValue4 => {
                                #[cfg(
                                    any(feature = "strict", feature = "ignore_duplicates")
                                )]
                                if _serde::__private::Option::is_some(&m_floatValue4) {
                                    #[cfg(feature = "ignore_duplicates")]
                                    {
                                        __A::skip_value(&mut __map)?;
                                        continue;
                                    }
                                    #[cfg(feature = "strict")]
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "floatValue4",
                                        ),
                                    );
                                }
                                m_floatValue4 = _serde::__private::Some(
                                    match __A::next_value::<f32>(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            __Field::m_floatVariable5 => {
                                #[cfg(
                                    any(feature = "strict", feature = "ignore_duplicates")
                                )]
                                if _serde::__private::Option::is_some(&m_floatVariable5) {
                                    #[cfg(feature = "ignore_duplicates")]
                                    {
                                        __A::skip_value(&mut __map)?;
                                        continue;
                                    }
                                    #[cfg(feature = "strict")]
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "floatVariable5",
                                        ),
                                    );
                                }
                                m_floatVariable5 = _serde::__private::Some(
                                    match __A::next_value::<f32>(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            __Field::m_floatValue5 => {
                                #[cfg(
                                    any(feature = "strict", feature = "ignore_duplicates")
                                )]
                                if _serde::__private::Option::is_some(&m_floatValue5) {
                                    #[cfg(feature = "ignore_duplicates")]
                                    {
                                        __A::skip_value(&mut __map)?;
                                        continue;
                                    }
                                    #[cfg(feature = "strict")]
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "floatValue5",
                                        ),
                                    );
                                }
                                m_floatValue5 = _serde::__private::Some(
                                    match __A::next_value::<f32>(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            __Field::m_floatVariable6 => {
                                #[cfg(
                                    any(feature = "strict", feature = "ignore_duplicates")
                                )]
                                if _serde::__private::Option::is_some(&m_floatVariable6) {
                                    #[cfg(feature = "ignore_duplicates")]
                                    {
                                        __A::skip_value(&mut __map)?;
                                        continue;
                                    }
                                    #[cfg(feature = "strict")]
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "floatVariable6",
                                        ),
                                    );
                                }
                                m_floatVariable6 = _serde::__private::Some(
                                    match __A::next_value::<f32>(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            __Field::m_floatValue6 => {
                                #[cfg(
                                    any(feature = "strict", feature = "ignore_duplicates")
                                )]
                                if _serde::__private::Option::is_some(&m_floatValue6) {
                                    #[cfg(feature = "ignore_duplicates")]
                                    {
                                        __A::skip_value(&mut __map)?;
                                        continue;
                                    }
                                    #[cfg(feature = "strict")]
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "floatValue6",
                                        ),
                                    );
                                }
                                m_floatValue6 = _serde::__private::Some(
                                    match __A::next_value::<f32>(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            __Field::m_floatVariable7 => {
                                #[cfg(
                                    any(feature = "strict", feature = "ignore_duplicates")
                                )]
                                if _serde::__private::Option::is_some(&m_floatVariable7) {
                                    #[cfg(feature = "ignore_duplicates")]
                                    {
                                        __A::skip_value(&mut __map)?;
                                        continue;
                                    }
                                    #[cfg(feature = "strict")]
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "floatVariable7",
                                        ),
                                    );
                                }
                                m_floatVariable7 = _serde::__private::Some(
                                    match __A::next_value::<f32>(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            __Field::m_floatValue7 => {
                                #[cfg(
                                    any(feature = "strict", feature = "ignore_duplicates")
                                )]
                                if _serde::__private::Option::is_some(&m_floatValue7) {
                                    #[cfg(feature = "ignore_duplicates")]
                                    {
                                        __A::skip_value(&mut __map)?;
                                        continue;
                                    }
                                    #[cfg(feature = "strict")]
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "floatValue7",
                                        ),
                                    );
                                }
                                m_floatValue7 = _serde::__private::Some(
                                    match __A::next_value::<f32>(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            __Field::m_floatVariable8 => {
                                #[cfg(
                                    any(feature = "strict", feature = "ignore_duplicates")
                                )]
                                if _serde::__private::Option::is_some(&m_floatVariable8) {
                                    #[cfg(feature = "ignore_duplicates")]
                                    {
                                        __A::skip_value(&mut __map)?;
                                        continue;
                                    }
                                    #[cfg(feature = "strict")]
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "floatVariable8",
                                        ),
                                    );
                                }
                                m_floatVariable8 = _serde::__private::Some(
                                    match __A::next_value::<f32>(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            __Field::m_floatValue8 => {
                                #[cfg(
                                    any(feature = "strict", feature = "ignore_duplicates")
                                )]
                                if _serde::__private::Option::is_some(&m_floatValue8) {
                                    #[cfg(feature = "ignore_duplicates")]
                                    {
                                        __A::skip_value(&mut __map)?;
                                        continue;
                                    }
                                    #[cfg(feature = "strict")]
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "floatValue8",
                                        ),
                                    );
                                }
                                m_floatValue8 = _serde::__private::Some(
                                    match __A::next_value::<f32>(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            __Field::m_floatVariable9 => {
                                #[cfg(
                                    any(feature = "strict", feature = "ignore_duplicates")
                                )]
                                if _serde::__private::Option::is_some(&m_floatVariable9) {
                                    #[cfg(feature = "ignore_duplicates")]
                                    {
                                        __A::skip_value(&mut __map)?;
                                        continue;
                                    }
                                    #[cfg(feature = "strict")]
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "floatVariable9",
                                        ),
                                    );
                                }
                                m_floatVariable9 = _serde::__private::Some(
                                    match __A::next_value::<f32>(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            __Field::m_floatValue9 => {
                                #[cfg(
                                    any(feature = "strict", feature = "ignore_duplicates")
                                )]
                                if _serde::__private::Option::is_some(&m_floatValue9) {
                                    #[cfg(feature = "ignore_duplicates")]
                                    {
                                        __A::skip_value(&mut __map)?;
                                        continue;
                                    }
                                    #[cfg(feature = "strict")]
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "floatValue9",
                                        ),
                                    );
                                }
                                m_floatValue9 = _serde::__private::Some(
                                    match __A::next_value::<f32>(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            __Field::m_floatVariable10 => {
                                #[cfg(
                                    any(feature = "strict", feature = "ignore_duplicates")
                                )]
                                if _serde::__private::Option::is_some(&m_floatVariable10) {
                                    #[cfg(feature = "ignore_duplicates")]
                                    {
                                        __A::skip_value(&mut __map)?;
                                        continue;
                                    }
                                    #[cfg(feature = "strict")]
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "floatVariable10",
                                        ),
                                    );
                                }
                                m_floatVariable10 = _serde::__private::Some(
                                    match __A::next_value::<f32>(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            __Field::m_floatValue10 => {
                                #[cfg(
                                    any(feature = "strict", feature = "ignore_duplicates")
                                )]
                                if _serde::__private::Option::is_some(&m_floatValue10) {
                                    #[cfg(feature = "ignore_duplicates")]
                                    {
                                        __A::skip_value(&mut __map)?;
                                        continue;
                                    }
                                    #[cfg(feature = "strict")]
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "floatValue10",
                                        ),
                                    );
                                }
                                m_floatValue10 = _serde::__private::Some(
                                    match __A::next_value::<f32>(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            __Field::m_floatVariable11 => {
                                #[cfg(
                                    any(feature = "strict", feature = "ignore_duplicates")
                                )]
                                if _serde::__private::Option::is_some(&m_floatVariable11) {
                                    #[cfg(feature = "ignore_duplicates")]
                                    {
                                        __A::skip_value(&mut __map)?;
                                        continue;
                                    }
                                    #[cfg(feature = "strict")]
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "floatVariable11",
                                        ),
                                    );
                                }
                                m_floatVariable11 = _serde::__private::Some(
                                    match __A::next_value::<f32>(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            __Field::m_floatValue11 => {
                                #[cfg(
                                    any(feature = "strict", feature = "ignore_duplicates")
                                )]
                                if _serde::__private::Option::is_some(&m_floatValue11) {
                                    #[cfg(feature = "ignore_duplicates")]
                                    {
                                        __A::skip_value(&mut __map)?;
                                        continue;
                                    }
                                    #[cfg(feature = "strict")]
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "floatValue11",
                                        ),
                                    );
                                }
                                m_floatValue11 = _serde::__private::Some(
                                    match __A::next_value::<f32>(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            __Field::m_floatVariable12 => {
                                #[cfg(
                                    any(feature = "strict", feature = "ignore_duplicates")
                                )]
                                if _serde::__private::Option::is_some(&m_floatVariable12) {
                                    #[cfg(feature = "ignore_duplicates")]
                                    {
                                        __A::skip_value(&mut __map)?;
                                        continue;
                                    }
                                    #[cfg(feature = "strict")]
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "floatVariable12",
                                        ),
                                    );
                                }
                                m_floatVariable12 = _serde::__private::Some(
                                    match __A::next_value::<f32>(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            __Field::m_floatValue12 => {
                                #[cfg(
                                    any(feature = "strict", feature = "ignore_duplicates")
                                )]
                                if _serde::__private::Option::is_some(&m_floatValue12) {
                                    #[cfg(feature = "ignore_duplicates")]
                                    {
                                        __A::skip_value(&mut __map)?;
                                        continue;
                                    }
                                    #[cfg(feature = "strict")]
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "floatValue12",
                                        ),
                                    );
                                }
                                m_floatValue12 = _serde::__private::Some(
                                    match __A::next_value::<f32>(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            __Field::m_floatVariable13 => {
                                #[cfg(
                                    any(feature = "strict", feature = "ignore_duplicates")
                                )]
                                if _serde::__private::Option::is_some(&m_floatVariable13) {
                                    #[cfg(feature = "ignore_duplicates")]
                                    {
                                        __A::skip_value(&mut __map)?;
                                        continue;
                                    }
                                    #[cfg(feature = "strict")]
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "floatVariable13",
                                        ),
                                    );
                                }
                                m_floatVariable13 = _serde::__private::Some(
                                    match __A::next_value::<f32>(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            __Field::m_floatValue13 => {
                                #[cfg(
                                    any(feature = "strict", feature = "ignore_duplicates")
                                )]
                                if _serde::__private::Option::is_some(&m_floatValue13) {
                                    #[cfg(feature = "ignore_duplicates")]
                                    {
                                        __A::skip_value(&mut __map)?;
                                        continue;
                                    }
                                    #[cfg(feature = "strict")]
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "floatValue13",
                                        ),
                                    );
                                }
                                m_floatValue13 = _serde::__private::Some(
                                    match __A::next_value::<f32>(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            __Field::m_floatVariable14 => {
                                #[cfg(
                                    any(feature = "strict", feature = "ignore_duplicates")
                                )]
                                if _serde::__private::Option::is_some(&m_floatVariable14) {
                                    #[cfg(feature = "ignore_duplicates")]
                                    {
                                        __A::skip_value(&mut __map)?;
                                        continue;
                                    }
                                    #[cfg(feature = "strict")]
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "floatVariable14",
                                        ),
                                    );
                                }
                                m_floatVariable14 = _serde::__private::Some(
                                    match __A::next_value::<f32>(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            __Field::m_floatValue14 => {
                                #[cfg(
                                    any(feature = "strict", feature = "ignore_duplicates")
                                )]
                                if _serde::__private::Option::is_some(&m_floatValue14) {
                                    #[cfg(feature = "ignore_duplicates")]
                                    {
                                        __A::skip_value(&mut __map)?;
                                        continue;
                                    }
                                    #[cfg(feature = "strict")]
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "floatValue14",
                                        ),
                                    );
                                }
                                m_floatValue14 = _serde::__private::Some(
                                    match __A::next_value::<f32>(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            __Field::m_floatVariable15 => {
                                #[cfg(
                                    any(feature = "strict", feature = "ignore_duplicates")
                                )]
                                if _serde::__private::Option::is_some(&m_floatVariable15) {
                                    #[cfg(feature = "ignore_duplicates")]
                                    {
                                        __A::skip_value(&mut __map)?;
                                        continue;
                                    }
                                    #[cfg(feature = "strict")]
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "floatVariable15",
                                        ),
                                    );
                                }
                                m_floatVariable15 = _serde::__private::Some(
                                    match __A::next_value::<f32>(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            __Field::m_floatValue15 => {
                                #[cfg(
                                    any(feature = "strict", feature = "ignore_duplicates")
                                )]
                                if _serde::__private::Option::is_some(&m_floatValue15) {
                                    #[cfg(feature = "ignore_duplicates")]
                                    {
                                        __A::skip_value(&mut __map)?;
                                        continue;
                                    }
                                    #[cfg(feature = "strict")]
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "floatValue15",
                                        ),
                                    );
                                }
                                m_floatValue15 = _serde::__private::Some(
                                    match __A::next_value::<f32>(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            __Field::m_floatVariable16 => {
                                #[cfg(
                                    any(feature = "strict", feature = "ignore_duplicates")
                                )]
                                if _serde::__private::Option::is_some(&m_floatVariable16) {
                                    #[cfg(feature = "ignore_duplicates")]
                                    {
                                        __A::skip_value(&mut __map)?;
                                        continue;
                                    }
                                    #[cfg(feature = "strict")]
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "floatVariable16",
                                        ),
                                    );
                                }
                                m_floatVariable16 = _serde::__private::Some(
                                    match __A::next_value::<f32>(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            __Field::m_floatValue16 => {
                                #[cfg(
                                    any(feature = "strict", feature = "ignore_duplicates")
                                )]
                                if _serde::__private::Option::is_some(&m_floatValue16) {
                                    #[cfg(feature = "ignore_duplicates")]
                                    {
                                        __A::skip_value(&mut __map)?;
                                        continue;
                                    }
                                    #[cfg(feature = "strict")]
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "floatValue16",
                                        ),
                                    );
                                }
                                m_floatValue16 = _serde::__private::Some(
                                    match __A::next_value::<f32>(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            __Field::m_floatVariable17 => {
                                #[cfg(
                                    any(feature = "strict", feature = "ignore_duplicates")
                                )]
                                if _serde::__private::Option::is_some(&m_floatVariable17) {
                                    #[cfg(feature = "ignore_duplicates")]
                                    {
                                        __A::skip_value(&mut __map)?;
                                        continue;
                                    }
                                    #[cfg(feature = "strict")]
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "floatVariable17",
                                        ),
                                    );
                                }
                                m_floatVariable17 = _serde::__private::Some(
                                    match __A::next_value::<f32>(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            __Field::m_floatValue17 => {
                                #[cfg(
                                    any(feature = "strict", feature = "ignore_duplicates")
                                )]
                                if _serde::__private::Option::is_some(&m_floatValue17) {
                                    #[cfg(feature = "ignore_duplicates")]
                                    {
                                        __A::skip_value(&mut __map)?;
                                        continue;
                                    }
                                    #[cfg(feature = "strict")]
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "floatValue17",
                                        ),
                                    );
                                }
                                m_floatValue17 = _serde::__private::Some(
                                    match __A::next_value::<f32>(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            __Field::m_floatVariable18 => {
                                #[cfg(
                                    any(feature = "strict", feature = "ignore_duplicates")
                                )]
                                if _serde::__private::Option::is_some(&m_floatVariable18) {
                                    #[cfg(feature = "ignore_duplicates")]
                                    {
                                        __A::skip_value(&mut __map)?;
                                        continue;
                                    }
                                    #[cfg(feature = "strict")]
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "floatVariable18",
                                        ),
                                    );
                                }
                                m_floatVariable18 = _serde::__private::Some(
                                    match __A::next_value::<f32>(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            __Field::m_floatValue18 => {
                                #[cfg(
                                    any(feature = "strict", feature = "ignore_duplicates")
                                )]
                                if _serde::__private::Option::is_some(&m_floatValue18) {
                                    #[cfg(feature = "ignore_duplicates")]
                                    {
                                        __A::skip_value(&mut __map)?;
                                        continue;
                                    }
                                    #[cfg(feature = "strict")]
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "floatValue18",
                                        ),
                                    );
                                }
                                m_floatValue18 = _serde::__private::Some(
                                    match __A::next_value::<f32>(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            __Field::m_floatVariable19 => {
                                #[cfg(
                                    any(feature = "strict", feature = "ignore_duplicates")
                                )]
                                if _serde::__private::Option::is_some(&m_floatVariable19) {
                                    #[cfg(feature = "ignore_duplicates")]
                                    {
                                        __A::skip_value(&mut __map)?;
                                        continue;
                                    }
                                    #[cfg(feature = "strict")]
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "floatVariable19",
                                        ),
                                    );
                                }
                                m_floatVariable19 = _serde::__private::Some(
                                    match __A::next_value::<f32>(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            __Field::m_floatValue19 => {
                                #[cfg(
                                    any(feature = "strict", feature = "ignore_duplicates")
                                )]
                                if _serde::__private::Option::is_some(&m_floatValue19) {
                                    #[cfg(feature = "ignore_duplicates")]
                                    {
                                        __A::skip_value(&mut __map)?;
                                        continue;
                                    }
                                    #[cfg(feature = "strict")]
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "floatValue19",
                                        ),
                                    );
                                }
                                m_floatValue19 = _serde::__private::Some(
                                    match __A::next_value::<f32>(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            __Field::m_floatVariable20 => {
                                #[cfg(
                                    any(feature = "strict", feature = "ignore_duplicates")
                                )]
                                if _serde::__private::Option::is_some(&m_floatVariable20) {
                                    #[cfg(feature = "ignore_duplicates")]
                                    {
                                        __A::skip_value(&mut __map)?;
                                        continue;
                                    }
                                    #[cfg(feature = "strict")]
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "floatVariable20",
                                        ),
                                    );
                                }
                                m_floatVariable20 = _serde::__private::Some(
                                    match __A::next_value::<f32>(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            __Field::m_floatValue20 => {
                                #[cfg(
                                    any(feature = "strict", feature = "ignore_duplicates")
                                )]
                                if _serde::__private::Option::is_some(&m_floatValue20) {
                                    #[cfg(feature = "ignore_duplicates")]
                                    {
                                        __A::skip_value(&mut __map)?;
                                        continue;
                                    }
                                    #[cfg(feature = "strict")]
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "floatValue20",
                                        ),
                                    );
                                }
                                m_floatValue20 = _serde::__private::Some(
                                    match __A::next_value::<f32>(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            __Field::m_intVariable1 => {
                                #[cfg(
                                    any(feature = "strict", feature = "ignore_duplicates")
                                )]
                                if _serde::__private::Option::is_some(&m_intVariable1) {
                                    #[cfg(feature = "ignore_duplicates")]
                                    {
                                        __A::skip_value(&mut __map)?;
                                        continue;
                                    }
                                    #[cfg(feature = "strict")]
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "intVariable1",
                                        ),
                                    );
                                }
                                m_intVariable1 = _serde::__private::Some(
                                    match __A::next_value::<I32<'de>>(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            __Field::m_intValue1 => {
                                #[cfg(
                                    any(feature = "strict", feature = "ignore_duplicates")
                                )]
                                if _serde::__private::Option::is_some(&m_intValue1) {
                                    #[cfg(feature = "ignore_duplicates")]
                                    {
                                        __A::skip_value(&mut __map)?;
                                        continue;
                                    }
                                    #[cfg(feature = "strict")]
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "intValue1",
                                        ),
                                    );
                                }
                                m_intValue1 = _serde::__private::Some(
                                    match __A::next_value::<I32<'de>>(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            __Field::m_intVariable2 => {
                                #[cfg(
                                    any(feature = "strict", feature = "ignore_duplicates")
                                )]
                                if _serde::__private::Option::is_some(&m_intVariable2) {
                                    #[cfg(feature = "ignore_duplicates")]
                                    {
                                        __A::skip_value(&mut __map)?;
                                        continue;
                                    }
                                    #[cfg(feature = "strict")]
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "intVariable2",
                                        ),
                                    );
                                }
                                m_intVariable2 = _serde::__private::Some(
                                    match __A::next_value::<I32<'de>>(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            __Field::m_intValue2 => {
                                #[cfg(
                                    any(feature = "strict", feature = "ignore_duplicates")
                                )]
                                if _serde::__private::Option::is_some(&m_intValue2) {
                                    #[cfg(feature = "ignore_duplicates")]
                                    {
                                        __A::skip_value(&mut __map)?;
                                        continue;
                                    }
                                    #[cfg(feature = "strict")]
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "intValue2",
                                        ),
                                    );
                                }
                                m_intValue2 = _serde::__private::Some(
                                    match __A::next_value::<I32<'de>>(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            __Field::m_intVariable3 => {
                                #[cfg(
                                    any(feature = "strict", feature = "ignore_duplicates")
                                )]
                                if _serde::__private::Option::is_some(&m_intVariable3) {
                                    #[cfg(feature = "ignore_duplicates")]
                                    {
                                        __A::skip_value(&mut __map)?;
                                        continue;
                                    }
                                    #[cfg(feature = "strict")]
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "intVariable3",
                                        ),
                                    );
                                }
                                m_intVariable3 = _serde::__private::Some(
                                    match __A::next_value::<I32<'de>>(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            __Field::m_intValue3 => {
                                #[cfg(
                                    any(feature = "strict", feature = "ignore_duplicates")
                                )]
                                if _serde::__private::Option::is_some(&m_intValue3) {
                                    #[cfg(feature = "ignore_duplicates")]
                                    {
                                        __A::skip_value(&mut __map)?;
                                        continue;
                                    }
                                    #[cfg(feature = "strict")]
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "intValue3",
                                        ),
                                    );
                                }
                                m_intValue3 = _serde::__private::Some(
                                    match __A::next_value::<I32<'de>>(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            __Field::m_intVariable4 => {
                                #[cfg(
                                    any(feature = "strict", feature = "ignore_duplicates")
                                )]
                                if _serde::__private::Option::is_some(&m_intVariable4) {
                                    #[cfg(feature = "ignore_duplicates")]
                                    {
                                        __A::skip_value(&mut __map)?;
                                        continue;
                                    }
                                    #[cfg(feature = "strict")]
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "intVariable4",
                                        ),
                                    );
                                }
                                m_intVariable4 = _serde::__private::Some(
                                    match __A::next_value::<I32<'de>>(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            __Field::m_intValue4 => {
                                #[cfg(
                                    any(feature = "strict", feature = "ignore_duplicates")
                                )]
                                if _serde::__private::Option::is_some(&m_intValue4) {
                                    #[cfg(feature = "ignore_duplicates")]
                                    {
                                        __A::skip_value(&mut __map)?;
                                        continue;
                                    }
                                    #[cfg(feature = "strict")]
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "intValue4",
                                        ),
                                    );
                                }
                                m_intValue4 = _serde::__private::Some(
                                    match __A::next_value::<I32<'de>>(&mut __map) {
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
                    let m_floatVariable1 = match m_floatVariable1 {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            #[cfg(feature = "strict")]
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "floatVariable1",
                                ),
                            );
                            #[cfg(not(feature = "strict"))] Default::default()
                        }
                    };
                    let m_floatValue1 = match m_floatValue1 {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            #[cfg(feature = "strict")]
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "floatValue1",
                                ),
                            );
                            #[cfg(not(feature = "strict"))] Default::default()
                        }
                    };
                    let m_floatVariable2 = match m_floatVariable2 {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            #[cfg(feature = "strict")]
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "floatVariable2",
                                ),
                            );
                            #[cfg(not(feature = "strict"))] Default::default()
                        }
                    };
                    let m_floatValue2 = match m_floatValue2 {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            #[cfg(feature = "strict")]
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "floatValue2",
                                ),
                            );
                            #[cfg(not(feature = "strict"))] Default::default()
                        }
                    };
                    let m_floatVariable3 = match m_floatVariable3 {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            #[cfg(feature = "strict")]
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "floatVariable3",
                                ),
                            );
                            #[cfg(not(feature = "strict"))] Default::default()
                        }
                    };
                    let m_floatValue3 = match m_floatValue3 {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            #[cfg(feature = "strict")]
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "floatValue3",
                                ),
                            );
                            #[cfg(not(feature = "strict"))] Default::default()
                        }
                    };
                    let m_floatVariable4 = match m_floatVariable4 {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            #[cfg(feature = "strict")]
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "floatVariable4",
                                ),
                            );
                            #[cfg(not(feature = "strict"))] Default::default()
                        }
                    };
                    let m_floatValue4 = match m_floatValue4 {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            #[cfg(feature = "strict")]
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "floatValue4",
                                ),
                            );
                            #[cfg(not(feature = "strict"))] Default::default()
                        }
                    };
                    let m_floatVariable5 = match m_floatVariable5 {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            #[cfg(feature = "strict")]
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "floatVariable5",
                                ),
                            );
                            #[cfg(not(feature = "strict"))] Default::default()
                        }
                    };
                    let m_floatValue5 = match m_floatValue5 {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            #[cfg(feature = "strict")]
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "floatValue5",
                                ),
                            );
                            #[cfg(not(feature = "strict"))] Default::default()
                        }
                    };
                    let m_floatVariable6 = match m_floatVariable6 {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            #[cfg(feature = "strict")]
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "floatVariable6",
                                ),
                            );
                            #[cfg(not(feature = "strict"))] Default::default()
                        }
                    };
                    let m_floatValue6 = match m_floatValue6 {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            #[cfg(feature = "strict")]
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "floatValue6",
                                ),
                            );
                            #[cfg(not(feature = "strict"))] Default::default()
                        }
                    };
                    let m_floatVariable7 = match m_floatVariable7 {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            #[cfg(feature = "strict")]
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "floatVariable7",
                                ),
                            );
                            #[cfg(not(feature = "strict"))] Default::default()
                        }
                    };
                    let m_floatValue7 = match m_floatValue7 {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            #[cfg(feature = "strict")]
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "floatValue7",
                                ),
                            );
                            #[cfg(not(feature = "strict"))] Default::default()
                        }
                    };
                    let m_floatVariable8 = match m_floatVariable8 {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            #[cfg(feature = "strict")]
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "floatVariable8",
                                ),
                            );
                            #[cfg(not(feature = "strict"))] Default::default()
                        }
                    };
                    let m_floatValue8 = match m_floatValue8 {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            #[cfg(feature = "strict")]
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "floatValue8",
                                ),
                            );
                            #[cfg(not(feature = "strict"))] Default::default()
                        }
                    };
                    let m_floatVariable9 = match m_floatVariable9 {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            #[cfg(feature = "strict")]
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "floatVariable9",
                                ),
                            );
                            #[cfg(not(feature = "strict"))] Default::default()
                        }
                    };
                    let m_floatValue9 = match m_floatValue9 {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            #[cfg(feature = "strict")]
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "floatValue9",
                                ),
                            );
                            #[cfg(not(feature = "strict"))] Default::default()
                        }
                    };
                    let m_floatVariable10 = match m_floatVariable10 {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            #[cfg(feature = "strict")]
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "floatVariable10",
                                ),
                            );
                            #[cfg(not(feature = "strict"))] Default::default()
                        }
                    };
                    let m_floatValue10 = match m_floatValue10 {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            #[cfg(feature = "strict")]
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "floatValue10",
                                ),
                            );
                            #[cfg(not(feature = "strict"))] Default::default()
                        }
                    };
                    let m_floatVariable11 = match m_floatVariable11 {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            #[cfg(feature = "strict")]
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "floatVariable11",
                                ),
                            );
                            #[cfg(not(feature = "strict"))] Default::default()
                        }
                    };
                    let m_floatValue11 = match m_floatValue11 {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            #[cfg(feature = "strict")]
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "floatValue11",
                                ),
                            );
                            #[cfg(not(feature = "strict"))] Default::default()
                        }
                    };
                    let m_floatVariable12 = match m_floatVariable12 {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            #[cfg(feature = "strict")]
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "floatVariable12",
                                ),
                            );
                            #[cfg(not(feature = "strict"))] Default::default()
                        }
                    };
                    let m_floatValue12 = match m_floatValue12 {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            #[cfg(feature = "strict")]
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "floatValue12",
                                ),
                            );
                            #[cfg(not(feature = "strict"))] Default::default()
                        }
                    };
                    let m_floatVariable13 = match m_floatVariable13 {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            #[cfg(feature = "strict")]
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "floatVariable13",
                                ),
                            );
                            #[cfg(not(feature = "strict"))] Default::default()
                        }
                    };
                    let m_floatValue13 = match m_floatValue13 {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            #[cfg(feature = "strict")]
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "floatValue13",
                                ),
                            );
                            #[cfg(not(feature = "strict"))] Default::default()
                        }
                    };
                    let m_floatVariable14 = match m_floatVariable14 {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            #[cfg(feature = "strict")]
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "floatVariable14",
                                ),
                            );
                            #[cfg(not(feature = "strict"))] Default::default()
                        }
                    };
                    let m_floatValue14 = match m_floatValue14 {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            #[cfg(feature = "strict")]
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "floatValue14",
                                ),
                            );
                            #[cfg(not(feature = "strict"))] Default::default()
                        }
                    };
                    let m_floatVariable15 = match m_floatVariable15 {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            #[cfg(feature = "strict")]
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "floatVariable15",
                                ),
                            );
                            #[cfg(not(feature = "strict"))] Default::default()
                        }
                    };
                    let m_floatValue15 = match m_floatValue15 {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            #[cfg(feature = "strict")]
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "floatValue15",
                                ),
                            );
                            #[cfg(not(feature = "strict"))] Default::default()
                        }
                    };
                    let m_floatVariable16 = match m_floatVariable16 {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            #[cfg(feature = "strict")]
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "floatVariable16",
                                ),
                            );
                            #[cfg(not(feature = "strict"))] Default::default()
                        }
                    };
                    let m_floatValue16 = match m_floatValue16 {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            #[cfg(feature = "strict")]
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "floatValue16",
                                ),
                            );
                            #[cfg(not(feature = "strict"))] Default::default()
                        }
                    };
                    let m_floatVariable17 = match m_floatVariable17 {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            #[cfg(feature = "strict")]
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "floatVariable17",
                                ),
                            );
                            #[cfg(not(feature = "strict"))] Default::default()
                        }
                    };
                    let m_floatValue17 = match m_floatValue17 {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            #[cfg(feature = "strict")]
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "floatValue17",
                                ),
                            );
                            #[cfg(not(feature = "strict"))] Default::default()
                        }
                    };
                    let m_floatVariable18 = match m_floatVariable18 {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            #[cfg(feature = "strict")]
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "floatVariable18",
                                ),
                            );
                            #[cfg(not(feature = "strict"))] Default::default()
                        }
                    };
                    let m_floatValue18 = match m_floatValue18 {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            #[cfg(feature = "strict")]
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "floatValue18",
                                ),
                            );
                            #[cfg(not(feature = "strict"))] Default::default()
                        }
                    };
                    let m_floatVariable19 = match m_floatVariable19 {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            #[cfg(feature = "strict")]
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "floatVariable19",
                                ),
                            );
                            #[cfg(not(feature = "strict"))] Default::default()
                        }
                    };
                    let m_floatValue19 = match m_floatValue19 {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            #[cfg(feature = "strict")]
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "floatValue19",
                                ),
                            );
                            #[cfg(not(feature = "strict"))] Default::default()
                        }
                    };
                    let m_floatVariable20 = match m_floatVariable20 {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            #[cfg(feature = "strict")]
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "floatVariable20",
                                ),
                            );
                            #[cfg(not(feature = "strict"))] Default::default()
                        }
                    };
                    let m_floatValue20 = match m_floatValue20 {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            #[cfg(feature = "strict")]
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "floatValue20",
                                ),
                            );
                            #[cfg(not(feature = "strict"))] Default::default()
                        }
                    };
                    let m_intVariable1 = match m_intVariable1 {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            #[cfg(feature = "strict")]
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "intVariable1",
                                ),
                            );
                            #[cfg(not(feature = "strict"))] Default::default()
                        }
                    };
                    let m_intValue1 = match m_intValue1 {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            #[cfg(feature = "strict")]
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "intValue1",
                                ),
                            );
                            #[cfg(not(feature = "strict"))] Default::default()
                        }
                    };
                    let m_intVariable2 = match m_intVariable2 {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            #[cfg(feature = "strict")]
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "intVariable2",
                                ),
                            );
                            #[cfg(not(feature = "strict"))] Default::default()
                        }
                    };
                    let m_intValue2 = match m_intValue2 {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            #[cfg(feature = "strict")]
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "intValue2",
                                ),
                            );
                            #[cfg(not(feature = "strict"))] Default::default()
                        }
                    };
                    let m_intVariable3 = match m_intVariable3 {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            #[cfg(feature = "strict")]
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "intVariable3",
                                ),
                            );
                            #[cfg(not(feature = "strict"))] Default::default()
                        }
                    };
                    let m_intValue3 = match m_intValue3 {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            #[cfg(feature = "strict")]
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "intValue3",
                                ),
                            );
                            #[cfg(not(feature = "strict"))] Default::default()
                        }
                    };
                    let m_intVariable4 = match m_intVariable4 {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            #[cfg(feature = "strict")]
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "intVariable4",
                                ),
                            );
                            #[cfg(not(feature = "strict"))] Default::default()
                        }
                    };
                    let m_intValue4 = match m_intValue4 {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            #[cfg(feature = "strict")]
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "intValue4",
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
                    let parent = hkbModifier {
                        __ptr: __ptr.clone(),
                        parent,
                        m_enable,
                        ..Default::default()
                    };
                    let __ptr = __A::class_ptr(&mut __map);
                    _serde::__private::Ok(BSAssignVariablesModifier {
                        __ptr: __ptr.clone(),
                        parent,
                        m_floatVariable1,
                        m_floatValue1,
                        m_floatVariable2,
                        m_floatValue2,
                        m_floatVariable3,
                        m_floatValue3,
                        m_floatVariable4,
                        m_floatValue4,
                        m_floatVariable5,
                        m_floatValue5,
                        m_floatVariable6,
                        m_floatValue6,
                        m_floatVariable7,
                        m_floatValue7,
                        m_floatVariable8,
                        m_floatValue8,
                        m_floatVariable9,
                        m_floatValue9,
                        m_floatVariable10,
                        m_floatValue10,
                        m_floatVariable11,
                        m_floatValue11,
                        m_floatVariable12,
                        m_floatValue12,
                        m_floatVariable13,
                        m_floatValue13,
                        m_floatVariable14,
                        m_floatValue14,
                        m_floatVariable15,
                        m_floatValue15,
                        m_floatVariable16,
                        m_floatValue16,
                        m_floatVariable17,
                        m_floatValue17,
                        m_floatVariable18,
                        m_floatValue18,
                        m_floatVariable19,
                        m_floatValue19,
                        m_floatVariable20,
                        m_floatValue20,
                        m_intVariable1,
                        m_intValue1,
                        m_intVariable2,
                        m_intValue2,
                        m_intVariable3,
                        m_intValue3,
                        m_intVariable4,
                        m_intValue4,
                    })
                }
            }
            const FIELDS: &[&str] = &[
                "floatVariable1",
                "floatValue1",
                "floatVariable2",
                "floatValue2",
                "floatVariable3",
                "floatValue3",
                "floatVariable4",
                "floatValue4",
                "floatVariable5",
                "floatValue5",
                "floatVariable6",
                "floatValue6",
                "floatVariable7",
                "floatValue7",
                "floatVariable8",
                "floatValue8",
                "floatVariable9",
                "floatValue9",
                "floatVariable10",
                "floatValue10",
                "floatVariable11",
                "floatValue11",
                "floatVariable12",
                "floatValue12",
                "floatVariable13",
                "floatValue13",
                "floatVariable14",
                "floatValue14",
                "floatVariable15",
                "floatValue15",
                "floatVariable16",
                "floatValue16",
                "floatVariable17",
                "floatValue17",
                "floatVariable18",
                "floatValue18",
                "floatVariable19",
                "floatValue19",
                "floatVariable20",
                "floatValue20",
                "intVariable1",
                "intValue1",
                "intVariable2",
                "intValue2",
                "intVariable3",
                "intValue3",
                "intVariable4",
                "intValue4",
            ];
            _serde::Deserializer::deserialize_struct(
                deserializer,
                "BSAssignVariablesModifier",
                FIELDS,
                __BSAssignVariablesModifierVisitor {
                    marker: _serde::__private::PhantomData::<BSAssignVariablesModifier>,
                    lifetime: _serde::__private::PhantomData,
                },
            )
        }
    }
};
