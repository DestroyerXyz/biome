//! Generated file, do not edit by hand, see `xtask/codegen`

use biome_analyze::declare_group;

pub(crate) mod no_invalid_use_before_declaration;
pub(crate) mod no_misleading_character_class;
pub(crate) mod no_nodejs_modules;
pub(crate) mod no_then_property;
pub(crate) mod no_unused_imports;
pub(crate) mod use_export_type;
pub(crate) mod use_for_of;
pub(crate) mod use_number_properties;

declare_group! {
    pub (crate) Nursery {
        name : "nursery" ,
        rules : [
            self :: no_invalid_use_before_declaration :: NoInvalidUseBeforeDeclaration ,
            self :: no_misleading_character_class :: NoMisleadingCharacterClass ,
            self :: no_nodejs_modules :: NoNodejsModules ,
            self :: no_then_property :: NoThenProperty ,
            self :: no_unused_imports :: NoUnusedImports ,
            self :: use_export_type :: UseExportType ,
            self :: use_for_of :: UseForOf ,
            self :: use_number_properties :: UseNumberProperties ,
        ]
     }
}
