use std::{collections::HashMap, net::Ipv4Addr};

use yeelight_protocol::{bulb_protocol, messages_command, types, Serialize};

use super::tcp_request::tcp_request;

pub struct Bulb {
    bulb_protocol: bulb_protocol::BulbProtocol,
}

impl Bulb {
    pub fn new(address: &Ipv4Addr) -> Self {
        Self {
            bulb_protocol: bulb_protocol::BulbProtocol::new(address),
        }
    }

    /// This method is used to retrieve current property of smart LED.
    pub fn get_prop(
        &self,
        props: &Vec<messages_command::AllProperties>,
    ) -> Result<HashMap<messages_command::AllProperties, String>, String> {
        let request = messages_command::GetProp::new(props);
        let result = self.request(&request)?;

        let mut hash = HashMap::new();
        props.iter().zip(result.iter()).for_each(|(prop, result1)| {
            hash.insert(prop.clone(), String::from(result1));
        });

        Ok(hash)
    }

    /// This method is used to change the color temperature of a smart LED.
    pub fn set_ct_abx(
        &self,
        ct_value: types::ColorTemperature,
        effect: types::Effect,
        duration: types::Duration,
    ) -> Result<(), String> {
        let request =
            messages_command::SetCtAbx::new(ct_value, effect, duration);
        let _ = self.request(&request)?;
        Ok(())
    }

    /// This method is used to change the color of a smart LED.
    pub fn set_rgb(
        &self,
        rgb_value: types::Rgb,
        effect: types::Effect,
        duration: types::Duration,
    ) -> Result<(), String> {
        let request =
            messages_command::SetRgb::new(rgb_value, effect, duration);
        let _ = self.request(&request)?;
        Ok(())
    }

    /// This method is used to change the color of a smart LED.
    pub fn set_hsv(
        &self,
        hue: types::Hue,
        sat: types::Brightness,
        effect: types::Effect,
        duration: types::Duration,
    ) -> Result<(), String> {
        let request = messages_command::SetHsv::new(hue, sat, effect, duration);
        let _ = self.request(&request)?;
        Ok(())
    }

    /// This method is used to change the brightness of a smart LED.
    pub fn set_bright(
        &self,
        brightness: types::Brightness,
        effect: types::Effect,
        duration: types::Duration,
    ) -> Result<(), String> {
        let request =
            messages_command::SetBright::new(brightness, effect, duration);
        let _ = self.request(&request)?;
        Ok(())
    }

    /// This method is used to switch on or off the smart LED.
    pub fn set_power(
        &self,
        power: types::Power,
        effect: types::Effect,
        duration: types::Duration,
    ) -> Result<(), String> {
        let request = messages_command::SetPower::new(power, effect, duration);
        let _ = self.request(&request)?;
        Ok(())
    }

    /// This method is used to toggle the smart LED.
    pub fn toggle(&self) {
        todo!()
    }

    /// This method is used to save current state of smart LED in
    /// persistent memory.
    pub fn set_default(&self) {
        todo!()
    }

    /// This method is used to start a color flow.
    pub fn start_cf(&self) {
        todo!()
    }

    /// This method is used to stop a running color flow.
    pub fn stop_cf(&self) {
        todo!()
    }

    /// This method is used to set the smart LED directly to specified state.
    pub fn set_scene(&self) {
        todo!()
    }

    /// This method is used to start a timer job on the smart LED.
    pub fn cron_add(&self) {
        todo!()
    }

    /// This method is used to retrieve the setting of the current cron job
    /// of the specified type.
    pub fn cron_get(&self) {
        todo!()
    }

    /// This method is used to stop the specified cron job.
    pub fn cron_del(&self) {
        todo!()
    }

    /// This method is used to change brightness, CT or color of a smart LED
    /// without knowing the current value, it's main used by controllers.
    pub fn set_adjust(&self) {
        todo!()
    }

    /// This method is used to start or stop music mode on a device.
    /// Under music mode, no property will be reported and no message
    /// quota is checked.
    pub fn set_music(&self) {
        todo!()
    }

