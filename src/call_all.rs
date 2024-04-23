use crate::parse_upgrade_tx::parse_upgrade_tx;

pub async fn call_all() {
    let upgrade_tx = vec![
        "0xc78a986be023f367f121c06fa9662ef950ad76f2cfe9397693f63de6c5959c61",
        "0xa5fd3584a815267a84a5686b386d911ed7e53d6c1863ff64a57ef0f7085bd4d7",
        "0x937dd21a05142c02159170dafb1bbaaa145ae7bd2c29bf512534fbec9ff801ab",
        "0x2200e7109d3abbb74cb03144fea37f7227188e1fcaba4538bd9dfa3fa17cca02",
        "0x9f0e9ecd78b5c17ff95c130b183df452486a0f784705927e608fd90a00aa9bcd",
        "0x4d3e09380ee604e75800fd61da0c1771987e1cbca7c5254e8c7479e3dd0e3b37",
        "0x0de4556791139b205562b388f2ddc4a2d2ec1bf0996feea38158535cd7e1a5c6",
        "0x5e3ce9e7d3920f293487a5581146f8333191a4068762db6fe4d1eec65a3fb805",
        "0x72983cd25802230545bcb38b805638b0ffa17990ad51e8843e55519fe96d702c",
        "0x8cdc268e23c0fa073ab3f1b75bd32a2cf05ea1e268a07c1aec44d5805f22fb12",
    ];

    let rpc_url = "https://nd-422-757-666.p2pify.com/0a9d79d93fb2f4a4b1e04695da2b77a7/";

    for (i, &tx_hash) in upgrade_tx.iter().enumerate() {
        if let Err(err) = parse_upgrade_tx(tx_hash, rpc_url).await {
            eprintln!("Parse upgrade transaction error: {}", err);
        } else {
            println!("\x1b[38;5;49mOK\x1b[0m {} {}", i, tx_hash);
        }
    }
}
