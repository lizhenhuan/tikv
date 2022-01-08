use j4rs::{Instance, Jvm, JvmBuilder, ClasspathEntry, InvocationArg};
use lazy_static::lazy_static;
use std::convert::TryFrom;
use std::fs;

pub struct RawClient {
    pd_address: String,
    sender: Instance,
    jvm: Jvm
}

impl RawClient{
    pub fn new(pd_address: String) -> RawClient {
        let entry = ClasspathEntry::new("/tmp/tidb/tikv_raw_assembly-jar-with-dependencies.jar");
        let jvm = JvmBuilder::new().classpath_entry(entry).build().unwrap();
        let str_arg = InvocationArg::try_from(pd_address.as_str());
        jvm.invoke_static(
            "org.pingcap.tikv_raw_java.RawApiSender",
            "init",
            &[str_arg.unwrap()]
        );
        let sender = jvm.create_instance(
            "org.pingcap.tikv_raw_java.RawApiSender",
            &Vec::new(),
        ).unwrap();
        RawClient{
            pd_address,
            sender,
            jvm,
        }
    }
    pub fn put(&self, key: &str, value: &str) {
        let key = InvocationArg::try_from(key).unwrap();
        let value = InvocationArg::try_from(value).unwrap();
        self.jvm.chain(&self.sender).unwrap().invoke("sendRawApi", &vec![key, value]);
    }
    pub fn delete(&self, key: &str) {
        let key = InvocationArg::try_from(key).unwrap();
        self.jvm.chain(&self.sender).unwrap().invoke("sendRawApiDelete", &vec![key]);
    }
}

fn get_config() -> String {
    let x = fs::read_to_string("/tmp/tidb/pd").unwrap();
    println!("!!!!! get_config is {}", &x);
    x
}


lazy_static! {
    pub static ref RAW_CLIENT: RawClient = RawClient::new(get_config());
}

unsafe impl Send for RawClient {}
unsafe impl Sync for RawClient {}