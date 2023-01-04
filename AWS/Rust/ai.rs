use std::error::Error;

use rusoto_core::Region;
use rusoto_ec2::{
    AttachClassicLinkVpcRequest, CreateInternetGatewayRequest, CreateVpcRequest, CreateVpcEndpointRequest,
    CreateVpcEndpointConnectionNotificationRequest, DeleteInternetGatewayRequest, DescribeVpcEndpointConnectionsRequest,
    Ec2, Ec2Client, ModifyVpcEndpointRequest,
};
use rusoto_s3::{CreateBucketRequest, S3, S3Client};
use rusoto_secretsmanager::{CreateSecretRequest, DeleteSecretRequest, SecretsManager, SecretsManagerClient};
use rusoto_cloudwatch::{
    CloudWatch, CloudWatchClient, CreateAlarmRequest, DeleteAlarmsInput, DescribeAlarmsInput, PutMetricAlarmRequest,
};

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    // Set up the AWS clients
    let ec2_client = Ec2Client::new(Region::UsEast1);
    let s3_client = S3Client::new(Region::UsEast1);
    let secrets_client = SecretsManagerClient::new(Region::UsEast1);
    let cloudwatch_client = CloudWatchClient::new(Region::UsEast1);

    // Create a VPC
    let vpc_request = CreateVpcRequest {
        cidr_block: "10.0.0.0/16".to_owned(),
        ..Default::default()
    };
    let vpc_result = ec2_client.create_vpc(vpc_request).await?;
    let vpc_id = vpc_result.vpc.unwrap().vpc_id;
    println!("VPC ID: {}", vpc_id);

    // Create an EC2 instance
    // Add other required configuration, such as security groups, key pairs, etc.

    // Create an Elastic Inference Accelerator
    // Add other required configuration, such as volume size, security groups, etc.

    // Create an S3 bucket
    let bucket_request = CreateBucketRequest {
        bucket: "ai-data".to_owned(),
        ..Default::default()
    };
    let _bucket_result = s3_client.create_bucket(bucket_request).await?;

    // Create an Internet Gateway
    let igw_request = CreateInternetGatewayRequest {
        ..Default::default()
    };
    let igw_result = ec2_client.create_internet_gateway(igw_request).await?;
    let igw_id = igw_result.internet_gateway.unwrap().internet_gateway_id;
    println!("Internet Gateway ID: {}", igw_id);

    // Attach the Internet Gateway to the VPC
    let attach_request = AttachClassicLinkVpcRequest {
        vpc_id: vpc_id.clone(),
       
        # Attach the Internet Gateway to the VPC
        let attach_request = AttachClassicLinkVpcRequest {
            vpc_id: vpc_id.clone(),
            internet_gateway_id: igw_id.clone(),
            ..Default::default()
        };
        let _attach_result = ec2_client.attach_classic_link_vpc(attach_request).await?;
        
        // Create a VPC Endpoint for S3
        let endpoint_request = CreateVpcEndpointRequest {
            vpc_id: vpc_id.clone(),
            service_name: "com.amazonaws.us-east-1.s3".to_owned(),
            vpc_endpoint_type: "Gateway".to_owned(),
            ..Default::default()
        };
        let endpoint_result = ec2_client.create_vpc_endpoint(endpoint_request).await?;
        let endpoint_id = endpoint_result.vpc_endpoint.unwrap().vpc_endpoint_id;
        println!("VPC Endpoint ID: {}", endpoint_id);
        
        // Set up connection notification for the VPC Endpoint
        let notification_request = CreateVpcEndpointConnectionNotificationRequest {
            connection_notification: Some(rusoto_ec2::ConnectionNotification {
                connection_events: Some(vec!["Accept".to_owned()]),
                connection_notification_arn: Some("arn:aws:sns:us-east-1:012345678901:my-topic".to_owned()),
                connection_notification_id: Some("my-notification".to_owned()),
                ..Default::default()
            }),
            vpc_endpoint_id: endpoint_id.clone(),
        };
        let _notification_result = ec2_client.create_vpc_endpoint_connection_notification(notification_request).await?;
        
// Modify the VPC Endpoint to allow S3 to modify the route table
let modify_request = ModifyVpcEndpointRequest {
    add_route_table_ids: Some(vec![vpc_id.clone()]),
    vpc_endpoint_id: endpoint_id,
};
let _modify_result = ec2_client.modify_vpc_endpoint(modify_request).await?;

// Check the connection status of the VPC Endpoint
let connection_request = DescribeVpcEndpointConnectionsRequest {
    vpc_endpoint_id: Some(endpoint_id),
};
let connection_result = ec2_client.describe_vpc_endpoint_connections(connection_request).await?;
let connection_status = connection_result.connections.unwrap().first().unwrap().state.unwrap();
println!("VPC Endpoint Connection Status: {}", connection_status);

// Create a Secrets Manager secret
let secret_request = CreateSecretRequest {
    name: "ai-secret".to_owned(),
    ..Default::default()
};
let _secret_result = secrets_client.create_secret(secret_request).await?;

# Create a CloudWatch Alarm
let alarm_request = PutMetricAlarmRequest {
    alarm_name: "ai-alarm".to_owned(),
    comparison_operator: "GreaterThanOrEqualToThreshold".to_owned(),
    evaluation_periods: 1,
    metric_name: "CPUUtilization".to_owned(),
    namespace: "AWS/EC2".to_owned(),
    period: 60,
    threshold: 80.0,
    statistic: "Average".to_owned(),
    ..Default::default()
};
let _alarm_result = cloudwatch_client.put_metric_alarm(alarm_request).await?;

// Delete the CloudWatch Alarm
let delete_request = DeleteAlarmsInput {
    alarm_names: vec!["ai-alarm".to_owned()],
};
let _delete_result = cloudwatch_client.delete_alarms(delete_request).await?;

// Delete the Secrets Manager secret
let secret_delete_request = DeleteSecretRequest {
    secret_id: "ai-secret".to_owned(),
    ..Default::default()
};
let _secret_delete_result = secrets_client.delete_secret(secret_delete_request).await?;

// Detach and delete the Internet Gateway
let _detach_result = ec2_client.detach_internet_gateway(rusoto_ec2::DetachInternetGatewayRequest {
    internet_gateway_id: igw_id,
    vpc_id: vpc_id,
}).await?;
let _igw_delete_result = ec2_client.delete_internet_gateway(DeleteInternetGatewayRequest {
    internet_gateway_id: igw_id,
}).await?;

Ok(())

# Create a CloudWatch Alarm
let alarm_request = PutMetricAlarmRequest {
    alarm_name: "ai-alarm".to_owned(),
    comparison_operator: "GreaterThanOrEqualToThreshold".to_owned(),
    evaluation_periods: 1,
    metric_name: "CPUUtilization".to_owned(),
    namespace: "AWS/EC2".to_owned(),
    period: 60,
    threshold: 80.0,
    statistic: "Average".to_owned(),
    ..Default::default()
};
let _alarm_result = cloudwatch_client.put_metric_alarm(alarm_request).await?;

// Delete the CloudWatch Alarm
let delete_request = DeleteAlarmsInput {
    alarm_names: vec!["ai-alarm".to_owned()],
};
let _delete_result = cloudwatch_client.delete_alarms(delete_request).await?;

// Delete the Secrets Manager secret
let secret_delete_request = DeleteSecretRequest {
    secret_id: "ai-secret".to_owned(),
    ..Default::default()
};
let _secret_delete_result = secrets_client.delete_secret(secret_delete_request).await?;

// Detach and delete the Internet Gateway
let _detach_result = ec2_client.detach_internet_gateway(rusoto_ec2::DetachInternetGatewayRequest {
    internet_gateway_id: igw_id,
    vpc_id: vpc_id,
}).await?;
let _igw_delete_result = ec2_client.delete_internet_gateway(DeleteInternetGatewayRequest {
    internet_gateway_id: igw_id,
}).await?;

Ok(())

# Create a CloudWatch Alarm
let alarm_request = PutMetricAlarmRequest {
    alarm_name: "ai-alarm".to_owned(),
    comparison_operator: "GreaterThanOrEqualToThreshold".to_owned(),
    evaluation_periods: 1,
    metric_name: "CPUUtilization".to_owned(),
    namespace: "AWS/EC2".to_owned(),
    period: 60,
    threshold: 80.0,
    statistic: "Average".to_owned(),
    ..Default::default()
};
let _alarm_result = cloudwatch_client.put_metric_alarm(alarm_request).await?;

// Delete the CloudWatch Alarm
let delete_request = DeleteAlarmsInput {
    alarm_names: vec!["ai-alarm".to_owned()],
};
let _delete_result = cloudwatch_client.delete_alarms(delete_request).await?;

// Delete the Secrets Manager secret
let secret_delete_request = DeleteSecretRequest {
    secret_id: "ai-secret".to_owned(),
    ..Default::default()
};
let _secret_delete_result = secrets_client.delete_secret(secret_delete_request).await?;

// Detach and delete the Internet Gateway
let _detach_result = ec2_client.detach_internet_gateway(rusoto_ec2::DetachInternetGatewayRequest {
    internet_gateway_id: igw_id,
    vpc_id: vpc_id,
}).await?;
let _igw_delete_result = ec2_client.delete_internet_gateway(DeleteInternetGatewayRequest {
    internet_gateway_id: igw_id,
}).await?;

Ok(())


# Create a CloudWatch Alarm
let alarm_request = PutMetricAlarmRequest {
    alarm_name: "ai-alarm".to_owned(),
    comparison_operator: "GreaterThanOrEqualToThreshold".to_owned(),
    evaluation_periods: 1,
    metric_name: "CPUUtilization".to_owned(),
    namespace: "AWS/EC2".to_owned(),
    period: 60,
    threshold: 80.0,
    statistic: "Average".to_owned(),
    ..Default::default()
};
let _alarm_result = cloudwatch_client.put_metric_alarm(alarm_request).await?;

// Delete the CloudWatch Alarm
let delete_request = DeleteAlarmsInput {
    alarm_names: vec!["ai-alarm".to_owned()],
};
let _delete_result = cloudwatch_client.delete_alarms(delete_request).await?;

// Delete the Secrets Manager secret
let secret_delete_request = DeleteSecretRequest {
    secret_id: "ai-secret".to_owned(),
    ..Default::default()
};
let _secret_delete_result = secrets_client.delete_secret(secret_delete_request).await?;

// Detach and delete the Internet Gateway
let _detach_result = ec2_client.detach_internet_gateway(rusoto_ec2::DetachInternetGatewayRequest {
    internet_gateway_id: igw_id,
    vpc_id: vpc_id,
}).await?;
let _igw_delete_result = ec2_client.delete_internet_gateway(DeleteInternetGatewayRequest {
    internet_gateway_id: igw_id,
}).await?;

Ok(())

# Create a CloudWatch Alarm
let alarm_request = PutMetricAlarmRequest {
    alarm_name: "ai-alarm".to_owned(),
    comparison_operator: "GreaterThanOrEqualToThreshold".to_owned(),
    evaluation_periods: 1,
    metric_name: "CPUUtilization".to_owned(),
    namespace: "AWS/EC2".to_owned(),
    period: 60,
    threshold: 80.0,
    statistic: "Average".to_owned(),
    ..Default::default()
};
let _alarm_result = cloudwatch_client.put_metric_alarm(alarm_request).await?;

// Delete the CloudWatch Alarm
let delete_request = DeleteAlarmsInput {
    alarm_names: vec!["ai-alarm".to_owned()],
};
let _delete_result = cloudwatch_client.delete_alarms(delete_request).await?;

// Delete the Secrets Manager secret
let secret_delete_request = DeleteSecretRequest {
    secret_id: "ai-secret".to_owned(),
    ..Default::default()
};
let _secret_delete_result = secrets_client.delete_secret(secret_delete_request).await?;

// Detach and delete the Internet Gateway
let _detach_result = ec2_client.detach_internet_gateway(rusoto_ec2::DetachInternetGatewayRequest {
    internet_gateway_id: igw_id,
    vpc_id: vpc_id,
}).await?;
let _igw_delete_result = ec2_client.delete_internet_gateway(DeleteInternetGatewayRequest {
    internet_gateway_id: igw_id,
}).await?;

Ok(())

# Create a CloudWatch Alarm
let alarm_request = PutMetricAlarmRequest {
    alarm_name: "ai-alarm".to_owned(),
    comparison_operator: "GreaterThanOrEqualToThreshold".to_owned(),
    evaluation_periods: 1,
    metric_name: "CPUUtilization".to_owned(),
    namespace: "AWS/EC2".to_owned(),
    period: 60,
    threshold: 80.0,
    statistic: "Average".to_owned(),
    ..Default::default()
};
let _alarm_result = cloudwatch_client.put_metric_alarm(alarm_request).await?;

// Delete the CloudWatch Alarm
let delete_request = DeleteAlarmsInput {
    alarm_names: vec!["ai-alarm".to_owned()],
};
let _delete_result = cloudwatch_client.delete_alarms(delete_request).await?;

// Delete the Secrets Manager secret
let secret_delete_request = DeleteSecretRequest {
    secret_id: "ai-secret".to_owned(),
    ..Default::default()
};
let _secret_delete_result = secrets_client.delete_secret(secret_delete_request).await?;

// Detach and delete the Internet Gateway
let _detach_result = ec2_client.detach_internet_gateway(rusoto_ec2::DetachInternetGatewayRequest {
    internet_gateway_id: igw_id,
    vpc_id: vpc_id,
}).await?;
let _igw_delete_result = ec2_client.delete_internet_gateway(DeleteInternetGatewayRequest {
    internet_gateway_id: igw_id,
}).await?;

Ok(())

# Create a CloudWatch Alarm
let alarm_request = PutMetricAlarmRequest {
    alarm_name: "ai-alarm".to_owned(),
    comparison_operator: "GreaterThanOrEqualToThreshold".to_owned(),
    evaluation_periods: 1,
    metric_name: "CPUUtilization".to_owned(),
    namespace: "AWS/EC2".to_owned(),
    period: 60,
    threshold: 80.0,
    statistic: "Average".to_owned(),
    ..Default::default()
};
let _alarm_result = cloudwatch_client.put_metric_alarm(alarm_request).await?;

// Delete the CloudWatch Alarm
let delete_request = DeleteAlarmsInput {
    alarm_names: vec!["ai-alarm".to_owned()],
};
let _delete_result = cloudwatch_client.delete_alarms(delete_request).await?;

// Delete the Secrets Manager secret
let secret_delete_request = DeleteSecretRequest {
    secret_id: "ai-secret".to_owned(),
    ..Default::default()
};
let _secret_delete_result = secrets_client.delete_secret(secret_delete_request).await?;

// Detach and delete the Internet Gateway
let _detach_result = ec2_client.detach_internet_gateway(rusoto_ec2::DetachInternetGatewayRequest {
    internet_gateway_id: igw_id,
    vpc_id: vpc_id,
}).await?;
let _igw_delete_result = ec2_client.delete_internet_gateway(DeleteInternetGatewayRequest {
    internet_gateway_id: igw_id,
}).await?;

Ok(())
