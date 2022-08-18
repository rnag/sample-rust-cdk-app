import { Duration, Stack } from 'aws-cdk-lib';
// import * as ec2 from 'aws-cdk-lib/aws-ec2';
import { Effect, PolicyStatement } from 'aws-cdk-lib/aws-iam';
import {
    BlockPublicAccess,
    Bucket,
    BucketAccessControl,
} from 'aws-cdk-lib/aws-s3';
import * as sm from 'aws-cdk-lib/aws-secretsmanager';
import { Construct } from 'constructs';
import { RustFunction, Settings } from 'rust.aws-cdk-lambda';

import { StageConfig } from './stage-config-model';

export class SampleRustCDKAppStack extends Stack {
    constructor(
        scope: Construct,
        id: string,
        readonly props: StageConfig
    ) {
        super(scope, id, props);

        const envName = props.env.name;
        const bucketName = props.s3.bucket_name;
        const uploadPath = props.s3.upload_path;
        const secretNameAdmin = props.secrets.admin_user;
        const secretNameCreds = props.secrets.creds;

        // TODO
        const SES_IDENTITY = 'my-sender@email.org';

        // Enable features and environment variables at compile or build time.
        Settings.FEATURES = [envName];
        Settings.BUILD_ENVIRONMENT = {
            // TODO
            ACCOUNT_NAME: `aws-my-account-${envName}`,
            BUCKET_NAME: bucketName,
            UPLOAD_PATH: uploadPath,
            SES_SENDER: SES_IDENTITY,
            SOURCE_CODE: props.repository_url,
            ADMIN_SECRET: secretNameAdmin,
            CREDS_SECRET: secretNameCreds,
        };

        const secretAdmin = sm.Secret.fromSecretNameV2(
            this,
            `admin-secret-${envName}`,
            secretNameAdmin
        );
        const secretCreds = sm.Secret.fromSecretNameV2(
            this,
            `smtp-secret-${envName}`,
            secretNameCreds
        );

        const bucket = new Bucket(this, bucketName, {
            bucketName: bucketName,
            accessControl: BucketAccessControl.PRIVATE,
            blockPublicAccess: new BlockPublicAccess({
                blockPublicAcls: true,
                blockPublicPolicy: true,
                ignorePublicAcls: true,
                restrictPublicBuckets: true,
            }),
        });

        // const vpc = ec2.Vpc.fromLookup(
        //     this,
        //     `internal-vpc-${envName}`,
        //     {
        //         vpcId: props.vpc.id,
        //     }
        // );
        //
        // const securityGroup = ec2.SecurityGroup.fromSecurityGroupId(
        //     this,
        //     `internal-sg-${envName}`,
        //     props.vpc.security_group_id
        // );

        let myLambda = new RustFunction(this, 'MyRustLambda', {
            functionName: `${envName}-my-sample-rust-lambda`,
            memorySize: 128,
            timeout: Duration.seconds(30),
            environment: {},
            // TODO uncomment if needed
            // vpc: vpc,
            // securityGroups: [securityGroup],
            // Useful so library logs show up in CloudWatch
            setupLogging: true,
            // no retries on asynchronous invocation
            retryAttempts: 0,
        });

        bucket.grantReadWrite(myLambda);
        secretAdmin.grantRead(myLambda);
        secretCreds.grantRead(myLambda);

        // allow the lambda to set tags on the creds secret
        myLambda.addToRolePolicy(
            new PolicyStatement({
                effect: Effect.ALLOW,
                actions: ['secretsmanager:TagResource'],
                resources: [`${secretCreds.secretArn}-??????`],
            })
        );

        // add extra permissions for lambdas
        const sesSenderRole = new PolicyStatement({
            effect: Effect.ALLOW,
            actions: ['ses:Send*Email'],
            resources: [
                `arn:aws:ses:us-east-1:${
                    Stack.of(this).account
                }:identity/${SES_IDENTITY}`,
            ],
        });
        myLambda.addToRolePolicy(sesSenderRole);
    }
}
