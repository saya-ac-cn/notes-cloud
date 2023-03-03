use serde::{Deserialize, Serialize};
use rust_decimal::Decimal;

/// 基金参数
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct FundSettingDTO {
    pub fund_code:Option<String>, //基金代码
    pub start_date:Option<i64>,//开始日期(时间戳)
    pub end_date:Option<i64>,//结束日期(时间戳)
    pub flag:Option<bool>,// true -> 按百分比 ；false -> 按照价格
    pub rise:Option<Decimal>,//涨
    pub buy:Option<i32>,// 买入量
    pub fall:Option<Decimal>,//跌
    pub sell:Option<i32>,// 买入量
}