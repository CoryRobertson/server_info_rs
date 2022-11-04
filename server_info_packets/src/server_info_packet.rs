pub mod server_info_packet {
    use chrono::TimeZone;
    use chrono::{DateTime, NaiveDateTime, Utc};
    use chrono_tz::Tz;
    use chrono_tz::US::Pacific;
    use serde::ser::SerializeStruct;
    use serde::{Deserialize, Serialize, Serializer};
    use std::fmt;
    use std::fmt::Formatter;

    #[derive(Deserialize, Default)]
    pub struct ServerInfo {
        pub date: i64,
        pub disks: Vec<String>,
        pub net_interfaces: Vec<String>,
        pub components: Vec<String>,
        pub total_ram: u64,
        pub used_memory: u64,
        pub system_name: String,
        pub kernel_version: String,
        pub os_version: String,
        pub host_name: String,
        pub total_cpus: usize,
        pub cpus: Vec<String>,
        pub avg_cpu_usage: f32,
    }

    impl ServerInfo {
        pub fn get_date_time(&self) -> DateTime<Tz> {
            // pacific time zone conversion
            let utc = DateTime::<Utc>::from_utc(NaiveDateTime::from_timestamp(self.date, 0), Utc)
                .naive_utc();
            Pacific.from_utc_datetime(&utc)
        }
    }

    impl fmt::Display for ServerInfo {
        fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
            //let mut str = "";

            f.write_str("Date: ")?;
            f.write_str(&self.date.to_string())?;
            f.write_str("\n")?;

            f.write_str("Disks: ")?;
            for disk in &self.disks {
                f.write_str(&*disk)?;
                f.write_str("\n")?;
            }

            f.write_str("Network Interfaces: ")?;
            for interface in &self.net_interfaces {
                f.write_str(&*interface)?;
                f.write_str("\n")?;
            }

            // f.write_str("Components: ")?;
            // for component in &self.components {
            //     f.write_str(component)?;
            //     f.write_str("\n")?;
            // }

            f.write_str("Total Ram: ")?;
            f.write_str(self.total_ram.to_string().as_str())?;
            f.write_str("\n")?;

            f.write_str("Used Memory: ")?;
            f.write_str(self.used_memory.to_string().as_str())?;
            f.write_str("\n")?;

            f.write_str("System Name: ")?;
            f.write_str(self.system_name.as_str())?;
            f.write_str("\n")?;

            f.write_str("Kernel Version: ")?;
            f.write_str(self.kernel_version.as_str())?;
            f.write_str("\n")?;

            f.write_str("OS Version: ")?;
            f.write_str(self.os_version.as_str())?;
            f.write_str("\n")?;

            f.write_str("Host Name: ")?;
            f.write_str(self.host_name.as_str())?;
            f.write_str("\n")?;

            f.write_str("Total CPUS: ")?;
            f.write_str(self.total_cpus.to_string().as_str())?;
            f.write_str("\n")?;

            for cpu in &self.cpus {
                f.write_str(cpu)?;
                f.write_str("\n")?;
            }

            f.write_str("Average CPU Usage: ")?;
            f.write_str(self.avg_cpu_usage.to_string().as_str())?;

            Ok(())
        }
    }

    impl Serialize for ServerInfo {
        fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            let mut state = serializer.serialize_struct("ServerInfo", 13)?;
            state.serialize_field("date", &self.date)?;
            state.serialize_field("disks", &self.disks)?;
            state.serialize_field("net_interfaces", &self.net_interfaces)?;
            state.serialize_field("components", &self.components)?;
            state.serialize_field("total_ram", &self.total_ram)?;
            state.serialize_field("used_memory", &self.used_memory)?;
            state.serialize_field("system_name", &self.system_name)?;
            state.serialize_field("kernel_version", &self.kernel_version)?;
            state.serialize_field("os_version", &self.os_version)?;
            state.serialize_field("host_name", &self.host_name)?;
            state.serialize_field("total_cpus", &self.total_cpus)?;
            state.serialize_field("cpus", &self.cpus)?;
            state.serialize_field("avg_cpu_usage", &self.avg_cpu_usage)?;
            state.end()
        }
    }
}
