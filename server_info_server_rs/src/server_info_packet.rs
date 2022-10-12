


pub mod server_info_packet {
    use std::fmt;
    use std::fmt::Formatter;
    use chrono::{DateTime, NaiveDateTime, Utc};
    use chrono_tz::Tz;
    use serde::{Deserialize, Serialize, Serializer};
    use chrono_tz::US::Pacific;
    use chrono::{TimeZone};
    use serde::ser::SerializeStruct;

    #[derive(Deserialize)]
    #[derive(Default)]
    pub struct ServerInfo {
        pub date: i64,
    }

    impl ServerInfo {
        pub fn get_date_time(&self) -> DateTime<Tz> {
            // pacific time zone conversion
            let utc = DateTime::<Utc>::from_utc(NaiveDateTime::from_timestamp(self.date, 0), Utc).naive_utc();
            Pacific.from_utc_datetime(&utc)
        }

    }

    impl fmt::Display for ServerInfo {
        fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
            //let mut str = "";

            f.write_str("Date: ")?;
            f.write_str(&self.date.to_string())?;


            Ok(())
        }
    }

    impl Serialize for ServerInfo {
        fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
            where
                S: Serializer,
        {
            let mut state = serializer.serialize_struct("ServerInfo",1)?;
            state.serialize_field("date", &self.date)?;
            state.end()
        }
    }

}

