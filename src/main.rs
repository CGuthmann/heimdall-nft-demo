use std::{str::FromStr, thread::sleep, time::Duration};

use crypto::{poseidon::{PoseidonHasher, PoseidonSignature}, HashFunction};
use heimdall_nft::NFTCredential;
use heimdall_nft_server::heimdall_nft_server_structs::*;
use num_bigint::BigInt;



const ALICE_URL: &str = "http://localhost:9797";
const BOB_URL: &str = "http://localhost:9797";
const CHARLY_URL: &str = "http://localhost:9797";
const SHOP_URL: &str = "http://localhost:9797";
const OEM_URL: &str = "http://localhost:9797";


fn demo_1() {
    let alice_priv = BigInt::from_str("1").unwrap();
    let alice_pub = BigInt::from_str(
        "18586133768512220936620570745912940619677854269274689475585506675881198879027",
    )
    .unwrap();

    let bob_priv = BigInt::from_str("6").unwrap();
    let bob_pub = BigInt::from_str(
        "4204312525841135841975512941763794313765175850880841168060295322266705003157",
    )
    .unwrap(); 

    let charly_priv = BigInt::from_str("7").unwrap();
    let charly_pub = BigInt::from_str(
        "7061949393491957813657776856458368574501817871421526214197139795307327923534",
    )
    .unwrap();


    println!("Farmer Alice harvests 140t of Wheat");
    let grain_params = NewParams{
        attributes: vec![
            "Wheat".to_owned(),
            "t".to_owned(),
            "140".to_owned(),
            "".to_owned(),
            "".to_owned(),
            "".to_owned(),
            "".to_owned(),
            "".to_owned(),
        ],
        id:255 as u64,
        expiration:365,
        credential_type: "green_grain".to_owned(),
        delegatable:false,
        registry: "revocRegistry".to_owned(),
        sk_issuer: alice_priv.clone(),
        owner:alice_pub.clone(),
        trust_list_index: 0
    };
    
    let client = reqwest::blocking::Client::new();
    let mut resp = client.post(ALICE_URL.to_owned() + "/new").json(&grain_params).send().expect("Alice new post failed");
    let mut nft: NFTCredential<PoseidonHasher, PoseidonSignature> = resp.json().expect("Alice new post failed to unwrap");

    println!("Wheat heimdall-nft created: {}\n", serde_json::to_string(&nft).expect("Unexpected conversion error"));

    print!("Waiting for finalization...");
    sleep(Duration::from_secs(15));
    println!("done!");

    println!("\n Alice sells the Wheat to miller Bob, therefore transfers the heimdall-nft ownership");
    let params = TransferParams{
        nft,
        private_key: alice_priv.clone(),
        fraction:[BigInt::from_str("1").unwrap(), BigInt::from_str("1").unwrap()],
        target_a: bob_pub.clone(),
        target_b: bob_pub.clone(),
    };

    let client = reqwest::blocking::Client::new();
    let mut resp = client.post(ALICE_URL.to_owned() + "/transfer_split").json(&params).send().expect("Alice transfer post failed");
    let mut nft_bob:(
        NFTCredential<PoseidonHasher, PoseidonSignature>,
        NFTCredential<PoseidonHasher, PoseidonSignature>,
    ) = resp.json().expect("Alice transfer post failed to unwrap");

    let mut nft_bob = nft_bob.0;

    
    println!("Bob recieved the heimdall-nft: {}\n", serde_json::to_string(&nft_bob).expect("Unexpected conversion error"));

    print!("Waiting for finalization...");
    sleep(Duration::from_secs(15));
    println!("done!");

    println!("Bob now mills the wheat. Therefore, he consumes the wheat nft and creates flour nft.");
    let params = BurnParams{
        nft: nft_bob, private_key: bob_priv.clone()
    };

    let client = reqwest::blocking::Client::new();
    let mut resp = client.post(BOB_URL.to_owned() + "/burn").json(&params).send().expect("Bob burn post failed");
    
    let mut nft_bob: NFTCredential<PoseidonHasher, PoseidonSignature> = resp.json().expect("Bob burn post unwrap failed");
    println!("Bob consumed the heimdall-nft: {}\n", serde_json::to_string(&nft_bob).expect("Unexpected conversion error"));

    print!("Waiting for finalization...");
    sleep(Duration::from_secs(15));
    println!("done!");


    let flour_params = NewParams{
        attributes: vec![
            "Flour".to_owned(),
            "t".to_owned(),
            "100".to_owned(),
            "".to_owned(),
            "".to_owned(),
            "".to_owned(),
            "".to_owned(),
            "".to_owned(),
        ],
        id:255 as u64,
        expiration:365,
        credential_type: "green_flour".to_owned(),
        delegatable:false,
        registry: "revocRegistry".to_owned(),
        sk_issuer: bob_priv.clone(),
        owner:bob_pub.clone(),
        trust_list_index: 1
    };

    
    let client = reqwest::blocking::Client::new();
    let mut resp = client.post(BOB_URL.to_owned() + "/new").json(&flour_params).send().expect("Bob new post failed");
    dbg!(&resp);
    let mut nft: NFTCredential<PoseidonHasher, PoseidonSignature> = resp.json().expect("Bob new post failed to unwrap");

    println!("Flour heimdall-nft created: {}\n", serde_json::to_string(&nft).expect("Unexpected conversion error"));


    print!("Waiting for finalization...");
    sleep(Duration::from_secs(15));
    println!("done!");

    let params = TransferParams{
        nft: nft_bob,
        private_key: bob_priv.clone(),
        fraction:[BigInt::from_str("1").unwrap(), BigInt::from_str("1").unwrap()],
        target_a: nft.nft.source_hash.clone(),                                                  //works since source_hash of a heimdall_nft is randomly generated, not in general
        target_b: bob_pub.clone(),
    };

    let client = reqwest::blocking::Client::new();
    let mut resp = client.post(BOB_URL.to_owned() + "/transfer_split").json(&params).send().expect("Bob transfer post failed");
    let mut nft_bob:(
        NFTCredential<PoseidonHasher, PoseidonSignature>,
        NFTCredential<PoseidonHasher, PoseidonSignature>,
    ) = resp.json().expect("Bob transfer post failed to unwrap");

    let mut nft_bob = nft_bob.0;
    
    println!("Attatched consumed wheat to flour heimdall-nft: {}\n", serde_json::to_string(&nft_bob).expect("Unexpected conversion error"));




    print!("Waiting for finalization...");
    sleep(Duration::from_secs(15));
    println!("done!");

    println!("Bob now sells half of the flour to the distributor Charly.");
    let params = TransferParams{
        nft,
        private_key: bob_priv.clone(),
        fraction:[BigInt::from_str("1").unwrap(), BigInt::from_str("2").unwrap()],
        target_a: bob_pub.clone(),
        target_b: charly_pub.clone(),
    };

    let client = reqwest::blocking::Client::new();
    let mut resp = client.post(BOB_URL.to_owned() + "/transfer_split").json(&params).send().expect("Bob transfer post failed");
    let mut nft_bob_charly:(
        NFTCredential<PoseidonHasher, PoseidonSignature>,
        NFTCredential<PoseidonHasher, PoseidonSignature>,
    ) = resp.json().expect("Bob transfer post failed to unwrap");

    
    println!("Flour heimdall-nfts created: {}\n", serde_json::to_string(&nft_bob_charly).expect("Unexpected conversion error"));

    print!("Waiting for finalization...");
    sleep(Duration::from_secs(15));
    println!("done!");



}


