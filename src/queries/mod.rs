// conditions :
// these all cant be async
// but we need concurrency

fn load_currencies(&mut self) -> anyhow::Result<HashMap<Ustr, Currency>> {
    let mut currencies = HashMap::new();
    let pattern = format!("{CURRENCIES}*");

    for key in scan_keys(&mut self.database.con, pattern).await? {
        let parts: Vec<&str> = key.as_str().rsplitn(2, ':').collect();
        let currency_code = Ustr::from(parts.first().unwrap());
        let result = self.load_currency(&currency_code)?;
        match result {
            Some(currency) => {
                currencies.insert(currency_code, currency);
            }
            None => {
                log::error!("Currency not found: {currency_code}");
            }
        }
    }
    Ok(currencies)
}

fn load_instruments(&mut self) -> anyhow::Result<HashMap<InstrumentId, InstrumentAny>> {
    let mut instruments = HashMap::new();
    let pattern = format!("{INSTRUMENTS}*");

    for key in scan_keys(&mut self.database.con, pattern)? {
        let parts: Vec<&str> = key.as_str().rsplitn(2, ':').collect();
        let instrument_id = InstrumentId::from_str(parts.first().unwrap())?;
        let result = self.load_instrument(&instrument_id)?;
        match result {
            Some(instrument) => {
                instruments.insert(instrument_id, instrument);
            }
            None => {
                log::error!("Instrument not found: {instrument_id}");
            }
        }
    }

    Ok(instruments)
}

fn load_synthetics(&mut self) -> anyhow::Result<HashMap<InstrumentId, SyntheticInstrument>> {
    let mut synthetics = HashMap::new();
    let pattern = format!("{SYNTHETICS}*");

    for key in scan_keys(&mut self.database.con, pattern)? {
        let parts: Vec<&str> = key.as_str().rsplitn(2, ':').collect();
        let instrument_id = InstrumentId::from_str(parts.first().unwrap())?;
        let synthetic = self.load_synthetic(&instrument_id)?;
        synthetics.insert(instrument_id, synthetic);
    }

    Ok(synthetics)
}

fn load_accounts(&mut self) -> anyhow::Result<HashMap<AccountId, AccountAny>> {
    let mut accounts = HashMap::new();
    let pattern = format!("{ACCOUNTS}*");

    for key in scan_keys(&mut self.database.con, pattern)? {
        let parts: Vec<&str> = key.as_str().rsplitn(2, ':').collect();
        let account_id = AccountId::from(*parts.first().unwrap());
        let result = self.load_account(&account_id)?;
        match result {
            Some(account) => {
                accounts.insert(account_id, account);
            }
            None => {
                log::error!("Account not found: {account_id}");
            }
        }
    }

    Ok(accounts)
}

fn load_orders(&mut self) -> anyhow::Result<HashMap<ClientOrderId, OrderAny>> {
    let mut orders = HashMap::new();
    let pattern = format!("{ORDERS}*");

    for key in scan_keys(&mut self.database.con, pattern)? {
        let parts: Vec<&str> = key.as_str().rsplitn(2, ':').collect();
        let client_order_id = ClientOrderId::from(*parts.first().unwrap());
        let result = self.load_order(&client_order_id)?;
        match result {
            Some(order) => {
                orders.insert(client_order_id, order);
            }
            None => {
                log::error!("Order not found: {client_order_id}");
            }
        }
    }
    Ok(orders)
}

fn load_positions(&mut self) -> anyhow::Result<HashMap<PositionId, Position>> {
    let mut positions = HashMap::new();
    let pattern = format!("{POSITIONS}*");

    for key in scan_keys(&mut self.database.con, pattern)? {
        let parts: Vec<&str> = key.as_str().rsplitn(2, ':').collect();
        let position_id = PositionId::from(*parts.first().unwrap());
        let position = self.load_position(&position_id)?;
        positions.insert(position_id, position);
    }

    Ok(positions)
}
