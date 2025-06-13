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

 // Matter 1.4 Application Cluster Specification 8.2. Temperature Control Cluster 

 use core::cell::Cell;
use rs_matter_macros::idl_import; 

use strum::{EnumDiscriminants, FromRepr};

use crate::data_model::objects::{Cluster, Handler};
use crate::error::Error;
use crate::transport::exchange::Exchange;
use crate::{attribute_enum, cluster_attrs, command_enum};
 
use super::objects::{AttrDataEncoder, AttrDetails, ChangeNotifier, Dataver, NonBlockingHandler};


use super::objects::*;
 
idl_import!(clusters = ["TemperatureControl"]);

pub use temperature_control::ID;
pub use temperature_control::Commands;
pub use temperature_control::CommandsDiscriminants;


// Matter 1.4 Application Cluster Specification 8.2.5 Attributes
#[derive(FromRepr, EnumDiscriminants)]
#[repr(u32)]
pub enum Attributes {
    TemperatureSetpoint(AttrType<i16>) = 0x0,
    MinTemperature(AttrType<i16>) = 0x1,
    MaxTemperature(AttrType<i16>) = 0x2,
    Step(AttrType<i16>) = 0x3,
    SelectedTemperatureLevel(AttrType<u8>) = 0x4,
    SupportedTemperatureLevels(AttrType<[u8; 32]>) = 0x5,
}

attribute_enum!(Attributes);
command_enum!(Commands);

pub const CLUSTER: Cluster<'static> = Cluster {
    id: ID as _,
    revision: 1,
    feature_map: 0,
    attributes: cluster_attrs!(Attribute::new(
        AttributesDiscriminants::TemperatureSetpoint as _,
        Access::RV,
        Quality::NONE,
    ), Attribute::new(
        AttributesDiscriminants::MinTemperature as _,
        Access::RV,
        Quality::F,
    ), Attribute::new(
        AttributesDiscriminants::MaxTemperature as _,
        Access::RV,
        Quality::F,
    ), Attribute::new(
        AttributesDiscriminants::Step as _,
        Access::RV,
        Quality::F,
    ), Attribute::new(
        AttributesDiscriminants::SelectedTemperatureLevel as _,
        Access::RV,
        Quality::NONE,
    ), Attribute::new(
        AttributesDiscriminants::SupportedTemperatureLevels as _,
        Access::RV,
        Quality::NONE,
    ),),
    accepted_commands: &[
        CommandsDiscriminants::SetTemperature as _,
    ],
    generated_commands: &[],
};

#[derive(Clone)]
pub struct TemperatureControlCluster {
    data_ver: Dataver,
    temperature_setpoint: Cell<i16>,
    min_temperature: Cell<i16>,
    max_temperature: Cell<i16>,
    step: Cell<i16>,
    selected_temperature_level: Cell<u8>,
    supported_temperature_levels: Cell<[u8; 32]>,
}

impl TemperatureControlCluster {
    pub const fn new(data_ver: Dataver) -> Self {
        Self { 
            data_ver,
            temperature_setpoint: Cell::new(0),
            min_temperature: Cell::new(std::i16::MIN),
            max_temperature: Cell::new(std::i16::MAX),
            step: Cell::new(1),
            selected_temperature_level: Cell::new(0),
            supported_temperature_levels: Cell::new([0; 32]),
        }
    }

    // Getters
    pub fn get_temperature_setpoint(&self) -> i16 {
        self.temperature_setpoint.get()
    }

    pub fn get_min_temperature(&self) -> i16 {
        self.min_temperature.get()
    }

    pub fn get_max_temperature(&self) -> i16 {
        self.max_temperature.get()
    }

    pub fn get_step(&self) -> i16 {
        self.step.get()
    }

    pub fn get_selected_temperature_level(&self) -> u8 {
        self.selected_temperature_level.get()
    }

    pub fn get_supported_temperature_levels(&self) -> [u8; 32] {
        self.supported_temperature_levels.get()
    }

    // Setters
    pub fn set_temperature_setpoint(&self, temperature_setpoint: i16) {
        if self.temperature_setpoint.get() != temperature_setpoint {
            self.temperature_setpoint.set(temperature_setpoint);
            self.data_ver.changed();
        }
    }
    pub fn set_min_temperature(&self, min_temperature: i16) {
        if self.min_temperature.get() != min_temperature {
            self.min_temperature.set(min_temperature);
            self.data_ver.changed();
        }
    }
    pub fn set_max_temperature(&self, max_temperature: i16) {
        if self.max_temperature.get() != max_temperature {
            self.max_temperature.set(max_temperature);
            self.data_ver.changed();
        }
    }
    pub fn set_step(&self, step: i16) {
        if self.step.get() != step {
            self.step.set(step);
            self.data_ver.changed();
        }
    }
    pub fn set_selected_temperature_level(&self, selected_temperature_level: u8) {
        if self.selected_temperature_level.get() != selected_temperature_level {
            self.selected_temperature_level.set(selected_temperature_level);
            self.data_ver.changed();
        }
    }
    pub fn set_supported_temperature_levels(&self, supported_temperature_levels: [u8; 32]) {
        if self.supported_temperature_levels.get() != supported_temperature_levels {
            self.supported_temperature_levels.set(supported_temperature_levels);
            self.data_ver.changed();
        }
    }

    pub fn read(
        &self,
        _exchange: &Exchange,
        attr: &AttrDetails,
        encoder: AttrDataEncoder,
    ) -> Result<(), Error> {
        print!("foo1 - Reading attribute: {:?}", attr.attr_id);
        if let Some(writer) = encoder.with_dataver(self.data_ver.get())? {
            if attr.is_system() {
                CLUSTER.read(attr.attr_id, writer)
            } else {
                println!("foo2 - Reading attribute: {:?}", attr.attr_id);
                match attr.attr_id.try_into()? {
                    Attributes::TemperatureSetpoint(codec) => codec.encode(writer, self.temperature_setpoint.get()),
                    Attributes::MinTemperature(codec) => codec.encode(writer, self.min_temperature.get()),
                    Attributes::MaxTemperature(codec) => codec.encode(writer, self.max_temperature.get()),
                    Attributes::Step(codec) => codec.encode(writer, self.step.get()),
                    Attributes::SelectedTemperatureLevel(codec) => codec.encode(writer, self.selected_temperature_level.get()),
                    Attributes::SupportedTemperatureLevels(codec) => codec.encode(writer, self.supported_temperature_levels.get()),
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


impl Handler for TemperatureControlCluster {
    fn read(
        &self,
        exchange: &Exchange,
        attr: &AttrDetails,
        encoder: AttrDataEncoder,
    ) -> Result<(), Error> {
        TemperatureControlCluster::read(self, exchange, attr, encoder)
    }
}

impl NonBlockingHandler for TemperatureControlCluster {}

impl ChangeNotifier<()> for TemperatureControlCluster {
    fn consume_change(&mut self) -> Option<()> {
        self.data_ver.consume_change(())
    }
}
 