fn demo_paper(){
    let alice_priv = BigInt::from_str("1").unwrap(); //0
    let alice_pub = BigInt::from_str(
        "18586133768512220936620570745912940619677854269274689475585506675881198879027",
    )
    .unwrap();

    
    let oem_priv = BigInt::from_str("8").unwrap(); //3
    let oem_pub = BigInt::from_str(
        "8761383103374198182292249284037598775384145428470309206166618811601037048804",
    )
    .unwrap(); 

    let bob_priv = BigInt::from_str("6").unwrap(); //1
    let bob_pub = BigInt::from_str(
        "4204312525841135841975512941763794313765175850880841168060295322266705003157",
    )
    .unwrap(); 

    let shop_priv = BigInt::from_str("7").unwrap(); //2
    let shop_pub = BigInt::from_str(
        "7061949393491957813657776856458368574501817871421526214197139795307327923534",
    )
    .unwrap();

    println!("OEM mints car DPP on Alice's pk.");
    let car_params = NewParams{
        attributes: vec![
            "car_model".to_owned(),
            "car_specification".to_owned(),
            "car_id".to_owned(),
            "".to_owned(),
            "".to_owned(),
            "".to_owned(),
            "".to_owned(),
            "".to_owned(),
        ],
        id:255 as u64,
        expiration:3650,
        credential_type: "car".to_owned(),
        delegatable:false,
        registry: "revocRegistry".to_owned(),
        sk_issuer: oem_priv.clone(),
        owner:alice_pub.clone(),
        trust_list_index: 3
    };
    

    
    let client = reqwest::blocking::Client::new();
    let mut resp = client.post(OEM_URL.to_owned() + "/new").json(&car_params).send().expect("OEM new post failed");
    let mut car_dpp: NFTCredential<PoseidonHasher, PoseidonSignature> = resp.json().expect("OEM new post failed to unwrap");

    println!("Car DPP created: {}\n", serde_json::to_string(&car_dpp).expect("Unexpected conversion error"));


    print!("Waiting for finalization...");
    sleep(Duration::from_secs(15));
    println!("done!");

    println!("OEM mints battery DPP on car's DPP.");

    let hasher = PoseidonHasher::new();
    let car_transferable_key = hasher.hash_big_int(&vec![car_dpp.nft.source_hash.clone()]);

    let battery_params = NewParams{
        attributes: vec![
            "battery_model".to_owned(),
            "battery_specification".to_owned(),
            "battery_id".to_owned(),
            "".to_owned(),
            "".to_owned(),
            "".to_owned(),
            "".to_owned(),
            "".to_owned(),
        ],
        id:255 as u64,
        expiration:3650,
        credential_type: "car".to_owned(),
        delegatable:false,
        registry: "revocRegistry".to_owned(),
        sk_issuer: oem_priv.clone(),
        owner:car_transferable_key.clone(),
        trust_list_index: 3
    };
    

    
    let client = reqwest::blocking::Client::new();
    let mut resp = client.post(OEM_URL.to_owned() + "/new").json(&battery_params).send().expect("OEM new post failed");
    let mut battery_dpp: NFTCredential<PoseidonHasher, PoseidonSignature> = resp.json().expect("OEM new post failed to unwrap");

    println!("Battery DPP created: {}\n", serde_json::to_string(&battery_dpp).expect("Unexpected conversion error"));


    print!("Waiting for finalization...");
    sleep(Duration::from_secs(15));
    println!("done!");


    
    println!("Shop mints maintanance DPP on car's DPP.");
    let car_maintanance_params = NewParams{
        attributes: vec![
            "maintanance_type".to_owned(),
            "maintanance_status".to_owned(),
            "car_id".to_owned(),
            "".to_owned(),
            "".to_owned(),
            "".to_owned(),
            "".to_owned(),
            "".to_owned(),
        ],
        id:255 as u64,
        expiration:3650,
        credential_type: "car_maintanance".to_owned(),
        delegatable:false,
        registry: "revocRegistry".to_owned(),
        sk_issuer: shop_priv.clone(),
        owner:car_dpp.nft.source_hash.clone(),
        trust_list_index: 2
    };
    

    
    let client = reqwest::blocking::Client::new();
    let mut resp = client.post(OEM_URL.to_owned() + "/new").json(&car_maintanance_params).send().expect("Shop new post failed");
    let mut car_maintanance_dpp: NFTCredential<PoseidonHasher, PoseidonSignature> = resp.json().expect("Shop new post failed to unwrap");

    println!("Car maintanance DPP created: {}\n", serde_json::to_string(&car_maintanance_dpp).expect("Unexpected conversion error"));


    print!("Waiting for finalization...");
    sleep(Duration::from_secs(15));
    println!("done!");


    
    println!("Alice transfers battery DPP to Bob's pk.");
    let battary_sale_params = TransferParams{
        nft:battery_dpp,
        private_key: car_dpp.nft.source_hash.clone(),
        fraction:[BigInt::from_str("1").unwrap(), BigInt::from_str("1").unwrap()],
        target_a: bob_pub.clone(),                                                  
        target_b: bob_pub.clone(),
    };
    

    
    let client = reqwest::blocking::Client::new();
    let mut resp = client.post(ALICE_URL.to_owned() + "/transfer_split").json(&battary_sale_params).send().expect("Alice transfer post failed");
    let mut resp_dpp: (
        NFTCredential<PoseidonHasher, PoseidonSignature>,
        NFTCredential<PoseidonHasher, PoseidonSignature>,
    ) = resp.json().expect("Alice transfer post failed to unwrap");

    let mut battery_dpp_bob = resp_dpp.0;

    println!("Bob's battery DPP created: {}\n", serde_json::to_string(&battery_dpp_bob).expect("Unexpected conversion error"));


    print!("Waiting for finalization...");
    sleep(Duration::from_secs(15));
    println!("done!");



}

