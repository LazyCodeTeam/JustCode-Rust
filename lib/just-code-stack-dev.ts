import * as cdk from 'aws-cdk-lib';
import * as cognito from 'aws-cdk-lib/aws-cognito'
import { Construct } from 'constructs';
import { Duration, RemovalPolicy } from 'aws-cdk-lib';
import * as ec2 from 'aws-cdk-lib/aws-ec2';
import { CodeServiceDev } from './code-service-dev';


export class JustCodeStackDev extends cdk.Stack {
  constructor(scope: Construct, id: string, props?: cdk.StackProps) {
    super(scope, id, props);

    const vpc = new ec2.Vpc(this, 'JustCodeDevVpc');

    new CodeServiceDev(this, vpc);

    const pool = new cognito.UserPool(this, 'Pool', {
      accountRecovery: cognito.AccountRecovery.EMAIL_ONLY,
      autoVerify: {
        email: true,
      },
      // email: cognito.UserPoolEmail.withSES({
      //   fromEmail: 'noreply@flutterlerneo.com',
      //   fromName: 'Flutter lerneo',
      //   replyTo: 'support@flutterlerneo.com',
      // }),
      email: cognito.UserPoolEmail.withCognito("support@flutterlerneo.com"),
      enableSmsRole: false,
      passwordPolicy: {
        minLength: 8,
        requireLowercase: false,
        requireUppercase: false,
        requireDigits: false,
        requireSymbols: false,
        tempPasswordValidity: Duration.days(3),
      },
      userPoolName: 'JustCodeDevUserGroup',
      standardAttributes: {
        email: {
          required: true,
          mutable: true,
        },
        nickname: {
          required: true,
          mutable: false,
        },
        profilePicture: {
          required: false,
          mutable: true,
        }
      },
      selfSignUpEnabled: true,
      userVerification: {
        emailSubject: 'Verify your email for our awesome app!',
        emailBody: 'Thanks for signing up to our awesome app! Your verification code is {####}',
        emailStyle: cognito.VerificationEmailStyle.CODE,
      },
      signInCaseSensitive: false,
      removalPolicy: RemovalPolicy.DESTROY,
    });

    const client = pool.addClient('AppClient', {
      authFlows: {
        userPassword: true,

      },
      generateSecret: true,
    })

    const domain = pool.addDomain('Domain', {
      cognitoDomain: {
        domainPrefix: 'justcode',
      }
    });

    domain.signInUrl(client, {
      redirectUri: 'https://example.com',
    })
  }
}
