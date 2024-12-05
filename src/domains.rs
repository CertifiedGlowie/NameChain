use std::{
    fmt::Display,
    net::{Ipv4Addr, Ipv6Addr},
    str::FromStr,
};

use serde::{Deserialize, Serialize};

pub type TopLevel = String;
pub type SecondLevel = String;
pub type SubLevel = Option<String>;

#[derive(Serialize, Deserialize)]
pub struct Domain {
    tld: TopLevel,
    sld: SecondLevel,
    sub: SubLevel,
}

impl Display for Domain {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let formatted_string = match &self.sub {
            Some(sub) => {
                format!("{}.{}.{}", sub, self.sld, self.tld)
            }
            None => {
                format!("{}.{}", self.sld, self.tld)
            }
        };
        write!(f, "{}", formatted_string)
    }
}

impl FromStr for Domain {
    type Err = anyhow::Error;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let parts: Vec<&str> = s.split('.').collect();

        if parts.len() < 2 {
            return Err(anyhow::anyhow!("Invalid domain format"));
        }

        let sld = parts[parts.len() - 2];
        let tld = parts[parts.len() - 1];

        let sub = if parts.len() > 2 {
            Some(parts[0..parts.len() - 2].join("."))
        } else {
            None
        };

        let tld = TopLevel::from(tld);
        let sld = SecondLevel::from(sld);

        Ok(Domain { tld, sld, sub })
    }
}

#[derive(Serialize, Deserialize)]
pub enum Record {
    A {
        index: SubLevel,
        target: Option<Ipv4Addr>,
    },
    AAAA {
        index: SubLevel,
        target: Option<Ipv6Addr>,
    },
    CNAME {
        index: SubLevel,
        target: Option<Domain>,
    },
    NS {
        index: SubLevel,
        target: Option<Domain>,
    },
    PTR {
        index: SubLevel,
        target: Option<Domain>,
    },
    TXT {
        index: SubLevel,
        data: Option<String>,
    },
    SRV {
        at: SubLevel,
        priority: u16,
        weight: u16,
        port: u16,
        target: Option<Domain>,
    },
}
