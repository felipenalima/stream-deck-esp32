use esp_idf_hal::peripheral;
use esp_idf_svc::{
	eventloop::EspSystemEventLoop,
	wifi::{
		BlockingWifi,
		ClientConfiguration,
		AccessPointConfiguration,
		Configuration,
		EspWifi,
	},
};
use anyhow::Result;

pub fn init(
	ssid: &str,
	password: &str,
	modem: impl peripheral::Peripheral<P = esp_idf_svc::hal::modem::Modem> + 'static,
	sysloop: EspSystemEventLoop,
) -> Result<Box<EspWifi<'static>>>
{
	let mut esp_wifi = EspWifi::new(modem, sysloop.clone(), None)?;
	let mut wifi = BlockingWifi::wrap(&mut esp_wifi, sysloop)?;

	wifi.set_configuration(&Configuration::Client(ClientConfiguration::default()))?;

	log::info!("Starting wifi...");

	wifi.start()?;

	log::info!("Scanning...");

	let ap_infos = wifi.scan()?;
	let ours = ap_infos.into_iter().find(|a| a.ssid == ssid);
	let channel = if let Some(ours) = ours {
		log::info!("Found configured access point {} on channel {}", ours.ssid, ours.channel);
		Some(ours.channel)
	} else {
		log::warn!("Configured access point {} not found during scanning, will go with unknown channel", ssid);
		None
	};

	let _ = wifi.set_configuration(&Configuration::Mixed(
		ClientConfiguration{
			ssid: ssid.try_into().unwrap(),
			password: password.try_into().unwrap(),
			channel,
			..Default::default()
		},
		AccessPointConfiguration {
			ssid: "aptest".try_into().unwrap(),
			channel: channel.unwrap_or(1),
			..Default::default()
		},
	));

	log::info!("Connecting wifi...");
	wifi.connect()?;

	log::info!("Waiting for DHCP lease...");
	wifi.wait_netif_up()?;

	let ip_info = wifi.wifi().sta_netif().get_ip_info()?;
	log::info!("Wifi DHCP info: {:?}", ip_info);

	Ok(Box::new(esp_wifi))
}