    /// This method is used to name the device. The name will be stored on
    /// the device and reported in discovering response. User can also read
    /// the name through “get_prop” method.
    pub fn set_name(&self) {
        todo!()
    }

    /// This method is used to toggle the main light and background light at
    /// the same time.
    pub fn dev_toggle(&self) {
        todo!()
    }

    /// This method is used to adjust the brightness by specified percentage
    ///  within specified duration.
    pub fn adjust_bright(&self) {
        todo!()
    }

    /// This method is used to adjust the color temperature by specified
    ///  percentage within specified duration.
    pub fn adjust_ct(&self) {
        todo!()
    }

    /// This method is used to adjust the color within specified duration.
    pub fn adjust_color(&self) {
        todo!()
    }

    fn request<T>(&self, request: &T) -> Result<Vec<String>, String>
    where
        T: Serialize,
    {
        let addr = self.bulb_protocol.get_socket_addr();
        let request = self.bulb_protocol.serialize(request)?;

        let response = tcp_request(&addr, &request)?;

        let response = self.bulb_protocol.deserialize(&response);
        println!("result: {:?}", response);
        response
    }
}

#[cfg(test)]
mod tests {
    use std::{thread, time};

    use yeelight_protocol::messages_command::AllProperties;
    use yeelight_protocol::types;

    use super::*;

    const ADDRESS: Ipv4Addr = Ipv4Addr::new(192, 168, 1, 104);

    #[test]
    fn set_power() {
        let bulb = Bulb::new(&ADDRESS);
        bulb.set_power(
            types::Power::Off,
            types::Effect::Smooth,
            types::Duration::new(500),
        )
        .unwrap();

        thread::sleep(time::Duration::from_secs(2));

        bulb.set_power(
            types::Power::On,
            types::Effect::Smooth,
            types::Duration::new(500),
        )
        .unwrap();
    }

    #[test]
    fn set_ct_abx() {
        let bulb = Bulb::new(&ADDRESS);
        bulb.set_ct_abx(
            types::ColorTemperature::new(3500),
            types::Effect::Smooth,
            types::Duration::new(1000),
        )
        .unwrap();
    }

    #[test]
    fn set_rgb() {
        let bulb = Bulb::new(&ADDRESS);
        bulb.set_rgb(
            types::Rgb::new(16711680),
            types::Effect::Smooth,
            types::Duration::new(1000),
        )
        .unwrap();
    }

    #[test]
    fn set_hsv() {
        let bulb = Bulb::new(&ADDRESS);
        bulb.set_hsv(
            types::Hue::new(255),
            types::Brightness::new(45),
            types::Effect::Smooth,
            types::Duration::new(500),
        )
        .unwrap();
    }

    #[test]
    fn set_bright() {
        let bulb = Bulb::new(&ADDRESS);
        bulb.set_bright(
            types::Brightness::new(50),
            types::Effect::Smooth,
            types::Duration::new(500),
        )
        .unwrap();

        thread::sleep(time::Duration::from_secs(2));

        bulb.set_bright(
            types::Brightness::new(100),
            types::Effect::Smooth,
            types::Duration::new(500),
        )
        .unwrap();
    }

    #[test]
    fn get_prop() {
        let bulb = Bulb::new(&ADDRESS);
        let res = bulb
            .get_prop(&vec![
                AllProperties::ActiveMode,
                AllProperties::BgBright,
                AllProperties::BgCt,
                AllProperties::BgFlowing,
                AllProperties::BgFlowParams,
                AllProperties::BgHue,
                AllProperties::BgLmode,
                AllProperties::BgPower,
                AllProperties::BgRgb,
                AllProperties::BgSat,
                AllProperties::Bright,
                AllProperties::ColorMode,
                AllProperties::Ct,
                AllProperties::Delayoff,
                AllProperties::Hue,
                AllProperties::Flowing,
                AllProperties::FlowParams,
                AllProperties::MusicOn,
                AllProperties::Name,
                AllProperties::NlBr,
                AllProperties::Power,
                AllProperties::Rgb,
                AllProperties::Sat,
            ])
            .unwrap();
        println!("result: {:?}", res);
    }
}
