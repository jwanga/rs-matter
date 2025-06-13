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

 // Matter 1.4 Application Cluster Specification 8.7. Refrigerator And Temperature Controlled Cabinet Mode Cluster

 use core::cell::Cell;

use strum::{EnumDiscriminants, FromRepr};

use crate::data_model::objects::{Cluster, Handler};
use crate::error::Error;
use crate::transport::exchange::Exchange;
use crate::{attribute_enum, cluster_attrs};
 
use super::objects::{AttrDataEncoder, AttrDetails, ChangeNotifier, Dataver, NonBlockingHandler};


use super::objects::*;

// Cluster ID
pub const ID: u32 = 0x0052; 


// Matter 1.4 Application Cluster Specification 8.7.5.1 ModeOptionStruct Type 
// #[derive(Copy,Clone, Default)]
// struct ModeOptionStruct {
//     label: AttrType<String>,
//     mode: AttrType<u8>,
//     mode_tags: AttrType<[u8; 8]>,
// } 

// Matter 1.4 Application Cluster Specification 8.7.6 Attributes
#[derive(FromRepr, EnumDiscriminants)]
#[repr(u32)]
pub enum Attributes {
    SupportedModes(AttrType<[u8; 8]>) = 0x0,
    CurrentMode(AttrType<u8>) = 0x1,
    StartupMode(AttrType<u8>) = 0x2,
    OnMode(AttrType<[u8; 8]>) = 0x3
}

attribute_enum!(Attributes);
// command_enum!(Commands);

pub const CLUSTER: Cluster<'static> = Cluster {
    id: ID as _,
    revision: 1,
    feature_map: 0,
    attributes: cluster_attrs!(Attribute::new(
        AttributesDiscriminants::SupportedModes as _,
        Access::RV,
        Quality::NONE,
    ), Attribute::new(
        AttributesDiscriminants::CurrentMode as _,
        Access::RV,
        Quality::NONE,
    ), Attribute::new(
        AttributesDiscriminants::StartupMode as _,
        Access::RV,
        Quality::NONE,
    ), Attribute::new(
        AttributesDiscriminants::OnMode as _,
        Access::RV,
        Quality::NONE,
    )),
    accepted_commands: &[],
    generated_commands: &[],
};

#[derive(Clone)]
pub struct RefrigeratorAndTemperatureControlledCabinetModeCluster {
    data_ver: Dataver,
    supported_mode: Cell<[u8; 8]>,
    current_mode: Cell<u8>,
    start_up_mode: Cell<u8>,
    on_mode: Cell<[u8; 8]>,
}

impl RefrigeratorAndTemperatureControlledCabinetModeCluster {
    pub const fn new(data_ver: Dataver) -> Self {
        Self { 
            data_ver,
            supported_mode: Cell::new([0; 8]),
            current_mode: Cell::new(0),
            start_up_mode: Cell::new(0),
            on_mode: Cell::new([0; 8]),
        }
    }

    // Getters
    pub fn get_supported_mode(&self) -> [u8; 8] {
        self.supported_mode.get()
    }

    pub fn get_current_mode(&self) -> u8 {
        self.current_mode.get()
    }

    pub fn get_start_up_mode(&self) -> u8 {
        self.start_up_mode.get()
    }

    pub fn get_on_mode(&self) -> [u8; 8] {
        self.on_mode.get()
    }

    // Setters
    pub fn set_supported_mode(&self, supported_mode: [u8; 8]) {
        if self.supported_mode.get() != supported_mode {
            self.supported_mode.set(supported_mode);
            self.data_ver.changed();
        }
    }
    pub fn set_current_mode(&self, current_mode: u8) {
        if self.current_mode.get() != current_mode {
            self.current_mode.set(current_mode);
            self.data_ver.changed();
        }
    }
    pub fn set_start_up_mode(&self, start_up_mode: u8) {
        if self.start_up_mode.get() != start_up_mode {
            self.start_up_mode.set(start_up_mode);
            self.data_ver.changed();
        }
    }
    pub fn set_on_mode(&self, on_mode: [u8; 8]) {
        if self.on_mode.get() != on_mode {
            self.on_mode.set(on_mode);
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
                    Attributes::SupportedModes(codec) => codec.encode(writer, self.supported_mode.get()),
                    Attributes::CurrentMode(codec) => codec.encode(writer, self.current_mode.get()),
                    Attributes::StartupMode(codec) => codec.encode(writer, self.start_up_mode.get()),
                    Attributes::OnMode(codec) => codec.encode(writer, self.on_mode.get()),
                }
            }
        } else {
            Ok(())
        }
    }

    // pub fn invoke(
    //     &self,
    //     _exchange: &Exchange,
    //     cmd: &CmdDetails,
    //     _data: &TLVElement,
    //     _encoder: CmdDataEncoder,
    // ) -> Result<(), Error> {
    //     match cmd.cmd_id.try_into()? {
    //         Commands::SetTemperature
    //     }
    // }
}


impl Handler for RefrigeratorAndTemperatureControlledCabinetModeCluster {
    fn read(
        &self,
        exchange: &Exchange,
        attr: &AttrDetails,
        encoder: AttrDataEncoder,
    ) -> Result<(), Error> {
        RefrigeratorAndTemperatureControlledCabinetModeCluster::read(self, exchange, attr, encoder)
    }
}

impl NonBlockingHandler for RefrigeratorAndTemperatureControlledCabinetModeCluster {}

impl ChangeNotifier<()> for RefrigeratorAndTemperatureControlledCabinetModeCluster {
    fn consume_change(&mut self) -> Option<()> {
        self.data_ver.consume_change(())
    }
}
 