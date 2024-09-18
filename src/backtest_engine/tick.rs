use chrono::{DateTime, FixedOffset, TimeZone};
use lazy_static::lazy_static;
use serde::Deserialize;


lazy_static! {
    // 随便编的日期，没有找到只包含Hour:Minute:Second的日期库
    static ref YEAR: i32 = 2021;
    static ref MONTH: u32 = 10;
    static ref DAY: u32 = 30;
    //9:30 start_morning
    pub static ref START_TIME_MORNINIG: DateTime<FixedOffset> = FixedOffset::east_opt(8 * 60 *60)
        .unwrap()
        .with_ymd_and_hms(*YEAR, *MONTH, *DAY, 9, 30 , 0)
        .unwrap();
    //11:30 end_morning
    pub static ref END_TIME_MORNINIG: DateTime<FixedOffset> = FixedOffset::east_opt(8 * 60 * 60)
        .unwrap()
        .with_ymd_and_hms(*YEAR, *MONTH, *DAY, 11, 30 , 0)
        .unwrap();
    //13:00 strat_afternoon
    pub static ref START_TIME_AFTERNOON: DateTime<FixedOffset> = FixedOffset::east_opt(8 * 60 * 60)
        .unwrap()
        .with_ymd_and_hms(*YEAR, *MONTH, *DAY, 13, 0 , 0)
        .unwrap();
    //15:00 end_afternoon
    pub static ref END_TIME_AFTERNOON: DateTime<FixedOffset> = FixedOffset::east_opt(8 * 60 * 60)
        .unwrap()
        .with_ymd_and_hms(*YEAR, *MONTH, *DAY, 15, 0 , 0)
        .unwrap();
}
pub fn get_time(ntime: u64) -> DateTime<FixedOffset> {
    // 91003000 = 9:10:03
    let pst = FixedOffset::east_opt(8 * 60 * 60).unwrap();
    //println!("hms is :{} {} {}", (ntime/10000000) as u32, (ntime%10000000/100000) as u32, (ntime%100000/1000) as u32);
    let dt = pst
        .with_ymd_and_hms(*YEAR, *MONTH, *DAY, (ntime / 10000000) as u32,  (ntime % 10000000 / 100000) as u32, (ntime % 100000 / 1000) as u32)
        .unwrap();
        // (ntime / 10000000) as u32,
        // (ntime % 10000000 / 100000) as u32,
        // (ntime % 100000 / 1000) as u32,
    
    dt
}

pub fn default_dt() -> DateTime<FixedOffset> {
    FixedOffset::east(8 * 60 * 60)
        .ymd(1970, 1, 1)
        .and_hms(0, 0, 1)
}


#[derive(Debug, Deserialize, Clone)]
pub struct Tick {
    pub ch_wind_code: String,
    pub n_time: u64,
    pub status: u64,
    pub pre_close: u64,
    pub open: u64,
    pub high: u64,
    pub low: u64,
    pub n_price: u64,
    pub n_ask_price1: u64,
    pub n_ask_price2: u64,
    pub n_ask_price3: u64,
    pub n_ask_price4: u64,
    pub n_ask_price5: u64,
    pub n_ask_price6: u64,
    pub n_ask_price7: u64,
    pub n_ask_price8: u64,
    pub n_ask_price9: u64,
    pub n_ask_price10: u64,
    pub n_ask_volume1: u64,
    pub n_ask_volume2: u64,
    pub n_ask_volume3: u64,
    pub n_ask_volume4: u64,
    pub n_ask_volume5: u64,
    pub n_ask_volume6: u64,
    pub n_ask_volume7: u64,
    pub n_ask_volume8: u64,
    pub n_ask_volume9: u64,
    pub n_ask_volume10: u64,
    pub n_bid_price1: u64,
    pub n_bid_price2: u64,
    pub n_bid_price3: u64,
    pub n_bid_price4: u64,
    pub n_bid_price5: u64,
    pub n_bid_price6: u64,
    pub n_bid_price7: u64,
    pub n_bid_price8: u64,
    pub n_bid_price9: u64,
    pub n_bid_price10: u64,
    pub n_bid_volume1: u64,
    pub n_bid_volume2: u64,
    pub n_bid_volume3: u64,
    pub n_bid_volume4: u64,
    pub n_bid_volume5: u64,
    pub n_bid_volume6: u64,
    pub n_bid_volume7: u64,
    pub n_bid_volume8: u64,
    pub n_bid_volume9: u64,
    pub n_bid_volume10: u64,
    pub n_match_items: u64,
    pub total_volume: u64,
    pub total_turnover: u64,
    pub total_bid_volume: u64,
    pub total_ask_volume: u64,
    pub weighted_avg_bid_price: u64,
    pub weighted_avg_ask_price: u64,
    pub iopv: u64,
    pub yield_to_maturity: u64,
    pub high_limited: u64, // tick数据中的涨停价比普通值少了一位，需要特殊处理
    pub low_limited: u64,  //tick数据中的跌停价比普通值少了一位，需要特殊处理
    #[serde(skip_deserializing)]
    #[serde(default = "default_dt")]
    pub dt: DateTime<FixedOffset>,
}
