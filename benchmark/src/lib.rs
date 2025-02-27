use serde::{Serialize, Deserialize};
use borsh::{BorshSerialize, BorshDeserialize};
use prost::Message;

#[derive(Serialize, Deserialize, BorshSerialize, BorshDeserialize, Message )]
pub struct SampleData {
    #[prost(uint32, tag = "1")]
    pub id: u32,
    
    #[prost(string, tag = "2")]
    pub name: String,
    
    #[prost(bool, tag = "3")]
    pub active: bool,
    
    #[prost(bytes, tag = "4")]
    pub values: Vec<u8>,
}


// pub fn sample_data() -> SampleData {
//     SampleData {
//         id: 1231566666,
//         name: "Hi My Name is Rohit i am from India and i am learning Rust adfdnfsdnmfnsmdfmsdfmnsmdnfsndfs
//         sdfnksndfnskdfjksdfknsdfnksdnfksjdkfjskdfjksdjfjsdfkjsdkfjksdjfkjsdfjsdfjdfmsndf skdf dfsdmf sd
//         ksjdkfksdfnskdnfnsdkfnksdfksdkfnskdnfs dfksdfksndknsdfnskdnfksndfknsdkfnsdfnsd
//         Hi My Name is Rohit i am from India and i am learning Rust adfdnfsdnmfnsmdfmsdfmnsmdnfsndfs
//         sdfnksndfnskdfjksdfknsdfnksdnfksjdkfjskdfjksdjfjsdfkjsdkfjksdjfkjsdfjsdfjdfmsndf skdf dfsdmf sd
//         ksjdkfksdfnskdnfnsdkfnksdfksdkfnskdnfs dfksdfksndknsdfnskdnfksndfknsdkfnsdfnsd".to_string(),
//         active: true,
//         values: vec![1,2,3,4,5,6,7,8,9,10,23,1,2,3,4,5,5,6,7,8,9,63,3,3,3,4,5,5,6,7,7,8,9,9,91,2,2,34,4,5,5,6,7,78] 
//     }
// }

pub fn sample_data() -> SampleData {
    SampleData {
        id: 1,
        name: "H".to_string(),
        active: true,
        values: vec![1]
    }
}
