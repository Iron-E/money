use core::str::FromStr;
use std::collections::HashMap;

use strum::EnumCount;

use super::ExchangeRates;
use crate::{Currency, Decimal, Error, Result};

impl FromStr for ExchangeRates
{
	type Err = Error;

	fn from_str(csv: &str) -> Result<Self>
	{
		// {{{
		let mut rows_by_columns = csv.lines().map(|line| line.split(", "));

		#[rustfmt::skip] let currencies = rows_by_columns.next().ok_or_else(|| Error::csv_row_missing("currency"))?;
		#[rustfmt::skip] let rates = rows_by_columns.next().ok_or_else(|| Error::csv_row_missing("exchange rate"))?;

		drop(rows_by_columns);
		// }}}

		let mut map = HashMap::with_capacity(Currency::COUNT);

		// NOTE: conversion to EUR is not stored in ECB exchange rates, since the rates are given in
		//       context of EUR to some other currency.
		map.insert(Currency::Eur, 1.into());
		currencies
			.zip(rates)
			.skip(1)
			.filter(|(c, _)| !c.is_empty())
			.try_for_each(|(c, r)| -> Result<()> {
				let currency = c.parse::<Currency>()?;
				map.insert(currency, r.parse::<Decimal>()?);
				Ok(())
			})
			.and(Ok(Self(map)))
	}
}

#[cfg(test)]
mod tests
{
	use pretty_assertions::assert_eq;

	use super::ExchangeRates;
	use crate::{Currency, Decimal, SAMPLE_EXCHANGE_RATES_CSV};

	#[tokio::test]
	async fn new()
	{
		assert_eq!(
			SAMPLE_EXCHANGE_RATES_CSV.parse::<ExchangeRates>().unwrap(),
			ExchangeRates(
				[
					(Currency::Aud, Decimal::new(1_5792, 4)),
					(Currency::Bgn, Decimal::new(1_9558, 4)),
					(Currency::Brl, Decimal::new(6_1894, 4)),
					(Currency::Cad, Decimal::new(1_4710, 4)),
					(Currency::Chf, Decimal::new(1_0961, 4)),
					(Currency::Cny, Decimal::new(7_7910, 4)),
					(Currency::Czk, Decimal::new(25_448, 3)),
					(Currency::Dkk, Decimal::new(7_4365, 4)),
					(Currency::Eur, Decimal::new(1, 0)),
					(Currency::Gbp, Decimal::new(85955, 5)),
					(Currency::Hkd, Decimal::new(9_4551, 4)),
					(Currency::Huf, Decimal::new(345_82, 2)),
					(Currency::Idr, Decimal::new(17420_91, 2)),
					(Currency::Ils, Decimal::new(3_9598, 4)),
					(Currency::Inr, Decimal::new(8_88755, 4)),
					(Currency::Isk, Decimal::new(146_30, 2)),
					(Currency::Jpy, Decimal::new(133_81, 2)),
					(Currency::Krw, Decimal::new(1357_75, 2)),
					(Currency::Mxn, Decimal::new(24_3300, 4)),
					(Currency::Myr, Decimal::new(5_0241, 4)),
					(Currency::Nok, Decimal::new(10_1501, 4)),
					(Currency::Nzd, Decimal::new(1_6915, 4)),
					(Currency::Php, Decimal::new(58_208, 3)),
					(Currency::Pln, Decimal::new(4_4520, 4)),
					(Currency::Ron, Decimal::new(4_9220, 4)),
					(Currency::Rub, Decimal::new(89_2163, 4)),
					(Currency::Sek, Decimal::new(10_1145, 4)),
					(Currency::Sgd, Decimal::new(1_6141, 4)),
					(Currency::Thb, Decimal::new(37_938, 3)),
					(Currency::Try, Decimal::new(10_5650, 4)),
					(Currency::Usd, Decimal::new(1_2187, 4)),
					(Currency::Zar, Decimal::new(16_5218, 4)),
				]
				.into_iter()
				.collect(),
			),
		);
	}
}
