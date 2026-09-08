CREATE TABLE price_records (
    id               INTEGER PRIMARY KEY AUTOINCREMENT,
    cycle_id         INTEGER NOT NULL,
    ticker           TEXT NOT NULL,
    date             TEXT NOT NULL,
    cycle_principal  REAL NOT NULL,
    cycle_start_date TEXT NOT NULL,
    buy_price        REAL,
    buy_qty          INTEGER NOT NULL,
    buy_filled       BOOLEAN,
    sell_price       REAL,
    sell_qty         INTEGER NOT NULL,
    sell_filled      BOOLEAN,
    created_at       TEXT NOT NULL DEFAULT (datetime('now')),
    UNIQUE(ticker, date)
);