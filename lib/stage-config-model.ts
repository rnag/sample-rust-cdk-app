/**
 * Models for stage_configs/
 */

import { Environment, StackProps } from 'aws-cdk-lib';

export interface StageConfig extends StackProps {
    /**
     * The BitBucket repository url in the `package.json` file
     */
    repository_url: string;
    /**
     * CDK environment, including the environment name / type
     */
    readonly env: CBEnvironment;
    /**
     * AWS Secrets Manager - secrets
     */
    readonly secrets: SecretNames;
    /**
     * Stack tags
     */
    tags: {
        [key: string]: string;
    };
    /**
     * VPC Configuration
     */
    readonly vpc: VPCConfig;
}

export interface VPCConfig {
    /**
     * VPC ID
     */
    readonly id: string;
    /**
     * VPC Security Group ID
     */
    readonly security_group_id: string;
}

export interface CBEnvironment extends Environment {
    /**
     * AWS environment name (ex. dev)
     */
    readonly name: string;
    /**
     * AWS environment type (ex. nonprod)
     */
    readonly type: string;
}

export interface SecretNames {
    /**
     * Admin User credentials
     */
    readonly admin_user: string,
    /**
     * Dummy credentials
     */
    readonly creds: string,
}
