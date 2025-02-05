// This file is part of the Polymesh distribution (https://github.com/PolymeshAssociation/Polymesh).
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

use rand_core::{CryptoRng, Error, RngCore};
use sp_runtime_interface::runtime_interface;

#[cfg(not(feature = "std"))]
use core::num::NonZeroU32;

#[cfg(feature = "std")]
use rand::rngs::OsRng;

/// Host functions to allow native access to OS RNG from Wasm.
///
/// # Change Log
/// - Runtime Spec version 1005. It is incompatible with previous versions, and it requires a
/// binary update.
///
/// # TODO
///  - Use lazy static RNG object to mitigate performance impact during the `OsRng` object
///  creation. We should double-check the security implications of having a global RNG in a
///  multi-threading environment.
#[runtime_interface]
pub trait NativeRng {
    /// Returns the next random `u32` generated by `OsRng`.
    fn next_u32() -> u32 {
        OsRng::default().next_u32()
    }

    /// Returns the next random `u64` generated by `OsRng`.
    fn next_u64() -> u64 {
        OsRng::default().next_u64()
    }

    /// Fills `dest` with random data, using `OsRng`.
    fn fill_bytes(dest: &mut [u8]) {
        OsRng::default().fill_bytes(dest)
    }

    /// Fills `dest` entirely with random data, using `OsRng`.
    fn try_fill_bytes(&mut self, dest: &mut [u8]) -> i32 {
        match OsRng::default().try_fill_bytes(dest) {
            Ok(..) => 0,
            Err(err) => err.raw_os_error().unwrap_or(1),
        }
    }
}

/// Wrapper over `OsRng` which supports WASM using `NativeRng` host functions.
#[derive(Clone, Default)]
pub struct Rng {
    #[cfg(feature = "std")]
    inner: OsRng,
}

impl RngCore for Rng {
    #[cfg(feature = "std")]
    fn next_u32(&mut self) -> u32 {
        self.inner.next_u32()
    }

    #[cfg(not(feature = "std"))]
    fn next_u32(&mut self) -> u32 {
        native_rng::next_u32()
    }

    #[cfg(feature = "std")]
    fn next_u64(&mut self) -> u64 {
        self.inner.next_u64()
    }

    #[cfg(not(feature = "std"))]
    fn next_u64(&mut self) -> u64 {
        native_rng::next_u64()
    }

    #[cfg(feature = "std")]
    fn fill_bytes(&mut self, dest: &mut [u8]) {
        self.inner.fill_bytes(dest)
    }

    #[cfg(not(feature = "std"))]
    fn fill_bytes(&mut self, dest: &mut [u8]) {
        native_rng::fill_bytes(dest);
    }

    #[cfg(feature = "std")]
    fn try_fill_bytes(&mut self, dest: &mut [u8]) -> Result<(), Error> {
        self.inner.try_fill_bytes(dest)
    }

    #[cfg(not(feature = "std"))]
    fn try_fill_bytes(&mut self, dest: &mut [u8]) -> Result<(), Error> {
        match NonZeroU32::new(native_rng::try_fill_bytes(dest) as u32) {
            None => Ok(()),
            Some(nz_code) => Err(Error::from(nz_code)),
        }
    }
}

impl CryptoRng for Rng {}
