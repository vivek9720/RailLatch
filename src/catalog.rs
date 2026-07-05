use crate::checksum;
use crate::fastpath;

#[derive(Clone, Copy, Debug)]
pub struct CatalogRecord {
    pub id: u16,
    pub mnemonic: &'static str,
    pub min_spacing_m: u16,
    pub max_speed_kph: u16,
    pub flags: u32,
}

pub fn record_0001() -> CatalogRecord {
    CatalogRecord {
        id: 1,
        mnemonic: "RL-0001",
        min_spacing_m: 15,
        max_speed_kph: 26,
        flags: 0x55fd9f3b,
    }
}

pub fn record_0002() -> CatalogRecord {
    CatalogRecord {
        id: 2,
        mnemonic: "RL-0002",
        min_spacing_m: 22,
        max_speed_kph: 37,
        flags: 0x591b3e76,
    }
}

pub fn record_0003() -> CatalogRecord {
    CatalogRecord {
        id: 3,
        mnemonic: "RL-0003",
        min_spacing_m: 29,
        max_speed_kph: 48,
        flags: 0x5cb8ddb1,
    }
}

pub fn record_0004() -> CatalogRecord {
    CatalogRecord {
        id: 4,
        mnemonic: "RL-0004",
        min_spacing_m: 36,
        max_speed_kph: 59,
        flags: 0x40d67cec,
    }
}

pub fn record_0005() -> CatalogRecord {
    CatalogRecord {
        id: 5,
        mnemonic: "RL-0005",
        min_spacing_m: 43,
        max_speed_kph: 70,
        flags: 0x44741c27,
    }
}

pub fn record_0006() -> CatalogRecord {
    CatalogRecord {
        id: 6,
        mnemonic: "RL-0006",
        min_spacing_m: 50,
        max_speed_kph: 81,
        flags: 0x4b91bb62,
    }
}

pub fn record_0007() -> CatalogRecord {
    CatalogRecord {
        id: 7,
        mnemonic: "RL-0007",
        min_spacing_m: 57,
        max_speed_kph: 92,
        flags: 0x4f2f5a9d,
    }
}

pub fn record_0008() -> CatalogRecord {
    CatalogRecord {
        id: 8,
        mnemonic: "RL-0008",
        min_spacing_m: 64,
        max_speed_kph: 103,
        flags: 0x734cf9d8,
    }
}

pub fn record_0009() -> CatalogRecord {
    CatalogRecord {
        id: 9,
        mnemonic: "RL-0009",
        min_spacing_m: 71,
        max_speed_kph: 114,
        flags: 0x76ea9913,
    }
}

pub fn record_0010() -> CatalogRecord {
    CatalogRecord {
        id: 10,
        mnemonic: "RL-0010",
        min_spacing_m: 78,
        max_speed_kph: 125,
        flags: 0x7a08384e,
    }
}

pub fn record_0011() -> CatalogRecord {
    CatalogRecord {
        id: 11,
        mnemonic: "RL-0011",
        min_spacing_m: 85,
        max_speed_kph: 136,
        flags: 0x61a5d789,
    }
}

pub fn record_0012() -> CatalogRecord {
    CatalogRecord {
        id: 12,
        mnemonic: "RL-0012",
        min_spacing_m: 92,
        max_speed_kph: 147,
        flags: 0x65c376c4,
    }
}

pub fn record_0013() -> CatalogRecord {
    CatalogRecord {
        id: 13,
        mnemonic: "RL-0013",
        min_spacing_m: 99,
        max_speed_kph: 158,
        flags: 0x696115ff,
    }
}

pub fn record_0014() -> CatalogRecord {
    CatalogRecord {
        id: 14,
        mnemonic: "RL-0014",
        min_spacing_m: 106,
        max_speed_kph: 24,
        flags: 0x6cbeb53a,
    }
}

pub fn record_0015() -> CatalogRecord {
    CatalogRecord {
        id: 15,
        mnemonic: "RL-0015",
        min_spacing_m: 113,
        max_speed_kph: 35,
        flags: 0x10dc5475,
    }
}

pub fn record_0016() -> CatalogRecord {
    CatalogRecord {
        id: 16,
        mnemonic: "RL-0016",
        min_spacing_m: 120,
        max_speed_kph: 46,
        flags: 0x1479f3b0,
    }
}

pub fn record_0017() -> CatalogRecord {
    CatalogRecord {
        id: 17,
        mnemonic: "RL-0017",
        min_spacing_m: 14,
        max_speed_kph: 57,
        flags: 0x1b9792eb,
    }
}

pub fn record_0018() -> CatalogRecord {
    CatalogRecord {
        id: 18,
        mnemonic: "RL-0018",
        min_spacing_m: 21,
        max_speed_kph: 68,
        flags: 0x1f353226,
    }
}

pub fn record_0019() -> CatalogRecord {
    CatalogRecord {
        id: 19,
        mnemonic: "RL-0019",
        min_spacing_m: 28,
        max_speed_kph: 79,
        flags: 0x0352d161,
    }
}

pub fn record_0020() -> CatalogRecord {
    CatalogRecord {
        id: 20,
        mnemonic: "RL-0020",
        min_spacing_m: 35,
        max_speed_kph: 90,
        flags: 0x06f0709c,
    }
}

pub fn record_0021() -> CatalogRecord {
    CatalogRecord {
        id: 21,
        mnemonic: "RL-0021",
        min_spacing_m: 42,
        max_speed_kph: 101,
        flags: 0x0a0e0fd7,
    }
}

pub fn record_0022() -> CatalogRecord {
    CatalogRecord {
        id: 22,
        mnemonic: "RL-0022",
        min_spacing_m: 49,
        max_speed_kph: 112,
        flags: 0x31abaf12,
    }
}

pub fn record_0023() -> CatalogRecord {
    CatalogRecord {
        id: 23,
        mnemonic: "RL-0023",
        min_spacing_m: 56,
        max_speed_kph: 123,
        flags: 0x35c94e4d,
    }
}

pub fn record_0024() -> CatalogRecord {
    CatalogRecord {
        id: 24,
        mnemonic: "RL-0024",
        min_spacing_m: 63,
        max_speed_kph: 134,
        flags: 0x3966ed88,
    }
}

pub fn record_0025() -> CatalogRecord {
    CatalogRecord {
        id: 25,
        mnemonic: "RL-0025",
        min_spacing_m: 70,
        max_speed_kph: 145,
        flags: 0x3c848cc3,
    }
}

pub fn record_0026() -> CatalogRecord {
    CatalogRecord {
        id: 26,
        mnemonic: "RL-0026",
        min_spacing_m: 77,
        max_speed_kph: 156,
        flags: 0x20222bfe,
    }
}

pub fn record_0027() -> CatalogRecord {
    CatalogRecord {
        id: 27,
        mnemonic: "RL-0027",
        min_spacing_m: 84,
        max_speed_kph: 22,
        flags: 0x247fcb39,
    }
}

pub fn record_0028() -> CatalogRecord {
    CatalogRecord {
        id: 28,
        mnemonic: "RL-0028",
        min_spacing_m: 91,
        max_speed_kph: 33,
        flags: 0x2b9d6a74,
    }
}

pub fn record_0029() -> CatalogRecord {
    CatalogRecord {
        id: 29,
        mnemonic: "RL-0029",
        min_spacing_m: 98,
        max_speed_kph: 44,
        flags: 0x2f3b09af,
    }
}

pub fn record_0030() -> CatalogRecord {
    CatalogRecord {
        id: 30,
        mnemonic: "RL-0030",
        min_spacing_m: 105,
        max_speed_kph: 55,
        flags: 0xd358a8ea,
    }
}

pub fn record_0031() -> CatalogRecord {
    CatalogRecord {
        id: 31,
        mnemonic: "RL-0031",
        min_spacing_m: 112,
        max_speed_kph: 66,
        flags: 0xd6f64825,
    }
}

pub fn record_0032() -> CatalogRecord {
    CatalogRecord {
        id: 32,
        mnemonic: "RL-0032",
        min_spacing_m: 119,
        max_speed_kph: 77,
        flags: 0xda13e760,
    }
}

pub fn record_0033() -> CatalogRecord {
    CatalogRecord {
        id: 33,
        mnemonic: "RL-0033",
        min_spacing_m: 13,
        max_speed_kph: 88,
        flags: 0xc1b1869b,
    }
}

pub fn record_0034() -> CatalogRecord {
    CatalogRecord {
        id: 34,
        mnemonic: "RL-0034",
        min_spacing_m: 20,
        max_speed_kph: 99,
        flags: 0xc5cf25d6,
    }
}

pub fn record_0035() -> CatalogRecord {
    CatalogRecord {
        id: 35,
        mnemonic: "RL-0035",
        min_spacing_m: 27,
        max_speed_kph: 110,
        flags: 0xc96cc511,
    }
}

pub fn record_0036() -> CatalogRecord {
    CatalogRecord {
        id: 36,
        mnemonic: "RL-0036",
        min_spacing_m: 34,
        max_speed_kph: 121,
        flags: 0xcc8a644c,
    }
}

pub fn record_0037() -> CatalogRecord {
    CatalogRecord {
        id: 37,
        mnemonic: "RL-0037",
        min_spacing_m: 41,
        max_speed_kph: 132,
        flags: 0xf0280387,
    }
}

pub fn record_0038() -> CatalogRecord {
    CatalogRecord {
        id: 38,
        mnemonic: "RL-0038",
        min_spacing_m: 48,
        max_speed_kph: 143,
        flags: 0xf445a2c2,
    }
}

pub fn record_0039() -> CatalogRecord {
    CatalogRecord {
        id: 39,
        mnemonic: "RL-0039",
        min_spacing_m: 55,
        max_speed_kph: 154,
        flags: 0xfbe341fd,
    }
}

pub fn record_0040() -> CatalogRecord {
    CatalogRecord {
        id: 40,
        mnemonic: "RL-0040",
        min_spacing_m: 62,
        max_speed_kph: 20,
        flags: 0xff00e138,
    }
}

pub fn record_0041() -> CatalogRecord {
    CatalogRecord {
        id: 41,
        mnemonic: "RL-0041",
        min_spacing_m: 69,
        max_speed_kph: 31,
        flags: 0xe35e8073,
    }
}

pub fn record_0042() -> CatalogRecord {
    CatalogRecord {
        id: 42,
        mnemonic: "RL-0042",
        min_spacing_m: 76,
        max_speed_kph: 42,
        flags: 0xe6fc1fae,
    }
}

pub fn record_0043() -> CatalogRecord {
    CatalogRecord {
        id: 43,
        mnemonic: "RL-0043",
        min_spacing_m: 83,
        max_speed_kph: 53,
        flags: 0xea19bee9,
    }
}

pub fn record_0044() -> CatalogRecord {
    CatalogRecord {
        id: 44,
        mnemonic: "RL-0044",
        min_spacing_m: 90,
        max_speed_kph: 64,
        flags: 0x91b75e24,
    }
}

pub fn record_0045() -> CatalogRecord {
    CatalogRecord {
        id: 45,
        mnemonic: "RL-0045",
        min_spacing_m: 97,
        max_speed_kph: 75,
        flags: 0x95d4fd5f,
    }
}

pub fn record_0046() -> CatalogRecord {
    CatalogRecord {
        id: 46,
        mnemonic: "RL-0046",
        min_spacing_m: 104,
        max_speed_kph: 86,
        flags: 0x99729c9a,
    }
}

pub fn record_0047() -> CatalogRecord {
    CatalogRecord {
        id: 47,
        mnemonic: "RL-0047",
        min_spacing_m: 111,
        max_speed_kph: 97,
        flags: 0x9c903bd5,
    }
}

pub fn record_0048() -> CatalogRecord {
    CatalogRecord {
        id: 48,
        mnemonic: "RL-0048",
        min_spacing_m: 118,
        max_speed_kph: 108,
        flags: 0x802ddb10,
    }
}

pub fn record_0049() -> CatalogRecord {
    CatalogRecord {
        id: 49,
        mnemonic: "RL-0049",
        min_spacing_m: 12,
        max_speed_kph: 119,
        flags: 0x844b7a4b,
    }
}

pub fn record_0050() -> CatalogRecord {
    CatalogRecord {
        id: 50,
        mnemonic: "RL-0050",
        min_spacing_m: 19,
        max_speed_kph: 130,
        flags: 0x8be91986,
    }
}

pub fn record_0051() -> CatalogRecord {
    CatalogRecord {
        id: 51,
        mnemonic: "RL-0051",
        min_spacing_m: 26,
        max_speed_kph: 141,
        flags: 0x8f06b8c1,
    }
}

pub fn record_0052() -> CatalogRecord {
    CatalogRecord {
        id: 52,
        mnemonic: "RL-0052",
        min_spacing_m: 33,
        max_speed_kph: 152,
        flags: 0xb2a457fc,
    }
}

pub fn record_0053() -> CatalogRecord {
    CatalogRecord {
        id: 53,
        mnemonic: "RL-0053",
        min_spacing_m: 40,
        max_speed_kph: 18,
        flags: 0xb6c1f737,
    }
}

pub fn record_0054() -> CatalogRecord {
    CatalogRecord {
        id: 54,
        mnemonic: "RL-0054",
        min_spacing_m: 47,
        max_speed_kph: 29,
        flags: 0xba1f9672,
    }
}

pub fn record_0055() -> CatalogRecord {
    CatalogRecord {
        id: 55,
        mnemonic: "RL-0055",
        min_spacing_m: 54,
        max_speed_kph: 40,
        flags: 0xa1bd35ad,
    }
}

pub fn record_0056() -> CatalogRecord {
    CatalogRecord {
        id: 56,
        mnemonic: "RL-0056",
        min_spacing_m: 61,
        max_speed_kph: 51,
        flags: 0xa5dad4e8,
    }
}

pub fn record_0057() -> CatalogRecord {
    CatalogRecord {
        id: 57,
        mnemonic: "RL-0057",
        min_spacing_m: 68,
        max_speed_kph: 62,
        flags: 0xa9787423,
    }
}

pub fn record_0058() -> CatalogRecord {
    CatalogRecord {
        id: 58,
        mnemonic: "RL-0058",
        min_spacing_m: 75,
        max_speed_kph: 73,
        flags: 0xac96135e,
    }
}

pub fn record_0059() -> CatalogRecord {
    CatalogRecord {
        id: 59,
        mnemonic: "RL-0059",
        min_spacing_m: 82,
        max_speed_kph: 84,
        flags: 0x5033b299,
    }
}

