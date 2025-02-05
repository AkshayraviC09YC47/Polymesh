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

//! Autogenerated weights for pallet_portfolio
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
// -p=pallet_portfolio
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

/// Weights for pallet_portfolio using the Substrate node and recommended hardware.
pub struct WeightInfo;
impl pallet_portfolio::WeightInfo for WeightInfo {
    // Storage: Identity KeyRecords (r:1 w:0)
    // Storage: Portfolio NameToNumber (r:1 w:1)
    // Storage: Portfolio NextPortfolioNumber (r:1 w:1)
    // Storage: Portfolio Portfolios (r:0 w:1)
    fn create_portfolio() -> Weight {
        (55_827_000 as Weight)
            .saturating_add(DbWeight::get().reads(3 as Weight))
            .saturating_add(DbWeight::get().writes(3 as Weight))
    }
    // Storage: Identity KeyRecords (r:1 w:0)
    // Storage: Portfolio PortfolioAssetCount (r:1 w:1)
    // Storage: Portfolio Portfolios (r:1 w:1)
    // Storage: Portfolio PortfolioCustodian (r:1 w:1)
    // Storage: Portfolio PortfoliosInCustody (r:0 w:1)
    // Storage: Portfolio NameToNumber (r:0 w:1)
    fn delete_portfolio() -> Weight {
        (82_934_000 as Weight)
            .saturating_add(DbWeight::get().reads(4 as Weight))
            .saturating_add(DbWeight::get().writes(5 as Weight))
    }
    // Storage: Identity KeyRecords (r:1 w:0)
    // Storage: Portfolio PortfolioCustodian (r:1 w:0)
    // Storage: Portfolio Portfolios (r:1 w:0)
    // Storage: Portfolio PortfolioAssetBalances (r:2 w:2)
    // Storage: Portfolio PortfolioLockedAssets (r:1 w:0)
    // Storage: Portfolio PortfolioAssetCount (r:2 w:2)
    fn move_portfolio_funds(a: u32) -> Weight {
        (240_712_000 as Weight)
            // Standard Error: 397_000
            .saturating_add((38_563_000 as Weight).saturating_mul(a as Weight))
            .saturating_add(DbWeight::get().reads(5 as Weight))
            .saturating_add(DbWeight::get().reads((3 as Weight).saturating_mul(a as Weight)))
            .saturating_add(DbWeight::get().writes(2 as Weight))
            .saturating_add(DbWeight::get().writes((2 as Weight).saturating_mul(a as Weight)))
    }
    // Storage: Identity KeyRecords (r:1 w:0)
    // Storage: Portfolio Portfolios (r:1 w:1)
    // Storage: Portfolio NameToNumber (r:1 w:1)
    fn rename_portfolio(i: u32) -> Weight {
        (61_140_000 as Weight)
            // Standard Error: 3_000
            .saturating_add((17_000 as Weight).saturating_mul(i as Weight))
            .saturating_add(DbWeight::get().reads(3 as Weight))
            .saturating_add(DbWeight::get().writes(2 as Weight))
    }
    // Storage: Identity KeyRecords (r:1 w:0)
    // Storage: Portfolio PortfolioCustodian (r:1 w:1)
    // Storage: Portfolio PortfoliosInCustody (r:0 w:1)
    fn quit_portfolio_custody() -> Weight {
        (51_711_000 as Weight)
            .saturating_add(DbWeight::get().reads(2 as Weight))
            .saturating_add(DbWeight::get().writes(2 as Weight))
    }
    // Storage: Identity KeyRecords (r:1 w:0)
    // Storage: Identity Authorizations (r:1 w:1)
    // Storage: Portfolio PortfolioCustodian (r:1 w:1)
    // Storage: Portfolio PortfoliosInCustody (r:0 w:2)
    // Storage: Identity AuthorizationsGiven (r:0 w:1)
    fn accept_portfolio_custody() -> Weight {
        (75_530_000 as Weight)
            .saturating_add(DbWeight::get().reads(3 as Weight))
            .saturating_add(DbWeight::get().writes(5 as Weight))
    }
}
