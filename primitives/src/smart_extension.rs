// This file is part of the Polymesh distribution (https://github.com/PolymathNetwork/Polymesh).
// Copyright (c) 2020 Polymath

// This program is free software: you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation, version 3.

// This program is distributed in the hope that it will be useful, but
// WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE. See the GNU
// General Public License for more details.

// You should have received a copy of the GNU General Public License
// along with this program. If not, see <http://www.gnu.org/licenses/>.

use codec::{Decode, Encode};
use polymesh_primitives_derive::VecU8StrongTyped;
use sp_std::prelude::Vec;
/// Smart Extension types
#[allow(missing_docs)]
#[derive(Encode, Decode, Clone, PartialEq, Eq, Debug, PartialOrd, Ord)]
pub enum SmartExtensionType {
    TransferManager,
    Offerings,
    SmartWallet,
    Custom(Vec<u8>),
}

impl Default for SmartExtensionType {
    fn default() -> Self {
        SmartExtensionType::Custom(b"undefined".to_vec())
    }
}

/// A wrapper for a smart extension name.
#[derive(
    Decode, Encode, Clone, Debug, Default, Hash, PartialEq, Eq, PartialOrd, Ord, VecU8StrongTyped,
)]
pub struct SmartExtensionName(pub Vec<u8>);

#[derive(Encode, Decode, Default, Clone, PartialEq, Debug)]
/// Smart Extension details when SE instance
/// is attached with asset.
pub struct SmartExtension<AccountId> {
    /// Type of the extension
    pub extension_type: SmartExtensionType,
    /// Name of extension
    pub extension_name: SmartExtensionName,
    /// AccountId of the smart extension
    pub extension_id: AccountId,
    /// Status of the smart extension
    pub is_archive: bool,
}

/// The url string of the SE template.
#[derive(Encode, Decode, Default, Debug, Clone, PartialEq, VecU8StrongTyped)]
pub struct MetaUrl(pub Vec<u8>);

/// The description string about the SE template.
#[derive(Encode, Decode, Default, Debug, Clone, PartialEq, VecU8StrongTyped)]
pub struct MetaDescription(pub Vec<u8>);

/// The version string of the SE template.
#[derive(Encode, Decode, Default, Debug, Clone, PartialEq, VecU8StrongTyped)]
pub struct MetaVersion(pub Vec<u8>);

/// Subset of the SE template metadata that is provided by the template owner.
#[derive(Encode, Decode, Default, Debug, Clone, PartialEq)]
pub struct SmartExtensionMetadata<Balance> {
    /// Url that can contain the details about the template
    /// Ex- license, audit report.
    pub url: Option<MetaUrl>,
    /// Type of smart extension template.
    pub se_type: SmartExtensionType,
    /// Fee paid at the time on creating new instance form the template.
    pub instantiation_fee: Balance,
    /// Fee paid at the time of usage of the SE (A given operation performed).
    pub usage_fee: Balance,
    /// Description about the SE template.
    pub description: MetaDescription,
    /// Version of the template.
    pub version: MetaVersion,
}

/// Data structure that hold all the relevant metadata of the smart extension template.
#[derive(Encode, Decode, Default, Debug, Clone, PartialEq)]
pub struct TemplateMetadata<Balance, AccountId> {
    /// Meta details of the SE template
    pub meta_info: SmartExtensionMetadata<Balance>,
    /// Owner of the SE template.
    pub owner: AccountId,
    /// power button to switch on/off the instantiation from the template
    pub is_freeze: bool,
}

impl<Balance, AccountId> TemplateMetadata<Balance, AccountId>
where
    Balance: Clone + Copy,
{
    /// Return the instantiation fee
    pub fn get_instantiation_fee(&self) -> Balance {
        self.meta_info.instantiation_fee
    }

    /// Check whether the instantiation of the template is allowed or not.
    pub fn is_instantiation_freezed(&self) -> bool {
        self.is_freeze
    }
}