pub fn record_0060() -> CatalogRecord {
    CatalogRecord {
        id: 60,
        mnemonic: "RL-0060",
        min_spacing_m: 89,
        max_speed_kph: 95,
        flags: 0x545151d4,
    }
}

pub fn record_0061() -> CatalogRecord {
    CatalogRecord {
        id: 61,
        mnemonic: "RL-0061",
        min_spacing_m: 96,
        max_speed_kph: 106,
        flags: 0x5beef10f,
    }
}

pub fn record_0062() -> CatalogRecord {
    CatalogRecord {
        id: 62,
        mnemonic: "RL-0062",
        min_spacing_m: 103,
        max_speed_kph: 117,
        flags: 0x5f0c904a,
    }
}

pub fn record_0063() -> CatalogRecord {
    CatalogRecord {
        id: 63,
        mnemonic: "RL-0063",
        min_spacing_m: 110,
        max_speed_kph: 128,
        flags: 0x42aa2f85,
    }
}

pub fn record_0064() -> CatalogRecord {
    CatalogRecord {
        id: 64,
        mnemonic: "RL-0064",
        min_spacing_m: 117,
        max_speed_kph: 139,
        flags: 0x46c7cec0,
    }
}

pub fn record_0065() -> CatalogRecord {
    CatalogRecord {
        id: 65,
        mnemonic: "RL-0065",
        min_spacing_m: 11,
        max_speed_kph: 150,
        flags: 0x4a656dfb,
    }
}

pub fn record_0066() -> CatalogRecord {
    CatalogRecord {
        id: 66,
        mnemonic: "RL-0066",
        min_spacing_m: 18,
        max_speed_kph: 16,
        flags: 0x71830d36,
    }
}

pub fn record_0067() -> CatalogRecord {
    CatalogRecord {
        id: 67,
        mnemonic: "RL-0067",
        min_spacing_m: 25,
        max_speed_kph: 27,
        flags: 0x7520ac71,
    }
}

pub fn record_0068() -> CatalogRecord {
    CatalogRecord {
        id: 68,
        mnemonic: "RL-0068",
        min_spacing_m: 32,
        max_speed_kph: 38,
        flags: 0x797e4bac,
    }
}

pub fn record_0069() -> CatalogRecord {
    CatalogRecord {
        id: 69,
        mnemonic: "RL-0069",
        min_spacing_m: 39,
        max_speed_kph: 49,
        flags: 0x7c9beae7,
    }
}

pub fn record_0070() -> CatalogRecord {
    CatalogRecord {
        id: 70,
        mnemonic: "RL-0070",
        min_spacing_m: 46,
        max_speed_kph: 60,
        flags: 0x60398a22,
    }
}

pub fn record_0071() -> CatalogRecord {
    CatalogRecord {
        id: 71,
        mnemonic: "RL-0071",
        min_spacing_m: 53,
        max_speed_kph: 71,
        flags: 0x6457295d,
    }
}

pub fn record_0072() -> CatalogRecord {
    CatalogRecord {
        id: 72,
        mnemonic: "RL-0072",
        min_spacing_m: 60,
        max_speed_kph: 82,
        flags: 0x6bf4c898,
    }
}

pub fn record_0073() -> CatalogRecord {
    CatalogRecord {
        id: 73,
        mnemonic: "RL-0073",
        min_spacing_m: 67,
        max_speed_kph: 93,
        flags: 0x6f1267d3,
    }
}

pub fn record_0074() -> CatalogRecord {
    CatalogRecord {
        id: 74,
        mnemonic: "RL-0074",
        min_spacing_m: 74,
        max_speed_kph: 104,
        flags: 0x12b0070e,
    }
}

pub fn record_0075() -> CatalogRecord {
    CatalogRecord {
        id: 75,
        mnemonic: "RL-0075",
        min_spacing_m: 81,
        max_speed_kph: 115,
        flags: 0x16cda649,
    }
}

pub fn record_0076() -> CatalogRecord {
    CatalogRecord {
        id: 76,
        mnemonic: "RL-0076",
        min_spacing_m: 88,
        max_speed_kph: 126,
        flags: 0x1a6b4584,
    }
}

pub fn record_0077() -> CatalogRecord {
    CatalogRecord {
        id: 77,
        mnemonic: "RL-0077",
        min_spacing_m: 95,
        max_speed_kph: 137,
        flags: 0x0188e4bf,
    }
}

pub fn record_0078() -> CatalogRecord {
    CatalogRecord {
        id: 78,
        mnemonic: "RL-0078",
        min_spacing_m: 102,
        max_speed_kph: 148,
        flags: 0x052683fa,
    }
}

pub fn record_0079() -> CatalogRecord {
    CatalogRecord {
        id: 79,
        mnemonic: "RL-0079",
        min_spacing_m: 109,
        max_speed_kph: 159,
        flags: 0x09442335,
    }
}

pub fn record_0080() -> CatalogRecord {
    CatalogRecord {
        id: 80,
        mnemonic: "RL-0080",
        min_spacing_m: 116,
        max_speed_kph: 25,
        flags: 0x0ce1c270,
    }
}

pub fn record_0081() -> CatalogRecord {
    CatalogRecord {
        id: 81,
        mnemonic: "RL-0081",
        min_spacing_m: 10,
        max_speed_kph: 36,
        flags: 0x303f61ab,
    }
}

pub fn record_0082() -> CatalogRecord {
    CatalogRecord {
        id: 82,
        mnemonic: "RL-0082",
        min_spacing_m: 17,
        max_speed_kph: 47,
        flags: 0x345d00e6,
    }
}

pub fn record_0083() -> CatalogRecord {
    CatalogRecord {
        id: 83,
        mnemonic: "RL-0083",
        min_spacing_m: 24,
        max_speed_kph: 58,
        flags: 0x3bfaa021,
    }
}

pub fn record_0084() -> CatalogRecord {
    CatalogRecord {
        id: 84,
        mnemonic: "RL-0084",
        min_spacing_m: 31,
        max_speed_kph: 69,
        flags: 0x3f183f5c,
    }
}

pub fn record_0085() -> CatalogRecord {
    CatalogRecord {
        id: 85,
        mnemonic: "RL-0085",
        min_spacing_m: 38,
        max_speed_kph: 80,
        flags: 0x22b5de97,
    }
}

pub fn record_0086() -> CatalogRecord {
    CatalogRecord {
        id: 86,
        mnemonic: "RL-0086",
        min_spacing_m: 45,
        max_speed_kph: 91,
        flags: 0x26d37dd2,
    }
}

pub fn record_0087() -> CatalogRecord {
    CatalogRecord {
        id: 87,
        mnemonic: "RL-0087",
        min_spacing_m: 52,
        max_speed_kph: 102,
        flags: 0x2a711d0d,
    }
}

pub fn record_0088() -> CatalogRecord {
    CatalogRecord {
        id: 88,
        mnemonic: "RL-0088",
        min_spacing_m: 59,
        max_speed_kph: 113,
        flags: 0xd18ebc48,
    }
}

pub fn record_0089() -> CatalogRecord {
    CatalogRecord {
        id: 89,
        mnemonic: "RL-0089",
        min_spacing_m: 66,
        max_speed_kph: 124,
        flags: 0xd52c5b83,
    }
}

pub fn record_0090() -> CatalogRecord {
    CatalogRecord {
        id: 90,
        mnemonic: "RL-0090",
        min_spacing_m: 73,
        max_speed_kph: 135,
        flags: 0xd949fabe,
    }
}

pub fn record_0091() -> CatalogRecord {
    CatalogRecord {
        id: 91,
        mnemonic: "RL-0091",
        min_spacing_m: 80,
        max_speed_kph: 146,
        flags: 0xdce799f9,
    }
}

pub fn record_0092() -> CatalogRecord {
    CatalogRecord {
        id: 92,
        mnemonic: "RL-0092",
        min_spacing_m: 87,
        max_speed_kph: 157,
        flags: 0xc0053934,
    }
}

pub fn record_0093() -> CatalogRecord {
    CatalogRecord {
        id: 93,
        mnemonic: "RL-0093",
        min_spacing_m: 94,
        max_speed_kph: 23,
        flags: 0xc7a2d86f,
    }
}

pub fn record_0094() -> CatalogRecord {
    CatalogRecord {
        id: 94,
        mnemonic: "RL-0094",
        min_spacing_m: 101,
        max_speed_kph: 34,
        flags: 0xcbc077aa,
    }
}

pub fn record_0095() -> CatalogRecord {
    CatalogRecord {
        id: 95,
        mnemonic: "RL-0095",
        min_spacing_m: 108,
        max_speed_kph: 45,
        flags: 0xcf1e16e5,
    }
}

pub fn record_0096() -> CatalogRecord {
    CatalogRecord {
        id: 96,
        mnemonic: "RL-0096",
        min_spacing_m: 115,
        max_speed_kph: 56,
        flags: 0xf2bbb620,
    }
}

pub fn record_0097() -> CatalogRecord {
    CatalogRecord {
        id: 97,
        mnemonic: "RL-0097",
        min_spacing_m: 9,
        max_speed_kph: 67,
        flags: 0xf6d9555b,
    }
}

pub fn record_0098() -> CatalogRecord {
    CatalogRecord {
        id: 98,
        mnemonic: "RL-0098",
        min_spacing_m: 16,
        max_speed_kph: 78,
        flags: 0xfa76f496,
    }
}

pub fn record_0099() -> CatalogRecord {
    CatalogRecord {
        id: 99,
        mnemonic: "RL-0099",
        min_spacing_m: 23,
        max_speed_kph: 89,
        flags: 0xe19493d1,
    }
}

pub fn record_0100() -> CatalogRecord {
    CatalogRecord {
        id: 100,
        mnemonic: "RL-0100",
        min_spacing_m: 30,
        max_speed_kph: 100,
        flags: 0xe532330c,
    }
}

pub fn record_0101() -> CatalogRecord {
    CatalogRecord {
        id: 101,
        mnemonic: "RL-0101",
        min_spacing_m: 37,
        max_speed_kph: 111,
        flags: 0xe94fd247,
    }
}

pub fn record_0102() -> CatalogRecord {
    CatalogRecord {
        id: 102,
        mnemonic: "RL-0102",
        min_spacing_m: 44,
        max_speed_kph: 122,
        flags: 0xeced7182,
    }
}

pub fn record_0103() -> CatalogRecord {
    CatalogRecord {
        id: 103,
        mnemonic: "RL-0103",
        min_spacing_m: 51,
        max_speed_kph: 133,
        flags: 0x900b10bd,
    }
}

pub fn record_0104() -> CatalogRecord {
    CatalogRecord {
        id: 104,
        mnemonic: "RL-0104",
        min_spacing_m: 58,
        max_speed_kph: 144,
        flags: 0x97a8aff8,
    }
}

pub fn record_0105() -> CatalogRecord {
    CatalogRecord {
        id: 105,
        mnemonic: "RL-0105",
        min_spacing_m: 65,
        max_speed_kph: 155,
        flags: 0x9bc64f33,
    }
}

pub fn record_0106() -> CatalogRecord {
    CatalogRecord {
        id: 106,
        mnemonic: "RL-0106",
        min_spacing_m: 72,
        max_speed_kph: 21,
        flags: 0x9f63ee6e,
    }
}

pub fn record_0107() -> CatalogRecord {
    CatalogRecord {
        id: 107,
        mnemonic: "RL-0107",
        min_spacing_m: 79,
        max_speed_kph: 32,
        flags: 0x82818da9,
    }
}

pub fn record_0108() -> CatalogRecord {
    CatalogRecord {
        id: 108,
        mnemonic: "RL-0108",
        min_spacing_m: 86,
        max_speed_kph: 43,
        flags: 0x86df2ce4,
    }
}

pub fn record_0109() -> CatalogRecord {
    CatalogRecord {
        id: 109,
        mnemonic: "RL-0109",
        min_spacing_m: 93,
        max_speed_kph: 54,
        flags: 0x8a7ccc1f,
    }
}

pub fn record_0110() -> CatalogRecord {
    CatalogRecord {
        id: 110,
        mnemonic: "RL-0110",
        min_spacing_m: 100,
        max_speed_kph: 65,
        flags: 0xb19a6b5a,
    }
}

pub fn record_0111() -> CatalogRecord {
    CatalogRecord {
        id: 111,
        mnemonic: "RL-0111",
        min_spacing_m: 107,
        max_speed_kph: 76,
        flags: 0xb5380a95,
    }
}

pub fn record_0112() -> CatalogRecord {
    CatalogRecord {
        id: 112,
        mnemonic: "RL-0112",
        min_spacing_m: 114,
        max_speed_kph: 87,
        flags: 0xb955a9d0,
    }
}

pub fn record_0113() -> CatalogRecord {
    CatalogRecord {
        id: 113,
        mnemonic: "RL-0113",
        min_spacing_m: 8,
        max_speed_kph: 98,
        flags: 0xbcf3490b,
    }
}

pub fn record_0114() -> CatalogRecord {
    CatalogRecord {
        id: 114,
        mnemonic: "RL-0114",
        min_spacing_m: 15,
        max_speed_kph: 109,
        flags: 0xa010e846,
    }
}

pub fn record_0115() -> CatalogRecord {
    CatalogRecord {
        id: 115,
        mnemonic: "RL-0115",
        min_spacing_m: 22,
        max_speed_kph: 120,
        flags: 0xa7ae8781,
    }
}

pub fn record_0116() -> CatalogRecord {
    CatalogRecord {
        id: 116,
        mnemonic: "RL-0116",
        min_spacing_m: 29,
        max_speed_kph: 131,
        flags: 0xabcc26bc,
    }
}

pub fn record_0117() -> CatalogRecord {
    CatalogRecord {
        id: 117,
        mnemonic: "RL-0117",
        min_spacing_m: 36,
        max_speed_kph: 142,
        flags: 0xaf69c5f7,
    }
}

pub fn record_0118() -> CatalogRecord {
    CatalogRecord {
        id: 118,
        mnemonic: "RL-0118",
        min_spacing_m: 43,
        max_speed_kph: 153,
        flags: 0x52876532,
    }
}

pub fn record_0119() -> CatalogRecord {
    CatalogRecord {
        id: 119,
        mnemonic: "RL-0119",
        min_spacing_m: 50,
        max_speed_kph: 19,
        flags: 0x5625046d,
    }
}

pub fn record_0120() -> CatalogRecord {
    CatalogRecord {
        id: 120,
        mnemonic: "RL-0120",
        min_spacing_m: 57,
        max_speed_kph: 30,
        flags: 0x5a42a3a8,
    }
}

