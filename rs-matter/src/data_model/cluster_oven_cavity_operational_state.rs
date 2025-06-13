/*
 *
 *    Copyright (c) 2020-2022 Project CHIP Authors
 *
 *    Licensed under the Apache License, Version 2.0 (the "License");
 *    you may not use this file except in compliance with the License.
 *    You may obtain a copy of the License at
 *
 *        http://www.apache.org/licenses/LICENSE-2.0
 *
 *    Unless required by applicable law or agreed to in writing, software
 *    distributed under the License is distributed on an "AS IS" BASIS,
 *    WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
 *    See the License for the specific language governing permissions and
 *    limitations under the License.
 */

 // Matter 1.4 Application Cluster Specification 8.10. Oven Cavity Operational State Cluster

 use core::cell::Cell;
// use rs_matter_macros::idl_import; 

use strum::{EnumDiscriminants, FromRepr};

use crate::data_model::objects::{Cluster, Handler};
use crate::error::Error;
use crate::tlv::TLVElement;
use crate::transport::exchange::Exchange;
use crate::{attribute_enum, cluster_attrs, command_enum};
 
use super::objects::{AttrDataEncoder, AttrDetails, ChangeNotifier, Dataver, NonBlockingHandler};


use super::objects::*;

// idl_import!(clusters = ["OvenCavityOperationalState"]);

// pub use oven_cavity_operational_state::Commands;
// pub use oven_cavity_operational_state::CommandsDiscriminants;

// Cluster ID
pub const ID: u32 = 0x0048; 

// Matter 1.4 Application Cluster Specification 8.10.4 Attributes
#[derive(FromRepr, EnumDiscriminants)]
#[repr(u32)]
pub enum Attributes {
    PhaseList(AttrType<[&'static str; 32]>) = 0x0,
}

#[derive(FromRepr)]
#[repr(u32)]
pub enum Commands {
    Pause = 0x0,
    Stop = 0x1,
    Start = 0x2,
    Resume = 0x3,
}

pub enum CommandsDiscriminants {
    Pause = 0x0,
    Stop = 0x1,
    Start = 0x2,
    Resume = 0x3,
}

attribute_enum!(Attributes);
command_enum!(Commands);

pub const CLUSTER: Cluster<'static> = Cluster {
    id: ID as _,
    revision: 1,
    feature_map: 0,
    attributes: cluster_attrs!(Attribute::new(
        AttributesDiscriminants::PhaseList as _,
        Access::RV,
        Quality::X,
    )),
    accepted_commands: &[
        CommandsDiscriminants::Pause as _,
        CommandsDiscriminants::Stop as _,
        CommandsDiscriminants::Start as _,
        CommandsDiscriminants::Resume as _,
    ],
    generated_commands: &[],
};

#[derive(Clone)]
pub struct OvenCavityOperationalStateCluster {
    data_ver: Dataver,
    phase_list: Cell<[&'static str; 32]>
}

impl OvenCavityOperationalStateCluster {
    pub const fn new(data_ver: Dataver) -> Self {
        Self { 
            data_ver,
            phase_list: Cell::new([""; 32]),
        }
    }

    // Getters
    pub fn get_phase_list(&self) -> [&'static str; 32] {
        self.phase_list.get()
    }

    // Setters
    pub fn set_phase_list(&self, phase_list: [&'static str; 32]) {
        if self.phase_list.get() != phase_list {
            self.phase_list.set(phase_list);
            self.data_ver.changed();
        }
    }

    pub fn read(
        &self,
        _exchange: &Exchange,
        attr: &AttrDetails,
        encoder: AttrDataEncoder,
    ) -> Result<(), Error> {
        if let Some(writer) = encoder.with_dataver(self.data_ver.get())? {
            if attr.is_system() {
                CLUSTER.read(attr.attr_id, writer)
            } else {
                match attr.attr_id.try_into()? {
                    Attributes::PhaseList(codec) => codec.encode(writer, self.get_phase_list()),
                }
            }
        } else {
            Ok(())
        }
    }

    pub fn invoke(
        &self,
        _exchange: &Exchange,
        cmd: &CmdDetails,
        _data: &TLVElement,
        _encoder: CmdDataEncoder,
    ) -> Result<(), Error> {
        match cmd.cmd_id.try_into()? {
            Commands::Pause => {
                // Handle Pause command
                Ok(())
            }
            Commands::Stop => {
                // Handle Stop command
                Ok(())
            }
            Commands::Start => {
                // Handle Start command
                Ok(())
            }
            Commands::Resume => {
                // Handle Resume command
                Ok(())
            }
        }
    }
}


impl Handler for OvenCavityOperationalStateCluster {
    fn read(
        &self,
        exchange: &Exchange,
        attr: &AttrDetails,
        encoder: AttrDataEncoder,
    ) -> Result<(), Error> {
        OvenCavityOperationalStateCluster::read(self, exchange, attr, encoder)
    }
}

impl NonBlockingHandler for OvenCavityOperationalStateCluster {}

impl ChangeNotifier<()> for OvenCavityOperationalStateCluster {
    fn consume_change(&mut self) -> Option<()> {
        self.data_ver.consume_change(())
    }
}
 