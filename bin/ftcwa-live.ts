#!/usr/bin/env node
import 'source-map-support/register';
import * as cdk from 'aws-cdk-lib';
import { FTCWALiveStack } from '../lib/ftcwa-live-stack';
import { CertificateStack } from '../lib/certificate-stack';

const app = new cdk.App();

const certificateStack = new CertificateStack(app, 'FTCWACertificateStack');
new FTCWALiveStack(app, 'FTCWALiveStack', {
  certificateArn: certificateStack.certificateArn,
  crossRegionReferences: true,
  env: {
    region: 'us-west-2',
  },
});
