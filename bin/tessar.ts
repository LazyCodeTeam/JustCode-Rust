#!/usr/bin/env node
import * as cdk from 'aws-cdk-lib';
import { TessarStack } from '../lib/tessar-stack';

const app = new cdk.App();
new TessarStack(app, 'TessarStackDev', {});
