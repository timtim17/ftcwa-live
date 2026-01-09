import { Certificate } from 'aws-cdk-lib/aws-certificatemanager';
import * as cdk from 'aws-cdk-lib';
import * as cloudfront from 'aws-cdk-lib/aws-cloudfront';
import { Construct } from 'constructs';
import { FunctionUrlOrigin } from 'aws-cdk-lib/aws-cloudfront-origins';
import * as lambda from 'aws-cdk-lib/aws-lambda';
import { RetentionDays } from 'aws-cdk-lib/aws-logs';
import { RustFunction } from 'cargo-lambda-cdk';

interface FTCWALiveStackProps extends cdk.StackProps {
    certificateArn: string;
}

export class FTCWALiveStack extends cdk.Stack {
  constructor(scope: Construct, id: string, props: FTCWALiveStackProps) {
    super(scope, id, props);

    // create Lambda
    const rocketFn = new RustFunction(this, 'RocketRouterFunction', {
        manifestPath: './lambda',
        architecture: lambda.Architecture.ARM_64,
        logRetention: RetentionDays.THREE_DAYS,
        bundling: {
            cargoLambdaFlags: [
                '-F',
                'lambda',
            ],
        },
    });
    const rocketFnUrl = rocketFn.addFunctionUrl({
        authType: lambda.FunctionUrlAuthType.NONE,
    });
    new cdk.CfnOutput(this, 'RocketFnUrl', { value: rocketFnUrl.url });
    const cfDistribution = new cloudfront.Distribution(this, 'FTCWACloudfront', {
        defaultBehavior: {
            origin: new FunctionUrlOrigin(rocketFnUrl),
            viewerProtocolPolicy: cloudfront.ViewerProtocolPolicy.REDIRECT_TO_HTTPS,
            allowedMethods: cloudfront.AllowedMethods.ALLOW_GET_HEAD,
            cachePolicy: cloudfront.CachePolicy.CACHING_DISABLED,
            originRequestPolicy: cloudfront.OriginRequestPolicy.ALL_VIEWER_EXCEPT_HOST_HEADER,
        },
        domainNames: ['ftcwa.live', 'www.ftcwa.live',],
        certificate: Certificate.fromCertificateArn(this, 'ImportedCert', props.certificateArn),
        webAclId: 'arn:aws:wafv2:us-east-1:267253737119:global/webacl/CreatedByCloudFront-708b232c/df70b133-746a-4374-890b-bedfb0c1c0b9',   // Cloudfront Free plan created in console
    });
    new cdk.CfnOutput(this, 'FTCWADistributionDomainName', {
        value: cfDistribution.distributionDomainName,
    });

    this.tags.setTag('AppManagerCFNStackKey', this.stackName);
  }
}
