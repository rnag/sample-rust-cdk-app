#!/usr/bin/env node
import * as cdk from 'aws-cdk-lib';

import { SampleRustCDKAppStack } from '../lib/sample-rust-cdk-app-stack';
import { StageConfig } from '../lib/stage-config-model';
import { findStageConfig, getStageConfig } from '../lib/utils';
import * as pkg from '../package.json';

const app = new cdk.App();

let stage = app.node.tryGetContext('stage');
let config: StageConfig;

if (stage) {
    config = getStageConfig(stage);
} else {
    config = findStageConfig();
    stage = config.env.name;

    // Credits: https://simplernerd.com/js-console-colors/
    const Log = {
        reset: '\x1b[0m',
        // Background colors
        bg: {
            black: '\x1b[40m',
            red: '\x1b[41m',
            green: '\x1b[42m',
            yellow: '\x1b[43m',
            blue: '\x1b[44m',
            magenta: '\x1b[45m',
            cyan: '\x1b[46m',
            white: '\x1b[47m',
            crimson: '\x1b[48m',
        },
    };

    const log = (color: string, text: string) => {
        console.log(`${color}%s${Log.reset}`, text);
    };

    log(
        Log.bg.cyan,
        `No stage specified, using \`${stage}\`. If needed, a stage name can be passed in: -c stage=[yourStage] (it is case sensitive)`
    );
}

const appName = pkg.name;
const envName = config.env.name;
const envType = config.env.type;
const sourceURL = (config.repository_url = pkg.repository.url);

config.tags = {
    name: appName,
    purpose: 'demo',
    env: envName,
    env_type: envType,
    accessibility: 'private',
    data_classification: 'proprietary-confidential',
    engineer: 'abc@xyz.org',
    provisioned_by: 'cdk',
    source: sourceURL,
};

new SampleRustCDKAppStack(
    app,
    `${appName}-${envName}`,
    // For more information, see https://docs.aws.amazon.com/cdk/latest/guide/environments.html
    config
);

// new SampleRustCDKAppStack(app, STACK_NAME, {
//     /* If you don't specify 'env', this stack will be environment-agnostic.
//      * Account/Region-dependent features and context lookups will not work,
//      * but a single synthesized template can be deployed anywhere. */
//
//     /* Uncomment the next line to specialize this stack for the AWS Account
//      * and Region that are implied by the current CLI configuration. */
//     // env: { account: process.env.CDK_DEFAULT_ACCOUNT, region: process.env.CDK_DEFAULT_REGION },
//
//     /* Uncomment the next line if you know exactly what Account and Region you
//      * want to deploy the stack to. */
//     // env: { account: '123456789012', region: 'us-east-1' },
//
//     /* For more information, see https://docs.aws.amazon.com/cdk/latest/guide/environments.html */
// });