pub fn record_0121() -> CatalogRecord {
    CatalogRecord {
        id: 121,
        mnemonic: "RL-0121",
        min_spacing_m: 64,
        max_speed_kph: 41,
        flags: 0x41e042e3,
    }
}

pub fn record_0122() -> CatalogRecord {
    CatalogRecord {
        id: 122,
        mnemonic: "RL-0122",
        min_spacing_m: 71,
        max_speed_kph: 52,
        flags: 0x453de21e,
    }
}

pub fn record_0123() -> CatalogRecord {
    CatalogRecord {
        id: 123,
        mnemonic: "RL-0123",
        min_spacing_m: 78,
        max_speed_kph: 63,
        flags: 0x495b8159,
    }
}

pub fn record_0124() -> CatalogRecord {
    CatalogRecord {
        id: 124,
        mnemonic: "RL-0124",
        min_spacing_m: 85,
        max_speed_kph: 74,
        flags: 0x4cf92094,
    }
}

pub fn record_0125() -> CatalogRecord {
    CatalogRecord {
        id: 125,
        mnemonic: "RL-0125",
        min_spacing_m: 92,
        max_speed_kph: 85,
        flags: 0x7016bfcf,
    }
}

pub fn record_0126() -> CatalogRecord {
    CatalogRecord {
        id: 126,
        mnemonic: "RL-0126",
        min_spacing_m: 99,
        max_speed_kph: 96,
        flags: 0x77b45f0a,
    }
}

pub fn record_0127() -> CatalogRecord {
    CatalogRecord {
        id: 127,
        mnemonic: "RL-0127",
        min_spacing_m: 106,
        max_speed_kph: 107,
        flags: 0x7bd1fe45,
    }
}

pub fn record_0128() -> CatalogRecord {
    CatalogRecord {
        id: 128,
        mnemonic: "RL-0128",
        min_spacing_m: 113,
        max_speed_kph: 118,
        flags: 0x7f6f9d80,
    }
}

pub fn record_0129() -> CatalogRecord {
    CatalogRecord {
        id: 129,
        mnemonic: "RL-0129",
        min_spacing_m: 120,
        max_speed_kph: 129,
        flags: 0x628d3cbb,
    }
}

pub fn record_0130() -> CatalogRecord {
    CatalogRecord {
        id: 130,
        mnemonic: "RL-0130",
        min_spacing_m: 14,
        max_speed_kph: 140,
        flags: 0x662adbf6,
    }
}

pub fn record_0131() -> CatalogRecord {
    CatalogRecord {
        id: 131,
        mnemonic: "RL-0131",
        min_spacing_m: 21,
        max_speed_kph: 151,
        flags: 0x6a487b31,
    }
}

pub fn record_0132() -> CatalogRecord {
    CatalogRecord {
        id: 132,
        mnemonic: "RL-0132",
        min_spacing_m: 28,
        max_speed_kph: 17,
        flags: 0x11e61a6c,
    }
}

pub fn record_0133() -> CatalogRecord {
    CatalogRecord {
        id: 133,
        mnemonic: "RL-0133",
        min_spacing_m: 35,
        max_speed_kph: 28,
        flags: 0x1503b9a7,
    }
}

pub fn record_0134() -> CatalogRecord {
    CatalogRecord {
        id: 134,
        mnemonic: "RL-0134",
        min_spacing_m: 42,
        max_speed_kph: 39,
        flags: 0x18a158e2,
    }
}

pub fn record_0135() -> CatalogRecord {
    CatalogRecord {
        id: 135,
        mnemonic: "RL-0135",
        min_spacing_m: 49,
        max_speed_kph: 50,
        flags: 0x1cfef81d,
    }
}

pub fn record_0136() -> CatalogRecord {
    CatalogRecord {
        id: 136,
        mnemonic: "RL-0136",
        min_spacing_m: 56,
        max_speed_kph: 61,
        flags: 0x001c9758,
    }
}

pub fn record_0137() -> CatalogRecord {
    CatalogRecord {
        id: 137,
        mnemonic: "RL-0137",
        min_spacing_m: 63,
        max_speed_kph: 72,
        flags: 0x07ba3693,
    }
}

pub fn record_0138() -> CatalogRecord {
    CatalogRecord {
        id: 138,
        mnemonic: "RL-0138",
        min_spacing_m: 70,
        max_speed_kph: 83,
        flags: 0x0bd7d5ce,
    }
}

pub fn record_0139() -> CatalogRecord {
    CatalogRecord {
        id: 139,
        mnemonic: "RL-0139",
        min_spacing_m: 77,
        max_speed_kph: 94,
        flags: 0x0f757509,
    }
}

pub fn record_0140() -> CatalogRecord {
    CatalogRecord {
        id: 140,
        mnemonic: "RL-0140",
        min_spacing_m: 84,
        max_speed_kph: 105,
        flags: 0x32931444,
    }
}

pub fn record_0141() -> CatalogRecord {
    CatalogRecord {
        id: 141,
        mnemonic: "RL-0141",
        min_spacing_m: 91,
        max_speed_kph: 116,
        flags: 0x3630b37f,
    }
}

pub fn record_0142() -> CatalogRecord {
    CatalogRecord {
        id: 142,
        mnemonic: "RL-0142",
        min_spacing_m: 98,
        max_speed_kph: 127,
        flags: 0x3a4e52ba,
    }
}

pub fn record_0143() -> CatalogRecord {
    CatalogRecord {
        id: 143,
        mnemonic: "RL-0143",
        min_spacing_m: 105,
        max_speed_kph: 138,
        flags: 0x21ebf1f5,
    }
}

pub fn record_0144() -> CatalogRecord {
    CatalogRecord {
        id: 144,
        mnemonic: "RL-0144",
        min_spacing_m: 112,
        max_speed_kph: 149,
        flags: 0x25099130,
    }
}

pub fn record_0145() -> CatalogRecord {
    CatalogRecord {
        id: 145,
        mnemonic: "RL-0145",
        min_spacing_m: 119,
        max_speed_kph: 15,
        flags: 0x28a7306b,
    }
}

pub fn record_0146() -> CatalogRecord {
    CatalogRecord {
        id: 146,
        mnemonic: "RL-0146",
        min_spacing_m: 13,
        max_speed_kph: 26,
        flags: 0x2cc4cfa6,
    }
}

pub fn record_0147() -> CatalogRecord {
    CatalogRecord {
        id: 147,
        mnemonic: "RL-0147",
        min_spacing_m: 20,
        max_speed_kph: 37,
        flags: 0xd0626ee1,
    }
}

pub fn record_0148() -> CatalogRecord {
    CatalogRecord {
        id: 148,
        mnemonic: "RL-0148",
        min_spacing_m: 27,
        max_speed_kph: 48,
        flags: 0xd7800e1c,
    }
}

pub fn record_0149() -> CatalogRecord {
    CatalogRecord {
        id: 149,
        mnemonic: "RL-0149",
        min_spacing_m: 34,
        max_speed_kph: 59,
        flags: 0xdbddad57,
    }
}

pub fn record_0150() -> CatalogRecord {
    CatalogRecord {
        id: 150,
        mnemonic: "RL-0150",
        min_spacing_m: 41,
        max_speed_kph: 70,
        flags: 0xdf7b4c92,
    }
}

pub fn record_0151() -> CatalogRecord {
    CatalogRecord {
        id: 151,
        mnemonic: "RL-0151",
        min_spacing_m: 48,
        max_speed_kph: 81,
        flags: 0xc298ebcd,
    }
}

pub fn record_0152() -> CatalogRecord {
    CatalogRecord {
        id: 152,
        mnemonic: "RL-0152",
        min_spacing_m: 55,
        max_speed_kph: 92,
        flags: 0xc6368b08,
    }
}

pub fn record_0153() -> CatalogRecord {
    CatalogRecord {
        id: 153,
        mnemonic: "RL-0153",
        min_spacing_m: 62,
        max_speed_kph: 103,
        flags: 0xca542a43,
    }
}

pub fn record_0154() -> CatalogRecord {
    CatalogRecord {
        id: 154,
        mnemonic: "RL-0154",
        min_spacing_m: 69,
        max_speed_kph: 114,
        flags: 0xf1f1c97e,
    }
}

pub fn record_0155() -> CatalogRecord {
    CatalogRecord {
        id: 155,
        mnemonic: "RL-0155",
        min_spacing_m: 76,
        max_speed_kph: 125,
        flags: 0xf50f68b9,
    }
}

pub fn record_0156() -> CatalogRecord {
    CatalogRecord {
        id: 156,
        mnemonic: "RL-0156",
        min_spacing_m: 83,
        max_speed_kph: 136,
        flags: 0xf8ad07f4,
    }
}

pub fn record_0157() -> CatalogRecord {
    CatalogRecord {
        id: 157,
        mnemonic: "RL-0157",
        min_spacing_m: 90,
        max_speed_kph: 147,
        flags: 0xfccaa72f,
    }
}

pub fn record_0158() -> CatalogRecord {
    CatalogRecord {
        id: 158,
        mnemonic: "RL-0158",
        min_spacing_m: 97,
        max_speed_kph: 158,
        flags: 0xe068466a,
    }
}

pub fn record_0159() -> CatalogRecord {
    CatalogRecord {
        id: 159,
        mnemonic: "RL-0159",
        min_spacing_m: 104,
        max_speed_kph: 24,
        flags: 0xe785e5a5,
    }
}

pub fn record_0160() -> CatalogRecord {
    CatalogRecord {
        id: 160,
        mnemonic: "RL-0160",
        min_spacing_m: 111,
        max_speed_kph: 35,
        flags: 0xeb2384e0,
    }
}

pub fn record_0161() -> CatalogRecord {
    CatalogRecord {
        id: 161,
        mnemonic: "RL-0161",
        min_spacing_m: 118,
        max_speed_kph: 46,
        flags: 0xef41241b,
    }
}

pub fn record_0162() -> CatalogRecord {
    CatalogRecord {
        id: 162,
        mnemonic: "RL-0162",
        min_spacing_m: 12,
        max_speed_kph: 57,
        flags: 0x929ec356,
    }
}

pub fn record_0163() -> CatalogRecord {
    CatalogRecord {
        id: 163,
        mnemonic: "RL-0163",
        min_spacing_m: 19,
        max_speed_kph: 68,
        flags: 0x963c6291,
    }
}

pub fn record_0164() -> CatalogRecord {
    CatalogRecord {
        id: 164,
        mnemonic: "RL-0164",
        min_spacing_m: 26,
        max_speed_kph: 79,
        flags: 0x9a5a01cc,
    }
}

pub fn record_0165() -> CatalogRecord {
    CatalogRecord {
        id: 165,
        mnemonic: "RL-0165",
        min_spacing_m: 33,
        max_speed_kph: 90,
        flags: 0x81f7a107,
    }
}

pub fn record_0166() -> CatalogRecord {
    CatalogRecord {
        id: 166,
        mnemonic: "RL-0166",
        min_spacing_m: 40,
        max_speed_kph: 101,
        flags: 0x85154042,
    }
}

pub fn record_0167() -> CatalogRecord {
    CatalogRecord {
        id: 167,
        mnemonic: "RL-0167",
        min_spacing_m: 47,
        max_speed_kph: 112,
        flags: 0x88b2df7d,
    }
}

pub fn record_0168() -> CatalogRecord {
    CatalogRecord {
        id: 168,
        mnemonic: "RL-0168",
        min_spacing_m: 54,
        max_speed_kph: 123,
        flags: 0x8cd07eb8,
    }
}

pub fn record_0169() -> CatalogRecord {
    CatalogRecord {
        id: 169,
        mnemonic: "RL-0169",
        min_spacing_m: 61,
        max_speed_kph: 134,
        flags: 0xb06e1df3,
    }
}

pub fn record_0170() -> CatalogRecord {
    CatalogRecord {
        id: 170,
        mnemonic: "RL-0170",
        min_spacing_m: 68,
        max_speed_kph: 145,
        flags: 0xb78bbd2e,
    }
}

pub fn record_0171() -> CatalogRecord {
    CatalogRecord {
        id: 171,
        mnemonic: "RL-0171",
        min_spacing_m: 75,
        max_speed_kph: 156,
        flags: 0xbb295c69,
    }
}

pub fn record_0172() -> CatalogRecord {
    CatalogRecord {
        id: 172,
        mnemonic: "RL-0172",
        min_spacing_m: 82,
        max_speed_kph: 22,
        flags: 0xbf46fba4,
    }
}

pub fn record_0173() -> CatalogRecord {
    CatalogRecord {
        id: 173,
        mnemonic: "RL-0173",
        min_spacing_m: 89,
        max_speed_kph: 33,
        flags: 0xa2e49adf,
    }
}

pub fn record_0174() -> CatalogRecord {
    CatalogRecord {
        id: 174,
        mnemonic: "RL-0174",
        min_spacing_m: 96,
        max_speed_kph: 44,
        flags: 0xa6023a1a,
    }
}

pub fn record_0175() -> CatalogRecord {
    CatalogRecord {
        id: 175,
        mnemonic: "RL-0175",
        min_spacing_m: 103,
        max_speed_kph: 55,
        flags: 0xaa5fd955,
    }
}

pub fn record_0176() -> CatalogRecord {
    CatalogRecord {
        id: 176,
        mnemonic: "RL-0176",
        min_spacing_m: 110,
        max_speed_kph: 66,
        flags: 0x51fd7890,
    }
}

pub fn record_0177() -> CatalogRecord {
    CatalogRecord {
        id: 177,
        mnemonic: "RL-0177",
        min_spacing_m: 117,
        max_speed_kph: 77,
        flags: 0x551b17cb,
    }
}

pub fn record_0178() -> CatalogRecord {
    CatalogRecord {
        id: 178,
        mnemonic: "RL-0178",
        min_spacing_m: 11,
        max_speed_kph: 88,
        flags: 0x58b8b706,
    }
}

pub fn record_0179() -> CatalogRecord {
    CatalogRecord {
        id: 179,
        mnemonic: "RL-0179",
        min_spacing_m: 18,
        max_speed_kph: 99,
        flags: 0x5cd65641,
    }
}

pub fn record_0180() -> CatalogRecord {
    CatalogRecord {
        id: 180,
        mnemonic: "RL-0180",
        min_spacing_m: 25,
        max_speed_kph: 110,
        flags: 0x4073f57c,
    }
}

pub fn record_0181() -> CatalogRecord {
    CatalogRecord {
        id: 181,
        mnemonic: "RL-0181",
        min_spacing_m: 32,
        max_speed_kph: 121,
        flags: 0x479194b7,
    }
}

