import { SSM } from 'aws-sdk';
import { readdirSync, readFileSync } from 'fs';
import path from 'path';
import { parse } from 'toml';

import { StageConfig } from './stage-config-model';

export const STAGE_DIR = 'stage_configs';

/**
 * Retrieve the stage config for CDK deployment or for a test environment.
 *
 * @param stage The name of the stage to retrieve the config for
 * @returns The config for the specified stage name
 */
export const getStageConfig = (stage: string): StageConfig => {
    const stageFile = path.join(
        __dirname,
        '..',
        STAGE_DIR,
        `${stage}.toml`
    );

    let config: StageConfig;
    try {
        config = parse(
            readFileSync(stageFile, 'utf8')
        ) as StageConfig;
    } catch (e) {
        throw new Error(
            `Incorrect file format or file is missing, needs to be a valid TOML file in ./${STAGE_DIR}/`
        );
    }

    return config;
};

/**
 * Determine the stage config to use for CDK deployment, given the
 * underlying AWS account used for deployment (provided by CDK).
 *
 * @param deployAccount The AWS account to deploy to
 * @returns The config for the assumed stage name
 */
export const findStageConfig = (
    deployAccount: string = process.env.CDK_DEFAULT_ACCOUNT!
): StageConfig => {
    const tomlFilesInDir = readdirSync(
        path.join(__dirname, '..', STAGE_DIR)
    ).filter((file) => path.extname(file) === '.toml');

    let deployStage;

    for (const file of tomlFilesInDir) {
        // normally we'd use `substr` and `lastIndexOf`, but here we
        // already know the length of the file extension.
        const stage = file.slice(0, -5);
        const config = getStageConfig(stage);

        if (config.env.account === deployAccount) {
            deployStage = config;
            break;
        } else if (config.env.name === 'dev') {
            deployStage = config;
        }
    }

    return deployStage as StageConfig;
};

/**
 * Get the value for a parameter in SSM Parameter Store. By default, decrypt
 * the value as we assume it is stored as a "SecretString"
 *
 * Ref: https://gist.github.com/cbschuld/938190f81d00934f7a158ff223fb5e02
 *
 * @param ssm The SSM client
 * @param name Name of the parameter
 * @param decrypt True to decrypt a secret parameter (default is true)
 * @returns The parameter value
 */
export const getParamValue = async (
    ssm: SSM,
    name: string,
    decrypt: boolean = true
): Promise<string> => {
    const params: SSM.GetParameterRequest = {
        Name: name,
        WithDecryption: decrypt,
    };

    const res = await ssm.getParameter(params).promise();
    // eslint-disable-next-line @typescript-eslint/no-non-null-asserted-optional-chain
    return res.Parameter?.Value!;
};

/**
 * Create a parameter in SSM Parameter Store.
 *
 * @param ssm The SSM client
 * @param name Name of the parameter
 * @param value Value of the parameter
 * @param type Type of the parameter, defaults to "SecretString" so the value
 * is encrypted.
 * @returns The parameter value
 */
export const createParamValue = (
    ssm: SSM,
    name: string,
    value: string,
    type: string = 'SecureString'
) => {
    const params: SSM.PutParameterRequest = {
        Type: type,
        Name: name,
        Value: value,
    };

    ssm.putParameter(params).send();
};
