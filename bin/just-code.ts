#!/usr/bin/env node
import * as cdk from 'aws-cdk-lib';
import { JustCodeStackDev } from '../lib/just-code-stack-dev';

const app = new cdk.App();

// new JustCodeStackDev(app, 'JustCodeStackDev', { env: { region: process.env.AWS_REGION, account: process.env.AWS_ACCOUNT, } });
new JustCodeStackDev(app, 'JustCodeStackDev', { env: { region: 'eu-central-1', account: '269175870005', } });