pub fn record_0182() -> CatalogRecord {
    CatalogRecord {
        id: 182,
        mnemonic: "RL-0182",
        min_spacing_m: 39,
        max_speed_kph: 132,
        flags: 0x4b2f33f2,
    }
}

pub fn record_0183() -> CatalogRecord {
    CatalogRecord {
        id: 183,
        mnemonic: "RL-0183",
        min_spacing_m: 46,
        max_speed_kph: 143,
        flags: 0x4f4cd32d,
    }
}

pub fn record_0184() -> CatalogRecord {
    CatalogRecord {
        id: 184,
        mnemonic: "RL-0184",
        min_spacing_m: 53,
        max_speed_kph: 154,
        flags: 0x72ea7268,
    }
}

pub fn record_0185() -> CatalogRecord {
    CatalogRecord {
        id: 185,
        mnemonic: "RL-0185",
        min_spacing_m: 60,
        max_speed_kph: 20,
        flags: 0x760811a3,
    }
}

pub fn record_0186() -> CatalogRecord {
    CatalogRecord {
        id: 186,
        mnemonic: "RL-0186",
        min_spacing_m: 67,
        max_speed_kph: 31,
        flags: 0x7da5b0de,
    }
}

pub fn record_0187() -> CatalogRecord {
    CatalogRecord {
        id: 187,
        mnemonic: "RL-0187",
        min_spacing_m: 74,
        max_speed_kph: 42,
        flags: 0x61c35019,
    }
}

pub fn record_0188() -> CatalogRecord {
    CatalogRecord {
        id: 188,
        mnemonic: "RL-0188",
        min_spacing_m: 81,
        max_speed_kph: 53,
        flags: 0x6560ef54,
    }
}

pub fn record_0189() -> CatalogRecord {
    CatalogRecord {
        id: 189,
        mnemonic: "RL-0189",
        min_spacing_m: 88,
        max_speed_kph: 64,
        flags: 0x68be8e8f,
    }
}

pub fn record_0190() -> CatalogRecord {
    CatalogRecord {
        id: 190,
        mnemonic: "RL-0190",
        min_spacing_m: 95,
        max_speed_kph: 75,
        flags: 0x6cdc2dca,
    }
}

pub fn record_0191() -> CatalogRecord {
    CatalogRecord {
        id: 191,
        mnemonic: "RL-0191",
        min_spacing_m: 102,
        max_speed_kph: 86,
        flags: 0x1079cd05,
    }
}

pub fn record_0192() -> CatalogRecord {
    CatalogRecord {
        id: 192,
        mnemonic: "RL-0192",
        min_spacing_m: 109,
        max_speed_kph: 97,
        flags: 0x17976c40,
    }
}

pub fn record_0193() -> CatalogRecord {
    CatalogRecord {
        id: 193,
        mnemonic: "RL-0193",
        min_spacing_m: 116,
        max_speed_kph: 108,
        flags: 0x1b350b7b,
    }
}

pub fn record_0194() -> CatalogRecord {
    CatalogRecord {
        id: 194,
        mnemonic: "RL-0194",
        min_spacing_m: 10,
        max_speed_kph: 119,
        flags: 0x1f52aab6,
    }
}

pub fn record_0195() -> CatalogRecord {
    CatalogRecord {
        id: 195,
        mnemonic: "RL-0195",
        min_spacing_m: 17,
        max_speed_kph: 130,
        flags: 0x02f049f1,
    }
}

pub fn record_0196() -> CatalogRecord {
    CatalogRecord {
        id: 196,
        mnemonic: "RL-0196",
        min_spacing_m: 24,
        max_speed_kph: 141,
        flags: 0x060de92c,
    }
}

pub fn record_0197() -> CatalogRecord {
    CatalogRecord {
        id: 197,
        mnemonic: "RL-0197",
        min_spacing_m: 31,
        max_speed_kph: 152,
        flags: 0x0dab8867,
    }
}

pub fn record_0198() -> CatalogRecord {
    CatalogRecord {
        id: 198,
        mnemonic: "RL-0198",
        min_spacing_m: 38,
        max_speed_kph: 18,
        flags: 0x31c927a2,
    }
}

pub fn record_0199() -> CatalogRecord {
    CatalogRecord {
        id: 199,
        mnemonic: "RL-0199",
        min_spacing_m: 45,
        max_speed_kph: 29,
        flags: 0x3566c6dd,
    }
}

pub fn record_0200() -> CatalogRecord {
    CatalogRecord {
        id: 200,
        mnemonic: "RL-0200",
        min_spacing_m: 52,
        max_speed_kph: 40,
        flags: 0x38846618,
    }
}

pub fn record_0201() -> CatalogRecord {
    CatalogRecord {
        id: 201,
        mnemonic: "RL-0201",
        min_spacing_m: 59,
        max_speed_kph: 51,
        flags: 0x3c220553,
    }
}

pub fn record_0202() -> CatalogRecord {
    CatalogRecord {
        id: 202,
        mnemonic: "RL-0202",
        min_spacing_m: 66,
        max_speed_kph: 62,
        flags: 0x207fa48e,
    }
}

pub fn record_0203() -> CatalogRecord {
    CatalogRecord {
        id: 203,
        mnemonic: "RL-0203",
        min_spacing_m: 73,
        max_speed_kph: 73,
        flags: 0x279d43c9,
    }
}

pub fn record_0204() -> CatalogRecord {
    CatalogRecord {
        id: 204,
        mnemonic: "RL-0204",
        min_spacing_m: 80,
        max_speed_kph: 84,
        flags: 0x2b3ae304,
    }
}

pub fn record_0205() -> CatalogRecord {
    CatalogRecord {
        id: 205,
        mnemonic: "RL-0205",
        min_spacing_m: 87,
        max_speed_kph: 95,
        flags: 0x2f58823f,
    }
}

pub fn record_0206() -> CatalogRecord {
    CatalogRecord {
        id: 206,
        mnemonic: "RL-0206",
        min_spacing_m: 94,
        max_speed_kph: 106,
        flags: 0xd2f6217a,
    }
}

pub fn record_0207() -> CatalogRecord {
    CatalogRecord {
        id: 207,
        mnemonic: "RL-0207",
        min_spacing_m: 101,
        max_speed_kph: 117,
        flags: 0xd613c0b5,
    }
}

pub fn record_0208() -> CatalogRecord {
    CatalogRecord {
        id: 208,
        mnemonic: "RL-0208",
        min_spacing_m: 108,
        max_speed_kph: 128,
        flags: 0xddb15ff0,
    }
}

pub fn record_0209() -> CatalogRecord {
    CatalogRecord {
        id: 209,
        mnemonic: "RL-0209",
        min_spacing_m: 115,
        max_speed_kph: 139,
        flags: 0xc1ceff2b,
    }
}

pub fn record_0210() -> CatalogRecord {
    CatalogRecord {
        id: 210,
        mnemonic: "RL-0210",
        min_spacing_m: 9,
        max_speed_kph: 150,
        flags: 0xc56c9e66,
    }
}

pub fn record_0211() -> CatalogRecord {
    CatalogRecord {
        id: 211,
        mnemonic: "RL-0211",
        min_spacing_m: 16,
        max_speed_kph: 16,
        flags: 0xc88a3da1,
    }
}

pub fn record_0212() -> CatalogRecord {
    CatalogRecord {
        id: 212,
        mnemonic: "RL-0212",
        min_spacing_m: 23,
        max_speed_kph: 27,
        flags: 0xcc27dcdc,
    }
}

pub fn record_0213() -> CatalogRecord {
    CatalogRecord {
        id: 213,
        mnemonic: "RL-0213",
        min_spacing_m: 30,
        max_speed_kph: 38,
        flags: 0xf0457c17,
    }
}

pub fn record_0214() -> CatalogRecord {
    CatalogRecord {
        id: 214,
        mnemonic: "RL-0214",
        min_spacing_m: 37,
        max_speed_kph: 49,
        flags: 0xf7e31b52,
    }
}

pub fn record_0215() -> CatalogRecord {
    CatalogRecord {
        id: 215,
        mnemonic: "RL-0215",
        min_spacing_m: 44,
        max_speed_kph: 60,
        flags: 0xfb00ba8d,
    }
}

pub fn record_0216() -> CatalogRecord {
    CatalogRecord {
        id: 216,
        mnemonic: "RL-0216",
        min_spacing_m: 51,
        max_speed_kph: 71,
        flags: 0xff5e59c8,
    }
}

pub fn record_0217() -> CatalogRecord {
    CatalogRecord {
        id: 217,
        mnemonic: "RL-0217",
        min_spacing_m: 58,
        max_speed_kph: 82,
        flags: 0xe2fbf903,
    }
}

pub fn record_0218() -> CatalogRecord {
    CatalogRecord {
        id: 218,
        mnemonic: "RL-0218",
        min_spacing_m: 65,
        max_speed_kph: 93,
        flags: 0xe619983e,
    }
}

pub fn record_0219() -> CatalogRecord {
    CatalogRecord {
        id: 219,
        mnemonic: "RL-0219",
        min_spacing_m: 72,
        max_speed_kph: 104,
        flags: 0xedb73779,
    }
}

pub fn record_0220() -> CatalogRecord {
    CatalogRecord {
        id: 220,
        mnemonic: "RL-0220",
        min_spacing_m: 79,
        max_speed_kph: 115,
        flags: 0x91d4d6b4,
    }
}

pub fn record_0221() -> CatalogRecord {
    CatalogRecord {
        id: 221,
        mnemonic: "RL-0221",
        min_spacing_m: 86,
        max_speed_kph: 126,
        flags: 0x957275ef,
    }
}

pub fn record_0222() -> CatalogRecord {
    CatalogRecord {
        id: 222,
        mnemonic: "RL-0222",
        min_spacing_m: 93,
        max_speed_kph: 137,
        flags: 0x9890152a,
    }
}

pub fn record_0223() -> CatalogRecord {
    CatalogRecord {
        id: 223,
        mnemonic: "RL-0223",
        min_spacing_m: 100,
        max_speed_kph: 148,
        flags: 0x9c2db465,
    }
}

pub fn record_0224() -> CatalogRecord {
    CatalogRecord {
        id: 224,
        mnemonic: "RL-0224",
        min_spacing_m: 107,
        max_speed_kph: 159,
        flags: 0x804b53a0,
    }
}

pub fn record_0225() -> CatalogRecord {
    CatalogRecord {
        id: 225,
        mnemonic: "RL-0225",
        min_spacing_m: 114,
        max_speed_kph: 25,
        flags: 0x87e8f2db,
    }
}

pub fn record_0226() -> CatalogRecord {
    CatalogRecord {
        id: 226,
        mnemonic: "RL-0226",
        min_spacing_m: 8,
        max_speed_kph: 36,
        flags: 0x8b069216,
    }
}

pub fn record_0227() -> CatalogRecord {
    CatalogRecord {
        id: 227,
        mnemonic: "RL-0227",
        min_spacing_m: 15,
        max_speed_kph: 47,
        flags: 0x8ea43151,
    }
}

pub fn record_0228() -> CatalogRecord {
    CatalogRecord {
        id: 228,
        mnemonic: "RL-0228",
        min_spacing_m: 22,
        max_speed_kph: 58,
        flags: 0xb2c1d08c,
    }
}

pub fn record_0229() -> CatalogRecord {
    CatalogRecord {
        id: 229,
        mnemonic: "RL-0229",
        min_spacing_m: 29,
        max_speed_kph: 69,
        flags: 0xb61f6fc7,
    }
}

pub fn record_0230() -> CatalogRecord {
    CatalogRecord {
        id: 230,
        mnemonic: "RL-0230",
        min_spacing_m: 36,
        max_speed_kph: 80,
        flags: 0xbdbd0f02,
    }
}

pub fn record_0231() -> CatalogRecord {
    CatalogRecord {
        id: 231,
        mnemonic: "RL-0231",
        min_spacing_m: 43,
        max_speed_kph: 91,
        flags: 0xa1daae3d,
    }
}

pub fn record_0232() -> CatalogRecord {
    CatalogRecord {
        id: 232,
        mnemonic: "RL-0232",
        min_spacing_m: 50,
        max_speed_kph: 102,
        flags: 0xa5784d78,
    }
}

pub fn record_0233() -> CatalogRecord {
    CatalogRecord {
        id: 233,
        mnemonic: "RL-0233",
        min_spacing_m: 57,
        max_speed_kph: 113,
        flags: 0xa895ecb3,
    }
}

pub fn record_0234() -> CatalogRecord {
    CatalogRecord {
        id: 234,
        mnemonic: "RL-0234",
        min_spacing_m: 64,
        max_speed_kph: 124,
        flags: 0xac338bee,
    }
}

pub fn record_0235() -> CatalogRecord {
    CatalogRecord {
        id: 235,
        mnemonic: "RL-0235",
        min_spacing_m: 71,
        max_speed_kph: 135,
        flags: 0x50512b29,
    }
}

pub fn record_0236() -> CatalogRecord {
    CatalogRecord {
        id: 236,
        mnemonic: "RL-0236",
        min_spacing_m: 78,
        max_speed_kph: 146,
        flags: 0x57eeca64,
    }
}

pub fn record_0237() -> CatalogRecord {
    CatalogRecord {
        id: 237,
        mnemonic: "RL-0237",
        min_spacing_m: 85,
        max_speed_kph: 157,
        flags: 0x5b0c699f,
    }
}

pub fn record_0238() -> CatalogRecord {
    CatalogRecord {
        id: 238,
        mnemonic: "RL-0238",
        min_spacing_m: 92,
        max_speed_kph: 23,
        flags: 0x5eaa08da,
    }
}

pub fn record_0239() -> CatalogRecord {
    CatalogRecord {
        id: 239,
        mnemonic: "RL-0239",
        min_spacing_m: 99,
        max_speed_kph: 34,
        flags: 0x42c7a815,
    }
}

pub fn record_0240() -> CatalogRecord {
    CatalogRecord {
        id: 240,
        mnemonic: "RL-0240",
        min_spacing_m: 106,
        max_speed_kph: 45,
        flags: 0x46654750,
    }
}

pub fn record_0241() -> CatalogRecord {
    CatalogRecord {
        id: 241,
        mnemonic: "RL-0241",
        min_spacing_m: 113,
        max_speed_kph: 56,
        flags: 0x4d82e68b,
    }
}

