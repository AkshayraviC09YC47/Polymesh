// This file is part of Substrate.

// Copyright (C) 2021 Parity Technologies (UK) Ltd.
// SPDX-License-Identifier: Apache-2.0

// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
// http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

//! Autogenerated weights for pallet_capital_distribution
//!
//! THIS FILE WAS AUTO-GENERATED USING THE SUBSTRATE BENCHMARK CLI VERSION 4.0.0-dev
//! DATE: 2022-06-25, STEPS: `100`, REPEAT: 5, LOW RANGE: `[]`, HIGH RANGE: `[]`
//! EXECUTION: Some(Wasm), WASM-EXECUTION: Compiled, CHAIN: None, DB CACHE: 512

// Executed Command:
// ./target/release/polymesh
// benchmark
// pallet
// -s
// 100
// -r
// 5
// -p=pallet_capital_distribution
// -e=*
// --heap-pages
// 4096
// --db-cache
// 512
// --execution
// wasm
// --wasm-execution
// compiled
// --output
// ./pallets/weights/src/
// --template
// ./.maintain/frame-weight-template.hbs

#![allow(unused_parens)]
#![allow(unused_imports)]

use polymesh_runtime_common::{RocksDbWeight as DbWeight, Weight};

/// Weights for pallet_capital_distribution using the Substrate node and recommended hardware.
pub struct WeightInfo;
impl pallet_corporate_actions::distribution::WeightInfo for WeightInfo {
    // Storage: Identity KeyRecords (r:1 w:0)
    // Storage: ExternalAgents GroupOfAgent (r:1 w:0)
    // Storage: Permissions CurrentPalletName (r:1 w:0)
    // Storage: Permissions CurrentDispatchableName (r:1 w:0)
    // Storage: CapitalDistribution Distributions (r:1 w:1)
    // Storage: Portfolio PortfolioCustodian (r:1 w:0)
    // Storage: Portfolio Portfolios (r:1 w:0)
    // Storage: CorporateAction CorporateActions (r:1 w:0)
    // Storage: Portfolio PortfolioAssetBalances (r:1 w:0)
    // Storage: Portfolio PortfolioLockedAssets (r:1 w:1)
    // Storage: ProtocolFee Coefficient (r:1 w:0)
    // Storage: ProtocolFee BaseFees (r:1 w:0)
    fn distribute() -> Weight {
        (116_871_000 as Weight)
            .saturating_add(DbWeight::get().reads(12 as Weight))
            .saturating_add(DbWeight::get().writes(2 as Weight))
    }
    // Storage: Identity KeyRecords (r:1 w:0)
    // Storage: CapitalDistribution HolderPaid (r:1 w:1)
    // Storage: CapitalDistribution Distributions (r:1 w:1)
    // Storage: Timestamp Now (r:1 w:0)
    // Storage: CorporateAction CorporateActions (r:1 w:0)
    // Storage: Checkpoint SchedulePoints (r:1 w:0)
    // Storage: Asset BalanceOf (r:3 w:2)
    // Storage: Asset Tokens (r:1 w:0)
    // Storage: unknown [0x3a7472616e73616374696f6e5f6c6576656c3a] (r:1 w:1)
    // Storage: Portfolio PortfolioLockedAssets (r:1 w:1)
    // Storage: Asset Frozen (r:1 w:0)
    // Storage: Asset DisableInvestorUniqueness (r:1 w:0)
    // Storage: Identity Claims (r:2 w:0)
    // Storage: Portfolio Portfolios (r:1 w:0)
    // Storage: Portfolio PortfolioAssetBalances (r:2 w:2)
    // Storage: Asset ScopeIdOf (r:2 w:0)
    // Storage: Asset AggregateBalance (r:2 w:2)
    // Storage: Statistics AssetTransferCompliances (r:1 w:0)
    // Storage: ComplianceManager AssetCompliances (r:1 w:0)
    // Storage: Checkpoint Schedules (r:1 w:0)
    // Storage: Checkpoint CheckpointIdSequence (r:1 w:0)
    // Storage: Asset BalanceOfAtScope (r:0 w:2)
    fn claim(t: u32, w: u32) -> Weight {
        (516_215_000 as Weight)
            // Standard Error: 7_000
            .saturating_add((159_000 as Weight).saturating_mul(t as Weight))
            // Standard Error: 7_000
            .saturating_add((81_000 as Weight).saturating_mul(w as Weight))
            .saturating_add(DbWeight::get().reads(27 as Weight))
            .saturating_add(DbWeight::get().writes(12 as Weight))
    }
    // Storage: Identity KeyRecords (r:1 w:0)
    // Storage: ExternalAgents GroupOfAgent (r:1 w:0)
    // Storage: Permissions CurrentPalletName (r:1 w:0)
    // Storage: Permissions CurrentDispatchableName (r:1 w:0)
    // Storage: CapitalDistribution HolderPaid (r:1 w:1)
    // Storage: CapitalDistribution Distributions (r:1 w:1)
    // Storage: Timestamp Now (r:1 w:0)
    // Storage: CorporateAction CorporateActions (r:1 w:0)
    // Storage: Checkpoint SchedulePoints (r:1 w:0)
    // Storage: Asset BalanceOf (r:3 w:2)
    // Storage: Asset Tokens (r:1 w:0)
    // Storage: unknown [0x3a7472616e73616374696f6e5f6c6576656c3a] (r:1 w:1)
    // Storage: Portfolio PortfolioLockedAssets (r:1 w:1)
    // Storage: Asset Frozen (r:1 w:0)
    // Storage: Asset DisableInvestorUniqueness (r:1 w:0)
    // Storage: Identity Claims (r:2 w:0)
    // Storage: Portfolio Portfolios (r:1 w:0)
    // Storage: Portfolio PortfolioAssetBalances (r:2 w:2)
    // Storage: Asset ScopeIdOf (r:2 w:0)
    // Storage: Asset AggregateBalance (r:2 w:2)
    // Storage: Statistics AssetTransferCompliances (r:1 w:0)
    // Storage: ComplianceManager AssetCompliances (r:1 w:0)
    // Storage: Checkpoint Schedules (r:1 w:0)
    // Storage: Checkpoint CheckpointIdSequence (r:1 w:0)
    // Storage: Asset BalanceOfAtScope (r:0 w:2)
    fn push_benefit(t: u32, w: u32) -> Weight {
        (517_263_000 as Weight)
            // Standard Error: 8_000
            .saturating_add((205_000 as Weight).saturating_mul(t as Weight))
            // Standard Error: 8_000
            .saturating_add((95_000 as Weight).saturating_mul(w as Weight))
            .saturating_add(DbWeight::get().reads(30 as Weight))
            .saturating_add(DbWeight::get().writes(12 as Weight))
    }
    // Storage: Identity KeyRecords (r:1 w:0)
    // Storage: ExternalAgents GroupOfAgent (r:1 w:0)
    // Storage: Permissions CurrentPalletName (r:1 w:0)
    // Storage: Permissions CurrentDispatchableName (r:1 w:0)
    // Storage: CapitalDistribution Distributions (r:1 w:1)
    // Storage: Timestamp Now (r:1 w:0)
    // Storage: Portfolio PortfolioCustodian (r:1 w:0)
    // Storage: Portfolio PortfolioLockedAssets (r:1 w:1)
    fn reclaim() -> Weight {
        (101_522_000 as Weight)
            .saturating_add(DbWeight::get().reads(8 as Weight))
            .saturating_add(DbWeight::get().writes(2 as Weight))
    }
    // Storage: Identity KeyRecords (r:1 w:0)
    // Storage: ExternalAgents GroupOfAgent (r:1 w:0)
    // Storage: Permissions CurrentPalletName (r:1 w:0)
    // Storage: Permissions CurrentDispatchableName (r:1 w:0)
    // Storage: CapitalDistribution Distributions (r:1 w:1)
    // Storage: Timestamp Now (r:1 w:0)
    // Storage: Portfolio PortfolioLockedAssets (r:1 w:1)
    fn remove_distribution() -> Weight {
        (96_594_000 as Weight)
            .saturating_add(DbWeight::get().reads(7 as Weight))
            .saturating_add(DbWeight::get().writes(2 as Weight))
    }
}
