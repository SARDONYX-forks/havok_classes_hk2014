use super::class_requires::*;
use super::*;
/// # C++ Info
/// - name: `hkbBehaviorGraph`
/// - version: `1`
/// - signature: `0xfdedb83b`
/// - size: `  0`(x86)/`432`(x86_64)
/// -  vtable: `true`
#[allow(non_upper_case_globals, non_snake_case)]
#[cfg_attr(feature = "json_schema", derive(schemars::JsonSchema))]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(educe::Educe)]
#[educe(Debug, Clone, Default, PartialEq)]
pub struct hkbBehaviorGraph<'a> {
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
    pub parent: hkbGenerator<'a>,
    /// # C++ Info
    /// - name: `variableMode`(ctype: `enum VariableMode`)
    /// - offset: `  0`(x86)/`136`(x86_64)
    /// - type_size: `  0`(x86)/`  1`(x86_64)
    #[cfg_attr(feature = "serde_default", serde(default))]
    #[cfg_attr(feature = "json_schema", schemars(rename = "variableMode"))]
    #[cfg_attr(feature = "serde", serde(rename = "variableMode"))]
    pub m_variableMode: VariableMode,
    /// # C++ Info
    /// - name: `uniqueIdPool`(ctype: `hkArray<void>`)
    /// - offset: `  0`(x86)/`144`(x86_64)
    /// - type_size: `  0`(x86)/` 16`(x86_64)
    /// - flags: `SERIALIZE_IGNORED`
    #[cfg_attr(feature = "serde", serde(default))]
    #[cfg_attr(feature = "json_schema", schemars(rename = "uniqueIdPool"))]
    #[cfg_attr(feature = "serde", serde(rename = "uniqueIdPool"))]
    pub m_uniqueIdPool: Vec<()>,
    /// # C++ Info
    /// - name: `idToStateMachineTemplateMap`(ctype: `void*`)
    /// - offset: `  0`(x86)/`160`(x86_64)
    /// - type_size: `  0`(x86)/`  8`(x86_64)
    /// - flags: `SERIALIZE_IGNORED`
    #[cfg_attr(feature = "serde", serde(default))]
    #[cfg_attr(feature = "serde", serde(borrow))]
    #[cfg_attr(
        feature = "json_schema",
        schemars(rename = "idToStateMachineTemplateMap")
    )]
    #[cfg_attr(feature = "serde", serde(rename = "idToStateMachineTemplateMap"))]
    pub m_idToStateMachineTemplateMap: Pointer<'a>,
    /// # C++ Info
    /// - name: `mirroredExternalIdMap`(ctype: `hkArray<void>`)
    /// - offset: `  0`(x86)/`168`(x86_64)
    /// - type_size: `  0`(x86)/` 16`(x86_64)
    /// - flags: `SERIALIZE_IGNORED`
    #[cfg_attr(feature = "serde", serde(default))]
    #[cfg_attr(feature = "json_schema", schemars(rename = "mirroredExternalIdMap"))]
    #[cfg_attr(feature = "serde", serde(rename = "mirroredExternalIdMap"))]
    pub m_mirroredExternalIdMap: Vec<()>,
    /// # C++ Info
    /// - name: `pseudoRandomGenerator`(ctype: `void*`)
    /// - offset: `  0`(x86)/`184`(x86_64)
    /// - type_size: `  0`(x86)/`  8`(x86_64)
    /// - flags: `SERIALIZE_IGNORED`
    #[cfg_attr(feature = "serde", serde(default))]
    #[cfg_attr(feature = "serde", serde(borrow))]
    #[cfg_attr(feature = "json_schema", schemars(rename = "pseudoRandomGenerator"))]
    #[cfg_attr(feature = "serde", serde(rename = "pseudoRandomGenerator"))]
    pub m_pseudoRandomGenerator: Pointer<'a>,
    /// # C++ Info
    /// - name: `rootGenerator`(ctype: `struct hkbGenerator*`)
    /// - offset: `  0`(x86)/`192`(x86_64)
    /// - type_size: `  0`(x86)/`  8`(x86_64)
    #[cfg_attr(feature = "serde_default", serde(default))]
    #[cfg_attr(feature = "serde", serde(borrow))]
    #[cfg_attr(feature = "json_schema", schemars(rename = "rootGenerator"))]
    #[cfg_attr(feature = "serde", serde(rename = "rootGenerator"))]
    pub m_rootGenerator: Pointer<'a>,
    /// # C++ Info
    /// - name: `data`(ctype: `struct hkbBehaviorGraphData*`)
    /// - offset: `  0`(x86)/`200`(x86_64)
    /// - type_size: `  0`(x86)/`  8`(x86_64)
    #[cfg_attr(feature = "serde_default", serde(default))]
    #[cfg_attr(feature = "serde", serde(borrow))]
    #[cfg_attr(feature = "json_schema", schemars(rename = "data"))]
    #[cfg_attr(feature = "serde", serde(rename = "data"))]
    pub m_data: Pointer<'a>,
    /// # C++ Info
    /// - name: `template`(ctype: `void*`)
    /// - offset: `  0`(x86)/`208`(x86_64)
    /// - type_size: `  0`(x86)/`  8`(x86_64)
    /// - flags: `SERIALIZE_IGNORED`
    #[cfg_attr(feature = "serde", serde(default))]
    #[cfg_attr(feature = "serde", serde(borrow))]
    #[cfg_attr(feature = "json_schema", schemars(rename = "template"))]
    #[cfg_attr(feature = "serde", serde(rename = "template"))]
    pub m_template: Pointer<'a>,
    /// # C++ Info
    /// - name: `rootGeneratorClone`(ctype: `void*`)
    /// - offset: `  0`(x86)/`216`(x86_64)
    /// - type_size: `  0`(x86)/`  8`(x86_64)
    /// - flags: `SERIALIZE_IGNORED`
    #[cfg_attr(feature = "serde", serde(default))]
    #[cfg_attr(feature = "serde", serde(borrow))]
    #[cfg_attr(feature = "json_schema", schemars(rename = "rootGeneratorClone"))]
    #[cfg_attr(feature = "serde", serde(rename = "rootGeneratorClone"))]
    pub m_rootGeneratorClone: Pointer<'a>,
    /// # C++ Info
    /// - name: `activeNodes`(ctype: `void*`)
    /// - offset: `  0`(x86)/`224`(x86_64)
    /// - type_size: `  0`(x86)/`  8`(x86_64)
    /// - flags: `SERIALIZE_IGNORED`
    #[cfg_attr(feature = "serde", serde(default))]
    #[cfg_attr(feature = "serde", serde(borrow))]
    #[cfg_attr(feature = "json_schema", schemars(rename = "activeNodes"))]
    #[cfg_attr(feature = "serde", serde(rename = "activeNodes"))]
    pub m_activeNodes: Pointer<'a>,
    /// # C++ Info
    /// - name: `globalTransitionData`(ctype: `void*`)
    /// - offset: `  0`(x86)/`232`(x86_64)
    /// - type_size: `  0`(x86)/`  8`(x86_64)
    /// - flags: `SERIALIZE_IGNORED`
    #[cfg_attr(feature = "serde", serde(default))]
    #[cfg_attr(feature = "serde", serde(borrow))]
    #[cfg_attr(feature = "json_schema", schemars(rename = "globalTransitionData"))]
    #[cfg_attr(feature = "serde", serde(rename = "globalTransitionData"))]
    pub m_globalTransitionData: Pointer<'a>,
    /// # C++ Info
    /// - name: `eventIdMap`(ctype: `void*`)
    /// - offset: `  0`(x86)/`240`(x86_64)
    /// - type_size: `  0`(x86)/`  8`(x86_64)
    /// - flags: `SERIALIZE_IGNORED`
    #[cfg_attr(feature = "serde", serde(default))]
    #[cfg_attr(feature = "serde", serde(borrow))]
    #[cfg_attr(feature = "json_schema", schemars(rename = "eventIdMap"))]
    #[cfg_attr(feature = "serde", serde(rename = "eventIdMap"))]
    pub m_eventIdMap: Pointer<'a>,
    /// # C++ Info
    /// - name: `attributeIdMap`(ctype: `void*`)
    /// - offset: `  0`(x86)/`248`(x86_64)
    /// - type_size: `  0`(x86)/`  8`(x86_64)
    /// - flags: `SERIALIZE_IGNORED`
    #[cfg_attr(feature = "serde", serde(default))]
    #[cfg_attr(feature = "serde", serde(borrow))]
    #[cfg_attr(feature = "json_schema", schemars(rename = "attributeIdMap"))]
    #[cfg_attr(feature = "serde", serde(rename = "attributeIdMap"))]
    pub m_attributeIdMap: Pointer<'a>,
    /// # C++ Info
    /// - name: `variableIdMap`(ctype: `void*`)
    /// - offset: `  0`(x86)/`256`(x86_64)
    /// - type_size: `  0`(x86)/`  8`(x86_64)
    /// - flags: `SERIALIZE_IGNORED`
    #[cfg_attr(feature = "serde", serde(default))]
    #[cfg_attr(feature = "serde", serde(borrow))]
    #[cfg_attr(feature = "json_schema", schemars(rename = "variableIdMap"))]
    #[cfg_attr(feature = "serde", serde(rename = "variableIdMap"))]
    pub m_variableIdMap: Pointer<'a>,
    /// # C++ Info
    /// - name: `characterPropertyIdMap`(ctype: `void*`)
    /// - offset: `  0`(x86)/`264`(x86_64)
    /// - type_size: `  0`(x86)/`  8`(x86_64)
    /// - flags: `SERIALIZE_IGNORED`
    #[cfg_attr(feature = "serde", serde(default))]
    #[cfg_attr(feature = "serde", serde(borrow))]
    #[cfg_attr(feature = "json_schema", schemars(rename = "characterPropertyIdMap"))]
    #[cfg_attr(feature = "serde", serde(rename = "characterPropertyIdMap"))]
    pub m_characterPropertyIdMap: Pointer<'a>,
    /// # C++ Info
    /// - name: `variableValueSet`(ctype: `void*`)
    /// - offset: `  0`(x86)/`272`(x86_64)
    /// - type_size: `  0`(x86)/`  8`(x86_64)
    /// - flags: `SERIALIZE_IGNORED`
    #[cfg_attr(feature = "serde", serde(default))]
    #[cfg_attr(feature = "serde", serde(borrow))]
    #[cfg_attr(feature = "json_schema", schemars(rename = "variableValueSet"))]
    #[cfg_attr(feature = "serde", serde(rename = "variableValueSet"))]
    pub m_variableValueSet: Pointer<'a>,
    /// # C++ Info
    /// - name: `nodeTemplateToCloneMap`(ctype: `void*`)
    /// - offset: `  0`(x86)/`280`(x86_64)
    /// - type_size: `  0`(x86)/`  8`(x86_64)
    /// - flags: `SERIALIZE_IGNORED`
    #[cfg_attr(feature = "serde", serde(default))]
    #[cfg_attr(feature = "serde", serde(borrow))]
    #[cfg_attr(feature = "json_schema", schemars(rename = "nodeTemplateToCloneMap"))]
    #[cfg_attr(feature = "serde", serde(rename = "nodeTemplateToCloneMap"))]
    pub m_nodeTemplateToCloneMap: Pointer<'a>,
    /// # C++ Info
    /// - name: `stateListenerTemplateToCloneMap`(ctype: `void*`)
    /// - offset: `  0`(x86)/`288`(x86_64)
    /// - type_size: `  0`(x86)/`  8`(x86_64)
    /// - flags: `SERIALIZE_IGNORED`
    #[cfg_attr(feature = "serde", serde(default))]
    #[cfg_attr(feature = "serde", serde(borrow))]
    #[cfg_attr(
        feature = "json_schema",
        schemars(rename = "stateListenerTemplateToCloneMap")
    )]
    #[cfg_attr(feature = "serde", serde(rename = "stateListenerTemplateToCloneMap"))]
    pub m_stateListenerTemplateToCloneMap: Pointer<'a>,
    /// # C++ Info
    /// - name: `recentlyCreatedClones`(ctype: `hkArray<void*>`)
    /// - offset: `  0`(x86)/`296`(x86_64)
    /// - type_size: `  0`(x86)/` 16`(x86_64)
    /// - flags: `SERIALIZE_IGNORED`
    #[cfg_attr(feature = "serde", serde(default))]
    #[cfg_attr(feature = "serde", serde(borrow))]
    #[cfg_attr(feature = "json_schema", schemars(rename = "recentlyCreatedClones"))]
    #[cfg_attr(feature = "serde", serde(rename = "recentlyCreatedClones"))]
    pub m_recentlyCreatedClones: Vec<Pointer<'a>>,
    /// # C++ Info
    /// - name: `nodePartitionInfo`(ctype: `void*`)
    /// - offset: `  0`(x86)/`312`(x86_64)
    /// - type_size: `  0`(x86)/`  8`(x86_64)
    /// - flags: `SERIALIZE_IGNORED`
    #[cfg_attr(feature = "serde", serde(default))]
    #[cfg_attr(feature = "serde", serde(borrow))]
    #[cfg_attr(feature = "json_schema", schemars(rename = "nodePartitionInfo"))]
    #[cfg_attr(feature = "serde", serde(rename = "nodePartitionInfo"))]
    pub m_nodePartitionInfo: Pointer<'a>,
    /// # C++ Info
    /// - name: `numIntermediateOutputs`(ctype: `hkInt32`)
    /// - offset: `  0`(x86)/`320`(x86_64)
    /// - type_size: `  0`(x86)/`  4`(x86_64)
    /// - flags: `SERIALIZE_IGNORED`
    #[cfg_attr(feature = "serde", serde(default))]
    #[cfg_attr(feature = "serde", serde(borrow))]
    #[cfg_attr(feature = "json_schema", schemars(rename = "numIntermediateOutputs"))]
    #[cfg_attr(feature = "serde", serde(rename = "numIntermediateOutputs"))]
    pub m_numIntermediateOutputs: I32<'a>,
    /// # C++ Info
    /// - name: `intermediateOutputSizes`(ctype: `hkArray<void>`)
    /// - offset: `  0`(x86)/`328`(x86_64)
    /// - type_size: `  0`(x86)/` 16`(x86_64)
    /// - flags: `SERIALIZE_IGNORED`
    #[cfg_attr(feature = "serde", serde(default))]
    #[cfg_attr(feature = "json_schema", schemars(rename = "intermediateOutputSizes"))]
    #[cfg_attr(feature = "serde", serde(rename = "intermediateOutputSizes"))]
    pub m_intermediateOutputSizes: Vec<()>,
    /// # C++ Info
    /// - name: `jobs`(ctype: `hkArray<void*>`)
    /// - offset: `  0`(x86)/`344`(x86_64)
    /// - type_size: `  0`(x86)/` 16`(x86_64)
    /// - flags: `SERIALIZE_IGNORED`
    #[cfg_attr(feature = "serde", serde(default))]
    #[cfg_attr(feature = "serde", serde(borrow))]
    #[cfg_attr(feature = "json_schema", schemars(rename = "jobs"))]
    #[cfg_attr(feature = "serde", serde(rename = "jobs"))]
    pub m_jobs: Vec<Pointer<'a>>,
    /// # C++ Info
    /// - name: `allPartitionMemory`(ctype: `hkArray<void*>`)
    /// - offset: `  0`(x86)/`360`(x86_64)
    /// - type_size: `  0`(x86)/` 16`(x86_64)
    /// - flags: `SERIALIZE_IGNORED`
    #[cfg_attr(feature = "serde", serde(default))]
    #[cfg_attr(feature = "serde", serde(borrow))]
    #[cfg_attr(feature = "json_schema", schemars(rename = "allPartitionMemory"))]
    #[cfg_attr(feature = "serde", serde(rename = "allPartitionMemory"))]
    pub m_allPartitionMemory: Vec<Pointer<'a>>,
    /// # C++ Info
    /// - name: `internalToRootVariableIdMap`(ctype: `hkArray<void>`)
    /// - offset: `  0`(x86)/`376`(x86_64)
    /// - type_size: `  0`(x86)/` 16`(x86_64)
    /// - flags: `SERIALIZE_IGNORED`
    #[cfg_attr(feature = "serde", serde(default))]
    #[cfg_attr(
        feature = "json_schema",
        schemars(rename = "internalToRootVariableIdMap")
    )]
    #[cfg_attr(feature = "serde", serde(rename = "internalToRootVariableIdMap"))]
    pub m_internalToRootVariableIdMap: Vec<()>,
    /// # C++ Info
    /// - name: `internalToCharacterCharacterPropertyIdMap`(ctype: `hkArray<void>`)
    /// - offset: `  0`(x86)/`392`(x86_64)
    /// - type_size: `  0`(x86)/` 16`(x86_64)
    /// - flags: `SERIALIZE_IGNORED`
    #[cfg_attr(feature = "serde", serde(default))]
    #[cfg_attr(
        feature = "json_schema",
        schemars(rename = "internalToCharacterCharacterPropertyIdMap")
    )]
    #[cfg_attr(
        feature = "serde",
        serde(rename = "internalToCharacterCharacterPropertyIdMap")
    )]
    pub m_internalToCharacterCharacterPropertyIdMap: Vec<()>,
    /// # C++ Info
    /// - name: `internalToRootAttributeIdMap`(ctype: `hkArray<void>`)
    /// - offset: `  0`(x86)/`408`(x86_64)
    /// - type_size: `  0`(x86)/` 16`(x86_64)
    /// - flags: `SERIALIZE_IGNORED`
    #[cfg_attr(feature = "serde", serde(default))]
    #[cfg_attr(
        feature = "json_schema",
        schemars(rename = "internalToRootAttributeIdMap")
    )]
    #[cfg_attr(feature = "serde", serde(rename = "internalToRootAttributeIdMap"))]
    pub m_internalToRootAttributeIdMap: Vec<()>,
    /// # C++ Info
    /// - name: `nextUniqueId`(ctype: `hkUint16`)
    /// - offset: `  0`(x86)/`424`(x86_64)
    /// - type_size: `  0`(x86)/`  2`(x86_64)
    /// - flags: `SERIALIZE_IGNORED`
    #[cfg_attr(feature = "serde", serde(default))]
    #[cfg_attr(feature = "serde", serde(borrow))]
    #[cfg_attr(feature = "json_schema", schemars(rename = "nextUniqueId"))]
    #[cfg_attr(feature = "serde", serde(rename = "nextUniqueId"))]
    pub m_nextUniqueId: U16<'a>,
    /// # C++ Info
    /// - name: `isActive`(ctype: `hkBool`)
    /// - offset: `  0`(x86)/`426`(x86_64)
    /// - type_size: `  0`(x86)/`  1`(x86_64)
    /// - flags: `SERIALIZE_IGNORED`
    #[cfg_attr(feature = "serde", serde(default))]
    #[cfg_attr(feature = "json_schema", schemars(rename = "isActive"))]
    #[cfg_attr(feature = "serde", serde(rename = "isActive"))]
    pub m_isActive: bool,
    /// # C++ Info
    /// - name: `isLinked`(ctype: `hkBool`)
    /// - offset: `  0`(x86)/`427`(x86_64)
    /// - type_size: `  0`(x86)/`  1`(x86_64)
    /// - flags: `SERIALIZE_IGNORED`
    #[cfg_attr(feature = "serde", serde(default))]
    #[cfg_attr(feature = "json_schema", schemars(rename = "isLinked"))]
    #[cfg_attr(feature = "serde", serde(rename = "isLinked"))]
    pub m_isLinked: bool,
    /// # C++ Info
    /// - name: `updateActiveNodes`(ctype: `hkBool`)
    /// - offset: `  0`(x86)/`428`(x86_64)
    /// - type_size: `  0`(x86)/`  1`(x86_64)
    /// - flags: `SERIALIZE_IGNORED`
    #[cfg_attr(feature = "serde", serde(default))]
    #[cfg_attr(feature = "json_schema", schemars(rename = "updateActiveNodes"))]
    #[cfg_attr(feature = "serde", serde(rename = "updateActiveNodes"))]
    pub m_updateActiveNodes: bool,
    /// # C++ Info
    /// - name: `updateActiveNodesForEnable`(ctype: `hkBool`)
    /// - offset: `  0`(x86)/`429`(x86_64)
    /// - type_size: `  0`(x86)/`  1`(x86_64)
    /// - flags: `SERIALIZE_IGNORED`
    #[cfg_attr(feature = "serde", serde(default))]
    #[cfg_attr(feature = "json_schema", schemars(rename = "updateActiveNodesForEnable"))]
    #[cfg_attr(feature = "serde", serde(rename = "updateActiveNodesForEnable"))]
    pub m_updateActiveNodesForEnable: bool,
    /// # C++ Info
    /// - name: `checkNodeValidity`(ctype: `hkBool`)
    /// - offset: `  0`(x86)/`430`(x86_64)
    /// - type_size: `  0`(x86)/`  1`(x86_64)
    /// - flags: `SERIALIZE_IGNORED`
    #[cfg_attr(feature = "serde", serde(default))]
    #[cfg_attr(feature = "json_schema", schemars(rename = "checkNodeValidity"))]
    #[cfg_attr(feature = "serde", serde(rename = "checkNodeValidity"))]
    pub m_checkNodeValidity: bool,
    /// # C++ Info
    /// - name: `stateOrTransitionChanged`(ctype: `hkBool`)
    /// - offset: `  0`(x86)/`431`(x86_64)
    /// - type_size: `  0`(x86)/`  1`(x86_64)
    /// - flags: `SERIALIZE_IGNORED`
    #[cfg_attr(feature = "serde", serde(default))]
    #[cfg_attr(feature = "json_schema", schemars(rename = "stateOrTransitionChanged"))]
    #[cfg_attr(feature = "serde", serde(rename = "stateOrTransitionChanged"))]
    pub m_stateOrTransitionChanged: bool,
}
const _: () = {
    use havok_serde as _serde;
    impl<'a> _serde::HavokClass for hkbBehaviorGraph<'a> {
        #[inline]
        fn name(&self) -> &'static str {
            "hkbBehaviorGraph"
        }
        #[inline]
        fn signature(&self) -> _serde::__private::Signature {
            _serde::__private::Signature::new(0xfdedb83b)
        }
        #[allow(clippy::let_and_return, clippy::vec_init_then_push)]
        fn deps_indexes(&self) -> Vec<&Pointer<'_>> {
            let mut v = Vec::new();
            v.push(&self.parent.parent.parent.m_variableBindingSet);
            v.extend(self.parent.parent.parent.m_cachedBindables.iter());
            v.push(&self.parent.parent.m_nodeInfo);
            v.extend(self.parent.m_partitionInfo.deps_indexes());
            v.push(&self.parent.m_syncInfo);
            v.push(&self.m_idToStateMachineTemplateMap);
            v.push(&self.m_pseudoRandomGenerator);
            v.push(&self.m_rootGenerator);
            v.push(&self.m_data);
            v.push(&self.m_template);
            v.push(&self.m_rootGeneratorClone);
            v.push(&self.m_activeNodes);
            v.push(&self.m_globalTransitionData);
            v.push(&self.m_eventIdMap);
            v.push(&self.m_attributeIdMap);
            v.push(&self.m_variableIdMap);
            v.push(&self.m_characterPropertyIdMap);
            v.push(&self.m_variableValueSet);
            v.push(&self.m_nodeTemplateToCloneMap);
            v.push(&self.m_stateListenerTemplateToCloneMap);
            v.extend(self.m_recentlyCreatedClones.iter());
            v.push(&self.m_nodePartitionInfo);
            v.extend(self.m_jobs.iter());
            v.extend(self.m_allPartitionMemory.iter());
            v
        }
    }
    impl<'a> _serde::Serialize for hkbBehaviorGraph<'a> {
        fn serialize<S>(&self, __serializer: S) -> Result<S::Ok, S::Error>
        where
            S: _serde::ser::Serializer,
        {
            let class_meta = self
                .__ptr
                .as_ref()
                .map(|name| (name, _serde::__private::Signature::new(0xfdedb83b)));
            let mut serializer = __serializer
                .serialize_struct("hkbBehaviorGraph", class_meta, (0u64, 432u64))?;
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
            serializer.skip_field("partitionInfo", &self.parent.m_partitionInfo)?;
            serializer.skip_field("syncInfo", &self.parent.m_syncInfo)?;
            serializer
                .skip_fixed_array_field(
                    "pad",
                    self.parent.m_pad.as_slice(),
                    TypeSize::NonPtr,
                )?;
            serializer.pad_field([0u8; 0usize].as_slice(), [0u8; 4usize].as_slice())?;
            serializer.serialize_field("variableMode", &self.m_variableMode)?;
            serializer.pad_field([0u8; 0usize].as_slice(), [0u8; 7usize].as_slice())?;
            serializer
                .skip_array_field(
                    "uniqueIdPool",
                    &self.m_uniqueIdPool,
                    TypeSize::NonPtr,
                )?;
            serializer
                .skip_field(
                    "idToStateMachineTemplateMap",
                    &self.m_idToStateMachineTemplateMap,
                )?;
            serializer
                .skip_array_field(
                    "mirroredExternalIdMap",
                    &self.m_mirroredExternalIdMap,
                    TypeSize::NonPtr,
                )?;
            serializer
                .skip_field("pseudoRandomGenerator", &self.m_pseudoRandomGenerator)?;
            serializer.serialize_field("rootGenerator", &self.m_rootGenerator)?;
            serializer.serialize_field("data", &self.m_data)?;
            serializer.skip_field("template", &self.m_template)?;
            serializer.skip_field("rootGeneratorClone", &self.m_rootGeneratorClone)?;
            serializer.skip_field("activeNodes", &self.m_activeNodes)?;
            serializer.skip_field("globalTransitionData", &self.m_globalTransitionData)?;
            serializer.skip_field("eventIdMap", &self.m_eventIdMap)?;
            serializer.skip_field("attributeIdMap", &self.m_attributeIdMap)?;
            serializer.skip_field("variableIdMap", &self.m_variableIdMap)?;
            serializer
                .skip_field("characterPropertyIdMap", &self.m_characterPropertyIdMap)?;
            serializer.skip_field("variableValueSet", &self.m_variableValueSet)?;
            serializer
                .skip_field("nodeTemplateToCloneMap", &self.m_nodeTemplateToCloneMap)?;
            serializer
                .skip_field(
                    "stateListenerTemplateToCloneMap",
                    &self.m_stateListenerTemplateToCloneMap,
                )?;
            serializer
                .skip_array_field(
                    "recentlyCreatedClones",
                    &self.m_recentlyCreatedClones,
                    TypeSize::NonPtr,
                )?;
            serializer.skip_field("nodePartitionInfo", &self.m_nodePartitionInfo)?;
            serializer
                .skip_field("numIntermediateOutputs", &self.m_numIntermediateOutputs)?;
            serializer.pad_field([0u8; 0usize].as_slice(), [0u8; 4usize].as_slice())?;
            serializer
                .skip_array_field(
                    "intermediateOutputSizes",
                    &self.m_intermediateOutputSizes,
                    TypeSize::NonPtr,
                )?;
            serializer.skip_array_field("jobs", &self.m_jobs, TypeSize::NonPtr)?;
            serializer
                .skip_array_field(
                    "allPartitionMemory",
                    &self.m_allPartitionMemory,
                    TypeSize::NonPtr,
                )?;
            serializer
                .skip_array_field(
                    "internalToRootVariableIdMap",
                    &self.m_internalToRootVariableIdMap,
                    TypeSize::NonPtr,
                )?;
            serializer
                .skip_array_field(
                    "internalToCharacterCharacterPropertyIdMap",
                    &self.m_internalToCharacterCharacterPropertyIdMap,
                    TypeSize::NonPtr,
                )?;
            serializer
                .skip_array_field(
                    "internalToRootAttributeIdMap",
                    &self.m_internalToRootAttributeIdMap,
                    TypeSize::NonPtr,
                )?;
            serializer.skip_field("nextUniqueId", &self.m_nextUniqueId)?;
            serializer.skip_field("isActive", &self.m_isActive)?;
            serializer.skip_field("isLinked", &self.m_isLinked)?;
            serializer.skip_field("updateActiveNodes", &self.m_updateActiveNodes)?;
            serializer
                .skip_field(
                    "updateActiveNodesForEnable",
                    &self.m_updateActiveNodesForEnable,
                )?;
            serializer.skip_field("checkNodeValidity", &self.m_checkNodeValidity)?;
            serializer
                .skip_field(
                    "stateOrTransitionChanged",
                    &self.m_stateOrTransitionChanged,
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
    impl<'de> _serde::Deserialize<'de> for hkbBehaviorGraph<'de> {
        fn deserialize<__D>(deserializer: __D) -> core::result::Result<Self, __D::Error>
        where
            __D: _serde::Deserializer<'de>,
        {
            #[allow(non_camel_case_types)]
            enum __Field {
                m_variableBindingSet,
                m_userData,
                m_name,
                m_variableMode,
                m_rootGenerator,
                m_data,
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
                        "variableMode" => Ok(__Field::m_variableMode),
                        "rootGenerator" => Ok(__Field::m_rootGenerator),
                        "data" => Ok(__Field::m_data),
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
            struct __hkbBehaviorGraphVisitor<'de> {
                marker: _serde::__private::PhantomData<hkbBehaviorGraph<'de>>,
                lifetime: _serde::__private::PhantomData<&'de ()>,
            }
            #[allow(clippy::match_single_binding)]
            #[allow(clippy::reversed_empty_ranges)]
            #[allow(clippy::single_match)]
            impl<'de> _serde::de::Visitor<'de> for __hkbBehaviorGraphVisitor<'de> {
                type Value = hkbBehaviorGraph<'de>;
                fn expecting(
                    &self,
                    __formatter: &mut core::fmt::Formatter,
                ) -> core::fmt::Result {
                    core::fmt::Formatter::write_str(
                        __formatter,
                        "struct hkbBehaviorGraph",
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
                    let mut m_variableMode: _serde::__private::Option<VariableMode> = _serde::__private::None;
                    let mut m_uniqueIdPool: _serde::__private::Option<Vec<()>> = _serde::__private::None;
                    let mut m_idToStateMachineTemplateMap: _serde::__private::Option<
                        Pointer<'de>,
                    > = _serde::__private::None;
                    let mut m_mirroredExternalIdMap: _serde::__private::Option<
                        Vec<()>,
                    > = _serde::__private::None;
                    let mut m_pseudoRandomGenerator: _serde::__private::Option<
                        Pointer<'de>,
                    > = _serde::__private::None;
                    let mut m_rootGenerator: _serde::__private::Option<Pointer<'de>> = _serde::__private::None;
                    let mut m_data: _serde::__private::Option<Pointer<'de>> = _serde::__private::None;
                    let mut m_template: _serde::__private::Option<Pointer<'de>> = _serde::__private::None;
                    let mut m_rootGeneratorClone: _serde::__private::Option<
                        Pointer<'de>,
                    > = _serde::__private::None;
                    let mut m_activeNodes: _serde::__private::Option<Pointer<'de>> = _serde::__private::None;
                    let mut m_globalTransitionData: _serde::__private::Option<
                        Pointer<'de>,
                    > = _serde::__private::None;
                    let mut m_eventIdMap: _serde::__private::Option<Pointer<'de>> = _serde::__private::None;
                    let mut m_attributeIdMap: _serde::__private::Option<Pointer<'de>> = _serde::__private::None;
                    let mut m_variableIdMap: _serde::__private::Option<Pointer<'de>> = _serde::__private::None;
                    let mut m_characterPropertyIdMap: _serde::__private::Option<
                        Pointer<'de>,
                    > = _serde::__private::None;
                    let mut m_variableValueSet: _serde::__private::Option<
                        Pointer<'de>,
                    > = _serde::__private::None;
                    let mut m_nodeTemplateToCloneMap: _serde::__private::Option<
                        Pointer<'de>,
                    > = _serde::__private::None;
                    let mut m_stateListenerTemplateToCloneMap: _serde::__private::Option<
                        Pointer<'de>,
                    > = _serde::__private::None;
                    let mut m_recentlyCreatedClones: _serde::__private::Option<
                        Vec<Pointer<'de>>,
                    > = _serde::__private::None;
                    let mut m_nodePartitionInfo: _serde::__private::Option<
                        Pointer<'de>,
                    > = _serde::__private::None;
                    let mut m_numIntermediateOutputs: _serde::__private::Option<
                        I32<'de>,
                    > = _serde::__private::None;
                    let mut m_intermediateOutputSizes: _serde::__private::Option<
                        Vec<()>,
                    > = _serde::__private::None;
                    let mut m_jobs: _serde::__private::Option<Vec<Pointer<'de>>> = _serde::__private::None;
                    let mut m_allPartitionMemory: _serde::__private::Option<
                        Vec<Pointer<'de>>,
                    > = _serde::__private::None;
                    let mut m_internalToRootVariableIdMap: _serde::__private::Option<
                        Vec<()>,
                    > = _serde::__private::None;
                    let mut m_internalToCharacterCharacterPropertyIdMap: _serde::__private::Option<
                        Vec<()>,
                    > = _serde::__private::None;
                    let mut m_internalToRootAttributeIdMap: _serde::__private::Option<
                        Vec<()>,
                    > = _serde::__private::None;
                    let mut m_nextUniqueId: _serde::__private::Option<U16<'de>> = _serde::__private::None;
                    let mut m_isActive: _serde::__private::Option<bool> = _serde::__private::None;
                    let mut m_isLinked: _serde::__private::Option<bool> = _serde::__private::None;
                    let mut m_updateActiveNodes: _serde::__private::Option<bool> = _serde::__private::None;
                    let mut m_updateActiveNodesForEnable: _serde::__private::Option<
                        bool,
                    > = _serde::__private::None;
                    let mut m_checkNodeValidity: _serde::__private::Option<bool> = _serde::__private::None;
                    let mut m_stateOrTransitionChanged: _serde::__private::Option<
                        bool,
                    > = _serde::__private::None;
                    for i in 0..34usize {
                        match i {
                            0usize => {
                                if _serde::__private::Option::is_some(&m_variableMode) {
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "variableMode",
                                        ),
                                    );
                                }
                                m_variableMode = _serde::__private::Some(
                                    match __A::next_value::<VariableMode>(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            1usize => {
                                if _serde::__private::Option::is_some(&m_uniqueIdPool) {
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "uniqueIdPool",
                                        ),
                                    );
                                }
                                __A::pad(&mut __map, 0usize, 7usize)?;
                                m_uniqueIdPool = _serde::__private::Some(
                                    match __A::next_value::<Vec<()>>(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            2usize => {
                                if _serde::__private::Option::is_some(
                                    &m_idToStateMachineTemplateMap,
                                ) {
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "idToStateMachineTemplateMap",
                                        ),
                                    );
                                }
                                m_idToStateMachineTemplateMap = _serde::__private::Some(
                                    match __A::next_value::<Pointer<'de>>(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            3usize => {
                                if _serde::__private::Option::is_some(
                                    &m_mirroredExternalIdMap,
                                ) {
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "mirroredExternalIdMap",
                                        ),
                                    );
                                }
                                m_mirroredExternalIdMap = _serde::__private::Some(
                                    match __A::next_value::<Vec<()>>(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            4usize => {
                                if _serde::__private::Option::is_some(
                                    &m_pseudoRandomGenerator,
                                ) {
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "pseudoRandomGenerator",
                                        ),
                                    );
                                }
                                m_pseudoRandomGenerator = _serde::__private::Some(
                                    match __A::next_value::<Pointer<'de>>(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            5usize => {
                                if _serde::__private::Option::is_some(&m_rootGenerator) {
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "rootGenerator",
                                        ),
                                    );
                                }
                                m_rootGenerator = _serde::__private::Some(
                                    match __A::next_value::<Pointer<'de>>(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            6usize => {
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
                            7usize => {
                                if _serde::__private::Option::is_some(&m_template) {
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "template",
                                        ),
                                    );
                                }
                                m_template = _serde::__private::Some(
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
                                    &m_rootGeneratorClone,
                                ) {
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "rootGeneratorClone",
                                        ),
                                    );
                                }
                                m_rootGeneratorClone = _serde::__private::Some(
                                    match __A::next_value::<Pointer<'de>>(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            9usize => {
                                if _serde::__private::Option::is_some(&m_activeNodes) {
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "activeNodes",
                                        ),
                                    );
                                }
                                m_activeNodes = _serde::__private::Some(
                                    match __A::next_value::<Pointer<'de>>(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            10usize => {
                                if _serde::__private::Option::is_some(
                                    &m_globalTransitionData,
                                ) {
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "globalTransitionData",
                                        ),
                                    );
                                }
                                m_globalTransitionData = _serde::__private::Some(
                                    match __A::next_value::<Pointer<'de>>(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            11usize => {
                                if _serde::__private::Option::is_some(&m_eventIdMap) {
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "eventIdMap",
                                        ),
                                    );
                                }
                                m_eventIdMap = _serde::__private::Some(
                                    match __A::next_value::<Pointer<'de>>(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            12usize => {
                                if _serde::__private::Option::is_some(&m_attributeIdMap) {
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "attributeIdMap",
                                        ),
                                    );
                                }
                                m_attributeIdMap = _serde::__private::Some(
                                    match __A::next_value::<Pointer<'de>>(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            13usize => {
                                if _serde::__private::Option::is_some(&m_variableIdMap) {
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "variableIdMap",
                                        ),
                                    );
                                }
                                m_variableIdMap = _serde::__private::Some(
                                    match __A::next_value::<Pointer<'de>>(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            14usize => {
                                if _serde::__private::Option::is_some(
                                    &m_characterPropertyIdMap,
                                ) {
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "characterPropertyIdMap",
                                        ),
                                    );
                                }
                                m_characterPropertyIdMap = _serde::__private::Some(
                                    match __A::next_value::<Pointer<'de>>(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            15usize => {
                                if _serde::__private::Option::is_some(&m_variableValueSet) {
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "variableValueSet",
                                        ),
                                    );
                                }
                                m_variableValueSet = _serde::__private::Some(
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
                                    &m_nodeTemplateToCloneMap,
                                ) {
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "nodeTemplateToCloneMap",
                                        ),
                                    );
                                }
                                m_nodeTemplateToCloneMap = _serde::__private::Some(
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
                                    &m_stateListenerTemplateToCloneMap,
                                ) {
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "stateListenerTemplateToCloneMap",
                                        ),
                                    );
                                }
                                m_stateListenerTemplateToCloneMap = _serde::__private::Some(
                                    match __A::next_value::<Pointer<'de>>(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            18usize => {
                                if _serde::__private::Option::is_some(
                                    &m_recentlyCreatedClones,
                                ) {
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "recentlyCreatedClones",
                                        ),
                                    );
                                }
                                m_recentlyCreatedClones = _serde::__private::Some(
                                    match __A::next_value::<Vec<Pointer<'de>>>(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            19usize => {
                                if _serde::__private::Option::is_some(
                                    &m_nodePartitionInfo,
                                ) {
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "nodePartitionInfo",
                                        ),
                                    );
                                }
                                m_nodePartitionInfo = _serde::__private::Some(
                                    match __A::next_value::<Pointer<'de>>(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            20usize => {
                                if _serde::__private::Option::is_some(
                                    &m_numIntermediateOutputs,
                                ) {
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "numIntermediateOutputs",
                                        ),
                                    );
                                }
                                m_numIntermediateOutputs = _serde::__private::Some(
                                    match __A::next_value::<I32<'de>>(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            21usize => {
                                if _serde::__private::Option::is_some(
                                    &m_intermediateOutputSizes,
                                ) {
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "intermediateOutputSizes",
                                        ),
                                    );
                                }
                                __A::pad(&mut __map, 0usize, 4usize)?;
                                m_intermediateOutputSizes = _serde::__private::Some(
                                    match __A::next_value::<Vec<()>>(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            22usize => {
                                if _serde::__private::Option::is_some(&m_jobs) {
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field("jobs"),
                                    );
                                }
                                m_jobs = _serde::__private::Some(
                                    match __A::next_value::<Vec<Pointer<'de>>>(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            23usize => {
                                if _serde::__private::Option::is_some(
                                    &m_allPartitionMemory,
                                ) {
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "allPartitionMemory",
                                        ),
                                    );
                                }
                                m_allPartitionMemory = _serde::__private::Some(
                                    match __A::next_value::<Vec<Pointer<'de>>>(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            24usize => {
                                if _serde::__private::Option::is_some(
                                    &m_internalToRootVariableIdMap,
                                ) {
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "internalToRootVariableIdMap",
                                        ),
                                    );
                                }
                                m_internalToRootVariableIdMap = _serde::__private::Some(
                                    match __A::next_value::<Vec<()>>(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            25usize => {
                                if _serde::__private::Option::is_some(
                                    &m_internalToCharacterCharacterPropertyIdMap,
                                ) {
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "internalToCharacterCharacterPropertyIdMap",
                                        ),
                                    );
                                }
                                m_internalToCharacterCharacterPropertyIdMap = _serde::__private::Some(
                                    match __A::next_value::<Vec<()>>(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            26usize => {
                                if _serde::__private::Option::is_some(
                                    &m_internalToRootAttributeIdMap,
                                ) {
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "internalToRootAttributeIdMap",
                                        ),
                                    );
                                }
                                m_internalToRootAttributeIdMap = _serde::__private::Some(
                                    match __A::next_value::<Vec<()>>(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            27usize => {
                                if _serde::__private::Option::is_some(&m_nextUniqueId) {
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "nextUniqueId",
                                        ),
                                    );
                                }
                                m_nextUniqueId = _serde::__private::Some(
                                    match __A::next_value::<U16<'de>>(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            28usize => {
                                if _serde::__private::Option::is_some(&m_isActive) {
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "isActive",
                                        ),
                                    );
                                }
                                m_isActive = _serde::__private::Some(
                                    match __A::next_value::<bool>(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            29usize => {
                                if _serde::__private::Option::is_some(&m_isLinked) {
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "isLinked",
                                        ),
                                    );
                                }
                                m_isLinked = _serde::__private::Some(
                                    match __A::next_value::<bool>(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            30usize => {
                                if _serde::__private::Option::is_some(
                                    &m_updateActiveNodes,
                                ) {
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "updateActiveNodes",
                                        ),
                                    );
                                }
                                m_updateActiveNodes = _serde::__private::Some(
                                    match __A::next_value::<bool>(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            31usize => {
                                if _serde::__private::Option::is_some(
                                    &m_updateActiveNodesForEnable,
                                ) {
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "updateActiveNodesForEnable",
                                        ),
                                    );
                                }
                                m_updateActiveNodesForEnable = _serde::__private::Some(
                                    match __A::next_value::<bool>(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            32usize => {
                                if _serde::__private::Option::is_some(
                                    &m_checkNodeValidity,
                                ) {
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "checkNodeValidity",
                                        ),
                                    );
                                }
                                m_checkNodeValidity = _serde::__private::Some(
                                    match __A::next_value::<bool>(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            33usize => {
                                if _serde::__private::Option::is_some(
                                    &m_stateOrTransitionChanged,
                                ) {
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "stateOrTransitionChanged",
                                        ),
                                    );
                                }
                                m_stateOrTransitionChanged = _serde::__private::Some(
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
                    let m_variableMode = match m_variableMode {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "variableMode",
                                ),
                            );
                        }
                    };
                    let m_uniqueIdPool = match m_uniqueIdPool {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "uniqueIdPool",
                                ),
                            );
                        }
                    };
                    let m_idToStateMachineTemplateMap = match m_idToStateMachineTemplateMap {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "idToStateMachineTemplateMap",
                                ),
                            );
                        }
                    };
                    let m_mirroredExternalIdMap = match m_mirroredExternalIdMap {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "mirroredExternalIdMap",
                                ),
                            );
                        }
                    };
                    let m_pseudoRandomGenerator = match m_pseudoRandomGenerator {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "pseudoRandomGenerator",
                                ),
                            );
                        }
                    };
                    let m_rootGenerator = match m_rootGenerator {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "rootGenerator",
                                ),
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
                    let m_template = match m_template {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field("template"),
                            );
                        }
                    };
                    let m_rootGeneratorClone = match m_rootGeneratorClone {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "rootGeneratorClone",
                                ),
                            );
                        }
                    };
                    let m_activeNodes = match m_activeNodes {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "activeNodes",
                                ),
                            );
                        }
                    };
                    let m_globalTransitionData = match m_globalTransitionData {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "globalTransitionData",
                                ),
                            );
                        }
                    };
                    let m_eventIdMap = match m_eventIdMap {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "eventIdMap",
                                ),
                            );
                        }
                    };
                    let m_attributeIdMap = match m_attributeIdMap {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "attributeIdMap",
                                ),
                            );
                        }
                    };
                    let m_variableIdMap = match m_variableIdMap {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "variableIdMap",
                                ),
                            );
                        }
                    };
                    let m_characterPropertyIdMap = match m_characterPropertyIdMap {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "characterPropertyIdMap",
                                ),
                            );
                        }
                    };
                    let m_variableValueSet = match m_variableValueSet {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "variableValueSet",
                                ),
                            );
                        }
                    };
                    let m_nodeTemplateToCloneMap = match m_nodeTemplateToCloneMap {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "nodeTemplateToCloneMap",
                                ),
                            );
                        }
                    };
                    let m_stateListenerTemplateToCloneMap = match m_stateListenerTemplateToCloneMap {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "stateListenerTemplateToCloneMap",
                                ),
                            );
                        }
                    };
                    let m_recentlyCreatedClones = match m_recentlyCreatedClones {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "recentlyCreatedClones",
                                ),
                            );
                        }
                    };
                    let m_nodePartitionInfo = match m_nodePartitionInfo {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "nodePartitionInfo",
                                ),
                            );
                        }
                    };
                    let m_numIntermediateOutputs = match m_numIntermediateOutputs {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "numIntermediateOutputs",
                                ),
                            );
                        }
                    };
                    let m_intermediateOutputSizes = match m_intermediateOutputSizes {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "intermediateOutputSizes",
                                ),
                            );
                        }
                    };
                    let m_jobs = match m_jobs {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field("jobs"),
                            );
                        }
                    };
                    let m_allPartitionMemory = match m_allPartitionMemory {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "allPartitionMemory",
                                ),
                            );
                        }
                    };
                    let m_internalToRootVariableIdMap = match m_internalToRootVariableIdMap {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "internalToRootVariableIdMap",
                                ),
                            );
                        }
                    };
                    let m_internalToCharacterCharacterPropertyIdMap = match m_internalToCharacterCharacterPropertyIdMap {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "internalToCharacterCharacterPropertyIdMap",
                                ),
                            );
                        }
                    };
                    let m_internalToRootAttributeIdMap = match m_internalToRootAttributeIdMap {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "internalToRootAttributeIdMap",
                                ),
                            );
                        }
                    };
                    let m_nextUniqueId = match m_nextUniqueId {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "nextUniqueId",
                                ),
                            );
                        }
                    };
                    let m_isActive = match m_isActive {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field("isActive"),
                            );
                        }
                    };
                    let m_isLinked = match m_isLinked {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field("isLinked"),
                            );
                        }
                    };
                    let m_updateActiveNodes = match m_updateActiveNodes {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "updateActiveNodes",
                                ),
                            );
                        }
                    };
                    let m_updateActiveNodesForEnable = match m_updateActiveNodesForEnable {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "updateActiveNodesForEnable",
                                ),
                            );
                        }
                    };
                    let m_checkNodeValidity = match m_checkNodeValidity {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "checkNodeValidity",
                                ),
                            );
                        }
                    };
                    let m_stateOrTransitionChanged = match m_stateOrTransitionChanged {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "stateOrTransitionChanged",
                                ),
                            );
                        }
                    };
                    _serde::__private::Ok(hkbBehaviorGraph {
                        __ptr,
                        parent,
                        m_variableMode,
                        m_uniqueIdPool,
                        m_idToStateMachineTemplateMap,
                        m_mirroredExternalIdMap,
                        m_pseudoRandomGenerator,
                        m_rootGenerator,
                        m_data,
                        m_template,
                        m_rootGeneratorClone,
                        m_activeNodes,
                        m_globalTransitionData,
                        m_eventIdMap,
                        m_attributeIdMap,
                        m_variableIdMap,
                        m_characterPropertyIdMap,
                        m_variableValueSet,
                        m_nodeTemplateToCloneMap,
                        m_stateListenerTemplateToCloneMap,
                        m_recentlyCreatedClones,
                        m_nodePartitionInfo,
                        m_numIntermediateOutputs,
                        m_intermediateOutputSizes,
                        m_jobs,
                        m_allPartitionMemory,
                        m_internalToRootVariableIdMap,
                        m_internalToCharacterCharacterPropertyIdMap,
                        m_internalToRootAttributeIdMap,
                        m_nextUniqueId,
                        m_isActive,
                        m_isLinked,
                        m_updateActiveNodes,
                        m_updateActiveNodesForEnable,
                        m_checkNodeValidity,
                        m_stateOrTransitionChanged,
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
                    let mut m_variableMode: _serde::__private::Option<VariableMode> = _serde::__private::None;
                    let mut m_rootGenerator: _serde::__private::Option<Pointer<'de>> = _serde::__private::None;
                    let mut m_data: _serde::__private::Option<Pointer<'de>> = _serde::__private::None;
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
                            __Field::m_variableMode => {
                                #[cfg(
                                    any(feature = "strict", feature = "ignore_duplicates")
                                )]
                                if _serde::__private::Option::is_some(&m_variableMode) {
                                    #[cfg(feature = "ignore_duplicates")]
                                    {
                                        __A::skip_value(&mut __map)?;
                                        continue;
                                    }
                                    #[cfg(feature = "strict")]
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "variableMode",
                                        ),
                                    );
                                }
                                m_variableMode = _serde::__private::Some(
                                    match __A::next_value::<VariableMode>(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            __Field::m_rootGenerator => {
                                #[cfg(
                                    any(feature = "strict", feature = "ignore_duplicates")
                                )]
                                if _serde::__private::Option::is_some(&m_rootGenerator) {
                                    #[cfg(feature = "ignore_duplicates")]
                                    {
                                        __A::skip_value(&mut __map)?;
                                        continue;
                                    }
                                    #[cfg(feature = "strict")]
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "rootGenerator",
                                        ),
                                    );
                                }
                                m_rootGenerator = _serde::__private::Some(
                                    match __A::next_value::<Pointer<'de>>(&mut __map) {
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
                    let m_variableMode = match m_variableMode {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            #[cfg(feature = "strict")]
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "variableMode",
                                ),
                            );
                            #[cfg(not(feature = "strict"))] Default::default()
                        }
                    };
                    let m_rootGenerator = match m_rootGenerator {
                        _serde::__private::Some(__field) => __field,
                        _serde::__private::None => {
                            #[cfg(feature = "strict")]
                            return _serde::__private::Err(
                                <__A::Error as _serde::de::Error>::missing_field(
                                    "rootGenerator",
                                ),
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
                    let parent = hkbGenerator {
                        __ptr: __ptr.clone(),
                        parent,
                        ..Default::default()
                    };
                    let __ptr = __A::class_ptr(&mut __map);
                    _serde::__private::Ok(hkbBehaviorGraph {
                        __ptr: __ptr.clone(),
                        parent,
                        m_variableMode,
                        m_rootGenerator,
                        m_data,
                        ..Default::default()
                    })
                }
            }
            const FIELDS: &[&str] = &[
                "variableMode",
                "uniqueIdPool",
                "idToStateMachineTemplateMap",
                "mirroredExternalIdMap",
                "pseudoRandomGenerator",
                "rootGenerator",
                "data",
                "template",
                "rootGeneratorClone",
                "activeNodes",
                "globalTransitionData",
                "eventIdMap",
                "attributeIdMap",
                "variableIdMap",
                "characterPropertyIdMap",
                "variableValueSet",
                "nodeTemplateToCloneMap",
                "stateListenerTemplateToCloneMap",
                "recentlyCreatedClones",
                "nodePartitionInfo",
                "numIntermediateOutputs",
                "intermediateOutputSizes",
                "jobs",
                "allPartitionMemory",
                "internalToRootVariableIdMap",
                "internalToCharacterCharacterPropertyIdMap",
                "internalToRootAttributeIdMap",
                "nextUniqueId",
                "isActive",
                "isLinked",
                "updateActiveNodes",
                "updateActiveNodesForEnable",
                "checkNodeValidity",
                "stateOrTransitionChanged",
            ];
            _serde::Deserializer::deserialize_struct(
                deserializer,
                "hkbBehaviorGraph",
                FIELDS,
                __hkbBehaviorGraphVisitor {
                    marker: _serde::__private::PhantomData::<hkbBehaviorGraph>,
                    lifetime: _serde::__private::PhantomData,
                },
            )
        }
    }
};
/// # C++ Info
/// - name: `VariableMode`(ctype: `hkEnum<VariableMode, hkInt8>`)
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
pub enum VariableMode {
    #[default]
    VARIABLE_MODE_DISCARD_WHEN_INACTIVE = 0isize,
    VARIABLE_MODE_MAINTAIN_VALUES_WHEN_INACTIVE = 1isize,
}
const _: () = {
    use havok_serde as __serde;
    impl __serde::Serialize for VariableMode {
        fn serialize<S>(&self, __serializer: S) -> Result<S::Ok, S::Error>
        where
            S: __serde::ser::Serializer,
        {
            let mut __serializer = __serializer.serialize_enum_flags()?;
            match self {
                Self::VARIABLE_MODE_DISCARD_WHEN_INACTIVE => {
                    __serializer
                        .serialize_field("VARIABLE_MODE_DISCARD_WHEN_INACTIVE", &0u64)
                }
                Self::VARIABLE_MODE_MAINTAIN_VALUES_WHEN_INACTIVE => {
                    __serializer
                        .serialize_field(
                            "VARIABLE_MODE_MAINTAIN_VALUES_WHEN_INACTIVE",
                            &1u64,
                        )
                }
            }?;
            use num_traits::ToPrimitive as _;
            let num = self
                .to_i8()
                .ok_or(S::Error::custom("Failed enum VariableMode to_i8"))?;
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
    impl<'de> _serde::Deserialize<'de> for VariableMode {
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
                fn visit_int8<__E>(
                    self,
                    __value: I8<'de>,
                ) -> _serde::__private::Result<Self::Value, __E>
                where
                    __E: _serde::de::Error,
                {
                    match __value {
                        I8::Number(0i8) => _serde::__private::Ok(__Field::__field0),
                        I8::Number(1i8) => _serde::__private::Ok(__Field::__field1),
                        _ => {
                            _serde::__private::Err(
                                _serde::de::Error::invalid_value(
                                    _serde::de::Unexpected::Int8(__value),
                                    &"value(i8) of variant is one of 0, 1",
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
                                || v
                                    .eq_ignore_ascii_case(
                                        "VARIABLE_MODE_DISCARD_WHEN_INACTIVE",
                                    ) => _serde::__private::Ok(__Field::__field0),
                            v if v == "1"
                                || v
                                    .eq_ignore_ascii_case(
                                        "VARIABLE_MODE_MAINTAIN_VALUES_WHEN_INACTIVE",
                                    ) => _serde::__private::Ok(__Field::__field1),
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
                        _serde::de::ReadEnumSize::Int8,
                        __FieldVisitor,
                    )
                }
            }
            #[doc(hidden)]
            struct __Visitor<'de> {
                marker: _serde::__private::PhantomData<VariableMode>,
                lifetime: _serde::__private::PhantomData<&'de ()>,
            }
            impl<'de> _serde::de::Visitor<'de> for __Visitor<'de> {
                type Value = VariableMode;
                fn expecting(
                    &self,
                    __formatter: &mut _serde::__private::Formatter,
                ) -> _serde::__private::fmt::Result {
                    _serde::__private::Formatter::write_str(
                        __formatter,
                        "enum VariableMode",
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
                                VariableMode::VARIABLE_MODE_DISCARD_WHEN_INACTIVE,
                            )
                        }
                        (__Field::__field1, __variant) => {
                            _serde::de::VariantAccess::unit_variant(__variant)?;
                            _serde::__private::Ok(
                                VariableMode::VARIABLE_MODE_MAINTAIN_VALUES_WHEN_INACTIVE,
                            )
                        }
                    }
                }
            }
            #[doc(hidden)]
            const VARIANTS: &'static [&'static str] = &[
                "VARIABLE_MODE_DISCARD_WHEN_INACTIVE",
                "VARIABLE_MODE_MAINTAIN_VALUES_WHEN_INACTIVE",
            ];
            _serde::Deserializer::deserialize_enum(
                __deserializer,
                "VariableMode",
                VARIANTS,
                __Visitor {
                    marker: _serde::__private::PhantomData::<VariableMode>,
                    lifetime: _serde::__private::PhantomData,
                },
            )
        }
    }
};
