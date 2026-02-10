// enum without Data or Value Associated!
enum IPVersionType{
    IPv4,
    IPv6
}

#[derive(Debug)]
enum IPV{
    IPv4(String),
    IPv6(String)
}

struct IPAddress{
    ip_type:IPVersionType,
    address:String
}


fn main() {
    let ip1 = IPVersionType::IPv6;

    let localhost = IPAddress{
        ip_type:IPVersionType::IPv4,
        address:String::from("127.0.0.1")
    };

    let localhost = IPV::IPv4(String::from("127.0.0.1"));

    println!("Hi : {:#?}",localhost);

    let x = Some(12);
    let y = 122;

    let sum = x.unwrap_or(12) + y;

    let price_of_btc = get_current_price(CryptoCurrency::Bitcoin);

    println!("Current Price of BTC = {price_of_btc}$");

}

enum CryptoCurrency{
    Bitcoin,
    Ethereum,
    Solana,
    XRP
}

fn get_current_price(coin:CryptoCurrency)->u32{

    match coin {
        CryptoCurrency::Bitcoin => 100_000,
        CryptoCurrency::Ethereum => 4_000,
        CryptoCurrency::Solana => 89,
        CryptoCurrency::XRP => 2
    }
}
