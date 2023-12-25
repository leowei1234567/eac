use common::genetrate_random_eth_key;
use dotenv::dotenv;
use local_calculator::configuration::get_configuration;
use std::collections::HashSet;
use std::env;
use std::fs::File;
use std::io::{BufRead, BufReader, Write};
use std::sync::atomic::{AtomicU64, Ordering};
use std::thread;
use std::time::Duration;

static GLOBAL_U64: AtomicU64 = AtomicU64::new(0);

fn main() -> std::io::Result<()> {
    println!("local-calculator start");

    dotenv().ok();
    let mut report_key: String = String::new();
    let mut enable_push_plus: bool = false;

    match env::var("PUSH_PLUS") {
        Ok(value) => {
            report_key = value;
            enable_push_plus = true;
        }
        Err(err) => {
            println!("push plus key initialized error: {}", err);
        }
    }

    let configuration = get_configuration().expect("Failed to get configuration");
    println!("configuration: {:?}", configuration);

    let address_map = match configuration.build_addresses_from_file {
        true => build_knwon_address_map_from_file(configuration.address_file_path)?,
        false => build_known_address_map_inside()?,
    };

    println!("The address map len is {:?}", address_map.len());

    println!("Real work will start after 1 seconds");
    thread::sleep(Duration::from_secs(1));

    let mut handles = vec![];

    let stat_handle = thread::spawn(move || loop {
        println!("Timer tick every 1 second.");
        thread::sleep(Duration::from_secs(1));
        let count = GLOBAL_U64.load(Ordering::SeqCst);
        println!("Count {} has been calculated", count);
    });
    handles.push(stat_handle);

    for _i in 0..configuration.thread_count {
        let clone_map = address_map.clone();
        let push_plus_key = report_key.to_owned();
        let handle = thread::spawn(move || loop {
            let eth_key = genetrate_random_eth_key();
            let address = eth_key.get_lowercase_address_with_0x_prefix();
            GLOBAL_U64.fetch_add(1, Ordering::SeqCst);

            if clone_map.contains(&address) {
                let key_str = eth_key.get_secret_key_string();
                println!(
                    "i found the key of address {:?}, key {:?}",
                    address, key_str
                );
                write_key_to_file(&address, &key_str);
                if enable_push_plus {
                    report_result(&address, &key_str, &(push_plus_key));
                }

                break;
            }

            if configuration.sleep_each_round {
                thread::sleep(Duration::from_micros(1));
            }
        });

        handles.push(handle);
    }

    for handle in handles {
        let _result = handle.join().unwrap();
        //println!("Thread result: {}", result);
    }

    println!("local-calculator end");

    Ok(())
}

fn write_key_to_file(address: &str, key: &str) {
    let mut file = File::create("output.txt").unwrap();
    let data = format!("address:{}, key:{}", address, key);
    file.write_all(data.as_bytes()).unwrap();
}

fn report_result(address: &str, key: &str, push_key: &str) {
    let content = format!("address:{}, key:{}", address, key);
    let url = format!(
        "http://www.pushplus.plus/send?token={}&content={}",
        push_key, content
    );

    ureq::get(&url).call().unwrap();
}

fn build_knwon_address_map_from_file(path: String) -> std::io::Result<HashSet<String>> {
    let file: File = File::open(path)?;
    let reader = BufReader::new(file);

    let mut known_addresses: Vec<String> = Vec::new();
    for line in reader.lines() {
        let line = line?;
        known_addresses.push(line);
    }

    let hash_set: HashSet<_> = known_addresses.into_iter().collect();

    Ok(hash_set)
}