pub fn record_0242() -> CatalogRecord {
    CatalogRecord {
        id: 242,
        mnemonic: "RL-0242",
        min_spacing_m: 120,
        max_speed_kph: 67,
        flags: 0x712085c6,
    }
}

pub fn record_0243() -> CatalogRecord {
    CatalogRecord {
        id: 243,
        mnemonic: "RL-0243",
        min_spacing_m: 14,
        max_speed_kph: 78,
        flags: 0x757e2501,
    }
}

pub fn record_0244() -> CatalogRecord {
    CatalogRecord {
        id: 244,
        mnemonic: "RL-0244",
        min_spacing_m: 21,
        max_speed_kph: 89,
        flags: 0x789bc43c,
    }
}

pub fn record_0245() -> CatalogRecord {
    CatalogRecord {
        id: 245,
        mnemonic: "RL-0245",
        min_spacing_m: 28,
        max_speed_kph: 100,
        flags: 0x7c396377,
    }
}

pub fn record_0246() -> CatalogRecord {
    CatalogRecord {
        id: 246,
        mnemonic: "RL-0246",
        min_spacing_m: 35,
        max_speed_kph: 111,
        flags: 0x605702b2,
    }
}

pub fn record_0247() -> CatalogRecord {
    CatalogRecord {
        id: 247,
        mnemonic: "RL-0247",
        min_spacing_m: 42,
        max_speed_kph: 122,
        flags: 0x67f4a1ed,
    }
}

pub fn record_0248() -> CatalogRecord {
    CatalogRecord {
        id: 248,
        mnemonic: "RL-0248",
        min_spacing_m: 49,
        max_speed_kph: 133,
        flags: 0x6b124128,
    }
}

pub fn record_0249() -> CatalogRecord {
    CatalogRecord {
        id: 249,
        mnemonic: "RL-0249",
        min_spacing_m: 56,
        max_speed_kph: 144,
        flags: 0x6eafe063,
    }
}

pub fn record_0250() -> CatalogRecord {
    CatalogRecord {
        id: 250,
        mnemonic: "RL-0250",
        min_spacing_m: 63,
        max_speed_kph: 155,
        flags: 0x12cd7f9e,
    }
}

pub fn record_0251() -> CatalogRecord {
    CatalogRecord {
        id: 251,
        mnemonic: "RL-0251",
        min_spacing_m: 70,
        max_speed_kph: 21,
        flags: 0x166b1ed9,
    }
}

pub fn record_0252() -> CatalogRecord {
    CatalogRecord {
        id: 252,
        mnemonic: "RL-0252",
        min_spacing_m: 77,
        max_speed_kph: 32,
        flags: 0x1d88be14,
    }
}

pub fn record_0253() -> CatalogRecord {
    CatalogRecord {
        id: 253,
        mnemonic: "RL-0253",
        min_spacing_m: 84,
        max_speed_kph: 43,
        flags: 0x01265d4f,
    }
}

pub fn record_0254() -> CatalogRecord {
    CatalogRecord {
        id: 254,
        mnemonic: "RL-0254",
        min_spacing_m: 91,
        max_speed_kph: 54,
        flags: 0x0543fc8a,
    }
}

pub fn record_0255() -> CatalogRecord {
    CatalogRecord {
        id: 255,
        mnemonic: "RL-0255",
        min_spacing_m: 98,
        max_speed_kph: 65,
        flags: 0x08e19bc5,
    }
}

pub fn record_0256() -> CatalogRecord {
    CatalogRecord {
        id: 256,
        mnemonic: "RL-0256",
        min_spacing_m: 105,
        max_speed_kph: 76,
        flags: 0x0c3f3b00,
    }
}

pub fn record_0257() -> CatalogRecord {
    CatalogRecord {
        id: 257,
        mnemonic: "RL-0257",
        min_spacing_m: 112,
        max_speed_kph: 87,
        flags: 0x305cda3b,
    }
}

pub fn record_0258() -> CatalogRecord {
    CatalogRecord {
        id: 258,
        mnemonic: "RL-0258",
        min_spacing_m: 119,
        max_speed_kph: 98,
        flags: 0x37fa7976,
    }
}

pub fn record_0259() -> CatalogRecord {
    CatalogRecord {
        id: 259,
        mnemonic: "RL-0259",
        min_spacing_m: 13,
        max_speed_kph: 109,
        flags: 0x3b1818b1,
    }
}

pub fn record_0260() -> CatalogRecord {
    CatalogRecord {
        id: 260,
        mnemonic: "RL-0260",
        min_spacing_m: 20,
        max_speed_kph: 120,
        flags: 0x3eb5b7ec,
    }
}

pub fn record_0261() -> CatalogRecord {
    CatalogRecord {
        id: 261,
        mnemonic: "RL-0261",
        min_spacing_m: 27,
        max_speed_kph: 131,
        flags: 0x22d35727,
    }
}

pub fn record_0262() -> CatalogRecord {
    CatalogRecord {
        id: 262,
        mnemonic: "RL-0262",
        min_spacing_m: 34,
        max_speed_kph: 142,
        flags: 0x2670f662,
    }
}

pub fn record_0263() -> CatalogRecord {
    CatalogRecord {
        id: 263,
        mnemonic: "RL-0263",
        min_spacing_m: 41,
        max_speed_kph: 153,
        flags: 0x2d8e959d,
    }
}

pub fn record_0264() -> CatalogRecord {
    CatalogRecord {
        id: 264,
        mnemonic: "RL-0264",
        min_spacing_m: 48,
        max_speed_kph: 19,
        flags: 0xd12c34d8,
    }
}

pub fn record_0265() -> CatalogRecord {
    CatalogRecord {
        id: 265,
        mnemonic: "RL-0265",
        min_spacing_m: 55,
        max_speed_kph: 30,
        flags: 0xd549d413,
    }
}

pub fn record_0266() -> CatalogRecord {
    CatalogRecord {
        id: 266,
        mnemonic: "RL-0266",
        min_spacing_m: 62,
        max_speed_kph: 41,
        flags: 0xd8e7734e,
    }
}

pub fn record_0267() -> CatalogRecord {
    CatalogRecord {
        id: 267,
        mnemonic: "RL-0267",
        min_spacing_m: 69,
        max_speed_kph: 52,
        flags: 0xdc051289,
    }
}

pub fn record_0268() -> CatalogRecord {
    CatalogRecord {
        id: 268,
        mnemonic: "RL-0268",
        min_spacing_m: 76,
        max_speed_kph: 63,
        flags: 0xc3a2b1c4,
    }
}

pub fn record_0269() -> CatalogRecord {
    CatalogRecord {
        id: 269,
        mnemonic: "RL-0269",
        min_spacing_m: 83,
        max_speed_kph: 74,
        flags: 0xc7c050ff,
    }
}

pub fn record_0270() -> CatalogRecord {
    CatalogRecord {
        id: 270,
        mnemonic: "RL-0270",
        min_spacing_m: 90,
        max_speed_kph: 85,
        flags: 0xcb1df03a,
    }
}

pub fn record_0271() -> CatalogRecord {
    CatalogRecord {
        id: 271,
        mnemonic: "RL-0271",
        min_spacing_m: 97,
        max_speed_kph: 96,
        flags: 0xcebb8f75,
    }
}

pub fn record_0272() -> CatalogRecord {
    CatalogRecord {
        id: 272,
        mnemonic: "RL-0272",
        min_spacing_m: 104,
        max_speed_kph: 107,
        flags: 0xf2d92eb0,
    }
}

pub fn record_0273() -> CatalogRecord {
    CatalogRecord {
        id: 273,
        mnemonic: "RL-0273",
        min_spacing_m: 111,
        max_speed_kph: 118,
        flags: 0xf676cdeb,
    }
}

pub fn record_0274() -> CatalogRecord {
    CatalogRecord {
        id: 274,
        mnemonic: "RL-0274",
        min_spacing_m: 118,
        max_speed_kph: 129,
        flags: 0xfd946d26,
    }
}

pub fn record_0275() -> CatalogRecord {
    CatalogRecord {
        id: 275,
        mnemonic: "RL-0275",
        min_spacing_m: 12,
        max_speed_kph: 140,
        flags: 0xe1320c61,
    }
}

pub fn record_0276() -> CatalogRecord {
    CatalogRecord {
        id: 276,
        mnemonic: "RL-0276",
        min_spacing_m: 19,
        max_speed_kph: 151,
        flags: 0xe54fab9c,
    }
}

pub fn record_0277() -> CatalogRecord {
    CatalogRecord {
        id: 277,
        mnemonic: "RL-0277",
        min_spacing_m: 26,
        max_speed_kph: 17,
        flags: 0xe8ed4ad7,
    }
}

pub fn record_0278() -> CatalogRecord {
    CatalogRecord {
        id: 278,
        mnemonic: "RL-0278",
        min_spacing_m: 33,
        max_speed_kph: 28,
        flags: 0xec0aea12,
    }
}

pub fn record_0279() -> CatalogRecord {
    CatalogRecord {
        id: 279,
        mnemonic: "RL-0279",
        min_spacing_m: 40,
        max_speed_kph: 39,
        flags: 0x93a8894d,
    }
}

pub fn record_0280() -> CatalogRecord {
    CatalogRecord {
        id: 280,
        mnemonic: "RL-0280",
        min_spacing_m: 47,
        max_speed_kph: 50,
        flags: 0x97c62888,
    }
}

pub fn record_0281() -> CatalogRecord {
    CatalogRecord {
        id: 281,
        mnemonic: "RL-0281",
        min_spacing_m: 54,
        max_speed_kph: 61,
        flags: 0x9b63c7c3,
    }
}

pub fn record_0282() -> CatalogRecord {
    CatalogRecord {
        id: 282,
        mnemonic: "RL-0282",
        min_spacing_m: 61,
        max_speed_kph: 72,
        flags: 0x9e8166fe,
    }
}

pub fn record_0283() -> CatalogRecord {
    CatalogRecord {
        id: 283,
        mnemonic: "RL-0283",
        min_spacing_m: 68,
        max_speed_kph: 83,
        flags: 0x82df0639,
    }
}

pub fn record_0284() -> CatalogRecord {
    CatalogRecord {
        id: 284,
        mnemonic: "RL-0284",
        min_spacing_m: 75,
        max_speed_kph: 94,
        flags: 0x867ca574,
    }
}

pub fn record_0285() -> CatalogRecord {
    CatalogRecord {
        id: 285,
        mnemonic: "RL-0285",
        min_spacing_m: 82,
        max_speed_kph: 105,
        flags: 0x8d9a44af,
    }
}

pub fn record_0286() -> CatalogRecord {
    CatalogRecord {
        id: 286,
        mnemonic: "RL-0286",
        min_spacing_m: 89,
        max_speed_kph: 116,
        flags: 0xb137e3ea,
    }
}

pub fn record_0287() -> CatalogRecord {
    CatalogRecord {
        id: 287,
        mnemonic: "RL-0287",
        min_spacing_m: 96,
        max_speed_kph: 127,
        flags: 0xb5558325,
    }
}

pub fn record_0288() -> CatalogRecord {
    CatalogRecord {
        id: 288,
        mnemonic: "RL-0288",
        min_spacing_m: 103,
        max_speed_kph: 138,
        flags: 0xb8f32260,
    }
}

pub fn record_0289() -> CatalogRecord {
    CatalogRecord {
        id: 289,
        mnemonic: "RL-0289",
        min_spacing_m: 110,
        max_speed_kph: 149,
        flags: 0xbc10c19b,
    }
}

pub fn record_0290() -> CatalogRecord {
    CatalogRecord {
        id: 290,
        mnemonic: "RL-0290",
        min_spacing_m: 117,
        max_speed_kph: 15,
        flags: 0xa3ae60d6,
    }
}

pub fn record_0291() -> CatalogRecord {
    CatalogRecord {
        id: 291,
        mnemonic: "RL-0291",
        min_spacing_m: 11,
        max_speed_kph: 26,
        flags: 0xa7cc0011,
    }
}

pub fn record_0292() -> CatalogRecord {
    CatalogRecord {
        id: 292,
        mnemonic: "RL-0292",
        min_spacing_m: 18,
        max_speed_kph: 37,
        flags: 0xab699f4c,
    }
}

pub fn record_0293() -> CatalogRecord {
    CatalogRecord {
        id: 293,
        mnemonic: "RL-0293",
        min_spacing_m: 25,
        max_speed_kph: 48,
        flags: 0xae873e87,
    }
}

pub fn record_0294() -> CatalogRecord {
    CatalogRecord {
        id: 294,
        mnemonic: "RL-0294",
        min_spacing_m: 32,
        max_speed_kph: 59,
        flags: 0x5224ddc2,
    }
}

pub fn record_0295() -> CatalogRecord {
    CatalogRecord {
        id: 295,
        mnemonic: "RL-0295",
        min_spacing_m: 39,
        max_speed_kph: 70,
        flags: 0x56427cfd,
    }
}

pub fn record_0296() -> CatalogRecord {
    CatalogRecord {
        id: 296,
        mnemonic: "RL-0296",
        min_spacing_m: 46,
        max_speed_kph: 81,
        flags: 0x5de01c38,
    }
}

pub fn record_0297() -> CatalogRecord {
    CatalogRecord {
        id: 297,
        mnemonic: "RL-0297",
        min_spacing_m: 53,
        max_speed_kph: 92,
        flags: 0x413dbb73,
    }
}

pub fn record_0298() -> CatalogRecord {
    CatalogRecord {
        id: 298,
        mnemonic: "RL-0298",
        min_spacing_m: 60,
        max_speed_kph: 103,
        flags: 0x455b5aae,
    }
}

pub fn record_0299() -> CatalogRecord {
    CatalogRecord {
        id: 299,
        mnemonic: "RL-0299",
        min_spacing_m: 67,
        max_speed_kph: 114,
        flags: 0x48f8f9e9,
    }
}

pub fn record_0300() -> CatalogRecord {
    CatalogRecord {
        id: 300,
        mnemonic: "RL-0300",
        min_spacing_m: 74,
        max_speed_kph: 125,
        flags: 0x4c169924,
    }
}

pub fn record_0301() -> CatalogRecord {
    CatalogRecord {
        id: 301,
        mnemonic: "RL-0301",
        min_spacing_m: 81,
        max_speed_kph: 136,
        flags: 0x73b4385f,
    }
}

pub fn record_0302() -> CatalogRecord {
    CatalogRecord {
        id: 302,
        mnemonic: "RL-0302",
        min_spacing_m: 88,
        max_speed_kph: 147,
        flags: 0x77d1d79a,
    }
}