fn demo_elma(){
    let alice_priv = BigInt::from_str("1").unwrap(); //0
    let alice_pub = BigInt::from_str(
        "18586133768512220936620570745912940619677854269274689475585506675881198879027",
    )
    .unwrap();

    
    let oem_priv = BigInt::from_str("8").unwrap(); //3
    let oem_pub = BigInt::from_str(
        "8761383103374198182292249284037598775384145428470309206166618811601037048804",
    )
    .unwrap(); 

    let bob_priv = BigInt::from_str("6").unwrap(); //1
    let bob_pub = BigInt::from_str(
        "4204312525841135841975512941763794313765175850880841168060295322266705003157",
    )
    .unwrap(); 

    let shop_priv = BigInt::from_str("7").unwrap(); //2
    let shop_pub = BigInt::from_str(
        "7061949393491957813657776856458368574501817871421526214197139795307327923534",
    )
    .unwrap();

    println!("OEM mints car DPP on Alice's pk.");
    let car_params = NewParams{
        attributes: vec![
            "car_model".to_owned(),
            "car_specification".to_owned(),
            "car_id".to_owned(),
            "".to_owned(),
            "".to_owned(),
            "".to_owned(),
            "".to_owned(),
            "".to_owned(),
        ],
        id:255 as u64,
        expiration:3650,
        credential_type: "car".to_owned(),
        delegatable:false,
        registry: "revocRegistry".to_owned(),
        sk_issuer: oem_priv.clone(),
        owner:alice_pub.clone(),
        trust_list_index: 3
    };
    

    
    let client = reqwest::blocking::Client::new();
    let mut resp = client.post(OEM_URL.to_owned() + "/new").json(&car_params).send().expect("OEM new post failed");
    let mut car_dpp: NFTCredential<PoseidonHasher, PoseidonSignature> = resp.json().expect("OEM new post failed to unwrap");

    println!("Car DPP created: {}\n", serde_json::to_string(&car_dpp).expect("Unexpected conversion error"));


    print!("Waiting for finalization...");
    sleep(Duration::from_secs(15));
    println!("done!");

    println!("OEM mints battery DPP on car's DPP.");

    let battery_params = NewParams{
        attributes: vec![
            "battery_model".to_owned(),
            "battery_specification".to_owned(),
            "battery_id".to_owned(),
            "".to_owned(),
            "".to_owned(),
            "".to_owned(),
            "".to_owned(),
            "".to_owned(),
        ],
        id:255 as u64,
        expiration:3650,
        credential_type: "battery".to_owned(),
        delegatable:false,
        registry: "revocRegistry".to_owned(),
        sk_issuer: oem_priv.clone(),
        owner:oem_pub.clone(),
        trust_list_index: 3
    };
    

    
    let client = reqwest::blocking::Client::new();
    let mut resp = client.post(OEM_URL.to_owned() + "/new").json(&battery_params).send().expect("OEM new post failed");
    let battery_dpp: NFTCredential<PoseidonHasher, PoseidonSignature> = resp.json().expect("OEM new post failed to unwrap");

    println!("Battery DPP created: {}\n", serde_json::to_string(&battery_dpp).expect("Unexpected conversion error"));


    print!("Waiting for finalization...");
    sleep(Duration::from_secs(15));
    println!("done!");

    let battery_attach_params = AttachParams{
        nft: battery_dpp,
        private_key: oem_priv.clone(),
        target: car_dpp,
    };
    

    
    let client = reqwest::blocking::Client::new();
    let mut resp = client.post(OEM_URL.to_owned() + "/attach").json(&battery_attach_params).send().expect("OEM attach post failed");
    dbg!(&resp);
    let mut battery_dpp: NFTCredential<PoseidonHasher, PoseidonSignature> = resp.json().expect("OEM attach post failed to unwrap");

    println!("Battery DPP attached: {}\n", serde_json::to_string(&battery_dpp).expect("Unexpected conversion error"));


    print!("Waiting for finalization...");
    sleep(Duration::from_secs(15));
    println!("done!");

    println!("OEM mints body DPP on car's DPP.");

    let body_params = NewParams{
        attributes: vec![
            "body_model".to_owned(),
            "body_specification".to_owned(),
            "body_id".to_owned(),
            "".to_owned(),
            "".to_owned(),
            "".to_owned(),
            "".to_owned(),
            "".to_owned(),
        ],
        id:255 as u64,
        expiration:3650,
        credential_type: "body".to_owned(),
        delegatable:false,
        registry: "revocRegistry".to_owned(),
        sk_issuer: oem_priv.clone(),
        owner:oem_pub.clone(),
        trust_list_index: 3
    };
    

    
    let client = reqwest::blocking::Client::new();
    let mut resp = client.post(OEM_URL.to_owned() + "/new").json(&body_params).send().expect("OEM new post failed");
    let body_dpp: NFTCredential<PoseidonHasher, PoseidonSignature> = resp.json().expect("OEM new post failed to unwrap");

    println!("Body DPP created: {}\n", serde_json::to_string(&body_dpp).expect("Unexpected conversion error"));


    print!("Waiting for finalization...");
    sleep(Duration::from_secs(15));
    println!("done!");

    let body_attach_params = AttachParams{
        nft: body_dpp,
        private_key: oem_priv.clone(),
        target: battery_attach_params.target,
    };
    

    
    let client = reqwest::blocking::Client::new();
    let mut resp = client.post(OEM_URL.to_owned() + "/attach").json(&body_attach_params).send().expect("OEM attach post failed");
    let mut body_dpp: NFTCredential<PoseidonHasher, PoseidonSignature> = resp.json().expect("OEM attach post failed to unwrap");

    println!("Body DPP attached: {}\n", serde_json::to_string(&body_dpp).expect("Unexpected conversion error"));


    print!("Waiting for finalization...");
    sleep(Duration::from_secs(15));
    println!("done!");


    
    println!("Alice transfers battery DPP to Bob's pk.");
    let battery_detach_params = DetachParams{
        nft: body_dpp,
        private_key: alice_priv.clone(),
        target: bob_pub.clone(),
        path_nft_credential: vec![body_attach_params.target],
    };

    
    let client = reqwest::blocking::Client::new();
    let mut resp = client.post(ALICE_URL.to_owned() + "/detach").json(&battery_detach_params).send().expect("Alice detach post failed");
    let mut resp_dpp: (
        NFTCredential<PoseidonHasher, PoseidonSignature>
    ) = resp.json().expect("Alice detach post failed to unwrap");

    let mut battery_dpp_bob = resp_dpp;

    println!("Bob's battery DPP created: {}\n", serde_json::to_string(&battery_dpp_bob).expect("Unexpected conversion error"));


    print!("Waiting for finalization...");
    sleep(Duration::from_secs(15));
    println!("done!");

}

fn main(){
    //demo_1();
    //demo_paper();
    demo_elma();
}