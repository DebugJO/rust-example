mod clinet {
    #[derive(Debug)]
    pub struct InternetClient {
        pub client_id: u32,
    }
}

use clinet::InternetClient;
use core::fmt;

struct Customer<'a> {
    money: u32,
    name: &'a str,
    client: &'a InternetClient,
}

impl fmt::Debug for Customer<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("Customer").field("money", &self.money).field("name", &self.name).field("client", &self.client).finish()
    }
}

fn main() {
    let client = clinet::InternetClient { client_id: 0 };

    let customer = Customer { money: 1234, name: "가나닭", client: &client };

    println!("{:?}", customer);
}
