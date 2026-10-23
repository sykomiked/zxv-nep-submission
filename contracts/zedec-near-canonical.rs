//! zxv-canonical — the canonical routing contract, deployed on zedec.near.
//! Copyright (c) 2024-2026 Michael Laurence Curzi and 36N9 Genetics, LLC
//! SPDX-License-Identifier: Apache-2.0

use near_sdk::borsh::{BorshDeserialize, BorshSerialize};
use near_sdk::{env, near_bindgen, PanicOnDefault};

#[near_bindgen]
#[derive(BorshDeserialize, BorshSerialize, PanicOnDefault)]
pub struct ZxvCanonical {
    pub registry: String,
    pub gate: String,
    pub core: String,
}

#[near_bindgen]
impl ZxvCanonical {
    #[init]
    pub fn new(registry: String, gate: String, core: String) -> Self {
        Self { registry, gate, core }
    }

    pub fn route(&self) -> String {
        let mut o = String::from("canonical=");
        o.push_str(&env::current_account_id().to_string());
        o.push_str("|registry="); o.push_str(&self.registry);
        o.push_str("|gate="); o.push_str(&self.gate);
        o.push_str("|core="); o.push_str(&self.core);
        o.push_str("|note=route to core for the 110 kits; registry for the 131 capabilities; gate for the invariant");
        o
    }

    pub fn protocol_meta(&self) -> String {
        let mut o = String::from("name=ZXV|fullName=The ZXV Paraconsistent Settlement Layer|primaryWallet=");
        o.push_str(&env::current_account_id().to_string());
        o.push_str("|protocolAddress="); o.push_str(&self.core);
        o.push_str("|invariant=only TRUE settles|kits=110|capabilities=131|tagline=The system is open. The tribute is 11%.");
        o
    }

    pub fn kits(&self) -> String {
        "110 kits: gate, treasury, rails, receipts, vino, tribute, surplus, chronicle, allotment, broker, rmag, scp144, aipi, chiglet, count_house, vswap, vauction, vbond, identity, reputation, provenance, grants, community_chest, royalty, telemetry, market, intent, mesh_token + layers, systems, meta, api, ux, rewards, player, ops, gas station, Port, circulation, pool, decrees".to_string()
    }

    pub fn is_settleable(&self, state: u8) -> bool {
        state == 0x01
    }

    pub fn registry(&self) -> String { self.registry.clone() }
    pub fn gate(&self) -> String { self.gate.clone() }
    pub fn core(&self) -> String { self.core.clone() }
}
