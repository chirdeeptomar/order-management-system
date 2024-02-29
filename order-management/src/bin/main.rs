use actix::prelude::*;
use oms::{Order, OrderType};
use uuid::Uuid;

struct OrderRecieverActor;

impl Actor for OrderRecieverActor {
    type Context = Context<Self>;
}

#[derive(Message)]
#[rtype(result = "String")]
struct NewOrder(Order);

impl Handler<NewOrder> for OrderRecieverActor {
    type Result = String;

    fn handle(&mut self, new_order: NewOrder, _ctx: &mut Context<Self>) -> Self::Result {
        Uuid::new_v4().to_string()
    }
}

#[actix_rt::main]
async fn main() {
    let _new_order = create_new_order();

    // start new actor
    let addr = OrderRecieverActor.start();

    // send message and get future for result
    let res = addr.send(NewOrder(_new_order)).await;

    // handle() returns tokio handle
    println!("Order Id: {}", res.unwrap());

    // stop system and exit
    System::current().stop();
}

fn create_new_order() -> Order {
    Order {
        symbol: "AAPL".to_string(),
        size: 1000,
        order_type: OrderType::Buy,
        instruction: oms::Instruction::DayOrder,
        transmission: oms::Transmission::Broker,
    }
}
