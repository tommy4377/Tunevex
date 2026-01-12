use crate::modules::types::Tweak;

pub mod advertising;
pub mod apps;
pub mod maintenance;
pub mod policies;
pub mod services; // New
pub mod settings;
pub mod tasks;
pub mod telemetry; // New

pub fn get_privacy_tweaks() -> Vec<Tweak> {
    let mut tweaks = Vec::new();

    tweaks.extend(telemetry::get_tweaks());
    tweaks.extend(apps::get_tweaks());
    tweaks.extend(settings::get_tweaks());
    tweaks.extend(advertising::get_tweaks());
    tweaks.extend(maintenance::get_tweaks());
    tweaks.extend(policies::get_tweaks());
    tweaks.extend(services::get_service_tweaks()); // New
    tweaks.extend(tasks::get_task_tweaks()); // New

    tweaks
}
