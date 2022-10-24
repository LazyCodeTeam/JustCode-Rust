#!/usr/bin/env node
import * as cdk from 'aws-cdk-lib';
import { JustCodeStack } from '../lib/just-code-stack';

const app = new cdk.App();
new JustCodeStack(app, 'JustCodeStackDev', {});