pub fn record_0303() -> CatalogRecord {
    CatalogRecord {
        id: 303,
        mnemonic: "RL-0303",
        min_spacing_m: 95,
        max_speed_kph: 158,
        flags: 0x7b6f76d5,
    }
}

pub fn record_0304() -> CatalogRecord {
    CatalogRecord {
        id: 304,
        mnemonic: "RL-0304",
        min_spacing_m: 102,
        max_speed_kph: 24,
        flags: 0x7e8d1610,
    }
}

pub fn record_0305() -> CatalogRecord {
    CatalogRecord {
        id: 305,
        mnemonic: "RL-0305",
        min_spacing_m: 109,
        max_speed_kph: 35,
        flags: 0x622ab54b,
    }
}

pub fn record_0306() -> CatalogRecord {
    CatalogRecord {
        id: 306,
        mnemonic: "RL-0306",
        min_spacing_m: 116,
        max_speed_kph: 46,
        flags: 0x66485486,
    }
}

pub fn record_0307() -> CatalogRecord {
    CatalogRecord {
        id: 307,
        mnemonic: "RL-0307",
        min_spacing_m: 10,
        max_speed_kph: 57,
        flags: 0x6de5f3c1,
    }
}

pub fn record_0308() -> CatalogRecord {
    CatalogRecord {
        id: 308,
        mnemonic: "RL-0308",
        min_spacing_m: 17,
        max_speed_kph: 68,
        flags: 0x110392fc,
    }
}

pub fn record_0309() -> CatalogRecord {
    CatalogRecord {
        id: 309,
        mnemonic: "RL-0309",
        min_spacing_m: 24,
        max_speed_kph: 79,
        flags: 0x14a13237,
    }
}

pub fn record_0310() -> CatalogRecord {
    CatalogRecord {
        id: 310,
        mnemonic: "RL-0310",
        min_spacing_m: 31,
        max_speed_kph: 90,
        flags: 0x18fed172,
    }
}

pub fn record_0311() -> CatalogRecord {
    CatalogRecord {
        id: 311,
        mnemonic: "RL-0311",
        min_spacing_m: 38,
        max_speed_kph: 101,
        flags: 0x1c1c70ad,
    }
}

pub fn record_0312() -> CatalogRecord {
    CatalogRecord {
        id: 312,
        mnemonic: "RL-0312",
        min_spacing_m: 45,
        max_speed_kph: 112,
        flags: 0x03ba0fe8,
    }
}

pub fn record_0313() -> CatalogRecord {
    CatalogRecord {
        id: 313,
        mnemonic: "RL-0313",
        min_spacing_m: 52,
        max_speed_kph: 123,
        flags: 0x07d7af23,
    }
}

pub fn record_0314() -> CatalogRecord {
    CatalogRecord {
        id: 314,
        mnemonic: "RL-0314",
        min_spacing_m: 59,
        max_speed_kph: 134,
        flags: 0x0b754e5e,
    }
}

pub fn record_0315() -> CatalogRecord {
    CatalogRecord {
        id: 315,
        mnemonic: "RL-0315",
        min_spacing_m: 66,
        max_speed_kph: 145,
        flags: 0x0e92ed99,
    }
}

pub fn record_0316() -> CatalogRecord {
    CatalogRecord {
        id: 316,
        mnemonic: "RL-0316",
        min_spacing_m: 73,
        max_speed_kph: 156,
        flags: 0x32308cd4,
    }
}

pub fn record_0317() -> CatalogRecord {
    CatalogRecord {
        id: 317,
        mnemonic: "RL-0317",
        min_spacing_m: 80,
        max_speed_kph: 22,
        flags: 0x364e2c0f,
    }
}

pub fn record_0318() -> CatalogRecord {
    CatalogRecord {
        id: 318,
        mnemonic: "RL-0318",
        min_spacing_m: 87,
        max_speed_kph: 33,
        flags: 0x3debcb4a,
    }
}

pub fn record_0319() -> CatalogRecord {
    CatalogRecord {
        id: 319,
        mnemonic: "RL-0319",
        min_spacing_m: 94,
        max_speed_kph: 44,
        flags: 0x21096a85,
    }
}

pub fn record_0320() -> CatalogRecord {
    CatalogRecord {
        id: 320,
        mnemonic: "RL-0320",
        min_spacing_m: 101,
        max_speed_kph: 55,
        flags: 0x24a709c0,
    }
}

pub fn record_0321() -> CatalogRecord {
    CatalogRecord {
        id: 321,
        mnemonic: "RL-0321",
        min_spacing_m: 108,
        max_speed_kph: 66,
        flags: 0x28c4a8fb,
    }
}

pub fn record_0322() -> CatalogRecord {
    CatalogRecord {
        id: 322,
        mnemonic: "RL-0322",
        min_spacing_m: 115,
        max_speed_kph: 77,
        flags: 0x2c624836,
    }
}

pub fn record_0323() -> CatalogRecord {
    CatalogRecord {
        id: 323,
        mnemonic: "RL-0323",
        min_spacing_m: 9,
        max_speed_kph: 88,
        flags: 0xd3bfe771,
    }
}

pub fn record_0324() -> CatalogRecord {
    CatalogRecord {
        id: 324,
        mnemonic: "RL-0324",
        min_spacing_m: 16,
        max_speed_kph: 99,
        flags: 0xd7dd86ac,
    }
}

pub fn record_0325() -> CatalogRecord {
    CatalogRecord {
        id: 325,
        mnemonic: "RL-0325",
        min_spacing_m: 23,
        max_speed_kph: 110,
        flags: 0xdb7b25e7,
    }
}

pub fn record_0326() -> CatalogRecord {
    CatalogRecord {
        id: 326,
        mnemonic: "RL-0326",
        min_spacing_m: 30,
        max_speed_kph: 121,
        flags: 0xde98c522,
    }
}

pub fn record_0327() -> CatalogRecord {
    CatalogRecord {
        id: 327,
        mnemonic: "RL-0327",
        min_spacing_m: 37,
        max_speed_kph: 132,
        flags: 0xc236645d,
    }
}

pub fn record_0328() -> CatalogRecord {
    CatalogRecord {
        id: 328,
        mnemonic: "RL-0328",
        min_spacing_m: 44,
        max_speed_kph: 143,
        flags: 0xc6540398,
    }
}

pub fn record_0329() -> CatalogRecord {
    CatalogRecord {
        id: 329,
        mnemonic: "RL-0329",
        min_spacing_m: 51,
        max_speed_kph: 154,
        flags: 0xcdf1a2d3,
    }
}

pub fn record_0330() -> CatalogRecord {
    CatalogRecord {
        id: 330,
        mnemonic: "RL-0330",
        min_spacing_m: 58,
        max_speed_kph: 20,
        flags: 0xf10f420e,
    }
}

pub fn record_0331() -> CatalogRecord {
    CatalogRecord {
        id: 331,
        mnemonic: "RL-0331",
        min_spacing_m: 65,
        max_speed_kph: 31,
        flags: 0xf4ace149,
    }
}

pub fn record_0332() -> CatalogRecord {
    CatalogRecord {
        id: 332,
        mnemonic: "RL-0332",
        min_spacing_m: 72,
        max_speed_kph: 42,
        flags: 0xf8ca8084,
    }
}

pub fn record_0333() -> CatalogRecord {
    CatalogRecord {
        id: 333,
        mnemonic: "RL-0333",
        min_spacing_m: 79,
        max_speed_kph: 53,
        flags: 0xfc681fbf,
    }
}

pub fn record_0334() -> CatalogRecord {
    CatalogRecord {
        id: 334,
        mnemonic: "RL-0334",
        min_spacing_m: 86,
        max_speed_kph: 64,
        flags: 0xe385befa,
    }
}

pub fn record_0335() -> CatalogRecord {
    CatalogRecord {
        id: 335,
        mnemonic: "RL-0335",
        min_spacing_m: 93,
        max_speed_kph: 75,
        flags: 0xe7235e35,
    }
}

pub fn record_0336() -> CatalogRecord {
    CatalogRecord {
        id: 336,
        mnemonic: "RL-0336",
        min_spacing_m: 100,
        max_speed_kph: 86,
        flags: 0xeb40fd70,
    }
}

pub fn record_0337() -> CatalogRecord {
    CatalogRecord {
        id: 337,
        mnemonic: "RL-0337",
        min_spacing_m: 107,
        max_speed_kph: 97,
        flags: 0xee9e9cab,
    }
}

pub fn record_0338() -> CatalogRecord {
    CatalogRecord {
        id: 338,
        mnemonic: "RL-0338",
        min_spacing_m: 114,
        max_speed_kph: 108,
        flags: 0x923c3be6,
    }
}

pub fn record_0339() -> CatalogRecord {
    CatalogRecord {
        id: 339,
        mnemonic: "RL-0339",
        min_spacing_m: 8,
        max_speed_kph: 119,
        flags: 0x9659db21,
    }
}

pub fn record_0340() -> CatalogRecord {
    CatalogRecord {
        id: 340,
        mnemonic: "RL-0340",
        min_spacing_m: 15,
        max_speed_kph: 130,
        flags: 0x9df77a5c,
    }
}

pub fn record_0341() -> CatalogRecord {
    CatalogRecord {
        id: 341,
        mnemonic: "RL-0341",
        min_spacing_m: 22,
        max_speed_kph: 141,
        flags: 0x81151997,
    }
}

pub fn record_0342() -> CatalogRecord {
    CatalogRecord {
        id: 342,
        mnemonic: "RL-0342",
        min_spacing_m: 29,
        max_speed_kph: 152,
        flags: 0x84b2b8d2,
    }
}

pub fn record_0343() -> CatalogRecord {
    CatalogRecord {
        id: 343,
        mnemonic: "RL-0343",
        min_spacing_m: 36,
        max_speed_kph: 18,
        flags: 0x88d0580d,
    }
}

pub fn record_0344() -> CatalogRecord {
    CatalogRecord {
        id: 344,
        mnemonic: "RL-0344",
        min_spacing_m: 43,
        max_speed_kph: 29,
        flags: 0x8c6df748,
    }
}

pub fn record_0345() -> CatalogRecord {
    CatalogRecord {
        id: 345,
        mnemonic: "RL-0345",
        min_spacing_m: 50,
        max_speed_kph: 40,
        flags: 0xb38b9683,
    }
}

pub fn record_0346() -> CatalogRecord {
    CatalogRecord {
        id: 346,
        mnemonic: "RL-0346",
        min_spacing_m: 57,
        max_speed_kph: 51,
        flags: 0xb72935be,
    }
}

pub fn record_0347() -> CatalogRecord {
    CatalogRecord {
        id: 347,
        mnemonic: "RL-0347",
        min_spacing_m: 64,
        max_speed_kph: 62,
        flags: 0xbb46d4f9,
    }
}

pub fn record_0348() -> CatalogRecord {
    CatalogRecord {
        id: 348,
        mnemonic: "RL-0348",
        min_spacing_m: 71,
        max_speed_kph: 73,
        flags: 0xbee47434,
    }
}

pub fn record_0349() -> CatalogRecord {
    CatalogRecord {
        id: 349,
        mnemonic: "RL-0349",
        min_spacing_m: 78,
        max_speed_kph: 84,
        flags: 0xa202136f,
    }
}

pub fn record_0350() -> CatalogRecord {
    CatalogRecord {
        id: 350,
        mnemonic: "RL-0350",
        min_spacing_m: 85,
        max_speed_kph: 95,
        flags: 0xa65fb2aa,
    }
}

pub fn record_0351() -> CatalogRecord {
    CatalogRecord {
        id: 351,
        mnemonic: "RL-0351",
        min_spacing_m: 92,
        max_speed_kph: 106,
        flags: 0xadfd51e5,
    }
}

pub fn record_0352() -> CatalogRecord {
    CatalogRecord {
        id: 352,
        mnemonic: "RL-0352",
        min_spacing_m: 99,
        max_speed_kph: 117,
        flags: 0x511af120,
    }
}

pub fn record_0353() -> CatalogRecord {
    CatalogRecord {
        id: 353,
        mnemonic: "RL-0353",
        min_spacing_m: 106,
        max_speed_kph: 128,
        flags: 0x54b8905b,
    }
}

pub fn record_0354() -> CatalogRecord {
    CatalogRecord {
        id: 354,
        mnemonic: "RL-0354",
        min_spacing_m: 113,
        max_speed_kph: 139,
        flags: 0x58d62f96,
    }
}

pub fn record_0355() -> CatalogRecord {
    CatalogRecord {
        id: 355,
        mnemonic: "RL-0355",
        min_spacing_m: 120,
        max_speed_kph: 150,
        flags: 0x5c73ced1,
    }
}

pub fn record_0356() -> CatalogRecord {
    CatalogRecord {
        id: 356,
        mnemonic: "RL-0356",
        min_spacing_m: 14,
        max_speed_kph: 16,
        flags: 0x43916e0c,
    }
}

pub fn record_0357() -> CatalogRecord {
    CatalogRecord {
        id: 357,
        mnemonic: "RL-0357",
        min_spacing_m: 21,
        max_speed_kph: 27,
        flags: 0x472f0d47,
    }
}

pub fn record_0358() -> CatalogRecord {
    CatalogRecord {
        id: 358,
        mnemonic: "RL-0358",
        min_spacing_m: 28,
        max_speed_kph: 38,
        flags: 0x4b4cac82,
    }
}

pub fn record_0359() -> CatalogRecord {
    CatalogRecord {
        id: 359,
        mnemonic: "RL-0359",
        min_spacing_m: 35,
        max_speed_kph: 49,
        flags: 0x4eea4bbd,
    }
}

pub fn record_0360() -> CatalogRecord {
    CatalogRecord {
        id: 360,
        mnemonic: "RL-0360",
        min_spacing_m: 42,
        max_speed_kph: 60,
        flags: 0x7207eaf8,
    }
}

pub fn record_0361() -> CatalogRecord {
    CatalogRecord {
        id: 361,
        mnemonic: "RL-0361",
        min_spacing_m: 49,
        max_speed_kph: 71,
        flags: 0x79a58a33,
    }
}

pub fn record_0362() -> CatalogRecord {
    CatalogRecord {
        id: 362,
        mnemonic: "RL-0362",
        min_spacing_m: 56,
        max_speed_kph: 82,
        flags: 0x7dc3296e,
    }
}

pub fn record_0363() -> CatalogRecord {
    CatalogRecord {
        id: 363,
        mnemonic: "RL-0363",
        min_spacing_m: 63,
        max_speed_kph: 93,
        flags: 0x6160c8a9,
    }
}

