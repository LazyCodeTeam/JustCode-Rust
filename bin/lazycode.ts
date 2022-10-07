#!/usr/bin/env node
import * as cdk from 'aws-cdk-lib';
import { LazycodeStack } from '../lib/lazycode-stack';

const app = new cdk.App();
new LazycodeStack(app, 'LazycodeStackDev', {});
