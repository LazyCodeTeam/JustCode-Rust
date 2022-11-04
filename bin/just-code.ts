#!/usr/bin/env node
import * as cdk from 'aws-cdk-lib';
import { JustCodeStackDev } from '../lib/just-code-stack-dev';

const app = new cdk.App();
new JustCodeStackDev(app, 'JustCodeStackDev');
