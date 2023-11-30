import * as cdk from 'aws-cdk-lib';
import { Construct } from 'constructs';
import * as lambda from 'aws-cdk-lib/aws-lambda';
import { RetentionDays } from 'aws-cdk-lib/aws-logs';

export class FTCWALiveStack extends cdk.Stack {
  constructor(scope: Construct, id: string, props?: cdk.StackProps) {
    super(scope, id, props);

    // create Lambda
    const rocketFn = new lambda.Function(this, 'RocketRouterFunction', {
        code: lambda.Code.fromAsset('./lambda/target/lambda/ftcwa-live/'),
        handler: '.',
        runtime: lambda.Runtime.PROVIDED_AL2,
        architecture: lambda.Architecture.ARM_64,
        logRetention: RetentionDays.THREE_DAYS,
    });
    const rocketFnUrl = rocketFn.addFunctionUrl({
        authType: lambda.FunctionUrlAuthType.NONE,
    });
    new cdk.CfnOutput(this, 'RocketFnUrl', { value: rocketFnUrl.url });
  }
}
