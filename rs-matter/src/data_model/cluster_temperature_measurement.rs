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

 // Matter 1.4 Application Cluster Specification 2.3. Temperature Measurement Cluster 

 use core::cell::Cell;
// use rs_matter_macros::idl_import; 

use strum::{EnumDiscriminants, FromRepr};

use crate::data_model::objects::{Cluster, Handler};
use crate::error::Error;
use crate::transport::exchange::Exchange;
use crate::{attribute_enum, cluster_attrs};
 
use super::objects::{AttrDataEncoder, AttrDetails, ChangeNotifier, Dataver, NonBlockingHandler};


use super::objects::*;
 
// idl_import!(clusters = ["TemperatureMeasurement"]); // removed due to zero-variant Commands enum generation
pub const ID: u32 = 0x0402; // TemperatureMeasurement Cluster ID
// pub use temperature_measurement::Commands;
// pub use temperature_measurement::CommandsDiscriminants;


#[derive(FromRepr, EnumDiscriminants)]
#[repr(u32)]

// Matter 1.4 Application Cluster Specification 2.4.4 Attributes
pub enum Attributes {
    MeasuredValue(AttrType<i16>) = 0x0,
    MinMeasuredValue(AttrType<i16>) = 0x1,
    MaxMeasuredValue(AttrType<i16>) = 0x2
}

attribute_enum!(Attributes);
// command_enum!(Commands);

pub const CLUSTER: Cluster<'static> = Cluster {
    id: ID as _,
    revision: 1,
    feature_map: 0,
    attributes: cluster_attrs!(Attribute::new(
        AttributesDiscriminants::MeasuredValue as _,
        Access::RV,
        Quality::NONE,
    ), Attribute::new(
        AttributesDiscriminants::MinMeasuredValue as _,
        Access::RV,
        Quality::F,
    ), Attribute::new(
        AttributesDiscriminants::MaxMeasuredValue as _,
        Access::RV,
        Quality::F,
    )),
    accepted_commands: &[],
    generated_commands: &[],
};

#[derive(Clone)]
pub struct TemperatureMeasurementCluster {
    data_ver: Dataver,
    measured_value: Cell<i16>,
    min_measured_value: Cell<i16>,
    max_measured_value: Cell<i16>,
}

impl TemperatureMeasurementCluster {
    pub const fn new(data_ver: Dataver) -> Self {
        Self { 
            data_ver,
            measured_value: Cell::new(0),
            min_measured_value: Cell::new(std::i16::MIN),
            max_measured_value: Cell::new(std::i16::MAX),
        }
    }

    // Getters
    pub fn get_measured_value(&self) -> i16 {
        self.measured_value.get()
    }

    pub fn get_min_measured_value(&self) -> i16 {
        self.min_measured_value.get()
    }

    pub fn get_max_measured_value(&self) -> i16 {
        self.max_measured_value.get()
    }

    // Setters
    pub fn set_measured_value(&self, measured_value: i16) {
        if self.measured_value.get() != measured_value {
            self.measured_value.set(measured_value);
            self.data_ver.changed();
        }
    }
    pub fn set_min_measured_value(&self, min_measured_value: i16) {
        if self.min_measured_value.get() != min_measured_value {
            self.min_measured_value.set(min_measured_value);
            self.data_ver.changed();
        }
    }
    pub fn set_max_measured_value(&self, max_measured_value: i16) {
        if self.max_measured_value.get() != max_measured_value {
            self.max_measured_value.set(max_measured_value);
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
                    Attributes::MeasuredValue(codec) => codec.encode(writer, self.measured_value.get()),
                    Attributes::MinMeasuredValue(codec) => codec.encode(writer, self.min_measured_value.get()),
                    Attributes::MaxMeasuredValue(codec) => codec.encode(writer, self.max_measured_value.get()),
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


impl Handler for TemperatureMeasurementCluster {
    fn read(
        &self,
        exchange: &Exchange,
        attr: &AttrDetails,
        encoder: AttrDataEncoder,
    ) -> Result<(), Error> {
        TemperatureMeasurementCluster::read(self, exchange, attr, encoder)
    }
}

impl NonBlockingHandler for TemperatureMeasurementCluster {}

impl ChangeNotifier<()> for TemperatureMeasurementCluster {
    fn consume_change(&mut self) -> Option<()> {
        self.data_ver.consume_change(())
    }
}
 