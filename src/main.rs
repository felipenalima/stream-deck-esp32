pub mod wifi_network;

use esp_idf_hal::{delay::FreeRtos, peripherals::Peripherals};
use esp_idf_svc::{eventloop::EspSystemEventLoop, nvs::EspDefaultNvsPartition};
use anyhow::Result;

use crate::wifi_network::wifi:: init as wifi_init;

//Change your ssid and password in cfg.toml file
#[toml_cfg::toml_config]
pub struct Config {
	#[default("")]
	wifi_ssid: &'static str,
	#[default("")]
	wifi_psk: &'static str,
}

fn main() -> Result<()>  {
    let _nvs = EspDefaultNvsPartition::take();
    let peripherals = Peripherals::take().unwrap();
    let app_config = CONFIG;
    let sysloop = EspSystemEventLoop::take().unwrap();
    
    esp_idf_svc::log::EspLogger::initialize_default();

    log::info!("{}", app_config.wifi_ssid);
	log::info!("{}", app_config.wifi_psk);

    let _wifi = wifi_init(
		app_config.wifi_ssid,
		app_config.wifi_psk,
		peripherals.modem,
		sysloop.clone());
    
    loop {
        FreeRtos::delay_ms(1000);
    }   
}