fn build_known_address_map_inside() -> std::io::Result<HashSet<String>> {
    let known_addresses = vec![
        "0x00000000219ab540356cbb839cbe05303d7705fa",
        "0xc02aaa39b223fe8d0a0e5c4f27ead9083c756cc2",
        "0xbe0eb53f46cd790cd13851d5eff43d12404d33e8",
        "0xda9dfa130df4de4673b89022ee50ff26f6ea73cf",
        "0x40b38765696e3d5d8d9d834d8aad4bb6e418e489",
        "0x8315177ab297ba92a06054ce80a67ed4dbd7ed3a",
        "0xf977814e90da44bfa03b6295a0616a897441acec",
        "0xf977814e90da44bfa03b6295a0616a897441acec",
        "0xe92d1a43df510f82c66382592a047d288f85226f",
        "0x61edcdf5bb737adffe5043706e7c5bb1f1a56eea",
        "0xdf9eb223bafbe5c5271415c75aecd68c21fe3d7f",
        "0xc61b9bb3a7a0767e3179713f3a5c7a9aedce193c",
        "0xca8fa8f0b631ecdb18cda619c4fc9d197c8affca",
        "0xbeb5fc579115071764c7423a4f12edde41f106ed",
        "0xde0b295669a9fd93d5f28d9ec85e40f4cb697bae",
        "0xde0b295669a9fd93d5f28d9ec85e40f4cb697bae",
        "0x8103683202aa8da10536036edef04cdd865c225e",
        "0x0a4c79ce84202b03e95b7a692e5d728d83c44c76",
        "0x539c92186f7c6cc4cbf443f26ef84c595babbca1",
        "0xbfbbfaccd1126a11b8f84c60b09859f80f3bd10f",
        "0x220866b1a2219f40e72f5c628b65d54268ca3a9d",
        "0x1b3cb81e51011b549d78bf720b0d924ac763a7c2",
        "0x9845e1909dca337944a0272f1f9f7249833d2d19",
        "0x4ddc2d193948926d02f9b1fe9e1daa0718270ed5",
        "0x1db92e2eebc8e0c075a02bea49a2935bcd2dfcf4",
        "0xafcd96e580138cfa2332c632e66308eacd45c5da",
        "0x189b9cbd4aff470af2c0102f365fc1823d857965",
        "0x78605df79524164911c144801f41e9811b7db73d",
        "0xf3b0073e3a7f747c7a38b36b805247b222c302a3",
        "0x15c22df3e71e7380012668fb837c537d0f8b38a1",
        "0xa90aa5a93fa074de79306e44596109dc53e01410",
        "0x8484ef722627bf18ca5ae6bcf031c23e6e922b30",
        "0xbf3aeb96e164ae67e763d9e050ff124e7c3fdd28",
        "0x558553d54183a8542f7832742e7b4ba9c33aa1e6",
        "0x5b5b69f4e0add2df5d2176d7dbd20b4897bc7ec4",
        "0x32400084c286cf3e17e7b677ea9583e60a000324",
        "0x36a85757645e8e8aec062a1dee289c7d615901ca",
        "0x9e927c02c9eadae63f5efb0dd818943c7262fb8e",
        "0x210b3cb99fa1de0a64085fa80e18c22fe4722a1b",
        "0x2f2d854c1d6d5bb8936bb85bc07c28ebb42c9b10",
        "0x0c23fc0ef06716d2f8ba19bc4bed56d045581f2d",
        "0xa7e4fecddc20d83f36971b67e13f1abc98dfcfa6",
        "0xfd898a0f677e97a9031654fc79a27cb5e31da34a",
        "0xfe01a216234f79cfc3bea7513e457c6a9e50250d",
        "0xb8cda067fabedd1bb6c11c626862d7255a2414fe",
        "0x701c484bfb40ac628afa487b6082f084b14af0bd",
        "0x9c2fc4fc75fa2d7eb5ba9147fa7430756654faa9",
        "0xb20411c403687d1036e05c8a7310a0f218429503",
        "0x9a1ed80ebc9936cee2d3db944ee6bd8d407e7f9f",
        "0xba18ded5e0d604a86428282964ae0bb249ceb9d0",
        "0x0f00294c6e4c30d9ffc0557fec6c586e6f8c3935",
        "0xb9fa6e54025b4f0829d8e1b42e8b846914659632",
        "0x0c05ec4db907cfb91b2a1a29e7b86688b7568a6d",
        "0x35aeed3aa9657abf8b847038bb591b51e1e4c69f",
        "0xb93d8596ac840816bd366dc0561e8140afd0d1cb",
        "0xb5ab08d153218c1a6a5318b14eeb92df0fb168d6",
        "0xdb3c617cdd2fbf0bb4309c325f47678e37f096d9",
        "0x7ead3a4361bd26a20deb89c9470be368ee9cb6f1",
        "0xd5268a476aadd1a6729df5b3e5e8f2c1004139af",
        "0xd47b4a4c6207b1ee0eb1dd4e5c46a19b50fec00b",
        "0xd65fb7d4cb595833e84c3c094bd4779bab0d4c62",
        "0x595faf77e533a5cd30ab5859c9a0116de8bad8db",
        "0x1bd3fc5ac794e7af8e834a8a4d25b08acd9266ce",
        "0xf481b7fab9f5d0e74f21ae595a749634fb053619",
        "0xc882b111a75c0c657fc507c04fbfcd2cc984f071",
        "0x79f67f689b9925710d4dda2a39d680e9cea61c81",
        "0x7f1502605a2f2cc01f9f4e7dd55e549954a8cd0c",
        "0x368d43c23843ca9b49dc861d80251bda6a090367",
        "0x2d1566722288be5525b548a642c98b546f116aa0",
        "0xb4f4317b7885de16305d1303570879c21f378255",
        "0x6d9d2b30df394f17a5058aceb9a4d3446f1bc042",
        "0xf443864ba5d5361bbc54298551067194f980a635",
        "0x84bf16e7675cee22d0e0302913ccf58b45333ddf",
        "0x0548f59fee79f8832c299e01dca5c76f034f558e",
        "0xa160cdab225685da1d56aa342ad8841c3b53f291",
        "0xa1a45e91164cdab8fa596809a9b24f8d4fdbe0f3",
        "0x376c3e5547c68bc26240d8dcc6729fff665a4448",
        "0x85ce6d7989c8133c31c8fab6a2d2c48a9d9273a1",
        "0x73af3bcf944a6559933396c1577b257e2054d935",
        "0xd9858d573a26bca124282afa21ca4f4a06eff98a",
        "0x999e77c988c4c1451d3b1c104a6cca7813a9946e",
        "0x084ef8564b4f75a70b7ad5e8aabf73edac005397",
        "0xcac9c634b4464efe71a9a5910edba06686baf457",
        "0x04b7f4195595d8132dd8249cc8dc7e79a6ae772b",
        "0xff2ac8c5834a7585fcc97edb8ba2431c4beab487",
        "0x4ed97d6470f5121a8e02498ea37a50987da0eec0",
        "0x6e414cfad874d8ee716ea0299d40011207c907b8",
        "0x19bf56fca395a600c20f732b05757f30ad24a719",
        "0xfcd159d0fef5b1003e10d91a5b79d52bbb8cd05d",
        "0x8ae880b5d35305da48b63ce3e52b22d17859f293",
        "0xa463597d49f54fe6a811fb894cbd67c7f92852b0",
        "0x5a710a3cdf2af218740384c52a10852d8870626a",
        "0xbddf00563c9abd25b576017f08c46982012f12be",
        "0x3262f13a39efaca789ae58390441c9ed76bc658a",
    ];

    let hash_set: HashSet<_> = known_addresses.iter().cloned().map(String::from).collect();
    Ok(hash_set)
}
