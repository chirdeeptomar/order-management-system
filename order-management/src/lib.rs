// A trading order typically contains the following information:

// Security identifier (i.e., its ticker symbol)
// Order type (i.e., buy, sell, or short)
// Order size
// Order type (e.g., market, limit, stop, etc.)
// Order instructions (e.g., day order, fill or kill, good-’til-canceled, etc.)
// Order transmission (broker, electronic communication network [ECN], at-the-close [ATC], etc.)

#[derive(Debug)]
pub enum OrderType {
    Buy,
    Sell,
    Short,
}

#[derive(Debug)]
pub enum Instruction {
    DayOrder,
    Fill,
    Kill,
    GoodTillCancelled,
}

#[derive(Debug)]
pub enum Transmission {
    Broker,
    Electronic,
    AtTheClose,
}

#[derive(Debug)]
pub struct Order {
    pub symbol: String,
    pub size: i64,
    pub order_type: OrderType,
    pub instruction: Instruction,
    pub transmission: Transmission,
}
