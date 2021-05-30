#![allow(non_snake_case)]

use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct Event {
    pub Records: Vec<Records>,
}

#[derive(Debug, Deserialize)]
pub struct Records {
    pub EventSource: String,
    pub EventVersion: String,
    pub EventSubscriptionArn: String,
    pub Sns: Sns,
}

#[derive(Debug, Deserialize)]
pub struct Sns {
    pub Type: String,
    pub MessageId: String,
    pub TopicArn: String,
    pub Subject: String,
    pub Message: String,
    pub Timestamp: String,
    pub SignatureVersion: String,
    pub Signature: String,
    pub SigningCertUrl: String,
    pub UnsubscribeUrl: String,
    pub MessageAttributes: MessageAttributes,
}

#[derive(Debug, Deserialize)]
pub struct MessageAttributes {
    pub Test: Test,
    pub TestBinary: TestBinary,
}

#[derive(Debug, Deserialize)]
pub struct Test {
    pub Type: String,
    pub Value: String,
}

#[derive(Debug, Deserialize)]
pub struct TestBinary {
    pub Type: String,
    pub Value: String,
}
