import * as cdk from 'aws-cdk-lib';
import * as cognito from 'aws-cdk-lib/aws-cognito'
import { Construct } from 'constructs';
import { Duration, RemovalPolicy } from 'aws-cdk-lib';
import * as ecs from 'aws-cdk-lib/aws-ecs';
import * as ec2 from 'aws-cdk-lib/aws-ec2';
import * as ecr from 'aws-cdk-lib/aws-ecr';
import * as elbv2 from 'aws-cdk-lib/aws-elasticloadbalancingv2';


export class JustCodeStackDev extends cdk.Stack {
  constructor(scope: Construct, id: string, props?: cdk.StackProps) {
    super(scope, id, props);

    const vpc = new ec2.Vpc(this, 'JustCodeDevVpc');

    const cluster = new ecs.Cluster(this, 'JustCode-CodeService-Dev', { vpc });
    cluster.addCapacity('CodeServiceASG-Dev', {
      instanceType: ec2.InstanceType.of(ec2.InstanceClass.T2, ec2.InstanceSize.MICRO)
    });
    const taskDefinition = new ecs.Ec2TaskDefinition(this, 'CodeService-TaskDefinision');
    const repo = ecr.Repository.fromRepositoryName(this, 'CodeSeriviceRepository', 'code_service');
    const container = taskDefinition.addContainer('CodeService-Container', {
      image: ecs.ContainerImage.fromEcrRepository(repo),
      memoryLimitMiB: 256,
    });

    container.addPortMappings({
      containerPort: 80,
      hostPort: 8080,
      protocol: ecs.Protocol.TCP
    });

    const service = new ecs.Ec2Service(this, "CodeService", {
      cluster,
      taskDefinition,
    });

    const lb = new elbv2.ApplicationLoadBalancer(this, 'LB', {
      vpc,
      internetFacing: true
    });
    const listener = lb.addListener('PublicListener', { port: 80, open: true });
    listener.addTargets('ECS', {
      port: 8080,
      targets: [service.loadBalancerTarget({
        containerName: 'CodeService-Container',
        containerPort: 80
      })],
    });

    new cdk.CfnOutput(this, 'LoadBalancerDNS', { value: lb.loadBalancerDnsName, });


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
