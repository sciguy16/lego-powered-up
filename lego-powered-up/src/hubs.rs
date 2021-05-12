use crate::consts::blecharacteristic;
use crate::Hub;
use anyhow::{Context, Result};
use btleplug::api::{Characteristic, Peripheral, WriteType};
use std::collections::HashMap;

#[derive(Debug, Default)]
pub struct HubProperties {
    pub name: String,
    pub fw_version: String,
    pub hw_version: String,
    pub mac_address: String,
    pub battery_level: usize,
    pub rssi: i8,
    pub port_map: PortMap,
}

pub type PortMap = HashMap<Port, u8>;

#[non_exhaustive]
#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
pub enum Port {
    A,
    B,
    C,
    D,
    HubLed,
    CurrentSensor,
    VoltageSensor,
    Accelerometer,
    GyroSensor,
    TiltSensor,
}

impl Port {
    pub fn id(&self) -> u8 {
        todo!()
    }
}

pub struct TechnicHub<P: Peripheral> {
    peripheral: P,
    lpf_characteristic: Characteristic,
    properties: HubProperties,
}

impl<P: Peripheral> Hub for TechnicHub<P> {
    fn name(&self) -> String {
        self.peripheral.properties().local_name.unwrap_or_default()
    }

    fn disconnect(&self) -> Result<()> {
        if self.is_connected() {
            self.peripheral.disconnect()?;
        }
        Ok(())
    }

    fn is_connected(&self) -> bool {
        self.peripheral.is_connected()
    }

    fn properties(&self) -> &HubProperties {
        &self.properties
    }
    fn send(
        &self,
        port: Port,
        mode: u8,
        msg: &[u8],
        request_reply: bool,
    ) -> Result<()> {
        let write_type = if request_reply {
            WriteType::WithResponse
        } else {
            WriteType::WithoutResponse
        };

        let port_id =
            self.port_map().get(&port).context("Invalid port for hub")?;

        let mut buf = Vec::new();
        buf.push(0x81);
        buf.push(*port_id);
        buf.extend_from_slice(&[0x11, 0x51]);
        buf.push(mode);
        buf.extend_from_slice(&msg);

        Ok(self
            .peripheral
            .write(&self.lpf_characteristic, &buf, write_type)?)
    }
    fn subscribe(&self, char: Characteristic) -> Result<()> {
        Ok(self.peripheral.subscribe(&char)?)
    }
}

impl<P: Peripheral> TechnicHub<P> {
    pub fn init(peripheral: P, chars: Vec<Characteristic>) -> Result<Self> {
        // Peripheral is already connected before we get here

        println!("\n\nCHARACTERISTICS:\n\n{:?}\n\n", chars);
        let lpf_characteristic = chars
            .iter()
            .find(|c| c.uuid == *blecharacteristic::LPF2_ALL)
            .context("Device does not advertise LPF2_ALL characteristic")?
            .clone();

        let props = peripheral.properties();

        let mut port_map = PortMap::with_capacity(10);
        port_map.insert(Port::A, 0);
        port_map.insert(Port::B, 1);
        port_map.insert(Port::C, 2);
        port_map.insert(Port::D, 3);
        port_map.insert(Port::HubLed, 50);
        port_map.insert(Port::CurrentSensor, 59);
        port_map.insert(Port::VoltageSensor, 60);
        port_map.insert(Port::Accelerometer, 97);
        port_map.insert(Port::GyroSensor, 98);
        port_map.insert(Port::TiltSensor, 99);

        let properties = HubProperties {
            mac_address: props.address.to_string(),
            name: props.local_name.unwrap_or_default(),
            rssi: props.tx_power_level.unwrap_or_default(),
            port_map,
            ..Default::default()
        };

        Ok(Self {
            peripheral,
            lpf_characteristic,
            properties,
        })
    }
}