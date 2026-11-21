use super::class_requires::*;
use super::*;
/// # C++ Info
/// - name: `hkbCharacter`
/// - version: `4`
/// - signature: `0x393e4e8d`
/// - size: `  0`(x86)/`216`(x86_64)
/// -  vtable: `true`
#[allow(non_upper_case_globals, non_snake_case)]
#[cfg_attr(feature = "json_schema", derive(schemars::JsonSchema))]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(educe::Educe)]
#[educe(Debug, Clone, Default, PartialEq)]
pub struct hkbCharacter<'a> {
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
    pub parent: hkReferencedObject<'a>,
    /// # C++ Info
    /// - name: `nearbyCharacters`(ctype: `hkArray<hkbCharacter*>`)
    /// - offset: `  0`(x86)/` 16`(x86_64)
    /// - type_size: `  0`(x86)/` 16`(x86_64)
    #[cfg_attr(feature = "serde_default", serde(default))]
    #[cfg_attr(feature = "serde", serde(borrow))]
    #[cfg_attr(feature = "json_schema", schemars(rename = "nearbyCharacters"))]
    #[cfg_attr(feature = "serde", serde(rename = "nearbyCharacters"))]
    pub m_nearbyCharacters: Vec<Pointer<'a>>,
    /// # C++ Info
    /// - name: `userData`(ctype: `hkUlong`)
    /// - offset: `  0`(x86)/` 32`(x86_64)
    /// - type_size: `  0`(x86)/`  8`(x86_64)
    #[cfg_attr(feature = "serde_default", serde(default))]
    #[cfg_attr(feature = "json_schema", schemars(rename = "userData"))]
    #[cfg_attr(feature = "serde", serde(rename = "userData"))]
    pub m_userData: Ulong,
    /// # C++ Info
    /// - name: `currentLod`(ctype: `hkInt16`)
    /// - offset: `  0`(x86)/` 40`(x86_64)
    /// - type_size: `  0`(x86)/`  2`(x86_64)
    #[cfg_attr(feature = "serde_default", serde(default))]
    #[cfg_attr(feature = "serde", serde(borrow))]
    #[cfg_attr(feature = "json_schema", schemars(rename = "currentLod"))]
    #[cfg_attr(feature = "serde", serde(rename = "currentLod"))]
    pub m_currentLod: I16<'a>,
    /// # C++ Info
    /// - name: `numTracksInLod`(ctype: `hkInt16`)
    /// - offset: `  0`(x86)/` 42`(x86_64)
    /// - type_size: `  0`(x86)/`  2`(x86_64)
    /// - flags: `SERIALIZE_IGNORED`
    #[cfg_attr(feature = "serde", serde(default))]
    #[cfg_attr(feature = "serde", serde(borrow))]
    #[cfg_attr(feature = "json_schema", schemars(rename = "numTracksInLod"))]
    #[cfg_attr(feature = "serde", serde(rename = "numTracksInLod"))]
    pub m_numTracksInLod: I16<'a>,
    /// # C++ Info
    /// - name: `generatorOutput`(ctype: `void*`)
    /// - offset: `  0`(x86)/` 48`(x86_64)
    /// - type_size: `  0`(x86)/`  8`(x86_64)
    /// - flags: `SERIALIZE_IGNORED`
    #[cfg_attr(feature = "serde", serde(default))]
    #[cfg_attr(feature = "serde", serde(borrow))]
    #[cfg_attr(feature = "json_schema", schemars(rename = "generatorOutput"))]
    #[cfg_attr(feature = "serde", serde(rename = "generatorOutput"))]
    pub m_generatorOutput: Pointer<'a>,
    /// # C++ Info
    /// - name: `name`(ctype: `hkStringPtr`)
    /// - offset: `  0`(x86)/` 56`(x86_64)
    /// - type_size: `  0`(x86)/`  8`(x86_64)
    #[cfg_attr(feature = "serde_default", serde(default))]
    #[cfg_attr(feature = "serde", serde(borrow))]
    #[cfg_attr(feature = "json_schema", schemars(rename = "name"))]
    #[cfg_attr(feature = "serde", serde(rename = "name"))]
    pub m_name: StringPtr<'a>,
    /// # C++ Info
    /// - name: `ragdollDriver`(ctype: `void*`)
    /// - offset: `  0`(x86)/` 64`(x86_64)
    /// - type_size: `  0`(x86)/`  8`(x86_64)
    /// - flags: `SERIALIZE_IGNORED`
    #[cfg_attr(feature = "serde", serde(default))]
    #[cfg_attr(feature = "serde", serde(borrow))]
    #[cfg_attr(feature = "json_schema", schemars(rename = "ragdollDriver"))]
    #[cfg_attr(feature = "serde", serde(rename = "ragdollDriver"))]
    pub m_ragdollDriver: Pointer<'a>,
    /// # C++ Info
    /// - name: `ragdollInterface`(ctype: `void*`)
    /// - offset: `  0`(x86)/` 72`(x86_64)
    /// - type_size: `  0`(x86)/`  8`(x86_64)
    /// - flags: `SERIALIZE_IGNORED`
    #[cfg_attr(feature = "serde", serde(default))]
    #[cfg_attr(feature = "serde", serde(borrow))]
    #[cfg_attr(feature = "json_schema", schemars(rename = "ragdollInterface"))]
    #[cfg_attr(feature = "serde", serde(rename = "ragdollInterface"))]
    pub m_ragdollInterface: Pointer<'a>,
    /// # C++ Info
    /// - name: `characterControllerDriver`(ctype: `void*`)
    /// - offset: `  0`(x86)/` 80`(x86_64)
    /// - type_size: `  0`(x86)/`  8`(x86_64)
    /// - flags: `SERIALIZE_IGNORED`
    #[cfg_attr(feature = "serde", serde(default))]
    #[cfg_attr(feature = "serde", serde(borrow))]
    #[cfg_attr(feature = "json_schema", schemars(rename = "characterControllerDriver"))]
    #[cfg_attr(feature = "serde", serde(rename = "characterControllerDriver"))]
    pub m_characterControllerDriver: Pointer<'a>,
    /// # C++ Info
    /// - name: `footIkDriver`(ctype: `void*`)
    /// - offset: `  0`(x86)/` 88`(x86_64)
    /// - type_size: `  0`(x86)/`  8`(x86_64)
    /// - flags: `SERIALIZE_IGNORED`
    #[cfg_attr(feature = "serde", serde(default))]
    #[cfg_attr(feature = "serde", serde(borrow))]
    #[cfg_attr(feature = "json_schema", schemars(rename = "footIkDriver"))]
    #[cfg_attr(feature = "serde", serde(rename = "footIkDriver"))]
    pub m_footIkDriver: Pointer<'a>,
    /// # C++ Info
    /// - name: `handIkDriver`(ctype: `void*`)
    /// - offset: `  0`(x86)/` 96`(x86_64)
    /// - type_size: `  0`(x86)/`  8`(x86_64)
    /// - flags: `SERIALIZE_IGNORED`
    #[cfg_attr(feature = "serde", serde(default))]
    #[cfg_attr(feature = "serde", serde(borrow))]
    #[cfg_attr(feature = "json_schema", schemars(rename = "handIkDriver"))]
    #[cfg_attr(feature = "serde", serde(rename = "handIkDriver"))]
    pub m_handIkDriver: Pointer<'a>,
    /// # C++ Info
    /// - name: `dockingDriver`(ctype: `void*`)
    /// - offset: `  0`(x86)/`104`(x86_64)
    /// - type_size: `  0`(x86)/`  8`(x86_64)
    /// - flags: `SERIALIZE_IGNORED`
    #[cfg_attr(feature = "serde", serde(default))]
    #[cfg_attr(feature = "serde", serde(borrow))]
    #[cfg_attr(feature = "json_schema", schemars(rename = "dockingDriver"))]
    #[cfg_attr(feature = "serde", serde(rename = "dockingDriver"))]
    pub m_dockingDriver: Pointer<'a>,
    /// # C++ Info
    /// - name: `aiControlDriver`(ctype: `void*`)
    /// - offset: `  0`(x86)/`112`(x86_64)
    /// - type_size: `  0`(x86)/`  8`(x86_64)
    /// - flags: `SERIALIZE_IGNORED`
    #[cfg_attr(feature = "serde", serde(default))]
    #[cfg_attr(feature = "serde", serde(borrow))]
    #[cfg_attr(feature = "json_schema", schemars(rename = "aiControlDriver"))]
    #[cfg_attr(feature = "serde", serde(rename = "aiControlDriver"))]
    pub m_aiControlDriver: Pointer<'a>,
    /// # C++ Info
    /// - name: `setup`(ctype: `struct hkbCharacterSetup*`)
    /// - offset: `  0`(x86)/`120`(x86_64)
    /// - type_size: `  0`(x86)/`  8`(x86_64)
    #[cfg_attr(feature = "serde_default", serde(default))]
    #[cfg_attr(feature = "serde", serde(borrow))]
    #[cfg_attr(feature = "json_schema", schemars(rename = "setup"))]
    #[cfg_attr(feature = "serde", serde(rename = "setup"))]
    pub m_setup: Pointer<'a>,
    /// # C++ Info
    /// - name: `behaviorGraph`(ctype: `struct hkbBehaviorGraph*`)
    /// - offset: `  0`(x86)/`128`(x86_64)
    /// - type_size: `  0`(x86)/`  8`(x86_64)
    #[cfg_attr(feature = "serde_default", serde(default))]
    #[cfg_attr(feature = "serde", serde(borrow))]
    #[cfg_attr(feature = "json_schema", schemars(rename = "behaviorGraph"))]
    #[cfg_attr(feature = "serde", serde(rename = "behaviorGraph"))]
    pub m_behaviorGraph: Pointer<'a>,
    /// # C++ Info
    /// - name: `projectData`(ctype: `struct hkbProjectData*`)
    /// - offset: `  0`(x86)/`136`(x86_64)
    /// - type_size: `  0`(x86)/`  8`(x86_64)
    #[cfg_attr(feature = "serde_default", serde(default))]
    #[cfg_attr(feature = "serde", serde(borrow))]
    #[cfg_attr(feature = "json_schema", schemars(rename = "projectData"))]
    #[cfg_attr(feature = "serde", serde(rename = "projectData"))]
    pub m_projectData: Pointer<'a>,
    /// # C++ Info
    /// - name: `animationBindingSet`(ctype: `void*`)
    /// - offset: `  0`(x86)/`144`(x86_64)
    /// - type_size: `  0`(x86)/`  8`(x86_64)
    /// - flags: `SERIALIZE_IGNORED`
    #[cfg_attr(feature = "serde", serde(default))]
    #[cfg_attr(feature = "serde", serde(borrow))]
    #[cfg_attr(feature = "json_schema", schemars(rename = "animationBindingSet"))]
    #[cfg_attr(feature = "serde", serde(rename = "animationBindingSet"))]
    pub m_animationBindingSet: Pointer<'a>,
    /// # C++ Info
    /// - name: `spatialQueryInterface`(ctype: `void*`)
    /// - offset: `  0`(x86)/`152`(x86_64)
    /// - type_size: `  0`(x86)/`  8`(x86_64)
    /// - flags: `SERIALIZE_IGNORED`
    #[cfg_attr(feature = "serde", serde(default))]
    #[cfg_attr(feature = "serde", serde(borrow))]
    #[cfg_attr(feature = "json_schema", schemars(rename = "spatialQueryInterface"))]
    #[cfg_attr(feature = "serde", serde(rename = "spatialQueryInterface"))]
    pub m_spatialQueryInterface: Pointer<'a>,
    /// # C++ Info
    /// - name: `world`(ctype: `void*`)
    /// - offset: `  0`(x86)/`160`(x86_64)
    /// - type_size: `  0`(x86)/`  8`(x86_64)
    /// - flags: `NOT_OWNED|SERIALIZE_IGNORED`
    #[cfg_attr(feature = "serde", serde(default))]
    #[cfg_attr(feature = "serde", serde(borrow))]
    #[cfg_attr(feature = "json_schema", schemars(rename = "world"))]
    #[cfg_attr(feature = "serde", serde(rename = "world"))]
    pub m_world: Pointer<'a>,
    /// # C++ Info
    /// - name: `boneAttachments`(ctype: `hkArray<void*>`)
    /// - offset: `  0`(x86)/`168`(x86_64)
    /// - type_size: `  0`(x86)/` 16`(x86_64)
    /// - flags: `SERIALIZE_IGNORED`
    #[cfg_attr(feature = "serde", serde(default))]
    #[cfg_attr(feature = "serde", serde(borrow))]
    #[cfg_attr(feature = "json_schema", schemars(rename = "boneAttachments"))]
    #[cfg_attr(feature = "serde", serde(rename = "boneAttachments"))]
    pub m_boneAttachments: Vec<Pointer<'a>>,
    /// # C++ Info
    /// - name: `eventQueue`(ctype: `void*`)
    /// - offset: `  0`(x86)/`184`(x86_64)
    /// - type_size: `  0`(x86)/`  8`(x86_64)
    /// - flags: `SERIALIZE_IGNORED`
    #[cfg_attr(feature = "serde", serde(default))]
    #[cfg_attr(feature = "serde", serde(borrow))]
    #[cfg_attr(feature = "json_schema", schemars(rename = "eventQueue"))]
    #[cfg_attr(feature = "serde", serde(rename = "eventQueue"))]
    pub m_eventQueue: Pointer<'a>,
    /// # C++ Info
    /// - name: `characterLuaState`(ctype: `void*`)
    /// - offset: `  0`(x86)/`192`(x86_64)
    /// - type_size: `  0`(x86)/`  8`(x86_64)
    /// - flags: `SERIALIZE_IGNORED`
    #[cfg_attr(feature = "serde", serde(default))]
    #[cfg_attr(feature = "serde", serde(borrow))]
    #[cfg_attr(feature = "json_schema", schemars(rename = "characterLuaState"))]
    #[cfg_attr(feature = "serde", serde(rename = "characterLuaState"))]
    pub m_characterLuaState: Pointer<'a>,
    /// # C++ Info
    /// - name: `assetManager`(ctype: `void*`)
    /// - offset: `  0`(x86)/`200`(x86_64)
    /// - type_size: `  0`(x86)/`  8`(x86_64)
    /// - flags: `SERIALIZE_IGNORED`
    #[cfg_attr(feature = "serde", serde(default))]
    #[cfg_attr(feature = "serde", serde(borrow))]
    #[cfg_attr(feature = "json_schema", schemars(rename = "assetManager"))]
    #[cfg_attr(feature = "serde", serde(rename = "assetManager"))]
    pub m_assetManager: Pointer<'a>,
    /// # C++ Info
    /// - name: `capabilities`(ctype: `hkInt32`)
    /// - offset: `  0`(x86)/`208`(x86_64)
    /// - type_size: `  0`(x86)/`  4`(x86_64)
    #[cfg_attr(feature = "serde_default", serde(default))]
    #[cfg_attr(feature = "serde", serde(borrow))]
    #[cfg_attr(feature = "json_schema", schemars(rename = "capabilities"))]
    #[cfg_attr(feature = "serde", serde(rename = "capabilities"))]
    pub m_capabilities: I32<'a>,
    /// # C++ Info
    /// - name: `effectiveCapabilities`(ctype: `hkInt32`)
    /// - offset: `  0`(x86)/`212`(x86_64)
    /// - type_size: `  0`(x86)/`  4`(x86_64)
    #[cfg_attr(feature = "serde_default", serde(default))]
    #[cfg_attr(feature = "serde", serde(borrow))]
    #[cfg_attr(feature = "json_schema", schemars(rename = "effectiveCapabilities"))]
    #[cfg_attr(feature = "serde", serde(rename = "effectiveCapabilities"))]
    pub m_effectiveCapabilities: I32<'a>,
}
const _: () = {
    use havok_serde as _serde;
    impl<'a> _serde::HavokClass for hkbCharacter<'a> {
        #[inline]
        fn name(&self) -> &'static str {
            "hkbCharacter"
        }
        #[inline]
        fn signature(&self) -> _serde::__private::Signature {
            _serde::__private::Signature::new(0x393e4e8d)
        }
        #[allow(clippy::let_and_return, clippy::vec_init_then_push)]
        fn deps_indexes(&self) -> Vec<&Pointer<'_>> {
            let mut v = Vec::new();
            v.extend(self.m_nearbyCharacters.iter());
            v.push(&self.m_generatorOutput);
            v.push(&self.m_ragdollDriver);
            v.push(&self.m_ragdollInterface);
            v.push(&self.m_characterControllerDriver);
            v.push(&self.m_footIkDriver);
            v.push(&self.m_handIkDriver);
            v.push(&self.m_dockingDriver);
            v.push(&self.m_aiControlDriver);
            v.push(&self.m_setup);
            v.push(&self.m_behaviorGraph);
            v.push(&self.m_projectData);
            v.push(&self.m_animationBindingSet);
            v.push(&self.m_spatialQueryInterface);
            v.push(&self.m_world);
            v.extend(self.m_boneAttachments.iter());
            v.push(&self.m_eventQueue);
            v.push(&self.m_characterLuaState);
            v.push(&self.m_assetManager);
            v
        }
    }
    impl<'a> _serde::Serialize for hkbCharacter<'a> {
        fn serialize<S>(&self, __serializer: S) -> Result<S::Ok, S::Error>
        where
            S: _serde::ser::Serializer,
        {
            let class_meta = self
                .__ptr
                .as_ref()
                .map(|name| (name, _serde::__private::Signature::new(0x393e4e8d)));
            let mut serializer = __serializer
                .serialize_struct("hkbCharacter", class_meta, (0u64, 216u64))?;
            serializer.pad_field([0u8; 0usize].as_slice(), [0u8; 8usize].as_slice())?;
            serializer
                .skip_field("memSizeAndRefCount", &self.parent.m_memSizeAndRefCount)?;
            serializer.pad_field([0u8; 0usize].as_slice(), [0u8; 4usize].as_slice())?;
            serializer
                .serialize_array_field(
                    "nearbyCharacters",
                    &self.m_nearbyCharacters,
                    TypeSize::NonPtr,
                )?;
            serializer.serialize_field("userData", &self.m_userData)?;
            serializer.serialize_field("currentLod", &self.m_currentLod)?;
            serializer.skip_field("numTracksInLod", &self.m_numTracksInLod)?;
            serializer.pad_field([0u8; 0usize].as_slice(), [0u8; 4usize].as_slice())?;
            serializer.skip_field("generatorOutput", &self.m_generatorOutput)?;
            serializer.serialize_field("name", &self.m_name)?;
            serializer.skip_field("ragdollDriver", &self.m_ragdollDriver)?;
            serializer.skip_field("ragdollInterface", &self.m_ragdollInterface)?;
            serializer
                .skip_field(
                    "characterControllerDriver",
                    &self.m_characterControllerDriver,
                )?;
            serializer.skip_field("footIkDriver", &self.m_footIkDriver)?;
            serializer.skip_field("handIkDriver", &self.m_handIkDriver)?;
            serializer.skip_field("dockingDriver", &self.m_dockingDriver)?;
            serializer.skip_field("aiControlDriver", &self.m_aiControlDriver)?;
            serializer.serialize_field("setup", &self.m_setup)?;
            serializer.serialize_field("behaviorGraph", &self.m_behaviorGraph)?;
            serializer.serialize_field("projectData", &self.m_projectData)?;
            serializer.skip_field("animationBindingSet", &self.m_animationBindingSet)?;
            serializer
                .skip_field("spatialQueryInterface", &self.m_spatialQueryInterface)?;
            serializer.skip_field("world", &self.m_world)?;
            serializer
                .skip_array_field(
                    "boneAttachments",
                    &self.m_boneAttachments,
                    TypeSize::NonPtr,
                )?;
            serializer.skip_field("eventQueue", &self.m_eventQueue)?;
            serializer.skip_field("characterLuaState", &self.m_characterLuaState)?;
            serializer.skip_field("assetManager", &self.m_assetManager)?;
            serializer.serialize_field("capabilities", &self.m_capabilities)?;
            serializer
                .serialize_field(
                    "effectiveCapabilities",
                    &self.m_effectiveCapabilities,
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
    impl<'de> _serde::Deserialize<'de> for hkbCharacter<'de> {
        fn deserialize<__D>(deserializer: __D) -> core::result::Result<Self, __D::Error>
        where
            __D: _serde::Deserializer<'de>,
        {
            #[allow(non_camel_case_types)]
            enum __Field {
                m_nearbyCharacters,
                m_userData,
                m_currentLod,
                m_name,
                m_setup,
                m_behaviorGraph,
                m_projectData,
                m_capabilities,
                m_effectiveCapabilities,
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
                        "nearbyCharacters" => Ok(__Field::m_nearbyCharacters),
                        "userData" => Ok(__Field::m_userData),
                        "currentLod" => Ok(__Field::m_currentLod),
                        "name" => Ok(__Field::m_name),
                        "setup" => Ok(__Field::m_setup),
                        "behaviorGraph" => Ok(__Field::m_behaviorGraph),
                        "projectData" => Ok(__Field::m_projectData),
                        "capabilities" => Ok(__Field::m_capabilities),
                        "effectiveCapabilities" => Ok(__Field::m_effectiveCapabilities),
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
            struct __hkbCharacterVisitor<'de> {
                marker: _serde::__private::PhantomData<hkbCharacter<'de>>,
                lifetime: _serde::__private::PhantomData<&'de ()>,
            }
            #[allow(clippy::match_single_binding)]
            #[allow(clippy::reversed_empty_ranges)]
            #[allow(clippy::single_match)]
            impl<'de> _serde::de::Visitor<'de> for __hkbCharacterVisitor<'de> {
                type Value = hkbCharacter<'de>;
                fn expecting(
                    &self,
                    __formatter: &mut core::fmt::Formatter,
                ) -> core::fmt::Result {
                    core::fmt::Formatter::write_str(__formatter, "struct hkbCharacter")
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
                    let mut m_nearbyCharacters: _serde::__private::Option<
                        Vec<Pointer<'de>>,
                    > = _serde::__private::None;
                    let mut m_userData: _serde::__private::Option<Ulong> = _serde::__private::None;
                    let mut m_currentLod: _serde::__private::Option<I16<'de>> = _serde::__private::None;
                    let mut m_numTracksInLod: _serde::__private::Option<I16<'de>> = _serde::__private::None;
                    let mut m_generatorOutput: _serde::__private::Option<Pointer<'de>> = _serde::__private::None;
                    let mut m_name: _serde::__private::Option<StringPtr<'de>> = _serde::__private::None;
                    let mut m_ragdollDriver: _serde::__private::Option<Pointer<'de>> = _serde::__private::None;
                    let mut m_ragdollInterface: _serde::__private::Option<
                        Pointer<'de>,
                    > = _serde::__private::None;
                    let mut m_characterControllerDriver: _serde::__private::Option<
                        Pointer<'de>,
                    > = _serde::__private::None;
                    let mut m_footIkDriver: _serde::__private::Option<Pointer<'de>> = _serde::__private::None;
                    let mut m_handIkDriver: _serde::__private::Option<Pointer<'de>> = _serde::__private::None;
                    let mut m_dockingDriver: _serde::__private::Option<Pointer<'de>> = _serde::__private::None;
                    let mut m_aiControlDriver: _serde::__private::Option<Pointer<'de>> = _serde::__private::None;
                    let mut m_setup: _serde::__private::Option<Pointer<'de>> = _serde::__private::None;
                    let mut m_behaviorGraph: _serde::__private::Option<Pointer<'de>> = _serde::__private::None;
                    let mut m_projectData: _serde::__private::Option<Pointer<'de>> = _serde::__private::None;
                    let mut m_animationBindingSet: _serde::__private::Option<
                        Pointer<'de>,
                    > = _serde::__private::None;
                    let mut m_spatialQueryInterface: _serde::__private::Option<
                        Pointer<'de>,
                    > = _serde::__private::None;
                    let mut m_world: _serde::__private::Option<Pointer<'de>> = _serde::__private::None;
                    let mut m_boneAttachments: _serde::__private::Option<
                        Vec<Pointer<'de>>,
                    > = _serde::__private::None;
                    let mut m_eventQueue: _serde::__private::Option<Pointer<'de>> = _serde::__private::None;
                    let mut m_characterLuaState: _serde::__private::Option<
                        Pointer<'de>,
                    > = _serde::__private::None;
                    let mut m_assetManager: _serde::__private::Option<Pointer<'de>> = _serde::__private::None;
                    let mut m_capabilities: _serde::__private::Option<I32<'de>> = _serde::__private::None;
                    let mut m_effectiveCapabilities: _serde::__private::Option<
                        I32<'de>,
                    > = _serde::__private::None;
                    for i in 0..25usize {
                        match i {
                            0usize => {
                                if _serde::__private::Option::is_some(&m_nearbyCharacters) {
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "nearbyCharacters",
                                        ),
                                    );
                                }
                                m_nearbyCharacters = _serde::__private::Some(
                                    match __A::next_value::<Vec<Pointer<'de>>>(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            1usize => {
                                if _serde::__private::Option::is_some(&m_userData) {
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
                            2usize => {
                                if _serde::__private::Option::is_some(&m_currentLod) {
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "currentLod",
                                        ),
                                    );
                                }
                                m_currentLod = _serde::__private::Some(
                                    match __A::next_value::<I16<'de>>(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            3usize => {
                                if _serde::__private::Option::is_some(&m_numTracksInLod) {
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "numTracksInLod",
                                        ),
                                    );
                                }
                                m_numTracksInLod = _serde::__private::Some(
                                    match __A::next_value::<I16<'de>>(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            4usize => {
                                if _serde::__private::Option::is_some(&m_generatorOutput) {
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "generatorOutput",
                                        ),
                                    );
                                }
                                __A::pad(&mut __map, 0usize, 4usize)?;
                                m_generatorOutput = _serde::__private::Some(
                                    match __A::next_value::<Pointer<'de>>(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            5usize => {
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
                            6usize => {
                                if _serde::__private::Option::is_some(&m_ragdollDriver) {
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "ragdollDriver",
                                        ),
                                    );
                                }
                                m_ragdollDriver = _serde::__private::Some(
                                    match __A::next_value::<Pointer<'de>>(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            7usize => {
                                if _serde::__private::Option::is_some(&m_ragdollInterface) {
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "ragdollInterface",
                                        ),
                                    );
                                }
                                m_ragdollInterface = _serde::__private::Some(
                                    match __A::next_value::<Pointer<'de>>(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            8usize => {
                                if _serde::__private::Option::is_some(
                                    &m_characterControllerDriver,
                                ) {
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "characterControllerDriver",
                                        ),
                                    );
                                }
                                m_characterControllerDriver = _serde::__private::Some(
                                    match __A::next_value::<Pointer<'de>>(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            9usize => {
                                if _serde::__private::Option::is_some(&m_footIkDriver) {
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "footIkDriver",
                                        ),
                                    );
                                }
                                m_footIkDriver = _serde::__private::Some(
                                    match __A::next_value::<Pointer<'de>>(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            10usize => {
                                if _serde::__private::Option::is_some(&m_handIkDriver) {
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "handIkDriver",
                                        ),
                                    );
                                }
                                m_handIkDriver = _serde::__private::Some(
                                    match __A::next_value::<Pointer<'de>>(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            11usize => {
                                if _serde::__private::Option::is_some(&m_dockingDriver) {
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "dockingDriver",
                                        ),
                                    );
                                }
                                m_dockingDriver = _serde::__private::Some(
                                    match __A::next_value::<Pointer<'de>>(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            12usize => {
                                if _serde::__private::Option::is_some(&m_aiControlDriver) {
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "aiControlDriver",
                                        ),
                                    );
                                }
                                m_aiControlDriver = _serde::__private::Some(
                                    match __A::next_value::<Pointer<'de>>(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            13usize => {
                                if _serde::__private::Option::is_some(&m_setup) {
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field("setup"),
                                    );
                                }
                                m_setup = _serde::__private::Some(
                                    match __A::next_value::<Pointer<'de>>(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            14usize => {
                                if _serde::__private::Option::is_some(&m_behaviorGraph) {
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "behaviorGraph",
                                        ),
                                    );
                                }
                                m_behaviorGraph = _serde::__private::Some(
                                    match __A::next_value::<Pointer<'de>>(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            15usize => {
                                if _serde::__private::Option::is_some(&m_projectData) {
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "projectData",
                                        ),
                                    );
                                }
                                m_projectData = _serde::__private::Some(
                                    match __A::next_value::<Pointer<'de>>(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            16usize => {
                                if _serde::__private::Option::is_some(
                                    &m_animationBindingSet,
                                ) {
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "animationBindingSet",
                                        ),
                                    );
                                }
                                m_animationBindingSet = _serde::__private::Some(
                                    match __A::next_value::<Pointer<'de>>(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            17usize => {
                                if _serde::__private::Option::is_some(
                                    &m_spatialQueryInterface,
                                ) {
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "spatialQueryInterface",
                                        ),
                                    );
                                }
                                m_spatialQueryInterface = _serde::__private::Some(
                                    match __A::next_value::<Pointer<'de>>(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            18usize => {
                                if _serde::__private::Option::is_some(&m_world) {
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field("world"),
                                    );
                                }
                                m_world = _serde::__private::Some(
                                    match __A::next_value::<Pointer<'de>>(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            19usize => {
                                if _serde::__private::Option::is_some(&m_boneAttachments) {
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "boneAttachments",
                                        ),
                                    );
                                }
                                m_boneAttachments = _serde::__private::Some(
                                    match __A::next_value::<Vec<Pointer<'de>>>(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            20usize => {
                                if _serde::__private::Option::is_some(&m_eventQueue) {
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "eventQueue",
                                        ),
                                    );
                                }
                                m_eventQueue = _serde::__private::Some(
                                    match __A::next_value::<Pointer<'de>>(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            21usize => {
                                if _serde::__private::Option::is_some(
                                    &m_characterLuaState,
                                ) {
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "characterLuaState",
                                        ),
                                    );
                                }
                                m_characterLuaState = _serde::__private::Some(
                                    match __A::next_value::<Pointer<'de>>(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            22usize => {
                                if _serde::__private::Option::is_some(&m_assetManager) {
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "assetManager",
                                        ),
                                    );
                                }
                                m_assetManager = _serde::__private::Some(
                                    match __A::next_value::<Pointer<'de>>(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            23usize => {
                                if _serde::__private::Option::is_some(&m_capabilities) {
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "capabilities",
                                        ),
                                    );
                                }
                                m_capabilities = _serde::__private::Some(
                                    match __A::next_value::<I32<'de>>(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            24usize => {
                                if _serde::__private::Option::is_some(
                                    &m_effectiveCapabilities,
                                ) {
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "effectiveCapabilities",
                                        ),
                                    );
                                }
                                m_effectiveCapabilities = _serde::__private::Some(
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
                    let m_nearbyCharacters = match m_nearbyCharacters {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "nearbyCharacters",
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
                    let m_currentLod = match m_currentLod {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "currentLod",
                                ),
                            );
                        }
                    };
                    let m_numTracksInLod = match m_numTracksInLod {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "numTracksInLod",
                                ),
                            );
                        }
                    };
                    let m_generatorOutput = match m_generatorOutput {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "generatorOutput",
                                ),
                            );
                        }
                    };
                    let m_name = match m_name {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field("name"),
                            );
                        }
                    };
                    let m_ragdollDriver = match m_ragdollDriver {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "ragdollDriver",
                                ),
                            );
                        }
                    };
                    let m_ragdollInterface = match m_ragdollInterface {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "ragdollInterface",
                                ),
                            );
                        }
                    };
                    let m_characterControllerDriver = match m_characterControllerDriver {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "characterControllerDriver",
                                ),
                            );
                        }
                    };
                    let m_footIkDriver = match m_footIkDriver {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "footIkDriver",
                                ),
                            );
                        }
                    };
                    let m_handIkDriver = match m_handIkDriver {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "handIkDriver",
                                ),
                            );
                        }
                    };
                    let m_dockingDriver = match m_dockingDriver {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "dockingDriver",
                                ),
                            );
                        }
                    };
                    let m_aiControlDriver = match m_aiControlDriver {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "aiControlDriver",
                                ),
                            );
                        }
                    };
                    let m_setup = match m_setup {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field("setup"),
                            );
                        }
                    };
                    let m_behaviorGraph = match m_behaviorGraph {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "behaviorGraph",
                                ),
                            );
                        }
                    };
                    let m_projectData = match m_projectData {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "projectData",
                                ),
                            );
                        }
                    };
                    let m_animationBindingSet = match m_animationBindingSet {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "animationBindingSet",
                                ),
                            );
                        }
                    };
                    let m_spatialQueryInterface = match m_spatialQueryInterface {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "spatialQueryInterface",
                                ),
                            );
                        }
                    };
                    let m_world = match m_world {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field("world"),
                            );
                        }
                    };
                    let m_boneAttachments = match m_boneAttachments {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "boneAttachments",
                                ),
                            );
                        }
                    };
                    let m_eventQueue = match m_eventQueue {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "eventQueue",
                                ),
                            );
                        }
                    };
                    let m_characterLuaState = match m_characterLuaState {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "characterLuaState",
                                ),
                            );
                        }
                    };
                    let m_assetManager = match m_assetManager {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "assetManager",
                                ),
                            );
                        }
                    };
                    let m_capabilities = match m_capabilities {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "capabilities",
                                ),
                            );
                        }
                    };
                    let m_effectiveCapabilities = match m_effectiveCapabilities {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "effectiveCapabilities",
                                ),
                            );
                        }
                    };
                    _serde::__private::Ok(hkbCharacter {
                        __ptr,
                        parent,
                        m_nearbyCharacters,
                        m_userData,
                        m_currentLod,
                        m_numTracksInLod,
                        m_generatorOutput,
                        m_name,
                        m_ragdollDriver,
                        m_ragdollInterface,
                        m_characterControllerDriver,
                        m_footIkDriver,
                        m_handIkDriver,
                        m_dockingDriver,
                        m_aiControlDriver,
                        m_setup,
                        m_behaviorGraph,
                        m_projectData,
                        m_animationBindingSet,
                        m_spatialQueryInterface,
                        m_world,
                        m_boneAttachments,
                        m_eventQueue,
                        m_characterLuaState,
                        m_assetManager,
                        m_capabilities,
                        m_effectiveCapabilities,
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
                    let mut m_nearbyCharacters: _serde::__private::Option<
                        Vec<Pointer<'de>>,
                    > = _serde::__private::None;
                    let mut m_userData: _serde::__private::Option<Ulong> = _serde::__private::None;
                    let mut m_currentLod: _serde::__private::Option<I16<'de>> = _serde::__private::None;
                    let mut m_name: _serde::__private::Option<StringPtr<'de>> = _serde::__private::None;
                    let mut m_setup: _serde::__private::Option<Pointer<'de>> = _serde::__private::None;
                    let mut m_behaviorGraph: _serde::__private::Option<Pointer<'de>> = _serde::__private::None;
                    let mut m_projectData: _serde::__private::Option<Pointer<'de>> = _serde::__private::None;
                    let mut m_capabilities: _serde::__private::Option<I32<'de>> = _serde::__private::None;
                    let mut m_effectiveCapabilities: _serde::__private::Option<
                        I32<'de>,
                    > = _serde::__private::None;
                    while let _serde::__private::Some(__key) = {
                        __A::next_key::<__Field>(&mut __map)?
                    } {
                        match __key {
                            __Field::m_nearbyCharacters => {
                                #[cfg(
                                    any(feature = "strict", feature = "ignore_duplicates")
                                )]
                                if _serde::__private::Option::is_some(&m_nearbyCharacters) {
                                    #[cfg(feature = "ignore_duplicates")]
                                    {
                                        __A::skip_value(&mut __map)?;
                                        continue;
                                    }
                                    #[cfg(feature = "strict")]
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "nearbyCharacters",
                                        ),
                                    );
                                }
                                m_nearbyCharacters = _serde::__private::Some(
                                    match __A::next_value::<Vec<Pointer<'de>>>(&mut __map) {
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
                            __Field::m_currentLod => {
                                #[cfg(
                                    any(feature = "strict", feature = "ignore_duplicates")
                                )]
                                if _serde::__private::Option::is_some(&m_currentLod) {
                                    #[cfg(feature = "ignore_duplicates")]
                                    {
                                        __A::skip_value(&mut __map)?;
                                        continue;
                                    }
                                    #[cfg(feature = "strict")]
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "currentLod",
                                        ),
                                    );
                                }
                                m_currentLod = _serde::__private::Some(
                                    match __A::next_value::<I16<'de>>(&mut __map) {
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
                            __Field::m_setup => {
                                #[cfg(
                                    any(feature = "strict", feature = "ignore_duplicates")
                                )]
                                if _serde::__private::Option::is_some(&m_setup) {
                                    #[cfg(feature = "ignore_duplicates")]
                                    {
                                        __A::skip_value(&mut __map)?;
                                        continue;
                                    }
                                    #[cfg(feature = "strict")]
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field("setup"),
                                    );
                                }
                                m_setup = _serde::__private::Some(
                                    match __A::next_value::<Pointer<'de>>(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            __Field::m_behaviorGraph => {
                                #[cfg(
                                    any(feature = "strict", feature = "ignore_duplicates")
                                )]
                                if _serde::__private::Option::is_some(&m_behaviorGraph) {
                                    #[cfg(feature = "ignore_duplicates")]
                                    {
                                        __A::skip_value(&mut __map)?;
                                        continue;
                                    }
                                    #[cfg(feature = "strict")]
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "behaviorGraph",
                                        ),
                                    );
                                }
                                m_behaviorGraph = _serde::__private::Some(
                                    match __A::next_value::<Pointer<'de>>(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            __Field::m_projectData => {
                                #[cfg(
                                    any(feature = "strict", feature = "ignore_duplicates")
                                )]
                                if _serde::__private::Option::is_some(&m_projectData) {
                                    #[cfg(feature = "ignore_duplicates")]
                                    {
                                        __A::skip_value(&mut __map)?;
                                        continue;
                                    }
                                    #[cfg(feature = "strict")]
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "projectData",
                                        ),
                                    );
                                }
                                m_projectData = _serde::__private::Some(
                                    match __A::next_value::<Pointer<'de>>(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            __Field::m_capabilities => {
                                #[cfg(
                                    any(feature = "strict", feature = "ignore_duplicates")
                                )]
                                if _serde::__private::Option::is_some(&m_capabilities) {
                                    #[cfg(feature = "ignore_duplicates")]
                                    {
                                        __A::skip_value(&mut __map)?;
                                        continue;
                                    }
                                    #[cfg(feature = "strict")]
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "capabilities",
                                        ),
                                    );
                                }
                                m_capabilities = _serde::__private::Some(
                                    match __A::next_value::<I32<'de>>(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            __Field::m_effectiveCapabilities => {
                                #[cfg(
                                    any(feature = "strict", feature = "ignore_duplicates")
                                )]
                                if _serde::__private::Option::is_some(
                                    &m_effectiveCapabilities,
                                ) {
                                    #[cfg(feature = "ignore_duplicates")]
                                    {
                                        __A::skip_value(&mut __map)?;
                                        continue;
                                    }
                                    #[cfg(feature = "strict")]
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "effectiveCapabilities",
                                        ),
                                    );
                                }
                                m_effectiveCapabilities = _serde::__private::Some(
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
                    let m_nearbyCharacters = match m_nearbyCharacters {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            #[cfg(feature = "strict")]
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "nearbyCharacters",
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
                    let m_currentLod = match m_currentLod {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            #[cfg(feature = "strict")]
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "currentLod",
                                ),
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
                    let m_setup = match m_setup {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            #[cfg(feature = "strict")]
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field("setup"),
                            );
                            #[cfg(not(feature = "strict"))] Default::default()
                        }
                    };
                    let m_behaviorGraph = match m_behaviorGraph {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            #[cfg(feature = "strict")]
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "behaviorGraph",
                                ),
                            );
                            #[cfg(not(feature = "strict"))] Default::default()
                        }
                    };
                    let m_projectData = match m_projectData {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            #[cfg(feature = "strict")]
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "projectData",
                                ),
                            );
                            #[cfg(not(feature = "strict"))] Default::default()
                        }
                    };
                    let m_capabilities = match m_capabilities {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            #[cfg(feature = "strict")]
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "capabilities",
                                ),
                            );
                            #[cfg(not(feature = "strict"))] Default::default()
                        }
                    };
                    let m_effectiveCapabilities = match m_effectiveCapabilities {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            #[cfg(feature = "strict")]
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "effectiveCapabilities",
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
                    let __ptr = __A::class_ptr(&mut __map);
                    _serde::__private::Ok(hkbCharacter {
                        __ptr: __ptr.clone(),
                        parent,
                        m_nearbyCharacters,
                        m_userData,
                        m_currentLod,
                        m_name,
                        m_setup,
                        m_behaviorGraph,
                        m_projectData,
                        m_capabilities,
                        m_effectiveCapabilities,
                        ..Default::default()
                    })
                }
            }
            const FIELDS: &[&str] = &[
                "nearbyCharacters",
                "userData",
                "currentLod",
                "numTracksInLod",
                "generatorOutput",
                "name",
                "ragdollDriver",
                "ragdollInterface",
                "characterControllerDriver",
                "footIkDriver",
                "handIkDriver",
                "dockingDriver",
                "aiControlDriver",
                "setup",
                "behaviorGraph",
                "projectData",
                "animationBindingSet",
                "spatialQueryInterface",
                "world",
                "boneAttachments",
                "eventQueue",
                "characterLuaState",
                "assetManager",
                "capabilities",
                "effectiveCapabilities",
            ];
            _serde::Deserializer::deserialize_struct(
                deserializer,
                "hkbCharacter",
                FIELDS,
                __hkbCharacterVisitor {
                    marker: _serde::__private::PhantomData::<hkbCharacter>,
                    lifetime: _serde::__private::PhantomData,
                },
            )
        }
    }
};
