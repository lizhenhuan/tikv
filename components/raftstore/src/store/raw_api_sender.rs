use j4rs::{Instance, Jvm, JvmBuilder, ClasspathEntry, InvocationArg};
use lazy_static::lazy_static;
use std::convert::TryFrom;

pub struct RawClient {
    pd_address: &'static str,
    sender: Instance,
    jvm: Jvm
}

impl RawClient{
    pub fn new(pd_address: &'static str) -> RawClient {
        let entry = ClasspathEntry::new("/tmp/tidb/tikv_raw_assembly-jar-with-dependencies.jar");
        let jvm = JvmBuilder::new().classpath_entry(entry).build().unwrap();
        let str_arg = InvocationArg::try_from(pd_address);
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
}


lazy_static! {
    pub static ref RAW_CLIENT: RawClient = RawClient::new("127.0.0.1:2379");
}

unsafe impl Send for RawClient {}
unsafe impl Sync for RawClient {}