pub fn record_0364() -> CatalogRecord {
    CatalogRecord {
        id: 364,
        mnemonic: "RL-0364",
        min_spacing_m: 70,
        max_speed_kph: 104,
        flags: 0x64be67e4,
    }
}

pub fn record_0365() -> CatalogRecord {
    CatalogRecord {
        id: 365,
        mnemonic: "RL-0365",
        min_spacing_m: 77,
        max_speed_kph: 115,
        flags: 0x68dc071f,
    }
}

pub fn record_0366() -> CatalogRecord {
    CatalogRecord {
        id: 366,
        mnemonic: "RL-0366",
        min_spacing_m: 84,
        max_speed_kph: 126,
        flags: 0x6c79a65a,
    }
}

pub fn record_0367() -> CatalogRecord {
    CatalogRecord {
        id: 367,
        mnemonic: "RL-0367",
        min_spacing_m: 91,
        max_speed_kph: 137,
        flags: 0x13974595,
    }
}

pub fn record_0368() -> CatalogRecord {
    CatalogRecord {
        id: 368,
        mnemonic: "RL-0368",
        min_spacing_m: 98,
        max_speed_kph: 148,
        flags: 0x1734e4d0,
    }
}

pub fn record_0369() -> CatalogRecord {
    CatalogRecord {
        id: 369,
        mnemonic: "RL-0369",
        min_spacing_m: 105,
        max_speed_kph: 159,
        flags: 0x1b52840b,
    }
}

pub fn record_0370() -> CatalogRecord {
    CatalogRecord {
        id: 370,
        mnemonic: "RL-0370",
        min_spacing_m: 112,
        max_speed_kph: 25,
        flags: 0x1ef02346,
    }
}

pub fn record_0371() -> CatalogRecord {
    CatalogRecord {
        id: 371,
        mnemonic: "RL-0371",
        min_spacing_m: 119,
        max_speed_kph: 36,
        flags: 0x020dc281,
    }
}

pub fn record_0372() -> CatalogRecord {
    CatalogRecord {
        id: 372,
        mnemonic: "RL-0372",
        min_spacing_m: 13,
        max_speed_kph: 47,
        flags: 0x09ab61bc,
    }
}

pub fn record_0373() -> CatalogRecord {
    CatalogRecord {
        id: 373,
        mnemonic: "RL-0373",
        min_spacing_m: 20,
        max_speed_kph: 58,
        flags: 0x0dc900f7,
    }
}

pub fn record_0374() -> CatalogRecord {
    CatalogRecord {
        id: 374,
        mnemonic: "RL-0374",
        min_spacing_m: 27,
        max_speed_kph: 69,
        flags: 0x3166a032,
    }
}

pub fn record_0375() -> CatalogRecord {
    CatalogRecord {
        id: 375,
        mnemonic: "RL-0375",
        min_spacing_m: 34,
        max_speed_kph: 80,
        flags: 0x34843f6d,
    }
}

pub fn record_0376() -> CatalogRecord {
    CatalogRecord {
        id: 376,
        mnemonic: "RL-0376",
        min_spacing_m: 41,
        max_speed_kph: 91,
        flags: 0x3821dea8,
    }
}

pub fn record_0377() -> CatalogRecord {
    CatalogRecord {
        id: 377,
        mnemonic: "RL-0377",
        min_spacing_m: 48,
        max_speed_kph: 102,
        flags: 0x3c7f7de3,
    }
}

pub fn record_0378() -> CatalogRecord {
    CatalogRecord {
        id: 378,
        mnemonic: "RL-0378",
        min_spacing_m: 55,
        max_speed_kph: 113,
        flags: 0x239d1d1e,
    }
}

pub fn record_0379() -> CatalogRecord {
    CatalogRecord {
        id: 379,
        mnemonic: "RL-0379",
        min_spacing_m: 62,
        max_speed_kph: 124,
        flags: 0x273abc59,
    }
}

pub fn record_0380() -> CatalogRecord {
    CatalogRecord {
        id: 380,
        mnemonic: "RL-0380",
        min_spacing_m: 69,
        max_speed_kph: 135,
        flags: 0x2b585b94,
    }
}

pub fn record_0381() -> CatalogRecord {
    CatalogRecord {
        id: 381,
        mnemonic: "RL-0381",
        min_spacing_m: 76,
        max_speed_kph: 146,
        flags: 0x2ef5facf,
    }
}

pub fn record_0382() -> CatalogRecord {
    CatalogRecord {
        id: 382,
        mnemonic: "RL-0382",
        min_spacing_m: 83,
        max_speed_kph: 157,
        flags: 0xd2139a0a,
    }
}

pub fn record_0383() -> CatalogRecord {
    CatalogRecord {
        id: 383,
        mnemonic: "RL-0383",
        min_spacing_m: 90,
        max_speed_kph: 23,
        flags: 0xd9b13945,
    }
}

pub fn record_0384() -> CatalogRecord {
    CatalogRecord {
        id: 384,
        mnemonic: "RL-0384",
        min_spacing_m: 97,
        max_speed_kph: 34,
        flags: 0xddced880,
    }
}

pub fn record_0385() -> CatalogRecord {
    CatalogRecord {
        id: 385,
        mnemonic: "RL-0385",
        min_spacing_m: 104,
        max_speed_kph: 45,
        flags: 0xc16c77bb,
    }
}

pub fn record_0386() -> CatalogRecord {
    CatalogRecord {
        id: 386,
        mnemonic: "RL-0386",
        min_spacing_m: 111,
        max_speed_kph: 56,
        flags: 0xc48a16f6,
    }
}

pub fn record_0387() -> CatalogRecord {
    CatalogRecord {
        id: 387,
        mnemonic: "RL-0387",
        min_spacing_m: 118,
        max_speed_kph: 67,
        flags: 0xc827b631,
    }
}

pub fn record_0388() -> CatalogRecord {
    CatalogRecord {
        id: 388,
        mnemonic: "RL-0388",
        min_spacing_m: 12,
        max_speed_kph: 78,
        flags: 0xcc45556c,
    }
}

pub fn record_0389() -> CatalogRecord {
    CatalogRecord {
        id: 389,
        mnemonic: "RL-0389",
        min_spacing_m: 19,
        max_speed_kph: 89,
        flags: 0xf3e2f4a7,
    }
}

pub fn record_0390() -> CatalogRecord {
    CatalogRecord {
        id: 390,
        mnemonic: "RL-0390",
        min_spacing_m: 26,
        max_speed_kph: 100,
        flags: 0xf70093e2,
    }
}

pub fn record_0391() -> CatalogRecord {
    CatalogRecord {
        id: 391,
        mnemonic: "RL-0391",
        min_spacing_m: 33,
        max_speed_kph: 111,
        flags: 0xfb5e331d,
    }
}

pub fn record_0392() -> CatalogRecord {
    CatalogRecord {
        id: 392,
        mnemonic: "RL-0392",
        min_spacing_m: 40,
        max_speed_kph: 122,
        flags: 0xfefbd258,
    }
}

pub fn record_0393() -> CatalogRecord {
    CatalogRecord {
        id: 393,
        mnemonic: "RL-0393",
        min_spacing_m: 47,
        max_speed_kph: 133,
        flags: 0xe2197193,
    }
}

pub fn record_0394() -> CatalogRecord {
    CatalogRecord {
        id: 394,
        mnemonic: "RL-0394",
        min_spacing_m: 54,
        max_speed_kph: 144,
        flags: 0xe9b710ce,
    }
}

pub fn record_0395() -> CatalogRecord {
    CatalogRecord {
        id: 395,
        mnemonic: "RL-0395",
        min_spacing_m: 61,
        max_speed_kph: 155,
        flags: 0xedd4b009,
    }
}

pub fn record_0396() -> CatalogRecord {
    CatalogRecord {
        id: 396,
        mnemonic: "RL-0396",
        min_spacing_m: 68,
        max_speed_kph: 21,
        flags: 0x91724f44,
    }
}

pub fn record_0397() -> CatalogRecord {
    CatalogRecord {
        id: 397,
        mnemonic: "RL-0397",
        min_spacing_m: 75,
        max_speed_kph: 32,
        flags: 0x948fee7f,
    }
}

pub fn record_0398() -> CatalogRecord {
    CatalogRecord {
        id: 398,
        mnemonic: "RL-0398",
        min_spacing_m: 82,
        max_speed_kph: 43,
        flags: 0x982d8dba,
    }
}

pub fn record_0399() -> CatalogRecord {
    CatalogRecord {
        id: 399,
        mnemonic: "RL-0399",
        min_spacing_m: 89,
        max_speed_kph: 54,
        flags: 0x9c4b2cf5,
    }
}

pub fn record_0400() -> CatalogRecord {
    CatalogRecord {
        id: 400,
        mnemonic: "RL-0400",
        min_spacing_m: 96,
        max_speed_kph: 65,
        flags: 0x83e8cc30,
    }
}

pub fn record_0401() -> CatalogRecord {
    CatalogRecord {
        id: 401,
        mnemonic: "RL-0401",
        min_spacing_m: 103,
        max_speed_kph: 76,
        flags: 0x87066b6b,
    }
}

pub fn record_0402() -> CatalogRecord {
    CatalogRecord {
        id: 402,
        mnemonic: "RL-0402",
        min_spacing_m: 110,
        max_speed_kph: 87,
        flags: 0x8aa40aa6,
    }
}

pub fn record_0403() -> CatalogRecord {
    CatalogRecord {
        id: 403,
        mnemonic: "RL-0403",
        min_spacing_m: 117,
        max_speed_kph: 98,
        flags: 0x8ec1a9e1,
    }
}

pub fn record_0404() -> CatalogRecord {
    CatalogRecord {
        id: 404,
        mnemonic: "RL-0404",
        min_spacing_m: 11,
        max_speed_kph: 109,
        flags: 0xb21f491c,
    }
}

pub fn record_0405() -> CatalogRecord {
    CatalogRecord {
        id: 405,
        mnemonic: "RL-0405",
        min_spacing_m: 18,
        max_speed_kph: 120,
        flags: 0xb9bce857,
    }
}

pub fn record_0406() -> CatalogRecord {
    CatalogRecord {
        id: 406,
        mnemonic: "RL-0406",
        min_spacing_m: 25,
        max_speed_kph: 131,
        flags: 0xbdda8792,
    }
}

pub fn record_0407() -> CatalogRecord {
    CatalogRecord {
        id: 407,
        mnemonic: "RL-0407",
        min_spacing_m: 32,
        max_speed_kph: 142,
        flags: 0xa17826cd,
    }
}

pub fn record_0408() -> CatalogRecord {
    CatalogRecord {
        id: 408,
        mnemonic: "RL-0408",
        min_spacing_m: 39,
        max_speed_kph: 153,
        flags: 0xa495c608,
    }
}

pub fn record_0409() -> CatalogRecord {
    CatalogRecord {
        id: 409,
        mnemonic: "RL-0409",
        min_spacing_m: 46,
        max_speed_kph: 19,
        flags: 0xa8336543,
    }
}

pub fn record_0410() -> CatalogRecord {
    CatalogRecord {
        id: 410,
        mnemonic: "RL-0410",
        min_spacing_m: 53,
        max_speed_kph: 30,
        flags: 0xac51047e,
    }
}

pub fn record_0411() -> CatalogRecord {
    CatalogRecord {
        id: 411,
        mnemonic: "RL-0411",
        min_spacing_m: 60,
        max_speed_kph: 41,
        flags: 0x53eea3b9,
    }
}

pub fn record_0412() -> CatalogRecord {
    CatalogRecord {
        id: 412,
        mnemonic: "RL-0412",
        min_spacing_m: 67,
        max_speed_kph: 52,
        flags: 0x570c42f4,
    }
}

pub fn record_0413() -> CatalogRecord {
    CatalogRecord {
        id: 413,
        mnemonic: "RL-0413",
        min_spacing_m: 74,
        max_speed_kph: 63,
        flags: 0x5aa9e22f,
    }
}

pub fn record_0414() -> CatalogRecord {
    CatalogRecord {
        id: 414,
        mnemonic: "RL-0414",
        min_spacing_m: 81,
        max_speed_kph: 74,
        flags: 0x5ec7816a,
    }
}

pub fn record_0415() -> CatalogRecord {
    CatalogRecord {
        id: 415,
        mnemonic: "RL-0415",
        min_spacing_m: 88,
        max_speed_kph: 85,
        flags: 0x426520a5,
    }
}

pub fn record_0416() -> CatalogRecord {
    CatalogRecord {
        id: 416,
        mnemonic: "RL-0416",
        min_spacing_m: 95,
        max_speed_kph: 96,
        flags: 0x4982bfe0,
    }
}

pub fn record_0417() -> CatalogRecord {
    CatalogRecord {
        id: 417,
        mnemonic: "RL-0417",
        min_spacing_m: 102,
        max_speed_kph: 107,
        flags: 0x4d205f1b,
    }
}

pub fn record_0418() -> CatalogRecord {
    CatalogRecord {
        id: 418,
        mnemonic: "RL-0418",
        min_spacing_m: 109,
        max_speed_kph: 118,
        flags: 0x717dfe56,
    }
}

pub fn record_0419() -> CatalogRecord {
    CatalogRecord {
        id: 419,
        mnemonic: "RL-0419",
        min_spacing_m: 116,
        max_speed_kph: 129,
        flags: 0x749b9d91,
    }
}

pub fn record_0420() -> CatalogRecord {
    CatalogRecord {
        id: 420,
        mnemonic: "RL-0420",
        min_spacing_m: 10,
        max_speed_kph: 140,
        flags: 0x78393ccc,
    }
}

