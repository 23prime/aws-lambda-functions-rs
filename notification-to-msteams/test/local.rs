use notification_to_msteams::{event, logger, run};

#[cfg(test)]
#[ctor::ctor]
fn init() {
    logger::init();
}

#[tokio::main]
#[test]
async fn send_json() {
    let test_message = "{\"AlarmName\":\"Synthetics-Alarm-sundai-oauth-api-1\",\"AlarmDescription\":\"Synthetics alarm metric: Failed  GreaterThanOrEqualToThreshold 1\",\"AWSAccountId\":\"678084882233\",\"NewStateValue\":\"ALARM\",\"NewStateReason\":\"Threshold Crossed: 1 out of the last 1 datapoints [1.0 (10/06/21 13:21:00)] was greater than or equal to the threshold (1.0) (minimum 1 datapoint for OK -> ALARM transition).\",\"StateChangeTime\":\"2021-06-10T13:26:56.025+0000\",\"Region\":\"Asia Pacific (Tokyo)\",\"AlarmArn\":\"arn:aws:cloudwatch:ap-northeast-1:678084882233:alarm:Synthetics-Alarm-sundai-oauth-api-1\",\"OldStateValue\":\"OK\",\"Trigger\":{\"MetricName\":\"Failed\",\"Namespace\":\"CloudWatchSynthetics\",\"StatisticType\":\"Statistic\",\"Statistic\":\"AVERAGE\",\"Unit\":null,\"Dimensions\":[{\"value\":\"sundai-oauth-api\",\"name\":\"CanaryName\"}],\"Period\":300,\"EvaluationPeriods\":1,\"ComparisonOperator\":\"GreaterThanOrEqualToThreshold\",\"Threshold\":1.0,\"TreatMissingData\":\"- TreatMissingData: notBreaching\",\"EvaluateLowSampleCountPercentile\":\"\"}}";

    let test_event = event::Event {
        Records: vec![event::Records {
            EventSource: "".to_string(),
            EventVersion: "".to_string(),
            EventSubscriptionArn: "".to_string(),
            Sns: event::Sns {
                Type: "".to_string(),
                MessageId: "".to_string(),
                TopicArn: "".to_string(),
                Subject: "Alert JSON sending test".to_string(),
                Message: test_message.to_string(),
                Timestamp: "1970-01-01 00:00:00".to_string(),
                SignatureVersion: "".to_string(),
                Signature: "".to_string(),
                SigningCertUrl: "".to_string(),
                UnsubscribeUrl: "".to_string(),
            },
        }],
    };

    let result = run(test_event).await;
    assert!(result.is_ok());
}

#[tokio::main]
#[test]
async fn send_not_json() {
    let test_message = "Hello, World!";

    let test_event = event::Event {
        Records: vec![event::Records {
            EventSource: "".to_string(),
            EventVersion: "".to_string(),
            EventSubscriptionArn: "".to_string(),
            Sns: event::Sns {
                Type: "".to_string(),
                MessageId: "".to_string(),
                TopicArn: "".to_string(),
                Subject: "Not JSON sending test".to_string(),
                Message: test_message.to_string(),
                Timestamp: "1970-01-01 00:00:00".to_string(),
                SignatureVersion: "".to_string(),
                Signature: "".to_string(),
                SigningCertUrl: "".to_string(),
                UnsubscribeUrl: "".to_string(),
            },
        }],
    };

    let result = run(test_event).await;
    assert!(result.is_ok());
}