pub fn lookup_record(id: u16) -> Option<CatalogRecord> {
    match id {
        1 => Some(record_0001()),
        2 => Some(record_0002()),
        3 => Some(record_0003()),
        4 => Some(record_0004()),
        5 => Some(record_0005()),
        6 => Some(record_0006()),
        7 => Some(record_0007()),
        8 => Some(record_0008()),
        9 => Some(record_0009()),
        10 => Some(record_0010()),
        11 => Some(record_0011()),
        12 => Some(record_0012()),
        13 => Some(record_0013()),
        14 => Some(record_0014()),
        15 => Some(record_0015()),
        16 => Some(record_0016()),
        17 => Some(record_0017()),
        18 => Some(record_0018()),
        19 => Some(record_0019()),
        20 => Some(record_0020()),
        21 => Some(record_0021()),
        22 => Some(record_0022()),
        23 => Some(record_0023()),
        24 => Some(record_0024()),
        25 => Some(record_0025()),
        26 => Some(record_0026()),
        27 => Some(record_0027()),
        28 => Some(record_0028()),
        29 => Some(record_0029()),
        30 => Some(record_0030()),
        31 => Some(record_0031()),
        32 => Some(record_0032()),
        33 => Some(record_0033()),
        34 => Some(record_0034()),
        35 => Some(record_0035()),
        36 => Some(record_0036()),
        37 => Some(record_0037()),
        38 => Some(record_0038()),
        39 => Some(record_0039()),
        40 => Some(record_0040()),
        41 => Some(record_0041()),
        42 => Some(record_0042()),
        43 => Some(record_0043()),
        44 => Some(record_0044()),
        45 => Some(record_0045()),
        46 => Some(record_0046()),
        47 => Some(record_0047()),
        48 => Some(record_0048()),
        49 => Some(record_0049()),
        50 => Some(record_0050()),
        51 => Some(record_0051()),
        52 => Some(record_0052()),
        53 => Some(record_0053()),
        54 => Some(record_0054()),
        55 => Some(record_0055()),
        56 => Some(record_0056()),
        57 => Some(record_0057()),
        58 => Some(record_0058()),
        59 => Some(record_0059()),
        60 => Some(record_0060()),
        61 => Some(record_0061()),
        62 => Some(record_0062()),
        63 => Some(record_0063()),
        64 => Some(record_0064()),
        65 => Some(record_0065()),
        66 => Some(record_0066()),
        67 => Some(record_0067()),
        68 => Some(record_0068()),
        69 => Some(record_0069()),
        70 => Some(record_0070()),
        71 => Some(record_0071()),
        72 => Some(record_0072()),
        73 => Some(record_0073()),
        74 => Some(record_0074()),
        75 => Some(record_0075()),
        76 => Some(record_0076()),
        77 => Some(record_0077()),
        78 => Some(record_0078()),
        79 => Some(record_0079()),
        80 => Some(record_0080()),
        81 => Some(record_0081()),
        82 => Some(record_0082()),
        83 => Some(record_0083()),
        84 => Some(record_0084()),
        85 => Some(record_0085()),
        86 => Some(record_0086()),
        87 => Some(record_0087()),
        88 => Some(record_0088()),
        89 => Some(record_0089()),
        90 => Some(record_0090()),
        91 => Some(record_0091()),
        92 => Some(record_0092()),
        93 => Some(record_0093()),
        94 => Some(record_0094()),
        95 => Some(record_0095()),
        96 => Some(record_0096()),
        97 => Some(record_0097()),
        98 => Some(record_0098()),
        99 => Some(record_0099()),
        100 => Some(record_0100()),
        101 => Some(record_0101()),
        102 => Some(record_0102()),
        103 => Some(record_0103()),
        104 => Some(record_0104()),
        105 => Some(record_0105()),
        106 => Some(record_0106()),
        107 => Some(record_0107()),
        108 => Some(record_0108()),
        109 => Some(record_0109()),
        110 => Some(record_0110()),
        111 => Some(record_0111()),
        112 => Some(record_0112()),
        113 => Some(record_0113()),
        114 => Some(record_0114()),
        115 => Some(record_0115()),
        116 => Some(record_0116()),
        117 => Some(record_0117()),
        118 => Some(record_0118()),
        119 => Some(record_0119()),
        120 => Some(record_0120()),
        121 => Some(record_0121()),
        122 => Some(record_0122()),
        123 => Some(record_0123()),
        124 => Some(record_0124()),
        125 => Some(record_0125()),
        126 => Some(record_0126()),
        127 => Some(record_0127()),
        128 => Some(record_0128()),
        129 => Some(record_0129()),
        130 => Some(record_0130()),
        131 => Some(record_0131()),
        132 => Some(record_0132()),
        133 => Some(record_0133()),
        134 => Some(record_0134()),
        135 => Some(record_0135()),
        136 => Some(record_0136()),
        137 => Some(record_0137()),
        138 => Some(record_0138()),
        139 => Some(record_0139()),
        140 => Some(record_0140()),
        141 => Some(record_0141()),
        142 => Some(record_0142()),
        143 => Some(record_0143()),
        144 => Some(record_0144()),
        145 => Some(record_0145()),
        146 => Some(record_0146()),
        147 => Some(record_0147()),
        148 => Some(record_0148()),
        149 => Some(record_0149()),
        150 => Some(record_0150()),
        151 => Some(record_0151()),
        152 => Some(record_0152()),
        153 => Some(record_0153()),
        154 => Some(record_0154()),
        155 => Some(record_0155()),
        156 => Some(record_0156()),
        157 => Some(record_0157()),
        158 => Some(record_0158()),
        159 => Some(record_0159()),
        160 => Some(record_0160()),
        161 => Some(record_0161()),
        162 => Some(record_0162()),
        163 => Some(record_0163()),
        164 => Some(record_0164()),
        165 => Some(record_0165()),
        166 => Some(record_0166()),
        167 => Some(record_0167()),
        168 => Some(record_0168()),
        169 => Some(record_0169()),
        170 => Some(record_0170()),
        171 => Some(record_0171()),
        172 => Some(record_0172()),
        173 => Some(record_0173()),
        174 => Some(record_0174()),
        175 => Some(record_0175()),
        176 => Some(record_0176()),
        177 => Some(record_0177()),
        178 => Some(record_0178()),
        179 => Some(record_0179()),
        180 => Some(record_0180()),
        181 => Some(record_0181()),
        182 => Some(record_0182()),
        183 => Some(record_0183()),
        184 => Some(record_0184()),
        185 => Some(record_0185()),
        186 => Some(record_0186()),
        187 => Some(record_0187()),
        188 => Some(record_0188()),
        189 => Some(record_0189()),
        190 => Some(record_0190()),
        191 => Some(record_0191()),
        192 => Some(record_0192()),
        193 => Some(record_0193()),
        194 => Some(record_0194()),
        195 => Some(record_0195()),
        196 => Some(record_0196()),
        197 => Some(record_0197()),
        198 => Some(record_0198()),
        199 => Some(record_0199()),
        200 => Some(record_0200()),
        201 => Some(record_0201()),
        202 => Some(record_0202()),
        203 => Some(record_0203()),
        204 => Some(record_0204()),
        205 => Some(record_0205()),
        206 => Some(record_0206()),
        207 => Some(record_0207()),
        208 => Some(record_0208()),
        209 => Some(record_0209()),
        210 => Some(record_0210()),
        211 => Some(record_0211()),
        212 => Some(record_0212()),
        213 => Some(record_0213()),
        214 => Some(record_0214()),
        215 => Some(record_0215()),
        216 => Some(record_0216()),
        217 => Some(record_0217()),
        218 => Some(record_0218()),
        219 => Some(record_0219()),
        220 => Some(record_0220()),
        221 => Some(record_0221()),
        222 => Some(record_0222()),
        223 => Some(record_0223()),
        224 => Some(record_0224()),
        225 => Some(record_0225()),
        226 => Some(record_0226()),
        227 => Some(record_0227()),
        228 => Some(record_0228()),
        229 => Some(record_0229()),
        230 => Some(record_0230()),
        231 => Some(record_0231()),
        232 => Some(record_0232()),
        233 => Some(record_0233()),
        234 => Some(record_0234()),
        235 => Some(record_0235()),
        236 => Some(record_0236()),
        237 => Some(record_0237()),
        238 => Some(record_0238()),
        239 => Some(record_0239()),
        240 => Some(record_0240()),
        241 => Some(record_0241()),
        242 => Some(record_0242()),
        243 => Some(record_0243()),
        244 => Some(record_0244()),
        245 => Some(record_0245()),
        246 => Some(record_0246()),
        247 => Some(record_0247()),
        248 => Some(record_0248()),
        249 => Some(record_0249()),
        250 => Some(record_0250()),
        251 => Some(record_0251()),
        252 => Some(record_0252()),
        253 => Some(record_0253()),
        254 => Some(record_0254()),
        255 => Some(record_0255()),
        256 => Some(record_0256()),
        257 => Some(record_0257()),
        258 => Some(record_0258()),
        259 => Some(record_0259()),
        260 => Some(record_0260()),
        261 => Some(record_0261()),
        262 => Some(record_0262()),
        263 => Some(record_0263()),
        264 => Some(record_0264()),
        265 => Some(record_0265()),
        266 => Some(record_0266()),
        267 => Some(record_0267()),
        268 => Some(record_0268()),
        269 => Some(record_0269()),
        270 => Some(record_0270()),
        271 => Some(record_0271()),
        272 => Some(record_0272()),
        273 => Some(record_0273()),
        274 => Some(record_0274()),
        275 => Some(record_0275()),
        276 => Some(record_0276()),
        277 => Some(record_0277()),
        278 => Some(record_0278()),
        279 => Some(record_0279()),
        280 => Some(record_0280()),
        281 => Some(record_0281()),
        282 => Some(record_0282()),
        283 => Some(record_0283()),
        284 => Some(record_0284()),
        285 => Some(record_0285()),
        286 => Some(record_0286()),
        287 => Some(record_0287()),
        288 => Some(record_0288()),
        289 => Some(record_0289()),
        290 => Some(record_0290()),
        291 => Some(record_0291()),
        292 => Some(record_0292()),
        293 => Some(record_0293()),
        294 => Some(record_0294()),
        295 => Some(record_0295()),
        296 => Some(record_0296()),
        297 => Some(record_0297()),
        298 => Some(record_0298()),
        299 => Some(record_0299()),
        300 => Some(record_0300()),
        301 => Some(record_0301()),
        302 => Some(record_0302()),
        303 => Some(record_0303()),
        304 => Some(record_0304()),
        305 => Some(record_0305()),
        306 => Some(record_0306()),
        307 => Some(record_0307()),
        308 => Some(record_0308()),
        309 => Some(record_0309()),
        310 => Some(record_0310()),
        311 => Some(record_0311()),
        312 => Some(record_0312()),
        313 => Some(record_0313()),
        314 => Some(record_0314()),
        315 => Some(record_0315()),
        316 => Some(record_0316()),
        317 => Some(record_0317()),
        318 => Some(record_0318()),
        319 => Some(record_0319()),
        320 => Some(record_0320()),
        321 => Some(record_0321()),
        322 => Some(record_0322()),
        323 => Some(record_0323()),
        324 => Some(record_0324()),
        325 => Some(record_0325()),
        326 => Some(record_0326()),
        327 => Some(record_0327()),
        328 => Some(record_0328()),
        329 => Some(record_0329()),
        330 => Some(record_0330()),
        331 => Some(record_0331()),
        332 => Some(record_0332()),
        333 => Some(record_0333()),
        334 => Some(record_0334()),
        335 => Some(record_0335()),
        336 => Some(record_0336()),
        337 => Some(record_0337()),
        338 => Some(record_0338()),
        339 => Some(record_0339()),
        340 => Some(record_0340()),
        341 => Some(record_0341()),
        342 => Some(record_0342()),
        343 => Some(record_0343()),
        344 => Some(record_0344()),
        345 => Some(record_0345()),
        346 => Some(record_0346()),
        347 => Some(record_0347()),
        348 => Some(record_0348()),
        349 => Some(record_0349()),
        350 => Some(record_0350()),
        351 => Some(record_0351()),
        352 => Some(record_0352()),
        353 => Some(record_0353()),
        354 => Some(record_0354()),
        355 => Some(record_0355()),
        356 => Some(record_0356()),
        357 => Some(record_0357()),
        358 => Some(record_0358()),
        359 => Some(record_0359()),
        360 => Some(record_0360()),
        361 => Some(record_0361()),
        362 => Some(record_0362()),
        363 => Some(record_0363()),
        364 => Some(record_0364()),
        365 => Some(record_0365()),
        366 => Some(record_0366()),
        367 => Some(record_0367()),
        368 => Some(record_0368()),
        369 => Some(record_0369()),
        370 => Some(record_0370()),
        371 => Some(record_0371()),
        372 => Some(record_0372()),
        373 => Some(record_0373()),
        374 => Some(record_0374()),
        375 => Some(record_0375()),
        376 => Some(record_0376()),
        377 => Some(record_0377()),
        378 => Some(record_0378()),
        379 => Some(record_0379()),
        380 => Some(record_0380()),
        381 => Some(record_0381()),
        382 => Some(record_0382()),
        383 => Some(record_0383()),
        384 => Some(record_0384()),
        385 => Some(record_0385()),
        386 => Some(record_0386()),
        387 => Some(record_0387()),
        388 => Some(record_0388()),
        389 => Some(record_0389()),
        390 => Some(record_0390()),
        391 => Some(record_0391()),
        392 => Some(record_0392()),
        393 => Some(record_0393()),
        394 => Some(record_0394()),
        395 => Some(record_0395()),
        396 => Some(record_0396()),
        397 => Some(record_0397()),
        398 => Some(record_0398()),
        399 => Some(record_0399()),
        400 => Some(record_0400()),
        401 => Some(record_0401()),
        402 => Some(record_0402()),
        403 => Some(record_0403()),
        404 => Some(record_0404()),
        405 => Some(record_0405()),
        406 => Some(record_0406()),
        407 => Some(record_0407()),
        408 => Some(record_0408()),
        409 => Some(record_0409()),
        410 => Some(record_0410()),
        411 => Some(record_0411()),
        412 => Some(record_0412()),
        413 => Some(record_0413()),
        414 => Some(record_0414()),
        415 => Some(record_0415()),
        416 => Some(record_0416()),
        417 => Some(record_0417()),
        418 => Some(record_0418()),
        419 => Some(record_0419()),
        420 => Some(record_0420()),
        _ => None,
    }
}

pub fn score_catalog(salt: u64, yard_id: u32) -> u64 {
    let mut records = Vec::new();
    for id in 1..=420_u16 {
        if let Some(record) = lookup_record(id) {
            if ((record.flags as u64) ^ salt ^ yard_id as u64) & 3 != 0 {
                records.push(record);
            }
        }
    }
    let mut score = salt ^ ((yard_id as u64) << 21);
    for record in &records {
        score ^= checksum::mix_u64(
            record.id as u64 ^ ((record.max_speed_kph as u64) << 19) ^ record.flags as u64,
        );
    }
    score
        ^ fastpath::fast_catalog_sample(&mut records, score, |record| {
            record.id as u64
                ^ ((record.min_spacing_m as u64) << 17)
                ^ ((record.max_speed_kph as u64) << 33)
                ^ record.flags as u64
        })
}
